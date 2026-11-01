# The machine — added questions (Blender 4.5)

**Sources consulted, and when.**
Blender 4.5 manual (`https://docs.blender.org/manual/en/4.5/`, including its images), Blender 4.5 Python API documentation (`https://docs.blender.org/api/4.5/`), Blender 4.5 release notes (`https://developer.blender.org/docs/release_notes/4.5/`), and Cycles' own files as supplied — all read 2026-07-30. Blender's requirements page (`https://www.blender.org/download/requirements/`), read 2026-08-01. Nothing else.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 7 was asked of me. Questions 1–6 are untouched.

---

## 7. Can anything a render does wait for a person, and what happens on a computer with nobody at it?

**The question is ambiguous, and I say so plainly.** "A render" can mean either (a) the narrow thing — the act of producing pixels once it has started — or (b) the whole run of Blender that ends with rendered files on disk: opening the blend-file, rendering, writing the output. I answer reading (b). Reading (a) is not dropped but left unanswered: the sources describe almost nothing about the interior of a running render, and the record of what was searched for is in 7.11. Reading (b) also carries the second half of the question, about an unattended computer, which only makes sense for a whole run. Where a finding belongs only to the wider reading, I say so.

I have split the answer into the two halves the question asks about. Each finding carries its own label. The two mandatory lines are at the end and cover the whole answer.

---

### Part one: can anything wait for a person?

#### 7.1 The render commands' parameter lists carry no way to wait for a person

**LABEL: documented.**

The full parameter list of the render operator, as printed in the API documentation, is:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')
>
> Render active scene"

Source: Python API documentation, Render Operators — `https://docs.blender.org/api/4.5/bpy.ops.render.html`. Date read: 2026-07-30.

The viewport-snapshot operator on the same page is likewise:

> "bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)"

Source and date as above.

This matters because other operators in the same documentation *do* carry a parameter for waiting on a person, so its absence here is visible rather than assumed. For example:

> "wait_for_input (boolean, (optional)) – Wait for Input, Wait for first click instead of painting immediately"

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.gpencil.html`. Date read: 2026-07-30. The same `wait_for_input` parameter appears on operators in `bpy.ops.transform`, `bpy.ops.anim`, `bpy.ops.action`, `bpy.ops.curve`, `bpy.ops.curves` and `bpy.ops.sculpt`. It appears nowhere in `bpy.ops.render`. That last comparison is the result of searching the whole of the 4.5 API documentation, not a quotation: only the `bpy.ops.gpencil` wording above is quoted.

**Not covered.** A parameter list is a list of what can be set from outside; it is not a statement that the operator shows no dialog of its own under any circumstance. This settles what the render operators *offer*, not everything they *do*.

#### 7.2 The Cycles files supplied contain nothing that waits for a person

**LABEL: observed** (for what the files contain) **and inferred** (for what that means).

Observed. Searching every Python file supplied under `cycles_4.5` for `def invoke`, `def modal`, `input(`, `raw_input`, `popup`, `confirm` and `dialog` returns no match that is anything waiting for a person. Five of the seven terms return nothing at all; the other two return one incidental match each — `input(` inside `input = find_node_input(node, input_name)` at `ui.py` line 1484, and `popup` inside `sub.template_icon_view(shading, "studio_light", scale_popup=3)` at `ui.py` line 2435. Neither is a way of asking anybody anything. The only operator definitions are three classes in `operators.py`, each of which defines `execute` and nothing else:

**[Citation check, 2026-08-07: corrected.** This sentence previously read "returns no match for any of them", which is not what the search returns — two of the seven terms do match, at the two lines now named. Nothing the finding concludes changes; the sentence was simply wrong about its own result. The same version's second-pass document had it right.**]**

> `class CYCLES_OT_use_shading_nodes(Operator):`
> `    """Enable nodes on a material, world or light"""`
> `    bl_idname = "cycles.use_shading_nodes"`

(`cycles_4.5/operators.py`, class beginning at line 14; it has a `poll` check at line 20 and `def execute(self, context):` at line 24, and no other method.)

> `class CYCLES_OT_denoise_animation(Operator):`

(`cycles_4.5/operators.py`, class beginning at line 35; its only method is `def execute(self, context):` at line 54.)

> `class CYCLES_OT_merge_images(Operator):`

(`cycles_4.5/operators.py`, class beginning at line 118; its only method is `def execute(self, context):` at line 142.)

Source: Cycles' own files as supplied, `cycles_4.5/operators.py`. Date read: 2026-07-30.

Inferred, with the step written out. The API documentation says of an operator built this way:

> "Since the operator only has an Operator.execute function it takes no user input."

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.types.Operator.html`. Date read: 2026-07-30.

The step: the documentation states that an operator with only `execute` takes no user input; the files show Cycles' three operators have only `execute`; therefore Cycles' own operators take no user input. That last sentence is mine, not the source's, so the label on it is inferred.

**Not covered.** None of these three operators is the render itself — Cycles renders through the render-engine mechanism, not through an operator. The C++ files supplied under `cycles_4.5/source` contain only `kernel` and `util`; there is no session or Blender-integration code there to examine, so I cannot say from the files what the running render does. I searched that source folder for `std::cin`, `getchar`, `scanf`, `MessageBox` and `readline(` and found none, but on so partial a copy that absence proves little.

#### 7.3 Driven from a script, operators do not call their interactive half

**LABEL: documented.**

> "This determines the context that is given for the operator to run in, and whether invoke() is called or only execute().
>
> EXEC_DEFAULT is used by default, running only the execute() method, but you may want the operator to take user interaction with INVOKE_DEFAULT which will also call invoke() if existing."

Source: Python API documentation, Operators (bpy.ops), Execution Context — `https://docs.blender.org/api/4.5/bpy.ops.html`. Date read: 2026-07-30.

And, on the same subject, from the operator page:

> "When an operator is called via bpy.ops, the execution context depends on the argument provided to bpy.ops. By default, it uses execute(). When an operator is activated from a button or menu item, it follows the setting in UILayout.operator_context. In most cases, invoke() is used."

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.types.Operator.html`. Date read: 2026-07-30.

**Not covered.** This says which half of an operator runs. It does not say that `execute` can never itself put something on screen, and it says nothing about add-ons that deliberately choose `INVOKE_DEFAULT`.

#### 7.4 Blender's own popup mechanism for menus does not stop a script

**LABEL: documented.**

> "Note that they will not block the scripts execution, so the caller can't wait for user input."

(Of `popup_menu`.) Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.types.WindowManager.html`. Date read: 2026-07-30.

The same page lists two mechanisms whose own descriptions speak of a person confirming, and these are the ones to be wary of:

> "classmethod invoke_confirm(operator, event, *, title='', message='', confirm_text='', icon='NONE', text_ctxt='', translate=True)
>
> Operator confirmation popup (only to let user confirm the execution, no operator properties shown)"

> "classmethod invoke_props_dialog(operator, *, width=300, title='', confirm_text='', cancel_default=False, text_ctxt='', translate=True)
>
> Operator dialog (non-autoexec popup) invoke (show operator properties and only execute it on click on OK button)"

Source and date as above.

**Not covered.** These are facilities available to any operator, including ones in add-ons. The documentation does not say which, if any, are used anywhere on the render path, and does not say what becomes of such a call when there is no interface.

#### 7.5 Saving the result: a still is not written out by itself, an animation is

**LABEL: documented.**

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: Manual, Rendering Animations — `https://docs.blender.org/manual/en/4.5/render/output/animation.html`. Date read: 2026-07-30.

A still can be made to save itself without a person, by a parameter or by a command-line argument:

> "write_still (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)"

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.render.html`. Date read: 2026-07-30.

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it."

Source: Manual, Command Line Arguments — `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`. Date read: 2026-07-30.

Where an animation would land on files that already exist, the choice is a setting and not a question put to anybody:

> "Overwrite
>
> Overwrite existing files when rendering."

> "Placeholders
>
> Create empty placeholder frames while rendering."

Source: Manual, Output — `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html`. Date read: 2026-07-30.

**Not covered.** The first quotation says the still is available to view and save; it does not spell out what becomes of it if nobody saves it.

#### 7.6 Places inside the interface that wait, none of them shown to sit on a render's path

**LABEL: documented** for the File Browser's behaviour; **inferred** for which render-related commands go through it.

Documented:

> "The most common way to use this editor is through modal operators (like opening or saving a blend-file). The File Browser will appear in a new window, wait for you to select a file, and then close again."

Source: Manual, File Browser — `https://docs.blender.org/manual/en/4.5/editors/file_browser.html`. Date read: 2026-07-30.

The API documentation confirms file selection is one of the modal, event-driven kinds of operator:

> "Examples of modal operators are Transform, Painting, Fly Navigation and File Select."

Source: Python API documentation, Gotchas — `https://docs.blender.org/api/4.5/info_gotchas_internal_data_and_python_objects.html`. Date read: 2026-07-30.

Documented, and this is the one that puts a warning in front of a person on a save:

> "check_existing (boolean, (optional)) – Check Existing, Check and warn on overwriting existing files"

The quoted words do not give this parameter's default, and no default is claimed here. The parameter is carried by `bpy.ops.sound.mixdown` (the Render Audio command on the Render menu), by `bpy.ops.image.save_as`, and by the save operators in `bpy.ops.wm`. Sources: Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.sound.html`, `https://docs.blender.org/api/4.5/bpy.ops.image.html`, `https://docs.blender.org/api/4.5/bpy.ops.wm.html`. Date read: 2026-07-30.

Inferred, with the step written out. `bpy.ops.sound.mixdown` — reached from the Render menu, per "Render Audio / Mix the scene's audio to a sound file" on `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html` — carries the whole cluster of parameters the manual associates with the File Browser (`filepath`, the `filter_*` family, `filemode`, `relative_path`, `display_type`, `sort_method`). The step: the manual says file operations go through the File Browser and that the File Browser waits for you to select a file; this operator's parameters are file-browser parameters; therefore, invoked from the menu, Render Audio waits for a person to choose a file. The source does not say this in so many words, so the label is inferred, not documented.

**Not covered.** I have not established that any of these sits on the path of an ordinary image render. Render Audio is a separate command; `image.save_as` is the manual save of a still. Neither is reached by rendering an animation to files.

#### 7.7 The one dialog the sources describe that can appear when a blend-file is opened

**LABEL: documented** for the dialog itself and for scripts not running by default; **inferred** for the link between that dialog and a render, and for the reading of the Freestyle sentence at the end.

> "When a blend-file tries to execute a script and is not allowed, a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts."

Source: Manual, Scripting & Security — `https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html`. Date read: 2026-07-30.

The picture on that page shows what the dialog holds. I opened `_images/advanced_scripting_security_autorun-scripts-dialog.png`, referenced from the same page. It shows a warning triangle, the text "For security reasons, automatic execution of Python scripts in this file was disabled:", a named text block below it, the line "This may lead to unexpected behavior", an unticked checkbox "Permanently allow execution of scripts", and two buttons: "Allow Execution" and "Ignore". That is what is visible in it; anything about whether it blocks is not visible and I do not claim it. Source: Manual, same page and same date read.

This bears on rendering because it governs driver expressions, which are Python:

> "Python scripts (including driver expressions) are not executed by default for security reasons."

Source: Manual, Save & Load preferences — `https://docs.blender.org/manual/en/4.5/editors/preferences/save_load.html`. Date read: 2026-07-30.

Inferred, with the step written out. The step: the manual says a dialog appears when a blend-file tries to execute a script and is not allowed; it says separately that driver expressions are Python scripts and are not executed by default; therefore a blend-file whose scene uses drivers can meet that dialog. The sources nowhere join those two sentences, so the join is mine, not theirs.

The manual's own remedy for an unattended run is a command-line switch, and the example it gives is a batch render:

> "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them:
>
> Enable with -y or --enable-autoexec
>
> Disable with -Y or --disable-autoexec"

> "To render an animation in background mode, allowing drivers and other scripts to run:
>
> blender --background --enable-autoexec my_movie.blend --render-anim"

Source: Manual, Scripting & Security — `https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html`. Date read: 2026-07-30.

One further sentence from the same page is worth quoting exactly, because it is easy to read backwards. Under the heading "Manual Execution":

> "There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off), but you should be aware that this is the case since it is not necessarily obvious."

and among the examples listed there:

> "Rendering with Freestyle, because Freestyle uses scripts to control line styles."

Source and date as above. Inferred, and the step is mine: read plainly, this says Freestyle's scripts run whether or not automatic execution is allowed — the "user interaction" being the act of asking for the render. It is not saying that Freestyle stops to ask anybody. I flag it because the phrase "require user interaction" invites the opposite reading, and the sources do not settle which is meant beyond the parenthesis.

**Not covered.** The single most important thing this does *not* settle: whether that dialog appears at all, is suppressed, is auto-answered, or blocks, when Blender runs without an interface. The manual gives the switch that avoids the situation; it never says what happens if you do not use it. See 7.11.

#### 7.8 A script cannot read typed input unless Blender is started for debugging

**LABEL: documented** for the statement; **inferred** for the consequence.

Documented, listed under the `-d, --debug` argument:

> "-d, --debug
>
> Turn debugging on.
>
> Enables memory error detection
>
> Disables mouse grab (to interact with a debugger in some cases)
>
> Keeps Python's sys.stdin rather than setting it to None"

Source: Manual, Command Line Arguments, Debug Options — `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`. Date read: 2026-07-30.

Inferred, with the step written out. The step: the argument's description says that keeping `sys.stdin` is a thing `--debug` does, which means that without `--debug` Blender sets `sys.stdin` to nothing; a script that tried to read typed input from that channel therefore has nothing to read from. The source never says "a script cannot wait for typed input", so the label is inferred.

**Not covered.** What actually happens to a script that tries anyway — an error, a silent nothing, a stop — is not stated anywhere I found.

**Added 2026-08-06 from the hand tests. LABEL: observed, and it does not agree with the inference above.** On the rented Linux machine described in 7.14, Blender 4.5 was asked what its Python's standard input was, with and without `--debug`. Both runs printed the same thing, and it was not nothing:

```
--- COMMAND: /opt/blender/4.5/blender --background --python-expr import sys; print('STDIN IS', sys.stdin)
STDIN IS <_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>
--- COMMAND: /opt/blender/4.5/blender --background --debug --python-expr import sys; print('STDIN IS', sys.stdin)
STDIN IS <_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>
```

The inference above is left standing as the researcher wrote it, and this observation sits beside it: on this machine, in this version, in an ordinary background run, a script did have a standard input to read from. What follows from that is in 7.14 (g) — a script that reads from it can hold an unattended run open for ever. Not covered: this is Linux only, and it does not say what the manual's sentence about `--debug` is describing.

---

### Part two: a computer with nobody at it

#### 7.9 Blender is meant to be run this way, and the manual says so

**LABEL: documented.**

> "-b, --background
>
> Run in background (often used for UI-less rendering).
>
> The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards."

Source: Manual, Command Line Arguments — `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`. Date read: 2026-07-30.

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

Source: Manual, Rendering From The Command Line — `https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html`. Date read: 2026-07-30.

From the API side:

> "bpy.app.background
>
> Boolean, True when blender is running without a user interface (started with -b)"

Source: Python API documentation, Application Data (bpy.app) — `https://docs.blender.org/api/4.5/bpy.app.html`. Date read: 2026-07-30.

That there is no interface at all in this mode is stated incidentally in the manual's add-on tutorial, in a code comment:

> "# Note that in background mode (no GUI available), keyconfigs are not available either,
> # so we have to check this to avoid nasty errors in background case."

Source: Manual, Add-on Tutorial — `https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html`. Date read: 2026-07-30.

**Not covered.** "No GUI available" is said in passing, about key configurations. It is not a general statement that no window of any kind can appear.

#### 7.10 The manual's own picture of unattended machines rendering together

**LABEL: documented.**

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: Manual, Output — `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html`. Date read: 2026-07-30.

And, for a run that is interrupted:

> "Just disable the Overwrite option to start rendering where you left off."

Source: Manual, Rendering Animations — `https://docs.blender.org/manual/en/4.5/render/output/animation.html`. Date read: 2026-07-30.

Nobody is asked anything at any point in either description.

**Not covered.** This describes a way of working, not what Blender does when something goes wrong on a machine with nobody at it.

#### 7.11 What the sources do not say, and where I looked

**LABEL: not in the sources.**

None of the sources says any of the following, and these are the heart of the question:

- Whether a dialog that would appear in the interface — the auto-run script dialog above all — appears, is suppressed, is answered automatically, or halts the run when Blender is started with `--background`.
- What happens to a render if something does stop and wait and nobody answers: whether it waits forever, times out, carries on, or fails.
- Whether Blender ever reports, logs or exits differently because there is nobody at the computer.

The API documentation does expose three values whose names suggest this territory, and then declines to explain them:

> "bpy.app.autoexec_fail
>
> Undocumented, consider contributing.
>
> bpy.app.autoexec_fail_message
>
> Undocumented, consider contributing.
>
> bpy.app.autoexec_fail_quiet
>
> Undocumented, consider contributing."

Source: Python API documentation, Application Data (bpy.app) — `https://docs.blender.org/api/4.5/bpy.app.html`. Date read: 2026-07-30.

**Where I looked.** All searches were run across the whole of the 4.5 manual, the whole of the 4.5 API documentation, the whole of the 4.5 release notes and the whole of the supplied Cycles files, not on selected pages.

Terms searched, plain and industry, with the outcome:

- `background mode`, `--background`, `headless`, `without a user`, `no GUI` — found the pages quoted in 7.9 and the EEVEE limitation in 7.13; nothing on dialogs in background mode.
- `dialog`, `popup`, `pop-up`, `prompt`, `confirm`, `modal` — found the security dialog, the File Browser, the `invoke_*` family, and a great deal of unrelated material about menus, brushes and node editors. Nothing about a render.
- `wait for`, `waits for`, `will wait`, `blocks until`, `until the user`, `user to confirm`, `asks you`, `ask you`, `requires user`, `user input`, `manual intervention` — found the File Browser sentence, the `wait_for_input` operator parameters listed in 7.1, the `popup_menu` note, and the Gotchas passage about scripts locking Blender. Nothing about renders waiting.
- `hang`, `stall`, `freeze`, `timeout`, `time out`, `blocking`, `nobody`, `no one`, `unattended`, `attended` — the only relevant hits were the GPU-compositor page (`https://docs.blender.org/manual/en/4.5/compositing/limitations.html`), which describes the interface becoming unresponsive during GPU compositing and the Windows driver timeout, and the `BLOCKING` operator flag, whose documented meaning is "Block anything else from using the cursor". Neither is a render waiting for a person, so I ruled both out.
- `screensaver`, `screen saver`, `sleep`, `logged in`, `log in`, `remote desktop`, `display server`, `X server`, `no display`, `without display`, `user is present` — only the SSH sentence quoted in 7.9. Nothing about session state, locked screens or sleeping machines.
- `Cannot write`, `permission denied`, `no write` — no matches at all. The sources say nothing about what a render does when it cannot write its output.
- In the Cycles files: `def invoke`, `def modal`, `input(`, `raw_input`, `popup`, `confirm`, `dialog`, `interactive`, `background` — results reported in 7.2.

I also read in full, rather than by search: the command-line arguments page, the command-line rendering page, the scripting and security page, the drivers troubleshooting page, the Output properties page, the Rendering Animations page, the render operators page, the File Browser page, and the WindowManager page. I ruled out the whole of the modelling, sculpting, animation-editor and video-sequencer sections of the manual, since their hits on these terms were all about tools operated by hand and never about rendering.

**Not covered.** Silence in the documentation is not evidence that nothing waits. It is evidence that the documentation does not describe it. Several of the items above are exactly the sort of thing a few minutes at a computer would settle; they are listed at the end.

#### 7.12 One thing the sources describe as needing a person to dismiss it, before any render begins

**LABEL: documented.**

> "When starting Blender, the splash screen appears in the center of the window. It contains options to create new projects or open recent ones."

> "To close the splash screen and start a new project, click anywhere outside the splash screen (but inside the Blender window) or press Esc."

> "When starting Blender for the first time or updating to a new version, the 'interactive region' contains a Quick Set Up Process."

Source: Manual, Splash Screen — `https://docs.blender.org/manual/en/4.5/interface/window_system/splash.html`. Date read: 2026-07-30.

I include this because the second half of the question is about a computer with nobody at it, and this is the one thing the sources describe that plainly appears on start-up and is dismissed by a person. It is not part of a render.

**Not covered.** The page does not say whether the splash screen prevents anything from proceeding, and says nothing about background mode. By 7.9, background mode has no interface, so by inference no splash screen — but that step is mine, not the source's.

#### 7.13 One documented case, in EEVEE, where headless rendering is said not to be supported

**LABEL: documented.**

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

Source: Manual, EEVEE Limitations — `https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html`. Date read: 2026-07-30.

**Not covered.** The manual does not define "headless" on that page, does not say what Blender does instead — fail, fall back, or something else — and this statement is about EEVEE alone. I found no equivalent statement for Cycles or Workbench.

---

### 7.14 Run by hand: what a computer actually did, Blender 4.5

**LABEL: observed.** Added 2026-08-06 from the hand tests. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's.

**On what kind of computer.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **with no display server at all** — no X server, no Wayland, `DISPLAY` empty, no X socket, no X or Wayland process. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`. **It was pointed at a settings folder of its own, `/root/the-machine/config/4.5`, made before it was used, never at the real one**; the starting value of Auto Run Python Scripts in that folder was checked and reported as `use_scripts_auto_execute = False`. The machine was **shared with other topics' tests throughout**, so no time below is a clean measurement and none is used as one. Date: 2026-08-06.

**(a) The auto-run warning does not stop a background render, and does not appear as anything to answer.** *(Entry 3. Bears on 7.7 and on the gap named in 7.11.)*

What was done: a blend-file was made holding the factory cube with a scripted-expression driver on its X position reading `__import__('math').floor(4.5)`, which is refused unless automatic execution is allowed. It was rendered twice with no window, once refusing and once allowing:

```
/opt/blender/4.5/blender --background --disable-autoexec …/driver2.blend --render-output …/e3r_a/d_ --render-frame 1
/opt/blender/4.5/blender --background --enable-autoexec  …/driver2.blend --render-output …/e3r_b/d_ --render-frame 1
```

What came back, from the refused run, word for word:

```
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support

Error in PyDriver: expression failed: __import__('math').floor(4.5)
For target: (type=Object, name="Cube", property=location, property_index=0)

Read blend: "/root/the-machine/files/4.5/driver2.blend"
Saved: '/root/the-machine/runs/4.5/e3r_a/d_0001.png'
Time: 00:10.55 (Saving: 00:00.12)
```

with exit status 0. The allowed run wrote its frame too, also with exit status 0. The pictures were compared as pictures, not as files:

```
PIXELS_DIFFER max_channel_difference=0.517647
```

So both runs finished on their own, both wrote a frame, and the two frames are different pictures — the refused one is the picture of a scene whose driver did not run.

**(b) A run can find out for itself that scripts were refused.** *(Entry 3. Bears on 7.11, which quotes these three values as undocumented.)* Asked in the same two conditions:

```
--- refused
AUTOEXEC_FAIL True
AUTOEXEC_FAIL_MESSAGE "Driver '__import__('math').floor(4.5)'"
AUTOEXEC_FAIL_QUIET False
CUBE_X 0.0
--- allowed
AUTOEXEC_FAIL False
AUTOEXEC_FAIL_MESSAGE ''
AUTOEXEC_FAIL_QUIET False
CUBE_X 4.0
```

**(c) A refused registered text data-block does not stop a background render either.** *(Entry 4.)* A file holding a text data-block named `Text.py` with Register switched on, printing `REGISTERED TEXT RAN`, rendered with no window. Refusing, the run printed:

```
scripts disabled for "/root/the-machine/files/4.5/regtext.blend", skipping 'Text.py'
```

and went on to write its frame with exit status 0. Allowing, the run printed `REGISTERED TEXT RAN` and wrote its frame.

**(d) Nothing else on the render path waited either.** *(Entries 1, 2, 5, 6, 7, 8, 12, 13, 14, 15, 19.)* Every one of these was run with no window and each ended by itself: a plain still render and a plain animation render, both writing their frames; Blender given a file and no render to do; a file changed and never saved, reaching Blender's own exit, which asked nothing and left the file on disk unchanged (same checksum before and after); a second file loaded over unsaved changes, which printed `SECOND FILE LOADED` and carried on; a save made with a render result in memory, which printed `Info: Saved "c.blend"` and moved the file's written-time; a frame rendered over a frame already on disk; two Blenders rendering one animation into one folder; a file whose texture had been deleted; and `bpy.ops.sound.mixdown` aimed at a path where a file already sat, which returned `{'FINISHED'}` and wrote straight over it without warning.

**(e) An output folder that cannot be written to gives an error and an exit status, and does not wait.** *(Entry 15. Bears on 7.11, which records that the sources say nothing about this, and on the researcher's own hand test 9.)* The output was aimed inside a folder mounted read-only. What came back, word for word:

**[Citation check, 2026-08-07: a repeat on the same machine disagreed on one word, and this is sent back to the hand tests.** Repeating this in 4.5 on that machine, with the folder made read-only by a read-only bind mount, gave `Error: Render error (Invalid argument) cannot save: …` where the record below gives `(Read-only file system)`. The line above it — `…x_0001.png: Read-only file system` — matched, and so did everything the entry rests on: an error, nothing written, nobody waited for, exit status 1. The record does not say how its folder was made read-only, so the two runs may not be the same test. The original is left exactly as written; neither run is being called right.**]**

```
Read blend: "/root/the-machine/files/4.5/plain.blend"
/root/the-machine/readonly/x_0001.png: Read-only file system
Error: Render error (Read-only file system) cannot save: '/root/the-machine/readonly/x_0001.png'
Time: 00:10.91


Blender quit
--- EXIT STATUS: 1
```

Nothing was written. This is the one place in the whole set where a failed unattended run told the outside world about it through its exit status.

**[Hand tests, 2026-08-07: run again on the same machine both ways of making a folder unwritable — see 7.17 (b).** The way the folder is made unwritable is **not** what the two runs differed over: both ways gave the same words as each other. What moves the bracketed word on this machine is which graphics library Blender gets, and **both wordings were reproduced on demand**, so neither run was mistaken. The record above is left exactly as written and the disagreement is not resolved. Everything the entry rests on — an error, nothing written, nobody waited for, exit status 1 — held in all eight runs.**]**

**(f) A missing texture is not mentioned at all.** *(Entry 14.)* A file whose material used an image file was rendered after that image file was deleted. The run ended by itself with exit status 0 and wrote its frame, and **nothing whatever was printed about the missing file** — the whole output was searched for it. This settles that a missing texture did not stop the run and did not make it ask anything. It says nothing about what the picture looks like without the texture, which was not compared.

**[Citation check, 2026-08-07: repeated, and a control added.** Repeating this in 4.5 on that machine gave the same answer — exit status 0, the frame written, and no word anywhere in the output about the missing file. But as written the test has no control showing the deleted image was doing anything in the render, so the silence could equally have come from a texture that never reached the picture, and a test whose only possible answer is the one it got carries nothing. The control was added on the repeat: the same file rendered once with the image present and once after deleting it came back `PIXELS_DIFFER max_channel_difference=0.694118`, so on that file the texture did reach the picture and the silence is real. The control belongs in the record for all four versions and is sent to the hand tests.**]**

**[Hand tests, 2026-08-07: the control was added to the file this entry actually used, and it failed — see 7.17 (c).** On `missingtex.blend`, in all four versions, the picture with the image and the picture without it are the same picture (`PIXELS_IDENTICAL max_channel_difference=0.000000`), because the textured material sits in a material slot no face of the cube uses. **So the silence recorded here is silence about a texture that reached nothing.** With that one thing put right, the control fires — and this version then prints `GPUTexture: Blender Texture Not Loaded!`, so **"not mentioned at all" is not what happens when the texture is in the picture**. The record above is left exactly as written. The half that did hold, and now holds on a test that could have failed: nothing waited, exit status 0, the frame written.**]**

**(g) A script inside Blender can hold an unattended run open for ever.** *(Entry 10. This is the one thing found that waits.)* With nothing on standard input:

```
--- COMMAND: /opt/blender/4.5/blender --background --python-expr print('ASKING NOW'); print('GOT', input('type something: '))
ASKING NOW
type something: Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Traceback (most recent call last):
  File "<string>", line 1, in <module>
EOFError: EOF when reading a line
```

— it failed at once. With something holding standard input open and never sending anything, the same command was **still sitting there sixty seconds later** and only a time limit stopped it.

**(h) A script hooked into a render holds the render up for as long as it runs.** *(Entry 16.)* A script that waits forty-five seconds was added to `bpy.app.handlers.render_pre`, then a still was rendered. `HANDLER STARTED` was printed, then `HANDLER FINISHED` forty-five seconds later, then the frame was saved, then `RENDER RETURNED`; the whole run took 55 seconds where the same scene rendered on its own in nine to fifteen.

**(i) Freestyle changes the picture with automatic script execution switched off.** *(Entry 20. Bears on the last paragraph of 7.7.)* The same scene was saved twice, once with `scene.render.use_freestyle` and `view_layer.use_freestyle` on and once off, and both were rendered with `--disable-autoexec`. Both ended by themselves with exit status 0, neither printed anything about scripts, and:

```
PIXELS_DIFFER max_channel_difference=0.658824
```

**(j) EEVEE renders on a computer with no display server.** *(Entry 1.)* Every render above used EEVEE, which is what the factory settings choose here — Blender reported its own list of engines as `['BLENDER_EEVEE_NEXT']`. On this machine every run also printed, before rendering:

```
libEGL warning: egl: failed to create dri2 screen
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
```

three times, and rendered anyway.

**[Citation check, 2026-08-07: a repeat on the same machine did not reproduce the warning lines, and this is sent back to the hand tests.** An EEVEE background render on that machine today, in 4.5, printed **no `libEGL` line and no `EGL Error` line at all** — the whole log was searched and returned nothing. The engine half of this finding did repeat: 4.5 on that machine still reports its engine list as `['BLENDER_EEVEE_NEXT']`. The one known difference between the two days is that the graphics card's driver was mismatched then and now reads 580.173.02 on both halves. The original is left exactly as written; neither run is being called right. This matters beyond a stray warning, because 7.15 (c) draws a contrast against it.**]**

**[Hand tests, 2026-08-07: run again on the same machine, and both runs were reproduced — see 7.17 (a).** With the card's own graphics library, EEVEE renders here and prints no `libEGL` or `EGL` line at all, and reports it is drawing on `NVIDIA GeForce RTX 4080 SUPER/PCIe/SSE2`. With that library hidden and only Mesa's left, **the same four lines come straight back, word for word**, and it reports `llvmpipe`. **So neither run was mistaken about what it saw**, and what decides it on this machine is which graphics library Blender gets — not, as this entry and 7.16 (e) read it, a machine reaching for a display server that is not there. The mismatched driver of 2026-08-06 cannot be put back, so nothing shows what this machine was doing that day. The record above is left exactly as written and **the disagreement is not resolved**.**]**

**(k) A render survives the connection it was started from being cut.** *(Entry 17.)* An animation render was started over an SSH connection which was then killed from the other end after forty-five seconds. On a fresh connection the Blender process was still there and had written 3 frames; ninety seconds later it was still there and had written 12.

**(l) Cycles on the graphics card could not be tried on this day, and the reason is the machine.** *(Entry 18.)* Asked for through OptiX and through CUDA, Blender printed `CUDA cuInit: Unknown error` and `Error: Found no Cycles device of the specified type` and gave the prompt back in about a second with exit status 1. The same machine's own `nvidia-smi` reported `Failed to initialize NVML: Driver/library version mismatch` throughout. **So this is a fault in the machine, not a finding about Blender.** The card was repaired afterwards and the same entry was run again; the answer is in 7.15. **One thing this run does settle on its own, and 7.15 leans on it**: when the device asked for is not there, Blender says so and stops with exit status 1 rather than quietly using the processor instead.

**Not covered by any of this.** Every result above is from Linux on a machine with no display server. Nothing here says what happens on macOS or on Windows, and nothing here says what any of these things do when Blender has a window — the entries aimed at that are listed at the end and have not been run.

---

### 7.15 Run by hand: Cycles on the graphics card, with no display server, Blender 4.5

**LABEL: observed.** Added 2026-08-07. This is entry 18 of `Audits/Blender Research/The machine/Hand tests — added questions.md`, run a second time after the card was repaired. It is the one thing 7.14 could not settle. The full record, with all of the machine's output word for word, is in that list.

**On what kind of computer.** The same rented Linux machine as 7.14, with its graphics card now working: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **still with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. The machine's **kernel driver and its driver library now agree on 580.173.02** (they had drifted apart on the day of 7.14, which is why that entry failed then), and `nvidia-smi` answers instead of failing. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, **pointed at a fresh settings folder of its own, `/root/the-machine/config-card/4.5`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. `nvidia-smi` reported no processes and no memory in use on the card both immediately before and immediately after, so nothing else was sharing it. Date: 2026-08-07.

**What was done.** The factory cube, camera and light at 64 by 64, saved by this version, rendered with no window twice — once through each of the two ways this card can be reached:

```
/opt/blender/4.5/blender --background …/plain.blend --engine CYCLES --render-output …/e18card_OPTIX/c_ --render-frame 1 -- --cycles-device OPTIX
/opt/blender/4.5/blender --background …/plain.blend --engine CYCLES --render-output …/e18card_CUDA/c_  --render-frame 1 -- --cycles-device CUDA
```

**What came back. Both runs rendered on the card, wrote their picture, and gave the prompt back by themselves with exit status 0.** Nothing waited and nothing was asked. Through OptiX, word for word, with the repeated per-sample lines cut where marked and nothing else removed:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/the-machine/files/4.5/plain.blend"
Fra:1 Mem:8.24M (Peak 9.12M) | Time:00:01.53 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Loading render kernels (may take a few minutes the first time)
Fra:1 Mem:8.33M (Peak 35.41M) | Time:00:02.96 | Mem:0.01M, Peak:0.01M | Scene, ViewLayer | Updating Geometry BVH Cube 2/1 | Building OptiX acceleration structure
Fra:1 Mem:8.33M (Peak 35.41M) | Time:00:02.96 | Mem:0.01M, Peak:0.01M | Scene, ViewLayer | Updating Scene BVH | Building OptiX acceleration structure
Fra:1 Mem:15.84M (Peak 35.41M) | Time:00:02.98 | Mem:660.10M, Peak:660.10M | Scene, ViewLayer | Sample 0/4096
   (the per-sample lines are cut here)
Saved: '/root/the-machine/runs/4.5/e18card_OPTIX/c_0001.png'
Time: 00:03.42 (Saving: 00:00.10)


Blender quit
--- EXIT STATUS: 0
--- SECONDS: 5
--- FILES:
total 8
-rw-r--r-- 1 root root 4609 Aug  7 00:31 c_0001.png
```

and through CUDA, word for word around the lines that matter:

```
Fra:1 Mem:8.24M (Peak 9.12M) | Time:00:01.44 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Loading render kernels (may take a few minutes the first time)
Fra:1 Mem:8.33M (Peak 35.41M) | Time:00:01.54 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Updating Scene BVH | Packing BVH nodes
Fra:1 Mem:15.84M (Peak 35.41M) | Time:00:01.55 | Mem:660.09M, Peak:660.09M | Scene, ViewLayer | Sample 0/4096
Saved: '/root/the-machine/runs/4.5/e18card_CUDA/c_0001.png'
Time: 00:01.98 (Saving: 00:00.10)


Blender quit
--- EXIT STATUS: 0
--- SECONDS: 2
--- FILES:
total 8
-rw-r--r-- 1 root root 4607 Aug  7 00:31 c_0001.png
```

**(a) Cycles renders on a graphics card on a computer with no display server.** Both ways worked, on a machine with no X server, no Wayland and no screen. The same happened in 5.0, 5.1 and 5.2, all on the same machine and the same day.

**(b) What names the device, and how far it goes.** Neither run named the card itself. What the OptiX run printed is `Building OptiX acceleration structure`, and what the CUDA run printed in the same place is `Packing BVH nodes`, so the two ran by different paths. Both reported about 660 megabytes of separate memory for the render beside Blender's own eight to sixteen. **A run that finished at all is a run that got the device it asked for**, because 7.14 (l) shows that when the device is not there Blender says `Found no Cycles device of the specified type` and stops with exit status 1 rather than quietly using the processor.

**[Citation check, 2026-08-07: the label on that last sentence is corrected to inferred.** It is not something either of these two runs printed; it is a step taken from a different run, so it does not belong under this section's **observed** label. The step written out: when the device asked for is not there, Blender prints `Found no Cycles device of the specified type` and stops with exit status 1; these two runs did not stop; therefore they got the device they asked for. This check ran the control the hand tests did not — on the repaired machine, asking 4.5 for a device this computer has not got gave exactly that error, exit status 1 and nothing written — so the step does hold. The label was the fault, not the reasoning.**]**

**(c) A Cycles run printed no display-server complaints at all.** Every EEVEE run on this machine prints `libEGL warning: egl: failed to create dri2 screen` and three `EGL Error (0x3009)` lines before rendering — see 7.14 (j). **Neither of these two Cycles runs printed any of them.** The whole record of all eight runs of this entry was searched for the words error, warning and failed, and nothing matched.

**[Citation check, 2026-08-07: the Cycles half repeated cleanly; the contrast did not.** Repeating all eight runs on that machine today and searching every log for error, warning and failed returned nothing, exactly as recorded. But the first sentence — that every EEVEE run on this machine prints those lines — is what a repeat did not reproduce; see the note at 7.14 (j). If EEVEE no longer prints them, the contrast has nothing left in it. Sent back to the hand tests with (j).**]**

**[Hand tests, 2026-08-07: the contrast is gone — see 7.17 (a).** With the card's own graphics library, **EEVEE on this machine prints none of those lines either**, so there is nothing here to set the Cycles runs against. What the Cycles half says on its own still stands and repeated cleanly: these runs printed no complaint of any kind. **The sentence that opens this entry should be read as a record of what EEVEE did on 2026-08-06 on that machine, not as a rule about EEVEE.**]**

**(d) Loading the render kernels for the first time was quick.** Blender's own line warns it "may take a few minutes the first time". In a settings folder never used before, on this machine, it took about a second and a half.

**Not covered by this.** It says nothing about macOS or Windows, nothing about a machine with more than one card, and nothing about how the picture compares with the same scene rendered on the processor — that was not tried. The times above are recorded, not measured; nothing rests on them.

---

### 7.16 Run by hand on the Creator's Mac: a second computer, and a window, Blender 4.5

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

### 7.17 Run by hand a third time: the three tests the citation check sent back, Blender 4.5

**LABEL: observed.** Added 2026-08-07. These are entries 28, 29 and 30 of `Audits/Blender Research/The machine/Hand tests — added questions.md`. They are the three items the citation check sent back that needed the rented Linux machine, run in a third sitting because that machine was about to be destroyed. The full record, with all of the machine's output word for word, is in that list.

**On what kind of computer.** The same rented Linux machine as 7.14 and 7.15, named in full: **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. Its graphics card is working and its **kernel driver and driver library agree on 580.173.02**; `nvidia-smi` reported no processes on the card, so nothing else was sharing it and the machine was not shared with other work this sitting. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, **pointed at a settings folder of its own, `/root/again-the-machine/config/4.5`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. All work sat under `/root/again-the-machine/`, and the prepared files were rebuilt by the same builder the earlier sittings used, checked to be the same file by checksum. Date: 2026-08-07.

**(a) On this machine, EEVEE renders on a computer with no display server without printing anything — and the same run prints the `libEGL` lines when it is given a different graphics library.** *(Entry 28. This is the first of the two results the citation check could not reproduce. Bears on 7.14 (j), 7.15 (c) and 7.16 (e).)*

What was done: `plain.blend` rendered with no window, and then the **whole** log searched rather than glanced at:

```
/opt/blender/4.5/blender --background …/plain.blend --render-output …/e28/e_ --render-frame 1
grep -c 'libEGL' …   grep -c 'EGL' …
```

What came back, word for word, the whole run:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/again-the-machine/files/4.5/plain.blend"
Fra:1 Mem:39.30M (Peak 39.33M) | Time:00:01.85 | Rendering 1 / 64 samples
Fra:1 Mem:39.30M (Peak 39.33M) | Time:00:01.87 | Rendering 25 / 64 samples
Fra:1 Mem:39.30M (Peak 39.33M) | Time:00:01.90 | Rendering 50 / 64 samples
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

Asked what it was drawing with, this version answered:

```
GPU_VENDOR NVIDIA Corporation
GPU_RENDERER NVIDIA GeForce RTX 4080 SUPER/PCIe/SSE2
GPU_VERSION 4.6.0 NVIDIA 580.173.02
GPU_BACKEND OPENGL
```

Then the same render again with the machine's NVIDIA graphics library hidden and only Mesa's left (`__EGL_VENDOR_LIBRARY_FILENAMES=/usr/share/glvnd/egl_vendor.d/50_mesa.json`), nothing else changed. **The lines came straight back**, word for word, at the top of the run:

```
libEGL warning: egl: failed to create dri2 screen
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/again-the-machine/files/4.5/plain.blend"
```

and the render finished by itself with exit status 0 and wrote its frame, as it did on 2026-08-06. Asked the same question, it now answered `GPU_RENDERER llvmpipe (LLVM 15.0.7, 256 bits)`. The same happened in 5.0, 5.1 and 5.2 on the same machine the same day.

**What this settles.** The words are exactly the words 7.14 (j) records, and they came back on demand. **So neither run was mistaken about what it saw.** What decides it on this machine is **which graphics library Blender gets**: with the NVIDIA one, EEVEE draws on the card and says nothing; with only Mesa's, it draws in software and prints those four lines. **The reading offered in 7.16 (e) — that these lines "belong to a machine reaching for a display server that is not there" — is not what was observed**: a machine with no display server at all printed nothing here while getting a graphics context on the card.

**What this does not settle.** The mismatched driver of 2026-08-06 cannot be put back, so **nothing here shows what that machine was doing on that day**. Whether the mismatch is what put it on Mesa's library is a step nobody has taken and it is not taken here. **The disagreement between the two runs stands recorded, not resolved. 7.14 (j) is left exactly as written.**

**(b) An output folder that cannot be written to: the way it is made unwritable is not what the two runs differed over.** *(Entry 29. This is the second of the two results the citation check could not reproduce. Bears on 7.14 (e) and 7.16 (f).)*

What was done: two unwritable folders made two different ways, each probed before use — **way A, a tmpfs mounted and then remounted read-only, which is how 7.14 (e)'s folder was made**, and **way B, a read-only bind mount, which is how the citation check's folder was made**. Both refused a write with `Read-only file system`. The same render was then aimed into each.

What came back. **Both ways, this version printed the same two lines**, word for word:

```
/root/again-the-machine/readonly-tmpfs/x_0001.png: Read-only file system
Error: Render error (Invalid argument) cannot save: '/root/again-the-machine/readonly-tmpfs/x_0001.png'
Time: 00:02.15


Blender quit
--- EXIT STATUS: 1
```

Nothing was written, nobody was waited for, and the exit status was 1 — as 7.14 (e) says. **So the way the folder is made unwritable is not what the two earlier runs differed over.** Then the same render again with only Mesa's graphics library, nothing else changed:

```
/root/again-the-machine/readonly-tmpfs/x_0001.png: Read-only file system
Error: Render error (Read-only file system) cannot save: '/root/again-the-machine/readonly-tmpfs/x_0001.png'
```

**That is 7.14 (e)'s wording, brought back on demand.** So here too **neither run was mistaken about what it saw**, and what moves the bracketed word on this machine is which graphics library Blender gets. 5.0, 5.1 and 5.2 were run the same way and did not move at all, saying `Success` in the brackets on both ways and both graphics paths. **The disagreement stands recorded, not resolved. 7.14 (e) is left exactly as written.**

**And the part the answer rests on did not move anywhere.** In all eight runs, both ways, both graphics paths, in all four versions: an error was printed, nothing was written, nobody was waited for, and the run came back with exit status 1. **What this does add is a warning: the bracketed word cannot be trusted to say what went wrong in any version now, 4.5 included.** The real reason is on the line above it in 4.5, and in the `OpenImageIO write failed` line in the other three.

**(c) A missing texture is not silent after all — and 7.14 (f) was measuring nothing.** *(Entry 30. This is the control the citation check asked for.)*

What was done, on the file 7.14 (f) actually used: rendered once with the image in place, the image deleted, rendered again, and **the two frames compared as pictures**. What came back:

```
--- the texture file before the first render:
-rw-r--r-- 1 root root 177 Aug  7 05:41 files/4.5/tex.png
--- now delete it:
ls: cannot access 'files/4.5/tex.png': No such file or directory
--- THE CONTROL, the two pictures compared:
PIXELS_IDENTICAL max_channel_difference=0.000000
```

**The control failed.** The file was then asked what it contains, and this version answered:

```
SLOTS ['Material', 'M']
MATERIAL M
  LINK Principled BSDF BSDF -> Material Output Surface
  LINK Image Texture Color -> Principled BSDF Base Color
MATERIAL Material
  LINK Principled BSDF BSDF -> Material Output Surface
IMAGE 'tex.png' source FILE size (0, 0) has_data False file //tex.png
```

The cube carries **two** material slots. The one holding the image was appended as slot 1, and every face of the cube is on slot 0. **So the deleted image was never in the picture, and 7.14 (f)'s silence is silence about a texture that reached nothing.**

The recipe was then rebuilt with **one thing changed** — the textured material put in the slot the faces use — and rendered three times: with the image as built, with the image replaced by a plain red one at the same path, and with the image deleted. What came back:

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

**The control fires.** The picture follows the image every way the image was moved. And with the texture in the picture, **this version does say something about the missing file**, word for word, the whole run:

```
GPUTexture: Blender Texture Not Loaded!
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/again-the-machine/files/4.5/mt2/missingtex2.blend"
Fra:1 Mem:41.33M (Peak 41.33M) | Time:00:01.99 | Rendering 1 / 64 samples
Fra:1 Mem:41.33M (Peak 41.33M) | Time:00:02.01 | Rendering 25 / 64 samples
Fra:1 Mem:41.33M (Peak 41.33M) | Time:00:02.03 | Rendering 50 / 64 samples
Fra:1 Mem:41.33M (Peak 41.33M) | Time:00:02.04 | Rendering 64 / 64 samples
Saved: '/root/again-the-machine/runs/4.5/e30e/m_0001.png'
Time: 00:02.16 (Saving: 00:00.10)


Blender quit
--- EXIT STATUS: 0
```

**And the message is tied to the missing file, not to the file being open.** Both runs with an image present were searched for the words texture, not loaded and failed, and returned nothing. Only the run with the image deleted printed anything. 5.0, 5.1 and 5.2 printed a different line in the same place — `ERROR Failed to create GPU texture from Blender image` — and are recorded in their own documents.

**What this settles.** **7.14 (f)'s own record stands and is not touched** — it is a true record of what that file did. But it does not answer the question it was written for, and its conclusion that a missing texture is "not mentioned at all" is **not what happens on a file where the texture is in the picture**: Blender 4.5 says `GPUTexture: Blender Texture Not Loaded!` before anything else. So an unattended run does have something to watch for here. **What did not change is the half that matters most: nothing waited.** On both files, with the image and without it, the run ended by itself with exit status 0 and wrote its frame, and that half is now carried by a test that could have failed.

**One thing this does not settle.** These messages came from EEVEE. Whether Cycles says the same about a missing image was not asked and is not answered.

**Not covered by any of this.** Every result above is from Linux on a machine with no display server. **The Mac half of two of these three entries was not run** — the citation check asked for it, and the Creator's Mac is not going anywhere, so it was left; the reasons are written out in the hand-test list. Nothing here says what happens on Windows.

---

### Outside the interface

This is where most of the answer lives, and the differences are real.

Inside the interface, things do wait: the File Browser "will appear in a new window, wait for you to select a file, and then close again" (7.6); a save warns before overwriting (7.6); the auto-run dialog offers Allow Execution or Ignore (7.7); the splash screen is dismissed by a click or Esc (7.12).

Driven from outside, the sources describe a different set of conditions. Blender started with `-b` runs "without a user interface" (7.9) and needs no graphical display, so it can be driven over a remote shell (7.9). Operators called from Python run only `execute()` by default, not `invoke()` (7.3). A still can be told to save itself with `write_still` or `-f` rather than being left in a buffer for somebody (7.5). Auto-execution of scripts, which is what the one documented render-path dialog is about, is settled by `-y` or `-Y` on the command line, and the manual's own example of doing so is a background animation render (7.7). Standard input is set to nothing unless Blender was started with `--debug` (7.8).

What the sources do **not** say — and this was the gap that mattered most for driving Blender from outside — is what becomes of a dialog that would have appeared, when there is no interface for it to appear in. See 7.11.

**That gap is now closed for Linux, by test rather than by the sources (7.14).** On a computer with no display server, driven from outside its own window, Blender 4.5 did not wait for anybody on any part of the render path that was tried: the auto-run warning became text on the terminal and the render carried straight on, a refused registered text data-block became one printed line and the render carried straight on, and unsaved changes at exit, a second file loaded over them, a save with a render result in memory, an existing output file, a missing texture, an unwritable output folder and Render Audio over an existing file all ran to their own end. A run can also read for itself, from `bpy.app.autoexec_fail`, that scripts were refused. **Two things do hold a run driven from outside**: a script that reads from standard input while something keeps standard input open, which waits for ever; and a script hooked into the render, which holds the render for as long as it runs. Standard input is a real stream in an ordinary background run, not nothing — the reading in 7.8 is not what was observed. What is still not settled from outside: everything on macOS and on Windows, and everything about what these same things do when Blender has a window.

**A second operating system has now been tried, and it agreed — see 7.16.** Every one of the nineteen entries the Linux machine ran outside Blender's own window was run again on the Creator's own Mac, in all four versions, and **the two machines agreed on every one of them**, down to the same figures where a figure came back. So what is said above about a run driven from outside is no longer true of one operating system only. **The two things that hold such a run hold on macOS too**: a script reading from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. Standard input is a real stream there as well, not nothing. **A render on the Mac also outlives the shell it was started from, and outlives a hang-up signal to its whole process group**, which is what closing a terminal window really sends. Still open from outside: everything on Windows, which no Windows machine has run.

**Three of these results were run a third time, and one of them changes (7.17).** A missing texture is **not** silent when the texture is actually in the picture: 4.5 prints `GPUTexture: Blender Texture Not Loaded!` before anything else, and the run still ends by itself with exit status 0 and writes its frame. The entry that reported silence was rendering a file whose textured material sat in a slot no face used, so it was measuring nothing; that record stands but does not answer its question. **So an unattended run driven from outside does have one more thing to watch for on its output, beyond an exit status.** Nothing else in the list above moves: an unwritable output folder still gives an error, writes nothing, waits for nobody and comes back with exit status 1, in every version, however the folder is made unwritable. **And one thing said above about this machine should not be read as a rule about Blender**: the `libEGL` and `EGL Error` lines are not what a machine without a display server prints when it reaches for one — with the card's own graphics library, EEVEE renders here on a machine with no display server and prints nothing at all.

**And the window has now been reached, on the Mac — the thing every version of this answer has been leaving open (7.16).** With a window open, the automatic-execution warning is a **dialog in the middle of the main window**, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with the dialog unanswered, and with the first-run Quick Set Up unanswered underneath it. A **file chooser opened on top of a running animation render stopped no frames at all**. A still render nobody asks to be saved still writes nothing — **but the picture opens in a window of its own either way**. A render asked for on the command line with a window open is written at once, **but that Blender then never quits**. A `--background` render **does register with the Mac's window server** for the whole run, though it never comes to the front and draws nothing. And Render Audio is the one place where the menu's way and a script's way part company: **with no path named it opens a chooser and writes nothing; with a path named it writes straight over an existing file without asking** — and that, not the window, is what decides.

### Operating system

The sources distinguish operating systems in a few specific places and nowhere else on this subject.

**Linux is now settled by test as well (7.14), and only Linux.** Everything in 7.14 was got on Ubuntu 22.04.5 LTS on a machine with no display server. It says nothing about macOS, where the same tests have not been run, and nothing about Windows, which no Windows machine has run. So where an answer above says the sources attach no operating system to it, that is still true of the sources; what has changed is that one operating system has now been tried.

- EEVEE: "Headless rendering is not supported on headless Windows systems" (7.13). This is the only operating-system-specific statement I found that bears directly on rendering with nobody at the computer.
- The remark that no graphical display is needed names Linux by example: "no need for X server on Linux for example" (7.9). The sentence itself is general; only its example is Linux.
- Two command-line arguments are marked for one system only: "-con, --start-console … Start with the console window open (ignored if -b is set), (Windows only)", and "/? … Print this help text and exit (Windows only)". Source: Manual, Command Line Arguments — `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`. Date read: 2026-07-30.
- The Windows driver-timeout note on the GPU compositor page (`https://docs.blender.org/manual/en/4.5/compositing/limitations.html`) is named for Windows, but it is about GPU compositing rather than about waiting for a person.

Everything else quoted above — background mode, the File Browser, auto-execution, `write_still`, `sys.stdin`, Overwrite and Placeholders — is stated without any operating system attached, and the sources do not say whether it differs from one to another.

**macOS has now been tried as well (7.16), and it agreed with Linux on everything the two machines both ran.** The Creator's own Mac — a MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, with Apple's own graphics and no second card — gave the same answer as the rented Linux machine on all nineteen entries that were run on both, in all four versions. **It also reached the one thing Linux could not: a window.** Two differences between the two machines showed in what was printed, and neither changes an answer: the Mac prints no `libEGL` or `EGL Error` lines, because it has a display server; and Blender 4.5 on the Mac prints a run of `Freestyle:` progress lines that the Linux record does not show. **One of those two printing differences has been taken back, by test on the Linux machine (7.17).** The `libEGL` and `EGL Error` lines are **not** the mark of a machine without a display server. On that same machine, with its own graphics card's library, EEVEE renders with no display server at all and prints none of them; with that library hidden and only Mesa's software one left, all four versions print them word for word. **So it is which graphics library Blender gets that decides, not whether there is a display server** — and the Mac's silence is not evidence either way. The other difference, the Freestyle progress lines, is untouched and stands.

**Windows is still untouched** — no Windows machine has run it — so every statement above that names Windows stays untested. **And one thing on the Mac was not run**: whether the screen locking or the display going to sleep interrupts a render. That needs the Creator's own machine to itself for several minutes, and this sitting had no standing to take it; the test is written out in full in `Audits/Blender Research/The machine/Hand tests — added questions.md`, entry 21.

---

## Things a person at a computer could settle by hand

**Pruned 2026-08-06 after the hand tests, again 2026-08-07 after the card tests, and again 2026-08-07 after the Mac sitting.** Everything a terminal could reach was run on the rented Linux machine; everything needing a window was then run on the Creator's own Mac. **All of it is answered in 7.14, 7.15 and 7.16, and those items are gone from this list.** The full list, with every command written out, is `Audits/Blender Research/The machine/Hand tests — added questions.md`.

**What is left is two things, and neither is about Blender being unwilling.**

1. **Does the screen locking, or the display going to sleep, interrupt a render? — Not run, and it needs the Creator.**
   The only machine this project has with a screen is the Creator's own computer. This test puts that screen to sleep on purpose and then needs the machine left untouched for several minutes, and the Mac sitting had no standing to take the Creator's own desk over that way. It is written out in full as entry 21 of the hand-test list. *What to look at:* whether any frame file carries a written-time from inside the stretch when the screen was asleep, whether the process is still alive at the end, and the last line of the log. **The nearest thing that was answered is that a render on that Mac survived the shell it was started from being killed, and survived a hang-up signal to its whole process group — which is a different and smaller event.**

2. **Everything on Windows. — Cannot be run.**
   No Windows machine has run it, so the EEVEE headless-Windows statement, the Windows crash pop-up, and the two Windows-only command-line arguments all stay untested rather than answered. Each is written out in the hand-test list, ready for the next Windows rental.

---

2026-08-06 — Opus 5 (Claude Code) — The machine (question 7), Blender 4.5

2026-08-07 — Opus 5 (Claude Code) — the card tests for The machine, added questions

2026-08-07 — Opus 5 (Claude Code) — the Mac hand tests for The machine, added questions

2026-08-07 — Opus 5 (Claude Code) — the tests the citation check sent back for The machine, added questions

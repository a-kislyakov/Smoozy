# The machine — added questions (Blender 5.1)

**Sources consulted, and when.** Blender's manual (`manual_5.1`, including its `_images` folder), Blender's Python API documentation (`api_5.1`), Blender's release notes (`release_notes_5.1`) and Cycles' own files (`cycles_5.1`) — all read 2026-07-30. Blender's requirements page — read 2026-08-01.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: only question 7 was asked of me. Every other question on the sheet is untouched.

---

## 7. Can anything a render does wait for a person, and what happens on a computer with nobody at it?

### The question was ambiguous, and here is the reading I took

"A render" can mean two things. The narrow reading is the act of computing and writing the pixels — everything between pressing render and the file appearing. The wide reading is the whole run of Blender that ends in a rendered file, including opening the blend-file first.

**I answer the wide reading.** On the narrow reading the answer is a flat "not in the sources": I found nothing in the manual, the API documentation, the release notes or Cycles' files that describes sampling, denoising, compositing or image writing stopping to ask a person anything. The only things the sources describe as asking a person sit at file-load time. Whether either of them holds a render up is a separate matter, and the entries below say where that is settled and where it is not. The other reading would leave the question with no content, so I report both.

### Label

**Mixed.** Part A below is **documented** for the things the sources describe as asking a person (they are on the load path, not inside the render), **not in the sources** for anything inside the render itself, and two items are **inferred** — the reading of the picture in A1, and A9. Part B is **documented** for how Blender is run with nobody at it, and **not in the sources** for most of what the machine itself must be doing.

---

### Part A — where the sources describe a person being asked something

Being asked is not the same as being made to wait. Where the sources do not settle which of the two a thing does, the entry below says so.

#### A1. A blend-file whose scripts are blocked shows a dialog. Documented that it appears; not established that it holds anything up.

> "When a blend-file tries to execute a script and is not allowed, a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts."

Manual, *Scripting & Security*. https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html — read 2026-07-30.

The same page names the picture that goes with it: `[image: _images/advanced_scripting_security_autorun-scripts-dialog.png]`. **I opened that picture.** It shows a warning triangle, the words "For security reasons, automatic execution of Python scripts in this file was disabled:", below that the line "Text 'Text.py'" and the line "This may lead to unexpected behavior", then a tick-box labelled "Permanently allow execution of scripts", then two buttons — "Allow Execution" on the left and "Ignore" on the right. That is what is visible. **Documented** (it is part of the manual).

The manual is not consistent about what this thing is. On the security page, quoted above, it is "a dialog". The drivers page shows the identical picture with a different caption:

> "An Auto-run warning in the Info editor's header."

Manual, *Troubleshooting* (drivers). https://docs.blender.org/manual/en/5.1/animation/drivers/troubleshooting.html — read 2026-07-30. *(Citation check, 2026-08-07: the quote is at that address, and the same caption also sits on the Scripting & Security page itself, at `manual_5.1/advanced/scripting/security.txt` line 15. So the manual contradicts itself on one page as well as between two — which strengthens this finding rather than weakening it.)*

**Inferred, and here is the step:** the picture's own words are in the past tense — execution "was disabled" — and one of the buttons is "Ignore". For those words to make sense, the file must already have loaded with its scripts off before the warning appeared. So this warns after the fact rather than halting and waiting. A reader has to supply that step; the sources do not state it. Do not take this further than it goes — I cannot tell you from the sources whether a render started immediately after would run.

Which scripts this covers, from the same page:

> "A text data-block can have its Register option enabled which means it will load on start."

> "Python expressions can be used to Drive values and are often used in more advanced rigs and animations."

#### A2. Rendering with Freestyle runs scripts even when auto execution is off. Documented.

> "There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off), but you should be aware that this is the case since it is not necessarily obvious."
>
> "Running a script in the Text editor."
>
> "Rendering with Freestyle, because Freestyle uses scripts to control line styles."

Manual, *Scripting & Security*. https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html — read 2026-07-30.

This is the only place in the sources that ties rendering to the words "user interaction", and what it says is that the scripts run rather than being blocked. The page does not say what counts as the user interaction when the render was not started by a person.

#### A3. The way to take A1 out of the picture from outside. Documented.

> "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them:"
>
> "Enable with -y or --enable-autoexec"
>
> "Disable with -Y or --disable-autoexec"

And the worked example on the same page:

> "To render an animation in background mode, allowing drivers and other scripts to run:"
>
> "blender --background --enable-autoexec my_movie.blend --render-anim"

> "These command-line arguments can be used to start a regular Blender instance and will still override the Preferences."

Manual, *Scripting & Security*. https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html — read 2026-07-30.

The command-line list gives the same two, and states which is the default:

> "-Y, --disable-autoexec / Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

Manual, *Command Line Arguments*. https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

The setting behind it, in Preferences:

> "Python scripts (including driver expressions) are not executed by default for security reasons."

Manual, *Save & Load* preferences. https://docs.blender.org/manual/en/5.1/editors/preferences/save_load.html — read 2026-07-30.

Blender also reports the failure to a script:

> "bpy.app.autoexec_fail / Boolean, True when auto-execution of scripts failed (read-only)."
>
> "bpy.app.autoexec_fail_message / String, message describing the auto-execution failure (read-only)."
>
> "bpy.app.autoexec_fail_quiet / Boolean, True when auto-execution failure should be quiet, set after the warning is shown once for the current blend file (read-only)."

API documentation, *Application Data*. https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30.

#### A4. Loading a file over unsaved changes asks you. Documented.

> "By default, when exiting Blender or loading a new blend-file, if you have unsaved changes, a pop-up will ask you to either confirm discarding those changes, or save them."
>
> "This behavior can be disabled with the Save Prompt option in the Save & Load section of the Preferences."

Manual, *Opening & Saving*. https://docs.blender.org/manual/en/5.1/files/blend/open_save.html — read 2026-07-30.

Also listed as one of three data-loss guards:

> "Save Prompt prompts you to save unsaved changes on exit"

Manual, *Recovering Data*. https://docs.blender.org/manual/en/5.1/troubleshooting/recover.html — read 2026-07-30.

This matters for a render only if the same running Blender loads a second file after something changed the first.

#### A5. Whether an operator takes interaction at all is a choice made by the caller. Documented.

> "This determines the context that is given for the operator to run in, and whether invoke() is called or only execute()."
>
> "EXEC_DEFAULT is used by default, running only the execute() method, but you may want the operator to take user interaction with INVOKE_DEFAULT which will also call invoke() if existing."

API documentation, *Operators*. https://docs.blender.org/api/5.1/bpy.ops.html — read 2026-07-30.

So the default when Blender is driven by a script is the path that does not ask.

The render operator itself carries no documentation text at all:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
>
> "Undocumented, consider contributing."
>
> "write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)"

API documentation, *Render Operators*. https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30.

#### A6. Which window-manager calls wait for a click, and which explicitly do not. Documented.

Wait for a click:

> "invoke_props_dialog(operator, *, width=300, title='', confirm_text='', cancel_default=False, text_ctxt='', translate=True) / Operator dialog (non-autoexec popup) invoke (show operator properties and only execute it on click on OK button)"

> "invoke_confirm(operator, event, *, title='', message='', confirm_text='', icon='NONE', text_ctxt='', translate=True) / Operator confirmation popup (only to let user confirm the execution, no operator properties shown)"

> "invoke_search_popup(operator) / Operator search popup invoke which searches values of the operator's bpy.types.Operator.bl_property (which must be an EnumProperty), executing it on confirmation"

Do not wait:

> "popup_menu(draw_func, *, title='', icon='NONE')"
>
> "Popup menus can be useful for creating menus without having to register menu classes."
>
> "Note that they will not block the scripts execution, so the caller can't wait for user input."

API documentation, *WindowManager*. https://docs.blender.org/api/5.1/bpy.types.WindowManager.html — read 2026-07-30.

The manual's general statement about the same idea:

> "Modal operators exist as a concept in between Tools and regular operators."
>
> "They require some sort of interactive input."
>
> "Esc, RMB / Cancels a modal operator."
>
> "Return, LMB / Confirms the action of a modal operator."

Manual, *Operators*. https://docs.blender.org/manual/en/5.1/interface/operators.html — read 2026-07-30.

#### A7. Opening a file from a call can suppress the file browser and set trust directly. Documented.

From `bpy.ops.wm.open_mainfile`:

> "use_scripts (bool) – Trusted Source, Allow .blend file to execute scripts automatically, default available from system preferences (optional)"

> "display_file_selector (bool) – Display File Selector, (optional)"

API documentation, *Window Manager Operators*. https://docs.blender.org/api/5.1/bpy.ops.wm.html — read 2026-07-30.

The matching thing in the interface:

> "Trusted Source"
>
> "When enabled, Python scripts and drivers that may be included in the file will be run automatically."

Manual, *Opening & Saving*. https://docs.blender.org/manual/en/5.1/files/blend/open_save.html — read 2026-07-30.

#### A8. Overwriting render output is governed by a switch. Documented that the switch exists; not in the sources that nothing is asked.

> "Overwrite existing files when rendering."

> "Create empty placeholder frames while rendering."

And the note that follows, which is the sources' own recipe for many machines at once:

> "An easy way to get multiple machines to share the rendering workload is to:"
>
> "Set up a shared directory over a network file system."
>
> "Disable Overwrite, enable Placeholders in the Render Output panel."
>
> "Start as many machines as you wish rendering to that directory."

Manual, *Output Properties*. https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

**Not in the sources**, for whether anything is asked. Searching the terms listed at the bottom, I found no prompt described anywhere for render output overwriting. That is silence, not a statement that nothing is asked, and it is the same kind of silence as A10 — so it carries the same label. The one warn-before-overwrite behaviour I did find is quoted on operators and pages about saving by hand rather than about rendering:

> "check_existing (bool) – Check Existing, Check and warn on overwriting existing files (optional)"

API documentation, *Image Operators* (on `bpy.ops.image.save_as` and others). https://docs.blender.org/api/5.1/bpy.ops.image.html — read 2026-07-30.

> "If a file with the same given name already exists, the text field will turn red as a warning that the file will be overwritten."

Manual, *Opening & Saving*. https://docs.blender.org/manual/en/5.1/files/blend/open_save.html — read 2026-07-30.

#### A9. A script cannot read the keyboard unless you ask for it. Inferred.

> "-d, --debug"
>
> "Turn debugging on."
>
> "Enables memory error detection"
>
> "Disables mouse grab (to interact with a debugger in some cases)"
>
> "Keeps Python's sys.stdin rather than setting it to None"

Manual, *Command Line Arguments*. https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

**The step:** the words say `--debug` *keeps* `sys.stdin` *rather than* setting it to None. For "rather than" to carry meaning, without `--debug` Blender sets `sys.stdin` to None. A script therefore has no standard input to read from unless `--debug` is passed. What a script that tries to read anyway actually does is not stated anywhere I found.

The one argument that openly waits for a person is a deliberate choice, not something a render does:

> "--python-console / Run Blender with an interactive console."

Same page, same date.

#### A10. Inside the render itself — not in the sources.

I found nothing describing anything in the rendering process — sampling, denoising, compositing, encoding, writing the image — that stops and waits for a person. The nearest thing the sources say is the opposite direction: Blender waiting on a script, not on a person.

> "While a script executes, Blender waits for it to finish and is effectively locked until it's done; while in this state Blender won't redraw or respond to user input."

API documentation, *Gotchas: internal data and Python objects*. https://docs.blender.org/api/5.1/info_gotchas_internal_data_and_python_objects.html — read 2026-07-30.

The record of where I looked for this is in the section at the bottom.

---

### Part B — what happens on a computer with nobody at it

#### B1. Blender has a mode with no interface at all. Documented.

> "-b, --background"
>
> "Run in background (often used for UI-less rendering)."
>
> "The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards."

Manual, *Command Line Arguments*. https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

> "bpy.app.background"
>
> "Boolean, True when blender is running without a user interface (started with -b)"

API documentation, *Application Data*. https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30.

A second argument implies it:

> "-c, --command <command>"
>
> "Run a command which consumes all remaining arguments."
>
> "This implies --background mode."

Manual, *Command Line Arguments*, same address and date.

#### B2. No graphical display is needed to render. Documented, and it names Linux only.

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

Manual, *Rendering From The Command Line*. https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30.

The same page gives the two shapes of the command:

> "blender -b file.blend -f 10"

> "blender -b file.blend -a"

> "-b / Render in the background (without UI)."

> "Arguments are executed in the order they are given!"

> "Always position -f or -a as the last arguments."

#### B3. Some parts of Blender are not there in background mode. Documented.

> "# Lazily import GPU module, so that the render engine works in background mode where the GPU module can't be imported by default."

API documentation, *RenderEngine* (a comment inside the worked example). https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

> "-con, --start-console"
>
> "Start with the console window open (ignored if -b is set), (Windows only)."

Manual, *Command Line Arguments*, same address and date.

> "--disable-depsgraph-on-file-load"
>
> "Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)."
>
> "NOTE: this is a temporary option, in the future depsgraph will never be automatically generated on file load in background mode."

Same page, same date.

And Blender's own shutdown handler distinguishes the two situations:

> "just before Blender shuts down, while all data is still valid. Accepts one boolean argument. True indicates either that a user has been using Blender and exited, or that Blender is exiting in a circumstance that should be treated as if that were the case. False indicates that Blender is running in background mode, or is exiting due to failed command line arguments, etc."

API documentation, *Application Handlers*. https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30.

#### B4. One documented limit that uses the word "headless". Documented.

> "Headless Rendering"
>
> "Headless rendering is not supported on headless Windows systems."

Manual, *EEVEE Limitations*. https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html — read 2026-07-30.

The page does not define "headless", and gives no equivalent statement for macOS or Linux. Two other limits sit beside it on the same page and are worth having in view because they bear on an unattended machine:

> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient."

> "There is currently no support for multiple GPU systems."

#### B5. How a run ends without anyone to see it. Documented.

> "--python-exit-code <code>"
>
> "Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables."

> "--debug-exit-on-error"
>
> "Immediately exit when internal errors are detected."

Manual, *Command Line Arguments*. https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

The API documentation's advice on watching an unattended script:

> "Once the script is running properly in background mode, you'll want to check the output of the script, this depends completely on the task at hand, however, here are some suggestions:"
>
> "Render the output to an image, use an image viewer and keep writing over the same image each time."
>
> "Save a new blend-file, or export the file using one of Blender's exporters."
>
> "If the results can be displayed as text then print them or write them to a file."

API documentation, *Tips and Tricks*. https://docs.blender.org/api/5.1/info_tips_and_tricks.html — read 2026-07-30.

#### B6. Release notes touching background mode in this version. Documented statements of change.

> "Show currently rendered frame in background mode"

> "`print_undo_steps` crash in empty background mode session"

> "Storm Hydra crashes with Vulkan backend in background mode over ssh"

Release notes 5.1, *Bug Fixes*. https://developer.blender.org/docs/release_notes/5.1/bugfixes/ — read 2026-07-30.

#### B7. What the sources do not say about the machine. Not in the sources.

I found nothing about any of these:

- Whether somebody must be logged in to the computer, on any operating system.
- Whether the screen may be locked, or the machine asleep.
- Whether Blender can be run as a Windows service or a Linux system daemon.
- Whether a graphics card is usable when no monitor is attached to it.
- What happens to the auto-run dialog (A1) or the unsaved-changes pop-up (A4) when there is no interface to draw them in — whether they are skipped, answered with a default, or block the run.
- What happens when a script calls something that would normally show a dialog while Blender is in background mode.

The requirements page has nothing on any of this either. Its only mention of displays is about working at a screen:

> "While it is technically possible to use Blender in all screen sizes, a minimum resolution of 1920x1080 is recommended for optimal use. Multiple-display setups are supported."

Blender's requirements page. https://www.blender.org/download/requirements/ — read 2026-08-01. Note that this page speaks for Blender 5.1 only where it names 5.1 outright, and it does not name it here.

---

### Part C — run by hand: what a computer actually did, Blender 5.1

**LABEL: observed.** Added 2026-08-06 from the hand tests. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's.

**On what kind of computer.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **with no display server at all** — no X server, no Wayland, `DISPLAY` empty, no X socket, no X or Wayland process. Blender 5.1.2 at `/opt/blender/5.1/blender`, **pointed at a settings folder of its own, `/root/the-machine/config/5.1`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. The machine was **shared with other topics' tests throughout**, so no time below is a clean measurement and none is used as one. Date: 2026-08-06.

**C1. The dialog of A1 does not hold up a background render.** *(Entry 3.)* A file was made holding the factory cube with a scripted-expression driver on its X position reading `__import__('math').floor(4.5)`, and rendered twice with no window. Refusing, word for word:

```
00:00.347  blend            | Read blend: "/root/the-machine/files/5.1/driver2.blend"
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support

Error in PyDriver: expression failed: __import__('math').floor(4.5)
For target: (type=Object, name="Cube", property=location, property_index=0)

00:00.355  render           | Rendering single frame
00:14.782  render           | Saved: '/root/the-machine/runs/5.1/e3r_a/d_0001.png'
00:14.782  render           | Time: 00:14.42 (Saving: 00:00.11)
```

exit status 0. Allowing wrote a frame too, exit status 0, and the pictures — compared as pictures, not as files — came back `PIXELS_DIFFER max_channel_difference=0.517647`. So on a computer with no window the thing A1 describes becomes text on the terminal and the render carries straight on with a scene whose driver did not run. **A1's inferred reading — that it warns after the fact rather than halting — is what a computer does here.** The inference is left as the researcher wrote it; this is the observation beside it.

**C2. The three values in A3 do what their names suggest.** *(Entry 3.)* Refused: `AUTOEXEC_FAIL True`, `AUTOEXEC_FAIL_MESSAGE "Driver '__import__('math').floor(4.5)'"`, `CUBE_X 0.0`. Allowed: `AUTOEXEC_FAIL False`, empty message, `CUBE_X 4.0`. So an unattended run can find out for itself that scripts were refused.

**C3. A refused registered text data-block behaves the same way.** *(Entry 4.)* The run printed `scripts disabled for "/root/the-machine/files/5.1/regtext.blend", skipping 'Text.py'` and went on to write its frame with exit status 0; allowing it, the text's own line `REGISTERED TEXT RAN` appeared.

**C4. The pop-up of A4 does not fire with no window.** *(Entries 6 and 7.)* A file changed and never saved, reaching Blender's own exit, exited by itself with status 0, asked nothing, and left the file on disk with the same checksum. A second file loaded over unsaved changes printed `SECOND FILE LOADED` and carried straight on.

**C5. A8's silence is answered: nothing is asked, and the switch is read.** *(Entry 12.)* A frame was rendered, then rendered again to the same path: nothing was asked, the written-time moved, and the picture was `PIXELS_IDENTICAL max_channel_difference=0.000000` with the first. A third run with Overwrite off and Placeholders off printed `00:00.464  reports          | No frames rendered, skipped to not overwrite` and left the file's written-time alone — the control that shows the switch was being read.

**C6. A9 is not what a computer does.** *(Entry 9.)* With and without `--debug`, both runs printed the same thing, and it was not nothing:

```
STDIN IS <_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>
```

The inference in A9 is left standing as the researcher wrote it, and this observation sits beside it. What follows from it is C7.

**C7. A script inside Blender can hold an unattended run open for ever.** *(Entry 10. This is the one thing found that waits.)* With nothing on standard input, `input()` failed at once with `EOFError: EOF when reading a line`. With something holding standard input open and never sending anything, the same command was **still sitting there sixty seconds later**, and only a time limit stopped it.

**C8. A script hooked into the render holds the render for as long as it runs.** *(Entry 16.)* A script waiting forty-five seconds was added to `bpy.app.handlers.render_pre`: `HANDLER STARTED`, then `HANDLER FINISHED`, then the frame saved, then `RENDER RETURNED`, 69 seconds in all where the same scene rendered alone in eleven to sixteen.

**C9. A still render writes nothing unless asked.** *(Entry 11.)* `bpy.ops.render.render()` with the output path set and printed left the folder empty; `bpy.ops.render.render(write_still=True)` wrote `s_.png`.

**C10. An output folder that cannot be written to gives an error and an exit status, and does not wait.** *(Entry 15. One of the four cases named in Part B's "Not covered".)* Word for word:

```
00:11.490  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/the-machine/readonly/x_0001.png"
00:11.490  reports          | ERROR Render error (Success) cannot save: '/root/the-machine/readonly/x_0001.png'

Blender quit
--- EXIT STATUS: 1
```

Nothing was written. This is the one place in the whole set where a failed unattended run told the outside world about it through its exit status.

**C11. A missing texture is not mentioned at all.** *(Entry 14. Another of those four cases.)* A file whose material used an image was rendered after that image was deleted. It ended by itself with exit status 0 and wrote its frame, and nothing whatever was printed about the missing file.

**C12. Two Blenders share one output folder without being asked anything.** *(Entry 13. The arrangement quoted in A8.)* Two runs of the same forty-frame animation were started at once into one folder with Overwrite off and Placeholders on. Both ended by themselves with exit status 0, all forty frames were there and none was empty, and each skipped what the other had taken. **This is not the arrangement the manual describes**, which has several machines sharing a folder over a network file system; this was two Blenders on one machine sharing one ordinary folder.

**C13. Freestyle changes the picture with automatic script execution switched off.** *(Entry 20. Bears on A2.)* The same scene saved with `scene.render.use_freestyle` and `view_layer.use_freestyle` on and off, both rendered with `--disable-autoexec`: both ended by themselves, neither said anything about scripts, the Freestyle run printed `00:10.756  render           | Rendering freestyle` as a step of its own, and `PIXELS_DIFFER max_channel_difference=0.658824`. So Freestyle's own work reached the picture in a run nobody started by hand.

**C14. Render Audio, driven from a script, writes straight over a file already there.** *(Entry 19.)* `bpy.ops.sound.mixdown(filepath=…)` aimed at a path where a file already sat returned `MIXDOWN RETURNED {'FINISHED'}`, ended in about a second with exit status 0, and replaced the file — its size went from 24 bytes to 8316 and its checksum changed. No window, nothing asked.

**C15. A render survives the connection it was started from being cut.** *(Entry 17.)* An animation render was started over an SSH connection which was then killed from the other end after forty-five seconds. On a fresh connection the process was still there with 4 frames written; ninety seconds later it was still there with 13.

**C16. EEVEE renders on a computer with no display server; Cycles could not be tried on this day.** *(Entries 1 and 18. Bears on the researcher's own hand test 5.)* Every render above used EEVEE, which is what the factory settings choose here. Cycles was asked for through OptiX and through CUDA and printed `00:00.822  cycles           | ERROR CUDA cuInit: Unknown error` and `00:00.826  reports          | ERROR Found no Cycles device of the specified type`, then gave the prompt back in about a second with exit status 1. The machine's own `nvidia-smi` reported `Failed to initialize NVML: Driver/library version mismatch` throughout, **so this is a fault in the machine, not a finding about Blender**. The card was repaired afterwards and the entry was run again; hand test 5 is answered in Part E. **One thing this run settles on its own, and Part E leans on it**: when the device asked for is not there, Blender says so and stops with exit status 1 rather than quietly using the processor instead.

**Not covered by any of this.** Every result above is from Linux on a machine with no display server. Nothing here says what happens on macOS or on Windows, and nothing here says what any of these things do when Blender has a window. Nothing here touches B7's questions about the machine itself — logging in, locking, sleeping — because this machine has no screen to lock.

**[Citation check, 2026-08-07: two notes on the entries above.** *The unwritable output folder:* repeating that entry in 4.5 on the same machine, with the folder made read-only by a read-only bind mount, gave a different word in the brackets from the one 4.5's own record gives, so the bracketed reason goes back to the hand tests for every version — the behaviour it rests on (an error, nothing written, nobody waited for, exit status 1) matched. *The missing texture:* as written that test has no control showing the deleted image was doing anything in the render. The control was added on a repeat in 4.5 and came back `PIXELS_DIFFER max_channel_difference=0.694118`; the control belongs in this version's record too and is sent to the hand tests.**]**

**[Hand tests, 2026-08-07: both went back to the machine, in this version — see Part G.** *C10, the unwritable output folder:* the folder was made unwritable both ways and **the two ways gave the same words as each other**, so that is not what the two runs differed over; this version said `Success` in the brackets on both ways and on both graphics paths, as both earlier runs found. *C11, the missing texture:* the control was added to the file that entry used, **and it failed** — the two pictures are the same picture, because the textured material sits in a slot no face uses. With that put right the control fires, and **this version then prints `ERROR Failed to create GPU texture from Blender image`**, so a missing texture is mentioned when it is in the picture. Both records above are left exactly as written.**]**

---

### Part E — run by hand: Cycles on the graphics card, with no display server, Blender 5.1

**LABEL: observed.** Added 2026-08-07. This is entry 18 of `Audits/Blender Research/The machine/Hand tests — added questions.md`, run a second time after the card was repaired. It is the one thing Part C could not settle, and it is the researcher's own hand test 5. The full record, with all of the machine's output word for word, is in that list.

**On what kind of computer.** The same rented Linux machine as Part C, with its graphics card now working: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **still with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. The machine's **kernel driver and its driver library now agree on 580.173.02** (they had drifted apart on the day of Part C, which is why that entry failed then), and `nvidia-smi` answers instead of failing. Blender 5.1.2 at `/opt/blender/5.1/blender`, **pointed at a fresh settings folder of its own, `/root/the-machine/config-card/5.1`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. `nvidia-smi` reported no processes and no memory in use on the card both immediately before and immediately after, so nothing else was sharing it. Date: 2026-08-07.

**What was done.** The factory cube, camera and light at 64 by 64, saved by this version, rendered with no window twice — once through each of the two ways this card can be reached:

```
/opt/blender/5.1/blender --background …/plain.blend --engine CYCLES --render-output …/e18card_OPTIX/c_ --render-frame 1 -- --cycles-device OPTIX
/opt/blender/5.1/blender --background …/plain.blend --engine CYCLES --render-output …/e18card_CUDA/c_  --render-frame 1 -- --cycles-device CUDA
```

**What came back. Both runs rendered on the card, wrote their picture, and gave the prompt back by themselves with exit status 0.** Nothing waited and nothing was asked. Through OptiX, word for word, with the repeated per-sample lines cut where marked and nothing else removed:

```
00:00.359  blend            | Read blend: "/root/the-machine/files/5.1/plain.blend"
00:00.368  render           | Rendering single frame
00:00.368  render           | Rendering frame 1
00:00.368  render           | Start rendering: Scene, ViewLayer
00:00.368  render           | Engine: Cycles
00:02.032  render           | Fra: 1 | Mem: 1M | Synchronizing object | Cube
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:02.032  render           | Fra: 1 | Mem: 1M | Loading render kernels (may take a few minutes the first time)
00:03.638  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building OptiX acceleration structure
00:03.666  render           | Fra: 1 | Mem: 688M | Sample 0/4096
   (the per-sample lines are cut here)
00:03.819  render           | Fra: 1 | Remaining: 00:01.48 | Mem: 688M | Finished
00:04.051  render           | Saved: '/root/the-machine/runs/5.1/e18card_OPTIX/c_0001.png'
00:04.051  render           | Time: 00:03.68 (Saving: 00:00.11)

Blender quit
--- EXIT STATUS: 0
--- SECONDS: 4
--- FILES:
total 8
-rw-r--r-- 1 root root 4634 Aug  7 00:31 c_0001.png
```

and through CUDA, word for word around the lines that matter:

```
00:00.363  render           | Engine: Cycles
00:01.810  render           | Fra: 1 | Mem: 0M | Loading render kernels (may take a few minutes the first time)
00:01.900  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Packing BVH nodes
00:01.919  render           | Fra: 1 | Mem: 688M | Sample 0/4096
00:02.297  render           | Saved: '/root/the-machine/runs/5.1/e18card_CUDA/c_0001.png'
00:02.297  render           | Time: 00:01.93 (Saving: 00:00.11)

Blender quit
--- EXIT STATUS: 0
--- SECONDS: 3
--- FILES:
total 8
-rw-r--r-- 1 root root 4634 Aug  7 00:32 c_0001.png
```

**E1. Cycles renders on a graphics card on a computer with no display server.** Both ways worked, on a machine with no X server, no Wayland and no screen. This is the positive outcome the researcher's own hand test 5 named, and it holds the manual's "no need for X server" line up rather than qualifying it. The same happened in 4.5, 5.0 and 5.2, all on the same machine and the same day.

**E2. What names the device, and how far it goes.** Neither run named the card itself. What the OptiX run printed is `Building OptiX acceleration structure`, and what the CUDA run printed in the same place is `Packing BVH nodes`, so the two ran by different paths. Both reported 688 megabytes of separate memory for the render beside Blender's own nought to one. **A run that finished at all is a run that got the device it asked for**, because C16 shows that when the device is not there Blender says `ERROR Found no Cycles device of the specified type` and stops with exit status 1 rather than quietly falling back to the processor — which is exactly the negative outcome hand test 5 asked about, and it did not happen here.

**[Citation check, 2026-08-07: the label on that last sentence is corrected to inferred.** It is not something either run printed; it is a step taken from a different run, so it does not belong under this section's **observed** label. This check repeated both runs on the repaired machine and got the same figures, and ran the control the hand tests did not: asking for a device this computer has not got gave `Found no Cycles device of the specified type`, exit status 1 and nothing written. So the step holds; the label was the fault.**]**

**E3. A Cycles run printed no display-server complaints at all.** Every EEVEE run on this machine prints `libEGL warning: egl: failed to create dri2 screen` and three `EGL Error (0x3009)` lines before rendering. **Neither of these two Cycles runs printed any of them.** The whole record of all eight runs of this entry was searched for the words error, warning and failed, and nothing matched.

**[Citation check, 2026-08-07: the Cycles half repeated cleanly; the contrast did not, and it is sent back to the hand tests.** Repeating all eight card runs on that machine today and searching every log for error, warning and failed returned nothing, exactly as recorded. But an EEVEE background render on the same machine today printed **no `libEGL` line and no `EGL Error` line at all**, so the sentence about every EEVEE run is what a repeat did not reproduce. The one known difference between the two days is that the graphics card's driver was mismatched then and now reads 580.173.02 on both halves. Neither run is being called right.**]**

**[Hand tests, 2026-08-07: both runs were reproduced, and the contrast is gone — see Part G (a).** With the card's own graphics library, **EEVEE on this machine prints none of those lines either**, and reports it is drawing on the card; with that library hidden and only Mesa's left, all four versions print them word for word and report `llvmpipe`. **So neither run was mistaken, and what decides it is which graphics library Blender gets, not whether there is a display server.** There is now nothing to set these Cycles runs against — what the Cycles half says on its own still stands. **E3's first sentence should be read as a record of what EEVEE did on that machine on 2026-08-06, not as a rule about EEVEE.** The disagreement is recorded, not resolved.**]**

**E4. Loading the render kernels for the first time was quick.** Blender's own line warns it "may take a few minutes the first time". In a settings folder never used before, on this machine, it took about a second and a half.

**Not covered by this.** It says nothing about macOS or Windows, nothing about a machine with more than one card, and nothing about how the picture compares with the same scene rendered on the processor — that was not tried. The times above are recorded, not measured; nothing rests on them.

---

### Part F — run by hand on the Creator's Mac: a second computer, and a window, Blender 5.1

**LABEL: observed.** Added 2026-08-07 from the second sitting of the hand tests, which was run on the Creator's own Mac. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's. It is the second half of the same hand-test list whose Linux half is already recorded above.

**On what kind of computer.** **The Creator's own Mac: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card of any kind.** Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. **It was pointed at a settings folder of its own, `/Users/artist/Documents/Smoozy hand tests/The machine/config-mac2/5.1`, made before it was used**, never at the Creator's own settings folder; a stand-in `HOME` does not move Blender's user folder on macOS, so `BLENDER_USER_RESOURCES` is what was used, and Blender was asked where its user folder actually was and answered with that path. The starting value of Auto Run Python Scripts in that folder was checked rather than assumed and reported `use_scripts_auto_execute = False`. Nothing else was running on the machine, but **no time and no memory was measured here** — this round's timings come from the Linux machine, and the seconds quoted below are only ever used to show where a wait sat, never as a measurement. Date: 2026-08-07.

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

**[Hand tests, 2026-08-07: run again on the Linux machine both ways — see Part G (b).** The way the folder is made unwritable is **not** what the two runs differed over: both ways give the same words. **The bracketed word is untrustworthy in 4.5 as well** — it moves with which graphics library Blender gets, and both wordings were reproduced on demand. The Mac half was not re-run: it needs the Creator's Mac, which is not going anywhere. The disagreement is recorded, not resolved.**]**

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

### Part G — run by hand a third time: the three tests the citation check sent back, Blender 5.1

**LABEL: observed.** Added 2026-08-07. These are entries 28, 29 and 30 of `Audits/Blender Research/The machine/Hand tests — added questions.md` — the three items the citation check sent back that needed the rented Linux machine, run in a third sitting because that machine was about to be destroyed. The full record is in that list.

**On what kind of computer.** The same rented Linux machine as Part C and Part E, named in full: **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. Its card is working and its **kernel driver and driver library agree on 580.173.02**; `nvidia-smi` reported no processes on the card, and the machine was not shared with other work this sitting. Blender 5.1.2 at `/opt/blender/5.1/blender`, **pointed at a settings folder of its own, `/root/again-the-machine/config/5.1`, made before it was used**, never at the real one; it reported `use_scripts_auto_execute = False` as its starting value. All work under `/root/again-the-machine/`, with the prepared files rebuilt by the same builder the earlier sittings used, checked by checksum. Date: 2026-08-07.

**(a) EEVEE renders here on a computer with no display server without printing anything — and prints the `libEGL` lines when it is given a different graphics library.** *(Entry 28. The first of the two results the citation check could not reproduce. Bears on Part C, E3 and Part F (e).)*

`plain.blend` rendered with no window, and the **whole** log searched rather than glanced at. What came back, word for word, the whole run:

```
00:00.354  blend            | Read blend: "/root/again-the-machine/files/5.1/plain.blend"
00:00.362  render           | Rendering single frame
00:00.362  render           | Rendering frame 1
00:00.362  render           | Start rendering: Scene, ViewLayer
00:00.362  render           | Engine: EEVEE
00:02.847  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:02.876  render           | Fra: 1 | Rendering 25 / 64 samples
00:02.902  render           | Fra: 1 | Rendering 50 / 64 samples
00:02.917  render           | Fra: 1 | Rendering 64 / 64 samples
00:03.044  render           | Saved: '/root/again-the-machine/runs/5.1/e28/e_0001.png'
00:03.044  render           | Time: 00:02.68 (Saving: 00:00.11)

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
00:00.358  render           | Engine: EEVEE
libEGL warning: egl: failed to create dri2 screen
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
```

— the whole-log count of lines containing `EGL` was **4** — and asked the same question it now answered `GPU_RENDERER llvmpipe (LLVM 15.0.7, 256 bits)`. The render finished by itself with exit status 0 and wrote its frame either way. The same happened in 4.5, 5.0 and 5.2 on the same machine the same day.

**What this settles.** The words are exactly the words Part C records, and they came back on demand. **So neither run was mistaken about what it saw.** What decides it on this machine is **which graphics library Blender gets**: with the NVIDIA one, EEVEE draws on the card and says nothing; with only Mesa's, it draws in software and prints those four lines. **The reading offered in Part F (e) — that these lines belong to a machine reaching for a display server that is not there — is not what was observed here.**

**What this does not settle.** The mismatched driver of 2026-08-06 cannot be put back, so **nothing here shows what that machine was doing that day**. **The disagreement stands recorded, not resolved, and Part C is left exactly as written.**

**(b) An output folder that cannot be written to: how the folder is made unwritable is not what the two runs differed over.** *(Entry 29. The second of the two results the citation check could not reproduce. Bears on C10 and Part F (f).)*

Two unwritable folders were made two different ways and each probed before use — **way A, a tmpfs mounted and then remounted read-only, which is how C10's folder was made**, and **way B, a read-only bind mount, which is how the citation check's folder was made**. Both refused a write with `Read-only file system`. The same render was aimed into each. **Both ways, this version printed the same two lines**, word for word:

```
00:02.082  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/again-the-machine/readonly-tmpfs/x_0001.png"
00:02.083  reports          | ERROR Render error (Success) cannot save: '/root/again-the-machine/readonly-tmpfs/x_0001.png'
00:02.083  render           | Time: 00:01.68

Blender quit
--- EXIT STATUS: 1
```

Nothing was written, nobody was waited for, and the exit status was 1. **So the way the folder is made unwritable is not what the two earlier runs differed over.** With only Mesa's graphics library, this version **did not move at all** — `ERROR Render error (Success) cannot save: …` again, both ways. **`Success` is what both earlier runs found in this version; there was never a disagreement here.** The disagreement was in 4.5, whose bracketed word does move with the graphics library, and **both of its wordings were reproduced on demand**, so neither run there was mistaken either. **The disagreement stands recorded, not resolved.**

**What this adds.** The bracketed word cannot be trusted to say what went wrong in any version now — 4.5's moves with something that has nothing to do with the failure, and this version's has said `Success` throughout. **The real reason is in the `OpenImageIO write failed` line above it.** And what the answer rests on did not move in any of the eight runs: an error, nothing written, nobody waited for, exit status 1.

**(c) A missing texture is not silent after all — and C11 was measuring nothing.** *(Entry 30. The control the citation check asked for.)*

On the file C11 actually used: rendered once with the image in place, the image deleted, rendered again, and **the two frames compared as pictures**:

```
--- the texture file before the first render:
-rw-r--r-- 1 root root 250 Aug  7 05:41 files/5.1/tex.png
--- now delete it:
ls: cannot access 'files/5.1/tex.png': No such file or directory
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

The cube carries **two** material slots. The one holding the image was appended as slot 1, and every face is on slot 0. **So the deleted image was never in the picture, and C11's silence is silence about a texture that reached nothing.**

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
00:00.374  blend            | Read blend: "/root/again-the-machine/files/5.1/mt2/missingtex2.blend"
00:00.383  render           | Rendering single frame
00:00.383  render           | Rendering frame 1
00:00.383  render           | Start rendering: Scene, ViewLayer
00:00.383  render           | Engine: EEVEE
00:02.241  gpu.texture      | ERROR Failed to create GPU texture from Blender image
00:02.304  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:02.329  render           | Fra: 1 | Rendering 25 / 64 samples
00:02.350  render           | Fra: 1 | Rendering 50 / 64 samples
00:02.361  render           | Fra: 1 | Rendering 64 / 64 samples
00:02.481  render           | Saved: '/root/again-the-machine/runs/5.1/e30e/m_0001.png'
00:02.481  render           | Time: 00:02.09 (Saving: 00:00.11)

Blender quit
--- EXIT STATUS: 0
```

**And the message is tied to the missing file, not to the file being open.** Both runs with an image present were searched for the words texture, not loaded and failed, and returned nothing. **This version marks it `ERROR`, in a column of its own named `gpu.texture`** — which is more than 4.5 does, where the same event prints `GPUTexture: Blender Texture Not Loaded!` with no marking at all.

**What this settles.** **C11's own record stands and is not touched** — it is a true record of what that file did — but it does not answer the question it was written for, and its conclusion that a missing texture is not mentioned is **not what happens on a file where the texture is in the picture**. So an unattended run does have something to watch for here. **What did not change is the half that matters most: nothing waited.** On both files, with the image and without it, the run ended by itself with exit status 0 and wrote its frame — and that half now rests on a test that could have failed.

**One thing this does not settle.** These messages came from EEVEE. Whether Cycles says the same about a missing image was not asked.

**Not covered by any of this.** All of it is Linux, on a machine with no display server. **The Mac half of two of these three entries was not run**: the citation check asked for it, and the Creator's Mac is not going anywhere, so it was left; the reasons are in the hand-test list. Nothing here says what happens on Windows.

---

### Not covered

- Whether the auto-run dialog actually stops a render or merely reports afterwards. My reading of the picture (A1) is inference, not documentation, and the sources never say.
- Whether anything at all is asked when a render would overwrite a file it already wrote (A8). The sources describe the switch and never describe a question, which is silence rather than an answer.
- What happens to any of the waiting things in Part A when there is no interface. This is the gap that matters most for the question as asked, and it is genuinely absent from the sources.
- Whether a render started by hand in the interface satisfies the "user interaction" that Freestyle's scripts are said to need (A2), and whether a render started from the command line does.
- What Blender does when a script tries to read the keyboard and `sys.stdin` is None.
- Anything about the state of the machine itself — login, lock, sleep, service accounts, attached monitors.
- Whether "headless" in the EEVEE limitation means no monitor, no logged-in session, or something else, and whether the same limit applies to Cycles.
- Everything that happens after the render finishes. `bpy.ops.render.play_rendered_anim()` is documented as "Play back rendered frames/movies using an external player" (API, *Render Operators*, same address and date), which would launch another program, but nothing says what that does with nobody there, and it is not part of a render.

**Several of the gaps listed just above have since been closed by test, and only by test — see Part F.** With a window open on the Creator's own Mac, the automatic-execution warning is a dialog in the middle of the main window, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with it unanswered. A window that *is* waiting for a person — a file chooser — did not stop a running animation render either. A still render nobody asks to be saved still writes nothing, though the picture opens in a window of its own. **The sources still say none of this**; what has changed is that it has been watched happening. What is left genuinely open in this section: everything on Windows, and what the machine's own screen locking or display sleeping does to a render, which was not run because it needs the Creator's own computer to itself.

---

### Outside the interface

This is most of the answer rather than a footnote to it, so the findings above are already written that way. In short:

- The one waiting thing the sources describe on the render path (A1) has a documented command-line override that removes the condition causing it: `-y` / `--enable-autoexec`, shown in a `--background --render-anim` example (A3).
- The unsaved-changes pop-up (A4) has a preference switch, and the file-browser trust tick has a direct argument on `open_mainfile` (A7).
- Calls made from a script default to the path that does not ask — `EXEC_DEFAULT` rather than `INVOKE_DEFAULT` (A5).
- Popup menus are documented as explicitly not blocking a script (A6); the confirm and dialog invokes are documented as acting only on a click.
- **What the sources never say is what those dialogs do when there is no window to draw them in.** That is not a small gap for this question and I will not guess at it.
- **It has now been settled for Linux by test rather than by the sources — see Part C.** On a computer with no display server, driven from outside its own window, Blender 5.1 did not wait for anybody on any part of the render path that was tried: the thing A1 describes became text on the terminal and the render carried straight on, A4's pop-up never fired, an existing output file was written over silently, a missing texture was not even mentioned, an unwritable output folder gave an error and exit status 1 rather than a question, Render Audio wrote straight over a file already there, and two Blenders shared one output folder without being asked anything. **Two things do hold a run driven from outside**: a script that reads from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. Standard input in an ordinary background run is a real stream, not nothing, so A9 is not what a computer does here. **Cycles can use a graphics card there** — that was the one thing Part C could not settle, and Part E settles it. Still open from outside: everything on macOS and Windows, and everything about what these same things do when Blender has a window.

**A second operating system has now been tried, and it agreed — see Part F.** Every one of the nineteen entries the Linux machine ran outside Blender's own window was run again on the Creator's own Mac, in all four versions, and **the two machines agreed on every one of them**, down to the same figures where a figure came back. So what is said above about a run driven from outside is no longer true of one operating system only. **The two things that hold such a run hold on macOS too**: a script reading from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. Standard input is a real stream there as well, not nothing. **A render on the Mac also outlives the shell it was started from, and outlives a hang-up signal to its whole process group**, which is what closing a terminal window really sends. Still open from outside: everything on Windows, which no Windows machine has run.

**Three of these results were run a third time, and one of them changes (Part G).** "A missing texture is not mentioned at all" is **not** what happens when the texture is in the picture: Blender 5.1 prints `ERROR Failed to create GPU texture from Blender image`, in a column of its own, and the run still ends by itself with exit status 0 and writes its frame. The entry that reported silence was rendering a file whose textured material sat in a slot no face used, so it was measuring nothing. **So a run driven from outside has one more thing to watch for on its output, beyond an exit status.** Nothing else moves: an unwritable output folder still gives an error rather than a question, writes nothing, waits for nobody and comes back with exit status 1, however the folder is made unwritable — though **the reason it names in brackets cannot be trusted in any version**, this one having said `Success` throughout. And one thing said above about that machine is not a rule about Blender: the `libEGL` lines are not what a machine without a display server prints when it reaches for one. With the card's own graphics library, EEVEE renders there with no display server at all and prints nothing.

**And the window has now been reached, on the Mac — the thing every version of this answer has been leaving open (Part F).** With a window open, the automatic-execution warning is a **dialog in the middle of the main window**, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with the dialog unanswered, and with the first-run Quick Set Up unanswered underneath it. A **file chooser opened on top of a running animation render stopped no frames at all**. A still render nobody asks to be saved still writes nothing — **but the picture opens in a window of its own either way**. A render asked for on the command line with a window open is written at once, **but that Blender then never quits**. A `--background` render **does register with the Mac's window server** for the whole run, though it never comes to the front and draws nothing. And Render Audio is the one place where the menu's way and a script's way part company: **with no path named it opens a chooser and writes nothing; with a path named it writes straight over an existing file without asking** — and that, not the window, is what decides.

### Operating system

The sources are uneven here, and where they are silent I say so.

- `-b` / `--background`, `bpy.app.background`, `-y` / `-Y`, `--python-exit-code` and the auto-run dialog carry no operating-system qualifier anywhere I found, so nothing indicates they differ.
- **Linux is the only system named** for not needing a graphical display: "no need for X server on Linux for example" (B2). The manual says nothing equivalent about Windows or macOS. Do not read the Linux statement as covering them.
- **Windows differs, twice, in writing.** "Headless rendering is not supported on headless Windows systems" for EEVEE (B4), and `-con` is marked "(Windows only)" and is "ignored if -b is set" (B3). Two other arguments are marked Linux-only or macOS-related on the same page (`--no-window-frame`, `--no-native-pixels`), which is about windows on screen rather than about rendering.
- macOS is not named in connection with anything in this answer, and nothing has been tested there either.
- **Linux is now settled by test as well, and only Linux (Part C).** Everything in Part C was got on Ubuntu 22.04.5 LTS on a machine with no display server. It says nothing about macOS, where the same tests have not been run, and nothing about Windows, which no Windows machine has run. So where an answer above says nothing indicates a difference between operating systems, that is still true of the sources; what has changed is that one operating system has now been tried.

**macOS has now been tried as well (Part F), and it agreed with Linux on everything the two machines both ran.** The Creator's own Mac — a MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, with Apple's own graphics and no second card — gave the same answer as the rented Linux machine on all nineteen entries that were run on both, in all four versions. **It also reached the one thing Linux could not: a window.** Two differences between the two machines showed in what was printed, and neither changes an answer: the Mac prints no `libEGL` or `EGL Error` lines, because it has a display server; and Blender 4.5 on the Mac prints a run of `Freestyle:` progress lines that the Linux record does not show.

**The first of those two has since been taken back, by test on the Linux machine (Part G).** The `libEGL` and `EGL Error` lines are **not** the mark of a machine without a display server. On that same machine, with its own card's graphics library, EEVEE renders with no display server at all and prints none of them; with that library hidden and only Mesa's software one left, all four versions print them word for word. **So it is which graphics library Blender gets that decides, not whether there is a display server**, and the Mac's silence is not evidence either way. The Freestyle difference is untouched and stands.

**Windows is still untouched** — no Windows machine has run it — so every statement above that names Windows stays untested. **And one thing on the Mac was not run**: whether the screen locking or the display going to sleep interrupts a render. That needs the Creator's own machine to itself for several minutes, and this sitting had no standing to take it; the test is written out in full in `Audits/Blender Research/The machine/Hand tests — added questions.md`, entry 21.

---

### Where I looked

**Searched across all of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1`** (case-insensitive, whole-tree), for: `background mode`, `--background`, `headless`, `no user interface`, `without a user interface`, `confirm`, `dialog`, `prompt`, `modal`, `popup`, `pop-up`, `interactive`, `waits for`, `waiting for`, `user input`, `user interaction`, `requires interaction`, `unattended`, `overnight`, `idle`, `daemon`, `block until`, `blocks until`, `until you`, `will ask`, `asks you`, `prompts you`, `prompt you`, `ask you`, `asked to`, `requires the user`, `stdin`, `sys.stdin`, `autoexec`, `auto run`, `auto_run`, `use_scripts_auto_execute`, `Trusted Source`, `Allow Execution`, `Overwrite`, `Placeholder`, `Save Prompt`, `already exists`, `check_existing`, `missing file`, `Find Missing`, `Report Missing`, `Esc`, `splash`, `render farm`, `network render`, `SSH`, `remotely`, `over a network`, `display server`, `X server`, `Wayland`, `remote desktop`, `logged in`, `log in`, `session`, `as a service`, `virtual machine`, `screen saver`, `sleep`, `monitor connected`, `no display`.

**Pages I opened in full or in large part:**

- Manual: *Scripting & Security*; *Command Line Arguments* (the whole render, Python, window, audio, debug and "other" sections); *Rendering From The Command Line*; the three *Command Line Launching* pages (Linux, macOS, Windows); *Output Properties*; *Rendering Animations*; *Output Introduction*; *Opening & Saving*; *Recovering Data*; *Save & Load* preferences; *Operators*; *Topbar* (the Render menu); *EEVEE Limitations*; *Drivers Troubleshooting*; *Cycles GPU Rendering*.
- API documentation: `bpy.app`; `bpy.app.handlers`; `bpy.types.WindowManager`; `bpy.ops`; `bpy.ops.render`; `bpy.ops.wm` (`open_mainfile`, `quit_blender`); `bpy.ops.image` (`save_as`); `bpy.types.RenderEngine`; *Tips and Tricks*; *Gotchas: using operators*; *Gotchas: internal data and Python objects*; *Blender as a Python Module*.
- Picture: `_images/advanced_scripting_security_autorun-scripts-dialog.png`, which sits on the *Scripting & Security* page and again on the *Drivers Troubleshooting* page.
- Release notes 5.1: searched all subject files; only *Bug Fixes* had matches, quoted above.
- Cycles' own files: searched every `.py` file for `invoke`, `confirm`, `dialog`, `modal`, `background`, `input(`, `stdin`, `popup`, `wait`. The only match that is about running without an interface is in `engine.py`, line 52: `_cycles.init(path, user_path, bpy.app.background)`. Every other `background` match is the world background or a background light and has nothing to do with this question. Cycles' add-on files define no operator that asks a person anything during a render.
- Requirements page: searched for `background`, `headless`, `display`, `remote`, `server`, `command line`. Only the *Displays* section matched, quoted above, and it is about working at a screen.

**Ruled out and why:**

- The compositor and sequencer pages — I searched the whole tree for the prompting words above and nothing in those sections matched, so I did not open them page by page.
- `--python-console` is a thing that waits for a person, but it is a mode you choose on purpose, not something a render does, so it is reported and set aside.
- The display-driver timeout material on the Cycles GPU page is about the operating system interrupting a long render, not about waiting for a person, so it is out of scope for this question.

---

### Things a person at a computer could settle in a few minutes

**Pruned 2026-08-06 after the hand tests, again 2026-08-07 after the card tests, and again 2026-08-07 after the Mac sitting.** Everything a terminal could reach was run on the rented Linux machine; everything needing a window was then run on the Creator's own Mac. **All of it is answered in Part C, Part E and Part F, and those items are gone from this list.** The full list, with every command written out, is `Audits/Blender Research/The machine/Hand tests — added questions.md`.

**What is left is two things, and neither is about Blender being unwilling.**

1. **Does the screen locking, or the display going to sleep, interrupt a render? — Not run, and it needs the Creator.**
   The only machine this project has with a screen is the Creator's own computer. This test puts that screen to sleep on purpose and then needs the machine left untouched for several minutes, and the Mac sitting had no standing to take the Creator's own desk over that way. It is written out in full as entry 21 of the hand-test list. *What to look at:* whether any frame file carries a written-time from inside the stretch when the screen was asleep, whether the process is still alive at the end, and the last line of the log. **The nearest thing that was answered is that a render on that Mac survived the shell it was started from being killed, and survived a hang-up signal to its whole process group — which is a different and smaller event.**

2. **Everything on Windows. — Cannot be run.**
   No Windows machine has run it, so the EEVEE headless-Windows statement, the Windows crash pop-up, and the two Windows-only command-line arguments all stay untested rather than answered. Each is written out in the hand-test list, ready for the next Windows rental.

---

2026-08-06 — Opus 5 (Claude Code) — The machine, Blender 5.1

2026-08-07 — Opus 5 (Claude Code) — the card tests for The machine, added questions

2026-08-07 — Opus 5 (Claude Code) — the Mac hand tests for The machine, added questions

2026-08-07 — Opus 5 (Claude Code) — the tests the citation check sent back for The machine, added questions

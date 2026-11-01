# The machine — added questions, second pass (Blender 5.1)

**Sources consulted, and when.** Blender's 5.1 manual, Blender's 5.1 Python API documentation, Blender's 5.1 release notes and Cycles' own 5.1 files, all as held in `Blender Sources/`, snapshot taken 2026-07-30 (used as the date read for all four). Blender's requirements page as held in `Blender Sources/requirements/requirements_2026-08-01.txt`, read 2026-08-01. Pictures from the manual's `_images` folder, same date as the manual.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

**Scope.** Only question 7 of the sheet is answered here. Every other question on the sheet was left alone.

---

## 7. Can anything a render does wait for a person, and what happens on a computer with nobody at it?

### The question was ambiguous — the reading I took

"Can anything a render does wait for a person" can be read two ways:

- **The reading I answer:** does any step of rendering stop and hold until a person answers something (a dialog, a prompt, a file chooser), so that a render started on an unwatched machine could sit there forever?
- **The other reading:** can a render be *made* to wait for a person on purpose — a pause-and-resume. I did not find that reading supported anywhere in the sources either; the closest documented things are stopping a render and restarting it later from where it left off (quoted below), which is a restart, not a wait.

I answer the first reading throughout, and note the second here so nobody assumes it was covered.

### Short answer

**LABEL: mixed — see each part below.**

The sources describe exactly one place where something Blender does can hold for a person's answer that a render can run into: the security dialog for automatic execution of Python scripts, which is refused by default. Everything else about rendering that I could find is described as running to completion without asking anybody anything, and there is a command-line mode built for exactly the case of nobody being there. What the sources do **not** say anywhere is what becomes of that security dialog — or of any other dialog — when there is no interface to show it in. That is the real gap in this answer.

---

### Part A — the one documented thing that waits: automatic execution of Python scripts

**LABEL: documented** (that the dialog exists, that it offers a choice, that automatic execution is off by default, and that a render can need scripts).

Exact words, from Blender's manual, *Scripting & Security*:

> "Automatic execution is disabled by default; however, some blend-files need this to function properly."

> "When a blend-file tries to execute a script and is not allowed, a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html`. Date read: 2026-07-30.

The picture the manual places right there is `_images/advanced_scripting_security_autorun-scripts-dialog.png`. I opened it. **LABEL: documented** — visible in it: the text "For security reasons, automatic execution of Python scripts in this file was disabled:", beneath it "Text 'Text.py'" and "This may lead to unexpected behavior", a checkbox "Permanently allow execution of scripts", and two buttons, "Allow Execution" and "Ignore". So the thing genuinely presents a person with a choice and two buttons.

The same picture is used again on the drivers page, with this caption:

> "An Auto-run warning in the Info editor's header."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/animation/drivers/troubleshooting.html`. Date read: 2026-07-30.

Note the two pages describe the same thing differently — one calls it "a dialog", the other places it in an editor's header. *(Citation check, 2026-08-07: both quotations are at the addresses given. Worth adding that the same "Info editor's header" caption also sits on the Scripting & Security page itself, at `manual_5.1/advanced/scripting/security.txt` line 15 — so the manual contradicts itself on one page as well as between two.)* I am not able to settle from the sources whether it blocks the program or merely sits there offering the choice. I have not assumed either.

That this can bite a render, in the sources' own words:

> "By default Blender will restrict execution of Python scripts."

> "If using a Scripted Expression Driver Type that doesn't follow the Simple Expressions subset, you will have to open the file as Trusted Source, or set Auto Run Python Scripts in Preferences ‣ Save & Load ‣ Blender Files."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/animation/drivers/troubleshooting.html`. Date read: 2026-07-30.

And the command line has its own switches for precisely this, with the default stated:

> "-y, --enable-autoexec — Enable automatic Python script execution."

> "-Y, --disable-autoexec — Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`. Date read: 2026-07-30.

The security page says outright that this is what you reach for when nobody is there:

> "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them"

and gives the worked example:

> "blender --background --enable-autoexec my_movie.blend --render-anim"

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html`. Date read: 2026-07-30.

One thing the same page says that cuts the other way — some scripts run during a render regardless of the auto-execution setting:

> "There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off) […] Rendering with Freestyle, because Freestyle uses scripts to control line styles."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html`. Date read: 2026-07-30.

**LABEL: inferred** — that starting the render is itself the "user interaction" the sentence means, so a Freestyle render's own style-module scripts do not trip the security dialog. The step: the page lists Freestyle rendering under "Manual Execution", defined as ways that "require user interaction (therefore will run even when auto execution is off)". The page never says what "user interaction" is in the Freestyle case, and never says what happens when the render is started without a person, e.g. from the command line. A reader has to supply that, so this is inferred, not documented.

**Not covered by Part A:** whether the dialog blocks the program or only offers a choice; what happens to it when Blender runs with no interface (nothing in the sources says); whether a render that needs a refused script fails, renders wrongly, or waits; whether the Freestyle exemption still holds when the render was not started by a person.

---

### Part B — everything else about a render is described as not asking anybody anything

**LABEL: documented** for each of the following individually; **LABEL: not in the sources** for the general claim that *nothing else* waits, since the sources nowhere make that general statement and I am only reporting that I could not find a counter-example.

Things a render does that the sources describe as proceeding without a question:

**Writing output files, including over files that already exist.** The choice is a setting, not a prompt:

> "Overwrite existing files when rendering."

> "Create empty placeholder frames while rendering."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/render/output/properties/output.html`. Date read: 2026-07-30. (In the plain-text snapshot both of these sit under the same mangled heading line, "Image Sequence – Overwrite Image Multi-Layer EXR"; the first sentence is the Overwrite option, the second is the Placeholders option.)

The same page then builds a multi-machine arrangement out of exactly that, with nobody deciding anything per frame:

> "An easy way to get multiple machines to share the rendering workload is to:" / "Set up a shared directory over a network file system." / "Disable Overwrite, enable Placeholders in the Render Output panel." / "Start as many machines as you wish rendering to that directory."

Same page and date.

**Stopping and picking up again is a restart, not a wait.**

> "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/render/output/introduction.html`. Date read: 2026-07-30.

> "Just disable the Overwrite option to start rendering where you left off."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/render/output/animation.html`. Date read: 2026-07-30.

**Rendering while a person is not using the machine is the manual's own advice.**

> "Rendering takes all available CPU time; you should render overnight, when the computer is not needed, or set Blender to a low priority while rendering, and work on other things (be careful with the RAM space!)."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/render/output/animation.html`. Date read: 2026-07-30.

**A render inside the interface does not need a window dismissed.** The preference that decides what the interface does during a render includes an option where it does nothing at all:

> "Keep User Interface: The user interface does not change and the render is computed in the background."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/editors/preferences/interface.html`. Date read: 2026-07-30. The other options listed on the same page open a temporary Image editor (maximised, in place of the largest area, or as a new window). The page does not say any of them must be closed by a person, and does not say a render waits on them.

**Blender not responding during a render is not a request for a person.** The manual's own account of it is about the graphics card, not about input:

> "On older GPU generations, graphics cards can only either render or draw the user interface. This can make Blender unresponsive while it is rendering."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`. Date read: 2026-07-30.

**Not covered by Part B:** what happens when the output path cannot be written to, when a linked file or texture is missing at render time, when the machine runs out of memory or disk mid-render, or when a movie file being written already exists — I searched for all four (terms below) and the sources do not describe any of them as prompting, but they do not describe them at all, so I cannot say they do not.

---

### Part C — what the sources say about a computer with nobody at it

**LABEL: documented.**

There is a mode built for it, and it is named for the absence of an interface:

> "-b, --background — Run in background (often used for UI-less rendering)."

> "The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`. Date read: 2026-07-30.

The Python API states the same fact from the other side:

> "bpy.app.background — Boolean, True when blender is running without a user interface (started with -b)"

Source: Blender Python API, `https://docs.blender.org/api/5.1/bpy.app.html`. Date read: 2026-07-30.

And the manual says plainly that no screen is needed:

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

> "-b — Render in the background (without UI)."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html`. Date read: 2026-07-30.

The interface machinery is simply absent in that mode, which the API documentation says in passing more than once:

> "Note that in background mode (no GUI available), keyconfigs are not available either, so we have to check this to avoid nasty errors in background case."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html`. Date read: 2026-07-30.

The API documentation carries an equivalent line:

> "return  # Can be None in background mode."

(commented against `wm.keyconfigs.addon`.) Source: Blender Python API, `https://docs.blender.org/api/5.1/bpy.types.KeyMaps.html`. Date read: 2026-07-30.

And:

> "The current space, may be None in background-mode, when the cursor is outside the window or when using menu-search (readonly)"

Source: Blender Python API, `https://docs.blender.org/api/5.1/bpy.types.Context.html`. Date read: 2026-07-30.

Two more things bear directly on an unwatched machine.

**Calling a tool from Python does not, by default, ask a person anything.**

> "EXEC_DEFAULT is used by default, running only the execute() method, but you may want the operator to take user interaction with INVOKE_DEFAULT which will also call invoke() if existing."

Source: Blender Python API, `https://docs.blender.org/api/5.1/bpy.ops.html`. Date read: 2026-07-30.

**A pop-up raised from Python cannot be waited on.**

> "Note that they will not block the scripts execution, so the caller can't wait for user input."

(on `WindowManager.popup_menu`.) Source: Blender Python API, `https://docs.blender.org/api/5.1/bpy.types.WindowManager.html`. Date read: 2026-07-30.

**One documented failure of the unwatched case, and it is a rendering one.** For EEVEE:

> "Headless Rendering" / "Headless rendering is not supported on headless Windows systems."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html`. Date read: 2026-07-30. The page does not define "headless Windows system" and does not say what happens when you try.

**A crash on an unwatched machine can put a window on the screen, on one operating system.**

> "After a crash, the macOS Crash Reporter shows a window with backtrace after some time, or when Blender is opened again."

Source: Blender manual, `https://docs.blender.org/manual/en/5.1/troubleshooting/crash.html`. Date read: 2026-07-30. The same page describes Windows and Linux crashes as writing files (`test.crash.txt`, `blender.crash.txt`) with no window mentioned. There are also command-line switches `--disable-crash-handler` and `--disable-abort-handler`, and `--debug-exit-on-error` ("Immediately exit when internal errors are detected"), from `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`, date read 2026-07-30.

**Release notes.** One entry in 5.1's fixed-bugs list touches unwatched rendering directly:

> "Show currently rendered frame in background mode"

Source: Blender release notes, `https://developer.blender.org/docs/release_notes/5.1/bugfixes/`. Date read: 2026-07-30. (Marked in the list as backported to 5.0.1.) This is a statement that the behaviour changed; it does not describe waiting.

One other 5.1 release note is about a dialog that waits, though not one a render is documented to raise:

> "On macOS, scripts and extensions can now access the device camera, microphone and system audio. A system permission dialog will be prompted to the user."

Source: Blender release notes, `https://developer.blender.org/docs/release_notes/5.1/python_api/`. Date read: 2026-07-30. **LABEL: inferred**, and weakly — a render does run Python (drivers, handlers, Freestyle style modules), so a script that touched those devices during a render on macOS would meet this prompt. The step a reader has to supply: that such a script runs during a render at all. Nothing in the sources connects this to rendering.

**Cycles' own files.** **LABEL: observed.** Cycles is told whether Blender has an interface, at start-up. The exact line, from `cycles_5.1/engine.py`, line 52, inside `def init():`

```
    _cycles.init(path, user_path, bpy.app.background)
```

That is the only place in Cycles' Python files where the presence or absence of an interface appears; I searched all of `cycles_5.1/*.py` for "background" and every other hit is about the background of a *scene* (background light, background sampling, "extra background CPU usage"). What Cycles then does with that flag is inside `_cycles`, which is not one of the files I was given, so I cannot say. Nothing in Cycles' Python files asks a person anything.

**Not covered by Part C:** nothing in any of the sources addresses whether the machine must be logged in, whether a screen must be attached, what a screen lock, a screensaver, sleep or power management does to a running render, or whether Blender can run as a background service. I searched for all of these (terms below) and found nothing at all — not a partial answer, an absence.

---

### Part D — run by hand: what a computer actually did, Blender 5.1

**LABEL: observed.** Added 2026-08-06 from the hand tests. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's.

**On what kind of computer.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **with no display server at all** — no X server, no Wayland, `DISPLAY` empty, no X socket, no X or Wayland process. Blender 5.1.2 at `/opt/blender/5.1/blender`, **pointed at a settings folder of its own, `/root/the-machine/config/5.1`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. The machine was **shared with other topics' tests throughout**, so no time below is a clean measurement. Date: 2026-08-06.

**D1. The security dialog of Part A does not stop a run with no window.** *(Entry 3.)* A file was made holding the factory cube with a scripted-expression driver on its X position reading `__import__('math').floor(4.5)`, and rendered twice with no window. Refusing, word for word:

```
00:00.347  blend            | Read blend: "/root/the-machine/files/5.1/driver2.blend"
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support

Error in PyDriver: expression failed: __import__('math').floor(4.5)
For target: (type=Object, name="Cube", property=location, property_index=0)

00:00.355  render           | Rendering single frame
00:14.782  render           | Saved: '/root/the-machine/runs/5.1/e3r_a/d_0001.png'
```

exit status 0. Allowing wrote a frame too, exit status 0, and the pictures — compared as pictures, not as files — came back `PIXELS_DIFFER max_channel_difference=0.517647`. So what Part A calls "the one documented thing that waits" does not wait when there is no window: it becomes text on the terminal, and the render carries straight on with a scene whose driver did not run. Whether it blocks *in* a window is still not settled; that is the one entry left for a machine with a screen.

**D2. A run can read its own state.** *(Entry 3.)* Refused: `AUTOEXEC_FAIL True`, `AUTOEXEC_FAIL_MESSAGE "Driver '__import__('math').floor(4.5)'"`, `CUBE_X 0.0`. Allowed: `AUTOEXEC_FAIL False`, empty message, `CUBE_X 4.0`.

**D3. A refused registered text data-block behaves the same way.** *(Entry 4.)* The run printed `scripts disabled for "/root/the-machine/files/5.1/regtext.blend", skipping 'Text.py'` and went on to write its frame with exit status 0; allowing it, the text's own line `REGISTERED TEXT RAN` appeared.

**D4. The four cases Part B says it cannot reach have now been reached, and none of them asks anything.** *(Entries 12, 14, 15, 19.)*

- *A file already on disk where a frame is going.* Rendered twice to the same path: nothing was asked, the written-time moved, and the picture was `PIXELS_IDENTICAL max_channel_difference=0.000000` with the first. A third run with Overwrite off printed `00:00.464  reports          | No frames rendered, skipped to not overwrite` and left the file alone, which is the control showing the switch was being read.
- *A texture the scene needs, missing.* The run ended by itself with exit status 0 and wrote its frame, and nothing whatever was printed about the missing file.
- *An output path that cannot be written to.* Word for word: `00:11.490  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/the-machine/readonly/x_0001.png"` and `00:11.490  reports          | ERROR Render error (Success) cannot save: '…'`, **exit status 1**, nothing written, no waiting. This is the one place in the whole set where a failed unattended run told the outside world about it through its exit status.
- *A sound file already there.* `bpy.ops.sound.mixdown(filepath=…)` returned `{'FINISHED'}` in about a second and wrote straight over it.

**D5. The unsaved-changes pop-up of Part C's list does not fire with no window.** *(Entries 6 and 7.)* A file changed and never saved, reaching Blender's own exit, exited by itself with status 0, asked nothing, and left the file on disk with the same checksum. A second file loaded over unsaved changes printed `SECOND FILE LOADED` and carried straight on.

**D6. A script inside Blender can hold an unattended run open for ever.** *(Entries 9 and 10. This is the one thing found that waits.)* With and without `--debug`, Blender printed the same thing, which is not nothing: `STDIN IS <_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>`. A script calling `input()` then failed at once with `EOFError: EOF when reading a line` when nothing was on standard input — and with something holding standard input open and never sending anything, the same command was **still sitting there sixty seconds later**, stopped only by a time limit.

**D7. A script hooked into the render holds the render for as long as it runs.** *(Entry 16.)* A script waiting forty-five seconds was added to `bpy.app.handlers.render_pre`: `HANDLER STARTED`, then `HANDLER FINISHED`, then the frame saved, then `RENDER RETURNED`, 69 seconds in all where the same scene rendered alone in eleven to sixteen.

**D8. A still render writes nothing unless asked.** *(Entry 11.)* `bpy.ops.render.render()` with the output path set and printed left the folder empty; `bpy.ops.render.render(write_still=True)` wrote `s_.png`.

**D9. Two Blenders share one output folder without being asked anything.** *(Entry 13.)* Two runs of the same forty-frame animation started at once into one folder with Overwrite off and Placeholders on. Both ended by themselves with exit status 0, all forty frames were there and none was empty, and each skipped what the other had taken. **This is not the arrangement the manual describes**, which has several machines sharing a folder over a network file system.

**D10. Freestyle changes the picture with automatic script execution switched off.** *(Entry 20.)* The same scene saved with Freestyle on and off, both rendered with `--disable-autoexec`: both ended by themselves, neither said anything about scripts, the Freestyle run printed `00:10.756  render           | Rendering freestyle` as a step of its own, and `PIXELS_DIFFER max_channel_difference=0.658824`.

**D11. A render survives the connection it was started from being cut.** *(Entry 17. The second half of the researcher's own check 8.)* An animation render started over an SSH connection which was then killed after forty-five seconds was still running on a fresh connection with 4 frames written, and ninety seconds later with 13.

**D12. EEVEE renders on a computer with no display server; Cycles could not be tried on this day.** *(Entries 1 and 18.)* Every render above used EEVEE, which is what the factory settings choose here. Cycles was asked for through OptiX and through CUDA and printed `ERROR CUDA cuInit: Unknown error` and `ERROR Found no Cycles device of the specified type`, then gave the prompt back in about a second with exit status 1. The machine's own `nvidia-smi` reported `Failed to initialize NVML: Driver/library version mismatch` throughout, **so this is a fault in the machine, not a finding about Blender**. The card was repaired afterwards and the entry was run again — see Part E. **One thing this run settles on its own**: when the device asked for is not there, Blender says so and stops rather than quietly using the processor.

**Not covered by any of this.** Every result above is from Linux on a machine with no display server. Nothing here says what happens on macOS or on Windows, and nothing here says what any of these things do when Blender has a window. Nothing here touches whether a machine must be logged in, or what a screen lock or sleep does, because this machine has no screen to lock.

**[Citation check, 2026-08-07: two notes on the entries above.** *The unwritable output folder:* repeating that entry in 4.5 on the same machine, with the folder made read-only by a read-only bind mount, gave a different word in the brackets from the one 4.5's own record gives, so the bracketed reason goes back to the hand tests for every version — the behaviour it rests on (an error, nothing written, nobody waited for, exit status 1) matched. *The missing texture:* as written that test has no control showing the deleted image was doing anything in the render. The control was added on a repeat in 4.5 and came back `PIXELS_DIFFER max_channel_difference=0.694118`; the control belongs in this version's record too and is sent to the hand tests.**]**

**[Hand tests, 2026-08-07: both went back to the machine, in this version — see Part G.** *The unwritable output folder:* the folder was made unwritable both ways and **the two ways gave the same words as each other**, so that is not what the two runs differed over; this version said `Success` on both ways and on both graphics paths, as both earlier runs found. *The missing texture:* the control was added to the file D4 used, **and it failed** — the two pictures are the same picture, because the textured material sits in a slot no face uses. With that put right the control fires, and **this version then prints `ERROR Failed to create GPU texture from Blender image`**, so a missing texture is mentioned when it is in the picture. Both records above are left exactly as written.**]**

---

### Part E — run by hand: Cycles on the graphics card, with no display server, Blender 5.1

**LABEL: observed.** Added 2026-08-07. This is entry 18 of `Audits/Blender Research/The machine/Hand tests — added questions.md`, run a second time after the card was repaired. It is the one thing Part D could not settle.

**On what kind of computer.** The same rented Linux machine as Part D, with its graphics card now working: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **still with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. The machine's **kernel driver and its driver library now agree on 580.173.02**; they had drifted apart on the day of Part D, which is why that entry failed then. Blender 5.1.2 at `/opt/blender/5.1/blender`, **pointed at a fresh settings folder of its own, `/root/the-machine/config-card/5.1`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. `nvidia-smi` reported no processes and no memory in use on the card immediately before and immediately after, so nothing else was sharing it. Date: 2026-08-07.

**What was done.** The factory cube, camera and light at 64 by 64, rendered with no window twice, once through each way the card can be reached: `--engine CYCLES … -- --cycles-device OPTIX`, then the same with `CUDA`.

**What came back. Both runs rendered on the card, wrote their picture, and gave the prompt back by themselves with exit status 0.** Nothing waited, nothing was asked. Through OptiX, word for word around the lines that matter, with the repeated per-sample lines cut:

```
00:00.368  render           | Engine: Cycles
00:02.032  render           | Fra: 1 | Mem: 1M | Loading render kernels (may take a few minutes the first time)
00:03.638  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building OptiX acceleration structure
00:03.666  render           | Fra: 1 | Mem: 688M | Sample 0/4096
00:04.051  render           | Saved: '/root/the-machine/runs/5.1/e18card_OPTIX/c_0001.png'
00:04.051  render           | Time: 00:03.68 (Saving: 00:00.11)

Blender quit
--- EXIT STATUS: 0
--- SECONDS: 4
```

and through CUDA:

```
00:01.900  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Packing BVH nodes
00:01.919  render           | Fra: 1 | Mem: 688M | Sample 0/4096
00:02.297  render           | Saved: '/root/the-machine/runs/5.1/e18card_CUDA/c_0001.png'
00:02.297  render           | Time: 00:01.93 (Saving: 00:00.11)

Blender quit
--- EXIT STATUS: 0
--- SECONDS: 3
```

Both wrote a picture of 4634 bytes. The same happened in 4.5, 5.0 and 5.2 on the same machine and the same day.

**So: Cycles renders on a graphics card on a computer with no display server.** No run named the card itself; what names the path is `Building OptiX acceleration structure` in one and `Packing BVH nodes` in the other, and both reported 688 megabytes of separate memory for the render. A run that finished at all got the device it asked for, because D12 shows Blender stops with an error rather than quietly using the processor when the device is not there.

**[Citation check, 2026-08-07: the label on that last sentence is corrected to inferred.** It is not something either run printed; it is a step taken from a different run, so it does not belong under this section's **observed** label. This check repeated both runs on the repaired machine and got the same figures, and ran the control the hand tests did not: asking for a device this computer has not got gave `Found no Cycles device of the specified type`, exit status 1 and nothing written. So the step holds; the label was the fault.**]**

**Two smaller things.** Neither Cycles run printed the `libEGL warning` or `EGL Error (0x3009)` lines that every EEVEE run on this machine prints. And loading the render kernels for the first time, in a settings folder never used before, took about a second and a half, not the "few minutes" Blender's own message warns of.

**[Citation check, 2026-08-07: the Cycles half repeated cleanly; the contrast did not, and it is sent back to the hand tests.** Repeating all eight card runs on that machine today and searching every log for error, warning and failed returned nothing, exactly as recorded. But an EEVEE background render on the same machine today printed **no `libEGL` line and no `EGL Error` line at all**, so the sentence about every EEVEE run is what a repeat did not reproduce. The one known difference between the two days is that the graphics card's driver was mismatched then and now reads 580.173.02 on both halves. Neither run is being called right.**]**

**[Hand tests, 2026-08-07: both runs were reproduced, and the contrast is gone — see Part G.** With the card's own graphics library, **EEVEE on this machine prints none of those lines either**, and reports it is drawing on the card; with that library hidden and only Mesa's left, all four versions print them word for word and report `llvmpipe`. **So neither run was mistaken, and what decides it is which graphics library Blender gets, not whether there is a display server.** There is now nothing to set these Cycles runs against — what the Cycles half says on its own still stands. The disagreement is recorded, not resolved.**]**

**Not covered by this.** Nothing about macOS or Windows, nothing about more than one card, and no comparison with the same scene rendered on the processor. The times are recorded, not measured.

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

**[Hand tests, 2026-08-07: run again on the Linux machine both ways — see Part G.** The way the folder is made unwritable is **not** what the two runs differed over: both ways give the same words. **The bracketed word is untrustworthy in 4.5 as well** — it moves with which graphics library Blender gets, and both wordings were reproduced on demand. The Mac half was not re-run: it needs the Creator's Mac, which is not going anywhere. The disagreement is recorded, not resolved.**]**

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

**On what kind of computer.** The same rented Linux machine as Part D and Part E, named in full: **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. Its card is working and its **kernel driver and driver library agree on 580.173.02**, with no other work on the card or the machine. Blender 5.1.2 at `/opt/blender/5.1/blender`, **pointed at a settings folder of its own, `/root/again-the-machine/config/5.1`, made before it was used**, which reported `use_scripts_auto_execute = False` as its starting value. All work under `/root/again-the-machine/`. Date: 2026-08-07.

**1. The `libEGL` lines are about which graphics library Blender gets, not about a missing display server.** *(Entry 28.)* `plain.blend` rendered with no window, and the **whole** log searched:

```
00:00.354  blend            | Read blend: "/root/again-the-machine/files/5.1/plain.blend"
00:00.362  render           | Start rendering: Scene, ViewLayer
00:00.362  render           | Engine: EEVEE
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:03.044  render           | Saved: '/root/again-the-machine/runs/5.1/e28/e_0001.png'
00:03.044  render           | Time: 00:02.68 (Saving: 00:00.11)

Blender quit
--- EXIT STATUS: 0
--- how many lines of the whole log contain libEGL:
0
--- how many lines of the whole log contain EGL:
0
```

Asked what it was drawing with, it answered `GPU_RENDERER NVIDIA GeForce RTX 4080 SUPER/PCIe/SSE2` on `4.6.0 NVIDIA 580.173.02`. **So EEVEE gets a graphics context on the card on a machine with no display server, and says nothing about it.** Then the same render with the NVIDIA graphics library hidden and only Mesa's left, nothing else changed — **and the lines came straight back**, after the engine line:

```
00:00.358  render           | Engine: EEVEE
libEGL warning: egl: failed to create dri2 screen
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
```

— the whole-log count of lines containing `EGL` was **4** — with `GPU_RENDERER llvmpipe (LLVM 15.0.7, 256 bits)`. The same happened in the other three versions. **So neither of the two disagreeing runs was mistaken about what it saw.** The mismatched driver of 2026-08-06 cannot be put back, so **nothing here shows what that machine was doing on that day, and the disagreement is recorded, not resolved** — D12 and Part E are left exactly as written.

**2. An unwritable output folder: how the folder is made unwritable is not what the two runs differed over.** *(Entry 29.)* Two folders made two ways — a tmpfs remounted read-only, which is how D4's folder was made, and a read-only bind mount, which is how the citation check's was made — each probed first and each refusing a write with `Read-only file system`. **Both ways, this version printed the same two lines:**

```
00:02.082  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/again-the-machine/readonly-tmpfs/x_0001.png"
00:02.083  reports          | ERROR Render error (Success) cannot save: '/root/again-the-machine/readonly-tmpfs/x_0001.png'
00:02.083  render           | Time: 00:01.68

Blender quit
--- EXIT STATUS: 1
```

With only Mesa's graphics library this version **did not move at all** — `Success` again, both ways. **`Success` is what both earlier runs found here; the disagreement was in 4.5 alone**, whose bracketed word does move with the graphics library, and **both of its wordings were reproduced on demand**. **The disagreement is recorded, not resolved.** In all eight runs the part the answer rests on held: an error printed, nothing written, nobody waited for, exit status 1. **The bracketed word cannot be trusted to say what went wrong in any version; the real reason is in the `OpenImageIO write failed` line above it.**

**3. A missing texture is not silent — and the entry that said so was measuring nothing.** *(Entry 30.)* The control was added to the file D4 used, and **it failed**: in all four versions the picture with the image and the picture without it are the same picture, `PIXELS_IDENTICAL max_channel_difference=0.000000`. The file was asked why:

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
00:00.374  blend            | Read blend: "/root/again-the-machine/files/5.1/mt2/missingtex2.blend"
00:00.383  render           | Start rendering: Scene, ViewLayer
00:00.383  render           | Engine: EEVEE
00:02.241  gpu.texture      | ERROR Failed to create GPU texture from Blender image
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:02.481  render           | Saved: '/root/again-the-machine/runs/5.1/e30e/m_0001.png'
00:02.481  render           | Time: 00:02.09 (Saving: 00:00.11)

Blender quit
--- EXIT STATUS: 0
```

Both runs with an image present were searched for the words texture, not loaded and failed and returned nothing, so the message is tied to the missing file. **D4's own record stands and is not touched, but its conclusion that a missing texture is not mentioned is not what happens when the texture is in the picture** — and this version marks it `ERROR`, in a column of its own. What did not change, and now rests on a test that could have failed: **nothing waited** — every run ended by itself with exit status 0 and wrote its frame.

**Not covered by any of this.** All of it is Linux, on a machine with no display server. **The Mac half of two of these three entries was not run**: the citation check asked for it, and the Creator's Mac is not going anywhere, so it was left. These messages came from EEVEE; Cycles was not asked.

---

### Where I looked

**Searched across the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` for:** background mode, background-mode, in background, background rendering, headless, no user interface, without a user interface, dialog, prompt, popup, pop-up, modal, confirmation, user interaction, user input, wait for, waits for, waiting for, will ask, ask you, asks you, requires user, require user, manual intervention, blocking, halt, interrupt, unresponsive, freeze, Lock Interface, batch, batch render, render farm, renderfarm, unattended, attended, overnight, nobody, no one, notification, beep, progress bar, exit code, overwrite, placeholder, missing file, Report Missing Files, Find Missing, cannot be found, not found, screensaver, screen saver, sleep, idle, power management, hibernate, session, log out, logout, service, daemon, SSH, terminal, no display, display server, X server, Wayland, logged, remote, online access, autoexec, auto exec.

**Pages opened in full or in substantial part:** Scripting & Security; Drivers ‣ Troubleshooting; Command Line Arguments (all sections: render, playback, window, Python, logging, debug, GPU, misc, argument parsing, environment variables); Rendering From The Command Line; Deploying Blender in Production; Render Output ‣ Introduction, Rendering Animations, Output properties, Metadata; EEVEE ‣ Limitations (index and page); Cycles ‣ GPU Rendering (whole page including its FAQ); Compositing ‣ Limitations; Preferences ‣ Interface (Temporary Editors); Topbar (Render menu, File ‣ External Data, Clean Up); Splash Screen; Troubleshooting ‣ Crashes; Troubleshooting ‣ Recovering Data; File Browser; Files ‣ Open & Save. In the API documentation: `bpy.app`, `bpy.app.handlers`, `bpy.ops`, `bpy.types.WindowManager`, `bpy.types.Operator`, `bpy.types.Context`, `bpy.types.KeyMaps`, `bpy.types.RenderEngine`, `bpy.utils`, `info_gotchas_operators`, `info_gotchas_internal_data_and_python_objects`, `info_tips_and_tricks`, `info_advanced_blender_as_bpy`. Release notes: every file in `release_notes_5.1` searched; `bugfixes.md`, `python_api.md`, `user_interface.md`, `core.md`, `cycles.md`, `corrective_releases.md` inspected around their hits. Cycles: `engine.py` read in full; `properties.py`, `ui.py`, `__init__.py`, `operators.py`, `osl.py`, `presets.py`, `camera.py`, `version_update.py` searched.

**Picture opened:** `_images/advanced_scripting_security_autorun-scripts-dialog.png`, from the Scripting & Security page.

**Ruled out, and why:**

- *Freestyle pages* (`render/freestyle/…`, including its Python page) — they describe style modules and the Python API for them, and never mention automatic execution, security, or anything that waits. So the only statement tying Freestyle to script security is the one on the Scripting & Security page, quoted above.
- *The confirmation pop-ups that Blender does have* — discarding unsaved changes when opening a file ("a pop-up will ask you to either confirm discarding those changes, or save them", `https://docs.blender.org/manual/en/5.1/files/blend/open_save.html`), the Save Prompt preference, overwriting a blend-file saved by a different version, and the File Browser ("The File Browser will appear in a new window, wait for you to select a file, and then close again", `https://docs.blender.org/manual/en/5.1/editors/file_browser.html`). All read 2026-07-30. Every one of these belongs to opening, saving or choosing a file. None is documented as part of rendering, so I did not count them as an answer to this question — but they are the proof that Blender does have things that wait, which is why the silence about the interface-less case matters.
- *The splash screen and its first-run "Quick Set Up Process"* (`https://docs.blender.org/manual/en/5.1/interface/window_system/splash.html`, read 2026-07-30) and the "Online Access" prompt (`https://docs.blender.org/manual/en/5.1/editors/preferences/system.html`; the API records "The user has been shown the 'Online Access' prompt and made a choice") — these are start-up, not rendering, and the manual says the splash is dismissed by clicking outside it or pressing Esc. Ruled out of the answer, noted here because they are things a first run on a fresh machine puts in front of a person.
- *The requirements page* — it never names Blender 5.1, so by its own nature it speaks for some other release. Read 2026-08-01, `https://www.blender.org/download/requirements/`. For completeness: it says nothing about waiting, dialogs, background rendering, headless machines or logged-in sessions at all. Its only near-miss is a "Displays" section recommending 1920x1080 "for optimal use", which is about a person using Blender, not about a render.
- *`--enable-event-simulate`* (`bpy.types.Window.event_simulate`) — this is the one command-line switch about supplying input events without a person, but the manual describes it only as a "testing feature", and says nothing about rendering. Not an answer to the question; recorded so nobody thinks it was missed.

**Searched for and found nothing whatsoever:** screensaver, sleep, power management, hibernation, logging out, running as a service or daemon, whether a user must be logged in, whether a monitor must be attached. These words do not appear in any relevant sense anywhere in the 5.1 manual, API documentation or release notes.

---

### Outside the interface

This is the heart of the question, and the sources answer it only halfway.

What is documented: there is a mode with no interface at all (`-b` / `--background`), it is named in the manual as the one for rendering, it needs no graphical display and works over a remote shell, and the security switches `-y` / `-Y` exist specifically so the automatic-execution decision can be made on the command line instead of by a person. That is Blender telling you, plainly, that the intended way to render with nobody there is to take the decision in advance on the command line.

What is **not** documented anywhere I could find: what happens to the auto-execution dialog — or to any dialog — when there is no interface to draw it in. The manual says the dialog appears; it never says it does not appear in background mode, and never says what the program does instead. The nearest thing to a statement is that the Preferences "are still used" in that mode and that you "may want to override them", which implies a decision is taken without asking, but does not say so. **LABEL: not in the sources.**

**It has now been settled for Linux by test rather than by the sources — see Part D.** On a computer with no display server, driven from outside its own window, Blender 5.1 did not wait for anybody on any part of the render path that was tried: the auto-execution dialog became text on the terminal and the render carried straight on, the unsaved-changes pop-up never fired, an existing output file was written over silently, a missing texture was not even mentioned, an unwritable output folder gave an error and exit status 1 rather than a question, and Render Audio wrote straight over a file already there. **Two things do hold a run driven from outside**: a script that reads from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. Standard input in an ordinary background run is a real stream, not nothing. **Cycles can use a graphics card there** — that was the one thing Part D could not settle, and Part E settles it. Still open from outside: everything on macOS and Windows, and everything about what these same things do when Blender has a window.

Differences from inside the interface that *are* documented: no key configurations exist (`wm.keyconfigs.addon` "Can be None in background mode"), `context.space_data` may be None, the audio device is off by default, and the render frame currently being worked on is printed (a 5.1 change). Calling tools from Python defaults to the mode that skips the interactive step, and a pop-up raised from Python "will not block the scripts execution, so the caller can't wait for user input".

**A second operating system has now been tried, and it agreed — see Part F.** Every one of the nineteen entries the Linux machine ran outside Blender's own window was run again on the Creator's own Mac, in all four versions, and **the two machines agreed on every one of them**, down to the same figures where a figure came back. So what is said above about a run driven from outside is no longer true of one operating system only. **The two things that hold such a run hold on macOS too**: a script reading from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. Standard input is a real stream there as well, not nothing. **A render on the Mac also outlives the shell it was started from, and outlives a hang-up signal to its whole process group**, which is what closing a terminal window really sends. Still open from outside: everything on Windows, which no Windows machine has run.

**Three of these results were run a third time, and one of them changes (Part G).** "A missing texture is not mentioned" is **not** what happens when the texture is in the picture: Blender 5.1 prints `ERROR Failed to create GPU texture from Blender image`, in a column of its own, and the run still ends by itself with exit status 0 and writes its frame. The entry that reported silence was rendering a file whose textured material sat in a slot no face used, so it was measuring nothing. **So a run driven from outside has one more thing to watch for on its output, beyond an exit status.** Nothing else moves: an unwritable output folder still gives an error rather than a question, writes nothing, waits for nobody and comes back with exit status 1, however the folder is made unwritable — though **the reason it names in brackets cannot be trusted in any version**, this one having said `Success` throughout. And one thing said above about that machine is not a rule about Blender: the `libEGL` lines are not what a machine without a display server prints when it reaches for one. With the card's own graphics library, EEVEE renders there with no display server at all and prints nothing.

**And the window has now been reached, on the Mac — the thing every version of this answer has been leaving open (Part F).** With a window open, the automatic-execution warning is a **dialog in the middle of the main window**, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with the dialog unanswered, and with the first-run Quick Set Up unanswered underneath it. A **file chooser opened on top of a running animation render stopped no frames at all**. A still render nobody asks to be saved still writes nothing — **but the picture opens in a window of its own either way**. A render asked for on the command line with a window open is written at once, **but that Blender then never quits**. A `--background` render **does register with the Mac's window server** for the whole run, though it never comes to the front and draws nothing. And Render Audio is the one place where the menu's way and a script's way part company: **with no path named it opens a chooser and writes nothing; with a path named it writes straight over an existing file without asking** — and that, not the window, is what decides.

### Operating system

Mostly the same everywhere, with these documented exceptions:

- **Windows.** EEVEE: "Headless rendering is not supported on headless Windows systems." A crash writes a text file named after the blend-file. `-con` / `--start-console` and `--qos` are Windows-only. The GPU compositor page notes Windows' driver Timeout Detection Recovery resets the driver after a default 2-second limit for a compositing task, and recommends increasing that timeout — which is a machine-level interruption with nobody involved, though it concerns the GPU compositor and not the render itself (`https://docs.blender.org/manual/en/5.1/compositing/limitations.html`, read 2026-07-30).
- **macOS.** The crash reporter "shows a window with backtrace after some time, or when Blender is opened again" — the one documented case of something appearing on screen on an unwatched machine. And in 5.1, a script reaching the camera, microphone or system audio raises a system permission dialog.
- **Linux.** The command-line render page names Linux specifically when saying no graphical display is needed ("no need for X server on Linux for example"). `--no-window-frame` is Linux-only.

Nothing in the sources says the auto-execution dialog, the background mode switches, or the rendering behaviour in Part B differ by operating system.

**Linux is now settled by test as well, and only Linux (Part D).** Everything in Part D was got on Ubuntu 22.04.5 LTS on a machine with no display server. It says nothing about macOS, where the same tests have not been run, and nothing about Windows, which no Windows machine has run.

**macOS has now been tried as well (Part F), and it agreed with Linux on everything the two machines both ran.** The Creator's own Mac — a MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, with Apple's own graphics and no second card — gave the same answer as the rented Linux machine on all nineteen entries that were run on both, in all four versions. **It also reached the one thing Linux could not: a window.** Two differences between the two machines showed in what was printed, and neither changes an answer: the Mac prints no `libEGL` or `EGL Error` lines, because it has a display server; and Blender 4.5 on the Mac prints a run of `Freestyle:` progress lines that the Linux record does not show.

**The first of those two has since been taken back, by test on the Linux machine (Part G).** The `libEGL` and `EGL Error` lines are **not** the mark of a machine without a display server. On that same machine, with its own card's graphics library, EEVEE renders with no display server at all and prints none of them; with that library hidden and only Mesa's software one left, all four versions print them word for word. **So it is which graphics library Blender gets that decides, not whether there is a display server**, and the Mac's silence is not evidence either way. The Freestyle difference is untouched and stands.

**Windows is still untouched** — no Windows machine has run it — so every statement above that names Windows stays untested. **And one thing on the Mac was not run**: whether the screen locking or the display going to sleep interrupts a render. That needs the Creator's own machine to itself for several minutes, and this sitting had no standing to take it; the test is written out in full in `Audits/Blender Research/The machine/Hand tests — added questions.md`, entry 21.

### Not covered (the whole answer's edge)

This answer does not settle: whether the auto-execution dialog blocks the program or merely offers a choice; what Blender does with that decision when it has no interface; whether a render that needed a refused script fails loudly, renders wrong, or stops; what happens when output cannot be written, files are missing, or memory or disk runs out mid-render; whether a machine must be logged in or have a screen attached; what a screen lock, sleep or power setting does to a running render; and what "headless Windows system" means or what trying it produces. Every one of those is absent from the sources rather than answered by them.

**Several of the gaps listed just above have since been closed by test, and only by test — see Part F.** With a window open on the Creator's own Mac, the automatic-execution warning is a dialog in the middle of the main window, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with it unanswered. A window that *is* waiting for a person — a file chooser — did not stop a running animation render either. A still render nobody asks to be saved still writes nothing, though the picture opens in a window of its own. **The sources still say none of this**; what has changed is that it has been watched happening. What is left genuinely open in this section: everything on Windows, and what the machine's own screen locking or display sleeping does to a render, which was not run because it needs the Creator's own computer to itself.

---

## Things a person with Blender open could settle by hand in a few minutes

**Pruned 2026-08-06 after the hand tests, again 2026-08-07 after the card tests, and again 2026-08-07 after the Mac sitting.** Everything a terminal could reach was run on the rented Linux machine; everything needing a window was then run on the Creator's own Mac. **All of it is answered in Part D, Part E and Part F, and those items are gone from this list.** The full list, with every command written out, is `Audits/Blender Research/The machine/Hand tests — added questions.md`.

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

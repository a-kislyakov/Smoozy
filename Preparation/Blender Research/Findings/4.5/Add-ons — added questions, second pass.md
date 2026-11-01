# Add-ons — added questions, second pass (Blender 4.5)

Questions 12, 13 and 14 only. Every other question on the sheet was left alone.

**Sources consulted, and when.** Blender 4.5 manual (`manual_4.5`, including its `_images` folder), Blender 4.5 Python API documentation (`api_4.5`), Blender 4.5 release notes (`release_notes_4.5`), and Cycles' own files (`cycles_4.5`) — all read on **2026-07-30** (the date the copies were taken). Blender's requirements page, read as the copy taken on **2026-08-01**.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

---

## 12. Does installing an add-on disturb a render already running on the same computer?

**Label: not in the sources.**

**That label answers the question from the sources only. A test has since been run and settled it — see the observed section below in this same answer.**

**How I read the question.** I answer the reading I take to be meant: a render is already under way on a computer, and an add-on is installed on that same computer while it runs — does the installing disturb the render. The question is ambiguous in one respect: it could also mean installing an add-on *into the very same Blender that is rendering*. Neither reading is covered by the sources, so the answer is the same either way, but I flag the ambiguity rather than choose silently.

**What the sources do not say.** Nothing in the 4.5 manual, the 4.5 API documentation, the 4.5 release notes, Cycles' files or the requirements page states what happens to a render in progress when an add-on or extension is installed. There is no statement that it is safe, none that it is unsafe, and none describing any effect at all — on the render's memory, its output files, its devices, or its running time.

**The three nearest things I found, and why none of them answers it.**

1. Add-ons can be switched on and off inside a running Blender, and the manual says the effect is immediate:

   > "To enable or disable an add-on check or uncheck the box to the right of the add-ons.
   >
   > The add-on functionality should be immediately available."

   Source: Blender manual, Preferences → Add-ons. `https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html` — read 2026-07-30.

   The API documentation says the same about the mechanism:

   > "The register/unregister calls are used so it's possible to toggle add-ons and reload scripts while Blender runs."

   Source: Blender Python API documentation, Overview. `https://docs.blender.org/api/4.5/info_overview.html` — read 2026-07-30.

   This is about *toggling while Blender runs*. It says nothing about a render being under way at that moment.

2. Extension repositories can be locked and unlocked on the file system. This is the only sign anywhere in the sources that Blender guards against two things touching extensions at once:

   > "bpy.ops.extensions.repo_unlock()
   >
   > Remove the repository file-system lock"

   and

   > "bpy.ops.extensions.repo_lock_all()
   >
   > Lock repositories - to test locking"

   Source: Blender Python API documentation, Extensions Operators. `https://docs.blender.org/api/4.5/bpy.ops.extensions.html` — read 2026-07-30.

   These are about repositories, not renders. Nothing says a rendering Blender holds such a lock, or is affected by one.

3. Installing writes into an extensions repository directory, while a render's scratch files go somewhere else entirely:

   > "./extensions
   >
   > Extension repositories.
   >
   > Located in both user and system directories."

   and

   > "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."

   Source: Blender manual, Blender's Directory Layout. `https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html` — read 2026-07-30.

   Two separate directories are described. Whether that means an install cannot disturb a render is not stated, and I will not supply the step.

### Observed — an install fired at a render that is already running

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. The machine is shared with other topics' tests, so **every time below is from a shared machine and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 19, 20, 21, 22 and 23.

**Reading B — a render in one Blender, installs in another. Nothing happened to the render.** A scene of eight frames was rendered twice with `blender --background scene.blend --render-output <folder>/f_ --render-anim` — Cycles on the processor, eight of the twenty-four cores, 512 samples, adaptive sampling and denoising off, a fixed seed, 640 by 480. The first run had nothing else going on. During the second, six installs and six removals of a small extension were fired at it from a second shell, one pair every twelve seconds, using `blender --command extension install-file -r user_default -e <package>.zip` and `blender --command extension remove smoozy_ok` against the same settings folder.

Both runs wrote eight frames. Every install and every removal reported `STATUS Installed "smoozy_ok"` and `STATUS Removed "smoozy_ok"` and exited zero. The render exited zero both times. Every frame was compared picture against picture — not file against file — by loading both images in Blender and comparing the pixel values one by one:

```
PIXELCMP <control>/f_0001.png <disturbed>/f_0001.png differing_values 0 of 1228800 worst 0.0
```

and the same for frames 2 to 8: **no pixel differed at all, in any of the eight frames**. Nothing appeared in the disturbed run's output that was not in the control run's. A Blender started fresh afterwards printed nothing out of the ordinary.

**Reading A — a render and an install in the same Blender. The install went through and the render was untouched.** The same scene was rendered with no window, with a handler on `render_post` that installed the extension after the second frame from inside that same Blender:

```
SMOOZY_MIDRENDER_FRAME 1 1
SMOOZY_MIDRENDER_FRAME 2 2
SMOOZY_MIDRENDER_INSTALL {'FINISHED'}
SMOOZY_MIDRENDER_ADDONS ['bl_ext.user_default.smoozy_ok']
SMOOZY_MIDRENDER_FRAME 3 3
...
SMOOZY_MIDRENDER_DONE
```

Eight frames were written, the process exited zero, and every frame matched the undisturbed control run pixel for pixel. The operator's argument names were asked of the machine first rather than guessed: `SMOOZY_ARGS ['directory', 'enable_on_install', 'filepath', 'files', 'filter_glob', 'overwrite', 'repo', 'rna_type', 'target', 'url']`.

**What an install actually writes.** Before and after one install into a settings folder holding a saved `userpref.blend`, the files that changed were:

```
<settings folder>/extensions/user_default/smoozy_ok/__init__.py
<settings folder>/extensions/user_default/smoozy_ok/blender_manifest.toml
<settings folder>/extensions/user_default/smoozy_ok/__pycache__/__init__.cpython-311.pyc
<settings folder>/extensions/.cache/compat.dat
<settings folder>/config/userpref.blend
```

With `--no-prefs` added, the same install touched the first four and **left `userpref.blend` alone** — and the add-on was then not switched on afterwards, despite `-e`: `SMOOZY_ENABLED []` against `SMOOZY_ENABLED ['bl_ext.user_default.smoozy_ok']` without the flag. So an install touches two things outside its own folder: the preferences file, which `--no-prefs` spares, and `extensions/.cache/compat.dat`, which it does not.

**A Blender that is already running sees the files at once but does not switch the add-on on.** A Blender was started with a script that printed the enabled add-ons and the contents of the extensions folder, slept forty seconds, then rescanned. An install was run from a second shell while it slept:

```
SMOOZY_WATCH_START [] cannot read: [Errno 2] No such file or directory: '<settings folder>/extensions/user_default'
SMOOZY_WATCH_MIDDLE [] ['.blender_ext', 'smoozy_ok']
SMOOZY_WATCH_REFRESH {'FINISHED'}
SMOOZY_WATCH_AFTER_REFRESH [] ['.blender_ext', 'smoozy_ok']
SMOOZY_WATCH_ENABLE <module 'bl_ext.user_default.smoozy_ok' from '<settings folder>/extensions/user_default/smoozy_ok/__init__.py'>
SMOOZY_WATCH_END ['bl_ext.user_default.smoozy_ok']
```

So the new files were visible to the running Blender straight away, the rescan changed nothing about what was switched on, and the add-on could be switched on and used in that same running Blender without restarting it.

**The repository lock is real, and it does stop a second Blender.** This was asked in three parts, the first of which existed only to show that a lock is genuinely held — an install going through untouched would otherwise not tell a lock that does nothing apart from a lock that was never there.

*Part one — the lock can be shown.* `bpy.ops.extensions.repo_lock_all()` returned `{'FINISHED'}`, and the extensions folder went from empty to holding two entries:

```
SMOOZY_BEFORE []
SMOOZY_LOCK1 {'FINISHED'}
SMOOZY_AFTER ['<settings folder>/extensions/blender_org', '<settings folder>/extensions/user_default']
```

Calling it a second time said so plainly:

```
Error locking repository "extensions.blender.org": lock exists (lock is already held by this session)
Error locking repository "User Default": lock exists (lock is already held by this session)
Error locking repository "System": lock exists (lock is already held by this session)
```

*Part two — a held lock stops another Blender's install dead.* With one Blender holding the lock and sleeping, an install run from a second shell failed at once:

```
Error locking repository "User Default": lock exists (lock is held by other session "/tmp/blender_NxIOhR/")
Error: Repository "User Default": lock exists (lock is held by other session "/tmp/blender_NxIOhR/")
If the lock was held by a Blender instance that exited unexpectedly,
use: "Force Unlock Repository" to clear the lock.
Access from the "Repositories" popover in the extensions preferences.
RepoLock: freed without releasing lock!
```

with exit status 1, in 0.567 seconds. The very same command with nothing holding the lock printed `STATUS Installed "smoozy_ok"` and exited zero, in 0.813 seconds. So it does not wait — it refuses, and it names the session holding the lock. Both timings are from a machine shared with other topics' tests.

*Part three — a Blender that is rendering does not hold the lock.* While a background animation render was under way in the same settings folder, a second Blender took the lock without difficulty. That is why the render in the runs above was never disturbed by the installs: a rendering Blender is not holding the thing the installs need.

**Not covered by this test:** none of it was run on macOS or Windows. The macOS half has since been run and follows in the next section. The machine was shared with other topics' tests throughout, so the per-frame times are recorded but not read as measurements. Nothing here covers a windowed Blender, which the machine cannot open; that has since been run on the Creator's Mac and is recorded in the Mac observed section in this same answer. `bpy.context.window_manager.reports` does not exist in this version.

**Not covered.** Everything the question asks. In particular: whether a Blender that is rendering re-reads add-on files during the render; whether an install into a repository that a rendering Blender is using is safe; whether the preferences file being rewritten by an install is read by an already-running Blender; whether any resource contention (disk, memory, processor) is expected. Also not covered: whether it matters that the render and the install are in the same Blender process or in two different ones.

### Observed — the same question asked again on a Mac, including with a window

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 19, 20, 21, 22, 23 and 25.

**The Mac agreed with Linux on every part that both machines could run.** The differences are only in the paths printed.

*What an install writes.* Taking a full record of the settings folder before and after, an ordinary install changed these five files and nothing else. Two of them sit outside the extension's own folder — the shared compatibility cache and the preferences file:

```
> .../extensions/user_default/smoozy_ok/__init__.py
> .../extensions/.cache/compat.dat
> .../config/userpref.blend
> .../extensions/user_default/smoozy_ok/blender_manifest.toml
> .../extensions/user_default/smoozy_ok/__pycache__/__init__.cpython-311.pyc
```

With `--no-prefs` added, `config/userpref.blend` did not change — and the add-on was then **not** switched on, although `-e` was given: `SMOOZY_ADDONS []` where the ordinary install left `SMOOZY_ADDONS ['bl_ext.user_default.smoozy_ok']`.

*A Blender that is already running.* A Blender was left running while a second Blender installed into the same settings folder. It never saw the add-on in its collection — `SMOOZY_WATCH_START []`, `SMOOZY_WATCH_MIDDLE []`, and after a rescan that returned `SMOOZY_WATCH_REFRESH {'FINISHED'}` still `SMOOZY_WATCH_AFTER_REFRESH []`. But it could switch it on without being restarted: `SMOOZY_WATCH_ENABLE <module 'bl_ext.user_default.smoozy_ok' from '.../extensions/user_default/smoozy_ok/__init__.py'>`.

*Installs fired at a running render, from a second Blender.* An eight-frame Cycles animation was rendered twice — once undisturbed, once with install-and-remove pairs fired at it from a second Blender for as long as it ran (five pairs). Both runs wrote all eight frames and exited zero. Every install and every removal exited zero. Comparing the two runs picture by picture rather than file by file, every frame was identical:

```
PIXELCMP .../control_4.5/f_0001.png .../test_4.5/f_0001.png differing_values 0 of 307200 worst 0.0
```

and the same for frames 2 to 8. Setting aside the file names, no line appeared in one run's output that was not in the other's.

One oddity is recorded rather than smoothed over: of the five installs fired during the 4.5 run, the third printed no `STATUS Installed "smoozy_ok"` line at all, where the other four did. It exited zero all the same, and the removal that followed it printed `STATUS Removed "smoozy_ok"` and exited zero, so the add-on had been installed. What this means is not settled here.

*An install fired from inside the very Blender that is rendering, with no window.* A handler installed a package after the second frame:

```
SMOOZY_MIDRENDER_INSTALL {'FINISHED'}
SMOOZY_MIDRENDER_ADDONS ['bl_ext.user_default.smoozy_ok']
```

The render carried on through frame 8, and every frame matched the control picture for picture.

*The repository lock.* A lock is really taken: calling it twice in one Blender printed `Error: Repository "User Default": lock exists (lock is already held by this session)`, and two folders appeared where none had been.

> **Recorded by the citation check (2026-08-06), not resolved.** The same call, made twice in one background Blender 4.5.12 on the same Mac against a settings folder made for the test, printed `Error locking repository "User Default": lock exists (lock is already held by this session)` — "Error locking repository", not "Error: Repository". Two runs, two lines. The original is left as it was written; neither has been picked over the other. Sent back to the hand tests.
>
> The same check's repeat of the blocked-install block below printed one line more than is recorded here: it ended `RepoLock: freed without releasing lock!`, which the Linux block in this same answer carries and this Mac block does not. Everything else matched word for word. With one Blender holding the lock, an install run from a second failed at once:

```
Error locking repository "User Default": lock exists (lock is held by other session "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/blender_5z2XS3/")
Error: Repository "User Default": lock exists (lock is held by other session "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/blender_5z2XS3/")
If the lock was held by a Blender instance that exited unexpectedly,
use: "Force Unlock Repository" to clear the lock.
Access from the "Repositories" popover in the extensions preferences.
```

with exit status 1. The same command with the lock free printed `STATUS Installed "smoozy_ok"` and exited zero. A Blender that was rendering did **not** hold the lock: a second Blender took it during an animation render and got `SMOOZY_LOCK_DURING_RENDER {'FINISHED'}`.

*With a window — the part no machine could reach before.* Blender 4.5.12 LTS was started with a window on the Mac's screen and driven by a script that called Blender's own commands and took Blender's own screenshots. An animation render was started, and after the second frame a handler installed a package:

```
SMOOZY_WINDOW_BACKGROUND False
SMOOZY_WINDOW_COUNT 1
SMOOZY_WINDOW_LOCK_INTERFACE False
SMOOZY_WINDOW_INSTALL {'FINISHED'}
SMOOZY_WINDOW_ADDONS ['bl_ext.user_default.smoozy_ok']
```

The render wrote all eight frames, and every frame matched the no-window control picture for picture. **The whole thing was then run again with `use_lock_interface` set to `True`**, the control the sources describe as existing to protect a render, and nothing changed: `SMOOZY_WINDOW_INSTALL {'FINISHED'}` again, all eight frames again, every frame identical again. So a window makes no difference to this question, and neither does that control.

> **Sent back by the citation check (2026-08-06).** This second run is not usable as it stands, because it could not have come out any other way. The first run reads the flag back and prints `SMOOZY_WINDOW_LOCK_INTERFACE False`; the second run prints no read-back at all. If the assignment silently did not take, the second run was the first run repeated, and "nothing changed" is the only answer it could have given. Re-run it with `use_lock_interface` printed back from inside the render handler, so the record shows the lock was on at the moment the install fired. See `Audits/Blender Research/Add-ons/Citation check/Added questions — Opus 5.md`, test T-A. **Rebuilt on 2026-08-07** in the form a machine with no screen can reach, with the flag read back from inside the render handler — see the observed section on the rebuilt lock test further down this answer. **The windowed form of it still needs the Creator's Mac and stays sent back.**

**Not covered by this test:** nothing was run on Windows, because no Windows machine has run it. The Mac was shared with other topics' Blender work throughout, so no time here is read as a measurement. Nothing here settles whether an install is harmless in general — only that these installs, fired this often, changed no frame.


### Observed — the interface lock rebuilt, with the flag read back during the render

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, on 2026-08-07. All four versions were run and all four agreed. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`, inside a working folder of this test's own at `/root/again-add-ons/`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. Other topics' tests were running Blender on the same machine throughout, so **no time here is read as a measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entry 28.

**Why this was run again.** The citation check sent the windowed `use_lock_interface` run back, because nothing in its record showed the setting was in force at the moment the install fired — see the note in the Mac section above. This is that test rebuilt in the form a machine with no screen can reach. **The windowed form still needs the Creator's Mac and has not been run again; that half stays sent back.**

**What was done.** The eight-frame scene was rendered three times in Blender 4.5.12 LTS: once undisturbed as a control; once with `use_lock_interface` left `False` and a package installed from a `render_post` handler after the second frame; and once with `use_lock_interface` set to `True` and the same install. Cycles on the processor, eight of the twenty-four cores, 512 samples, adaptive sampling and denoising off, a fixed seed, 640 by 480. In every run the flag was printed back before the assignment, after it, at each of the eight frames **from inside the handler**, immediately after the install, and after the render.

**The lock really was on, and the record now shows it.** From the lock-on run:

```
SMOOZY_LOCK_BACKGROUND True
SMOOZY_LOCK_READBACK before_assignment context_scene False data_scene False wm_is_interface_locked False is_job_running_RENDER False
SMOOZY_LOCK_READBACK after_assignment context_scene True data_scene True wm_is_interface_locked False is_job_running_RENDER False
SMOOZY_LOCK_FRAME 1 1 handler_scene_use_lock_interface True
SMOOZY_LOCK_READBACK in_handler_frame_1 context_scene True data_scene True wm_is_interface_locked False is_job_running_RENDER False
SMOOZY_LOCK_FRAME 2 2 handler_scene_use_lock_interface True
SMOOZY_LOCK_READBACK in_handler_frame_2 context_scene True data_scene True wm_is_interface_locked False is_job_running_RENDER False
SMOOZY_LOCK_INSTALL {'FINISHED'}
SMOOZY_LOCK_READBACK just_after_install context_scene True data_scene True wm_is_interface_locked False is_job_running_RENDER False
SMOOZY_LOCK_ADDONS ['bl_ext.user_default.smoozy_ok']
SMOOZY_LOCK_READBACK after_render context_scene True data_scene True wm_is_interface_locked False is_job_running_RENDER False
SMOOZY_LOCK_DONE 8
```

Frames 3 to 8 printed the same two lines with `True` and are left out only for length. The lock-off run printed the same lines with `False` throughout, and the same `SMOOZY_LOCK_INSTALL {'FINISHED'}`.

**The install went through and the render was untouched, lock on and lock off alike.** All three renders wrote eight frames and exited zero. Every frame of both install runs was compared with its control picture against picture, never file against file:

```
PIXELCMP <control>/f_0001.png <lock on>/f_0001.png differing_values 0 of 1228800 worst 0.0
```

and the same for frames 2 to 8, and the same again for the lock-off run — **no pixel differed at all, in any of the eight frames, in either run**. Sixty-four comparisons across the four versions, every one of them that line.

**Two things the read-back turned up.** `bpy.context.window_manager.is_interface_locked` read `False` at every one of those moments, in all four versions and in both runs — including while the animation render was under way with the scene's own `use_lock_interface` set to `True`. And `bpy.app.is_job_running('RENDER')` read `False` at every one of them too, including from inside a `render_post` handler with the render still running. So neither of them is a way for a program with no window to see the scene setting, or to see that a render is going on.

**Not covered by this test:** nothing was run on macOS or Windows. It says nothing about a Blender with a window, because the machine has no display server; the windowed form of this test is still open and still sent back to the Creator's Mac. It shows that these installs, fired this way, changed no frame with the lock on; it does not show that an install is harmless in general. Other topics' tests were running Blender on the same machine throughout, so no time here is read as a measurement.


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every part of this question was reached with no window at all: the render, the installs, the removals, the lock, and an install fired from inside the rendering Blender by a handler. None of it needed a screen. **The windowed half has since been run on the Creator's Mac and agrees with the no-window half:** with a window open, an install fired from inside the rendering Blender returned `{'FINISHED'}`, every frame was written, and every frame matched its control picture for picture — with the interface lock off and on alike. What the sources say follows. The sources do not say. Installing can be driven from outside the interface — `blender --command extension install`, `install-file`, `update` and `remove` are all documented at `https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html` (read 2026-07-30) — but nothing states what any of them does to a render running elsewhere on the machine. **Brought up to date 2026-08-07.** The interface-lock half has now been rebuilt with no window at all, with `use_lock_interface` read back at every step and from inside the render handler at the very frame the install fires — see the observed section on the rebuilt lock test above. With the lock genuinely on, the install returned `{'FINISHED'}`, all eight frames were written, and every frame matched its control picture for picture, in all four versions. Two further things came out of that read-back and bear on this line: `bpy.context.window_manager.is_interface_locked` and `bpy.app.is_job_running('RENDER')` both read `False` at every moment, including mid-render with the lock on, so neither gives a program outside the interface a view of the scene setting or of the running render. **The windowed form of the same test is still waiting on the Creator's Mac and stays sent back.**

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The sources do not say. I found no statement on this point for any operating system, and none that distinguishes between them. **Brought up to date 2026-08-07.** The rebuilt interface-lock test was run on the rented Linux machine only, in all four versions, and came out as the Linux runs above. The windowed form of it needs the Creator's Mac and has not been run again. Nothing here changes what stays open for Windows.

**Where I looked.**

- Searched all of `manual_4.5`, `api_4.5`, `release_notes_4.5`, `cycles_4.5` and `requirements` for: `while rendering`, `during a render`, `during render`, `render is running`, `rendering is in progress`, `in progress`, `another instance`, `multiple instances`, `second instance`, `other instances`, `instance of Blender`, `simultaneous`, `concurrently`, `at the same time`, `separate process`, `subprocess`, `another copy`, `same machine`, `same computer`, `same system`, `co-exist`, `coexist`, `side by side`, `side-by-side`, `render farm`, `farm`, `render job`, `queue`, `busy`, `wait for`, `pause`, `stop the render`, `cancel the render`, `long-running`, `interrupt`, `responsive`, `blocking`, `freeze`, `background job`, `modal`, `abort`, `crash while`, `affect running`, `already running`, `disturb`, `interfere`, `unaffected`, `does not affect`, `no effect`, `safe to`, `hot reload`, `hot-reload`, `reload scripts`, `sys.modules`, `lock`, `file-system lock`, `repository lock`, `shared directory`, `network drive`.
- Every hit was checked. The only ones on topic at all are the three above. All other hits were about unrelated subjects (gimbal lock, data-blocks, thin-film interference, sculpt brushes, particle boids, and so on).
- Opened and read in full: Preferences → Add-ons; Preferences → Get Extensions; Extensions → Add-ons; Extensions → How to Create Extensions; Extensions Command Line Arguments; Command Line Arguments; Blender's Directory Layout; Deploying Blender in Production; Render Output → Output (including the "Primitive Render Farm" hint, which describes several machines writing to one shared directory and says nothing about installing anything); Preferences → Introduction; Troubleshooting → Startup; Troubleshooting → Python Errors; `bpy.ops.extensions`; `bpy.ops.preferences`; `info_overview`; `info_gotchas_threading`.
- Ruled out `render_notes_4.5/rendering.md`, `cycles.md`, `core.md`, `python_api.md` and `index.md`: searched each for `extension` and `add-on`; the only add-on-related lines concern built-in shader variants and importer/exporter changes, not installing during a render.
- Ruled out the requirements page: it lists hardware and operating-system support for named versions and contains no word about add-ons or installing.

---

## 13. Once an add-on is installed and switched on, what more can be learned about whether it will work?

**Label: documented**, in several separate strands, plus two **observed** strands from Cycles' own files. I mark each strand.

There is no single "does it work" indicator in the sources. There are these:

**(a) The console — documented.** The manual repeats the same tip in three places:

> "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred."

Sources: Blender manual, Preferences → Add-ons (`https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html`); Preferences → Get Extensions (`https://docs.blender.org/manual/en/4.5/editors/preferences/extensions.html`); Extensions → Add-ons (`https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html`) — all read 2026-07-30.

The manual also says what that console carries:

> "The Blender Console Window can display many different types of status and error messages. Some messages simply inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender non-responsive or shut down completely. The Blender Console Window messages can also originate internally from within the Blender code or from external sources such as Python scripts."

Source: Blender manual, Troubleshooting → Startup. `https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html` — read 2026-07-30.

And for add-on work specifically:

> "To best troubleshoot any error message Python prints while writing scripts, you run Blender from a terminal."

Source: Blender manual, Add-on Tutorial. `https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html` — read 2026-07-30.

**(b) Switching on actually runs the add-on's code — documented.** For one kind of fault the sources say outright that it shows at that moment: a problem with a class definition *"will be raised on registering"* (quoted below). **LABEL: inferred** for the wider step to "any failure shows at that moment rather than later" — the sources state it only for class definitions. *(Changed by the citation check: this had read "So a failure shows at that moment rather than later" under a documented label; the quotes carry that only for class-definition problems.)*

> "When the add-on is enabled, Blender executes the code and runs the register() function. When the add-on is disabled, Blender runs the unregister() function."

Source: Blender manual, Add-on Tutorial. `https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html` — read 2026-07-30.

And what Blender checks as it does so:

> "When loading a class, Blender performs sanity checks making sure all required properties and functions are found, that properties have the correct type, and that functions have the right number of arguments.
>
> Mostly you will not need concern yourself with this but if there is a problem with the class definition it will be raised on registering"

Source: Blender Python API documentation, Overview → Class Registration. `https://docs.blender.org/api/4.5/info_overview.html` — read 2026-07-30. Two example messages are given there verbatim: `ValueError: expected Operator, SimpleOperator class "execute" function to have 2 args, found 3` and `TypeError: validating class error: Operator.bl_idname expected a string type, not int`.

**(c) A rescan that reports problems — documented.** Both the Add-ons and the Get Extensions sections offer:

> "Refresh Local
>
> Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting). Any issues are reported as warnings."

Source: Blender manual, Preferences → Add-ons and Preferences → Get Extensions. `https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html` and `https://docs.blender.org/manual/en/4.5/editors/preferences/extensions.html` — read 2026-07-30.

The same wording appears as an operator, which means it can be triggered by name:

> "bpy.ops.extensions.repo_refresh_all(*, use_active_only=False)
>
> Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting). Any issues are reported as warnings"

Source: Blender Python API documentation, Extensions Operators. `https://docs.blender.org/api/4.5/bpy.ops.extensions.html` — read 2026-07-30.

**(d) What the add-on itself declares, shown after install — documented.**

> "You can click the arrow at the left of the add-on box to see more information, such as its location, a description and a link to the documentation. Here you can also find a button to report a bug specific of this add-on."

Source: Blender manual, Preferences → Add-ons. `https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html` — read 2026-07-30.

The matching operator is `bpy.ops.preferences.addon_expand(*, module='')` — "Display information and preferences for this add-on" (`https://docs.blender.org/api/4.5/bpy.ops.preferences.html`, read 2026-07-30).

**(e) Being switched on is not the same as being turned on — documented.** This is a distinct and easily missed point:

> "Some add-ons use this section for example to enable/disable certain functions of the add-on. Sometimes these might even all default to off. So it is important to check if the enabled add-on has any particular preferences."

Source: Blender manual, Preferences → Add-ons. `https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html` — read 2026-07-30.

**(f) Whether its bundled Python modules actually loaded — documented.** For add-ons that ship Python wheels:

> "Once the extension has been installed you can check the module is being loaded by importing it in the Python console and printing it's location:
>
> import PIL
> print(PIL.__file__)"

Source: Blender manual, Extensions → Python Wheels. `https://docs.blender.org/manual/en/4.5/advanced/extensions/python_wheels.html` — read 2026-07-30.

**(g) Where it actually landed — documented.**

> "The destination of the add-on depends on your Blender configuration. When installing an add-on the source and destination paths are printed in the console. You can also find add-on path locations by running this in the Python Console:
>
> import addon_utils
> print(addon_utils.paths())"

Source: Blender manual, Add-on Tutorial. `https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html` — read 2026-07-30.

**(h) Whether something outside the add-on will stop it — documented.** An add-on that needs the internet is gated by a setting that has nothing to do with the add-on being enabled:

> "If the add-on needs to use internet, it must check for the (read-only) property bpy.app.online_access. This option is controlled by Preferences, which can be overriding via command-line (--offline-mode / --online-mode)."

and

> "Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule."

Source: Blender manual, Extensions → Add-ons. `https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html` — read 2026-07-30.

**(i) A named failure mode with a named check — documented.**

> "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular Python scripts, these are not portable between different platforms.
>
> It is possible the library is incompatible with your Blender installation (attempting to load a library built for a different version of Python, or loading a 32-bit library on a 64-bit system).
>
> If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating system."

and

> "If Python is raising errors or you have an add-on that just fails when enabled with an error – e.g: ... is not a valid Win32 application. – this may be caused by some inconsistency in the Python libraries."

Source: Blender manual, Troubleshooting → Python Errors. `https://docs.blender.org/manual/en/4.5/troubleshooting/python.html` — read 2026-07-30. That page also gives a full script for finding duplicate libraries, and notes "(The output will show in Command Line Window.)".

**(j) Asking the machine what it holds — documented.**

> "blender --command extension list [-h] [-s]
>
> List packages from all enabled repositories."

Source: Blender manual, Extensions Command Line Arguments. `https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html` — read 2026-07-30.

**(k) Observed, from Cycles' own files: an enabled add-on's own preferences object is reachable, and can be asked what the computer offers.** Cycles reads its own preferences through the add-on collection:

> `    if not context or bool(context.preferences.addons[__package__].preferences.get_devices_for_type('OPTIX')):`

File: `cycles_4.5/properties.py`, line 272 — read 2026-07-30.

and defines the query it is calling:

> `    # Be careful when deciding when to call this function,`
> `    # as Blender can crash with `_cycles.available_devices()` on some drivers.`
> `    def get_device_list(self, compute_device_type):`
> `        import _cycles`
> `        device_list = _cycles.available_devices(compute_device_type)`

File: `cycles_4.5/properties.py`, lines 1637–1641 — read 2026-07-30.

What this shows, plainly: an add-on's own preferences are addressable by package name, and an add-on may expose there a query about what the machine can actually do. It also shows, in the add-on's own words, that such a query can itself be dangerous to run. *(Changed by the citation check: this had read "once an add-on is enabled, its own preferences are addressable…"; the quoted file line reads the preferences by package name and says nothing about enabling being required.)*

**(l) Observed, from Cycles' own files: an add-on's own declaration carries a `warning` slot.**

> `bl_info = {`
> `    "name": "Cycles Render Engine",`
> `    "author": "",`
> `    "blender": (2, 80, 0),`
> `    "description": "Cycles renderer integration",`
> `    "warning": "",`
> `    "doc_url": "https://docs.blender.org/manual/en/latest/render/cycles/",`
> `    "tracker_url": "",`
> `    "support": 'OFFICIAL',`
> `    "category": "Render"}`

File: `cycles_4.5/__init__.py`, lines 7–16 — read 2026-07-30. This is a legacy-style declaration (the older form; the manual says extensions carry a manifest instead). I record that a `warning` field exists in it. **I did not find anywhere in the 4.5 manual or API documentation stating that Blender shows this warning to the user, so I do not claim it does.**

**(m) Observed — what Blender 4.5 actually reports, with no window.**

**Label: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the machine's own settings folder was never touched. The add-ons were written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 12, 14, 15, 16, 17 and 24.

*There is a machine-readable signal, and it is the operator raising.* Four small extensions were installed — one that works, one whose `register()` raises, one importing a module that is not there, one with `bl_idname = 1` — and each was switched on with `bpy.ops.preferences.addon_enable(module=...)`. The working one returned `SMOOZY_OPS_RESULT {'FINISHED'}`. Each broken one raised instead:

```
SMOOZY_OPS_RAISED RuntimeError Error: SMOOZY_BADREG_DELIBERATE_FAILURE
SMOOZY_OPS_RAISED RuntimeError Error: No module named 'smoozy_module_that_does_not_exist'
SMOOZY_OPS_RAISED RuntimeError Error: validating class: Operator.bl_idname expected a string type, not int
```

`addon_utils.enable()` returned the module for the working one and `None` for each broken one, and swallowed the error rather than raising. After each attempt, `mod in bpy.context.preferences.addons` and `addon_utils.check(mod)` gave `True` / `(True, True)` for the working one and `False` / `(False, False)` for each broken one. Full tracebacks went to the error stream; a one-line `Error: ...` went to the ordinary output.

*But the collection records what was asked for, not what loaded.* A working extension was installed with `-e`, then its `__init__.py` was replaced with `this is not python(`, and Blender was started again. The failure was printed at start-up — `SyntaxError: '(' was never closed` — and the collection still carried it: `SMOOZY_ADDONS_AFTER ['bl_ext.user_default.smoozy_ok']`. So the collection can only be trusted about a switch-on made in the same run.

*The rescan says nothing about damage.* With that damaged add-on in place, `bpy.ops.extensions.repo_refresh_all()` returned `SMOOZY_REFRESH {'FINISHED'}` and printed nothing naming it. The same held for a deleted manifest and a deleted `__init__.py`. All three faults were reported by Blender's own start-up load instead — a deleted manifest gave `Warning: add-on missing manifest, this can cause poor performance!: '<settings folder>/extensions/user_default/smoozy_ok/blender_manifest.toml'` and then quietly removed the extension from `blender --command extension list` while still loading and registering it; a deleted `__init__.py` gave `Add-on not loaded: "bl_ext.user_default.smoozy_ok", cause: module loaded with no associated file, ... aborting!`.

*The listing carries no health.* With one extension switched on, one installed and off, and one whose switch-on had failed, `blender --command extension list` printed all three identically as `[installed]`. `blender --command extension list -s` refused with `RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences` and exit status 1.

*`validate` only parses the manifest.* Pointed at the installed folder of the working extension and at the installed folder of the broken one, it printed `Success parsing TOML in "..."` for both and exited zero. With the manifest deleted it printed `FATAL_ERROR: Error, file "..." not found!` and exited 1.

*Which checks work with no window.* All of these returned rather than raising: `bpy.app.background` (`True`), `bpy.context.preferences.addons.keys()`, `addon_utils.paths()` (`['/opt/blender/4.5/4.5/scripts/addons_core']`), `bpy.app.online_access` (`False`), `bpy.app.is_job_running('RENDER')`, `bpy.utils.extension_path_user(...)`, `bpy.data.workspaces[0].use_filter_by_owner`, `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()`, `bpy.ops.preferences.addon_expand(module=...)`, `bpy.ops.extensions.status_clear_errors()` and `bpy.context.preferences.version` (`(4, 5, 92)`). One did not: `SMOOZY13 wm_reports RAISED AttributeError 'WindowManager' object has no attribute 'reports'`.

*The two spellings.* `hasattr(bpy.app, "online_access_override")` gave `True`; `hasattr(bpy.app, "online_access_overriden")` gave `False`.

*Logging arguments — and here 4.5 differs from the other three.* `blender --background --factory-startup --log-list-categories` is not accepted by 4.5 at all: `unknown argument, loading as file: --log-list-categories`, then `Error: Cannot read file "/root/--log-list-categories": No such file or directory`, exit status 1. The same argument is accepted by 5.0, 5.1 and 5.2, each printing a list beginning `addon`, `anim`, `anim.action`. `--log-level 2` is accepted by all four but only the other three print anything extra at that level. 4.5's help carries `--log-show-basename` and `--log-show-timestamp` where the other three carry `--log-show-memory` and `--log-show-source`.

*Exit status.* A failed switch-on left the exit status at zero unless `--python-exit-code 1` was given *and* the error was left uncaught, in which case it was 1. The same failure through `addon_utils.enable` exited 0 either way. Naming a broken add-on at launch with `--addons` printed the traceback and exited 0 in both cases. Naming a *working* add-on with `--addons` loaded and registered it but left `bpy.context.preferences.addons` empty.

*The `bl_info` `warning` field reads back exactly as written*, alongside five keys Blender fills in that the file never carried: `SMOOZY_BLINFO smoozy_legacy_warn {'name': 'Smoozy Legacy Warn', ..., 'warning': 'SMOOZY WARNING TEXT', 'category': 'Development', 'location': '', 'doc_url': '', 'support': 'COMMUNITY', 'show_expanded': False, '_init': None}`. Whether it is *shown* anywhere has since been run on the Creator's Mac, which has a screen: it is shown, both as a triangle on the add-on's row and as a `Warning` line inside it. See the Mac observed section in this same answer.

**Not covered by this test:** whether a broken add-on is marked in the Add-ons list, and whether a non-empty `warning` is displayed — both need a window, and both have since been run on the Creator's Mac; see the Mac observed section in this same answer. A declared warning is shown; a failure to switch on is not marked at all. What the extensions error status holds; `status_clear_errors` returned `{'FINISHED'}` and printed nothing. Nothing here was run on macOS or Windows.

**Not covered.** No source gives a single machine-readable answer to "is this add-on working". No source documents a return value or status from `bpy.ops.preferences.addon_enable` or `bpy.ops.extensions.package_install`. "Any issues are reported as warnings" does not say what counts as an issue, nor where the warnings appear when there is no window. Nothing says whether Blender checks an extension's declared minimum Blender version at the moment of enabling. Nothing describes a way to exercise an add-on's actual function short of using it. Nothing says how long after enabling a failure may still surface.

### Observed — the same checks asked again on a Mac, including the Add-ons list on screen

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 12, 13, 14, 15, 16, 17, 24 and 26.

**The Mac agreed with Linux on every check.** What follows is what Blender 4.5.12 LTS printed on the Mac.

*Every check, with no window.* Seventeen checks were run in one script in a Blender started with `--background`. All of them returned rather than raising, except one:

```
SMOOZY13 background -> True
SMOOZY13 addons_keys -> ['bl_ext.user_default.smoozy_ok', 'bl_pkg', 'cycles', ...]
SMOOZY13 addon_utils_paths -> ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/scripts/addons_core']
SMOOZY13 online_access -> False
SMOOZY13 has_online_access_override -> True
SMOOZY13 has_online_access_overriden -> False
SMOOZY13 is_job_running_render -> False
SMOOZY13 extension_path_user -> '.../extensions/.user/user_default/smoozy_ok'
SMOOZY13 workspace_filter -> False
SMOOZY13 repo_refresh_all -> {'FINISHED'}
SMOOZY13 addon_refresh -> {'FINISHED'}
SMOOZY13 addon_expand -> {'FINISHED'}
SMOOZY13 status_clear_errors -> {'FINISHED'}
SMOOZY13 addon_enable_args -> ['module', 'rna_type']
SMOOZY13 addon_prefs -> None    [citation check: not reproducible from this description alone — see T-D]
SMOOZY13 prefs_version -> (4, 5, 92)
```

`bpy.context.window_manager.reports` raised — `SMOOZY13 wm_reports RAISED AttributeError 'WindowManager' object has no attribute 'reports'`. On the two spellings the documents disagreed about: `bpy.app.online_access_override` exists and `bpy.app.online_access_overriden` does not.

*Command-line arguments.* `--log-list-categories` was **not accepted.** `--log-list-categories` came back `unknown argument, loading as file: --log-list-categories` and then `Error: Cannot read file "...--log-list-categories": No such file or directory`, exit status 1. The help text of this version carries `--log`, `--log-level`, `--log-show-basename`, `--log-show-backtrace`, `--log-show-timestamp` and `--log-file`, with no `--log-list-categories`.

*Switching on a broken add-on.* Four add-ons were installed and left switched off — a working one and three broken in different ways. The working one gave `SMOOZY_OPS_RESULT {'FINISHED'}`, `SMOOZY_IN_COLLECTION True`, `SMOOZY_CHECK (True, True)`. Each broken one gave a raise from the operator, `None` from the helper, and exclusion from both the collection and the check:

```
SMOOZY_OPS_RAISED RuntimeError Error: SMOOZY_BADREG_DELIBERATE_FAILURE
SMOOZY_UTILS_RESULT None
SMOOZY_IN_COLLECTION False
SMOOZY_CHECK (False, False)
```

with `Error: No module named 'smoozy_module_that_does_not_exist'` and `Error: validating class: Operator.bl_idname expected a string type, not int` for the other two. The full Python traceback went to the error stream while the short `Error:` line went to the ordinary stream. Exit status was zero in every case, and stayed zero with `--python-exit-code 1` while the script caught the error; with the error left uncaught it became 1 and Blender printed `Error: script failed, file: '...', exiting.` Naming a broken add-on at launch with `--addons` printed the traceback and left `SMOOZY_ADDONS []`, exit status zero; naming the working one printed its register line and also left `SMOOZY_ADDONS []`.

*A rescan after damage.* A working, switched-on add-on was damaged on disk three ways in turn. In every case the rescan returned `SMOOZY_REFRESH {'FINISHED'}` and said nothing about the damage; the fault appeared at start-up instead, before the rescan ran. The add-on stayed in the collection each time — `SMOOZY_LIST_AFTER ['bl_ext.user_default.smoozy_ok']`. One damage did change the listing: with `blender_manifest.toml` deleted, `extension list` no longer showed it under User Default, and start-up printed `Warning: add-on missing manifest, this can cause poor performance!`. With `__init__.py` deleted or filled with text that is not Python, the listing was unchanged.

*What a listing shows.* A working switched-on extension, one installed and left off, and one whose switch-on had failed all printed the same way:

```
  smoozy_badimport [installed]: "Smoozy Badimport", A tiny add-on made for a test
  smoozy_badreg [installed]: "Smoozy Badreg", A tiny add-on made for a test
  smoozy_ok [installed]: "Smoozy Ok", A tiny add-on made for a test
```

No health, no on-or-off. `extension list -s` refused with `RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences` and `Error synchronizing`, exit status 1.

*Validating.* `extension validate` accepted an installed folder and printed `Success parsing TOML in "..."`, exit status 0 — including for the add-on that cannot be imported at all. It only reads the manifest: a copy whose `__init__.py` was replaced with text that is not Python still passed. It refused a folder with no manifest — `FATAL_ERROR: Error, file ".../blender_manifest.toml" not found!`, exit status 1 — and one with no `__init__.py` — `Error, file missing from add-on: "__init__.py"`, exit status 1.

*A legacy add-on's warning.* The `warning` field read back exactly as written: `'warning': 'SMOOZY WARNING TEXT'`.

*On screen — the part no machine could reach before.* Blender 4.5.12 LTS was started with a window, four legacy add-ons were switched on from a script, the Add-ons section of Preferences was opened and searched for "smoozy", and Blender took its own screenshots. Three things are visible in them.

- **A declared warning is shown.** The row for the add-on carrying `"warning": "SMOOZY WARNING TEXT"` has a warning triangle at its right-hand end where the other rows have the plain legacy-add-on icon, and when the row is opened it carries a line reading `Warning` followed by the same triangle and the words `SMOOZY WARNING TEXT`.
- **An add-on that failed to switch on is not marked at all.** Its row looks exactly like a row for an add-on simply left switched off: the tick box is empty, there is no triangle, and opening the row shows only the ordinary Type, Maintainer, Version and File lines. Nothing on screen distinguishes "it broke" from "it is off".
- **The one declaring it needs Blender 99 is in the list and can be switched on.** Its row is present and its tick box can be ticked, which agrees with what the Python side reported.

`addon_utils.check` matched the picture: `(True, True)` for the three that switched on and `(False, False)` for the one that failed.

**Not covered by this test:** nothing was run on Windows, because no Windows machine has run it, so the Windows-only console instruction and the mixed-library check on the troubleshooting page stay untested. What the extensions error status holds is still unknown; `status_clear_errors` returned `{'FINISHED'}` and printed nothing.


**Outside the interface.** **Settled by test on both machines — see the two observed sections above ((m)).** Every check in this answer that is Python or command line was run in a Blender started with no window and returned rather than raising, including `bpy.ops.extensions.repo_refresh_all()`, which the sources leave unstated. `bpy.context.window_manager.reports` does not exist in 4.5. **The parts that are panels in the Preferences editor have since been reached on the Creator's Mac, which has a screen** — see the Mac observed section above. A declared warning is shown on the row and inside it; an add-on that failed to switch on is not marked at all. What the sources say follows.

- Enabling from outside is documented: `--addons <addon(s)>` — "Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons." (Blender manual, Command Line Arguments, `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`, read 2026-07-30). The deploying page also shows a startup script calling `addon_utils.enable("my-addon")` (`https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html`, read 2026-07-30).
- Listing from outside is documented: `blender --command extension list` (strand j).
- Strands (f) and (g) are Python one-liners, so they can be run from a script rather than typed into a console; the sources describe them as things to type in the Python Console, and do not say so in so many words.
- The console strand (a) — **inferred**, and here is the step. The manual says Blender's messages "can also originate ... from external sources such as Python scripts", and separately advises running Blender from a terminal to see error messages, and separately says a troubleshooting script's "output will show in Command Line Window". From those three I take it that the same error text reaches a terminal when Blender is driven from one. What the sources never say in so many words is that the tip "check the Console window" holds for a Blender started with no window at all.
- Strands (d) and (e) — the information box and the preferences box — are described as things you click in the Preferences editor. The sources do not describe reading them without a window.
- One documented difference that bears directly on this: an add-on can behave differently when there is no window. The manual's own tutorial says so in a code comment:

  > "# Note that in background mode (no GUI available), keyconfigs are not available either,
  > # so we have to check this to avoid nasty errors in background case."

  Source: Blender manual, Add-on Tutorial. `https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html` — read 2026-07-30. So an add-on that appears to work with a window may raise errors without one, and the sources say so.

**Operating system.** **Partly settled by test — see (m).** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. Mostly the same on all three, with named exceptions.

- Blender 4.5 is named as supported on Windows, macOS and Linux by the requirements page (`https://www.blender.org/download/requirements/`, read 2026-08-01), which lists "Blender 4.5 LTS" separately under each of those three headings.
- How you reach a console differs by operating system: "The specifics of this will vary by operating system. See command line." (Troubleshooting → Startup, address above, read 2026-07-30). The manual has separate launch pages for Linux, macOS and Windows.
- Strand (i) is explicitly platform-bound — "these are not portable between different platforms" — and its second half ("Mixed Python Libraries (DLLs)", the `is not a valid Win32 application` message, the `GetSystemDirectory` script) is under a "Windows" heading, so it is Windows only.
- The sources do not say whether the console tip, Refresh Local, or the information box differ between operating systems. I take them to be the same because nothing says otherwise; that is an absence, not a statement.

**Where I looked.** Read in full: Preferences → Add-ons; Preferences → Get Extensions; Extensions → Add-ons; Extensions → How to Create Extensions; Extensions → Python Wheels; Extensions Command Line Arguments; Command Line Arguments; Deploying Blender in Production; Blender's Directory Layout; Troubleshooting → Startup; Troubleshooting → Python Errors; Add-on Tutorial; Advanced → Operators; `bpy.ops.extensions`; `bpy.ops.preferences`; `bpy.types.Addon`; `bpy.types.Addons`; `bpy.types.AddonPreferences`; `bpy.types.PreferencesExtensions`; `bpy.types.UserExtensionRepo`; `info_overview`; `info_gotchas_threading`; the relevant parts of `bpy.utils` and `bpy.app`. Opened the picture `_images/editors_preferences_section_extensions.png` (the Get Extensions panel): it shows an "Installed" group and an "Available" group with Install buttons, a search field, an "Add-ons" type filter and a "Repositories" menu — it shows no health or status indicator against an installed extension, which is why I do not claim one exists. Searched all four version-4.5 source folders for: `addon_utils`, `blender_manifest`, `blender_version_min`, `blender_version_max`, `restart`, `refresh`, `warning`, `error`, `console`, `traceback`, `blocklist`, `blocked`, `online_access`, `validate`, `wheels`, `bl_info`, `register`, `sanity`.

---

## 14. Does an add-on set up for one Blender work in another Blender on the same computer, and what decides that?

**Label: documented** for the separation and for the declared version range; **inferred** for the consequence, with the step written out below; **observed** for the legacy declaration in Cycles' files.

**First: two Blenders can sit on one computer at once — documented.**

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: Blender manual, Deploying Blender in Production. `https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html` — read 2026-07-30.

> "Note, you do not have to overwrite your existing Blender installation. It's perfectly possible to have multiple versions installed side by side."

Source: Blender manual, Installing on Windows. `https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html` — read 2026-07-30.

**Second: what each Blender reads is a folder named after its own version — documented.**

> "User directories store preferences, startup file, installed extensions, presets and more. By default these use the standard configuration folders for each operating system."

followed by the three paths, each ending in the version number:

> "Linux
>
> $HOME/.config/blender/4.5/
>
> ...
>
> macOS
>
> /Users/$USER/Library/Application Support/Blender/4.5/
>
> ...
>
> Windows
>
> %USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\4.5\"

and

> "./extensions
>
> Extension repositories.
>
> Located in both user and system directories. Repositories are loaded from both directories."

Source: Blender manual, Blender's Directory Layout. `https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html` — read 2026-07-30.

The manual states the separation outright, and states it in the same breath as add-ons:

> "The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders."

and, of the option that does the importing:

> "Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and startup files from the previous version of Blender and load them. This will include installed add-ons and extensions."

Source: Blender manual, Defaults. `https://docs.blender.org/manual/en/4.5/getting_started/configuration/defaults.html` — read 2026-07-30.

**The inference, and its single step.** Each Blender version reads a user directory named after itself; installed extensions live in that directory; and a deliberate copying step exists precisely so that a newer version can pick up the older version's installed add-ons. From that I take it that an add-on installed under one version's user directory is *not* seen by another version on the same computer until it is copied or the directories are pointed at each other. The step a reader must supply is the word "not" — the manual never says a version cannot read another version's folder; it says the folders are separate and provides a means of copying between them. **Label for this step: inferred.**

**Third: even once it is in front of the other Blender, it may not work — documented.**

> "Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this option may lead to errors on startup. If this occurs, the recommended first step is to try Loading Factory Settings."

Source: Blender manual, Defaults. Address above — read 2026-07-30.

> "New Blender versions can sometimes introduce breaking changes in Blender's Python API or even change how entire features work. If this affects your extension, you may want to maintain two 'tracks' of your extension concurrently: one for Blender 'old' and one for Blender 'new'."

and

> "Make sure to correctly indicate the Blender versions that each version of your extension is compatible with in their manifest file. You can also update Blender version compatibility of already-uploaded versions of your add-on from the extensions website."

Source: Blender manual, Version Number Guidelines. `https://docs.blender.org/manual/en/4.5/advanced/extensions/version_number_guidelines.html` — read 2026-07-30.

**Fourth: what the add-on declares about which Blenders it is for — documented.** The extension manifest carries two fields:

> "blender_version_min:
>
> Minimum supported Blender version - use at least 4.2.0."

(listed under "Required values"), and

> "blender_version_max:
>
> Blender version that the extension does not support, earlier versions are supported."

(listed under "Optional values").

Source: Blender manual, How to Create Extensions. `https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html` — read 2026-07-30. The same two names appear again as download-link parameters at `https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/static_repository.html` (read 2026-07-30), where the manual says of them: "The more details you provide, the better the user experience."

**Observed, from Cycles' own files: the older form of the same declaration.**

> `    "blender": (2, 80, 0),`

File: `cycles_4.5/__init__.py`, line 10 — read 2026-07-30. A legacy-style add-on names a Blender version in its `bl_info` dictionary rather than in a manifest.

**Fifth: two further things that are pinned to a particular Blender — documented.**

> "Blender and the bpy module are only compatible with a single Python version. This makes it possible for add-ons and VFX software in general to only have to target a single Python version.
>
> Blender bundles a complete Python installation and does not interact with the system Python by default."

Source: Blender manual, Deploying Blender in Production. Address above — read 2026-07-30.

That matters because an add-on may carry compiled Python modules built for one Python version, named accordingly — the manual's own worked example uses `--python-version=3.11` and file names such as `pillow-10.3.0-cp311-cp311-win_amd64.whl` (Blender manual, Python Wheels, `https://docs.blender.org/manual/en/4.5/advanced/extensions/python_wheels.html`, read 2026-07-30). And the troubleshooting page names the failure directly: "attempting to load a library built for a different version of Python" (Troubleshooting → Python Errors, address above, read 2026-07-30).

Blender 4.5's own release notes list breaking changes that would affect an add-on moved between versions — quoting a release note here is reporting what it says, not comparing versions:

> "**Add-ons** that use **built-in shaders** should use `POLYLINES_*` or `POINT_*` shader variants to support Metal and Vulkan."

Source: Blender 4.5 release notes, index. `https://developer.blender.org/docs/release_notes/4.5/` — read 2026-07-30.

> "`gpu.types.GPUShader` constructor is deprecated and will be removed in Blender 5.0. Use `gpu.shader.create_from_info()` instead."

Source: Blender 4.5 release notes, Python API. `https://developer.blender.org/docs/release_notes/4.5/python_api/` — read 2026-07-30. (That page's "Breaking Changes" section lists several more of the same kind.)

**Sixth: the separation can be overridden — documented.** The version-named folders are the default, not the only possibility:

> "The BLENDER_USER_RESOURCES environment variable can be set to a custom directory to replace the default user directory."

and

> "BLENDER_SYSTEM_SCRIPTS and BLENDER_SYSTEM_EXTENSIONS environment variables can be used to bundle additional scripts and extensions, that are not part of the regular Blender installation."

Source: Blender manual, Blender's Directory Layout. Address above — read 2026-07-30.

The full list includes `BLENDER_USER_EXTENSIONS: Directory for user extensions.` and `BLENDER_SYSTEM_EXTENSIONS: Directory for system extensions repository.`, with matching launch arguments `--env-system-scripts` and `--env-system-extensions` (Blender manual, Command Line Arguments, `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`, read 2026-07-30).

There is also a per-installation arrangement that ties everything to one copy of Blender rather than to a version folder:

> "To enable this, create a folder named portable at the following locations: ... This folder will then store preferences, startup file, installed extensions and presets."

Source: Blender manual, Blender's Directory Layout. Address above — read 2026-07-30.

And a read-only shared repository intended for exactly this kind of sharing:

> "When working offline or in a more controlled environment, it may be useful to provide a set of extensions to all users. For this there is a default read-only System repository. This repository could be located on a read-only network drive or in a system directory."

Source: Blender manual, Deploying Blender in Production. Address above — read 2026-07-30.

**So, what decides it.** Four things, all from the quotes above:

1. **Which directories that Blender reads.** By default a folder named after its own version, so each version has its own installed add-ons; but `BLENDER_USER_RESOURCES`, `BLENDER_USER_EXTENSIONS`, `BLENDER_SYSTEM_EXTENSIONS`, `BLENDER_SYSTEM_SCRIPTS`, the `--env-system-*` arguments, a `portable` folder, or a System repository can all change what a given Blender looks at.
2. **The Blender version range the add-on declares** — `blender_version_min` (required) and `blender_version_max` (optional) in the manifest, or the `"blender"` entry in a legacy `bl_info`.
3. **Whether the add-on's code still matches that Blender's Python API** — the manual says breaking changes happen and that they may make an add-on "not compatible", and 4.5's release notes list specific ones.
4. **For anything compiled that the add-on carries** — the Python version it was built for, and the platform.

**Observed — what four Blenders on one computer actually do.**

**Label: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — on 2026-08-07, with Blender 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS side by side. Every Blender was pointed at a settings folder or a stand-in home folder made for the test beforehand; the machine's own settings folder was never touched. The add-ons were written on the machine, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 1 to 11 and 18.

*Which Python each carries*, from `print(sys.version)`: 4.5.12 LTS → `3.11.11`, 5.0.1 → `3.11.13`, 5.1.2 → `3.13.9`, 5.2.0 LTS → `3.13.13`. So the four split two and two.

*The folder each reads carries two numbers, not three.* With `HOME` pointed at a folder of the test's own, 4.5 printed `SMOOZY_USER <home>/.config/blender/4.5` and `SMOOZY_EXT <home>/.config/blender/4.5/extensions`, while `bpy.app.version` read `(4, 5, 12)`. The other three printed `5.0`, `5.1` and `5.2`. The patch number never enters the path.

*Nothing is shared by default.* Four differently-named extensions were installed, one by each Blender, into one shared home folder. Each Blender listed only its own, and four separate version-named folders existed on disk. The inferred step in this answer holds on this machine.

*Sharing works, in two degrees.* With `BLENDER_USER_EXTENSIONS` pointed at one folder for all four but each keeping its own settings folder, an extension installed by 4.5 was listed and could be switched on by all four — but only the one whose settings folder recorded it had it switched on at start-up. With `BLENDER_USER_RESOURCES` pointed at one folder for all four, all four listed it and all four had it switched on at start-up. The same held with 5.2 installing and the older three reading.

*A shared system-extensions folder serves all four.* With the extension unpacked at `<folder>/system/smoozy_ok` and `BLENDER_SYSTEM_EXTENSIONS` set to `<folder>`, each of the four listed it under `Repository: "System" (id=system)` and switched it on as `bl_ext.system.smoozy_ok`.

*A shared scripts folder serves all four, for a legacy add-on.* With `BLENDER_SYSTEM_SCRIPTS` set to a folder holding `addons/smoozy_legacy_ok.py`, each printed `SMOOZY_MODULES ['smoozy_legacy_ok']` and switched it on.

*Two copies of one build share; a `portable` folder takes over.* A second copy of `/opt/blender/4.5` listed the extension the first had installed, and both resolved `bpy.utils.resource_path('USER')` to the same `<home>/.config/blender/4.5`. A `portable` folder made next to the copy's program moved it to `<copy>/portable`, after which the copy listed nothing. The same for 5.0, 5.1 and 5.2.

*A folder copied by hand between versions works*, in both directions, out of 4.5 into each of the other three and out of 5.2 into each of the other three: listed, switched on, register line printed, no complaint.

*A declared version range is acted on, at install.* An extension declaring `blender_version_min = "99.0.0"` and one declaring `blender_version_max = "4.3.0"` both built and validated without complaint, then:

```
ERROR smoozy_min99: This Blender version (4.5.12) doesn't meet the minimum supported version (99.0.0)
ERROR smoozy_max40: This Blender version (4.5.12) must be less than the maximum version (4.3.0)
```

with the same shape from 5.0.1, 5.1.2 and 5.2.0, naming each one's own version. Nothing was installed, the listing stayed bare, and `addon_utils.enable` returned `None`. **The command exits zero even so.** With `-e` there is a second line, and its wording differs by version: 4.5 and 5.0 print `Package should have been installed but not found: smoozy_min99`; 5.1 and 5.2 print `Package was expected to be installed but not found: smoozy_min99`.

*A legacy add-on's declared minimum is not acted on.* A legacy add-on with `"blender": (99, 0, 0)` was listed by all four alongside a plain one, and all four switched it on and ran its `register()`.

*A bundled Python package built for a different Python is refused, and the message names both versions.* Three copies of one extension carried hand-built wheels tagged `cp39`, `cp311` and `cp313`. All nine builds succeeded; the installs did not:

```
4.5 (Python 3.11), cp39  -> Error: Extension bl_ext.user_default.smoozy_wheel39 is incompatible (This Python version (3.11) isn't compatible with (3.9)), exit 1
4.5 (Python 3.11), cp311 -> installed, exit 0
4.5 (Python 3.11), cp313 -> Error: ... (This Python version (3.11) isn't compatible with (3.13)), exit 1
5.1 (Python 3.13), cp311 -> Error: ... (This Python version (3.13) isn't compatible with (3.11)), exit 1
5.1 (Python 3.13), cp313 -> installed, exit 0
```

Where it installed, the bundled package imported and printed its place under `extensions/.local/lib/python3.11/site-packages` for 4.5 and 5.0 and `python3.13` for 5.1 and 5.2. **One reading is spoiled and is recorded rather than hidden:** in the 4.5 and 5.0 runs the `cp313` step showed the refusal above but the import still succeeded, because the earlier `cp311` install had left the package in that Python's site-packages folder and removing the extension did not clear it.

*`bpy.ops.preferences.copy_prev()` works with no window.* From a home folder holding a set-up 4.5 and nothing else, 5.0 started with `--background` printed `Info: Reload Start-Up file to restore settings` and `SMOOZY_COPYPREV {'FINISHED'}`, after which `blender/5.0/` existed with a `userpref.blend` and an `extensions/user_default/`, and 5.0's own listing carried 4.5's extension. 5.1 then picked up from 5.0 and 5.2 from 5.1. 4.5, having no earlier version on the machine, refused: `RuntimeError: Operator bpy.ops.preferences.copy_prev.poll() failed, context is incorrect`.

**Not covered by this test:** none of it was run on macOS or Windows, so the paths and the `portable` placement are confirmed for Linux only. Nothing here says what a remote repository does with a declared range; the machine had no internet access allowed. Two builds of one major-and-minor version could not be tried, because the project has one build of each.

**Not covered.** The sources describe `blender_version_min` and `blender_version_max` as declarations and never say what Blender does when they are not met — whether it refuses to install, refuses to enable, warns, or ignores them. Nothing says what happens when two different Blender versions are pointed at one extensions directory at the same time. Nothing says whether the files of an extension installed by 4.5 are byte-for-byte usable by another version. Nothing gives a way to ask, before trying, whether a particular installed add-on will work in a particular other Blender. And the manual's "may not be compatible ... may lead to errors on startup" is a warning, not a rule — it does not say which add-ons, or how you would know in advance.

### Observed — the same questions asked again of four Blenders on a Mac

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 1 to 11, 18 and 27. All four Blenders — 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS — sit side by side on this Mac and all four were used.

**The Mac agreed with Linux on everything except one thing, and that one thing is set out at the end.**

*Which Python each carries.* The four differ, and they split two and two:

```
4.5 -> 3.11.11    5.0 -> 3.11.13    5.1 -> 3.13.9    5.2 -> 3.13.13
```

Blender 4.5.12 LTS carries Python 3.11.11. These are the same four Pythons the Linux builds carry.

*Where each Blender's own folder sits.* With nothing overridden, Blender 4.5.12 LTS computes:

```
SMOOZY_USER /Users/artist/Library/Application Support/Blender/4.5
SMOOZY_LOCAL /Applications/Blender 4.5.12.app/Contents/Resources/4.5
SMOOZY_SYSTEM /Library/Application Support/Blender/4.5
```

The folder is named for the major and minor number only; the patch number never enters it, so 4.5.12 reads a folder called `4.5`. This is the macOS layout the sources describe, confirmed.

**One thing works differently on macOS from Linux, and it changed how every test below had to be run.** On the Linux machine a stand-in home folder redirects Blender's user folder. On macOS it does not: with `HOME` pointed at an empty folder made for the test, all four Blenders still computed `/Users/artist/Library/Application Support/Blender/<version>` and read the Creator's own settings. `BLENDER_USER_RESOURCES` does redirect it on macOS, and every test in this sitting used that instead. Where a test needed several versions to share one parent folder — the way Blender's own layout puts `4.5`, `5.0`, `5.1` and `5.2` side by side — that layout was laid out by hand and each version pointed at its own folder inside it.

*Nothing is shared between versions by default.* Each of the four installed a differently-named package into its own folder, and then each was asked what it held. Each listed only its own: Blender 4.5.12 LTS showed `smoozy_v45 [installed]` and nothing else, and `SMOOZY_ADDONS ['bl_ext.user_default.smoozy_v45']`.

*Handing several versions one folder does make an add-on cross.* Four arrangements were run, installing with the oldest and reading with the other three, then installing with the newest and reading with the other three:

- One extensions folder handed to all four with `BLENDER_USER_EXTENSIONS`: every version listed it, and every version loaded it — `SMOOZY_ENABLE <module 'bl_ext.user_default.smoozy_ok' from '.../shared_ext/user_default/smoozy_ok/__init__.py'>`. Only the version that did the installing had it switched on in its own collection.
- One settings folder handed to all four with `BLENDER_USER_RESOURCES`: every version listed it, loaded it, **and** had it switched on — `SMOOZY_ADDONS ['bl_ext.user_default.smoozy_ok']` in all four.
- One system-extensions folder handed to all four with `BLENDER_SYSTEM_EXTENSIONS`: every version listed it under System and loaded it.
- One scripts folder handed to all four with `BLENDER_SYSTEM_SCRIPTS`, holding a legacy add-on: every version saw it among its modules and switched it on, printing its register line.

*A folder copied by hand also works.* An extension installed by 4.5 was copied, unchanged, into 5.0's, 5.1's and 5.2's folders; then the same the other way, out of 5.2 into the older three. Every destination version listed it and loaded it, in both directions.

*Two copies of one Blender.* A second copy of Blender 4.5.12 LTS was made in a folder of its own. With nothing overridden, the copy computed the same user folder as the original — `/Users/artist/Library/Application Support/Blender/4.5` — so two installations of one version share by default. A `portable` folder made inside the copy's bundle at `Contents/Resources/portable`, which is where the sources say it goes on macOS, took over completely:

```
SMOOZY_USER .../copy_4.5.app/Contents/Resources/portable
SMOOZY_EXT .../copy_4.5.app/Contents/Resources/portable/extensions
SMOOZY_CONFIG .../copy_4.5.app/Contents/Resources/portable/config
```

and the copy's listing then held nothing. `BLENDER_USER_RESOURCES` still wins over a `portable` folder: with both in play, the copy read the folder the variable named.

*A declared version range is acted on.* An extension declaring it needs at least Blender 99 was refused at install:

```
ERROR smoozy_min99: This Blender version (4.5.12) doesn't meet the minimum supported version (99.0.0)
```

and one declaring it does not support 4.3 and above was refused too:

```
ERROR smoozy_max40: This Blender version (4.5.12) must be less than the maximum version (4.3.0)
```

Neither reached the listing and neither could be switched on — `Add-on not loaded: "bl_ext.user_default.smoozy_min99", cause: No module named 'bl_ext.user_default.smoozy_min99'`, `SMOOZY_ENABLE None`. The command still exits zero. When `-e` is given, a second line follows the refusal, and its wording is one of the two real differences between the versions:

```
Package should have been installed but not found: smoozy_min99
```

Building and validating the same folder is refused by nothing at all: `Success parsing TOML in "."` and a package built, in every version.

*A legacy add-on's declared minimum is not acted on.* A one-file add-on declaring `"blender": (99, 0, 0)` was listed among the modules alongside the ordinary one, read back with its declaration intact, and switched on with no message:

```
SMOOZY_MODULES ['smoozy_legacy_min99', 'smoozy_legacy_ok']
SMOOZY_ENABLE <module 'smoozy_legacy_min99' from '.../smoozy_legacy_min99.py'>
```

*A bundled Python package built for another Python.* Three packages were built by hand, each carrying a wheel tagged for a different Python, and each was installed into a settings folder of its very own so that nothing could be left behind from a previous case. Blender 4.5.12 LTS accepted only the one tagged `cp311` — `STATUS Installed`, then the bundled module imported from `.../extensions/.local/lib/python3.11/site-packages/smoozypkg/__init__.py`. The other two were refused:

```
Error: Extension bl_ext.user_default.smoozy_wheel_cp313 is incompatible (This Python version (3.11) isn't compatible with (3.13))
```

with exit status 1, and the bundled module could not then be imported: `ModuleNotFoundError: No module named 'smoozypkg'`.

**The one place the Mac and Linux disagree: `bpy.ops.preferences.copy_prev()`.** On the Linux machine this operator ran with no window and carried an older version's installed extension into a newer version's folder. **On this Mac it will not run at all**, in any of the four versions:

```
SMOOZY_COPYPREV RuntimeError: Operator bpy.ops.preferences.copy_prev.poll() failed, context is incorrect
```

Three arrangements were tried and all three gave the same answer. Asked whether it could run at all, `bpy.ops.preferences.copy_prev.poll()` returned `False` — with the settings folder handed over by `BLENDER_USER_RESOURCES` and an older version's folder sitting beside it; with nothing overridden at all, where the Creator's own 4.5, 5.0 and 5.1 folders genuinely exist as earlier versions; and **with a window open on the Mac's screen**, which rules out the window being the cause. So the one documented route for carrying add-ons from one version to the next is reachable from a script on Linux and is not reachable on macOS. Two machines, two results; they are not merged.

> **Added by the citation check (2026-08-06).** Both halves were run again and both came back as written. One thing the record does not cover: on the *Linux* machine the operator also refuses when the settings folder is handed over with `BLENDER_USER_RESOURCES` — `SMOOZY_POLL False` and the same `context is incorrect` error — and runs only when the version folders sit in Blender's own layout under a stand-in home folder. So the arrangement decides it as well as the operating system. The comparison above still stands, because the Mac was also tried with nothing overridden; but it would be truer if it named the arrangement each run used. See test T-B in the citation check. **The two Linux cells were run deliberately on 2026-08-07 and confirm this** — see the observed section on the folder arrangements further down this answer, which sets the four cells out as a table. The two macOS cells were not run again and are quoted from the Mac record above.

**Not covered by this test:** nothing was run on Windows, because no Windows machine has run it, so the Windows paths and the Windows `portable` placement stay untested. Nothing here covers two different patch builds of one version, because this Mac holds exactly one build of each.


### Observed — `copy_prev()` on Linux under each folder arrangement

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — on 2026-08-07, with Blender 4.5.12 LTS at `/opt/blender/4.5/blender` and the other three beside it; all four were run. Every Blender was pointed at a settings folder or a stand-in home folder made for the test beforehand, inside a working folder of this test's own at `/root/again-add-ons/`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entry 29.

**Why this was run.** The citation check found that on this same Linux machine the operator also refuses when the settings folder is handed over with `BLENDER_USER_RESOURCES` — which is how nearly every test in this round hands it over — and asked for the four cells, two machines by two folder arrangements, to be run deliberately rather than read off runs that had been set up differently. The two Linux cells were run and are below. **The two macOS cells were not run again in this sitting; they are quoted from the Mac record in this same answer.**

**A — Blender's own layout, under a stand-in home folder.** A home folder of the test's own, with 4.5 installing the extension into it and then saving its preferences, so the layout is the one Blender itself computes. 5.0, 5.1 and 5.2 each ran the operator and each carried the extension across. From 5.0.1:

```
SMOOZY_CP_USER /root/again-add-ons/runs/29/A/home/.config/blender/5.0
SMOOZY_CP_POLL True
Info: Reload Start-Up file to restore settings
SMOOZY_CP_CALL {'FINISHED'}
```

and 5.0's own listing afterwards carried `smoozy_ok [installed]: "Smoozy Ok", A tiny add-on made for a test`. 5.1.2 and 5.2.0 LTS printed `SMOOZY_CP_POLL True` and `SMOOZY_CP_CALL {'FINISHED'}` and listed it too; the `Info: Reload Start-Up file to restore settings` line appeared in the 5.0 run only. 4.5.12 LTS, which has no earlier version on the machine, refused — `SMOOZY_CP_POLL False` and `SMOOZY_CP_RAISED RuntimeError Operator bpy.ops.preferences.copy_prev.poll() failed, context is incorrect`. Afterwards the home folder held four version folders side by side, `4.5`, `5.0`, `5.1` and `5.2`, each with its own `config/userpref.blend` and `extensions/`.

**B — the settings folder handed over with `BLENDER_USER_RESOURCES`.** A parent folder with `4.5`, `5.0`, `5.1` and `5.2` made by hand side by side, the extension installed into `4.5` and its preferences saved there, each version then pointed at its own folder inside that parent. **All four refused**, 5.0, 5.1 and 5.2 included:

```
SMOOZY_CP_USER /root/again-add-ons/runs/29/B/parent/5.0
SMOOZY_CP_POLL False
SMOOZY_CP_RAISED RuntimeError Operator bpy.ops.preferences.copy_prev.poll() failed, context is incorrect
SMOOZY_CP_ADDONS []
```

Nothing crossed. The three newer folders were still empty at the end, and each of their listings came back bare — `Repository: "User Default" (id=user_default)` with nothing under it, and `[Errno 2] No such file or directory: '<folder>/5.0/extensions/user_default'`. The path each version reads is the handed-over folder itself, with no version segment of Blender's own making.

**The four cells, in one table.** The Linux row is from this test, 2026-08-07. **The macOS row is quoted from the Mac record above and was not run again.**

| | Blender's own layout | settings folder handed over with `BLENDER_USER_RESOURCES` |
| --- | --- | --- |
| **Linux** — rented Ubuntu 22.04.5 LTS | **Runs.** `poll()` `True` and `{'FINISHED'}` in 5.0, 5.1 and 5.2; the extension crosses. 4.5 refuses, having no predecessor on the machine. | **Refuses.** `poll()` `False` and `context is incorrect` in all four; nothing crosses. |
| **macOS** — the Creator's MacBook Pro | **Refuses** in all four, with nothing overridden and the Creator's own earlier-version folders genuinely present. | **Refuses** in all four. |

So on Linux the folder arrangement decides it, and on macOS it refuses under both arrangements. The two machines still disagree, and the arrangement does not explain the disagreement away: it is only the Linux "runs" that needed the arrangement named. Two machines, two results; they are not merged.

**Not covered by this test:** nothing was run on macOS or Windows in this sitting — the macOS cells are quoted from the earlier Mac record, not re-run. Nothing here says *why* the operator refuses under either arrangement; only what it returns. Nothing here covers a Blender with a window, because the machine has no display server.


**Outside the interface.** **Settled by test on Linux, and on the Mac for everything but one item — see the two observed sections above.** All of it holds with no window, including the one part the sources leave interface-bound: `bpy.ops.preferences.copy_prev()` ran under `--background`, returned `{'FINISHED'}`, and carried the older version's installed extension into the newer version's folder. **On the Creator's Mac this does not hold for one item:** `bpy.ops.preferences.copy_prev()` refuses to run there in all four versions, with a window open as well as without one — see the Mac observed section above. What the sources say follows. **Brought up to date 2026-08-07.** The one interface-bound item has now been pinned down further on Linux. `bpy.ops.preferences.copy_prev()` is reachable from a script there **only when the version folders sit in Blender's own layout**; when the settings folder is handed over with `BLENDER_USER_RESOURCES` — the way nearly every test in this round hands it over — it refuses in all four versions, `poll()` returning `False`. So on Linux the folder arrangement decides whether this route can be driven from outside the interface at all. See the observed section on the folder arrangements above.

- The directory rules, the environment variables and the `--env-system-*` arguments are all things you set before Blender starts, so they apply identically whether a person is clicking or not. The `--addons` argument enables add-ons by name at launch (address above, read 2026-07-30).
- `blender --command extension list` lists "packages from all enabled repositories", and `repo-list` lists "all repositories stored in Blender's preferences" (Extensions Command Line Arguments, `https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html`, read 2026-07-30) — so which Blender holds what can be asked from outside.
- The one part that is not: "Import Preferences From Previous Version" is described as an option in the interactive region of the splash screen (Defaults, address above). The sources give no command-line equivalent, and I found none searching for one.

**Operating system.** **Partly settled by test — see the observed section above.** The per-version folder, the shared-folder routes and the `portable` folder have now been confirmed on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with the macOS paths coming out as the sources describe them, including the `portable` folder inside `Blender.app/Contents/Resources`. **The two machines disagree on one point:** `bpy.ops.preferences.copy_prev()` runs on Linux and refuses to run on macOS. Two further macOS-only points: a stand-in `HOME` does not redirect Blender's user folder there, though `BLENDER_USER_RESOURCES` does. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The rule is the same everywhere; the paths are not. **Brought up to date 2026-08-07.** The two Linux cells were run deliberately and the four cells now stand as a table in the observed section above: on Linux it runs under Blender's own layout and refuses when the settings folder is handed over; on macOS it refuses under both arrangements. The two machines still disagree, and the arrangement does not explain the disagreement away — only the Linux "runs" needed the arrangement named. The macOS cells were not run again in this sitting and are quoted from the Mac record. Windows is unchanged and was still not tried.

- The user directory and system directory paths are given separately for Linux, macOS and Windows, each with the version number in it (Blender's Directory Layout, address above, read 2026-07-30) — so the version-by-version separation holds on all three, at three different addresses.
- The `portable` folder sits in a different place on macOS: "macOS: Inside the application bundle at Blender.app/Contents/Resources", against "Next to the Blender executable, in the unzipped folder" on Windows and Linux (same page).
- An extension can declare which operating systems it is for: "platforms: List of supported platforms. If omitted, the extension will be available in all operating systems. The available options are ["windows-x64", "windows-arm64", "macos-x64", "macos-arm64", "linux-x64"]" (How to Create Extensions, address above, read 2026-07-30). So moving an add-on between two Blenders of *different* operating systems is a separate matter from moving it between two Blenders on one computer, and the manual treats it separately.
- Compiled parts are explicitly not portable across platforms: "Unlike regular Python scripts, these are not portable between different platforms." (Troubleshooting → Python Errors, address above, read 2026-07-30).
- Blender 4.5 is named as supported on Windows, macOS and Linux by the requirements page (`https://www.blender.org/download/requirements/`, read 2026-08-01).

**Where I looked.** Read in full: Blender's Directory Layout; Deploying Blender in Production; Defaults; Installing on Windows; Extensions → How to Create Extensions; Extensions → Add-ons; Extensions → Python Wheels; Extensions → Version Number Guidelines; Creating a Static Extensions Repository; Extensions Command Line Arguments; Command Line Arguments; Preferences → Add-ons; Preferences → Get Extensions; Preferences → Introduction; Troubleshooting → Python Errors; Troubleshooting → Startup; Add-on Tutorial; 4.5 release notes index, core and Python API; `cycles_4.5/__init__.py`. Searched all four version-4.5 source folders for: `blender_version_min`, `blender_version_max`, `blender_manifest`, `previous version`, `co-exist`, `side by side`, `multiple versions`, `separate folders`, `BLENDER_USER`, `BLENDER_SYSTEM`, `portable`, `platforms`, `wheels`, `python-use-system-env`, `bl_info`, `schema_version`, `compatib`, `breaking`, `deprecated`, `upgrade`, `migrat`. Ruled out `release_notes_4.5/rendering.md`, `cycles.md`, `pipeline_assets_io.md` and `user_interface.md`: searched each for `extension` and `add-on`; their add-on-related lines concern shader variants and importer/exporter behaviour, not version portability. Ruled out the requirements page as a source on anything but which operating systems 4.5 runs on: it names no add-on or extension anywhere.

---

## Things a person at a computer with Blender open could settle in a few minutes

Seven items were listed here. All seven have since been run on **both** machines — a rented Linux machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and their results are written into the answers above; they are not repeated. Nothing on this list is still open, and nothing on it needed a screen.

The one thing the list did **not** cover needed a screen: whether installing from Preferences while that same Blender renders behaves differently when the Blender has a window. That has since been run on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — in all four versions, with the interface lock off and on, and its result is written into question 12 above as an observed finding. **Nothing on this list is still open.** The only thing left untested anywhere is Windows, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

The seven as they were written, kept so the results above can be traced back to what was asked:

**T1 — Does installing an add-on into one Blender disturb a render already running in another? (Question 12)**
Start a Blender 4.5 rendering an animation from the command line, writing image files to a folder — long enough to last several minutes. While it runs, in a second terminal, run `blender --command extension install-file -r <repo> -e <some-package.zip>` using the same Blender 4.5 installation and the same user directory. Watch the rendering process: does it keep writing frames at the same rate, does it print anything new, does it exit early, do any frames come out wrong or missing? Then check the rendering process's exit code.
*Negative result:* the render finishes with the same exit code, the same number of frames and no new messages — meaning no disturbance was observable. That is a real outcome and it is what I would expect the test to show; the point of running it is that the opposite outcome (a stall, a message, a missing frame, a non-zero exit) is equally possible and nothing in the documentation rules it out.

**T2 — Does the repository file-system lock block the second one? (Question 12)**
Same as T1, but instead of a render, have the first process hold the lock: run `blender --command extension install ...` twice at once against the same repository, or use `bpy.ops.extensions.repo_lock_all()` in one Blender and then try to install from another. Record the exact message the second one gives.
*Negative result:* the second install proceeds normally with no message — meaning the lock does not guard against a second process. Also settles whether a rendering Blender holds such a lock at all: run T1 and then try `bpy.ops.extensions.repo_unlock()` and see whether it reports a lock existed.

**T3 — Does enabling a broken add-on report failure to a script? (Question 13)**
Make a small add-on whose `register()` raises an exception on purpose. Install it. Then run `blender --background --python-expr "import bpy; r = bpy.ops.preferences.addon_enable(module='<name>'); print('RESULT', r)"` and record both the printed result and everything else that reaches the terminal. Repeat with a working add-on and compare.
*Negative result:* both print the same result value and the broken one differs only in a traceback on the error stream — meaning there is no status to read and the console text is the only signal. If instead the two results differ, there is a machine-readable signal and this test finds it.

**T4 — Where do "Refresh Local" warnings go with no window? (Question 13)**
Install an add-on, then damage it on disk (delete a file it imports, or corrupt its manifest). Run `blender --background --python-expr "import bpy; print(bpy.ops.extensions.repo_refresh_all())"` and record everything printed.
*Negative result:* nothing is printed about the damage and the operator reports success — meaning the warnings the manual describes are shown only in a window, and this check is useless without one.

**T5 — Is an add-on installed under 4.5 visible to another Blender version on the same computer? (Question 14)**
On a computer with Blender 4.5 and one other version installed, install any extension in 4.5. Then run `blender --command extension list` with each of the two Blender executables in turn and compare the two listings. Do not change any environment variable.
*Negative result:* the extension appears in both listings — which would contradict the inference I drew from the version-named directories, and would be the more valuable finding of the two.

**T6 — Is a declared Blender version range enforced? (Question 14)**
Build a small extension with `blender_version_min = "9.9.9"` (a version that does not exist). Try `blender --command extension validate` on it, then try `blender --command extension install-file -r <repo> -e` on it with Blender 4.5, then try enabling it. Record what each step says.
*Negative result:* all three succeed silently and the add-on runs — meaning the field is a declaration only and Blender does not act on it. Either outcome is informative and the documentation states neither.

**T7 — Does pointing two Blender versions at one extensions directory work? (Question 14)**
Set `BLENDER_USER_EXTENSIONS` to the same directory for Blender 4.5 and for another installed version. Install an extension with one, then run `blender --command extension list` with the other, then enable it with the other and watch the terminal.
*Negative result:* the second Blender does not list it, or lists it but fails on enable with a message — meaning the directory alone does not make an add-on shared. If it lists and enables cleanly, sharing one directory across versions is possible and this settles it.

---

2026-08-06 — Opus 5 (Claude Code) — Add-ons, Blender 4.5

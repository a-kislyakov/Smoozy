# Add-ons — added questions (12, 13, 14), Blender 4.5

**Sources consulted:** Blender 4.5 manual (`Blender Sources/manual_4.5`, including one image from `_images`), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), Cycles' own files (`Blender Sources/cycles_4.5`) — all read on **2026-07-30**. Blender's requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`) — read on **2026-08-01**.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 12. Does installing an add-on disturb a render already running on the same computer?

**LABEL: not in the sources.**

**That label answers the question from the sources only. A test has since been run and settled it — see the observed section below in this same answer.**

The question can be read two ways, and neither is settled:

- **Reading A (the one I take to be meant):** a render is under way on the computer — in one Blender — and an add-on is installed by some other means (a second Blender, a command-line install, files copied into place). Does the render suffer?
- **Reading B:** a render is under way inside one Blender, and an add-on is installed inside that same Blender.

The sources say nothing about either. They describe what installing does, and they describe rendering, but they never put the two in the same sentence.

### Where I looked

Searched the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` for: `while rendering`, `during render`, `render in progress`, `already running`, `another instance`, `second instance`, `multiple instances`, `concurrent`, `simultaneous`, `at the same time`, `interfere`, `restart`, `lock`, `reload scripts`, `site-packages`, `sys.path`. Also searched for `install` inside the whole of `manual_4.5/render` and `manual_4.5/interface/window_system` — the only hits there are about installing graphics drivers, the CUDA toolkit, and application templates.

Read in full: the Add-ons preferences page, the Get Extensions preferences page, the Creating Extensions → Add-ons page, the Extensions getting-started page, the Extensions command-line page, the general command-line arguments page, Deploying Blender in Production, Blender's Directory Layout, the add-on tutorial, the Python-errors troubleshooting page, and the API pages `bpy.ops.preferences`, `bpy.ops.extensions`, `bpy.app.handlers`, `bpy.utils`, `info_overview`, `info_gotchas_threading`.

Ruled out: `release_notes_4.5` mentions extensions only three times, all in `bugfixes.md` (a repository-refresh bug, a `Python3.dll` subprocess-import bug, a theme-update-listing bug) — none about rendering. The Cycles files under `cycles_4.5` contain no install machinery at all; they are the render engine's own registration and settings.

### The nearest documented facts, so the gap is clear

These are what the sources *do* say near the question. None of them answers it.

- Extension repositories have a file-system lock, and there are operators to set and clear it. Exact words: *"Lock repositories - to test locking"* (`bpy.ops.extensions.repo_lock_all`) and *"Remove the repository file-system lock"* (`bpy.ops.extensions.repo_unlock`). Source: Python API, https://docs.blender.org/api/4.5/bpy.ops.extensions.html — read 2026-07-30. **LABEL: documented** that such a lock exists. What it protects against, and whether a rendering Blender ever takes it, is not stated.
- A command-line extension operation runs Blender in background mode. Exact words: *"Run a command which consumes all remaining arguments."* and *"This implies --background mode."* Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. **LABEL: documented** that the command implies background mode. That background mode means no window, and that this is a separate Blender from one already running, are steps the quoted words do not take.
- Add-ons are loaded at startup only if they were selected in the user preferences. Exact words: *"They are only loaded on startup if selected from the user preferences."* Source: Python API, https://docs.blender.org/api/4.5/info_overview.html — read 2026-07-30. **LABEL: documented.**
- Blender offers a command that scans add-on files for changes and reports any issues as warnings. Exact words: *"Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting). Any issues are reported as warnings."* Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/extensions.html — read 2026-07-30. **LABEL: documented** that the command exists and does that. **LABEL: inferred** for the step to *"a running Blender does not notice changes made to add-on files on disk until it is told to look"* — the step being that a command described as *"similar to restarting"* would have nothing to do if changes were picked up on their own. No source says it.
- Enabling an add-on is described as taking effect at once. Exact words: *"The add-on functionality should be immediately available."* Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html — read 2026-07-30. **LABEL: documented.** This is about enabling inside the Blender you are sitting in, not about a render elsewhere.
- The interface can be locked for the duration of a render. Exact words: *"Lock interface during rendering in favor of giving more memory to the renderer."* Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30, and identically at https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html (`use_lock_interface`). The pages do not say what the lock stops a person from doing, so I will not say.

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

**Not covered:** whether a running render is slowed, interrupted, corrupted or unaffected by an install; whether the repository lock blocks or delays anything a rendering Blender does; whether an install writing user preferences collides with a rendering Blender that later saves its own preferences; whether installing inside a rendering Blender is even permitted. All unanswered.

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every part of this question was reached with no window at all: the render, the installs, the removals, the lock, and an install fired from inside the rendering Blender by a handler. None of it needed a screen. **The windowed half has since been run on the Creator's Mac and agrees with the no-window half:** with a window open, an install fired from inside the rendering Blender returned `{'FINISHED'}`, every frame was written, and every frame matched its control picture for picture — with the interface lock off and on alike. What the sources say follows. The sources do not say. The command-line extension subcommands exist and imply background mode, but nothing states how they interact with a render running elsewhere. **Brought up to date 2026-08-07.** The interface-lock half has now been rebuilt with no window at all, with `use_lock_interface` read back at every step and from inside the render handler at the very frame the install fires — see the observed section on the rebuilt lock test above. With the lock genuinely on, the install returned `{'FINISHED'}`, all eight frames were written, and every frame matched its control picture for picture, in all four versions. Two further things came out of that read-back and bear on this line: `bpy.context.window_manager.is_interface_locked` and `bpy.app.is_job_running('RENDER')` both read `False` at every moment, including mid-render with the lock on, so neither gives a program outside the interface a view of the scene setting or of the running render. **The windowed form of the same test is still waiting on the Creator's Mac and stays sent back.**

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The sources do not say. The one operating-system-specific fact I found nearby is that the autosave directory is Windows-only and other systems use the temporary directory (https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html), which does not bear on this question. **Brought up to date 2026-08-07.** The rebuilt interface-lock test was run on the rented Linux machine only, in all four versions, and came out as the Linux runs above. The windowed form of it needs the Creator's Mac and has not been run again. Nothing here changes what stays open for Windows.

---

## 13. Once an add-on is installed and switched on, what more can be learned about whether it will work?

**LABEL: documented** (for each check below individually). The sources do not give a single "will it work" verdict; they give a set of separate things you can look at. I list each with its own words.

### 13.1 The console carries the errors

Exact words: *"If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred."*
Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html — read 2026-07-30. The same sentence appears at https://docs.blender.org/manual/en/4.5/editors/preferences/extensions.html and https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html.

So the checked box in the preferences is not itself proof: the manual assumes the case where an add-on is enabled and still does not activate.

### 13.2 Re-scanning reports problems as warnings

Exact words: *"Refresh Local — Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting). Any issues are reported as warnings."*
Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html — read 2026-07-30. Identical wording as an operator description at https://docs.blender.org/api/4.5/bpy.ops.extensions.html (`bpy.ops.extensions.repo_refresh_all`).

### 13.3 The add-on's own information box

Exact words: *"You can click the arrow at the left of the add-on box to see more information, such as its location, a description and a link to the documentation. Here you can also find a button to report a bug specific of this add-on."*
Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html — read 2026-07-30.

### 13.4 Enabled is not the same as switched on inside

Exact words: *"Some add-ons use this section for example to enable/disable certain functions of the add-on. Sometimes these might even all default to off. So it is important to check if the enabled add-on has any particular preferences."*
Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html — read 2026-07-30.

### 13.5 Where it landed, printed at install time

Exact words: *"The destination of the add-on depends on your Blender configuration. When installing an add-on the source and destination paths are printed in the console. You can also find add-on path locations by running this in the Python Console:"* followed by *"import addon_utils"* and *"print(addon_utils.paths())"*.
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html — read 2026-07-30.

### 13.6 Whether its bundled third-party modules actually loaded

Exact words: *"Once the extension has been installed you can check the module is being loaded by importing it in the Python console and printing it's location:"* followed by *"import PIL"* and *"print(PIL.__file__)"*.
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/python_wheels.html — read 2026-07-30.

Also from that page: *"Now installing the package will extract the wheel into the extensions own site-packages directory."* — so there is a place on disk to look.

### 13.7 Compiled parts may not match the computer

Exact words: *"While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular Python scripts, these are not portable between different platforms."* and *"If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating system."*
Source: manual, https://docs.blender.org/manual/en/4.5/troubleshooting/python.html — read 2026-07-30.

The same page gives the way to read a failure: *"To find out which Python Library caused the Problem check the error message."* and *"This is normally reported somewhere around the bottom line of the traceback."* It carries a picture, `[image: _images/troubleshooting_python_traceback.png]`, captioned *"A Python traceback."*

### 13.8 Whether it is allowed to reach the internet

Exact words: *"If the add-on needs to use internet, it must check for the (read-only) property bpy.app.online_access. This option is controlled by Preferences, which can be overriding via command-line (--offline-mode / --online-mode)."* and *"Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule."*
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html — read 2026-07-30.

The API gives the two readable properties: *"bpy.app.online_access — Boolean, true when internet access is allowed by Blender & 3rd party scripts (read-only)"* and *"bpy.app.online_access_override — Boolean, true when internet access preference is overridden by the command line (read-only)"*.
Source: Python API, https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30.

Worth noting for anyone who tries this: the manual page names the second property *"bpy.app.online_access_overriden"* while the API page names it *"bpy.app.online_access_override"*. The two documents disagree on the spelling. I report both as read; I did not run anything to settle which is real. (Only the API spelling is quoted in full above. The manual sentence carrying the other spelling is not quoted here, so the disagreement cannot be checked from this page alone.)

### 13.9 What is registered can be read back

Exact words: *"addons"* with type *"Addons bpy_prop_collection of Addon, (readonly)"* on `bpy.types.Preferences`; and on `bpy.types.Addon`, *"Python add-ons to be loaded automatically"*, with *"module — Module name"* and *"preferences — Type: AddonPreferences, (readonly)"*.
Source: Python API, https://docs.blender.org/api/4.5/bpy.types.Preferences.html and https://docs.blender.org/api/4.5/bpy.types.Addon.html — read 2026-07-30.

The manual shows the reading pattern in words: *"addon_prefs = bpy.context.preferences.addons[__package__]"*.
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html — read 2026-07-30.

**LABEL for the step from these to "you can therefore confirm from a script that an add-on is enabled": inferred.** The step: the collection is described as the add-ons "to be loaded automatically" and is indexed by package name, so a lookup that succeeds shows the add-on is in that collection. The sources do not say the collection is exactly the set of *working* add-ons, and do not say what the lookup does when the add-on failed to activate.

### 13.10 From outside, the list and the validator

Exact words: *"list: List all packages."* and *"List packages from all enabled repositories."*
Exact words: *"validate: Validate a package."*, *"Validate the package meta-data in the current directory."* and *"You may also validate a package without having to extract it first."* with *"blender --command extension validate add-on-package.zip"*.
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html and https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30.

Validation is about the manifest, not about the add-on running.

### 13.11 There is an error status, but it is undocumented

Exact words: *"bpy.ops.extensions.status_clear_errors()"* followed by *"Undocumented, consider contributing."*
Source: Python API, https://docs.blender.org/api/4.5/bpy.ops.extensions.html — read 2026-07-30. **LABEL: documented** that an operator of that name exists and is undescribed. **LABEL: inferred** for the step to "an error status exists in the extensions system" — the step being that the operator's *name* is read as evidence of the thing it names. The documentation says nothing about such a status. *(Changed by the citation check: this had been labelled documented; the quoted words are an operator's name and the sentence "Undocumented, consider contributing.", and getting from those to "an error status exists" needs a step.)*

### The picture I opened

`[image: _images/editors_preferences_section_extensions.png]`, captioned *"Blender Preferences Extensions section."*, on https://docs.blender.org/manual/en/4.5/editors/preferences/extensions.html. **LABEL: documented.** What is visible: a search field, a type filter reading "Add-ons", a "Repositories" menu, an "Installed" group holding one entry, and an "Available" group whose entries each carry an "Install" button. No health, error, or compatibility marking is visible on any row in this picture. I draw nothing further from it.

### 13.12 Observed — what Blender 4.5 actually reports, with no window

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the machine's own settings folder was never touched. The add-ons were written on the machine for the test, not fetched. The full procedure is in `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 12, 14, 15, 16, 17 and 24.

**Switching one on does report success or failure to the caller.** Four small extensions were installed — one that works, one whose `register()` raises, one that imports a module that is not there, one whose operator has `bl_idname = 1`. Each was then switched on with `bpy.ops.preferences.addon_enable(module=...)`. For the working one:

```
SMOOZY_OPS_RESULT {'FINISHED'}
```

For each of the three broken ones the operator raised instead of returning, and the message named the fault:

```
SMOOZY_OPS_RAISED RuntimeError Error: SMOOZY_BADREG_DELIBERATE_FAILURE
SMOOZY_OPS_RAISED RuntimeError Error: No module named 'smoozy_module_that_does_not_exist'
SMOOZY_OPS_RAISED RuntimeError Error: validating class: Operator.bl_idname expected a string type, not int
```

`addon_utils.enable()` returns the module for the working one and `None` for each broken one.

**Two collections tell a working add-on from a broken one — but only for a switch-on done in that same run.** After the attempts above, for the working one:

```
SMOOZY_IN_COLLECTION True
SMOOZY_CHECK (True, True)
```

and for each broken one:

```
SMOOZY_IN_COLLECTION False
SMOOZY_CHECK (False, False)
```

**But an add-on that was already switched on and then broke stays in the collection.** A working extension was installed and switched on, then damaged on disk, then Blender was started again. The failure was printed at start-up, and the collection still carried it:

```
SyntaxError: '(' was never closed
SMOOZY_ADDONS_AFTER ['bl_ext.user_default.smoozy_ok']
```

So `bpy.context.preferences.addons` records what has been asked for, not what loaded.

**The rescan reports nothing about damage.** With the same damaged add-on in place, `bpy.ops.extensions.repo_refresh_all()` returned:

```
SMOOZY_REFRESH {'FINISHED'}
```

and printed nothing naming the add-on. Three kinds of damage were tried — a syntax error in `__init__.py`, a deleted `blender_manifest.toml`, and a deleted `__init__.py`. In every case the fault text came from Blender's own start-up load, not from the rescan. Deleting the manifest produced, at start-up:

```
Warning: add-on missing manifest, this can cause poor performance!: '<settings folder>/extensions/user_default/smoozy_ok/blender_manifest.toml'
```

and after that the add-on vanished from `blender --command extension list` while still loading and registering. Deleting `__init__.py` produced:

```
Add-on not loaded: "bl_ext.user_default.smoozy_ok", cause: module loaded with no associated file, __path__=_NamespacePath(['<settings folder>/extensions/user_default/smoozy_ok']), aborting!
```

**The listing carries no health and no on-or-off marking.** With one extension switched on, one installed and left off, and one whose switch-on had failed, `blender --command extension list` printed all three the same way:

```
Repository: "User Default" (id=user_default)
  smoozy_badimport [installed]: "Smoozy Badimport", A tiny add-on made for a test
  smoozy_badreg [installed]: "Smoozy Badreg", A tiny add-on made for a test
  smoozy_ok [installed]: "Smoozy Ok", A tiny add-on made for a test
```

`blender --command extension list -s` did not run at all with no internet allowed: `RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences`, followed by `Error synchronizing`, exit status 1.

**`validate` can be pointed at an installed extension, and only parses the manifest.** Against the installed folder of a working extension and against the installed folder of the one whose `register()` raises, it printed the same thing and exited zero:

```
Success parsing TOML in "<settings folder>/extensions/user_default/smoozy_ok"
Success parsing TOML in "<settings folder>/extensions/user_default/smoozy_badreg"
```

With the manifest deleted it printed `FATAL_ERROR: Error, file "<settings folder>/extensions/user_default/smoozy_ok/blender_manifest.toml" not found!` and exited 1.

**The spelling disagreement is settled.** `print(hasattr(bpy.app, "online_access_override"), hasattr(bpy.app, "online_access_overriden"))` in a Blender with no window gave:

```
SMOOZY13 has_online_access_override -> True
SMOOZY13 has_online_access_overriden -> False
```

So the API page's spelling is the real one and the manual's is a typo. `bpy.app.online_access` read `False` with no window and no flag given.

**Which of the checks work with no window at all.** Every one of these returned rather than raising:

```
SMOOZY13 background -> True
SMOOZY13 addons_keys -> ['bl_ext.user_default.smoozy_ok']
SMOOZY13 addon_utils_paths -> ['/opt/blender/4.5/4.5/scripts/addons_core']
SMOOZY13 is_job_running_render -> False
SMOOZY13 extension_path_user -> '<settings folder>/extensions/.user/user_default/smoozy_ok'
SMOOZY13 workspace_filter -> False
SMOOZY13 repo_refresh_all -> {'FINISHED'}
SMOOZY13 addon_refresh -> {'FINISHED'}
SMOOZY13 addon_expand -> {'FINISHED'}
SMOOZY13 status_clear_errors -> {'FINISHED'}
SMOOZY13 addon_enable_args -> ['module', 'rna_type']
SMOOZY13 addon_prefs -> None    [citation check: not reproducible from this description alone — see T-D]
SMOOZY13 prefs_version -> (4, 5, 92)
SMOOZY13 addon_utils_check -> (True, True)
```

One did not:

```
SMOOZY13 wm_reports RAISED AttributeError 'WindowManager' object has no attribute 'reports'
```

**Logging arguments — and here 4.5 really does differ from the other three.** `blender --background --factory-startup --log-list-categories` is not accepted at all by 4.5: it is read as a file name.

```
unknown argument, loading as file: --log-list-categories
Error: Cannot read file "/root/--log-list-categories": No such file or directory
```

exit status 1. The same argument is accepted by 5.0, 5.1 and 5.2, each printing a list beginning `addon`, `anim`, `anim.action`. `--log-level 2` is accepted by all four, but only 5.0, 5.1 and 5.2 print anything extra at that level, in a timestamped form such as `00:00.340  context          | unknown:0: screen=<Screen("Layout")...>`. 4.5's own help text carries `--log-show-basename` and `--log-show-timestamp`, where the other three carry `--log-show-memory` and `--log-show-source` instead. `--addons`, `--python-exit-code`, `--offline-mode`, `--online-mode`, `--factory-startup`, `--env-system-scripts`, `--env-system-extensions` and `-c/--command` are in all four; 5.2 alone also gives `--factory-startup` the short form `-X`.

**Exit status.** A failed switch-on leaves the exit status at zero unless `--python-exit-code` is given and nothing catches the error. `blender --background --python-exit-code 1 --python-expr "import bpy; print('R', bpy.ops.preferences.addon_enable(module='bl_ext.user_default.smoozy_badreg'))"` exited 1; the same line without `--python-exit-code` exited 0; the same with `addon_utils.enable` instead exited 0 either way, because that helper swallows the error. Naming a broken add-on at launch with `--addons` printed its traceback and still exited 0, with or without `--python-exit-code 1`.

**`--addons` loads an add-on without recording it.** Launching with `--addons bl_ext.user_default.smoozy_ok`:

```
SMOOZY_OK_REGISTER
SMOOZY_LOADED True
SMOOZY_CHECK (True, True)
SMOOZY_COLL []
```

The add-on ran, but `bpy.context.preferences.addons` stayed empty.

**A legacy add-on's `warning` field reads back exactly as written.** With two legacy add-ons in a scripts folder, one with an empty `warning` and one with text in it, `addon_utils.module_bl_info` gave:

```
SMOOZY_BLINFO smoozy_legacy_ok {'name': 'Smoozy Legacy Ok', ..., 'warning': '', 'category': 'Development', 'location': '', 'doc_url': '', 'support': 'COMMUNITY', 'show_expanded': False, '_init': None}
SMOOZY_BLINFO smoozy_legacy_warn {'name': 'Smoozy Legacy Warn', ..., 'warning': 'SMOOZY WARNING TEXT', 'category': 'Development', 'location': '', 'doc_url': '', 'support': 'COMMUNITY', 'show_expanded': False, '_init': None}
```

Blender fills in `location`, `doc_url`, `support`, `show_expanded` and `_init` that the file never carried. Whether the warning text is *shown* anywhere has since been run on the Creator's Mac, which has a screen: it is shown, both as a triangle on the add-on's row and as a `Warning` line inside it. See the Mac observed section in this same answer.

**Not covered by this test:** whether a broken add-on is marked in the Add-ons list, and whether a non-empty `warning` is displayed, both need a screen, and both have since been run on the Creator's Mac; see the Mac observed section in this same answer. A declared warning is shown; a failure to switch on is not marked at all. Nothing here says what the extensions error status holds; `status_clear_errors` returned `{'FINISHED'}` and printed nothing. Nothing here was run on macOS or Windows.

**Not covered:** nothing in the sources says whether enabling reports success or failure back to a caller in a machine-readable way; nothing says whether errors go to standard output or standard error; nothing says whether the console warnings from Refresh Local are retrievable from a script; nothing describes the contents of the extensions error status; and nothing tells you, before the add-on is used, whether the part of it you actually need will work. Every check above is a check that something has already gone wrong, not a guarantee that nothing will.

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above (13.12).** Every Python check listed above works in a Blender started with no window: the add-ons collection, `addon_utils.paths()`, `addon_utils.check()`, `bpy.app.online_access`, `bpy.app.online_access_override`, `bpy.utils.extension_path_user`, `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()`, `bpy.ops.preferences.addon_expand()` and `bpy.ops.extensions.status_clear_errors()` all returned rather than raising. `bpy.context.window_manager.reports` does not exist in 4.5. What the sources say, and what is still open, follows.
- Partly yes, partly not stated. The console check (13.1) refers to the *"Console window"*, and the only place the manual explains that window is Windows-specific: *"To display the console again, go to Window ‣ Toggle System Console."* (https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, read 2026-07-30) — that is a windowed-Blender instruction.
- The command-line checks (13.10) are explicitly outside the interface: `--command` *"implies --background mode."* Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.
- The Python-console checks (13.5, 13.6) and the property reads (13.8, 13.9) are Python, so **LABEL: inferred** that they work from a script run without a window — the step being that `--python` runs scripts and `--background` exists, and the properties are plain read-only properties with no stated interface requirement. The sources do not say this in so many words.
- Refresh Local (13.2) and the information box (13.3) and add-on preferences (13.4) are described only as parts of the Preferences editor. `bpy.ops.extensions.repo_refresh_all` exists as an operator, but the sources do not say it can be called without a window.

**Operating system.** **Partly settled by test — see 13.12.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The console check is documented with a Windows-only instruction for showing the console (above). The precompiled-library check (13.7) is explicitly about the operating system and, on its page, one part is filed under a *"Windows"* heading: *"Mixed Python Libraries (DLLs)"*. Everything else above is stated without an operating-system qualifier; the sources do not say it differs.

---

## 14. Does an add-on set up for one Blender work in another Blender on the same computer, and what decides that?

**LABEL: documented**, with one inferred step marked below (14.2), one further inferred step (14.5), and one observed reading of a supplied file (14.5).

Short answer: an add-on installed in the normal way is set up **inside one Blender version's own folder**, so another Blender version does not see it until it is copied or the folder is deliberately shared. Whether it then *works* is decided by the version range the add-on declares, by the platform it declares, and by Python. **The first sentence is the inferred step set out in 14.2, not something a source says outright** — read the rest of this answer before leaning on it.

### 14.1 Several Blenders on one computer is a supported arrangement

Exact words: *"Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."*
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30.

### 14.2 Each version keeps its own folder, and installed add-ons live in it

Exact words: *"The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders."*
Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/configuration/defaults.html — read 2026-07-30.

The folders themselves carry the version number. Exact words, under **User Directories**: *"$HOME/.config/blender/4.5/"* (Linux), *"/Users/$USER/Library/Application Support/Blender/4.5/"* (macOS), *"%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\4.5\"* (Windows). And under **Path Layout**: *"./extensions — Extension repositories. Located in both user and system directories. Repositories are loaded from both directories."* and *"./scripts — Add-ons, presets, templates, user interface, startup scripts. Located in both user and system directories."*
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30.

The API says the same in code form: *"bpy.utils.resource_path(type, major=bpy.app.version[0], minor=bpy.app.version[1])"*, *"Return the base path for storing system files."*, with *"major (int) – major version, defaults to current."*
Source: Python API, https://docs.blender.org/api/4.5/bpy.utils.html — read 2026-07-30.

**LABEL for the step from these to "an add-on installed into 4.5 is not visible to a different Blender version by default": inferred.** The step: installed extensions live under the versioned user directory, and that directory's path carries the version number, so a different version reads a different directory. The sources state the separation for *configuration files* outright, and state that extensions live in those directories, but do not spell out the conclusion in one sentence.

### 14.3 Carrying them across is an offered operation, and it is not guaranteed to work

Exact words: *"Import Preferences From Previous Version — Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and startup files from the previous version of Blender and load them. This will include installed add-ons and extensions."*

And immediately after: *"Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this option may lead to errors on startup. If this occurs, the recommended first step is to try Loading Factory Settings."*
Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/configuration/defaults.html — read 2026-07-30.

### 14.4 What an extension declares about versions

Exact words, required value: *"blender_version_min: Minimum supported Blender version - use at least 4.2.0."*
Exact words, optional value: *"blender_version_max: Blender version that the extension does not support, earlier versions are supported."*
The example manifest carries *"blender_version_min = "4.2.0""* and the commented line *"# blender_version_max = "5.1.0""* introduced by *"# # Optional: Blender version that the extension does not support, earlier versions are supported."*
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30.

The same two names are used as download-link parameters so a repository can say which Blenders a package is for: *"{URL}.zip?repository={repository}&blender_version_min={version_min}&blender_max={version_max_exclusive}&platforms={platform1,platform2}"*.
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/static_repository.html — read 2026-07-30.

### 14.5 What a legacy add-on declares, and what happens when the Blender is too old

Exact words: *"bl_info is a dictionary containing add-on metadata such as the title, version and author to be displayed in the Preferences add-on list. It also specifies the minimum Blender version required to run the script; older versions won't display the add-on in the list."*
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html — read 2026-07-30.

This is the only statement I found in any source about what actually *happens* when a Blender is outside an add-on's declared version range: for a legacy add-on below the minimum, it is simply not listed.

**Observed**, from an add-on shipped in this folder — Cycles' own `bl_info`, showing a `blender` entry in a real add-on:

```
bl_info = {
    "name": "Cycles Render Engine",
    "author": "",
    "blender": (2, 80, 0),
```
File: `cycles_4.5/__init__.py`, lines 7–10 (the dictionary runs to line 16). Read 2026-07-30.

The file shows that entry and its value, and nothing more. That this particular entry is the minimum-version specification the manual describes is a step the file does not take, and the manual quote above names no entry by name. **LABEL for that step: inferred.**

### 14.6 Platform is declared too, and compiled parts do not travel

Exact words: *"platforms: List of supported platforms. If omitted, the extension will be available in all operating systems. The available options are ["windows-x64", "windows-arm64", "macos-x64", "macos-arm64", "linux-x64"]"*.
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30.

Exact words: *"Unlike regular Python scripts, these are not portable between different platforms."* and *"It is possible the library is incompatible with your Blender installation (attempting to load a library built for a different version of Python, or loading a 32-bit library on a 64-bit system)."*
Source: manual, https://docs.blender.org/manual/en/4.5/troubleshooting/python.html — read 2026-07-30.

### 14.7 Python ties an add-on to a particular Blender

Exact words: *"Blender and the bpy module are only compatible with a single Python version. This makes it possible for add-ons and VFX software in general to only have to target a single Python version."* and *"Blender bundles a complete Python installation and does not interact with the system Python by default."*
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30.

Wheels are named for the Python version they are built for; the manual's own example downloads them with *"--python-version=3.11"*.
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/python_wheels.html — read 2026-07-30.

### 14.8 Ways to make one add-on serve several Blenders on purpose

- Script Directories. Exact words: *"By adding a user script path in the preferences an additional directory is used. This can be used to store your own scripts and add-ons independently of the current Blender version."* Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html — read 2026-07-30. Note also, from the API: *"Alternate script path, matching the default layout with sub-directories: startup, add-ons, modules, and presets (requires restart)"* — https://docs.blender.org/api/4.5/bpy.types.ScriptDirectory.html.
- Environment variables. Exact words: *"BLENDER_USER_RESOURCES: Replace default directory of all user files. Other BLENDER_USER_* variables override when set."*, *"BLENDER_USER_EXTENSIONS: Directory for user extensions."*, *"BLENDER_SYSTEM_EXTENSIONS: Directory for system extensions repository."*, *"BLENDER_SYSTEM_SCRIPTS: Directories to add extra scripts."* Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.
- A shared read-only system repository. Exact words: *"When working offline or in a more controlled environment, it may be useful to provide a set of extensions to all users. For this there is a default read-only System repository. This repository could be located on a read-only network drive or in a system directory."* and *"$BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml"*. Source: manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30.

None of these pages says what happens when two different Blender versions are pointed at the same directory.

### 14.9 A second reading of "another Blender"

The question can also mean a second *copy of the same version* on the same computer rather than a different version. The sources cover one case of that: a portable copy keeps its own everything. Exact words: *"When running Blender from a portable drive, it's possible to keep the configuration files on the same drive to take with you. To enable this, create a folder named portable at the following locations..."* and *"This folder will then store preferences, startup file, installed extensions and presets."*
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30.

For two ordinary installations of the same version, the user directory path carries only the version number, not the installation — but the sources do not draw that conclusion, so I leave it as an open point rather than an inference.

### 14.10 One more thing that changed with versions

Exact words: *"With the introduction of Extensions in Blender 4.2, the old way of creating add-ons is considered deprecated. While the changes are rather small they impact existing add-ons."* and *"In order to allow a smooth transition process, the so-called legacy add-ons will continue to be supported by Blender."*
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html — read 2026-07-30.

The same page states that the repository name is part of an extension's module name: *"To prevent this conflict, the repository name is now part of the namespace."* and *"For example, now instead of kitsu the module name would be bl_ext.{repository_module_name}.kitsu instead."* with the warning *"The value of __package__ will vary between systems so it should never be replaced with a literal string."*

### 14.11 Observed — what four Blenders on one computer actually do

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — on 2026-08-07, with Blender 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS side by side at `/opt/blender/4.5/blender` and so on. Every Blender was pointed at a settings folder or a stand-in home folder made for the test beforehand; the machine's own settings folder was never touched. The add-ons were written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 1 to 11 and 18.

**Which Python each one carries.** `print(sys.version)` in a Blender with no window:

```
4.5.12 LTS  ->  3.11.11 (main, Apr 25 2025, 12:39:20) [GCC 11.2.1 20220127 (Red Hat 11.2.1-9)]
5.0.1       ->  3.11.13 (main, Apr 25 2025, 12:39:20) [GCC 11.2.1 20220127 (Red Hat 11.2.1-9)]
5.1.2       ->  3.13.9  (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-7)]
5.2.0 LTS   ->  3.13.13 (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-11)]
```

So 4.5 and 5.0 carry Python 3.11 and 5.1 and 5.2 carry Python 3.13, on this machine, for these builds.

**The folder each one reads carries two numbers and not three.** With `HOME` pointed at a folder of the test's own and nothing else set, 4.5 printed:

```
SMOOZY_VER (4, 5, 12)
SMOOZY_USER <home>/.config/blender/4.5
SMOOZY_LOCAL /opt/blender/4.5/4.5
SMOOZY_SYSTEM 
SMOOZY_EXT <home>/.config/blender/4.5/extensions
SMOOZY_SCRIPTS <home>/.config/blender/4.5/scripts
SMOOZY_CONFIG <home>/.config/blender/4.5/config
```

The other three printed the same shape with `5.0`, `5.1` and `5.2`. The patch number — 12 here — is in `bpy.app.version` and not in the path. Starting Blender did not create the folder; only an install or a saved preference did.

**Nothing is shared by default.** A differently-named extension was installed by each of the four into one shared home folder. Each Blender then listed only its own:

```
4.5 -> smoozy_v45 [installed]
5.0 -> smoozy_v50 [installed]
5.1 -> smoozy_v51 [installed]
5.2 -> smoozy_v52 [installed]
```

and four separate folders existed on disk, `blender/4.5/`, `blender/5.0/`, `blender/5.1/`, `blender/5.2/`, each holding its own `extensions/user_default/`. So the inferred step at 14.2 holds on this machine.

**Sharing a folder does work, and there are two different degrees of it.** With `BLENDER_USER_EXTENSIONS` pointed at one folder for all four but each keeping its own settings folder, an extension installed by 4.5 was listed by all four and could be switched on by all four — but only the Blender whose settings folder recorded it had it in `bpy.context.preferences.addons` at start-up. With `BLENDER_USER_RESOURCES` pointed at one folder for all four, all four listed it *and* all four had it switched on at start-up. The same held with 5.2 doing the installing and the older three reading.

**A shared system-extensions folder serves all four.** With the extension's own folder placed at `<folder>/system/smoozy_ok` and `BLENDER_SYSTEM_EXTENSIONS` set to `<folder>`, every one of the four listed it under `Repository: "System" (id=system)` and switched it on as `bl_ext.system.smoozy_ok`.

**A shared scripts folder serves all four, for a legacy add-on.** With `BLENDER_SYSTEM_SCRIPTS` set to a folder holding `addons/smoozy_legacy_ok.py`, all four printed `SMOOZY_MODULES ['smoozy_legacy_ok']` and switched it on.

**Two separate copies of one build share, and a `portable` folder takes over.** `/opt/blender/4.5` was copied to a second folder. An extension installed with the original was listed by the copy, and both resolved `bpy.utils.resource_path('USER')` to the same `<home>/.config/blender/4.5`. A `portable` folder made next to the copy's program changed that at once: the copy then resolved to `<copy>/portable` and listed nothing. The same happened for 5.0, 5.1 and 5.2.

**A folder copied by hand from one version to another works.** The installed folder of an extension was copied unchanged out of `blender/4.5/extensions/user_default/` into the matching place under `5.0`, `5.1` and `5.2`. Each then listed it and switched it on with no complaint. The same worked in the other direction, out of `5.2` into `4.5`, `5.0` and `5.1`.

**A declared version range is acted on, at install, and the message names the versions.** An extension declaring `blender_version_min = "99.0.0"` and one declaring `blender_version_max = "4.3.0"` were both built and validated without complaint, then offered to each Blender:

```
ERROR smoozy_min99: This Blender version (4.5.12) doesn't meet the minimum supported version (99.0.0)
ERROR smoozy_max40: This Blender version (4.5.12) must be less than the maximum version (4.3.0)
```

with the same shape from 5.0.1, 5.1.2 and 5.2.0. In each case nothing was installed, the listing stayed bare, and `addon_utils.enable` returned `None`. **The install command exits zero even so** — the exit status was 0 with the error printed. Adding `-e` produced a second line, and the wording of that second line differs between versions: 4.5 and 5.0 print `Package should have been installed but not found: smoozy_min99`, and 5.1 and 5.2 print `Package was expected to be installed but not found: smoozy_min99`.

**A legacy add-on's declared minimum is not acted on.** A one-file legacy add-on with `"blender": (99, 0, 0)` sat in a shared scripts folder beside a plain one. All four Blenders listed both:

```
SMOOZY_MODULES ['smoozy_legacy_min99', 'smoozy_legacy_ok']
```

and all four switched the 99 one on, ran its `register()` and put it in `bpy.context.preferences.addons`. So the manual's *"older versions won't display the add-on in the list"* did not hold here, in any of the four.

**A bundled Python package built for a different Python is refused, and the message names both.** Three copies of one extension were made, each carrying a hand-built wheel tagged `cp39`, `cp311` and `cp313`. Every one of the nine builds succeeded. The installs did not:

```
4.5 (Python 3.11): cp39  -> Error: Extension bl_ext.user_default.smoozy_wheel39 is incompatible (This Python version (3.11) isn't compatible with (3.9)), exit 1
4.5 (Python 3.11): cp311 -> installed, exit 0
4.5 (Python 3.11): cp313 -> Error: Extension bl_ext.user_default.smoozy_wheel313 is incompatible (This Python version (3.11) isn't compatible with (3.13)), exit 1
5.1 (Python 3.13): cp311 -> Error: Extension bl_ext.user_default.smoozy_wheel311 is incompatible (This Python version (3.13) isn't compatible with (3.11)), exit 1
5.1 (Python 3.13): cp313 -> installed, exit 0
```

Where the install succeeded, the bundled package could be imported and printed its place: `<settings folder>/extensions/.local/lib/python3.11/site-packages/smoozypkg/__init__.py` for 4.5 and 5.0, `python3.13` for 5.1 and 5.2. **One reading here is spoiled and is recorded rather than hidden:** in the 4.5 and 5.0 runs the `cp313` step reported the refusal above but the import still succeeded, because the earlier `cp311` install had left the package in that Python's site-packages folder and removing the extension had not cleared it. The refusal message and its exit status are clean; that one import result is not.

**`bpy.ops.preferences.copy_prev()` works with no window, for the three that have a predecessor.** With a home folder holding a set-up 4.5 and nothing else, 5.0 was started with no window:

```
Info: Reload Start-Up file to restore settings
SMOOZY_COPYPREV {'FINISHED'}
```

and afterwards `blender/5.0/` existed, held a `userpref.blend` and an `extensions/user_default/`, and 5.0's own listing carried the extension 4.5 had installed. 5.1 then picked up from 5.0 and 5.2 from 5.1, each returning `{'FINISHED'}`. 4.5, which had no earlier version on the machine, refused:

```
RuntimeError: Operator bpy.ops.preferences.copy_prev.poll() failed, context is incorrect
```

**Not covered by this test:** none of it was run on macOS or Windows, so the paths and the `portable` folder placement are confirmed for Linux only. Nothing here says what a *remote* repository does with a declared range, since the machine had no internet access allowed. Two builds of one major-and-minor version could not be tried, because the project has one build of each.

**Not covered:** what a Blender does at load time with an *extension* (not a legacy add-on) whose declared version range excludes it — whether it is hidden, refused, or listed with a warning — is not stated anywhere I looked; only the legacy case is stated (14.5). Nothing says whether `blender_version_max` is inclusive or exclusive as Blender reads it, though the download-link parameter is named *"version_max_exclusive"*. Nothing says what happens when two Blender versions share one extensions directory. Nothing says whether an add-on's stored preferences carry across versions separately from the add-on itself.

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


**Outside the interface.** **Settled by test on Linux, and on the Mac for everything but one item — see the two observed sections above (14.11).** Everything in this answer was reached with no window at all, including the one part the sources leave interface-bound: `bpy.ops.preferences.copy_prev()` ran in a Blender started with `--background`, returned `{'FINISHED'}`, and carried the older version's installed extension into the newer version's folder. **On the Creator's Mac this does not hold for one item:** `bpy.ops.preferences.copy_prev()` refuses to run there in all four versions, with a window open as well as without one — see the Mac observed section above. What the sources say follows. The version-keyed directories, the environment variables, the `--command extension` subcommands and the manifest fields are all described without reference to a window, and the environment variables and command-line arguments only exist for use from outside. The one part the sources leave interface-bound is 14.3: *"Import Preferences From Previous Version"* is an option on the splash screen — *"When you start Blender for the first time or update to a new version, the interactive region of the Splash Screen is replaced with a few initial preferences"* (https://docs.blender.org/manual/en/4.5/getting_started/configuration/defaults.html). The sources give no equivalent from outside the interface. Also relevant: *"--factory-startup: Skip reading the startup.blend in the users home directory."* (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html) and *"--addons <addon(s)>: Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons."* — the second is a way to name add-ons per run rather than per version. **Brought up to date 2026-08-07.** The one interface-bound item has now been pinned down further on Linux. `bpy.ops.preferences.copy_prev()` is reachable from a script there **only when the version folders sit in Blender's own layout**; when the settings folder is handed over with `BLENDER_USER_RESOURCES` — the way nearly every test in this round hands it over — it refuses in all four versions, `poll()` returning `False`. So on Linux the folder arrangement decides whether this route can be driven from outside the interface at all. See the observed section on the folder arrangements above.

**Operating system.** **Partly settled by test — see 14.11.** The per-version folder, the shared-folder routes and the `portable` folder have now been confirmed on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with the macOS paths coming out as the sources describe them, including the `portable` folder inside `Blender.app/Contents/Resources`. **The two machines disagree on one point:** `bpy.ops.preferences.copy_prev()` runs on Linux and refuses to run on macOS. Two further macOS-only points: a stand-in `HOME` does not redirect Blender's user folder there, though `BLENDER_USER_RESOURCES` does. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The version-keyed user directory holds on all three, but the path differs by operating system; the three paths are quoted in 14.2. The portable folder location also differs: *"Windows: Next to the Blender executable, in the unzipped folder"*, *"Linux: Next to the Blender executable, in the unzipped folder"*, *"macOS: Inside the application bundle at Blender.app/Contents/Resources"* — source: manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30. The declared-platform field (14.6) is by its nature operating-system-specific, and its five options are quoted above. From the requirements page, read 2026-08-01, https://www.blender.org/download/requirements/, the only statement naming this version in so many words is: *"Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."* — which bears on the `macos-x64` platform value being useful for 4.5. **Brought up to date 2026-08-07.** The two Linux cells were run deliberately and the four cells now stand as a table in the observed section above: on Linux it runs under Blender's own layout and refuses when the settings folder is handed over; on macOS it refuses under both arrangements. The two machines still disagree, and the arrangement does not explain the disagreement away — only the Linux "runs" needed the arrangement named. The macOS cells were not run again in this sitting and are quoted from the Mac record. Windows is unchanged and was still not tried.

---

## Things a person with Blender open could settle by hand

Ten items were listed here. Nine were run on a rented Linux machine, and every one of the ten has since been run on the Creator's Mac as well; their results are written into the answers above, at 13.12 and 14.11 and in question 12's observed section; they are not repeated here. The tenth needed a screen, which that machine does not have, and has since been run on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — in all four versions. **Nothing on this list is still open.** The only thing left untested anywhere is Windows, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

The ten, kept here so the answers above can be traced back to what was asked:

1. **Does a running render notice an install?** (question 12, reading A)
   Start Blender 4.5 from a terminal in background mode rendering a long animation: `blender --background scene.blend --render-anim`. While it is still running, in a second terminal, install an extension from a file into a user repository: `blender --command extension install-file -r <repo-id> -e <package>.zip`. Watch the first terminal to the end.
   *Negative result:* the first render finishes with the same frame count, writes every frame it was asked for, prints no new messages, and its pictures match those of the same render run with no install alongside it. Compare the pictures themselves, not the saved files byte for byte. A positive result would be an error, a stall, a crash, or a change in the render's pictures or timing.

2. **Does the repository lock stop anything, and does a rendering Blender hold it?** (question 12)
   Two parts, so that each can come out either way. The one-step version of this test — racing two installs against each other — is not worth running: two installs that each take a moment may never overlap, so nothing happening tells you nothing.
   *Part one, does the lock stop an install at all.* In a windowed Blender 4.5, take the lock from the Python console with `bpy.ops.extensions.repo_lock_all()`. Leave that Blender open. From a terminal run `blender --command extension install-file -r <repo-id> -e <package>.zip` and read what it prints. Then in the first Blender run `bpy.ops.extensions.repo_unlock()`, and run the same install again.
   *Negative result:* the install behaves identically both times — the lock stops nothing, and it cannot explain anything about installing during a render. A positive result is the first install refusing, waiting, or printing something about the repository being locked, where the second does not.
   *Part two, does a rendering Blender hold the lock.* Only worth running if part one came back positive. Start a long background render as in test 1, and while it is running try the same install from a second terminal.
   *Negative result:* the install behaves as it did with the lock free — a render does not hold the lock. Positive: it behaves as it did with the lock held.

3. **Does installing while a render runs inside the same Blender work at all?** (question 12, reading B)
   In a windowed Blender 4.5, start an animation render, then, while it renders, open Preferences ‣ Get Extensions ‣ Install from Disk and pick a package.
   *Negative result:* the menu item is available and the install completes without affecting the render. A positive result is the item being greyed out, the window refusing input, or the render failing.

4. **Does enabling report failure to a caller?** (question 13)
   Run `blender --background --python-expr "import bpy; print(bpy.ops.preferences.addon_enable(module='no_such_addon_at_all'))"` and read what is printed and what the process exit code is.
   *Negative result:* it prints a normal finished result and exits zero — i.e. failure is not reported back. A positive result is an exception, a cancelled result, or a non-zero exit code.

5. **Is an add-on that failed to activate still in the preferences list?** (question 13.9)
   Write a one-file add-on whose `register()` raises an error. Install and enable it, then run `print("test" in bpy.context.preferences.addons)` in the Python console (using its real module name).
   *Negative result:* it prints `False` — the collection excludes broken add-ons, so the lookup is a real health check. `True` means the lookup proves only that it was asked for, not that it works.

6. **Which spelling of the online-access override property exists?** (question 13.8)
   In the Python console run `import bpy; print(hasattr(bpy.app, "online_access_override"), hasattr(bpy.app, "online_access_overriden"))`.
   *Negative result:* `True False` — the API page is right and the manual page has a typo. `False True` would mean the opposite; `True True` would mean both exist.

7. **What does 4.5 do with an extension whose declared minimum is higher than 4.5?** (question 14.4)
   Build a trivial extension with `blender_manifest.toml` carrying `blender_version_min = "9.9.0"`, then `blender --command extension install-file -r <repo-id> -e it.zip`, then look at Preferences ‣ Get Extensions and Preferences ‣ Add-ons and at what the command printed.
   *Negative result:* it installs, is listed, and enables normally — the declared minimum is advisory only. Positive results are: refused at install, installed but not listed, or listed but not enableable. Repeat with `blender_version_max = "4.0.0"` for the other end.

8. **Do two Blender versions on one computer see each other's installed add-ons?** (question 14.2)
   With 4.5 and one other version installed, install an extension in 4.5, then start the other version and run `blender --command extension list` for each.
   *Negative result:* both listings contain the extension — the directories are shared after all. The expected positive result is that only 4.5's listing contains it.

9. **Does a shared Script Directory make one legacy add-on visible to two Blender versions?** (question 14.8)
   Create `my_scripts/addons/` containing one simple legacy add-on. In each of two Blender versions, add `my_scripts` under Preferences ‣ File Paths ‣ Script Directories, save preferences, restart, and look in Preferences ‣ Add-ons.
   *Negative result:* it appears in only one of the two — the shared directory does not work across versions. Positive: it appears in both.

10. **Does `BLENDER_USER_EXTENSIONS` let a second Blender version reuse the first's installed extensions?** (question 14.8)
    Start the other Blender version with `BLENDER_USER_EXTENSIONS` set to 4.5's user extensions directory, then run `blender --command extension list`.
    *Negative result:* the listing is empty or errors — pointing a different version at the folder does not make its contents usable. Positive: the extensions installed under 4.5 are listed.

---

2026-08-06 — Opus 5 (Claude Code) — Add-ons, Blender 4.5

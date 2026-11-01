# Add-ons — added questions (12, 13, 14), Blender 5.0

**Sources consulted, and when.** Blender's manual for 5.0 (`https://docs.blender.org/manual/en/5.0/`), Blender's Python API documentation for 5.0 (`https://docs.blender.org/api/5.0/`), the 5.0 release notes (`https://developer.blender.org/docs/release_notes/5.0/`), and Cycles' own files as shipped with 5.0 — all four read from the snapshot taken **2026-07-30**. Blender's requirements page (`https://www.blender.org/download/requirements/`), read from the snapshot taken **2026-08-01**.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One thing said plainly, so nobody has to wonder: to confirm the folder I was told to save into exists, I listed its file names. Five names were visible. I opened none of them and read no word of their contents, and nothing in this document comes from them. I mention it because "did not open" and "did not glance at the folder" are not the same claim, and you should have the smaller one from me rather than assume the larger.

---

## 12. Does installing an add-on disturb a render already running on the same computer?

**The question is ambiguous, and I say so plainly.** It can be read two ways:

- **(a)** A render is running inside one Blender, and an add-on is installed into *that same* Blender.
- **(b)** A render is running in one Blender, and an add-on is installed by *something else* on the same computer — a second Blender, or a command-line call.

The phrase "on the same computer" points at (b), so that is the reading I answer. I cover (a) as well, because the sources are equally silent on it and a reader may have meant it.

**LABEL: not in the sources.**

**That label answers the question from the sources only. A test has since been run and settled it — see the observed section below in this same answer.**

Nothing in the manual, the API documentation, the 5.0 release notes or Cycles' files says what happens to a render that is already running when an add-on is installed — under either reading. There is no statement that it is safe, none that it is unsafe, and none that names the two things together at all.

What the sources *do* say, that sits next to the question without answering it:

**There is a file-system lock on extension repositories.** Three operators exist for it:

> `bpy.ops.extensions.repo_lock_all()`
> Lock repositories - to test locking

> `bpy.ops.extensions.repo_unlock()`
> Remove the repository file-system lock

> `bpy.ops.extensions.repo_unlock_all()`
> Unlock repositories - to test unlocking

— Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.extensions.html`, read 2026-07-30.

That a lock exists is documented. Who takes it, when, what waits on it, and whether a render has anything to do with it are not documented anywhere I could find.

**Installing from the command line can be told to leave the preferences alone.** Every extension sub-command that would write preferences takes:

> `--no-prefs`
> Treat the user-preferences as read-only,
> preventing updates for operations that would otherwise modify them.
> This means removing extensions or repositories for example, wont update the user-preferences.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html`, read 2026-07-30.

**The manual says twice that preferences are saved without being asked, and the two statements do not agree with each other.**

> By default changes to preferences are saved on exit,
> this allows changes to the keymap and Quick Favorites menu to be stored and used between
> Blender Sessions.

— Manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/introduction.html`, read 2026-07-30.

> The preferences are automatically saved when changed.

— Manual, `https://docs.blender.org/manual/en/5.0/getting_started/configuration/defaults.html`, read 2026-07-30.

I quote both because they do not say the same thing, and I am not able to settle which governs. Neither mentions a render.

**Enabling is described as taking effect at once.** The sources do not say what that means for a render already under way.

> The add-on functionality should be immediately available.

— Manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, read 2026-07-30.

> When the add-on is enabled, Blender executes the code and runs the `register()` function.
> When the add-on is disabled, Blender runs the `unregister()` function.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/scripting/addon_tutorial.html`, read 2026-07-30.

**Changing data while a render runs is documented as dangerous, but only for handlers.** This is the nearest the sources come to reading (a), and it is not about installing anything:

> Altering data from handlers should be done carefully. While rendering the
> `frame_change_pre` and `frame_change_post` handlers are called from one
> thread and the viewport updates from a different thread. If the handler changes
> data that is accessed by the viewport, this can cause a crash of Blender. In
> such cases, lock the interface (Render → Lock Interface or
> `bpy.types.RenderSettings.use_lock_interface`) before starting a render.

— Python API documentation, `https://docs.blender.org/api/5.0/bpy.app.handlers.html`, read 2026-07-30.

**The manual does contemplate using the computer for other things while a render runs, with a caution about memory only.**

> Rendering takes all available CPU time; you should render overnight,
> when the computer is not needed, or set Blender to a low priority while rendering,
> and work on other things (be careful with the RAM space!).

— Manual, `https://docs.blender.org/manual/en/5.0/render/output/animation.html`, read 2026-07-30.

That is about competing for the machine, not about installing an add-on, and it stops short of naming any effect on the render's result.

### Where I looked, and what I searched for

Files opened in full or in part: manual pages `editors/preferences/addons`, `editors/preferences/extensions`, `editors/preferences/introduction`, `editors/preferences/save_load`, `editors/preferences/interface`, `advanced/extensions/addons`, `advanced/extensions/getting_started`, `advanced/extensions/index`, `advanced/extensions/python_wheels`, `advanced/extensions/version_number_guidelines`, `advanced/extensions/creating_repository/static_repository`, `advanced/extensions/creating_repository/dynamic_repository`, `advanced/command_line/index`, `advanced/command_line/arguments`, `advanced/command_line/extension_arguments`, `advanced/blender_directory_layout`, `advanced/deploying_blender`, `advanced/operators`, `advanced/scripting/addon_tutorial`, `getting_started/configuration/defaults`, `troubleshooting/index`, `troubleshooting/python`, `troubleshooting/startup`, `troubleshooting/network`, `render/output/animation`, `render/cycles/gpu_rendering`, `interface/window_system/topbar`. API pages `bpy.ops.extensions`, `bpy.ops.preferences`, `bpy.ops.script`, `bpy.app`, `bpy.app.handlers`, `bpy.utils`, `bpy.types.Addon`, `bpy.types.Addons`, `bpy.types.AddonPreferences`, `bpy.types.Preferences`, `bpy.types.PreferencesExtensions`, `bpy.types.UserExtensionRepo`, `info_overview`. Release notes `python_api`, `core`, `bugfixes`, `animation_rigging`, `pipeline_io`, `user_interface`. Cycles files `__init__.py`, `engine.py`, `properties.py`, `version_update.py`.

Terms searched across the whole of the 5.0 manual, the whole of the 5.0 API documentation, the whole of the 5.0 release notes and the Cycles files: *simultaneous*, *concurrent*, *another instance*, *multiple instances*, *at the same time*, *while rendering*, *during a render*, *render in progress*, *while blender is running*, *another blender*, *second instance*, *other processes*, *interfere*, *interrupt*, *disturb*, *affect a render*, *running render*, *render job*, *background job*, *does not affect*, *safe to*, *lock*, *locked*, *locking*, *file-system lock*, *restart*, *reload scripts*.

Ruled out and why: the render chapter of the manual describes what a render does with a scene, never what other software on the machine may do to it. The troubleshooting chapter covers startup, graphics hardware, crashes, network, Python errors, recovering data and bug reporting — I read all of its index and the three pages that could plausibly touch this (`startup`, `python`, `network`) and none mentions installation during a render. The extensions command-line page documents every sub-command and its options; none carries a warning, precondition or note about anything else running. The `repo_lock_all` / `repo_unlock` operators are the only lock in the sources and their own descriptions say they are "to test locking".

### Observed — an install fired at a render that is already running

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 5.0.1 at `/opt/blender/5.0/blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. The machine is shared with other topics' tests, so **every time below is from a shared machine and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 19, 20, 21, 22 and 23.

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
Error locking repository "User Default": lock exists (lock is held by other session "/tmp/blender_TSIhVc/")
Error: Repository "User Default": lock exists (lock is held by other session "/tmp/blender_TSIhVc/")
If the lock was held by a Blender instance that exited unexpectedly,
use: "Force Unlock Repository" to clear the lock.
Access from the "Repositories" popover in the extensions preferences.
RepoLock: freed without releasing lock!
```

with exit status 1, in 0.697 seconds. The very same command with nothing holding the lock printed `STATUS Installed "smoozy_ok"` and exited zero, in 1.018 seconds. So it does not wait — it refuses, and it names the session holding the lock. Both timings are from a machine shared with other topics' tests.

*Part three — a Blender that is rendering does not hold the lock.* While a background animation render was under way in the same settings folder, a second Blender took the lock without difficulty. That is why the render in the runs above was never disturbed by the installs: a rendering Blender is not holding the thing the installs need.

**Not covered by this test:** none of it was run on macOS or Windows. The macOS half has since been run and follows in the next section. The machine was shared with other topics' tests throughout, so the per-frame times are recorded but not read as measurements. Nothing here covers a windowed Blender, which the machine cannot open; that has since been run on the Creator's Mac and is recorded in the Mac observed section in this same answer. `bpy.context.window_manager.reports` does not exist in this version.

**Not covered.** This answer does not settle whether the install would be blocked, would succeed, would corrupt anything, or would go unnoticed by the render. It does not settle whether the running Blender re-reads the extensions directory or the preferences file at any point during a render. It does not settle what a second Blender writing `userpref.blend` does to a first Blender that is mid-render. And it does not settle reading (a) either — whether the operators that install and enable can even be run while that same Blender is rendering.

### Observed — the same question asked again on a Mac, including with a window

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 19, 20, 21, 22, 23 and 25.

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
PIXELCMP .../control_5.0/f_0001.png .../test_5.0/f_0001.png differing_values 0 of 307200 worst 0.0
```

and the same for frames 2 to 8. Setting aside the file names, no line appeared in one run's output that was not in the other's.

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

*With a window — the part no machine could reach before.* Blender 5.0.1 was started with a window on the Mac's screen and driven by a script that called Blender's own commands and took Blender's own screenshots. An animation render was started, and after the second frame a handler installed a package:

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

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 5.0.1 at `/opt/blender/5.0/blender`, on 2026-08-07. All four versions were run and all four agreed. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`, inside a working folder of this test's own at `/root/again-add-ons/`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. Other topics' tests were running Blender on the same machine throughout, so **no time here is read as a measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entry 28.

**Why this was run again.** The citation check sent the windowed `use_lock_interface` run back, because nothing in its record showed the setting was in force at the moment the install fired — see the note in the Mac section above. This is that test rebuilt in the form a machine with no screen can reach. **The windowed form still needs the Creator's Mac and has not been run again; that half stays sent back.**

**What was done.** The eight-frame scene was rendered three times in Blender 5.0.1: once undisturbed as a control; once with `use_lock_interface` left `False` and a package installed from a `render_post` handler after the second frame; and once with `use_lock_interface` set to `True` and the same install. Cycles on the processor, eight of the twenty-four cores, 512 samples, adaptive sampling and denoising off, a fixed seed, 640 by 480. In every run the flag was printed back before the assignment, after it, at each of the eight frames **from inside the handler**, immediately after the install, and after the render.

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every part of this question was reached with no window at all: the render, the installs, the removals, and an install fired from inside the rendering Blender by a handler. None of it needed a screen. **The windowed half has since been run on the Creator's Mac and agrees with the no-window half:** with a window open, an install fired from inside the rendering Blender returned `{'FINISHED'}`, every frame was written, and every frame matched its control picture for picture — with the interface lock off and on alike. What the sources say follows. The sources do not say. The command-line install path exists and is fully documented (`blender --command extension install`, `install-file`, with `--no-prefs`), and preferences can be treated as read-only there, but nothing states how any of it behaves against a render already running. **Brought up to date 2026-08-07.** The interface-lock half has now been rebuilt with no window at all, with `use_lock_interface` read back at every step and from inside the render handler at the very frame the install fires — see the observed section on the rebuilt lock test above. With the lock genuinely on, the install returned `{'FINISHED'}`, all eight frames were written, and every frame matched its control picture for picture, in all four versions. Two further things came out of that read-back and bear on this line: `bpy.context.window_manager.is_interface_locked` and `bpy.app.is_job_running('RENDER')` both read `False` at every moment, including mid-render with the lock on, so neither gives a program outside the interface a view of the scene setting or of the running render. **The windowed form of the same test is still waiting on the Creator's Mac and stays sent back.**

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The sources do not say. Nothing about installation, locking or rendering in these pages is qualified by operating system. **Brought up to date 2026-08-07.** The rebuilt interface-lock test was run on the rented Linux machine only, in all four versions, and came out as the Linux runs above. The windowed form of it needs the Creator's Mac and has not been run again. Nothing here changes what stays open for Windows.

---

## 13. Once an add-on is installed and switched on, what more can be learned about whether it will work?

**LABEL: documented** — for the list of checks below, each of which the sources describe outright. Note carefully: the sources describe these as ways to *find problems*. Nowhere do they state that passing them means the add-on will work.

There are nine documented things a person or a script can go and look at after an add-on has been installed and enabled.

**1. The console, for errors raised while enabling.** The same sentence appears on three pages:

> If the Add-on does not activate when enabled,
> check the Console window
> for any errors that may have occurred.

— Manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, `https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html` and `https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html`, read 2026-07-30.

What that console is:

> The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html`, read 2026-07-30.

**2. A rescan that reports its own findings as warnings.**

> Refresh Local
>
> Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting).
> Any issues are reported as warnings.

— Manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, read 2026-07-30. The same wording is on the extensions page and on the operator behind the button:

> `bpy.ops.extensions.repo_refresh_all(*, use_active_only=False)`
> Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting). Any issues are reported as warnings

— Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.extensions.html`, read 2026-07-30.

**3. The add-on's own information box.**

> You can click the arrow at the left of the add-on box to see more information,
> such as its location, a description and a link to the documentation.
> Here you can also find a button to report a bug specific of this add-on.

— Manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, read 2026-07-30.

**4. The add-on's preferences, which may be the reason it does nothing.**

> Some add-ons may have their own preferences which can be found
> in the Preferences section of the add-on information box.
>
> Some add-ons use this section for example to enable/disable
> certain functions of the add-on. Sometimes these might even all default to off.
> So it is important to check if the enabled add-on has any particular preferences.

— Manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, read 2026-07-30.

**5. Whether a bundled third-party module actually loaded.** This is the sources' own suggested check, and it is a real one:

> Running
>
> Once the extension has been installed you can check the module is being loaded by importing it in the
> Python console and printing it's location:
>
> ```
> import PIL
> print(PIL.__file__)
> ```

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/python_wheels.html`, read 2026-07-30.

**6. Where it actually landed.**

> The destination of the add-on depends on your Blender configuration.
> When installing an add-on the source and destination paths are printed in the console.
> You can also find add-on path locations by running this in the Python Console:
>
> ```
> import addon_utils
> print(addon_utils.paths())
> ```

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/scripting/addon_tutorial.html`, read 2026-07-30.

**7. What Blender checks when the add-on's classes are loaded, and what it says when they fail.**

> When loading a class, Blender performs sanity checks making sure all required properties and functions are found,
> that properties have the correct type, and that functions have the right number of arguments.
>
> Mostly you will not need concern yourself with this but if there is a problem
> with the class definition it will be raised on registering:
>
> Using the function arguments `def execute(self, context, spam)`, will raise an exception:
>
> `ValueError: expected Operator, SimpleOperator class "execute" function to have 2 args, found 3`
>
> Using `bl_idname = 1` will raise:
>
> `TypeError: validating class error: Operator.bl_idname expected a string type, not int`

— Python API documentation, `https://docs.blender.org/api/5.0/info_overview.html`, read 2026-07-30.

**8. The list of what is installed, and where from.** Two sub-commands, both usable without opening Blender's window:

> Subcommand: `list`
> `blender --command extension list [-h] [-s]`
> List packages from all enabled repositories.

> Subcommand: `repo-list`
> `blender --command extension repo-list [-h]`
> List all repositories stored in Blender's preferences.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html`, read 2026-07-30.

In Python, the enabled add-ons are a readable collection:

> `addons`
> Type: `Addons` `bpy_prop_collection` of `Addon`, (readonly)

— Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Preferences.html`, read 2026-07-30. Each entry carries a `module` (string) and a `preferences` — Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Addon.html`, read 2026-07-30.

**9. Asking Blender to report more than it normally does.**

> `--log <match>`
> Enable logging categories, taking a single comma separated argument.
>
> `--log-level <level>`
> Set the logging verbosity level.
> fatal: Fatal errors only / error: Errors only / warning: Warnings / info: Information about devices, files, configuration, operations / debug: Verbose messages for developers / trace: Very verbose code execution tracing
>
> `--log-file <filepath>`
> Set a file to output the log to.
>
> `--log-list-categories`
> List all available logging categories for `--log`, and exit.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, read 2026-07-30.

And for a script that must fail loudly:

> `--python-exit-code <code>`
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables.

— Same page and date.

The 5.0 release notes state that this reporting changed in this version:

> Most of Blender now uses the same logging system,
> including operators reports and background render progress.
> The output format has changed, and log levels are easier to control.

— Release notes, `https://developer.blender.org/docs/release_notes/5.0/core/`, read 2026-07-30.

**Two more things the sources name as causes of an installed, enabled add-on still not working — worth checking, though the sources do not describe a check for them.**

Compiled parts:

> While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular
> Python scripts, these are not portable between different platforms.
>
> It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system).
>
> If the add-on contains `.pyd` or `.so` files, check that the distribution is compatible with your operating
> system.

— Manual, `https://docs.blender.org/manual/en/5.0/troubleshooting/python.html`, read 2026-07-30.

Internet access:

> If the add-on needs to use internet, it must check for the (read-only) property `bpy.app.online_access`.
> This option is controlled by Preferences, which can be overriding via command-line
> (`--offline-mode` / `--online-mode`).

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html`, read 2026-07-30.

> `bpy.app.online_access`
> Boolean, true when internet access is allowed by Blender & 3rd party scripts (read-only).

— Python API documentation, `https://docs.blender.org/api/5.0/bpy.app.html`, read 2026-07-30.

**One check that is not "after install".** `blender --command extension validate` exists, but it validates a package's manifest, not an installed and enabled add-on:

> Validate the package meta-data in the current directory.
> ... The package source path (either directory containing package files or the package archive).

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html`, read 2026-07-30. I list it only to say it does not answer this question.

**Picture consulted.** `_images/editors_preferences_section_addons.webp`, shown on `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, read 2026-07-30. **LABEL for what follows: documented** (the picture is part of the manual). One caution about this entry, since it is unlike every other one here: a picture has no words to quote, so what follows is my description of what the picture shows, not the manual speaking. Anyone checking it has to open the picture and look, not read a sentence. What I see: the add-ons list, with a search field, an "Enabled Only" checkbox, a tag filter, and one row per add-on carrying an expand arrow, a tick-box, the name, and an icon at the right-hand end. Enabled rows are drawn bright and unticked rows dim. **No status, health or warning column appears in this picture.** I do not read anything further into it — the picture shows a settled state, not what appears when something goes wrong.

### Observed — what Blender 5.0 actually reports about an add-on, with no window

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 5.0.1 at `/opt/blender/5.0/blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the machine's own settings folder was never touched. The add-ons were written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 12, 13, 14, 15, 16, 17 and 24.

**There is a machine-readable signal, and it is the operator raising.** Four small extensions were installed — one that works, one whose `register()` raises, one importing a module that is not there, one with `bl_idname = 1` — then each switched on with `bpy.ops.preferences.addon_enable(module=...)`. The working one returned `SMOOZY_OPS_RESULT {'FINISHED'}`. Each broken one raised instead, naming the fault:

```
SMOOZY_OPS_RAISED RuntimeError Error: SMOOZY_BADREG_DELIBERATE_FAILURE
SMOOZY_OPS_RAISED RuntimeError Error: No module named 'smoozy_module_that_does_not_exist'
SMOOZY_OPS_RAISED RuntimeError Error: validating class: Operator.bl_idname expected a string type, not int
```

`addon_utils.enable()` returned the module for the working one and `None` for each broken one, swallowing the error rather than raising. Full tracebacks went to the error stream; a single `Error: ...` line went to the ordinary output.

**Two collections tell them apart — but only for a switch-on made in the same run.** After each attempt, `mod in bpy.context.preferences.addons` and `addon_utils.check(mod)` gave `True` and `(True, True)` for the working one, `False` and `(False, False)` for each broken one.

**An add-on that was already switched on and then broke stays in the collection.** A working extension was installed with `-e`, its `__init__.py` replaced with `this is not python(`, and Blender started again. The failure was printed at start-up — `SyntaxError: '(' was never closed` — and the collection still carried it: `SMOOZY_ADDONS_AFTER ['bl_ext.user_default.smoozy_ok']`. So the collection records what has been asked for, not what loaded.

**The rescan reports nothing about damage.** With that damage in place, `bpy.ops.extensions.repo_refresh_all()` returned `SMOOZY_REFRESH {'FINISHED'}` and printed nothing naming the add-on. The same for a deleted `blender_manifest.toml` and a deleted `__init__.py`. All three faults were reported by Blender's own start-up load instead:

```
Warning: add-on missing manifest, this can cause poor performance!: '<settings folder>/extensions/user_default/smoozy_ok/blender_manifest.toml'
Add-on not loaded: "bl_ext.user_default.smoozy_ok", cause: module loaded with no associated file, __path__=_NamespacePath(['<settings folder>/extensions/user_default/smoozy_ok']), aborting!
```

With the manifest gone the extension vanished from `blender --command extension list` while still loading and registering.

**The listing carries no health and no on-or-off marking.** With one extension switched on, one installed and left off, and one whose switch-on had failed, all three printed the same way:

```
Repository: "User Default" (id=user_default)
  smoozy_badimport [installed]: "Smoozy Badimport", A tiny add-on made for a test
  smoozy_badreg [installed]: "Smoozy Badreg", A tiny add-on made for a test
  smoozy_ok [installed]: "Smoozy Ok", A tiny add-on made for a test
```

`blender --command extension list -s` did not run at all with no internet allowed: `RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences`, then `Error synchronizing`, exit status 1.

**`validate` can be pointed at an installed extension, and only parses the manifest.** Against the installed folder of the working extension and against the installed folder of the one whose `register()` raises, it printed `Success parsing TOML in "..."` for both and exited zero. With the manifest deleted: `FATAL_ERROR: Error, file "..." not found!`, exit status 1.

**The spelling disagreement between the manual and the API page is settled.** `hasattr(bpy.app, "online_access_override")` gave `True` and `hasattr(bpy.app, "online_access_overriden")` gave `False`, in this version and in the other three. `bpy.app.online_access` read `False` with no window and no flag given.

**Which checks work with no window at all.** Every one of these returned rather than raising: `bpy.app.background` (`True`), `bpy.context.preferences.addons.keys()`, `addon_utils.paths()` (`['/opt/blender/5.0/5.0/scripts/addons_core']`), `bpy.app.online_access`, `bpy.app.is_job_running('RENDER')`, `bpy.utils.extension_path_user(...)` (`'<settings folder>/extensions/.user/user_default/smoozy_ok'`), `bpy.data.workspaces[0].use_filter_by_owner`, `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()`, `bpy.ops.preferences.addon_expand(module=...)`, `bpy.ops.extensions.status_clear_errors()`, `bpy.context.preferences.version` (`(5, 0, 119)`) and `addon_utils.check(...)`. One did not: `SMOOZY13 wm_reports RAISED AttributeError 'WindowManager' object has no attribute 'reports'`.

`bpy.ops.preferences.addon_enable` takes exactly `['module', 'rna_type']`, so the argument name used above is the real one.

**Logging arguments.** `--log-list-categories` is accepted and prints its categories, beginning `addon`, `anim`, `anim.action`, and `--log-level 2` prints timestamped lines such as `00:00.340  context          | unknown:0: screen=<Screen("Layout")...>`. In 4.5 the same argument is not accepted at all.

**Exit status.** A failed switch-on leaves the exit status at zero unless `--python-exit-code 1` is given *and* the error is left uncaught, in which case it is 1. The same failure through `addon_utils.enable` exits 0 either way. Naming a broken add-on at launch with `--addons` prints the traceback and exits 0 in both cases. Naming a *working* add-on with `--addons` loads and registers it but leaves `bpy.context.preferences.addons` empty: `SMOOZY_LOADED True`, `SMOOZY_CHECK (True, True)`, `SMOOZY_COLL []`.

**A legacy add-on's `warning` field reads back exactly as written**, alongside five keys Blender fills in that the file never carried:

```
SMOOZY_BLINFO smoozy_legacy_warn {'name': 'Smoozy Legacy Warn', ..., 'warning': 'SMOOZY WARNING TEXT', 'category': 'Development', 'location': '', 'doc_url': '', 'support': 'COMMUNITY', 'show_expanded': False, '_init': None}
```

Whether that text is *shown* anywhere has since been run on the Creator's Mac, which has a screen: it is shown, both as a triangle on the add-on's row and as a `Warning` line inside it. See the Mac observed section in this same answer.

**Not covered by this test:** whether a broken add-on is marked in the Add-ons list, and whether a non-empty `warning` is displayed — both need a window, and both have since been run on the Creator's Mac; see the Mac observed section in this same answer. A declared warning is shown; a failure to switch on is not marked at all. What the extensions error status holds; `status_clear_errors` returned `{'FINISHED'}` and printed nothing. Nothing here was run on macOS or Windows.

**Not covered.** This answer lists what can be looked at. It does not settle what any of it means: the sources nowhere say that a silent console, a clean Refresh Local, and a successful import together imply the add-on will work — nor that a warning implies it will not. It does not settle whether an add-on that registers cleanly will also produce correct results in a render, which is a different question. It does not settle whether the declared `permissions` in an extension's manifest are visible anywhere after installation. And it does not settle whether `bpy.ops.preferences.addon_enable` reports failure in a way a script can act on — the operator's documentation is one line, "Turn on this add-on", with no return described (`https://docs.blender.org/api/5.0/bpy.ops.preferences.html`, read 2026-07-30).

### Observed — the same checks asked again on a Mac, including the Add-ons list on screen

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 12, 13, 14, 15, 16, 17, 24 and 26.

**The Mac agreed with Linux on every check.** What follows is what Blender 5.0.1 printed on the Mac.

*Every check, with no window.* Seventeen checks were run in one script in a Blender started with `--background`. All of them returned rather than raising, except one:

```
SMOOZY13 background -> True
SMOOZY13 addons_keys -> ['bl_ext.user_default.smoozy_ok', 'bl_pkg', 'cycles', ...]
SMOOZY13 addon_utils_paths -> ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/scripts/addons_core']
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
SMOOZY13 prefs_version -> (5, 0, 119)
```

`bpy.context.window_manager.reports` raised — `SMOOZY13 wm_reports RAISED AttributeError 'WindowManager' object has no attribute 'reports'`. On the two spellings the documents disagreed about: `bpy.app.online_access_override` exists and `bpy.app.online_access_overriden` does not.

*Command-line arguments.* `--log-list-categories` was **accepted**, exit status 0, printing a list beginning `addon`, `anim`, `anim.action`. The help text of this version carries `--log`, `--log-level`, `--log-show-memory`, `--log-show-source`, `--log-show-backtrace`, `--log-file` and `--log-list-categories`.

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

*On screen — the part no machine could reach before.* Blender 5.0.1 was started with a window, four legacy add-ons were switched on from a script, the Add-ons section of Preferences was opened and searched for "smoozy", and Blender took its own screenshots. Three things are visible in them.

- **A declared warning is shown.** The row for the add-on carrying `"warning": "SMOOZY WARNING TEXT"` has a warning triangle at its right-hand end where the other rows have the plain legacy-add-on icon, and when the row is opened it carries a line reading `Warning` followed by the same triangle and the words `SMOOZY WARNING TEXT`.
- **An add-on that failed to switch on is not marked at all.** Its row looks exactly like a row for an add-on simply left switched off: the tick box is empty, there is no triangle, and opening the row shows only the ordinary Type, Maintainer, Version and File lines. Nothing on screen distinguishes "it broke" from "it is off".
- **The one declaring it needs Blender 99 is in the list and can be switched on.** Its row is present and its tick box can be ticked, which agrees with what the Python side reported.

`addon_utils.check` matched the picture: `(True, True)` for the three that switched on and `(False, False)` for the one that failed.

**Not covered by this test:** nothing was run on Windows, because no Windows machine has run it, so the Windows-only console instruction and the mixed-library check on the troubleshooting page stay untested. What the extensions error status holds is still unknown; `status_clear_errors` returned `{'FINISHED'}` and printed nothing.


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every Python and command-line check named in this answer was run in a Blender started with no window and returned rather than raising, including `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()` and `bpy.ops.preferences.addon_expand()`, which the sources leave unstated. **The parts that are panels in the Preferences editor have since been reached on the Creator's Mac, which has a screen** — see the Mac observed section above. A declared warning is shown on the row and inside it; an add-on that failed to switch on is not marked at all. What the sources say follows. Most of it holds. The console, the logging flags, `--python-exit-code`, `extension list`, `repo-list`, `bpy.context.preferences.addons`, `addon_utils.paths()`, importing a bundled module, and the registration exceptions are all text output or Python, and the manual presents the command line as the normal place to see them ("For Python development, to see the output of the `print()` function" — `https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html`, read 2026-07-30). Three items differ: the add-on information box, the Preferences section of that box, and the Refresh Local button are parts of the Preferences editor. The operator behind Refresh Local, `bpy.ops.extensions.repo_refresh_all()`, is callable from Python; whether it behaves the same with no window open is not stated. Whether the add-on's own preferences can be read and set without the interface is not stated either, beyond `Addon.preferences` being a readable property.

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. Mostly yes, with these differences named by the sources. On Windows the console is hidden by default and must be brought back:

> When Blender is started on a Microsoft Windows operating system, the Console Window
> is displayed first ... The main Blender window will also appear and the Console Window will then be toggled off.
> To display the console again, go to Window ‣ Toggle System Console.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html`, read 2026-07-30. There is also a Windows-only start-up flag, "`--start-console` ... Start with the console window open (ignored if `-b` is set), (Windows only)" (`https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, read 2026-07-30). And the "Mixed Python Libraries (DLLs)" section of the Python troubleshooting page is headed "Platform Specific — Windows" and applies there only (`https://docs.blender.org/manual/en/5.0/troubleshooting/python.html`, read 2026-07-30). The `.pyd` / `.so` distinction in that same page is itself an operating-system split.

---

## 14. Does an add-on set up for one Blender work in another Blender on the same computer, and what decides that?

**A note on reading.** "Another Blender on the same computer" covers two cases — a Blender of a *different version*, and a *second copy of the same version*. The sources answer them differently, so I answer both. I take the different-version case as the one meant.

**LABEL: documented**, for everything quoted below. Every step where I go beyond the quoted words is marked **inferred** and written out. There are four of them: that an add-on does not appear by itself in another version; that two copies of the same version share one user directory; that an add-on calling into a compiled module is tied to the Blender that supplies it; and what the requirements page means for macOS.

### Two Blenders of different versions

**They can sit side by side.**

> Blender downloads can be extracted to any directory on the system, as
> a self contained installation. Multiple Blender versions can
> co-exist on the same system, and deployment can be automated using
> standard file management tools.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html`, read 2026-07-30.

**Each version keeps its own user directory, and installed add-ons live in it.**

> User directories store preferences, startup file, installed extensions,
> presets and more. By default these use the standard configuration folders
> for each operating system.
>
> Linux
> `$HOME/.config/blender/5.0/`
> ...
> macOS
> `/Users/$USER/Library/Application Support/Blender/5.0/`
>
> Windows
> `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.0\`

> `./extensions`
> Extension repositories.
> Located in both user and system directories. Repositories are loaded from
> both directories.
>
> `./scripts`
> Add-ons, presets, templates, user interface, startup scripts.
> Located in both user and system directories. Scripts are loaded from
> both directories.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html`, read 2026-07-30.

**The manual states outright that this is why a new version needs the old one's add-ons brought across.**

> Import Preferences From Previous Version
>
> Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and
> startup files from the previous version of Blender and load them. **This will include installed add-ons and extensions.**
>
> The preferences need to be imported from previous versions because the configuration files of each Blender version are
> stored in separate folders.

(Emphasis mine; the words are the manual's.) And immediately after:

> Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this
> option may lead to errors on startup. If this occurs, the recommended first step is to try Loading Factory
> Settings.

— Manual, `https://docs.blender.org/manual/en/5.0/getting_started/configuration/defaults.html`, read 2026-07-30.

**LABEL for this next sentence: inferred.** *So: an add-on installed into one version does not, by itself, appear in another version — it has to be copied across, or the two have to be pointed at a shared directory.* The step I took: the manual says installed extensions live in the user directory, that each version's directory is a separate folder, and that an explicit import is what carries installed add-ons and extensions from the old folder to the new one. From those three, the add-on's absence from the second version until something moves it follows. The manual never writes that sentence.

The import also has an operator:

> `bpy.ops.preferences.copy_prev()`
> Copy settings from previous version

— Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.preferences.html`, read 2026-07-30.

**The per-version split is visible in the API too.**

> `bpy.utils.resource_path(type, *, major=bpy.app.version[0], minor=bpy.app.version[1])`
> Return the base path for storing system files.
> `type` (str) – string in ['USER', 'LOCAL', 'SYSTEM'].
> `major` (int) – major version, defaults to current.
> `minor` (str) – minor version, defaults to current.

— Python API documentation, `https://docs.blender.org/api/5.0/bpy.utils.html`, read 2026-07-30. The path is keyed on major and minor, and nothing finer.

**The split can be deliberately undone.** Three environment variables and a folder do it:

> The `BLENDER_USER_RESOURCES` environment variable
> can be set to a custom directory to replace the default user directory.

> `BLENDER_SYSTEM_SCRIPTS` and `BLENDER_SYSTEM_EXTENSIONS`
> environment variables
> can be used to bundle additional scripts and extensions,
> that are not part of the regular Blender installation.

> Portable Installation
>
> When running Blender from a portable drive, it's possible to keep the configuration
> files on the same drive to take with you.
>
> To enable this, create a folder named `portable` at the following locations:
>
> Windows: Next to the Blender executable, in the unzipped folder
> Linux: Next to the Blender executable, in the unzipped folder
> macOS: Inside the application bundle at `Blender.app/Contents/Resources`
>
> This folder will then store preferences, startup file, installed extensions
> and presets.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html`, read 2026-07-30.

The deployment page shows a shared system repository being used this way:

> When working offline or in a more controlled environment, it may be useful
> to provide a set of extensions to all users. For this there is a default
> read-only System repository. This repository could be located on a
> read-only network drive or in a system directory.
>
> The `$BLENDER_SYSTEM_EXTENSIONS`
> environment variable
> controls the default location.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html`, read 2026-07-30.

### What decides whether it then works

**(i) A version range the add-on declares.** For an extension, in its manifest:

> Required values:
> `blender_version_min`:
> Minimum supported Blender version - use at least 4.2.0.

> Optional values:
> `blender_version_max`:
> Blender version that the extension does not support, earlier versions are supported.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html`, read 2026-07-30.

For a legacy add-on, in `bl_info`:

> `bl_info`
> is a dictionary containing add-on metadata such as the title,
> version and author to be displayed in the Preferences add-on list.
> It also specifies the minimum Blender version required to run the script;
> older versions won't display the add-on in the list.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/scripting/addon_tutorial.html`, read 2026-07-30. Cycles' own file carries exactly such an entry — see the observed finding below.

**(ii) Blender tells a remote repository which version it is, and is served accordingly.**

> When Blender fetches the extensions listing it passes the following arguments to make sure only
> compatible extensions are listed:
>
> `platform`
>
> `blender_version`
>
> These arguments are passed as parameters to the server via a query URL:
> ...
> `https://extensions.blender.org/api/v1/extensions/?blender_version=4.2.0&platform=linux-x64`

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/dynamic_repository.html`, read 2026-07-30. The same two facts travel on a download link: "`blender_version_min`", "`blender_version_max`", "`platforms`" — `https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/static_repository.html`, read 2026-07-30.

**(iii) The Python version, for anything compiled.**

> Blender and the `bpy` module are only compatible
> with a single Python version. This makes it possible for add-ons and VFX software
> in general to only have to target a single Python version.
>
> Blender bundles a complete Python installation and does not interact with the
> system Python by default.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html`, read 2026-07-30.

> It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system).

— Manual, `https://docs.blender.org/manual/en/5.0/troubleshooting/python.html`, read 2026-07-30.

The API documentation states the same principle for the interpreter generally:

> Therefor you can't use Python 3.6 with Blender built to use Python 3.7.

— Python API documentation, `https://docs.blender.org/api/5.0/info_tips_and_tricks.html`, read 2026-07-30.

**(iv) Blender's own changes between versions.** The manual's advice to add-on authors:

> New Blender versions can sometimes introduce breaking changes in Blender's
> Python API or even change how entire features work. If this affects your
> extension, you may want to maintain two "tracks" of your extension concurrently:
> one for Blender "old" and one for Blender "new".
> ...
> Make sure to correctly indicate the Blender versions that each version of
> your extension is compatible with in their manifest file.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/version_number_guidelines.html`, read 2026-07-30.

The 5.0 release notes name concrete breaks of this kind. These are the release notes' own statements about what changed, quoted as such:

> Properties defined by the `bpy.props` API are no longer stored in the same container as user-defined Custom Properties. As a consequence, it is no more possible to access them directly through the Python 'dict-like' syntax.

> ### Bundled Modules Now Private
> Blender comes with various bundled Python modules,
> while these were not documented as part of the public API, scripts may have imported them.
> The following modules have been made private and should not be used by scripts.

> - The format of logging output has changed,
>   including background render progress.
>   Add-ons or render farms parsing this output may need to be updated.

— Release notes, `https://developer.blender.org/docs/release_notes/5.0/python_api/`, read 2026-07-30.

> - "Copy Global Transform" is no longer an add-on, but is fully integrated into Blender itself.

> - Since "Copy Global Transform" core add-on was integrated into Blender, any scripts accessing the add-on (trying to enable it, or check if its enabled) will break.

— Release notes, `https://developer.blender.org/docs/release_notes/5.0/animation_rigging/`, read 2026-07-30.

> Theme settings have changed significantly in Blender 5.0 to make creating custom themes easier. Numerous theme settings have been unified, and in total 300+ settings were removed. Because of that, custom themes from 4.5 LTS are expected to break in the new version.

— Release notes, `https://developer.blender.org/docs/release_notes/5.0/user_interface/`, read 2026-07-30.

> Python FBX import add-on is marked as legacy.

— Release notes, `https://developer.blender.org/docs/release_notes/5.0/pipeline_io/`, read 2026-07-30.

**(v) The operating system and processor the add-on was built for.** The manifest carries this:

> `platforms`:
> List of supported platforms. If omitted, the extension will be available in all operating systems.
> The available options are
> ["windows-x64", "windows-arm64", "macos-x64", "macos-arm64", "linux-x64"]

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html`, read 2026-07-30. This bears on question 14 only where the two Blenders on the computer are builds for different platforms; the sources do not say how often that happens or whether it is possible. I list it because it is one of the two things Blender sends to a repository (see (ii) above).

**(vi) The namespace an extension's preferences live under.** This decides whether preferences carry across, not whether the code runs:

> The legacy add-ons would use their module name to access the preferences. This could lead to a name clash when
> extensions with the same name (from different repositories) would be installed.
> To prevent this conflict, the repository name is now part of the namespace.
>
> For example, now instead of `kitsu` the module name would be `bl_ext.{repository_module_name}.kitsu` instead.

> The value of `__package__` will vary between systems so it should never be replaced with a literal string.

— Manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html`, read 2026-07-30.

### Two copies of the *same* version

**LABEL: inferred.** Two installations of the same 5.0 on one computer share one user directory, so an add-on installed by one is present for the other. The step: the manual gives the user directory as `.../Blender/5.0/` and `bpy.utils.resource_path` keys the path on major and minor version only — nothing in either identifies *which* installation is asking. The exception the manual names is the `portable` folder, which is placed next to a particular executable and "will then store preferences, startup file, installed extensions and presets" for that copy alone; `BLENDER_USER_RESOURCES` does the same by environment. The manual never states the shared case in words, which is why this is inferred and not documented.

### Observed, from Cycles' own files

Cycles ships as an add-on, and its files show two things this question turns on.

**It declares a minimum Blender version in the legacy style.**

```
bl_info = {
    "name": "Cycles Render Engine",
    "author": "",
    "blender": (2, 80, 0),
    "description": "Cycles renderer integration",
    "warning": "",
    "doc_url": "https://docs.blender.org/manual/en/latest/render/cycles/",
    "tracker_url": "",
    "support": 'OFFICIAL',
    "category": "Render"}
```

— `cycles_5.0/__init__.py`, lines 7–16, read 2026-07-30. **LABEL: observed.**

**Its Python code calls into a compiled module that is not part of the Python code.** Its registration runs `engine.init()`:

```
def init():
    import bpy
    import _cycles
    import os.path

    path = os.path.dirname(__file__)
    user_path = os.path.dirname(os.path.abspath(bpy.utils.user_resource('CONFIG', path='')))

    _cycles.init(path, user_path, bpy.app.background)
```

— `cycles_5.0/engine.py`, lines 44–52, read 2026-07-30. `import _cycles` appears twenty-odd times across `engine.py`, `operators.py`, `osl.py` and `properties.py`. **LABEL: observed** — that is what the files say.

**LABEL: inferred**, for the step beyond it: an add-on written this way is tied to the Blender build that provides `_cycles`, and moving its Python files to a Blender that does not provide that module would fail at `import`. The step: the file imports a module by name that is not among the Cycles files supplied, so it must come from elsewhere in the installation. I did not run this, and the sources do not describe the failure.

Also observed, on the preferences namespace point (vi) above: `cycles_5.0/properties.py` line 1570 reads `bl_idname = __package__`, and line 1583 reads `context.preferences.addons[__package__].preferences.get_devices_for_type('OPTIX')` — the add-on looks its own preferences up by package name rather than a literal.

### Observed — what four Blenders on one computer actually do

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — on 2026-08-07, with Blender 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS side by side at `/opt/blender/4.5/blender` and so on. Every Blender was pointed at a settings folder or a stand-in home folder made for the test beforehand; the machine's own settings folder was never touched. The add-ons were written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 1 to 11 and 18.

**Which Python each one carries**, from `print(sys.version)` in a Blender with no window:

```
4.5.12 LTS  ->  3.11.11 (main, Apr 25 2025, 12:39:20) [GCC 11.2.1 20220127 (Red Hat 11.2.1-9)]
5.0.1       ->  3.11.13 (main, Apr 25 2025, 12:39:20) [GCC 11.2.1 20220127 (Red Hat 11.2.1-9)]
5.1.2       ->  3.13.9  (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-7)]
5.2.0 LTS   ->  3.13.13 (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-11)]
```

Blender 5.0 carries Python 3.11.13 in the build on this machine. So the four split two and two.

**The folder each one reads carries two numbers and not three.** With `HOME` pointed at a folder of the test's own and nothing else set, Blender 5.0 printed:

```
SMOOZY_USER <home>/.config/blender/5.0
SMOOZY_LOCAL /opt/blender/5.0/5.0
SMOOZY_EXT <home>/.config/blender/5.0/extensions
SMOOZY_SCRIPTS <home>/.config/blender/5.0/scripts
SMOOZY_CONFIG <home>/.config/blender/5.0/config
```

while `bpy.app.version` read the full three numbers. The patch number never enters the path. Starting Blender did not create the folder; only an install or a saved preference did.

**Nothing is shared by default.** Four differently-named extensions were installed, one by each Blender, into one shared home folder. Each Blender listed only its own — `5.0` listed `smoozy_v50 [installed]` and nothing else — and four separate version-named folders existed on disk, each with its own `extensions/user_default/`.

**Sharing a folder does work, in two different degrees.** With `BLENDER_USER_EXTENSIONS` pointed at one folder for all four but each keeping its own settings folder, an extension installed by 4.5 was listed by all four and could be switched on by all four — but only the Blender whose settings folder recorded it had it switched on at start-up. With `BLENDER_USER_RESOURCES` pointed at one folder for all four, all four listed it *and* all four had it switched on at start-up. The same held with 5.2 installing and the older three reading.

**A shared system-extensions folder serves all four.** With the extension's own folder placed at `<folder>/system/smoozy_ok` and `BLENDER_SYSTEM_EXTENSIONS` set to `<folder>`, every one of the four listed it under `Repository: "System" (id=system)` and switched it on as `bl_ext.system.smoozy_ok`.

**A shared scripts folder serves all four, for a legacy add-on.** With `BLENDER_SYSTEM_SCRIPTS` set to a folder holding `addons/smoozy_legacy_ok.py`, all four printed `SMOOZY_MODULES ['smoozy_legacy_ok']` and switched it on.

**Two separate copies of one build share, and a `portable` folder takes over.** `/opt/blender/5.0` was copied to a second folder. An extension installed with the original was listed by the copy, and both resolved `bpy.utils.resource_path('USER')` to the same `<home>/.config/blender/5.0`. A `portable` folder made next to the copy's program changed that at once — the copy then resolved to `<copy>/portable` and listed nothing. The same for the other three versions.

**A folder copied by hand from one version to another works.** The installed folder of an extension was copied unchanged out of `blender/4.5/extensions/user_default/` into the matching place under 5.0, 5.1 and 5.2; each then listed it and switched it on with no complaint. The same worked out of 5.2 into 4.5, 5.0 and 5.1.

**A declared version range is acted on, at install, and the message names the versions.** An extension declaring `blender_version_min = "99.0.0"` and one declaring `blender_version_max = "4.3.0"` were both built and validated without complaint, then offered to Blender 5.0.1:

```
ERROR smoozy_min99: This Blender version (5.0.1) doesn't meet the minimum supported version (99.0.0)
ERROR smoozy_max40: This Blender version (5.0.1) must be less than the maximum version (4.3.0)
```

with the same shape from the other three, each naming its own version. Nothing was installed, the listing stayed bare, and `addon_utils.enable` returned `None`. **The install command exits zero even so.** Adding `-e` produces a second line whose wording differs between versions: 4.5 and 5.0 print `Package should have been installed but not found: smoozy_min99`, and 5.1 and 5.2 print `Package was expected to be installed but not found: smoozy_min99`.

**A legacy add-on's declared minimum is not acted on.** A one-file legacy add-on with `"blender": (99, 0, 0)` sat in a shared scripts folder beside a plain one. All four Blenders listed both — `SMOOZY_MODULES ['smoozy_legacy_min99', 'smoozy_legacy_ok']` — and all four switched the 99 one on, ran its `register()`, and put it in `bpy.context.preferences.addons`.

**A bundled Python package built for a different Python is refused, and the message names both.** Three copies of one extension were made, each carrying a hand-built wheel tagged `cp39`, `cp311` and `cp313`. All twelve builds succeeded. The installs did not:

```
4.5 and 5.0 (Python 3.11): cp39  -> Error: Extension bl_ext.user_default.smoozy_wheel39 is incompatible (This Python version (3.11) isn't compatible with (3.9)), exit 1
4.5 and 5.0 (Python 3.11): cp311 -> installed, exit 0
4.5 and 5.0 (Python 3.11): cp313 -> Error: ... (This Python version (3.11) isn't compatible with (3.13)), exit 1
5.1 and 5.2 (Python 3.13): cp311 -> Error: ... (This Python version (3.13) isn't compatible with (3.11)), exit 1
5.1 and 5.2 (Python 3.13): cp313 -> installed, exit 0
```

Where the install succeeded the bundled package could be imported and printed its place — `<settings folder>/extensions/.local/lib/python3.11/site-packages/smoozypkg/__init__.py` for 4.5 and 5.0, `python3.13` for 5.1 and 5.2. **One reading here is spoiled and is recorded rather than hidden:** in the 4.5 and 5.0 runs the `cp313` step reported the refusal above but the import still succeeded, because the earlier `cp311` install had left the package in that Python's site-packages folder and removing the extension had not cleared it. The refusal message and its exit status are clean; that one import result is not.

**`bpy.ops.preferences.copy_prev()` works with no window, for the three that have a predecessor on the machine.** From a home folder holding a set-up 4.5 and nothing else, 5.0 started with `--background` printed `Info: Reload Start-Up file to restore settings` and `SMOOZY_COPYPREV {'FINISHED'}`, after which `blender/5.0/` existed with a `userpref.blend` and an `extensions/user_default/`, and 5.0's own listing carried the extension 4.5 had installed. 5.1 then picked up from 5.0 and 5.2 from 5.1, each returning `{'FINISHED'}`. 4.5, having no earlier version on the machine, refused: `RuntimeError: Operator bpy.ops.preferences.copy_prev.poll() failed, context is incorrect`.

**Not covered by this test:** none of it was run on macOS or Windows, so the paths and the `portable` folder placement are confirmed for Linux only. Nothing here says what a *remote* repository does with a declared range; the machine had no internet access allowed. Two builds of one major-and-minor version could not be tried, because the project has one build of each.

**Not covered.** This answer does not settle what Blender actually *does* when it meets an add-on outside its declared version range — whether it refuses to list it, lists it greyed out, lets it be enabled and fails, or ignores the field. The manual states only that for legacy `bl_info`, "older versions won't display the add-on in the list"; for `blender_version_max` it defines the field and says nothing about the behaviour. It does not settle whether two Blenders of different versions can safely share one directory via `BLENDER_USER_RESOURCES` — the variable is documented, the consequence is not. It does not settle which Python version 5.0 bundles; I searched the 5.0 manual, API documentation and release notes for that and found only the general rule, plus one incidental mention of Python 3.14 in the corrective-releases notes, which does not state 5.0's bundled version. And it does not settle whether an add-on that loads in the second Blender produces the same render there.

### Observed — the same questions asked again of four Blenders on a Mac

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 1 to 11, 18 and 27. All four Blenders — 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS — sit side by side on this Mac and all four were used.

**The Mac agreed with Linux on everything except one thing, and that one thing is set out at the end.**

*Which Python each carries.* The four differ, and they split two and two:

```
4.5 -> 3.11.11    5.0 -> 3.11.13    5.1 -> 3.13.9    5.2 -> 3.13.13
```

Blender 5.0.1 carries Python 3.11.13. These are the same four Pythons the Linux builds carry.

*Where each Blender's own folder sits.* With nothing overridden, Blender 5.0.1 computes:

```
SMOOZY_USER /Users/artist/Library/Application Support/Blender/5.0
SMOOZY_LOCAL /Applications/Blender 5.0.1.app/Contents/Resources/5.0
SMOOZY_SYSTEM /Library/Application Support/Blender/5.0
```

The folder is named for the major and minor number only; the patch number never enters it, so 5.0.1 reads a folder called `5.0`. This is the macOS layout the sources describe, confirmed. *(Changed by the citation check: this had read "so 4.5.12 reads a folder called `4.5`", which names the wrong version — the lines printed above it are 5.0.1's.)*

**One thing works differently on macOS from Linux, and it changed how every test below had to be run.** On the Linux machine a stand-in home folder redirects Blender's user folder. On macOS it does not: with `HOME` pointed at an empty folder made for the test, all four Blenders still computed `/Users/artist/Library/Application Support/Blender/<version>` and read the Creator's own settings. `BLENDER_USER_RESOURCES` does redirect it on macOS, and every test in this sitting used that instead. Where a test needed several versions to share one parent folder — the way Blender's own layout puts `4.5`, `5.0`, `5.1` and `5.2` side by side — that layout was laid out by hand and each version pointed at its own folder inside it.

*Nothing is shared between versions by default.* Each of the four installed a differently-named package into its own folder, and then each was asked what it held. Each listed only its own: Blender 5.0.1 showed `smoozy_v50 [installed]` and nothing else, and `SMOOZY_ADDONS ['bl_ext.user_default.smoozy_v50']`.

*Handing several versions one folder does make an add-on cross.* Four arrangements were run, installing with the oldest and reading with the other three, then installing with the newest and reading with the other three:

- One extensions folder handed to all four with `BLENDER_USER_EXTENSIONS`: every version listed it, and every version loaded it — `SMOOZY_ENABLE <module 'bl_ext.user_default.smoozy_ok' from '.../shared_ext/user_default/smoozy_ok/__init__.py'>`. Only the version that did the installing had it switched on in its own collection.
- One settings folder handed to all four with `BLENDER_USER_RESOURCES`: every version listed it, loaded it, **and** had it switched on — `SMOOZY_ADDONS ['bl_ext.user_default.smoozy_ok']` in all four.
- One system-extensions folder handed to all four with `BLENDER_SYSTEM_EXTENSIONS`: every version listed it under System and loaded it.
- One scripts folder handed to all four with `BLENDER_SYSTEM_SCRIPTS`, holding a legacy add-on: every version saw it among its modules and switched it on, printing its register line.

*A folder copied by hand also works.* An extension installed by 4.5 was copied, unchanged, into 5.0's, 5.1's and 5.2's folders; then the same the other way, out of 5.2 into the older three. Every destination version listed it and loaded it, in both directions.

*Two copies of one Blender.* A second copy of Blender 5.0.1 was made in a folder of its own. With nothing overridden, the copy computed the same user folder as the original — `/Users/artist/Library/Application Support/Blender/5.0` — so two installations of one version share by default. A `portable` folder made inside the copy's bundle at `Contents/Resources/portable`, which is where the sources say it goes on macOS, took over completely:

```
SMOOZY_USER .../copy_5.0.app/Contents/Resources/portable
SMOOZY_EXT .../copy_5.0.app/Contents/Resources/portable/extensions
SMOOZY_CONFIG .../copy_5.0.app/Contents/Resources/portable/config
```

and the copy's listing then held nothing. `BLENDER_USER_RESOURCES` still wins over a `portable` folder: with both in play, the copy read the folder the variable named.

*A declared version range is acted on.* An extension declaring it needs at least Blender 99 was refused at install:

```
ERROR smoozy_min99: This Blender version (5.0.1) doesn't meet the minimum supported version (99.0.0)
```

and one declaring it does not support 4.3 and above was refused too:

```
ERROR smoozy_max40: This Blender version (5.0.1) must be less than the maximum version (4.3.0)
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

*A bundled Python package built for another Python.* Three packages were built by hand, each carrying a wheel tagged for a different Python, and each was installed into a settings folder of its very own so that nothing could be left behind from a previous case. Blender 5.0.1 accepted only the one tagged `cp311` — `STATUS Installed`, then the bundled module imported from `.../extensions/.local/lib/python3.11/site-packages/smoozypkg/__init__.py`. The other two were refused:

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

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — on 2026-08-07, with Blender 5.0.1 at `/opt/blender/5.0/blender` and the other three beside it; all four were run. Every Blender was pointed at a settings folder or a stand-in home folder made for the test beforehand, inside a working folder of this test's own at `/root/again-add-ons/`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entry 29.

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


**Outside the interface.** **Settled by test on Linux, and on the Mac for everything but one item — see the two observed sections above.** All of it holds with no window, including the one part the sources leave interface-bound: `bpy.ops.preferences.copy_prev()` ran under `--background`, returned `{'FINISHED'}`, and carried the older version's installed extension into the newer version's folder. **On the Creator's Mac this does not hold for one item:** `bpy.ops.preferences.copy_prev()` refuses to run there in all four versions, with a window open as well as without one — see the Mac observed section above. What the sources say follows. Nearly all of it holds, because nearly all of it is files, paths and environment. Directory layout, the `portable` folder, `BLENDER_USER_RESOURCES`, `BLENDER_SYSTEM_SCRIPTS`, `BLENDER_SYSTEM_EXTENSIONS`, the manifest fields, and the `--addons` start-up flag ("Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons" — `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, read 2026-07-30) are all outside the window. So is enabling by script, which the manual gives as a deployment recipe: "`import addon_utils` / `addon_utils.enable("my-addon")`" placed at `$BLENDER_SYSTEM_SCRIPTS/startup/enable_addons.py` (`https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html`, read 2026-07-30). One item is interface-bound: "Import Preferences From Previous Version" is presented on the first-run dialog — "When you start Blender for the first time or update to a new version, the interactive region of the Splash Screen is replaced with a few initial preferences" (`https://docs.blender.org/manual/en/5.0/getting_started/configuration/defaults.html`, read 2026-07-30). Its operator `bpy.ops.preferences.copy_prev()` exists, but the sources do not say it works with no window open. **Brought up to date 2026-08-07.** The one interface-bound item has now been pinned down further on Linux. `bpy.ops.preferences.copy_prev()` is reachable from a script there **only when the version folders sit in Blender's own layout**; when the settings folder is handed over with `BLENDER_USER_RESOURCES` — the way nearly every test in this round hands it over — it refuses in all four versions, `poll()` returning `False`. So on Linux the folder arrangement decides whether this route can be driven from outside the interface at all. See the observed section on the folder arrangements above.

**Operating system.** **Partly settled by test — see the observed section above.** The per-version folder, the shared-folder routes and the `portable` folder have now been confirmed on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with the macOS paths coming out as the sources describe them, including the `portable` folder inside `Blender.app/Contents/Resources`. **The two machines disagree on one point:** `bpy.ops.preferences.copy_prev()` runs on Linux and refuses to run on macOS. Two further macOS-only points: a stand-in `HOME` does not redirect Blender's user folder there, though `BLENDER_USER_RESOURCES` does. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The mechanism holds everywhere; the paths do not. The user directory, the system directory, the cache directory and the `portable` folder each have a different location per operating system, all three given verbatim in the directory-layout page quoted above — on macOS the `portable` folder goes *inside* the application bundle rather than beside the executable. Compiled parts of an add-on are per-operating-system by nature ("these are not portable between different platforms"; `.pyd` on Windows, `.so` elsewhere — `https://docs.blender.org/manual/en/5.0/troubleshooting/python.html`, read 2026-07-30), and the manifest's `platforms` list splits them five ways. One further constraint, from the requirements page, which speaks for 5.0 only where it names it: **Brought up to date 2026-08-07.** The two Linux cells were run deliberately and the four cells now stand as a table in the observed section above: on Linux it runs under Blender's own layout and refuses when the settings folder is handed over; on macOS it refuses under both arrangements. The two machines still disagree, and the arrangement does not explain the disagreement away — only the Linux "runs" needed the arrangement named. The macOS cells were not run again in this sitting and are quoted from the Mac record. Windows is unchanged and was still not tried.

> Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later.

— `https://www.blender.org/download/requirements/`, read 2026-08-01. **LABEL: inferred**, for the step that follows: on macOS the pairing of "another Blender on the same computer" with 5.0 is limited to Apple Silicon machines running macOS 13 or later. The requirements page states the requirement; it says nothing about add-ons.

---

## Things a person at a computer with Blender could settle by hand

The items on this list have since been run on **both** machines — a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores) and the Creator's own Mac (a MacBook Pro with an M1 Pro chip running macOS 26.5.2) — in all four Blender versions, and their results are written into the answers above as observed findings. They are kept here so those results can be traced back to what was asked. **Nothing on this list is still open.**

The two things those lists did not cover both needed a screen, which that machine does not have. Both have since been run on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — in all four versions, and their results are written into the answers above as observed findings. **Nothing on this list is still open.** The only thing left untested anywhere is Windows, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

For each, what would count as a negative result is written out, so the test can come out either way.

**A. Does an extension whose `blender_version_max` excludes 5.0 install, and does it enable?** (Question 14.)
Build a trivial extension: a folder with `__init__.py` containing empty `register()` and `unregister()` functions that each `print()` a distinct word, and a `blender_manifest.toml` copied from the manual's example with `blender_version_min = "4.2.0"` and `blender_version_max = "4.3.0"`. Run `blender --command extension build` in that folder, then install the resulting zip with `blender --command extension install-file`. Read every line the terminal prints. Then start Blender, open Preferences ‣ Add-ons, and search for the add-on's name.
*Before running it,* ask each sub-command for its own help — `blender --command extension build --help` and `blender --command extension install-file --help` — and use the option names it gives. This document establishes that the sub-commands `list`, `repo-list`, `install`, `install-file` and `validate` exist, and that `--no-prefs` exists; it does not establish `build`, nor the spelling of any option that picks a repository. Writing a guessed option into the command means a failure that cannot be told apart from Blender refusing the add-on, which is the very thing this test is measuring.
*Positive:* it is refused, or listed but not enablable, or a message names the version range.
*Negative:* it installs, enables, and prints its word with no mention of the version range — i.e. `blender_version_max` is not enforced at install or enable time on this machine.

**B. Do two Blender versions on one computer see each other's installed add-ons?** (Question 14.)
Install the same trivial extension from (A), with `blender_version_max` removed, into Blender 5.0 only. Then start a different Blender version already on the machine and run `blender --command extension list` for each in turn, and compare the two lists.
*Positive:* the add-on appears only in the 5.0 list.
*Negative:* it appears in both lists, which would mean the two versions share an extensions directory on this machine despite the per-version layout.

**C. Do two Blenders of the same version share it?** (Question 14.)
Unzip a second copy of the same 5.0 build to a different folder. Run `blender --command extension list` from each copy's executable and compare.
*Positive:* both list the add-on installed by the first.
*Negative:* only the first lists it.
Then repeat the comparison with a `portable` folder created for the second copy — on Windows and Linux next to that copy's executable, on macOS inside the application bundle at `Blender.app/Contents/Resources`, as the directory-layout page quoted above has it.
*Positive, second half:* the second copy no longer lists the add-on the first installed.
*Negative, second half:* it still lists it, meaning the `portable` folder did not take over from the shared user directory here.

**D. Does installing an add-on from a second Blender disturb a render already running?** (Question 12.)
Start a long render headless: `blender --background scene.blend --render-anim`, on a scene set to enough frames that it runs for several minutes, writing numbered image files. While it is running, in a second terminal install the zip from (A) with `blender --command extension install-file` (same warning about option names as in (A)). Watch both terminals to the end. Then check that every frame was written, and compare the finished frames against a run of the same scene with no install during it.
**Compare the pictures, not the files.** This instruction is the audit's, not the sources': the project's own guidance for hand tests is that comparing two of Blender's saved images byte for byte does not reliably show whether the two renders match. So compare pixel against pixel, and do not treat a byte-for-byte difference as a result.
*Positive:* the render errors, stalls, exits early, drops a frame, or produces a frame whose pixels differ from the undisturbed run; or the install fails or hangs.
*Negative:* the render finishes normally, every frame's pixels match the undisturbed run, and the install reports success — which would show the two do not collide, at least for this kind of install, in this direction, on this machine.

**E. The same, with the install going the other way and repeated.** (Question 12.)
As (D), but run the install command five or six times in a row during one render, and once more in the last few seconds before the render finishes. Also try `--no-prefs` on some of the runs and not on others. Keep every terminal's output. Afterwards, start Blender once more with no window and read what it prints on start-up.
*Positive:* any one of the runs prints something the others did not, or reports failure where another reported success; or a frame is missing; or a frame's pixels differ from the undisturbed run of (D); or the next start-up prints an error or warning.
*Negative:* all the runs report success and print the same thing, every frame is present with matching pixels, and the next start-up prints nothing it would not print anyway.

**F. Does installing into the *same* Blender that is rendering even work?** (Question 12, reading (a).) This one needs eyes on Blender's window, so it cannot be run on a computer with no screen.
Open Blender with a window, set the scene to a render that will take a few minutes and that writes its frames to disk, and set Preferences ‣ Interface ‣ Temporary Editors ‣ Render In to "Keep User Interface". Render the same scene once with nothing else done to it, and keep those frames. Then start the render again, and while it runs open Preferences ‣ Get Extensions ‣ Install from Disk and install the zip from (A). Write down whether the menu item can be clicked at all, whether the file browser opens, what appears on screen, and what the system console prints. When the render ends, compare its frames' pixels against the undisturbed run.
*Positive:* the control is greyed out or does nothing; or an error appears; or the render stops, drops a frame, or produces a frame whose pixels differ from the undisturbed run.
*Negative:* the install goes through and reports success, the render reaches its last frame, and every frame's pixels match the undisturbed run.

**G. What does the console actually say when an add-on fails to enable?** (Question 13.)
Take the trivial extension from (A) and break it deliberately in three separate copies: one with `def execute(self, context, spam)` on an operator, one with `bl_idname = 1`, one that does `import some_module_that_does_not_exist` at the top. Install and enable each in turn, from a terminal, and record the exact output. Then run `blender --command extension list` and check `bpy.context.preferences.addons.keys()` from `--python-expr` for each.
*Positive:* the console carries a distinguishable message for each, and the failed add-on is absent from `preferences.addons`.
*Negative:* the failure is silent, or a broken add-on still appears in `preferences.addons` — either of which would mean the checks in question 13 do not catch it.

**H. Does `Refresh Local` report anything for a broken add-on?** (Question 13.)
With one of the broken copies from (G) installed, run `bpy.ops.extensions.repo_refresh_all()` — first from Blender's window via Preferences ‣ Add-ons ‣ Refresh Local, then from `blender --background --python-expr "import bpy; bpy.ops.extensions.repo_refresh_all()"`.
*Positive:* a warning naming the add-on appears in both cases.
*Negative:* nothing is reported in either; or a warning appears but does not name the add-on, so a reader could not tell which one is broken; or it is reported in one of the two and not the other, which would show the check does not carry across the window.

**I. Does `--python-exit-code` make a failed enable visible to a script?** (Question 13.)
Run `blender --background --python-exit-code 1 --python-expr "import addon_utils; addon_utils.enable('<broken add-on module name>')"` and read the process's exit status.
*Positive:* a non-zero exit status.
*Negative:* zero — meaning a failed enable is not by itself an exception, and a caller would have to inspect `preferences.addons` instead.

**J. Does the repository lock actually hold another Blender off?** (Question 12.) Added by the audit, because the lock is the only thing in the sources that sits between two Blenders and nothing says what it does.
In one terminal, hold the lock: `blender --background --python-expr "import bpy, time; bpy.ops.extensions.repo_lock_all(); time.sleep(120)"`. While that is running, in a second terminal install the zip from (A) with `blender --command extension install-file` (same warning about option names as in (A)). Record exactly what the second terminal prints and how long it takes to finish. Then repeat the whole thing with nothing holding the lock, and compare the two.
*Positive:* with the lock held, the second command waits, or refuses, or says something about a lock, and behaves differently from the unlocked run.
*Negative:* the second command finishes the same way and in the same time whether or not the lock is held — meaning the lock does not stand between two Blenders installing, and it cannot be what protects a render either.

---

2026-08-06 — Opus 5 (Claude Code) — Add-ons, Blender 5.0

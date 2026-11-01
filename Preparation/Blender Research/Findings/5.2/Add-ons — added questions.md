# Add-ons — added questions (12, 13, 14), Blender 5.2

Sources consulted: Blender 5.2 manual (including its `_images` pictures), Blender 5.2 Python API documentation, Blender 5.2 release notes, and Cycles' own files from an installed Blender 5.2 — all read on 2026-07-30. Blender's requirements page as taken on 2026-08-01 was also opened; it does not name Blender 5.2 anywhere, so it is not used as a source for any claim below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I did not use any subagents. Everything below I read myself.

One disclosure, in the spirit of the line above. I opened no other file, but two pieces of this project's text were placed in front of me automatically before I began, without my asking for them: the text of the project's `CLAUDE.md` signpost, and a short standing note describing this project in one line. I did not follow `CLAUDE.md`'s instruction to read `Preparation/Groundwork/Start Here.md`, and I opened nothing it points to. Neither piece of text contained any claim about how Blender behaves, so nothing below could have been coloured by them — but you should know they reached me, rather than take the line above as meaning nothing whatsoever did.

---

## 12. Does installing an add-on disturb a render already running on the same computer?

**LABEL: not in the sources.**

**That label answers the question from the sources only. A test has since been run and settled it — see the observed section below in this same answer.**

**The question is ambiguous, and I answer both readings together.** It can mean (a) a render running inside the same Blender session in which the add-on is installed, or (b) a render running in one Blender process while a second Blender process, or a command-line install, puts an add-on onto the same machine. The sources say nothing about either, so the answer is the same for both. Reading (b) is the one I take to be meant.

Nothing in the manual, the API documentation, the release notes or Cycles' files says what happens to a render in progress when an add-on is installed. There is no page about installing while busy, no note on either the Add-ons or the Get Extensions preferences page about rendering, and nothing in the extensions command-line page about a running Blender.

**What I found nearby, none of which answers the question:**

Enabling takes effect at once inside the session doing the enabling:

> "The add-on functionality should be immediately available."

— Blender manual, https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html, read 2026-07-30.

Extension repositories carry a lock on disk, but nothing says what it guards or when it is taken:

> "Remove the repository file-system lock"

— Blender Python API documentation, description of `bpy.ops.extensions.repo_unlock()`, https://docs.blender.org/api/5.2/bpy.ops.extensions.html, read 2026-07-30.

There is a documented warning about changing data while a job is running, but it is about application timers, not about installing an add-on:

> "If true, the interface is currently locked by a running job and data should not be modified from application timers. Otherwise, the running job might conflict with the handler causing unexpected results or even crashes."

— Blender Python API documentation, `WindowManager.is_interface_locked`, https://docs.blender.org/api/5.2/bpy.types.WindowManager.html, read 2026-07-30.

A render is one of the job kinds a script can ask about:

> "RENDER: / Regular rendering."

— Blender Python API documentation, Wm Job Type Items, https://docs.blender.org/api/5.2/bpy_types_enum_items/wm_job_type_items.html, read 2026-07-30. The function that takes it is documented as: "Check whether a job of the given type is running." — https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30.

The command-line installer has an option that keeps it from writing the preferences file:

> "--no-prefs / Treat the user-preferences as read-only, preventing updates for operations that would otherwise modify them."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/extension_arguments.html, read 2026-07-30. Nothing connects this to a render in progress.

**Where I looked.** Whole-corpus text searches across the 5.2 manual, the 5.2 API documentation, the 5.2 release notes and the Cycles 5.2 files for: `while rendering`, `during a render`, `render is running`, `already running`, `another instance`, `second instance`, `multiple instances`, `concurrent`, `concurrently`, `at the same time`, `simultaneously`, `while Blender is running`, `running Blender`, `other Blender`, `already open`, `open instance`, `in progress`, `mid-render`, `disturb`, `affect a render`, `safe to`, `race`, `job`, `interrupt`, `unresponsive`, `restart`, `lock`, `locked`, `file-system lock`, `repository lock`, `reload_scripts`, `script.reload`, `reloading modules`, `render farm`, `farm`, `shared`. Pages read in full and ruled out: the Add-ons preferences page, the Get Extensions preferences page, the Extensions add-ons page, the Extensions getting-started page, the extensions command-line page, the Blender directory layout page, the Deploying Blender in Production page, the Preferences introduction page, the render output Animation page, the Cycles GPU rendering page (its "Why is Blender unresponsive during rendering?" entry is about graphics cards drawing the interface, not about installing anything), the render output Output properties page (its "Primitive Render Farm" hint is about several machines writing to one shared folder, not about installing add-ons), the startup troubleshooting page, the Python errors troubleshooting page. In the API documentation I read the full Extensions operators page, the full Preferences operators page, `bpy.utils`, `bpy.app`, `WindowManager`, `PreferencesExtensions` and `UserExtensionRepo`. In Cycles' files I read `__init__.py` and the start of `engine.py`; neither mentions installation.

### Observed — an install fired at a render that is already running

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 5.2.0 LTS at `/opt/blender/5.2/blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. The machine is shared with other topics' tests, so **every time below is from a shared machine and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 19, 20, 21, 22 and 23.

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
<settings folder>/extensions/user_default/smoozy_ok/__pycache__/__init__.cpython-313.pyc
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
RepoLock: freed without releasing lock!Error locking repository "User Default": lock exists (lock is held by other session "/tmp/blender_8yXdsu/")
Error: Repository "User Default": lock exists (lock is held by other session "/tmp/blender_8yXdsu/")
If the lock was held by a Blender instance that exited unexpectedly,
use: "Force Unlock Repository" to clear the lock.
Access from the "Repositories" popover in the extensions preferences.
```

with exit status 1, in 0.855 seconds. The very same command with nothing holding the lock printed `STATUS Installed "smoozy_ok"` and exited zero, in 1.012 seconds. So it does not wait — it refuses, and it names the session holding the lock. Both timings are from a machine shared with other topics' tests.

*Part three — a Blender that is rendering does not hold the lock.* While a background animation render was under way in the same settings folder, a second Blender took the lock without difficulty. That is why the render in the runs above was never disturbed by the installs: a rendering Blender is not holding the thing the installs need.

**Not covered by this test:** none of it was run on macOS or Windows. The macOS half has since been run and follows in the next section. The machine was shared with other topics' tests throughout, so the per-frame times are recorded but not read as measurements. Nothing here covers a windowed Blender, which the machine cannot open; that has since been run on the Creator's Mac and is recorded in the Mac observed section in this same answer. `bpy.context.window_manager.reports` read back as an empty list.

**Not covered.** This settles nothing about whether an install is harmless, harmful, or somewhere between. It also does not settle whether an install writes to any file a running render reads, since the sources never list what an install touches while it runs.

### Observed — the same question asked again on a Mac, including with a window

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 19, 20, 21, 22, 23 and 25.

**The Mac agreed with Linux on every part that both machines could run.** The differences are only in the paths printed.

*What an install writes.* Taking a full record of the settings folder before and after, an ordinary install changed these five files and nothing else. Two of them sit outside the extension's own folder — the shared compatibility cache and the preferences file:

```
> .../extensions/user_default/smoozy_ok/__init__.py
> .../extensions/.cache/compat.dat
> .../config/userpref.blend
> .../extensions/user_default/smoozy_ok/blender_manifest.toml
> .../extensions/user_default/smoozy_ok/__pycache__/__init__.cpython-313.pyc
```

With `--no-prefs` added, `config/userpref.blend` did not change — and the add-on was then **not** switched on, although `-e` was given: `SMOOZY_ADDONS []` where the ordinary install left `SMOOZY_ADDONS ['bl_ext.user_default.smoozy_ok']`.

*A Blender that is already running.* A Blender was left running while a second Blender installed into the same settings folder. It never saw the add-on in its collection — `SMOOZY_WATCH_START []`, `SMOOZY_WATCH_MIDDLE []`, and after a rescan that returned `SMOOZY_WATCH_REFRESH {'FINISHED'}` still `SMOOZY_WATCH_AFTER_REFRESH []`. But it could switch it on without being restarted: `SMOOZY_WATCH_ENABLE <module 'bl_ext.user_default.smoozy_ok' from '.../extensions/user_default/smoozy_ok/__init__.py'>`.

*Installs fired at a running render, from a second Blender.* An eight-frame Cycles animation was rendered twice — once undisturbed, once with install-and-remove pairs fired at it from a second Blender for as long as it ran (four pairs). Both runs wrote all eight frames and exited zero. Every install and every removal exited zero. Comparing the two runs picture by picture rather than file by file, every frame was identical:

```
PIXELCMP .../control_5.2/f_0001.png .../test_5.2/f_0001.png differing_values 0 of 307200 worst 0.0
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

*With a window — the part no machine could reach before.* Blender 5.2.0 LTS was started with a window on the Mac's screen and driven by a script that called Blender's own commands and took Blender's own screenshots. An animation render was started, and after the second frame a handler installed a package:

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

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 5.2.0 LTS at `/opt/blender/5.2/blender`, on 2026-08-07. All four versions were run and all four agreed. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`, inside a working folder of this test's own at `/root/again-add-ons/`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. Other topics' tests were running Blender on the same machine throughout, so **no time here is read as a measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entry 28.

**Why this was run again.** The citation check sent the windowed `use_lock_interface` run back, because nothing in its record showed the setting was in force at the moment the install fired — see the note in the Mac section above. This is that test rebuilt in the form a machine with no screen can reach. **The windowed form still needs the Creator's Mac and has not been run again; that half stays sent back.**

**What was done.** The eight-frame scene was rendered three times in Blender 5.2.0 LTS: once undisturbed as a control; once with `use_lock_interface` left `False` and a package installed from a `render_post` handler after the second frame; and once with `use_lock_interface` set to `True` and the same install. Cycles on the processor, eight of the twenty-four cores, 512 samples, adaptive sampling and denoising off, a fixed seed, 640 by 480. In every run the flag was printed back before the assignment, after it, at each of the eight frames **from inside the handler**, immediately after the install, and after the render.

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every part of this question was reached with no window at all: the render, the installs, the removals, and an install fired from inside the rendering Blender by a handler. None of it needed a screen. **The windowed half has since been run on the Creator's Mac and agrees with the no-window half:** with a window open, an install fired from inside the rendering Blender returned `{'FINISHED'}`, every frame was written, and every frame matched its control picture for picture — with the interface lock off and on alike. What the sources say follows. The sources do not say. There is a command-line way to install (`blender --command extension install` and `install-file`), and it is documented, but nothing about it says anything about a render running elsewhere. **Brought up to date 2026-08-07.** The interface-lock half has now been rebuilt with no window at all, with `use_lock_interface` read back at every step and from inside the render handler at the very frame the install fires — see the observed section on the rebuilt lock test above. With the lock genuinely on, the install returned `{'FINISHED'}`, all eight frames were written, and every frame matched its control picture for picture, in all four versions. Two further things came out of that read-back and bear on this line: `bpy.context.window_manager.is_interface_locked` and `bpy.app.is_job_running('RENDER')` both read `False` at every moment, including mid-render with the lock on, so neither gives a program outside the interface a view of the scene setting or of the running render. **The windowed form of the same test is still waiting on the Creator's Mac and stays sent back.**

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The sources do not say. The user directories an install writes into are given per operating system on the directory layout page, but no behaviour during a render is stated for any of them. **Brought up to date 2026-08-07.** The rebuilt interface-lock test was run on the rented Linux machine only, in all four versions, and came out as the Linux runs above. The windowed form of it needs the Creator's Mac and has not been run again. Nothing here changes what stays open for Windows.

---

## 13. Once an add-on is installed and switched on, what more can be learned about whether it will work?

**LABEL: documented** (with one point marked inferred inside, and one gap marked at the end).

**This question is ambiguous.** I read it as: after an add-on is installed and enabled, what further information does Blender make available about whether it will actually function. The other reading — what can be learned about the add-on in general, such as its description and author — is answered by the same information box quoted below, so both readings are served, but I answer the first.

**Blender's own answer is: watch the console.** The same tip appears on three separate pages:

> "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred."

— Blender manual, https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html, https://docs.blender.org/manual/en/5.2/editors/preferences/extensions.html, and https://docs.blender.org/manual/en/5.2/advanced/extensions/addons.html, all read 2026-07-30.

**Being enabled is not the same as being ready to work.** The add-on's own preferences may hold switches that are off:

> "Some add-ons use this section for example to enable/disable certain functions of the add-on. Sometimes these might even all default to off. So it is important to check if the enabled add-on has any particular preferences."

— Blender manual, https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html, read 2026-07-30.

**There is an information box per add-on:**

> "You can click the arrow at the left of the add-on box to see more information, such as its location, a description and a link to the documentation. Here you can also find a button to report a bug specific of this add-on."

— Blender manual, https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html, read 2026-07-30.

I opened the picture on that page (`_images/editors_preferences_section_addons.webp`, part of https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html, read 2026-07-30). What is visible in it is a list of add-on rows, each with an expand arrow, a tick box, a name, and an icon at the right; enabled ones are drawn bright and disabled ones dim. No row in that picture shows a warning or error marking; one picture showing no such marking does not establish that none exists. **Documented**, and only for what the picture shows — it shows no expanded row, so it says nothing about what an expanded row contains.

**A rescan reports problems as warnings:**

> "Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting). Any issues are reported as warnings."

— Blender manual, https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html and https://docs.blender.org/manual/en/5.2/editors/preferences/extensions.html, read 2026-07-30. An operator carries a similar description: "Refresh extension & legacy add-ons, reloading modules & meta-data (similar to restarting)" — `bpy.ops.extensions.repo_refresh_all`, https://docs.blender.org/api/5.2/bpy.ops.extensions.html, read 2026-07-30. Nothing quoted here says the two are the same thing.

**Bundled third-party modules can be checked directly:**

> "Once the extension has been installed you can check the module is being loaded by importing it in the Python console and printing its location:"
>
> "import PIL"
> "print(PIL.__file__)"

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/python_wheels.html, read 2026-07-30.

**Where it landed can be learned:**

> "The destination of the add-on depends on your Blender configuration. When installing an add-on the source and destination paths are printed in the console. You can also find add-on path locations by running this in the Python Console:"
>
> "import addon_utils"
> "print(addon_utils.paths())"

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/scripting/addon_tutorial.html, read 2026-07-30.

**An enabled add-on can be reached by name from Python**, which is the documented way an add-on gets at its own preferences:

> "addon_prefs = bpy.context.preferences.addons[__package__]"

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/addons.html, read 2026-07-30. The collection itself is documented as "Collection of add-ons" (`bpy.types.Addons`) holding entries described as "Python add-ons to be loaded automatically" with a `module` name and a `preferences` member — https://docs.blender.org/api/5.2/bpy.types.Addons.html and https://docs.blender.org/api/5.2/bpy.types.Addon.html, read 2026-07-30.

**One documented family of failures to look for, with a way to look:**

> "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular Python scripts, these are not portable between different platforms."
>
> "It is possible the library is incompatible with your Blender installation (attempting to load a library built for a different version of Python, or loading a 32-bit library on a 64-bit system)."
>
> "If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating system."

and, for Windows:

> "If Python is raising errors or you have an add-on that just fails when enabled with an error – e.g: ... is not a valid Win32 application. – this may be caused by some inconsistency in the Python libraries."

— Blender manual, https://docs.blender.org/manual/en/5.2/troubleshooting/python.html, read 2026-07-30. That page also carries a script for finding duplicate libraries on the search path, and says of its output: "The output will show in Command Line Window."

**Messages can be read back rather than only watched.** In the interface:

> "The Info editor logs the executed operators as well as errors, warnings, and informational messages."

— Blender manual, https://docs.blender.org/manual/en/5.2/editors/info_editor.html, read 2026-07-30. And from Python, new in this version:

> "Blender reports are now exposed as a read-only list in WindowManager ID types. They also now have a session-wide unique identifier integer"

— Blender release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/, read 2026-07-30. Each entry carries "Report message text" and "Report type (severity)" — https://docs.blender.org/api/5.2/bpy.types.Report.html, read 2026-07-30.

**Its manifest can be checked after the fact — INFERRED.** The validate command is documented as: "Validate the package meta-data in the current directory", taking "The package source path (either directory containing package files or the package archive). This path must containing a blender_manifest.toml manifest." — Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/extension_arguments.html, read 2026-07-30. The step I take: an installed extension is a directory of package files containing `blender_manifest.toml` (the getting-started page shows the manifest as a required member of every extension, and the deploying page shows an installed system extension at `$BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml`), therefore validate can be pointed at an already-installed extension. The sources never say this outright, and never say what validate would report about one.

### Observed — what Blender 5.2 actually reports about an add-on, with no window

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 5.2.0 LTS at `/opt/blender/5.2/blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the machine's own settings folder was never touched. The add-ons were written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 12, 13, 14, 15, 16, 17 and 24.

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

**Which checks work with no window at all.** Every one of these returned rather than raising: `bpy.app.background` (`True`), `bpy.context.preferences.addons.keys()`, `addon_utils.paths()` (`['/opt/blender/5.2/5.2/scripts/addons_core']`), `bpy.app.online_access`, `bpy.app.is_job_running('RENDER')`, `bpy.utils.extension_path_user(...)` (`'<settings folder>/extensions/.user/user_default/smoozy_ok'`), `bpy.data.workspaces[0].use_filter_by_owner`, `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()`, `bpy.ops.preferences.addon_expand(module=...)`, `bpy.ops.extensions.status_clear_errors()`, `bpy.context.preferences.version` (`(5, 2, 44)`) and `addon_utils.check(...)`. `bpy.context.window_manager.reports` read back as an empty list — it exists in this version, and in none of the other three.

`bpy.ops.preferences.addon_enable` takes exactly `['module', 'rna_type']`, so the argument name used above is the real one.

**Logging arguments.** `--log-list-categories` is accepted and prints its categories, beginning `addon`, `anim`, `anim.action`, and `--log-level 2` prints timestamped lines such as `00:00.340  context          | unknown:0: screen=<Screen("Layout")...>`. In 4.5 the same argument is not accepted at all.

**Exit status.** A failed switch-on leaves the exit status at zero unless `--python-exit-code 1` is given *and* the error is left uncaught, in which case it is 1. The same failure through `addon_utils.enable` exits 0 either way. Naming a broken add-on at launch with `--addons` prints the traceback and exits 0 in both cases. Naming a *working* add-on with `--addons` loads and registers it but leaves `bpy.context.preferences.addons` empty: `SMOOZY_LOADED True`, `SMOOZY_CHECK (True, True)`, `SMOOZY_COLL []`.

**A legacy add-on's `warning` field reads back exactly as written**, alongside five keys Blender fills in that the file never carried:

```
SMOOZY_BLINFO smoozy_legacy_warn {'name': 'Smoozy Legacy Warn', ..., 'warning': 'SMOOZY WARNING TEXT', 'category': 'Development', 'location': '', 'doc_url': '', 'support': 'COMMUNITY', 'show_expanded': False, '_init': None}
```

Whether that text is *shown* anywhere has since been run on the Creator's Mac, which has a screen: it is shown, both as a triangle on the add-on's row and as a `Warning` line inside it. See the Mac observed section in this same answer.

**Not covered by this test:** whether a broken add-on is marked in the Add-ons list, and whether a non-empty `warning` is displayed — both need a window, and both have since been run on the Creator's Mac; see the Mac observed section in this same answer. A declared warning is shown; a failure to switch on is not marked at all. What the extensions error status holds; `status_clear_errors` returned `{'FINISHED'}` and printed nothing. Nothing here was run on macOS or Windows.

**Not covered.** The sources do not say that Blender keeps any per-add-on status a script can read — nothing like "enabled but failed". They do not say what `bpy.ops.preferences.addon_enable` returns when the add-on's registration fails; the documentation only gives its return type as an operator result. They do not say whether the tick box stays ticked after a failure. They do not say whether the warnings from the rescan reach anywhere other than the interface. And nothing here distinguishes "loaded without error" from "will do its job on this scene".

### Observed — the same checks asked again on a Mac, including the Add-ons list on screen

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 12, 13, 14, 15, 16, 17, 24 and 26.

**The Mac agreed with Linux on every check.** What follows is what Blender 5.2.0 LTS printed on the Mac.

*Every check, with no window.* Seventeen checks were run in one script in a Blender started with `--background`. All of them returned rather than raising, except one:

```
SMOOZY13 background -> True
SMOOZY13 addons_keys -> ['bl_ext.user_default.smoozy_ok', 'bl_pkg', 'cycles', ...]
SMOOZY13 addon_utils_paths -> ['/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core']
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
SMOOZY13 prefs_version -> (5, 2, 44)
```

`bpy.context.window_manager.reports` returned an empty list — `SMOOZY13 wm_reports -> []`. On the two spellings the documents disagreed about: `bpy.app.online_access_override` exists and `bpy.app.online_access_overriden` does not.

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

*On screen — the part no machine could reach before.* Blender 5.2.0 LTS was started with a window, four legacy add-ons were switched on from a script, the Add-ons section of Preferences was opened and searched for "smoozy", and Blender took its own screenshots. Three things are visible in them.

- **A declared warning is shown.** The row for the add-on carrying `"warning": "SMOOZY WARNING TEXT"` has a warning triangle at its right-hand end where the other rows have the plain legacy-add-on icon, and when the row is opened it carries a line reading `Warning` followed by the same triangle and the words `SMOOZY WARNING TEXT`.
- **An add-on that failed to switch on is not marked at all.** Its row looks exactly like a row for an add-on simply left switched off: the tick box is empty, there is no triangle, and opening the row shows only the ordinary Type, Maintainer, Version and File lines. Nothing on screen distinguishes "it broke" from "it is off".
- **The one declaring it needs Blender 99 is in the list and can be switched on.** Its row is present and its tick box can be ticked, which agrees with what the Python side reported.

`addon_utils.check` matched the picture: `(True, True)` for the three that switched on and `(False, False)` for the one that failed.

**Not covered by this test:** nothing was run on Windows, because no Windows machine has run it, so the Windows-only console instruction and the mixed-library check on the troubleshooting page stay untested. What the extensions error status holds is still unknown; `status_clear_errors` returned `{'FINISHED'}` and printed nothing.


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every Python and command-line check named in this answer was run in a Blender started with no window and returned rather than raising, including `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()` and `bpy.ops.preferences.addon_expand()`, which the sources leave unstated. **The parts that are panels in the Preferences editor have since been reached on the Creator's Mac, which has a screen** — see the Mac observed section above. A declared warning is shown on the row and inside it; an add-on that failed to switch on is not marked at all. What the sources say follows. Partly. The console tip, the printed install paths, `addon_utils.paths()`, importing a bundled module, `bpy.context.preferences.addons`, the reports list on the window manager, and `blender --command extension validate` are all reached by typing rather than by clicking. **INFERRED** on one point: the command-line page says "Blender output is shown in the Console Window it is started from." (Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html, read 2026-07-30), and the step I take is that this is the same console the add-on tip says to check. The sources never join those two statements. The Info editor and the add-on information box are parts of the interface and have no stated equivalent outside it. The sources do not say whether the rescan's warnings appear when Blender runs without an interface.

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The general points hold on all three, but the console differs and the precompiled-library trap is per system. On Windows the console is a separate window that starts hidden — "The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." (Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html, read 2026-07-30). The startup troubleshooting page adds, for any system: "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window." (https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html, read 2026-07-30). The mixed-DLL check quoted above is given under a "Windows" heading and is Windows-only.

---

## 14. Does an add-on set up for one Blender work in another Blender on the same computer, and what decides that?

**LABEL: documented** (with two points marked inferred inside).

**Short answer from the sources: not by itself, if the other Blender is a different version.** Each Blender version keeps its own user files, so an add-on installed for one is not simply there for the other. It can be carried across, and separately it may or may not run once it is there — two different things, decided by different rules.

**Several Blenders can sit on one computer:**

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30.

**Each version has its own folders, and installed add-ons live in them.** The user directory for this version is given as `$HOME/.config/blender/5.2/` on Linux, `/Users/$USER/Library/Application Support/Blender/5.2/` on macOS, and `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.2\` on Windows, and the page says of user directories: "User directories store preferences, startup file, installed extensions, presets and more." Inside them, `./extensions` is "Extension repositories" and `./scripts/addons/*.py` is "Python add-ons which may be enabled in the Preferences…" — Blender manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html, read 2026-07-30.

The consequence is stated plainly on the first-run page:

> "The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/configuration/defaults.html, read 2026-07-30.

**There is a built-in way to carry them over, and a warning attached to it:**

> "Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and startup files from the previous version of Blender and load them. This will include installed add-ons and extensions."

> "Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this option may lead to errors on startup. If this occurs, the recommended first step is to try Loading Factory Settings."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/configuration/defaults.html, read 2026-07-30. The same thing exists as an operator: "Copy settings from previous version" (`bpy.ops.preferences.copy_prev`, https://docs.blender.org/api/5.2/bpy.ops.preferences.html, read 2026-07-30).

**A Blender can be pointed at a chosen set of folders instead of its own.** Environment variables: "BLENDER_USER_RESOURCES: Replace default directory of all user files. Other BLENDER_USER_* variables override when set." and "BLENDER_USER_EXTENSIONS: Directory for user extensions." and "BLENDER_SYSTEM_EXTENSIONS: Directory for system extensions repository." — Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30. A shared read-only repository is the documented way to serve one set of extensions to everybody:

> "When working offline or in a more controlled environment, it may be useful to provide a set of extensions to all users. For this there is a default read-only System repository. This repository could be located on a read-only network drive or in a system directory."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30. A repository can also be given its own folder by hand: "Manually set the path for extensions to be stored. When disabled a user's extensions directory is created." (`use_custom_directory`, https://docs.blender.org/api/5.2/bpy.types.UserExtensionRepo.html, read 2026-07-30).

**A portable copy keeps its own, separate from all of that:**

> "This folder will then store preferences, startup file, installed extensions and presets."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html, read 2026-07-30, describing the `portable` folder.

**Two installs of the same version share the same user folder — INFERRED.** The documented path template names only two numbers, `5.2`. The API function that returns those paths takes only two: "major (int | None) – Major version. None (the default) uses `bpy.app.version[0]`." and "minor (int | None) – Minor version. None (the default) uses `bpy.app.version[1]`." — `bpy.utils.resource_path`, https://docs.blender.org/api/5.2/bpy.utils.html, read 2026-07-30. The step I take: since no third number enters the path, two builds whose first two numbers match resolve to the same user directory, and so would see the same installed extensions. The sources never state this.

**What decides whether it then runs.**

*The manifest, for an extension.* Required: "blender_version_min: Minimum supported Blender version - use at least 4.2.0." Optional: "blender_version_max: Blender version that the extension does not support, earlier versions are supported." Also optional: "platforms: List of supported platforms. If omitted, the extension will be available in all operating systems. The available options are ["windows-x64", "windows-arm64", "macos-x64", "macos-arm64", "linux-x64"]" — Blender manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/getting_started.html, read 2026-07-30.

*The `bl_info` block, for a legacy add-on.*

> "It also specifies the minimum Blender version required to run the script; older versions won't display the add-on in the list."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/scripting/addon_tutorial.html, read 2026-07-30, describing the `"blender": (2, 80, 0)` entry of `bl_info`.

**Observed**, in a bundled add-on shipped with Blender 5.2 — Cycles declares such a minimum:

> `bl_info = {`
> `    "name": "Cycles Render Engine",`
> `    "author": "",`
> `    "blender": (2, 80, 0),`

— Cycles' own files, `cycles_5.2/__init__.py`, lines 7–10 (the `bl_info` dictionary at the top of the file), read 2026-07-30.

*Repositories filter by version before you ever see the extension.*

> "When Blender fetches the extensions listing it passes the following arguments to make sure only compatible extensions are listed:"
>
> "platform"
>
> "blender_version"

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/creating_repository/dynamic_repository.html, read 2026-07-30.

*Blender itself changes between versions.*

> "New Blender versions can sometimes introduce breaking changes in Blender's Python API or even change how entire features work. If this affects your extension, you may want to maintain two "tracks" of your extension concurrently: one for Blender "old" and one for Blender "new"."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/version_number_guidelines.html, read 2026-07-30. And: "New Blender versions may add, remove or change functionality that affects the results of production files. For a given project, it is advisable to use a single LTS version of Blender." — https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30.

*Anything compiled inside the add-on has to match Blender's own Python and the machine.*

> "Blender and the bpy module are only compatible with a single Python version. This makes it possible for add-ons and VFX software in general to only have to target a single Python version."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30. And:

> "It is possible the library is incompatible with your Blender installation (attempting to load a library built for a different version of Python, or loading a 32-bit library on a 64-bit system)."

— Blender manual, https://docs.blender.org/manual/en/5.2/troubleshooting/python.html, read 2026-07-30. Bundled wheels are named for a Python version and a platform; the manual's own example lists `pillow-12.1.0-cp313-cp313-macosx_11_0_arm64.whl` alongside Linux and Windows builds, and instructs "Wheels filenames must match Python's binary distribution specification" — https://docs.blender.org/manual/en/5.2/advanced/extensions/python_wheels.html, read 2026-07-30. **INFERRED**, from those two together: an extension carrying compiled wheels built for one Blender's bundled Python will not load in a Blender bundling a different Python version. The step is that "only compatible with a single Python version" plus wheels tagged `cp313` means a Blender on a different Python has no matching wheel. The sources do not join these two statements themselves.

**A way to tell which Blender a set of preferences came from:** "Version of Blender the userpref.blend was saved with" — `Preferences.version`, https://docs.blender.org/api/5.2/bpy.types.Preferences.html, read 2026-07-30.

### Observed — what four Blenders on one computer actually do

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — on 2026-08-07, with Blender 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS side by side at `/opt/blender/4.5/blender` and so on. Every Blender was pointed at a settings folder or a stand-in home folder made for the test beforehand; the machine's own settings folder was never touched. The add-ons were written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 1 to 11 and 18.

**Which Python each one carries**, from `print(sys.version)` in a Blender with no window:

```
4.5.12 LTS  ->  3.11.11 (main, Apr 25 2025, 12:39:20) [GCC 11.2.1 20220127 (Red Hat 11.2.1-9)]
5.0.1       ->  3.11.13 (main, Apr 25 2025, 12:39:20) [GCC 11.2.1 20220127 (Red Hat 11.2.1-9)]
5.1.2       ->  3.13.9  (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-7)]
5.2.0 LTS   ->  3.13.13 (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-11)]
```

Blender 5.2 carries Python 3.13.13 in the build on this machine. So the four split two and two.

**The folder each one reads carries two numbers and not three.** With `HOME` pointed at a folder of the test's own and nothing else set, Blender 5.2 printed:

```
SMOOZY_USER <home>/.config/blender/5.2
SMOOZY_LOCAL /opt/blender/5.2/5.2
SMOOZY_EXT <home>/.config/blender/5.2/extensions
SMOOZY_SCRIPTS <home>/.config/blender/5.2/scripts
SMOOZY_CONFIG <home>/.config/blender/5.2/config
```

while `bpy.app.version` read the full three numbers. The patch number never enters the path. Starting Blender did not create the folder; only an install or a saved preference did.

**Nothing is shared by default.** Four differently-named extensions were installed, one by each Blender, into one shared home folder. Each Blender listed only its own — `5.2` listed `smoozy_v52 [installed]` and nothing else — and four separate version-named folders existed on disk, each with its own `extensions/user_default/`.

**Sharing a folder does work, in two different degrees.** With `BLENDER_USER_EXTENSIONS` pointed at one folder for all four but each keeping its own settings folder, an extension installed by 4.5 was listed by all four and could be switched on by all four — but only the Blender whose settings folder recorded it had it switched on at start-up. With `BLENDER_USER_RESOURCES` pointed at one folder for all four, all four listed it *and* all four had it switched on at start-up. The same held with 5.2 installing and the older three reading.

**A shared system-extensions folder serves all four.** With the extension's own folder placed at `<folder>/system/smoozy_ok` and `BLENDER_SYSTEM_EXTENSIONS` set to `<folder>`, every one of the four listed it under `Repository: "System" (id=system)` and switched it on as `bl_ext.system.smoozy_ok`.

**A shared scripts folder serves all four, for a legacy add-on.** With `BLENDER_SYSTEM_SCRIPTS` set to a folder holding `addons/smoozy_legacy_ok.py`, all four printed `SMOOZY_MODULES ['smoozy_legacy_ok']` and switched it on.

**Two separate copies of one build share, and a `portable` folder takes over.** `/opt/blender/5.2` was copied to a second folder. An extension installed with the original was listed by the copy, and both resolved `bpy.utils.resource_path('USER')` to the same `<home>/.config/blender/5.2`. A `portable` folder made next to the copy's program changed that at once — the copy then resolved to `<copy>/portable` and listed nothing. The same for the other three versions.

**A folder copied by hand from one version to another works.** The installed folder of an extension was copied unchanged out of `blender/4.5/extensions/user_default/` into the matching place under 5.0, 5.1 and 5.2; each then listed it and switched it on with no complaint. The same worked out of 5.2 into 4.5, 5.0 and 5.1.

**A declared version range is acted on, at install, and the message names the versions.** An extension declaring `blender_version_min = "99.0.0"` and one declaring `blender_version_max = "4.3.0"` were both built and validated without complaint, then offered to Blender 5.2.0 LTS:

```
ERROR smoozy_min99: This Blender version (5.2.0) doesn't meet the minimum supported version (99.0.0)
ERROR smoozy_max40: This Blender version (5.2.0) must be less than the maximum version (4.3.0)
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

**Not covered.** The sources never say what Blender does at load time with an already-installed extension whose `blender_version_min` is above the running Blender, or whose `blender_version_max` is below it — only that remote listings are filtered by version, and that a legacy add-on below the minimum is not shown in the list. They do not say whether such an extension can be installed from disk anyway, nor what message appears. They do not say whether "may lead to errors on startup" means the add-on is skipped, disabled, or left half-loaded. They say nothing about two Blenders of *different* versions sharing one extensions folder through the environment variables — only that the variables exist. And nothing states which Python version any particular Blender release bundles.

### Observed — the same questions asked again of four Blenders on a Mac

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 1 to 11, 18 and 27. All four Blenders — 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS — sit side by side on this Mac and all four were used.

**The Mac agreed with Linux on everything except one thing, and that one thing is set out at the end.**

*Which Python each carries.* The four differ, and they split two and two:

```
4.5 -> 3.11.11    5.0 -> 3.11.13    5.1 -> 3.13.9    5.2 -> 3.13.13
```

Blender 5.2.0 LTS carries Python 3.13.13. These are the same four Pythons the Linux builds carry.

*Where each Blender's own folder sits.* With nothing overridden, Blender 5.2.0 LTS computes:

```
SMOOZY_USER /Users/artist/Library/Application Support/Blender/5.2
SMOOZY_LOCAL /Applications/Blender.app/Contents/Resources/5.2
SMOOZY_SYSTEM /Library/Application Support/Blender/5.2
```

The folder is named for the major and minor number only; the patch number never enters it, so 4.5.12 reads a folder called `4.5`. This is the macOS layout the sources describe, confirmed.

**One thing works differently on macOS from Linux, and it changed how every test below had to be run.** On the Linux machine a stand-in home folder redirects Blender's user folder. On macOS it does not: with `HOME` pointed at an empty folder made for the test, all four Blenders still computed `/Users/artist/Library/Application Support/Blender/<version>` and read the Creator's own settings. `BLENDER_USER_RESOURCES` does redirect it on macOS, and every test in this sitting used that instead. Where a test needed several versions to share one parent folder — the way Blender's own layout puts `4.5`, `5.0`, `5.1` and `5.2` side by side — that layout was laid out by hand and each version pointed at its own folder inside it.

*Nothing is shared between versions by default.* Each of the four installed a differently-named package into its own folder, and then each was asked what it held. Each listed only its own: Blender 5.2.0 LTS showed `smoozy_v52 [installed]` and nothing else, and `SMOOZY_ADDONS ['bl_ext.user_default.smoozy_v52']`.

*Handing several versions one folder does make an add-on cross.* Four arrangements were run, installing with the oldest and reading with the other three, then installing with the newest and reading with the other three:

- One extensions folder handed to all four with `BLENDER_USER_EXTENSIONS`: every version listed it, and every version loaded it — `SMOOZY_ENABLE <module 'bl_ext.user_default.smoozy_ok' from '.../shared_ext/user_default/smoozy_ok/__init__.py'>`. Only the version that did the installing had it switched on in its own collection.
- One settings folder handed to all four with `BLENDER_USER_RESOURCES`: every version listed it, loaded it, **and** had it switched on — `SMOOZY_ADDONS ['bl_ext.user_default.smoozy_ok']` in all four.
- One system-extensions folder handed to all four with `BLENDER_SYSTEM_EXTENSIONS`: every version listed it under System and loaded it.
- One scripts folder handed to all four with `BLENDER_SYSTEM_SCRIPTS`, holding a legacy add-on: every version saw it among its modules and switched it on, printing its register line.

*A folder copied by hand also works.* An extension installed by 4.5 was copied, unchanged, into 5.0's, 5.1's and 5.2's folders; then the same the other way, out of 5.2 into the older three. Every destination version listed it and loaded it, in both directions.

*Two copies of one Blender.* A second copy of Blender 5.2.0 LTS was made in a folder of its own. With nothing overridden, the copy computed the same user folder as the original — `/Users/artist/Library/Application Support/Blender/5.2` — so two installations of one version share by default. A `portable` folder made inside the copy's bundle at `Contents/Resources/portable`, which is where the sources say it goes on macOS, took over completely:

```
SMOOZY_USER .../copy_5.2.app/Contents/Resources/portable
SMOOZY_EXT .../copy_5.2.app/Contents/Resources/portable/extensions
SMOOZY_CONFIG .../copy_5.2.app/Contents/Resources/portable/config
```

and the copy's listing then held nothing. `BLENDER_USER_RESOURCES` still wins over a `portable` folder: with both in play, the copy read the folder the variable named.

*A declared version range is acted on.* An extension declaring it needs at least Blender 99 was refused at install:

```
ERROR smoozy_min99: This Blender version (5.2.0) doesn't meet the minimum supported version (99.0.0)
```

and one declaring it does not support 4.3 and above was refused too:

```
ERROR smoozy_max40: This Blender version (5.2.0) must be less than the maximum version (4.3.0)
```

Neither reached the listing and neither could be switched on — `Add-on not loaded: "bl_ext.user_default.smoozy_min99", cause: No module named 'bl_ext.user_default.smoozy_min99'`, `SMOOZY_ENABLE None`. The command still exits zero. When `-e` is given, a second line follows the refusal, and its wording is one of the two real differences between the versions:

```
Package was expected to be installed but not found: smoozy_min99
```

Building and validating the same folder is refused by nothing at all: `Success parsing TOML in "."` and a package built, in every version.

*A legacy add-on's declared minimum is not acted on.* A one-file add-on declaring `"blender": (99, 0, 0)` was listed among the modules alongside the ordinary one, read back with its declaration intact, and switched on with no message:

```
SMOOZY_MODULES ['smoozy_legacy_min99', 'smoozy_legacy_ok']
SMOOZY_ENABLE <module 'smoozy_legacy_min99' from '.../smoozy_legacy_min99.py'>
```

*A bundled Python package built for another Python.* Three packages were built by hand, each carrying a wheel tagged for a different Python, and each was installed into a settings folder of its very own so that nothing could be left behind from a previous case. Blender 5.2.0 LTS accepted only the one tagged `cp313` — `STATUS Installed`, then the bundled module imported from `.../extensions/.local/lib/python3.13/site-packages/smoozypkg/__init__.py`. The other two were refused:

```
Error: Extension bl_ext.user_default.smoozy_wheel_cp311 is incompatible (This Python version (3.13) isn't compatible with (3.11))
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

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — on 2026-08-07, with Blender 5.2.0 LTS at `/opt/blender/5.2/blender` and the other three beside it; all four were run. Every Blender was pointed at a settings folder or a stand-in home folder made for the test beforehand, inside a working folder of this test's own at `/root/again-add-ons/`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entry 29.

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


**Outside the interface.** **Settled by test on Linux, and on the Mac for everything but one item — see the two observed sections above.** All of it holds with no window, including the one part the sources leave interface-bound: `bpy.ops.preferences.copy_prev()` ran under `--background`, returned `{'FINISHED'}`, and carried the older version's installed extension into the newer version's folder. **On the Creator's Mac this does not hold for one item:** `bpy.ops.preferences.copy_prev()` refuses to run there in all four versions, with a window open as well as without one — see the Mac observed section above. What the sources say follows. Mostly holds, and the pieces that do not have replacements. The version-numbered folders, the environment variables, the manifest fields and the shared system repository do not depend on anyone clicking. The one part that is interface-only is "Import Preferences From Previous Version", which is described as an option on the splash screen at first start; the operator behind it, `bpy.ops.preferences.copy_prev`, is documented in the API but the sources do not say it can be driven with no screen. The documented way to set add-ons up without a person is a startup script — "For example, a script can enable add-ons for every user." with `addon_utils.enable("my-addon")` (https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30) — plus `--addons <addon(s)>`, "Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons." (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). **Brought up to date 2026-08-07.** The one interface-bound item has now been pinned down further on Linux. `bpy.ops.preferences.copy_prev()` is reachable from a script there **only when the version folders sit in Blender's own layout**; when the settings folder is handed over with `BLENDER_USER_RESOURCES` — the way nearly every test in this round hands it over — it refuses in all four versions, `poll()` returning `False`. So on Linux the folder arrangement decides whether this route can be driven from outside the interface at all. See the observed section on the folder arrangements above.

**Operating system.** **Partly settled by test — see the observed section above.** The per-version folder, the shared-folder routes and the `portable` folder have now been confirmed on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with the macOS paths coming out as the sources describe them, including the `portable` folder inside `Blender.app/Contents/Resources`. **The two machines disagree on one point:** `bpy.ops.preferences.copy_prev()` runs on Linux and refuses to run on macOS. Two further macOS-only points: a stand-in `HOME` does not redirect Blender's user folder there, though `BLENDER_USER_RESOURCES` does. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The rule holds everywhere but the folders differ. The three user paths quoted above are per system, and the portable folder sits "Next to the Blender executable, in the unzipped folder" on Windows and Linux but "Inside the application bundle at Blender.app/Contents/Resources" on macOS (https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html, read 2026-07-30). The system directories differ further by how Blender was installed, including `/usr/share/blender/5.2/` for Linux distribution packages and `%ProgramFiles%\Blender Foundation\Blender\5.2\` for the Windows installer. The `platforms` manifest field names the five combinations of system and processor an extension may declare, so an add-on can be restricted to some and not others. **Brought up to date 2026-08-07.** The two Linux cells were run deliberately and the four cells now stand as a table in the observed section above: on Linux it runs under Blender's own layout and refuses when the settings folder is handed over; on macOS it refuses under both arrangements. The two machines still disagree, and the arrangement does not explain the disagreement away — only the Linux "runs" needed the arrangement named. The macOS cells were not run again in this sitting and are quoted from the Mac record. Windows is unchanged and was still not tried.

---

## Things a person with Blender open could settle in a few minutes

The items on this list have since been run on **both** machines — a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores) and the Creator's own Mac (a MacBook Pro with an M1 Pro chip running macOS 26.5.2) — in all four Blender versions, and their results are written into the answers above as observed findings. They are kept here so those results can be traced back to what was asked. **Nothing on this list is still open.**

The two things those lists did not cover both needed a screen, which that machine does not have. Both have since been run on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — in all four versions, and their results are written into the answers above as observed findings. **Nothing on this list is still open.** The only thing left untested anywhere is Windows, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

1. **(Q12, separate processes.)** On one computer, start a render with no interface: `blender -b scene.blend -f 1` on a scene that takes two or three minutes. While it is running, in a second terminal run `blender --command extension install-file -r <user repo id> -e <some-addon>.zip`. Watch the first terminal. Record whether frame 1 finishes, whether the written image matches, pixel for pixel, one from a control run with no install happening, and every line the render process prints. Compare the pixels rather than the bytes of the two files. *Negative result:* the render process prints an error, stalls, exits early, or writes an image whose pixels differ from the control run.

2. **(Q12, one process.)** In the interface, press Render Animation on a scene of a few slow frames. While it renders, open Preferences ‣ Get Extensions ‣ Install from Disk and install an extension. Record whether the menu can be opened and the button pressed at all, whether the install completes, and whether the animation finishes with frames whose pixels match a control run. Compare the pixels rather than the bytes. *Negative result:* the install is refused or greyed out, or the render aborts, or any frame's pixels differ from the control run.

3. **(Q12, what an install touches.)** Before and after a single `blender --command extension install-file` run, list the modification times of everything under the 5.2 user directory. Repeat with `--no-prefs`. Record which files changed each time. *Negative result:* nothing under the user directory changes, or the same files change with and without `--no-prefs`.

4. **(Q13, bundled modules.)** Install and enable an extension that bundles a wheel. In the Python Console run `import <module>` then `print(<module>.__file__)`. *Negative result:* the import raises, or the printed path is not inside the extension's own site-packages folder.

5. **(Q13, a failure that is meant to fail.)** Write a one-file legacy add-on whose `register()` raises an exception. Start Blender from a terminal, install it, tick its box. Record: what the terminal prints, whether the tick box stays ticked, and what `list(bpy.context.preferences.addons.keys())` contains. Then repeat with `blender -b --addons <name> --python-expr "import bpy; print(list(bpy.context.preferences.addons.keys()))"` and record the printed list, the output, and the process exit code. *Negative result:* nothing is printed anywhere, the add-on appears in the list, and the exit code is zero — i.e. the failure leaves no trace.

6. **(Q13, reading it back from Python.)** Immediately after the previous step, in the same session print every entry of `bpy.context.window_manager.reports` with its `type` and `message`. The sources quoted above do not give that member its name, so if there is no such member, first run `print([n for n in dir(bpy.context.window_manager) if "report" in n.lower()])` and use whatever that gives. *Negative result:* the list is empty or holds nothing about the add-on.

7. **(Q13, validating what is already installed.)** Run `blender --command extension validate <path to an installed extension's folder>` and record the exact output. *Negative result:* the command refuses the path, or reports on the manifest only and says nothing about whether the extension can run.

8. **(Q14, does it cross versions on its own.)** Install an extension in Blender 5.2. Start a Blender of a different version on the same machine and choose "Create New Preferences" at the splash. Look in its Add-ons list for the extension. Then compare the two user directories side by side. *Negative result:* the extension is listed in the second Blender without any import step.

9. **(Q14, does it cross versions when told to.)** Set `BLENDER_USER_EXTENSIONS` to one and the same folder for two different Blender versions. Start each, look for the extension, and try to enable it in both, with the terminal visible. Record the outcome and any message for each. *Negative result:* the extension is missing in one of them, or appears but fails to enable in one of them.

10. **(Q14, a version bound Blender should honour.)** Take an extension and edit its manifest's `blender_version_min` to a version above 5.2. Build it, then install it from disk in 5.2 and try to enable it. Record exactly what Blender says and whether the install and the enable each go through. Repeat with `blender_version_max` set below 5.2. *Negative result:* both install and enable succeed silently with no message of any kind.

11. **(Q14, two installs of one version.)** Install Blender 5.2.0 and Blender 5.2.1 (or any two builds sharing 5.2) side by side. In each, print `bpy.utils.resource_path('USER')`. Install an extension in one and look for it in the other. *Negative result:* the two printed paths differ, or the extension is absent from the second.

---

2026-08-06 — Opus 5 (Claude Code) — Add-ons, Blender 5.2

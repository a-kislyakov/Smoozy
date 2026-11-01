# Add-ons — added questions (Blender 5.1)

Questions 12, 13 and 14 of the Add-ons question sheet. No other question on that sheet was touched.

**Sources consulted, and when.** Blender's manual for 5.1, Blender's Python API documentation for 5.1, the 5.1 release notes, and Cycles' own files as shipped with 5.1 — all read on **2026-07-30** (the date the copies were taken). Blender's requirements page, read as taken on **2026-08-01**. The requirements page never names version 5.1, so I have not used it to state anything about 5.1; where it comes up below it is only to say that it is silent.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

I started no helper agents. Everything below I read myself.

---

## 12. Does installing an add-on disturb a render already running on the same computer?

**Label: not in the sources.** That label is the answer to the question itself.

**That label answers the question from the sources only. A test has since been run and settled it — see the observed section below in this same answer.** The quoted material further down is documented, and is there only to show what the sources do say nearby; none of it answers the question.

**The question has two readings, and I could not settle either.** Reading A: a render is running inside a Blender, and an add-on is installed into that same Blender. Reading B: a render is running in one Blender, and an add-on is installed by a second, separate Blender on the same computer. I searched for both. The sources describe installing, enabling and removing add-ons, and they describe rendering, but nowhere do they put the two together. Nothing in the manual, the API documentation, the 5.1 release notes or Cycles' files states what happens to a render in progress when an add-on is installed.

**What I did find, and what it does and does not settle.**

The documentation is clear that enabling takes effect at once inside the Blender doing the enabling:

> "To enable or disable an add-on check or uncheck the box to the right of the add-ons.
>
> The add-on functionality should be immediately available."

Source: the manual, https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html — read 2026-07-30.

That sentence is about the interface of the Blender you are sitting in. It says nothing about a render running at that moment, and nothing about a second Blender.

The closest thing to a statement about two things touching the add-on store at once is that repositories can be locked on disk. Three operators exist for it:

> "bpy.ops.extensions.repo_lock_all()
>
> Lock repositories - to test locking"

> "bpy.ops.extensions.repo_unlock()
>
> Remove the repository file-system lock"

> "bpy.ops.extensions.repo_unlock_all()
>
> Unlock repositories - to test unlocking"

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.extensions.html — read 2026-07-30.

This shows that an extension repository can carry a lock on the file system, and that a lock can be removed. It does not say when Blender takes that lock, what a second Blender does when it meets one, or whether any of it touches a render. I am deliberately not building an answer on it.

I also note, without stretching it, that an installed extension's own storage is wiped on upgrade:

> "This directory will be kept between upgrades but will be removed if the extension is uninstalled."

and

> "Writing files into the add-on's directory also has the down side that upgrading the extension will remove all files."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html — read 2026-07-30.

That is about files being replaced when an extension is upgraded or uninstalled. It is not a statement about a render, and I am not turning it into one.

**Where I looked.**

Searched, across the whole of the 5.1 manual, the whole of the 5.1 Python API documentation, and all twenty-one 5.1 release-note files, for: `multiple instances`, `another instance`, `second instance`, `simultaneous`, `concurrent`, `concurrently`, `at the same time`, `already running`, `running Blender`, `while rendering`, `during a render`, `render in progress`, `install while`, `installing`, `in progress`, `interrupt`, `disturb`, `affect a render`, `does not affect`, `unaffected`, `same computer`, `same machine`, `other processes`, `background job`, `restart`, `restart Blender`, `reload scripts`, `lock`.

Pages opened and ruled out, with the reason:

- Add-ons preferences and Get Extensions preferences (`editors/preferences/addons.html`, `editors/preferences/extensions.html`) — describe installing, enabling, disabling, refreshing and removing. Say nothing about anything else running.
- Extensions Command Line Arguments (`advanced/command_line/extension_arguments.html`) — full list of the `install`, `install-file`, `remove`, `sync`, `update` subcommands and their options. No mention of other Blenders, other jobs or renders.
- Command Line Arguments (`advanced/command_line/arguments.html`) — read in full for background mode and for the `--addons` option. No statement about installing while something else runs.
- Blender's Directory Layout (`advanced/blender_directory_layout.html`) — says where extensions and preferences are written. Does not say what happens if something is reading them.
- Deploying Blender in Production (`advanced/deploying_blender.html`) — the page most likely to raise this, since it is about studios and automated deployment. It covers bundling extensions, offline working and startup scripts. It does not mention renders in progress.
- Troubleshooting: Crashes (`troubleshooting/crash.html`), Python Errors (`troubleshooting/python.html`), Network (`troubleshooting/network.html`) — no mention.
- Rendering an Animation (`render/output/animation.html`) — the one hit for `interrupt` is about a person choosing to stop a render to use the computer, not about an installation interfering.
- API: `bpy.ops.extensions.html`, `bpy.ops.preferences.html`, `bpy.types.Addon.html`, `bpy.types.Addons.html`, `bpy.types.AddonPreferences.html`, `bpy.types.Preferences.html`, `bpy.types.PreferencesExtensions.html`, `bpy.types.UserExtensionRepo.html`, `bpy.utils.html`, `bpy.app.html` — none of these say anything about a render in progress.
- Cycles' own files (`Blender Sources/cycles_5.1/`) — Cycles is the thing doing the rendering, but its files describe its own settings and its render engine class. Nothing there concerns add-on installation.
- The requirements page does not name 5.1 anywhere, so it cannot speak for 5.1, and in any case it is about hardware and operating systems, not this.

### Observed — an install fired at a render that is already running

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 5.1.2 at `/opt/blender/5.1/blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. The machine is shared with other topics' tests, so **every time below is from a shared machine and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 19, 20, 21, 22 and 23.

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
Error locking repository "User Default": lock exists (lock is held by other session "/tmp/blender_gHcXNk/")
Error: Repository "User Default": lock exists (lock is held by other session "/tmp/blender_gHcXNk/")
If the lock was held by a Blender instance that exited unexpectedly,
use: "Force Unlock Repository" to clear the lock.
Access from the "Repositories" popover in the extensions preferences.
RepoLock: freed without releasing lock!
```

with exit status 1, in 0.772 seconds. The very same command with nothing holding the lock printed `STATUS Installed "smoozy_ok"` and exited zero, in 0.969 seconds. So it does not wait — it refuses, and it names the session holding the lock. Both timings are from a machine shared with other topics' tests.

*Part three — a Blender that is rendering does not hold the lock.* While a background animation render was under way in the same settings folder, a second Blender took the lock without difficulty. That is why the render in the runs above was never disturbed by the installs: a rendering Blender is not holding the thing the installs need.

**Not covered by this test:** none of it was run on macOS or Windows. The macOS half has since been run and follows in the next section. The machine was shared with other topics' tests throughout, so the per-frame times are recorded but not read as measurements. Nothing here covers a windowed Blender, which the machine cannot open; that has since been run on the Creator's Mac and is recorded in the Mac observed section in this same answer. `bpy.context.window_manager.reports` does not exist in this version.

**Not covered.** This answer settles only that the sources are silent. It does not settle whether a render is in fact disturbed. It also does not settle the narrower questions underneath it: whether a running render holds anything open that an installation would touch, whether the preferences file being rewritten matters to a render already loaded, or whether the repository lock has any bearing. All of those would need a real test.

### Observed — the same question asked again on a Mac, including with a window

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 19, 20, 21, 22, 23 and 25.

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

*Installs fired at a running render, from a second Blender.* An eight-frame Cycles animation was rendered twice — once undisturbed, once with install-and-remove pairs fired at it from a second Blender for as long as it ran (five pairs). Both runs wrote all eight frames and exited zero. Every install and every removal exited zero. Comparing the two runs picture by picture rather than file by file, every frame was identical:

```
PIXELCMP .../control_5.1/f_0001.png .../test_5.1/f_0001.png differing_values 0 of 307200 worst 0.0
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

*With a window — the part no machine could reach before.* Blender 5.1.2 was started with a window on the Mac's screen and driven by a script that called Blender's own commands and took Blender's own screenshots. An animation render was started, and after the second frame a handler installed a package:

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

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 5.1.2 at `/opt/blender/5.1/blender`, on 2026-08-07. All four versions were run and all four agreed. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`, inside a working folder of this test's own at `/root/again-add-ons/`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. Other topics' tests were running Blender on the same machine throughout, so **no time here is read as a measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entry 28.

**Why this was run again.** The citation check sent the windowed `use_lock_interface` run back, because nothing in its record showed the setting was in force at the moment the install fired — see the note in the Mac section above. This is that test rebuilt in the form a machine with no screen can reach. **The windowed form still needs the Creator's Mac and has not been run again; that half stays sent back.**

**What was done.** The eight-frame scene was rendered three times in Blender 5.1.2: once undisturbed as a control; once with `use_lock_interface` left `False` and a package installed from a `render_post` handler after the second frame; and once with `use_lock_interface` set to `True` and the same install. Cycles on the processor, eight of the twenty-four cores, 512 samples, adaptive sampling and denoising off, a fixed seed, 640 by 480. In every run the flag was printed back before the assignment, after it, at each of the eight frames **from inside the handler**, immediately after the install, and after the render.

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every part of this question was reached with no window at all: the render, the installs, the removals, and an install fired from inside the rendering Blender by a handler. None of it needed a screen. **The windowed half has since been run on the Creator's Mac and agrees with the no-window half:** with a window open, an install fired from inside the rendering Blender returned `{'FINISHED'}`, every frame was written, and every frame matched its control picture for picture — with the interface lock off and on alike. What the sources say follows. The silence is the same either way. The command-line install path (`blender --command extension install …`) is documented in full, and `-c, --command` "implies --background mode" (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30) — so installation without a person is well documented, but its effect on a running render is not documented for that path any more than for the interface. **Brought up to date 2026-08-07.** The interface-lock half has now been rebuilt with no window at all, with `use_lock_interface` read back at every step and from inside the render handler at the very frame the install fires — see the observed section on the rebuilt lock test above. With the lock genuinely on, the install returned `{'FINISHED'}`, all eight frames were written, and every frame matched its control picture for picture, in all four versions. Two further things came out of that read-back and bear on this line: `bpy.context.window_manager.is_interface_locked` and `bpy.app.is_job_running('RENDER')` both read `False` at every moment, including mid-render with the lock on, so neither gives a program outside the interface a view of the scene setting or of the running render. **The windowed form of the same test is still waiting on the Creator's Mac and stays sent back.**

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The sources say nothing, so there is nothing to hold or not hold per operating system. I found no operating-system-specific statement on this point anywhere. **Brought up to date 2026-08-07.** The rebuilt interface-lock test was run on the rented Linux machine only, in all four versions, and came out as the Linux runs above. The windowed form of it needs the Creator's Mac and has not been run again. Nothing here changes what stays open for Windows.

---

## 13. Once an add-on is installed and switched on, what more can be learned about whether it will work?

**Label: documented**, for the list of checks below. Each item is quoted from the sources. Where I have gone one step past the words, I say so and label that step **inferred**.

**The question is ambiguous.** I read it as: after installing and enabling, what further evidence does Blender offer that the add-on will actually function. The other reading would be: what facts about the add-on itself become readable once it is installed (its name, location, version and so on). I answer the first reading in full. The second reading is answered by items 3, 4, 8, 9 and 10 below, which between them cover what the add-on's own entry shows, its preferences, listing the installed packages from outside the interface, validating a package's metadata, and the `warning` field.

### The checks the sources describe

**1. The console carries the errors.** The same tip appears on three separate pages:

> "If the Add-on does not activate when enabled,
> check the Console window
> for any errors that may have occurred."

Sources: the manual, https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html, https://docs.blender.org/manual/en/5.1/editors/preferences/extensions.html, and https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html — all read 2026-07-30.

This is the sources' own answer to "how do I tell if it worked": look at the console. The quoted sentence uses "enabled" and "activate" as two different things — an add-on can be enabled and still not activate — and it sends the reader to the console to tell the two apart. That much is in the words. What the words do not say is whether the tick-box itself ever shows the difference.

**2. Refresh Local re-scans and reports warnings.**

> "Refresh Local
>
> Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting).
> Any issues are reported as warnings."

Source: the manual, https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html — read 2026-07-30. The same wording appears at https://docs.blender.org/manual/en/5.1/editors/preferences/extensions.html.

The matching operator:

> "bpy.ops.extensions.repo_refresh_all(*, use_active_only=False)
>
> Refresh extension & legacy add-ons, reloading modules & meta-data (similar to restarting)"

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.extensions.html — read 2026-07-30.

So a deliberate re-scan exists, and it produces warnings when there are issues. What "issues" covers is not spelled out.

**3. The add-on's own entry can be opened for its details.**

> "You can click the arrow at the left of the add-on box to see more information,
> such as its location, a description and a link to the documentation.
> Here you can also find a button to report a bug specific of this add-on."

Source: the manual, https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html — read 2026-07-30.

**4. The add-on may need its own preferences set before it does anything.** This is the sources' explicit warning that "installed and enabled" is not the end of the setup:

> "Some add-ons use this section for example to enable/disable
> certain functions of the add-on. Sometimes these might even all default to off.
> So it is important to check if the enabled add-on has any particular preferences."

Source: the manual, https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html — read 2026-07-30.

Those preferences are readable from Python. An add-on's preferences are reached through the package name:

> "addon_prefs = bpy.context.preferences.addons[__package__]"

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html — read 2026-07-30. The API describes the same collection: `Preferences.addons` is of type `Addons[Addon]`, and each `Addon` has a `module` ("Module name") and a read-only `preferences` (https://docs.blender.org/api/5.1/bpy.types.Preferences.html and https://docs.blender.org/api/5.1/bpy.types.Addon.html, read 2026-07-30). The class description for `Addon` is "Python add-ons to be loaded automatically".

**5. If the add-on brought Python modules with it, you can check they load.** This is a direct, documented test:

> "Once the extension has been installed you can check the module is being loaded by importing it in the
> Python console and printing it's location:
>
> import PIL
> print(PIL.__file__)"

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/python_wheels.html — read 2026-07-30.

**6. If the add-on carries compiled files, they may not match this computer or this Blender.**

> "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular
> Python scripts, these are not portable between different platforms."

> "It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system)."

> "If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating
> system."

Source: the manual, https://docs.blender.org/manual/en/5.1/troubleshooting/python.html — read 2026-07-30.

The same page gives a script to paste into the Text editor to find duplicate Python libraries on the search path, and says "(The output will show in Command Line Window.)". That is a further concrete check, though it is written for Windows DLL clashes.

**7. If the add-on needs the internet, that is a separate switch.**

> "If the add-on needs to use internet, it must check for the (read-only) property bpy.app.online_access.
> This option is controlled by Preferences, which can be overriding via command-line
> (--offline-mode / --online-mode)."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html — read 2026-07-30.

The property itself: "bpy.app.online_access — Boolean, true when internet access is allowed by Blender & 3rd party scripts (read-only)." and "bpy.app.online_access_override — Boolean, true when internet access preference is overridden by the command line (read-only)." (https://docs.blender.org/api/5.1/bpy.app.html, read 2026-07-30).

An extension may declare in its manifest what it needs, and that declaration is readable:

> "# # Optional: add-ons can list which resources they will require:
> # # * files (for access of any filesystem operations)
> # # * network (for internet access)
> # # * clipboard (to read and/or write the system clipboard)
> # # * camera (to capture photos and videos)
> # # * microphone (to capture audio)"

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/getting_started.html — read 2026-07-30. That page also says the permissions list is optional, so its absence proves nothing.

**8. The installed packages can be listed from outside the interface.**

> "Subcommand: list
>
> usage:
>
> blender --command extension list [-h] [-s]
>
> List packages from all enabled repositories."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html — read 2026-07-30.

Careful here: the sources say this lists packages. They do not say it reports health. Treating a name appearing in that list as proof the add-on works would be my invention, not the source's claim.

**9. A package's own metadata can be validated, including a packaged one.**

> "You may also validate a package without having to extract it first.
>
> blender --command extension validate add-on-package.zip"

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/getting_started.html — read 2026-07-30. The subcommand's own page says it "Validate the package meta-data in the current directory" and accepts "either directory containing package files or the package archive" (https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html, read 2026-07-30). So this checks the manifest, not the running add-on.

**10. A legacy add-on's `bl_info` can carry a `warning` field.** The manual's own tutorial says `bl_info` "is a dictionary containing add-on metadata such as the title, version and author to be displayed in the Preferences add-on list" (https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html, read 2026-07-30). Read that carefully: it names the title, version and author as the things displayed. It does not say the `warning` field is displayed, and I am not treating it as saying so. That a `warning` key exists at all is **observed** in a file supplied here — Cycles' own `bl_info` has one, empty:

> `    "warning": "",`

File: `Blender Sources/cycles_5.1/__init__.py`, line 12 (inside the `bl_info` dictionary that begins on line 7). Read 2026-07-30.

The API documentation's `AddonPreferences` example also carries `"warning": "",` in its `bl_info` (https://docs.blender.org/api/5.1/bpy.types.AddonPreferences.html, read 2026-07-30). The sources do not say what Blender does with a non-empty `warning`; I am not guessing.

### What the interface itself shows

I opened the manual's screenshot of the Add-ons preferences section, `_images/editors_preferences_section_addons.webp`, shown on https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html (read 2026-07-30). **Documented**, from the picture and not from any sentence — so there is no quote behind this paragraph, and anyone checking it has to open that picture rather than look for words. What the picture shows: each add-on is one row with a disclosure arrow, a tick-box, the add-on's name, and one icon at the right end of the row. Rows with the box ticked are drawn bright; rows with the box empty are drawn dimmed. Above the list are a search field, an "Enabled Only" tick-box, a tag icon and a drop-down.

What that picture does **not** show is any status, warning or error marking on any row. I am careful with this: the picture shows a list with no add-ons in trouble, so its silence is not proof that no such marking exists. What it does establish is that the checkbox is the only per-add-on state visible at a glance in that screenshot — consistent with the manual sending you to the console for anything more.

### Observed — what Blender 5.1 actually reports about an add-on, with no window

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — with Blender 5.1.2 at `/opt/blender/5.1/blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the machine's own settings folder was never touched. The add-ons were written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 12, 13, 14, 15, 16, 17 and 24.

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

**Which checks work with no window at all.** Every one of these returned rather than raising: `bpy.app.background` (`True`), `bpy.context.preferences.addons.keys()`, `addon_utils.paths()` (`['/opt/blender/5.1/5.1/scripts/addons_core']`), `bpy.app.online_access`, `bpy.app.is_job_running('RENDER')`, `bpy.utils.extension_path_user(...)` (`'<settings folder>/extensions/.user/user_default/smoozy_ok'`), `bpy.data.workspaces[0].use_filter_by_owner`, `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()`, `bpy.ops.preferences.addon_expand(module=...)`, `bpy.ops.extensions.status_clear_errors()`, `bpy.context.preferences.version` (`(5, 1, 30)`) and `addon_utils.check(...)`. One did not: `SMOOZY13 wm_reports RAISED AttributeError 'WindowManager' object has no attribute 'reports'`.

`bpy.ops.preferences.addon_enable` takes exactly `['module', 'rna_type']`, so the argument name used above is the real one.

**Logging arguments.** `--log-list-categories` is accepted and prints its categories, beginning `addon`, `anim`, `anim.action`, and `--log-level 2` prints timestamped lines such as `00:00.340  context          | unknown:0: screen=<Screen("Layout")...>`. In 4.5 the same argument is not accepted at all.

**Exit status.** A failed switch-on leaves the exit status at zero unless `--python-exit-code 1` is given *and* the error is left uncaught, in which case it is 1. The same failure through `addon_utils.enable` exits 0 either way. Naming a broken add-on at launch with `--addons` prints the traceback and exits 0 in both cases. Naming a *working* add-on with `--addons` loads and registers it but leaves `bpy.context.preferences.addons` empty: `SMOOZY_LOADED True`, `SMOOZY_CHECK (True, True)`, `SMOOZY_COLL []`.

**A legacy add-on's `warning` field reads back exactly as written**, alongside five keys Blender fills in that the file never carried:

```
SMOOZY_BLINFO smoozy_legacy_warn {'name': 'Smoozy Legacy Warn', ..., 'warning': 'SMOOZY WARNING TEXT', 'category': 'Development', 'location': '', 'doc_url': '', 'support': 'COMMUNITY', 'show_expanded': False, '_init': None}
```

Whether that text is *shown* anywhere has since been run on the Creator's Mac, which has a screen: it is shown, both as a triangle on the add-on's row and as a `Warning` line inside it. See the Mac observed section in this same answer.

**Not covered by this test:** whether a broken add-on is marked in the Add-ons list, and whether a non-empty `warning` is displayed — both need a window, and both have since been run on the Creator's Mac; see the Mac observed section in this same answer. A declared warning is shown; a failure to switch on is not marked at all. What the extensions error status holds; `status_clear_errors` returned `{'FINISHED'}` and printed nothing. Nothing here was run on macOS or Windows.

**Not covered.** This answer lists what the sources describe. It does not settle what "issues are reported as warnings" actually covers, what a non-empty `warning` field does, whether the Add-ons list marks a broken add-on in any way, or whether the operators (`bpy.ops.preferences.addon_enable`, `bpy.ops.extensions.package_install`) signal failure in a way a script can act on — the API pages give only the generic "Result of the operator call" for all of them. It also does not settle anything about whether the add-on will do the right thing at render time, as opposed to registering without error.

### Observed — the same checks asked again on a Mac, including the Add-ons list on screen

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 12, 13, 14, 15, 16, 17, 24 and 26.

**The Mac agreed with Linux on every check.** What follows is what Blender 5.1.2 printed on the Mac.

*Every check, with no window.* Seventeen checks were run in one script in a Blender started with `--background`. All of them returned rather than raising, except one:

```
SMOOZY13 background -> True
SMOOZY13 addons_keys -> ['bl_ext.user_default.smoozy_ok', 'bl_pkg', 'cycles', ...]
SMOOZY13 addon_utils_paths -> ['/Applications/Blender 5.1.2.app/Contents/Resources/5.1/scripts/addons_core']
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
SMOOZY13 prefs_version -> (5, 1, 30)
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

*On screen — the part no machine could reach before.* Blender 5.1.2 was started with a window, four legacy add-ons were switched on from a script, the Add-ons section of Preferences was opened and searched for "smoozy", and Blender took its own screenshots. Three things are visible in them.

- **A declared warning is shown.** The row for the add-on carrying `"warning": "SMOOZY WARNING TEXT"` has a warning triangle at its right-hand end where the other rows have the plain legacy-add-on icon, and when the row is opened it carries a line reading `Warning` followed by the same triangle and the words `SMOOZY WARNING TEXT`.
- **An add-on that failed to switch on is not marked at all.** Its row looks exactly like a row for an add-on simply left switched off: the tick box is empty, there is no triangle, and opening the row shows only the ordinary Type, Maintainer, Version and File lines. Nothing on screen distinguishes "it broke" from "it is off".
- **The one declaring it needs Blender 99 is in the list and can be switched on.** Its row is present and its tick box can be ticked, which agrees with what the Python side reported.

`addon_utils.check` matched the picture: `(True, True)` for the three that switched on and `(False, False)` for the one that failed.

**Not covered by this test:** nothing was run on Windows, because no Windows machine has run it, so the Windows-only console instruction and the mixed-library check on the troubleshooting page stay untested. What the extensions error status holds is still unknown; `status_clear_errors` returned `{'FINISHED'}` and printed nothing.


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every Python and command-line check named in this answer was run in a Blender started with no window and returned rather than raising, including `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()` and `bpy.ops.preferences.addon_expand()`, which the sources leave unstated. **The parts that are panels in the Preferences editor have since been reached on the Creator's Mac, which has a screen** — see the Mac observed section above. A declared warning is shown on the row and inside it; an add-on that failed to switch on is not marked at all. What the sources say follows. Most of the list survives without a window, but not all of it, and the split matters:

- Holds outside, **documented**: `blender --command extension list` and `blender --command extension validate` are both written as command-line subcommands, and `-c, --command` "implies --background mode" (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30). `bpy.app.online_access` is described as controllable by `--offline-mode` / `--online-mode` on the command line.
- Holds outside, **inferred**, and I mark it rather than let it pass: that the console output is the terminal when Blender is run without a window; that `bpy.context.preferences.addons` and an add-on's own preferences can be read from Python with no window; that a bundled module can be imported and its location printed with no window; and that `bpy.ops.extensions.repo_refresh_all()` runs with no window. None of the pages I read says any of these works in background mode. They are all plain Python and nothing in them obviously needs a window, which is the step — and it is the same step I refuse to take for `bpy.ops.preferences.copy_prev()` under question 14, so it is marked the same way here. Treat them as things to test.
- Interface only: the disclosure arrow, the description and documentation link, the report-a-bug button, and anything read off the Add-ons list by eye.
- The duplicate-library script on the Python Errors page is written as something to paste into the Text editor, which is an interface thing, but it is plain Python and nothing in it needs a window. Saying it would run identically under `--python` is a step past the words: **inferred**.
- Worth noting for automated use: the preferences still apply when Blender is driven from the command line. "In this case, the Preferences are still used but you may want to override them" (https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html, read 2026-07-30 — the sentence is about script auto-execution, so applying it to add-on preferences generally would be **inferred**, and I do not lean on it).

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. Two of the items are explicitly operating-system-shaped. The precompiled-library warning is about portability "between different platforms" and tells you to check `.pyd` or `.so` files "compatible with your operating system"; the duplicate-DLL section under it is headed "Platform Specific — Windows" and its script uses Windows-only calls. Everything else in the list is described without any operating-system qualification. The sources do not state that the rest behaves identically on every operating system; they simply do not distinguish.

---

## 14. Does an add-on set up for one Blender work in another Blender on the same computer, and what decides that?

**Label: documented**, for the mechanism. Three separate steps go past the words, and each is marked **inferred** where it occurs — in section 3, in section 5, and in section 6.

**The question is ambiguous.** "Another Blender on the same computer" can mean a different *version* of Blender, or a second *installation* of the same version. I answer the version reading, which is what the sources address at length, and I cover the same-version reading in a short section of its own.

### The short answer

On the version reading — a different version of Blender — no, not on its own. (The same-version reading comes out the other way, and is section 6.) What decides it is four separate things, and all four must line up:

1. Whether the second Blender looks in the same place on disk.
2. Whether the second Blender's preferences say the add-on is enabled.
3. Whether the add-on declares that it supports that Blender version.
4. Whether anything compiled inside the add-on matches that Blender's bundled Python and the computer's operating system.

### 1. Where each Blender looks: separate folders per version

The user directory carries the version number in its path. For 5.1:

> "Linux
>
> $HOME/.config/blender/5.1/
>
> …
>
> macOS
>
> /Users/$USER/Library/Application Support/Blender/5.1/
>
> Windows
>
> %USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.1\"

And what lives in there:

> "User directories store preferences, startup file, installed extensions,
> presets and more."

> "./extensions
>
> Extension repositories.
>
> Located in both user and system directories. Repositories are loaded from
> both directories."

> "./scripts
>
> Add-ons, presets, templates, user interface, startup scripts.
>
> Located in both user and system directories. Scripts are loaded from
> both directories."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30. The system directories on the same page are also version-numbered (`./5.1/`, `%ProgramFiles%\Blender Foundation\Blender\5.1\`, and so on).

The manual states the consequence outright, on the page about first-run setup:

> "The preferences need to be imported from previous versions because the configuration files of each Blender version are
> stored in separate folders."

Source: the manual, https://docs.blender.org/manual/en/5.1/getting_started/configuration/defaults.html — read 2026-07-30.

And that installations sit side by side:

> "Multiple Blender versions can co-exist on the same system, and deployment can be automated using
> standard file management tools."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30.

### 2. Carrying them across: the documented route, and its warning

Blender offers a one-off copy when a new version first runs:

> "When you start Blender for the first time or update to a new version, the interactive region of
> the Splash Screen is replaced with a few initial preferences to configure
> how you interact with Blender."

> "Import Preferences From Previous Version
>
> Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and
> startup files from the previous version of Blender and load them. This will include installed add-ons and extensions."

And, in the same breath, the warning that copying does not mean working:

> "Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this
> option may lead to errors on startup. If this occurs, the recommended first step is to try Loading Factory
> Settings."

Source: the manual, https://docs.blender.org/manual/en/5.1/getting_started/configuration/defaults.html — read 2026-07-30.

The operator behind it:

> "bpy.ops.preferences.copy_prev()
>
> Copy settings from previous version"

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.preferences.html — read 2026-07-30.

Note the shape of this: it copies. It does not share. Two versions each end up with their own copy.

### 3. Making two Blenders genuinely look in one place

The sources describe several ways to override the default per-version location, which is what would make one add-on serve two Blenders rather than being copied twice.

Environment variables, one per resource kind:

> "BLENDER_USER_RESOURCES:
>
> Replace default directory of all user files.
> Other BLENDER_USER_* variables override when set."

> "BLENDER_USER_EXTENSIONS:
>
> Directory for user extensions."

> "BLENDER_USER_SCRIPTS:
>
> Directory for user scripts."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. The Directory Layout page states the same for `BLENDER_USER_RESOURCES`, and adds `BLENDER_SYSTEM_SCRIPTS` and `BLENDER_SYSTEM_EXTENSIONS`, which "can be used to bundle additional scripts and extensions, that are not part of the regular Blender installation" (https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html, read 2026-07-30).

A shared read-only repository, which is the studio route:

> "When working offline or in a more controlled environment, it may be useful
> to provide a set of extensions to all users. For this there is a default
> read-only System repository. This repository could be located on a
> read-only network drive or in a system directory."

> "The $BLENDER_SYSTEM_EXTENSIONS
> environment variable
> controls the default location."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30.

A per-repository custom directory, set in preferences:

> "use_custom_directory
>
> Manually set the path for extensions to be stored. When disabled a user's extensions directory is created. (default False)"

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.types.UserExtensionRepo.html — read 2026-07-30.

And a Script Directories entry, which the Add-ons page describes step by step for folder-based add-ons, ending "Save the preferences and restart Blender for it to recognize the new add-on location." (https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html, read 2026-07-30).

**Inferred**, and here is the step: the sources say each Blender version defaults to its own version-numbered directory, and separately that these environment variables replace the default directory. Putting those two together — that pointing two different Blender versions at one directory would let both see the same installed add-on — is a step the sources do not take in so many words. Nothing I read states that two Blender versions sharing one extensions directory is supported or that it works. Treat it as a thing to test, not a thing established.

### 4. What the add-on itself declares about Blender versions

For an extension, the manifest carries a required minimum and an optional maximum:

> "blender_version_min:
>
> Minimum supported Blender version - use at least 4.2.0."

> "blender_version_max:
>
> Blender version that the extension does not support, earlier versions are supported."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/getting_started.html — read 2026-07-30. The same two fields appear in a repository listing (https://docs.blender.org/manual/en/5.1/advanced/extensions/creating_repository/static_repository.html, read 2026-07-30). Read `blender_version_max` carefully: it names the version that is *not* supported, not the last one that is.

For a legacy add-on, the `bl_info` dictionary carries a minimum, and the manual states the consequence:

> "is a dictionary containing add-on metadata such as the title,
> version and author to be displayed in the Preferences add-on list.
> It also specifies the minimum Blender version required to run the script;
> older versions won't display the add-on in the list."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html — read 2026-07-30. The key in question is `"blender"`, shown in that page's example as `"blender": (2, 80, 0),`.

**Observed**, in a file supplied here: Cycles, which ships with 5.1, declares exactly this:

> ```
> bl_info = {
>     "name": "Cycles Render Engine",
>     "author": "",
>     "blender": (2, 80, 0),
> ```

File: `Blender Sources/cycles_5.1/__init__.py`, lines 7–10. Read 2026-07-30. That is what the field looks like in a real add-on. What Blender does with it at load time is beyond what the file shows.

The maintainer's side of this is spelled out too:

> "New Blender versions can sometimes introduce breaking changes in Blender's
> Python API or even change how entire features work. If this affects your
> extension, you may want to maintain two "tracks" of your extension concurrently:
> one for Blender "old" and one for Blender "new"."

> "Make sure to correctly indicate the Blender versions that each version of
> your extension is compatible with in their manifest file. You can also update
> Blender version compatibility of already-uploaded versions of your add-on from
> the extensions website."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/version_number_guidelines.html — read 2026-07-30.

### 5. What the add-on carries inside it: Python version and operating system

This is the part that bites hardest when moving between Blender versions.

> "Python Version
>
> Blender and the bpy module are only compatible
> with a single Python version. This makes it possible for add-ons and VFX software
> in general to only have to target a single Python version."

> "Blender bundles a complete Python installation and does not interact with the
> system Python by default. This can be changed with the --python-use-system-env
> command line argument, if care is
> taken to set up a compatible Python version."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30.

Which Python 5.1 carries is stated in the release notes:

> "## Python 3.13
>
> Python has been upgraded to [version 3.13](https://www.python.org/downloads/release/python-3130/), matching the [VFX platform 2026](https://vfxplatform.com/)."

Source: the 5.1 release notes, https://developer.blender.org/docs/release_notes/5.1/python_api/ — read 2026-07-30. This is a release note stating a change, quoted as such.

Bundled wheels are built against a named Python version and a named platform. The manual's own example:

> "pip download pillow --dest ./wheels --only-binary=:all: --python-version=3.13 --platform=macosx_11_0_arm64"

and the resulting manifest entries:

> "wheels = [
> "./wheels/pillow-12.1.0-cp313-cp313-macosx_11_0_arm64.whl",
> "./wheels/pillow-12.1.0-cp313-cp313-manylinux_2_27_x86_64.manylinux_2_28_x86_64.whl",
> "./wheels/pillow-12.1.0-cp313-cp313-win_amd64.whl",
> ]"

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/python_wheels.html — read 2026-07-30. The `cp313` in those file names is the Python version the wheel was built for; the manual's `--python-version=3.13` command above it is where that comes from.

And the failure this produces is described directly:

> "It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system)."

Source: the manual, https://docs.blender.org/manual/en/5.1/troubleshooting/python.html — read 2026-07-30.

Putting these together: an add-on carrying compiled parts built for one Blender's Python will not automatically be fit for a Blender bundling a different Python. That much the sources say between them; the joining of "Blender bundles one Python version" and "a library built for a different Python version is incompatible" is a small step, so I label that joining **inferred** rather than documented.

### 6. Two installations of the same version

On the same-version reading, the same directory rules answer it: both installations of 5.1 default to the same version-numbered user directory, so both would find the same installed add-ons and the same preferences. The exception the sources give is a portable install:

> "When running Blender from a portable drive, it's possible to keep the configuration
> files on the same drive to take with you.
>
> To enable this, create a folder named portable at the following locations: …
>
> This folder will then store preferences, startup file, installed extensions
> and presets."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30. A portable install therefore keeps its own installed extensions rather than sharing.

**Inferred**, and stated as such: that two same-version installations sharing one user directory would both see the same add-on follows from the directory being defined by version rather than by installation. The sources do not say this in so many words.

### Observed — what four Blenders on one computer actually do

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05 [the citation check, reading the same machine on 2026-08-06, was told driver 580.173.02; the original reading is left as written], an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — on 2026-08-07, with Blender 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS side by side at `/opt/blender/4.5/blender` and so on. Every Blender was pointed at a settings folder or a stand-in home folder made for the test beforehand; the machine's own settings folder was never touched. The add-ons were written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 1 to 11 and 18.

**Which Python each one carries**, from `print(sys.version)` in a Blender with no window:

```
4.5.12 LTS  ->  3.11.11 (main, Apr 25 2025, 12:39:20) [GCC 11.2.1 20220127 (Red Hat 11.2.1-9)]
5.0.1       ->  3.11.13 (main, Apr 25 2025, 12:39:20) [GCC 11.2.1 20220127 (Red Hat 11.2.1-9)]
5.1.2       ->  3.13.9  (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-7)]
5.2.0 LTS   ->  3.13.13 (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-11)]
```

Blender 5.1 carries Python 3.13.9 in the build on this machine. So the four split two and two.

**The folder each one reads carries two numbers and not three.** With `HOME` pointed at a folder of the test's own and nothing else set, Blender 5.1 printed:

```
SMOOZY_USER <home>/.config/blender/5.1
SMOOZY_LOCAL /opt/blender/5.1/5.1
SMOOZY_EXT <home>/.config/blender/5.1/extensions
SMOOZY_SCRIPTS <home>/.config/blender/5.1/scripts
SMOOZY_CONFIG <home>/.config/blender/5.1/config
```

while `bpy.app.version` read the full three numbers. The patch number never enters the path. Starting Blender did not create the folder; only an install or a saved preference did.

**Nothing is shared by default.** Four differently-named extensions were installed, one by each Blender, into one shared home folder. Each Blender listed only its own — `5.1` listed `smoozy_v51 [installed]` and nothing else — and four separate version-named folders existed on disk, each with its own `extensions/user_default/`.

**Sharing a folder does work, in two different degrees.** With `BLENDER_USER_EXTENSIONS` pointed at one folder for all four but each keeping its own settings folder, an extension installed by 4.5 was listed by all four and could be switched on by all four — but only the Blender whose settings folder recorded it had it switched on at start-up. With `BLENDER_USER_RESOURCES` pointed at one folder for all four, all four listed it *and* all four had it switched on at start-up. The same held with 5.2 installing and the older three reading.

**A shared system-extensions folder serves all four.** With the extension's own folder placed at `<folder>/system/smoozy_ok` and `BLENDER_SYSTEM_EXTENSIONS` set to `<folder>`, every one of the four listed it under `Repository: "System" (id=system)` and switched it on as `bl_ext.system.smoozy_ok`.

**A shared scripts folder serves all four, for a legacy add-on.** With `BLENDER_SYSTEM_SCRIPTS` set to a folder holding `addons/smoozy_legacy_ok.py`, all four printed `SMOOZY_MODULES ['smoozy_legacy_ok']` and switched it on.

**Two separate copies of one build share, and a `portable` folder takes over.** `/opt/blender/5.1` was copied to a second folder. An extension installed with the original was listed by the copy, and both resolved `bpy.utils.resource_path('USER')` to the same `<home>/.config/blender/5.1`. A `portable` folder made next to the copy's program changed that at once — the copy then resolved to `<copy>/portable` and listed nothing. The same for the other three versions.

**A folder copied by hand from one version to another works.** The installed folder of an extension was copied unchanged out of `blender/4.5/extensions/user_default/` into the matching place under 5.0, 5.1 and 5.2; each then listed it and switched it on with no complaint. The same worked out of 5.2 into 4.5, 5.0 and 5.1.

**A declared version range is acted on, at install, and the message names the versions.** An extension declaring `blender_version_min = "99.0.0"` and one declaring `blender_version_max = "4.3.0"` were both built and validated without complaint, then offered to Blender 5.1.2:

```
ERROR smoozy_min99: This Blender version (5.1.2) doesn't meet the minimum supported version (99.0.0)
ERROR smoozy_max40: This Blender version (5.1.2) must be less than the maximum version (4.3.0)
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

**Not covered.** This answer does not settle what Blender actually does when an add-on's declared minimum or maximum version excludes the running Blender — the manual states the effect only for legacy add-ons ("older versions won't display the add-on in the list") and says nothing about what an extension outside its declared range does at install or enable time. It does not settle whether an enabled state carries across versions by itself (only that the whole preferences file can be copied at first run). It does not settle whether pointing two versions at one shared extensions directory is supported. And it does not settle what happens when an add-on is simply copied by hand from one version's folder into another's — no page I found describes that.

### Observed — the same questions asked again of four Blenders on a Mac

**LABEL: observed.** Done on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card — with Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, on 2026-08-07. Every Blender was pointed at a settings folder made for the test beforehand with `BLENDER_USER_RESOURCES`; the Creator's own Blender settings were never installed into and never changed. The add-ons were written on the Mac for the test, not fetched. Other topics were running Blender on the same Mac during this sitting, so **every time below is from a machine that was not idle and is not a clean measurement**. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entries 1 to 11, 18 and 27. All four Blenders — 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS — sit side by side on this Mac and all four were used.

**The Mac agreed with Linux on everything except one thing, and that one thing is set out at the end.**

*Which Python each carries.* The four differ, and they split two and two:

```
4.5 -> 3.11.11    5.0 -> 3.11.13    5.1 -> 3.13.9    5.2 -> 3.13.13
```

Blender 5.1.2 carries Python 3.13.9. These are the same four Pythons the Linux builds carry.

*Where each Blender's own folder sits.* With nothing overridden, Blender 5.1.2 computes:

```
SMOOZY_USER /Users/artist/Library/Application Support/Blender/5.1
SMOOZY_LOCAL /Applications/Blender 5.1.2.app/Contents/Resources/5.1
SMOOZY_SYSTEM /Library/Application Support/Blender/5.1
```

The folder is named for the major and minor number only; the patch number never enters it, so 4.5.12 reads a folder called `4.5`. This is the macOS layout the sources describe, confirmed.

**One thing works differently on macOS from Linux, and it changed how every test below had to be run.** On the Linux machine a stand-in home folder redirects Blender's user folder. On macOS it does not: with `HOME` pointed at an empty folder made for the test, all four Blenders still computed `/Users/artist/Library/Application Support/Blender/<version>` and read the Creator's own settings. `BLENDER_USER_RESOURCES` does redirect it on macOS, and every test in this sitting used that instead. Where a test needed several versions to share one parent folder — the way Blender's own layout puts `4.5`, `5.0`, `5.1` and `5.2` side by side — that layout was laid out by hand and each version pointed at its own folder inside it.

*Nothing is shared between versions by default.* Each of the four installed a differently-named package into its own folder, and then each was asked what it held. Each listed only its own: Blender 5.1.2 showed `smoozy_v51 [installed]` and nothing else, and `SMOOZY_ADDONS ['bl_ext.user_default.smoozy_v51']`.

*Handing several versions one folder does make an add-on cross.* Four arrangements were run, installing with the oldest and reading with the other three, then installing with the newest and reading with the other three:

- One extensions folder handed to all four with `BLENDER_USER_EXTENSIONS`: every version listed it, and every version loaded it — `SMOOZY_ENABLE <module 'bl_ext.user_default.smoozy_ok' from '.../shared_ext/user_default/smoozy_ok/__init__.py'>`. Only the version that did the installing had it switched on in its own collection.
- One settings folder handed to all four with `BLENDER_USER_RESOURCES`: every version listed it, loaded it, **and** had it switched on — `SMOOZY_ADDONS ['bl_ext.user_default.smoozy_ok']` in all four.
- One system-extensions folder handed to all four with `BLENDER_SYSTEM_EXTENSIONS`: every version listed it under System and loaded it.
- One scripts folder handed to all four with `BLENDER_SYSTEM_SCRIPTS`, holding a legacy add-on: every version saw it among its modules and switched it on, printing its register line.

*A folder copied by hand also works.* An extension installed by 4.5 was copied, unchanged, into 5.0's, 5.1's and 5.2's folders; then the same the other way, out of 5.2 into the older three. Every destination version listed it and loaded it, in both directions.

*Two copies of one Blender.* A second copy of Blender 5.1.2 was made in a folder of its own. With nothing overridden, the copy computed the same user folder as the original — `/Users/artist/Library/Application Support/Blender/5.1` — so two installations of one version share by default. A `portable` folder made inside the copy's bundle at `Contents/Resources/portable`, which is where the sources say it goes on macOS, took over completely:

```
SMOOZY_USER .../copy_5.1.app/Contents/Resources/portable
SMOOZY_EXT .../copy_5.1.app/Contents/Resources/portable/extensions
SMOOZY_CONFIG .../copy_5.1.app/Contents/Resources/portable/config
```

and the copy's listing then held nothing. `BLENDER_USER_RESOURCES` still wins over a `portable` folder: with both in play, the copy read the folder the variable named.

*A declared version range is acted on.* An extension declaring it needs at least Blender 99 was refused at install:

```
ERROR smoozy_min99: This Blender version (5.1.2) doesn't meet the minimum supported version (99.0.0)
```

and one declaring it does not support 4.3 and above was refused too:

```
ERROR smoozy_max40: This Blender version (5.1.2) must be less than the maximum version (4.3.0)
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

*A bundled Python package built for another Python.* Three packages were built by hand, each carrying a wheel tagged for a different Python, and each was installed into a settings folder of its very own so that nothing could be left behind from a previous case. Blender 5.1.2 accepted only the one tagged `cp313` — `STATUS Installed`, then the bundled module imported from `.../extensions/.local/lib/python3.13/site-packages/smoozypkg/__init__.py`. The other two were refused:

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

**LABEL: observed.** Done on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and **no display server at all** — on 2026-08-07, with Blender 5.1.2 at `/opt/blender/5.1/blender` and the other three beside it; all four were run. Every Blender was pointed at a settings folder or a stand-in home folder made for the test beforehand, inside a working folder of this test's own at `/root/again-add-ons/`; the machine's own settings folder was never touched. The add-on was written on the machine for the test, not fetched. Full procedure: `Audits/Blender Research/Add-ons/Hand tests — added questions.md`, entry 29.

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


**Outside the interface.** **Settled by test on Linux, and on the Mac for everything but one item — see the two observed sections above.** All of it holds with no window, including the one part the sources leave interface-bound: `bpy.ops.preferences.copy_prev()` ran under `--background`, returned `{'FINISHED'}`, and carried the older version's installed extension into the newer version's folder. **On the Creator's Mac this does not hold for one item:** `bpy.ops.preferences.copy_prev()` refuses to run there in all four versions, with a window open as well as without one — see the Mac observed section above. What the sources say follows. The parts that decide the answer are all reachable without a window: the directory locations are fixed paths, the environment variables are set before Blender starts, `--env-system-extensions` and `--env-system-scripts` set two of them on the command line, and `--addons` takes a "Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons" (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30). The manifest fields are read from files. Where it does differ: the "Import Preferences From Previous Version" choice is offered in the Splash Screen, which is an interface thing. Its operator `bpy.ops.preferences.copy_prev()` exists, but the sources do not say it can be run in background mode, and I am not assuming it. The deployment page's answer for the no-window case is different in kind — a startup script that enables add-ons for every user: **Brought up to date 2026-08-07.** The one interface-bound item has now been pinned down further on Linux. `bpy.ops.preferences.copy_prev()` is reachable from a script there **only when the version folders sit in Blender's own layout**; when the settings folder is handed over with `BLENDER_USER_RESOURCES` — the way nearly every test in this round hands it over — it refuses in all four versions, `poll()` returning `False`. So on Linux the folder arrangement decides whether this route can be driven from outside the interface at all. See the observed section on the folder arrangements above.

> "$BLENDER_SYSTEM_SCRIPTS/startup/enable_addons.py
>
> def register():
>     import addon_utils
>     addon_utils.enable("my-addon")"

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30.

**Operating system.** **Partly settled by test — see the observed section above.** The per-version folder, the shared-folder routes and the `portable` folder have now been confirmed on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with the macOS paths coming out as the sources describe them, including the `portable` folder inside `Blender.app/Contents/Resources`. **The two machines disagree on one point:** `bpy.ops.preferences.copy_prev()` runs on Linux and refuses to run on macOS. Two further macOS-only points: a stand-in `HOME` does not redirect Blender's user folder there, though `BLENDER_USER_RESOURCES` does. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The version-numbered user and system directories are given separately for Linux, macOS and Windows, and are different on each — so the mechanism holds on all three, but the paths do not transfer. The portable folder location also differs per operating system (next to the executable on Windows and Linux, inside `Blender.app/Contents/Resources` on macOS). Beyond Blender versions, an extension may also restrict itself by operating system: **Brought up to date 2026-08-07.** The two Linux cells were run deliberately and the four cells now stand as a table in the observed section above: on Linux it runs under Blender's own layout and refuses when the settings folder is handed over; on macOS it refuses under both arrangements. The two machines still disagree, and the arrangement does not explain the disagreement away — only the Linux "runs" needed the arrangement named. The macOS cells were not run again in this sitting and are quoted from the Mac record. Windows is unchanged and was still not tried.

> "platforms:
>
> List of supported platforms. If omitted, the extension will be available in all operating systems.
> The available options are
> ["windows-x64", "windows-arm64", "macos-x64", "macos-arm64", "linux-x64"]"

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/getting_started.html — read 2026-07-30. Blender's requirements page (https://www.blender.org/download/requirements/, read 2026-08-01) does not name version 5.1 anywhere on it, so I have taken nothing from it about 5.1.

---

## Things a person at a computer with Blender open could settle in a few minutes

The items on this list have since been run on **both** machines — a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores) and the Creator's own Mac (a MacBook Pro with an M1 Pro chip running macOS 26.5.2) — in all four Blender versions, and their results are written into the answers above as observed findings. They are kept here so those results can be traced back to what was asked. **Nothing on this list is still open.**

The two things those lists did not cover both needed a screen, which that machine does not have. Both have since been run on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — in all four versions, and their results are written into the answers above as observed findings. **Nothing on this list is still open.** The only thing left untested anywhere is Windows, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

Each of these is written so somebody who cannot see my reasoning can carry it out, and each says what a negative result looks like.

**A. Does installing an add-on disturb a render already running in a separate Blender?** (question 12, reading B)

Start a long render with no window: `blender --background scene.blend --render-anim --render-output /tmp/test_`. Wait until frames are appearing. While it is still running, in a second terminal on the same computer run `blender --command extension install-file -r user_default -e some-addon.zip`. Watch the first terminal until the render finishes.
Note the frame number and the clock time just before starting the install, and again just after it reports back.
*Outcome that says "it disturbs":* the render prints an error or warning, or stops printing frames, or exits before its last frame, or the frames printed across the install take longer each than the frames on either side of it.
*Outcome that says "it does not":* the render prints its frames through to the end with no new messages, the per-frame times across the install match the ones on either side, and the installing terminal reports success.
*Negative result:* the render completing untouched. This is a real negative — the test can come out either way, so it carries information.

**B. Does installing an add-on disturb a render running inside the same Blender?** (question 12, reading A)

In one Blender with a window, start a still render of a scene that takes a couple of minutes. While the render window shows progress, open Preferences ‣ Get Extensions ‣ Install from Disk and install an add-on package.
*Outcome that says "it disturbs":* the render aborts, the progress bar stops, or an error appears in the console.
*Outcome that says "it does not":* the install completes and the render finishes normally.
*Negative result:* both complete. Note in advance whether Blender even lets you reach the install control while rendering — if it refuses, record that, as it is itself the answer.

**C. Does the repository lock block a second Blender?** (question 12, background)

Start Blender A with a window. In its Python console run `print(bpy.ops.extensions.repo_lock_all())` and write down what it prints.

**First establish that a lock is actually being held**, because without this step the rest of the test carries nothing — an install that goes through untouched would not tell a lock that does not block a second Blender apart from a lock that was never held. Two ways to show it, and do both: note the contents of the user extensions directory before and after the lock call, so a file the lock leaves behind can be seen; and run `print(bpy.ops.extensions.repo_lock_all())` a second time in the same console and write down whether the second call reports anything different from the first. If neither shows any sign of a lock being held, stop and record that — the test cannot be run, and that is the finding.

Then, with A still open and the lock shown to be held, in a terminal run `blender --command extension install-file -r user_default some-addon.zip`, and write down everything it prints.
*Outcome that says the lock matters across processes:* the command-line install refuses, waits, or reports the repository is locked.
*Outcome that says it does not:* the install goes through and reports success, with the lock demonstrably still held.
Afterwards run `print(bpy.ops.extensions.repo_unlock_all())` in Blender A.
*Negative result:* the install succeeding normally while the lock is shown to be in place. That is a real negative, and it only counts as one because the step above proved the lock was there.

**D. Does an add-on installed in one Blender version appear in another?** (question 14)

On a computer with two Blender versions installed, install an extension in the older one from disk and enable it. Close it. Start the other version and open Preferences ‣ Add-ons. Search for the add-on's name.
*Outcome that says "yes":* the add-on appears in the list.
*Outcome that says "no":* it does not appear at all.
Then check whether the directories are in fact separate: in each Blender's Python console, run `import bpy; print(bpy.utils.user_resource('EXTENSIONS'))` and compare the two paths.
*Negative result:* the add-on not appearing, and the two paths differing. That is the expected result, and it still confirms rather than assumes.

**E. Does pointing two Blender versions at one extensions directory make one install serve both?** (question 14, the inferred step)

Set `BLENDER_USER_EXTENSIONS` to one directory. Start version A with that variable set, install an extension from disk, enable it, and confirm it works. Close A. Start version B with the same variable set to the same directory, and open Preferences ‣ Add-ons.
*Outcome that says "yes":* the add-on is listed in B, can be enabled, and produces no error in the console.
*Outcome that says "no":* it is missing from the list, or it is listed but errors on enable — record which, they mean different things.
*Negative result:* it not appearing, or appearing but failing. Both are possible, so the test is worth running.

**F. What does Blender do when an extension's declared Blender version excludes the running Blender?** (question 14)

Take a small working extension. Edit its `blender_manifest.toml` to set `blender_version_max = "5.0.0"`, rebuild it with `blender --command extension build`, then install it into Blender 5.1 from disk and try to enable it.
*Outcome that says the field is enforced:* the install or the enable is refused, or the add-on is shown but greyed out or marked, or an explanatory message appears.
*Outcome that says it is not enforced locally:* it installs and enables exactly as before, with no message.
*Negative result:* it working normally. Then repeat with `blender_version_min = "6.0.0"` to check the other end.

**G. Where do add-on failures actually surface, and can a script see them?** (question 13)

Write a two-line add-on whose `register()` function raises an error deliberately. Package it and install it. Enable it (i) by ticking the box with the console window visible, and (ii) from the command line with `blender --background --python-expr "import bpy; print(bpy.ops.preferences.addon_enable(module='the_module'))"`. **Check the argument name before running that line**: I took `module` from the `Addon` type's own property, not from the operator's page, so whoever runs this should open https://docs.blender.org/api/5.1/bpy.ops.preferences.html and use whatever that page names the argument. If the line fails because the argument is wrong, that is a mistake of mine and not a result.
*Outcome that says a script can tell:* the printed operator result differs from the result for a healthy add-on (for example `{'CANCELLED'}` rather than `{'FINISHED'}`).
*Outcome that says it cannot:* both print the same thing, and the only difference is text on the console.
*Negative result:* identical results. That is a real possibility and worth knowing, because it decides whether add-on health can be checked automatically or only by reading output.

**H. Does Refresh Local report a broken add-on as a warning?** (question 13)

With the deliberately broken add-on from G installed, run Preferences ‣ Add-ons ‣ Refresh Local, and separately `bpy.ops.extensions.repo_refresh_all()` from the console.
*Outcome that says yes:* a warning appears naming the add-on or its problem.
*Outcome that says no:* nothing is reported and the list looks unchanged.
*Negative result:* silence. Also record whether the Add-ons list marks the broken row in any way — the manual's screenshot shows no such marking, but it shows no broken add-ons either.

---

2026-08-06 — Opus 5 (Claude Code) — Add-ons, Blender 5.1

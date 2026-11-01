# Add-ons — added questions, second pass (Blender 5.1)

Sources consulted: Blender's manual for 5.1, Blender's Python API documentation for 5.1, the Blender 5.1 release notes, and Cycles' own files for 5.1 — all read on 2026-07-30 (the date the copies were taken). Blender's requirements page as taken on 2026-08-01 was also read; it does not name Blender 5.1 anywhere, so it does not speak for this version and nothing below rests on it.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: 12, 13 and 14 only. Every other question on the sheet was left alone.

---

## 12. Does installing an add-on disturb a render already running on the same computer?

**Label: not in the sources.**

**That label answers the question from the sources only. A test has since been run and settled it — see the observed section below in this same answer.**

Nothing in the 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes or Cycles' files says what happens to a render that is already running when an add-on is installed. The sources never put the two things in the same sentence. They describe installing, and they describe rendering, and they never join them.

What the sources *do* say, which bounds the question without answering it:

**A command-line install runs in its own Blender, started in background mode.**

> "-c, --command <command>
>
> Run a command which consumes all remaining arguments.
> Use -c help to list all other commands.
> Pass --help after the command to see its help text.
>
> This implies --background mode."

Source: manual, Command Line Arguments, <https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html>. Read 2026-07-30.

The extension sub-commands, including `install` and `install-file`, are run this way:

> "blender --command extension install [-h] [-s] [-e] [--no-prefs]
> PACKAGES"

Source: manual, Extensions Command Line Arguments, <https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html>. Read 2026-07-30.

**Enabling an add-on inside a Blender takes effect in that Blender straight away.**

> "To enable or disable an add-on check or uncheck the box to the right of the add-ons.
>
> The add-on functionality should be immediately available."

Source: manual, Preferences → Add-ons, <https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html>. Read 2026-07-30.

**A running Blender picks up changes made to add-ons on disk when it is told to scan for them.**

> "Refresh Local
>
> Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting).
> Any issues are reported as warnings."

Source: manual, Preferences → Add-ons, <https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html> (the same wording also appears at <https://docs.blender.org/manual/en/5.1/editors/preferences/extensions.html>). Read 2026-07-30.

The source describes the scan. It does not say what a Blender that never runs the scan does with files that changed underneath it, and it does not mention rendering at all.

**There is a file-system lock on extension repositories.**

> "bpy.ops.extensions.repo_unlock()
>
> Remove the repository file-system lock"

> "bpy.ops.extensions.repo_lock_all()
>
> Lock repositories - to test locking"

Source: Python API, Extensions Operators, <https://docs.blender.org/api/5.1/bpy.ops.extensions.html>. Read 2026-07-30.

That a lock exists is documented. What it guards against, whether it is taken across separate Blender processes, and whether it has anything to do with a render, are not stated anywhere I could find.

**Preferences are written when Blender exits.**

> "Auto-Save Preferences
>
> By default changes to preferences are saved on exit,
> this allows changes to the keymap and Quick Favorites menu to be stored and used between
> Blender Sessions."

Source: manual, Preferences → Introduction, <https://docs.blender.org/manual/en/5.1/editors/preferences/introduction.html>. Read 2026-07-30.

Installing and enabling change the preferences. The source says nothing about two Blenders sharing one preferences file at once, and nothing about renders.

**Where I looked, and what I ruled out**

Whole-tree searches of `manual_5.1` (all `.txt`), `api_5.1` (all `.txt`), `release_notes_5.1` (all `.md`) and `cycles_5.1`, for: `while rendering`, `during a render`, `during rendering`, `render is running`, `currently rendering`, `is being rendered`, `already running`, `in progress`, `another instance`, `second instance`, `concurrent`, `simultaneous`, `at the same time`, `interrupt`, `blocking`, `restart Blender`, `requires a restart`, `reload scripts`, `file-system lock`, `filesystem lock`, `in use by`, `already in use`, `background render`, `--background`, `background mode`. I also cross-searched every manual line containing `install` for the word `render`, and every API line containing `install` for `render`.

Pages opened in full and ruled out:

- `https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html` and `.../editors/preferences/extensions.html` — cover installing, enabling, refreshing. Never mention a render.
- `https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html`, `.../advanced/extensions/index.html`, `.../advanced/extensions/getting_started.html` — about writing and packaging extensions. No render.
- `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html` and `.../advanced/command_line/extension_arguments.html` — the install commands and the render commands are documented on these pages, side by side, with no statement about running one during the other.
- `https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html` — the page about setting Blender up in studios, which is where such a warning would most plausibly sit. It covers bundling extensions and multiple Blender versions co-existing, and says nothing about installing while work is running.
- `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`, the FAQ item "Why is Blender unresponsive during rendering?" — ruled out: it is about graphics cards being unable to render and draw the interface at once, not about installing anything.
- `https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html`, "Lock Interface — Lock interface during rendering in favor of giving more memory to the renderer." — ruled out: it is about memory and the interface, and says nothing about installing an add-on.
- `https://docs.blender.org/api/5.1/info_gotchas_threading.html` — ruled out: it warns that Python threads can crash Blender "while rendering with Cycles", but that is about a script starting threads, not about an add-on being installed. Reading it as an answer to this question would be an over-read.
- Release notes for 5.1: searched every file for `extension` and for `add-on`/`addon`. The only hits are a crash fix, a repository access-token field fix, an interface filter, and macOS device permissions. None touch this.

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

**Not covered.** This does not settle any of: whether a Blender that is rendering notices an add-on being installed by another Blender; whether the repository lock blocks a second Blender; whether a render started before an install uses the add-on files as they were, or as they became; whether a Blender that exits after an install overwrites preferences another Blender wrote. The sources are silent on all of them, so any statement about them would be invention.

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every part of this question was reached with no window at all: the render, the installs, the removals, and an install fired from inside the rendering Blender by a handler. None of it needed a screen. **The windowed half has since been run on the Creator's Mac and agrees with the no-window half:** with a window open, an install fired from inside the rendering Blender returned `{'FINISHED'}`, every frame was written, and every frame matched its control picture for picture — with the interface lock off and on alike. What the sources say follows. The sources do not say. What they do say is that `--command` "implies --background mode", so an install driven from outside the interface happens in a separate Blender started without a window. Whether that separate process affects a render running elsewhere is exactly what is not documented. **Brought up to date 2026-08-07.** The interface-lock half has now been rebuilt with no window at all, with `use_lock_interface` read back at every step and from inside the render handler at the very frame the install fires — see the observed section on the rebuilt lock test above. With the lock genuinely on, the install returned `{'FINISHED'}`, all eight frames were written, and every frame matched its control picture for picture, in all four versions. Two further things came out of that read-back and bear on this line: `bpy.context.window_manager.is_interface_locked` and `bpy.app.is_job_running('RENDER')` both read `False` at every moment, including mid-render with the lock on, so neither gives a program outside the interface a view of the scene setting or of the running render. **The windowed form of the same test is still waiting on the Creator's Mac and stays sent back.**

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The sources do not say. Note only that the directories an install writes into differ by operating system (Linux, macOS and Windows each have their own user directory path, listed at <https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html>), so any file-level answer would have to be checked per system. **Brought up to date 2026-08-07.** The rebuilt interface-lock test was run on the rented Linux machine only, in all four versions, and came out as the Linux runs above. The windowed form of it needs the Creator's Mac and has not been run again. Nothing here changes what stays open for Windows.

---

## 13. Once an add-on is installed and switched on, what more can be learned about whether it will work?

This question is ambiguous. I have answered the reading I think is meant: what can be found out, after enabling, about whether the add-on has loaded and is functional. The other reading is whether the add-on will produce the result the artist wants in a particular scene, and on that the sources say nothing at all.

**Label: documented**, for each of the channels below. Where I have gone a step beyond the words, I say so.

**a) Enabling runs the add-on's own start-up code, and errors from it go to the console.**

> "When the add-on is enabled, Blender executes the code and runs the register() function.
> When the add-on is disabled, Blender runs the unregister() function."

Source: manual, Add-on Tutorial, <https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html>. Read 2026-07-30.

> "Tip
>
> If the Add-on does not activate when enabled,
> check the Console window
> for any errors that may have occurred."

Source: manual, Preferences → Add-ons, <https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html>. The same tip appears at <https://docs.blender.org/manual/en/5.1/editors/preferences/extensions.html> and at <https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html>. Read 2026-07-30.

What the console is:

> "The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from."

Source: manual, Using Blender From The Command Line, <https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html>. Read 2026-07-30.

> "The Blender Console Window can display many different types of status and error messages. Some messages simply
> inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can
> indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender
> non-responsive or shut down completely. The Blender Console Window messages can also originate internally from
> within the Blender code or from external sources such as Python scripts."

Source: manual, Troubleshooting → Startup, <https://docs.blender.org/manual/en/5.1/troubleshooting/startup.html>. Read 2026-07-30.

**b) A re-scan can be asked for, and it reports problems as warnings.**

> "Refresh Local
>
> Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting).
> Any issues are reported as warnings."

Source: manual, Preferences → Add-ons, <https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html>. Read 2026-07-30.

The same thing exists as an operator:

> "bpy.ops.extensions.repo_refresh_all(*, use_active_only=False)
>
> Refresh extension & legacy add-ons, reloading modules & meta-data (similar to restarting)"

Source: Python API, Extensions Operators, <https://docs.blender.org/api/5.1/bpy.ops.extensions.html>. Read 2026-07-30.

**c) The add-on's own information panel.**

> "You can click the arrow at the left of the add-on box to see more information,
> such as its location, a description and a link to the documentation.
> Here you can also find a button to report a bug specific of this add-on."

Source: manual, Preferences → Add-ons, <https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html>. Read 2026-07-30.

I opened the picture named on that page, `_images/editors_preferences_section_addons.webp` ("Blender Preferences Add-ons section"). Label for what follows: **documented**, since the picture is part of the manual. Visible in it: a search field, an "Enabled Only" filter, a tag icon and a menu arrow, then one row per add-on. Each row has an expand arrow, a tick box, the add-on's name, and an icon at the right. Rows whose box is ticked are drawn bright; rows whose box is unticked are drawn dim. No error mark, no version number and no status text is visible on any row in this picture. That is what the picture shows; whether such a mark can appear in some other state is not something the picture can tell me.

**d) The add-on may have preferences of its own that must be set before it does anything.**

> "Some add-ons may have their own preferences which can be found
> in the Preferences section of the add-on information box.
>
> Some add-ons use this section for example to enable/disable
> certain functions of the add-on. Sometimes these might even all default to off.
> So it is important to check if the enabled add-on has any particular preferences."

Source: manual, Preferences → Add-ons, <https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html>. Read 2026-07-30.

**e) Bundled third-party modules can be checked by importing them.**

> "Running
>
> Once the extension has been installed you can check the module is being loaded by importing it in the
> Python console and printing it's location:
>
> import PIL
> print(PIL.__file__)"

Source: manual, Python Wheels, <https://docs.blender.org/manual/en/5.1/advanced/extensions/python_wheels.html>. Read 2026-07-30.

**f) Compiled parts are a known failure point, and which files are present can be checked.**

> "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular
> Python scripts, these are not portable between different platforms.
>
> It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system).
>
> If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating
> system."

Source: manual, Troubleshooting → Python Errors, <https://docs.blender.org/manual/en/5.1/troubleshooting/python.html>. Read 2026-07-30.

**g) If the add-on needs the internet, whether it is allowed to have it can be read.**

> "If the add-on needs to use internet, it must check for the (read-only) property bpy.app.online_access.
> This option is controlled by Preferences, which can be overriding via command-line
> (--offline-mode / --online-mode).
>
> For better error messages, you can check also for bpy.app.online_access_override,
> to determine whether users can turn Allow Online Access
> on the preferences, or not."

Source: manual, Extensions → Add-ons, <https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html>. Read 2026-07-30.

> "bpy.app.online_access
>
> Boolean, true when internet access is allowed by Blender & 3rd party scripts (read-only)."

> "bpy.app.online_access_override
>
> Boolean, true when internet access preference is overridden by the command line (read-only)."

Source: Python API, Application Data, <https://docs.blender.org/api/5.1/bpy.app.html>. Read 2026-07-30.

With the caveat, stated twice in the sources:

> "Add-ons that follow this setting will only connect to the internet if enabled.
> However, Blender cannot prevent third-party add-ons from violating this rule."

Source: manual, Preferences → System, <https://docs.blender.org/manual/en/5.1/editors/preferences/system.html>, and Deploying Blender in Production, <https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html>. Read 2026-07-30.

**h) The set of enabled add-ons can be read from Python.**

> "addons
>
> (default None, readonly)
>
> Type:
>
> Addons[Addon]"

Source: Python API, Preferences, <https://docs.blender.org/api/5.1/bpy.types.Preferences.html>. Read 2026-07-30.

> "class bpy.types.Addon(bpy_struct)
>
> Python add-ons to be loaded automatically
>
> module
>
> Module name (default "", never None)
>
> preferences
>
> (readonly)
>
> Type:
>
> AddonPreferences"

Source: Python API, Addon, <https://docs.blender.org/api/5.1/bpy.types.Addon.html>. Read 2026-07-30.

The documented use of that collection is to reach an add-on's own preferences:

> "addon_prefs = preferences.addons[__name__].preferences"

Source: Python API, AddonPreferences, <https://docs.blender.org/api/5.1/bpy.types.AddonPreferences.html>. Read 2026-07-30.

**i) The installed packages can be listed from outside.**

> "Subcommand: list
>
> usage:
>
> blender --command extension list [-h] [-s]
>
> List packages from all enabled repositories."

Source: manual, Extensions Command Line Arguments, <https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html>. Read 2026-07-30.

**j) A package's declared meta-data can be checked against Blender's rules.**

> "Subcommand: validate
>
> usage:
>
> blender --command extension validate [-h]
> [--valid-tags VALID_TAGS_JSON]
> [SOURCE_PATH]
>
> Validate the package meta-data in the current directory."

> "SOURCE_PATH:
>
> The package source path (either directory containing package files or the package archive).
> This path must containing a blender_manifest.toml manifest."

Source: manual, Extensions Command Line Arguments, <https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html>. Read 2026-07-30.

The documented purpose of `validate` is checking meta-data, not checking that a working add-on works. Note it takes a source path or an archive; the sources do not say it is meant to be pointed at an already-installed add-on.

**k) Observed, from Cycles' own files.** Cycles is shipped as an add-on, so its files show what "switched on" concretely does.

> ```
> bl_info = {
>     "name": "Cycles Render Engine",
>     "author": "",
>     "blender": (2, 80, 0),
>     "description": "Cycles renderer integration",
>     "warning": "",
>     "doc_url": "https://docs.blender.org/manual/en/latest/render/cycles/",
>     "tracker_url": "",
>     "support": 'OFFICIAL',
>     "category": "Render"}
> ```

File: `cycles_5.1/__init__.py`, lines 7–16. Read 2026-07-30.

> ```
> def register():
>     from bpy.utils import register_class
>     from . import ui
>     from . import operators
>     from . import properties
>     from . import presets
>     import atexit
>
>     # Make sure we only registered the callback once.
>     atexit.unregister(engine_exit)
>     atexit.register(engine_exit)
>
>     engine.init()
> ```

File: `cycles_5.1/__init__.py`, lines 133–145. Read 2026-07-30.

> ```
> def init():
>     import bpy
>     import _cycles
>     import os.path
>
>     path = os.path.dirname(__file__)
>     user_path = os.path.dirname(os.path.abspath(bpy.utils.user_resource('CONFIG', path='')))
>
>     _cycles.init(path, user_path, bpy.app.background)
>     _parse_command_line()
> ```

File: `cycles_5.1/engine.py`, lines 44–53. Read 2026-07-30.

**Inferred**, from those three quotes together with the documented statement that enabling runs `register()`: switching this add-on on causes an import of a compiled module (`_cycles`), a read of the user configuration directory, and the registration of classes. So the things that can go wrong at enable time are the ordinary Python ones — an import that fails, a path that is not there, a class that will not register — which is why the manual sends you to the console. The step I took: from "enabling runs register()" plus "this register() imports `_cycles` and calls `engine.init()`" to "an import failure at enable time is possible and would surface as a Python error". A reader has to supply the last link, so this is inferred, not documented.

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

**Not covered.** None of this settles: whether Blender keeps a per-add-on flag saying "this one failed", readable from Python — I found no such property documented; what the warnings from Refresh Local look like or where they appear; whether an add-on that enables cleanly will still work when a `.blend` file is loaded or when a render starts; whether the tick box in the list changes appearance for an add-on that failed. Two operators that look as though they bear on error reporting, `bpy.ops.extensions.status_clear()` and `bpy.ops.extensions.status_clear_errors()`, are both given in the API documentation as "Undocumented, consider contributing." (<https://docs.blender.org/api/5.1/bpy.ops.extensions.html>, read 2026-07-30), so they name an error status without describing it.

**Where I looked.** Whole-tree searches of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` for: `addon_utils`, `module_bl_info`, `disabled_by_error`, `bl_info`, `extension_path_user`, `online_access`, `permission`, `incompatible`, `not compatible`, `unsupported version`, `outdated`, `Console Window`, `Toggle System Console`, `status bar`, `restart`. Opened in full: the two Preferences pages for Add-ons and Get Extensions; Extensions → Add-ons, Getting Started, Python Wheels, Version Number Guidelines, Index; the Add-on Tutorial; Troubleshooting → Python Errors and Startup; Using Blender From The Command Line and both argument pages; Deploying Blender in Production; the API pages for `bpy.ops.preferences`, `bpy.ops.extensions`, `bpy.types.Addon`, `bpy.types.Addons`, `bpy.types.AddonPreferences`, `bpy.types.UserExtensionRepo`, `bpy.app`, `bpy.utils`; and Cycles' `__init__.py` and `engine.py`. Ruled out: `bpy.types.WORKSPACE_UL_addons_items` and `bpy.types.USERPREF_UL_extension_repos` (list-drawing types, no status information); the Info editor and Python Console editor pages (general editors, nothing add-on-specific).

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every Python and command-line check named in this answer was run in a Blender started with no window and returned rather than raising, including `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()` and `bpy.ops.preferences.addon_expand()`, which the sources leave unstated. **The parts that are panels in the Preferences editor have since been reached on the Creator's Mac, which has a screen** — see the Mac observed section above. A declared warning is shown on the row and inside it; an add-on that failed to switch on is not marked at all. What the sources say follows. Most of the above holds; some of it does not.

Holds outside: the console is where errors go, and "Blender output is shown in the Console Window it is started from" (<https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html>). `blender --command extension list` and `validate` are documented command-line sub-commands, and `--command` "implies --background mode" (<https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html>). Add-ons can be switched on at launch with `--addons <addon(s)>`, "Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons" (same page). `bpy.app.online_access` and `preferences.addons` are ordinary Python properties.

Does not hold outside: the Refresh Local button, the add-on information box, the bug-report button and the preferences panel are all parts of the Preferences editor, which is interface. The sources do not say whether their equivalents can be reached without a window — `bpy.ops.extensions.repo_refresh_all` exists as an operator, but nothing states it runs in background mode.

One documented caution about background mode from the tutorial's example add-on:

> "# Note that in background mode (no GUI available), keyconfigs are not available either,
> # so we have to check this to avoid nasty errors in background case."

Source: manual, Add-on Tutorial, <https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html>. Read 2026-07-30. So an add-on can enable cleanly in one mode and hit an error in the other; checking it in the interface does not prove it in background, and the reverse.

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. Mostly the same everywhere, with these documented differences. The console has to be brought up differently: on Windows, "The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." (<https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html>, read 2026-07-30). The compiled-library check names different file kinds per system — `.pyd` on Windows, `.so` elsewhere — and Troubleshooting → Python Errors carries a Windows-only section on mixed Python libraries. A macOS-specific change in 5.1: "On macOS, scripts and extensions can now access the device camera, microphone and system audio. A system permission dialog will be prompted to the user." (release notes, <https://developer.blender.org/docs/release_notes/5.1/python_api/>, read 2026-07-30) — so on macOS an add-on that uses those may work only after a system dialog is answered.

---

## 14. Does an add-on set up for one Blender work in another Blender on the same computer, and what decides that?

This question is ambiguous in one way. "Another Blender on the same computer" can mean another Blender *version*, or another *copy* of the same version. I answer the version reading first, because that is what the sources discuss, then say what they give for the second reading.

**Label: documented** for the mechanism; two specific points below are marked inferred.

### It does not carry over by itself, because the folders are separate

> "User Directories
>
> User directories store preferences, startup file, installed extensions,
> presets and more. By default these use the standard configuration folders
> for each operating system.
>
> Linux
>
> $HOME/.config/blender/5.1/
>
> If the $XDG_CONFIG_HOME environment variable is set:
>
> $XDG_CONFIG_HOME/blender/5.1/
>
> macOS
>
> /Users/$USER/Library/Application Support/Blender/5.1/
>
> Windows
>
> %USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.1\"

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

Source: manual, Blender's Directory Layout, <https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html>. Read 2026-07-30.

The version number is part of the path. And the manual says outright why that matters:

> "The preferences need to be imported from previous versions because the configuration files of each Blender version are
> stored in separate folders. Refer to the Blender's Directory Layout page for the location of these
> folders."

> "Import Preferences From Previous Version
>
> Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and
> startup files from the previous version of Blender and load them. This will include installed add-ons and extensions."

Source: manual, Defaults, <https://docs.blender.org/manual/en/5.1/getting_started/configuration/defaults.html>. Read 2026-07-30.

So the documented answer to the first half is: an add-on set up for one version is not automatically set up for another; it is copied over, once, by an explicit import step offered when the new version first starts.

And the sources warn that the copy may not work:

> "Note
>
> Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this
> option may lead to errors on startup. If this occurs, the recommended first step is to try Loading Factory
> Settings."

Source: manual, Defaults, <https://docs.blender.org/manual/en/5.1/getting_started/configuration/defaults.html>. Read 2026-07-30.

Several Blenders being on one machine at once is expected:

> "Blender downloads can be extracted to any directory on the system, as
> a self contained installation. Multiple Blender versions can
> co-exist on the same system, and deployment can be automated using
> standard file management tools."

Source: manual, Deploying Blender in Production, <https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html>. Read 2026-07-30.

### What decides whether it works once it is there

**1. The version range the add-on declares.** For an extension, in its manifest:

> "blender_version_min:
>
> Minimum supported Blender version - use at least 4.2.0."

> "blender_version_max:
>
> Blender version that the extension does not support, earlier versions are supported."

`blender_version_min` is listed under "Required values", `blender_version_max` under "Optional values".

Source: manual, How to Create Extensions, <https://docs.blender.org/manual/en/5.1/advanced/extensions/getting_started.html>. Read 2026-07-30.

The same two fields can be attached to a download link from a self-hosted repository (manual, Creating a Static Extensions Repository, <https://docs.blender.org/manual/en/5.1/advanced/extensions/creating_repository/static_repository.html>, read 2026-07-30).

For a legacy add-on the same idea sits in `bl_info`, and here the manual says what Blender does about it:

> "bl_info
>
> is a dictionary containing add-on metadata such as the title,
> version and author to be displayed in the Preferences add-on list.
> It also specifies the minimum Blender version required to run the script;
> older versions won't display the add-on in the list."

Source: manual, Add-on Tutorial, <https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html>. Read 2026-07-30.

**Observed**, in Cycles' own files, of what such a declaration looks like in practice:

> ```
> bl_info = {
>     "name": "Cycles Render Engine",
>     "author": "",
>     "blender": (2, 80, 0),
> ```

File: `cycles_5.1/__init__.py`, lines 7–10. Read 2026-07-30.

**2. Changes in Blender itself between the two versions.**

> "New Blender versions can sometimes introduce breaking changes in Blender's
> Python API or even change how entire features work. If this affects your
> extension, you may want to maintain two "tracks" of your extension concurrently:
> one for Blender "old" and one for Blender "new"."

> "Note
>
> Make sure to correctly indicate the Blender versions that each version of
> your extension is compatible with in their manifest file. You can also update
> Blender version compatibility of already-uploaded versions of your add-on from
> the extensions website."

Source: manual, Version Number Guidelines, <https://docs.blender.org/manual/en/5.1/advanced/extensions/version_number_guidelines.html>. Read 2026-07-30.

The 5.1 release notes are an explicit statement of such changes for this version. Under Compatibility:

> "Libraries have been upgraded to match [VFX platform 2026](https://vfxplatform.com/), including:
>   * Python 3.13"

Source: release notes, <https://developer.blender.org/docs/release_notes/5.1/>. Read 2026-07-30.

> "## Python 3.13
>
> Python has been upgraded to [version 3.13](https://www.python.org/downloads/release/python-3130/), matching the [VFX platform 2026](https://vfxplatform.com/)."

Source: release notes, <https://developer.blender.org/docs/release_notes/5.1/python_api/>. Read 2026-07-30. The same page lists removed and renamed API items, for instance "The `sculpt.sample_color` operator has been removed" and a set of Video Sequencer property renames.

**3. The Python that each Blender carries, and anything compiled against it.**

> "Python Version
>
> Blender and the bpy module are only compatible
> with a single Python version. This makes it possible for add-ons and VFX software
> in general to only have to target a single Python version.
>
> Blender bundles a complete Python installation and does not interact with the
> system Python by default."

Source: manual, Deploying Blender in Production, <https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html>. Read 2026-07-30.

> "It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system)."

Source: manual, Troubleshooting → Python Errors, <https://docs.blender.org/manual/en/5.1/troubleshooting/python.html>. Read 2026-07-30.

Bundled wheels are named for the Python version they were built for:

> ```
> wheels = [
> "./wheels/pillow-12.1.0-cp313-cp313-macosx_11_0_arm64.whl",
> "./wheels/pillow-12.1.0-cp313-cp313-manylinux_2_27_x86_64.manylinux_2_28_x86_64.whl",
> "./wheels/pillow-12.1.0-cp313-cp313-win_amd64.whl",
> ]
> ```

and the download example pins it:

> "pip download pillow --dest ./wheels --only-binary=:all: --python-version=3.13 --platform=macosx_11_0_arm64"

Source: manual, Python Wheels, <https://docs.blender.org/manual/en/5.1/advanced/extensions/python_wheels.html>. Read 2026-07-30.

**Inferred**, from those three quotes together: an add-on whose bundled wheel was built for one Python version will not have a matching wheel for a Blender that carries a different one. The step a reader must supply is that `cp313` in a wheel name means the Python version, which the manual shows only by pairing `--python-version=3.13` with the `cp313` filename rather than stating. Because that step is mine and not the source's, this is inferred, not documented.

**4. Where the add-on was put, and whether the other Blender is told to look there.** These are the documented ways to make one set of add-ons visible to more than one Blender:

> "The BLENDER_USER_RESOURCES environment variable
> can be set to a custom directory to replace the default user directory."

> "BLENDER_SYSTEM_SCRIPTS and BLENDER_SYSTEM_EXTENSIONS
> environment variables
> can be used to bundle additional scripts and extensions,
> that are not part of the regular Blender installation."

Source: manual, Blender's Directory Layout, <https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html>. Read 2026-07-30.

> "BLENDER_USER_EXTENSIONS:
>
> Directory for user extensions."

> "BLENDER_SYSTEM_EXTENSIONS:
>
> Directory for system extensions repository."

Source: manual, Command Line Arguments, <https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html>. Read 2026-07-30. The same page lists `--env-system-scripts` and `--env-system-extensions` as arguments that set those variables.

> "The $BLENDER_SYSTEM_EXTENSIONS
> environment variable
> controls the default location. This should point to a directory, within
> which a system directory should exist.
>
> Extensions packages should be extracted in this system directory,
> with a resulting path like this:
>
> $BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml"

Source: manual, Deploying Blender in Production, <https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html>. Read 2026-07-30.

And per repository, in the preferences:

> "use_custom_directory
>
> Manually set the path for extensions to be stored. When disabled a user's extensions directory is created. (default False)"

> "custom_directory
>
> The local directory containing extensions (default "", never None)"

Source: Python API, UserExtensionRepo, <https://docs.blender.org/api/5.1/bpy.types.UserExtensionRepo.html>. Read 2026-07-30.

**5. Portable installations tie the add-ons to the installation instead.**

> "When running Blender from a portable drive, it's possible to keep the configuration
> files on the same drive to take with you.
>
> To enable this, create a folder named portable at the following locations:
>
> Windows: Next to the Blender executable, in the unzipped folder
>
> Linux: Next to the Blender executable, in the unzipped folder
>
> macOS: Inside the application bundle at Blender.app/Contents/Resources
>
> This folder will then store preferences, startup file, installed extensions
> and presets."

Source: manual, Blender's Directory Layout, <https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html>. Read 2026-07-30.

**6. The name an extension is known by depends on the repository it came from.**

> "The legacy add-ons would use their module name to access the preferences. This could lead to a name clash when
> extensions with the same name (from different repositories) would be installed.
> To prevent this conflict, the repository name is now part of the namespace.
>
> For example, now instead of kitsu the module name would be bl_ext.{repository_module_name}.kitsu instead."

> "Note
>
> The value of __package__ will vary between systems so it should never be replaced with a literal string."

Source: manual, Extensions → Add-ons, <https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html>. Read 2026-07-30.

### The second reading: two copies of the same version

The sources give the user directory path with the version number in it (`.../Blender/5.1/`) and never key it to a particular installation. **Inferred**, from that path alone: two copies of Blender 5.1 on one computer would read the same user directory, and so would see the same installed add-ons, unless one of them is a portable installation or is pointed elsewhere by `BLENDER_USER_RESOURCES` or `BLENDER_USER_EXTENSIONS`. The step a reader must supply is that nothing else distinguishes the two copies in that path. The sources do not state this outright, so it is inferred, not documented.

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

**Not covered.** This does not settle: whether Blender 5.1 refuses to *enable* an extension whose `blender_version_min` is above 5.1, or merely hides it — the only statement I found about hiding is the legacy `bl_info` one, and it is about older Blenders and the add-on list, not about extensions and the enable step. It does not settle what a Blender older than an extension's `blender_version_min` does with that extension if the folder is shared by environment variable rather than copied. It does not settle whether `blender_version_max` is enforced by Blender or only used by the extensions platform when offering downloads. It does not settle what happens when the import-from-previous-version step is used to go backwards, from a newer version to an older one. And it does not settle whether a shared extensions directory used by two Blenders at once is safe.

**Where I looked.** Whole-tree searches of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` for: `blender_version_min`, `blender_version_max`, `version compatib`, `Compatibility`, `previous version`, `incompatible`, `not compatible`, `unsupported version`, `too old`, `outdated`, `BLENDER_USER`, `BLENDER_SYSTEM`, `portable`, `co-exist`, `factory-startup`, `addon_utils`, `bl_info`, `restart`. Opened in full: Blender's Directory Layout; Deploying Blender in Production; Defaults; Preferences → Add-ons, Get Extensions and Introduction; Extensions → Index, Getting Started, Add-ons, Python Wheels, Version Number Guidelines, Creating a Static Extensions Repository; the Add-on Tutorial; Troubleshooting → Python Errors; both command-line argument pages; the API pages for `bpy.ops.preferences`, `bpy.ops.extensions`, `bpy.types.UserExtensionRepo`, `bpy.types.Addon`, `bpy.types.Preferences`, `bpy.app`, `bpy.utils`; release notes index and Python API for 5.1; Cycles' `__init__.py` and `engine.py`. Ruled out: the requirements page as taken on 2026-08-01 (<https://www.blender.org/download/requirements/>) — it never names Blender 5.1, so it does not speak for this version; App Templates and File Paths preference pages (about templates and script directories generally, adding nothing about one Blender versus another beyond the script-directory point already quoted).

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


**Outside the interface.** **Settled by test on Linux, and on the Mac for everything but one item — see the two observed sections above.** All of it holds with no window, including the one part the sources leave interface-bound: `bpy.ops.preferences.copy_prev()` ran under `--background`, returned `{'FINISHED'}`, and carried the older version's installed extension into the newer version's folder. **On the Creator's Mac this does not hold for one item:** `bpy.ops.preferences.copy_prev()` refuses to run there in all four versions, with a window open as well as without one — see the Mac observed section above. What the sources say follows. The mechanics hold, with one gap. Every path lever is available without a window: `BLENDER_USER_RESOURCES`, `BLENDER_USER_EXTENSIONS`, `BLENDER_SYSTEM_SCRIPTS`, `BLENDER_SYSTEM_EXTENSIONS`, the matching `--env-system-*` arguments, the `portable` folder, and the `--command extension` sub-commands, which run with `--command` implying "--background mode" (<https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html>). Add-ons can be switched on at launch with `--addons <addon(s)>`. The gap: the "Import Preferences From Previous Version" step is documented only as part of the start-up screen — "When you start Blender for the first time or update to a new version, the interactive region of the Splash Screen is replaced with a few initial preferences" (<https://docs.blender.org/manual/en/5.1/getting_started/configuration/defaults.html>) — and the operator behind it, `bpy.ops.preferences.copy_prev()`, "Copy settings from previous version" (<https://docs.blender.org/api/5.1/bpy.ops.preferences.html>), is documented with no statement about whether it runs without a window. So the one documented route for carrying add-ons from one version to the next is described only as an interface action. **Brought up to date 2026-08-07.** The one interface-bound item has now been pinned down further on Linux. `bpy.ops.preferences.copy_prev()` is reachable from a script there **only when the version folders sit in Blender's own layout**; when the settings folder is handed over with `BLENDER_USER_RESOURCES` — the way nearly every test in this round hands it over — it refuses in all four versions, `poll()` returning `False`. So on Linux the folder arrangement decides whether this route can be driven from outside the interface at all. See the observed section on the folder arrangements above.

**Operating system.** **Partly settled by test — see the observed section above.** The per-version folder, the shared-folder routes and the `portable` folder have now been confirmed on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with the macOS paths coming out as the sources describe them, including the `portable` folder inside `Blender.app/Contents/Resources`. **The two machines disagree on one point:** `bpy.ops.preferences.copy_prev()` runs on Linux and refuses to run on macOS. Two further macOS-only points: a stand-in `HOME` does not redirect Blender's user folder there, though `BLENDER_USER_RESOURCES` does. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The mechanism is the same everywhere; the paths are not. The user directory, the system directory and the `portable` folder each have a different location on Linux, macOS and Windows, all listed at <https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html>. On Windows there are additionally distinct system paths for the zip, the installer and the Microsoft Store build. And an extension may declare which systems it is for at all: **Brought up to date 2026-08-07.** The two Linux cells were run deliberately and the four cells now stand as a table in the observed section above: on Linux it runs under Blender's own layout and refuses when the settings folder is handed over; on macOS it refuses under both arrangements. The two machines still disagree, and the arrangement does not explain the disagreement away — only the Linux "runs" needed the arrangement named. The macOS cells were not run again in this sitting and are quoted from the Mac record. Windows is unchanged and was still not tried.

> "platforms:
>
> List of supported platforms. If omitted, the extension will be available in all operating systems.
> The available options are
> ["windows-x64", "windows-arm64", "macos-x64", "macos-arm64", "linux-x64"]"

Source: manual, How to Create Extensions, <https://docs.blender.org/manual/en/5.1/advanced/extensions/getting_started.html>. Read 2026-07-30. Note that these names distinguish processor kinds as well as systems, so "the same computer" is not in question here, but a copied add-on directory shared between differently-built Blenders would be.

---

## Things a person at a computer with Blender open could settle by hand

The items on this list have since been run on **both** machines — a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores) and the Creator's own Mac (a MacBook Pro with an M1 Pro chip running macOS 26.5.2) — in all four Blender versions, and their results are written into the answers above as observed findings. They are kept here so those results can be traced back to what was asked. **Nothing on this list is still open.**

The two things those lists did not cover both needed a screen, which that machine does not have. Both have since been run on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — in all four versions, and their results are written into the answers above as observed findings. **Nothing on this list is still open.** The only thing left untested anywhere is Windows, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

Each of these is short. For each I say what a negative result looks like, so the test carries information either way.

**T1 — Does installing an add-on disturb a render already running? (question 12)**

Start a long render from the command line in one Blender: `blender --background scene.blend --render-anim`, with a scene slow enough to run for a few minutes. While it is running, in a second terminal, run `blender --command extension install <some-package-id>` (or `install-file` with a `.zip` on disk). Watch the first terminal.

Negative result: the render carries on, finishes, and writes the same frames it would have written otherwise, with no new messages in its output. Positive result: the render prints an error, stalls, exits early, or the install command reports it could not proceed.

Repeat the same test with the install done through a *third* Blender's Preferences window rather than the command line, and again with the render started from a Blender that has a window open. The three cases can come out differently and each is worth its own run.

**T2 — Does an install by one Blender reach a Blender that is already running? (question 12)**

Open Blender with a window. In a separate terminal, install an add-on with `blender --command extension install <id>`. Back in the first Blender, without restarting it, open Preferences → Add-ons and look for the new add-on. Then press Refresh Local and look again.

Negative result: the add-on is absent both before and after Refresh Local. Positive result: it appears — either straight away, which would say the running Blender re-reads the directory on its own, or only after Refresh Local, which would say the scan is what does it.

**T3 — Is there a per-add-on error mark in the list? (question 13)**

Write a one-file add-on whose `register()` raises an exception on purpose (a single `raise Exception("test")` inside `register`). Install it from disk and tick its box. Look at three places: the row in the Add-ons list, the expanded information box, and the console.

Negative result: the row looks exactly like any other unticked add-on, the information box shows nothing about the failure, and only the console carries the error. Positive result: the row or the box shows a mark, warning or message — which would mean the failure is readable without the console.

**T4 — Can a failed add-on be told from a working one in Python? (question 13)**

With the deliberately failing add-on from T3 installed and its box ticked, run in the Python console: `print([a.module for a in bpy.context.preferences.addons])`, and compare against the same list with a known-good add-on enabled.

Negative result: the failing add-on's module name appears in the list just as the good one does, so the collection cannot tell them apart. Positive result: it is absent, or carries something that distinguishes it.

**T5 — Does Refresh Local's warning say anything useful? (question 13)**

With a deliberately broken add-on present on disk (for instance an extension whose `blender_manifest.toml` is missing a required field such as `license`), press Refresh Local in Preferences → Add-ons and note exactly where the warning appears and what it says.

Negative result: nothing visible appears anywhere and the only sign is in the console. Positive result: a warning is shown in the interface, and its wording can be recorded.

**T6 — Does a per-version user folder really hide an add-on from another version? (question 14)**

Install an add-on in Blender 5.1. Without importing preferences, start a different Blender version on the same computer and open Preferences → Add-ons.

Negative result: the add-on is not listed there — which would confirm the separate-folders account. Positive result: it is listed, which would mean something is shared that the directory-layout page does not describe.

**T7 — Does one folder shared by two Blenders work? (question 14)**

Set `BLENDER_USER_EXTENSIONS` to the same directory for two different Blender versions on the same computer. Install an add-on in one. Start the other and open Preferences → Add-ons.

Negative result: the add-on does not appear, or appears and fails to enable. Positive result: it appears and enables cleanly — which would settle whether a shared extensions directory is a workable way to serve more than one Blender.

**T8 — Is `blender_version_min` enforced at enable time? (question 14)**

Build a trivial extension whose manifest has `blender_version_min = "9.9.0"`. Install it in Blender 5.1 from disk and try to enable it.

Negative result: it installs and enables without complaint, meaning the field is advisory at this point. Positive result: Blender refuses the install, hides the add-on from the list, or refuses to enable it — and the exact wording of the refusal is worth recording.

**T9 — Does a wheel built for another Python stop the add-on? (question 14)**

Build an extension bundling a wheel whose filename carries a Python tag other than the one Blender 5.1 uses (for example a `cp311` wheel). Install and enable it in Blender 5.1, then in the Python console import the bundled module.

Negative result: it installs, enables and imports without trouble, meaning the tag is not decisive. Positive result: the enable step or the import fails, and the console message names the reason.

---

2026-08-06 — Opus 5 (Claude Code) — Add-ons, Blender 5.1

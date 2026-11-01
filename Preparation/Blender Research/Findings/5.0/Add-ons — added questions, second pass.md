# Add-ons — added questions, second pass (Blender 5.0)

Sources consulted, and the date read:

- Blender 5.0 manual (plain-text copy), read 2026-07-30 — pages under `https://docs.blender.org/manual/en/5.0/`, including the pictures in the manual's `_images` folder.
- Blender 5.0 Python API documentation (plain-text copy), read 2026-07-30 — pages under `https://docs.blender.org/api/5.0/`.
- Blender 5.0 release notes (plain files), read 2026-07-30 — pages under `https://developer.blender.org/docs/release_notes/5.0/`.
- Cycles' own files for 5.0, read 2026-07-30 — cited by file and place in the file.
- Blender's requirements page, read 2026-08-01 — `https://www.blender.org/download/requirements/`. Checked; it says nothing bearing on questions 12, 13 or 14, and it is not cited below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: 12, 13 and 14 only. Questions 1–11 on the sheet were left alone.

---

## 12. Does installing an add-on disturb a render already running on the same computer?

**Label: not in the sources.**

**That label answers the question from the sources only. A test has since been run and settled it — see the observed section below in this same answer.**

The sources describe installing add-ons and extensions, and they describe rendering, but nowhere do they say what happens to a render that is already under way when an add-on is installed — neither for a render running inside the same Blender, nor for a render running in a separate Blender on the same computer.

**The question is ambiguous.** I answer the reading I think is meant: a render is already running somewhere on the machine, and an add-on is installed while it runs; does the render suffer. The other reading is narrower — a render running inside the very same Blender in which the install is performed. The sources are silent on both, so the choice of reading does not change the answer.

### What the sources do say near this, and why none of it settles the question

Each of these is a real documented statement, and none of them is an answer. I list them so the silence can be checked rather than taken on trust.

1. Enabling an add-on takes effect at once in the Blender doing the enabling.

   > "The add-on functionality should be immediately available."

   Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, section "Enabling & Disabling Add-ons". Read 2026-07-30.

   This is about the Blender that performs the action. It says nothing about a render.

2. Blender can re-scan installed add-ons without restarting, and reports problems as warnings.

   > "Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting). Any issues are reported as warnings."

   Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html`, "Refresh Local"; the same sentence appears as the description of `bpy.ops.extensions.repo_refresh_all` in the Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.extensions.html`. Read 2026-07-30.

   Again, nothing about a render in flight.

3. Extension repositories carry a file-system lock.

   > "Remove the repository file-system lock"

   (description of `bpy.ops.extensions.repo_unlock`), and

   > "Lock repositories - to test locking"

   (description of `bpy.ops.extensions.repo_lock_all`). Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.extensions.html`. Read 2026-07-30.

   This establishes that a lock exists on the extension repository. The sources do not say what it protects against, who takes it, or whether a rendering Blender ever holds or waits on it.

4. The only documented caution about acting while a render runs is about scene data, not about installing anything.

   > "Lock interface during rendering in favor of giving more memory to the renderer."

   Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html`, Render menu, "Lock Interface". Read 2026-07-30.

   > "Altering data from handlers should be done carefully. While rendering the frame_change_pre and frame_change_post handlers are called from one thread and the viewport updates from a different thread. If the handler changes data that is accessed by the viewport, this can cause a crash of Blender. In such cases, lock the interface (Render → Lock Interface or bpy.types.RenderSettings.use_lock_interface) before starting a render."

   Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.app.handlers.html`, "Note on Altering Data". Read 2026-07-30.

   This is the closest the sources come to "doing something during a render can hurt the render". It is about changing scene data from handlers while rendering. Installing an add-on is not mentioned, and extending this quote to cover installation would be my invention, not the source's.

5. Installing writes to disk, and preferences are written too.

   > "Now installing the package will extract the wheel into the extensions own site-packages directory."

   Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/python_wheels.html`, "Running". Read 2026-07-30.

   > "By default changes to preferences are saved on exit, this allows changes to the keymap and Quick Favorites menu to be stored and used between Blender Sessions."

   Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/introduction.html`, "Managing Preferences". Read 2026-07-30.

   > "Save preferences on exit when modified (unless factory settings have been loaded)"

   (description of `Preferences.use_preferences_save`). Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Preferences.html`. Read 2026-07-30.

   A third page states the timing differently:

   > "The preferences are automatically saved when changed."

   Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/getting_started/configuration/defaults.html`, "Saving Defaults". Read 2026-07-30.

   So an install touches shared files on disk, and a Blender that is rendering may write the preferences file when it exits. What happens when two Blenders touch these files at once is not stated anywhere I could find.

6. There is a documented way to install without writing preferences at all.

   > "Treat the user-preferences as read-only, preventing updates for operations that would otherwise modify them. This means removing extensions or repositories for example, wont update the user-preferences."

   (the `--no-prefs` option of `blender --command extension install`, `install-file`, `remove`, `repo-add` and `repo-remove`). Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html`. Read 2026-07-30.

   The sources give the option and its effect on the preferences file. They do not connect it to a render.

### Where I looked

Searched the whole of the 5.0 manual, the whole of the 5.0 Python API documentation, the whole of the 5.0 release notes and the Cycles 5.0 files for: "disturb", "interrupt", "interfere", "concurrent", "simultaneously", "at the same time", "another instance", "second instance", "multiple instances", "already running", "single instance", "one instance", "co-exist", "coexist", "side by side", "side-by-side", "while rendering", "during a render", "in progress", "running render", "active render", "render job", "background job", "safe to", "unsafe", "restart", "lock", "installing", "install".

Pages opened and ruled out, with the reason:

- `editors/preferences/addons.html` and `editors/preferences/extensions.html` — describe installing, enabling, refreshing and removing. No statement about a render.
- `advanced/extensions/addons.html`, `advanced/extensions/getting_started.html`, `advanced/extensions/python_wheels.html`, `advanced/extensions/version_number_guidelines.html`, `advanced/extensions/creating_repository/static_repository.html`, `advanced/extensions/creating_repository/dynamic_repository.html` — authoring, packaging and hosting. No statement about a render.
- `advanced/command_line/extension_arguments.html` — every extension subcommand. No statement about a render, and no option that mentions one.
- `advanced/command_line/arguments.html` — every command-line argument, including the background, logging and debug groups. Nothing about installing during a render.
- `advanced/deploying_blender.html` — production deployment, bundling extensions, working offline. Nothing about a render in flight.
- `advanced/blender_directory_layout.html` — which directories are written. Nothing about contention.
- `render/output/animation.html`, `render/output/introduction.html`, `render/output/properties/output.html`, `render/cycles/render_settings/performance.html` — the pages that talk about long renders and about sharing work between machines. The nearest sentence is advice to lower Blender's priority and "work on other things", which is about the computer's load, not about installing an add-on.
- `interface/window_system/topbar.html` and `bpy.app.handlers.html` — the "Lock Interface" material quoted above.
- `bpy.ops.extensions.html`, `bpy.ops.preferences.html`, `bpy.types.UserExtensionRepo.html`, `bpy.types.PreferencesExtensions.html`, `bpy.types.Preferences.html` — every documented install/enable/lock operator and property. None mentions rendering.
- `info_gotchas_threading.html` — warns that Python threads crash Blender, "for example, a crash can occur while rendering with Cycles". This is about Python threads started by a script, not about installing an add-on, so I ruled it out.
- Cycles 5.0 files — `__init__.py`, `engine.py`, `properties.py`, `ui.py`, `operators.py`, `version_update.py` searched for the same terms. Cycles contains nothing about add-on installation; it is an add-on, not the thing that installs them.
- 5.0 release notes, all subject pages — the only add-on and extension entries are about API changes and a build bug ("Duplicate wheels when extension build using \"paths\"…"). Nothing about installing during a render.

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

**Not covered.** Everything the question asks. In particular the sources do not settle: whether a running render is interrupted, slowed, or made to fail; whether the answer differs for a render in the same Blender versus a separate one; whether the extension repository lock can block or be blocked by a rendering Blender; and what happens if two Blenders write the preferences file at once.

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every part of this question was reached with no window at all: the render, the installs, the removals, and an install fired from inside the rendering Blender by a handler. None of it needed a screen. **The windowed half has since been run on the Creator's Mac and agrees with the no-window half:** with a window open, an install fired from inside the rendering Blender returned `{'FINISHED'}`, every frame was written, and every frame matched its control picture for picture — with the interface lock off and on alike. What the sources say follows. The sources do not say. The command-line extension subcommands are documented, and `--command` "implies --background mode" (`https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, read 2026-07-30), so installing without the window is documented — but the effect on a running render is unstated there as everywhere else. **Brought up to date 2026-08-07.** The interface-lock half has now been rebuilt with no window at all, with `use_lock_interface` read back at every step and from inside the render handler at the very frame the install fires — see the observed section on the rebuilt lock test above. With the lock genuinely on, the install returned `{'FINISHED'}`, all eight frames were written, and every frame matched its control picture for picture, in all four versions. Two further things came out of that read-back and bear on this line: `bpy.context.window_manager.is_interface_locked` and `bpy.app.is_job_running('RENDER')` both read `False` at every moment, including mid-render with the lock on, so neither gives a program outside the interface a view of the scene setting or of the running render. **The windowed form of the same test is still waiting on the Creator's Mac and stays sent back.**

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The sources do not say. I found no operating-system-specific statement about installing while a render runs. **Brought up to date 2026-08-07.** The rebuilt interface-lock test was run on the rented Linux machine only, in all four versions, and came out as the Linux runs above. The windowed form of it needs the Creator's Mac and has not been run again. Nothing here changes what stays open for Windows.

---

## 13. Once an add-on is installed and switched on, what more can be learned about whether it will work?

**Label: documented**, for the specific checks listed below. Two parts of the question are **not in the sources** and are marked as such at the end.

**The question is ambiguous.** "Whether it will work" can mean (a) whether it loaded and registered without error, and whether anything else must be set before it does anything; or (b) whether it will produce the right result when used. I answer reading (a), which is what the sources address. On reading (b) the sources say nothing at all; that is stated under "Not covered".

### What can be learned, each with the words that carry it

**1. Whether it failed while being switched on — read the console.**

> "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred."

Blender 5.0 manual. The same sentence appears on three pages: `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, `https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html` and `https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html`. Read 2026-07-30.

On what that console shows:

> "The Blender Console Window can display many different types of status and error messages. Some messages simply inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender non-responsive or shut down completely. The Blender Console Window messages can also originate internally from within the Blender code or from external sources such as Python scripts."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/troubleshooting/startup.html`. Read 2026-07-30.

**2. Errors and warnings are also logged inside Blender.**

> "The Info editor logs the executed operators as well as errors, warnings, and informational messages."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/editors/info_editor.html`. Read 2026-07-30.

**3. Re-scanning reports problems as warnings.**

> "Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting). Any issues are reported as warnings."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html` ("Refresh Local"), and the description of `bpy.ops.extensions.repo_refresh_all` in the Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.extensions.html`. Read 2026-07-30.

**4. The add-on's own information panel.**

> "You can click the arrow at the left of the add-on box to see more information, such as its location, a description and a link to the documentation. Here you can also find a button to report a bug specific of this add-on."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, "Add-on Information". Read 2026-07-30.

The matching operator:

> "Display information and preferences for this add-on"

(description of `bpy.ops.preferences.addon_expand`). Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.preferences.html`. Read 2026-07-30.

**5. Being switched on is not the same as being ready to work — its own preferences may still be off.**

> "Some add-ons use this section for example to enable/disable certain functions of the add-on. Sometimes these might even all default to off. So it is important to check if the enabled add-on has any particular preferences."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, "Add-on Preferences". Read 2026-07-30.

Those preferences can be read from Python. `bpy.types.Addon` has:

> "module — Module name" and "preferences — Type: AddonPreferences, (readonly)"

and `bpy.types.Preferences` has an `addons` property, "Addons bpy_prop_collection of Addon, (readonly)". Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Addon.html` and `https://docs.blender.org/api/5.0/bpy.types.Preferences.html`. Read 2026-07-30.

**6. Whether its bundled Python modules actually load.**

> "Once the extension has been installed you can check the module is being loaded by importing it in the Python console and printing it's location:"

followed by the example `import PIL` / `print(PIL.__file__)`. Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/python_wheels.html`, "Running". Read 2026-07-30.

The same page explains where those files went:

> "Now installing the package will extract the wheel into the extensions own site-packages directory."

**7. Whether a bundled compiled library can load at all on this machine.**

> "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular Python scripts, these are not portable between different platforms."

> "It is possible the library is incompatible with your Blender installation (attempting to load a library built for a different version of Python, or loading a 32-bit library on a 64-bit system)."

> "If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating system."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/troubleshooting/python.html`. Read 2026-07-30. The same page gives a Windows-specific procedure for finding duplicate Python libraries from a traceback.

**8. Whether it is allowed to reach the internet, if it needs to.**

> "If the add-on needs to use internet, it must check for the (read-only) property bpy.app.online_access. This option is controlled by Preferences, which can be overriding via command-line (--offline-mode / --online-mode)."

> "For better error messages, you can check also for bpy.app.online_access_overriden, to determine whether users can turn Allow Online Access on the preferences, or not."

> "Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html`, "Internet Access". Read 2026-07-30.

**9. What resources it declared it needs — readable from its manifest.**

> "permissions: Add-ons can list which resources they require. The available options are files, network, clipboard, camera, microphone. Each permission should be followed by an explanation (short single-sentence, up to 64 characters, with no end punctuation)."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html`, "Optional values". Read 2026-07-30. The manifest file `blender_manifest.toml` is part of the installed extension, per the same page: "An extension is shared as a .zip archive containing a manifest file and other files."

**10. Its manifest can be validated after the fact.**

> "Validate the package meta-data in the current directory."

with

> "SOURCE_PATH: The package source path (either directory containing package files or the package archive). This path must containing a blender_manifest.toml manifest."

(`blender --command extension validate`). Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html`. Read 2026-07-30.

**11. It can be listed.**

> "List packages from all enabled repositories."

(`blender --command extension list`). Same page and date.

**12. An add-on switched on globally can still be switched off for the workspace in use.**

> "Filter Add-ons — Determines which add-ons are enabled in the active workspace. When unchecked, the global add-ons will be used. When checked, you can enable individual add-ons in the list below."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/interface/window_system/workspaces.html`. Read 2026-07-30. The matching property is `WorkSpace.use_filter_by_owner`, "Filter the UI by tags", Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.WorkSpace.html`, read 2026-07-30.

**13. Messages can be turned up and captured.**

> "--debug-python — Enable debug messages for Python."

> "--log-level <level> — Set the logging verbosity level." with levels "fatal", "error", "warning", "info", "debug", "trace".

> "--log-file <filepath> — Set a file to output the log to."

> "--python-exit-code <code> — Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables."

> "--addons <addon(s)> — Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`. Read 2026-07-30.

**14. Some extensions cannot be switched off at all, which is itself information.**

> "Once an extension is installed it can be disabled (or re-enabled) as part of the user preferences. Some extension types do not support this, and will always be shown as enabled."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html`, "Enable/Disable". Read 2026-07-30.

**15. Observed, from Cycles' own files.** An installed legacy-style add-on carries a metadata block that a reader can inspect directly. Cycles 5.0, file `__init__.py`, lines 7–16:

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

Read 2026-07-30. Observed: the keys `warning`, `doc_url`, `tracker_url` and `support` exist in an installed add-on's own file and can be read there. That an empty `warning` string means "no known problem", or that Blender displays these fields, is a step beyond the file and I do not take it here.

Also observed, same file, in `register()`: the add-on's registration calls `engine.init()`, and `engine.init()` (Cycles 5.0, `engine.py`, function `init`) runs:

```
    _cycles.init(path, user_path, bpy.app.background)
```

Observed: switching this add-on on causes a compiled module `_cycles` to be imported and initialised at that moment. **Inferred**, one step further: if that import or initialisation fails, the failure happens at the moment of switching on and not later — which is why the manual's advice to read the console at that moment is where the information appears. The step I took is from "register() imports and initialises a compiled module" to "a failure surfaces at enable time"; the file does not say what happens on failure.

**16. Which preferences file is in play** is printed at startup:

> "Read prefs: {DIR}/userpref.blend — The preferences use this path."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/troubleshooting/startup.html`, "Common Startup Messages". Read 2026-07-30.

### Where I looked

Searched the whole 5.0 manual and the whole 5.0 Python API documentation for: "addon_utils", "is_enabled", "Console window", "error", "warning", "incompatible", "not supported", "unsupported", "disabled automatically", "fails to", "permission", "validate", "status", "traceback", "install", "installing", "enabled". Opened `editors/preferences/addons.html`, `editors/preferences/extensions.html`, `advanced/extensions/addons.html`, `advanced/extensions/getting_started.html`, `advanced/extensions/python_wheels.html`, `advanced/command_line/extension_arguments.html`, `advanced/command_line/arguments.html`, `advanced/command_line/launch/windows.html`, `advanced/scripting/addon_tutorial.html`, `advanced/deploying_blender.html`, `editors/info_editor.html`, `interface/window_system/workspaces.html`, `troubleshooting/startup.html`, `troubleshooting/python.html`, `troubleshooting/report_bug.html`; and in the API `bpy.ops.preferences.html`, `bpy.ops.extensions.html`, `bpy.ops.script.html`, `bpy.types.Addon.html`, `bpy.types.Addons.html`, `bpy.types.AddonPreferences.html`, `bpy.types.Preferences.html`, `bpy.types.PreferencesExtensions.html`, `bpy.types.UserExtensionRepo.html`, `bpy.types.WorkSpace.html`, `bpy.utils.html`, `bpy.app.html`.

I also looked at the picture the manual shows for the Add-ons preferences section, `_images/editors_preferences_section_addons.webp` (on `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`, read 2026-07-30). **Documented**, from the picture: the list shows one row per add-on with a tick box and an expand arrow, a search field, an "Enabled Only" filter and a tags button; some rows are ticked and some are not. Nothing in that picture shows an error, warning or health indicator on a row. I do not read anything further into it.

Ruled out: `addon_utils` is used in two manual pages (`advanced/deploying_blender.html` shows `addon_utils.enable("my-addon")`, `advanced/scripting/addon_tutorial.html` shows `addon_utils.paths()`), but the module has no page of its own anywhere in the 5.0 Python API documentation, so I cannot say what else it offers.

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

**Not covered.**

- Whether the add-on will produce a correct or useful result when actually used. Nothing in the sources addresses that.
- Whether Blender keeps a per-add-on error or status that a script can read. `bpy.ops.extensions.status_clear` and `bpy.ops.extensions.status_clear_errors` exist and each is described only as "Undocumented, consider contributing." (Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.extensions.html`, read 2026-07-30), so a status of some kind exists but the sources do not say what it holds or how to read it.
- What `blender --command extension list` actually prints — whether it shows enabled state or errors. The sources give only the one-line description quoted above.
- Whether a failed enable is reported with the add-on's name, or only as a Python traceback.

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every Python and command-line check named in this answer was run in a Blender started with no window and returned rather than raising, including `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()` and `bpy.ops.preferences.addon_expand()`, which the sources leave unstated. **The parts that are panels in the Preferences editor have since been reached on the Creator's Mac, which has a screen** — see the Mac observed section above. A declared warning is shown on the row and inside it; an add-on that failed to switch on is not marked at all. What the sources say follows. Most of this holds. The console messages, `--debug-python`, `--log`, `--log-level`, `--log-file`, `--python-exit-code`, `--addons`, and `blender --command extension list` and `validate` are all command-line things; `--command` "implies --background mode" (`https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, read 2026-07-30). Reading `bpy.context.preferences.addons` and importing a bundled module can be done from a script. What does **not** carry over: the Info editor, the add-on information panel, the Add-on Preferences panel, the "Refresh Local" button, and the workspace add-on filter are all parts of Blender's window. The sources give a Python operator for the refresh (`bpy.ops.extensions.repo_refresh_all`) but do not say whether it works without the window.

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. Mostly the same everywhere, with two documented differences. First, the console:

> "When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html`, read 2026-07-30 — so the "check the Console window" advice means a Windows-only menu item there, and elsewhere means starting Blender from a terminal:

> "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window. To ensure you can see any error messages that appear at launch, open a command-line window and navigate to the location where the Blender executable is, and run blender from that location. The specifics of this will vary by operating system."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/troubleshooting/startup.html`, read 2026-07-30.

Second, the duplicate-library diagnosis on `troubleshooting/python.html` is given under a "Windows" heading and its script is Windows-only.

---

## 14. Does an add-on set up for one Blender work in another Blender on the same computer, and what decides that?

**Label: documented**, with two points below explicitly marked **inferred**.

**Short answer.** By default, no. Add-ons and extensions are set up inside a directory that is named for the Blender version, so a Blender of a different version does not see them at all. What decides it is, in order: which user and system directories the second Blender reads; and then, if it does see the add-on, whether the add-on declares itself compatible with that Blender version and whether the Blender API it relies on still behaves the same way.

### Several Blenders on one computer is a documented, supported situation

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html`, "Installing Blender". Read 2026-07-30.

Per operating system:

> "Note, you do not have to overwrite your existing Blender installation. It's perfectly possible to have multiple versions installed side by side."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html`. Read 2026-07-30.

> "You can rename Blender.app or place it in a different folder to have more than one version at a time."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/getting_started/installing/macos.html`. Read 2026-07-30.

> "When using this method of installation, it is possible to have multiple versions of Blender installed."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html`. Read 2026-07-30.

### What is set up is stored per version, which is what makes the default answer "no"

> "User directories store preferences, startup file, installed extensions, presets and more. By default these use the standard configuration folders for each operating system."

with the paths for this version:

> "Linux — $HOME/.config/blender/5.0/ … If the $XDG_CONFIG_HOME environment variable is set: $XDG_CONFIG_HOME/blender/5.0/"
>
> "macOS — /Users/$USER/Library/Application Support/Blender/5.0/"
>
> "Windows — %USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.0\"

and, in the path layout:

> "./extensions — Extension repositories. Located in both user and system directories. Repositories are loaded from both directories."
>
> "./scripts — Add-ons, presets, templates, user interface, startup scripts. Located in both user and system directories. Scripts are loaded from both directories."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html`. Read 2026-07-30.

Said outright, on the page a person meets when starting a new version for the first time:

> "Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and startup files from the previous version of Blender and load them. This will include installed add-ons and extensions."
>
> "The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders. Refer to the Blender's Directory Layout page for the location of these folders."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/getting_started/configuration/defaults.html`, "Import Preferences From Previous Version". Read 2026-07-30.

That is the answer to the first half: another Blender version does not use the first one's add-ons; they are copied across only if the person asks for it.

**Inferred (one step).** How coarse is "version"? The Python API gives:

> "bpy.utils.resource_path(type, *, major=bpy.app.version[0], minor=bpy.app.version[1])" — "Return the base path for storing system files." with parameters "major (int) – major version, defaults to current" and "minor (str) – minor version, defaults to current".

Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.utils.html`. Read 2026-07-30. The path is a function of major and minor only, and the documented directory is `5.0`. **Inferred:** two Blenders that differ only in the third number share one user directory and therefore one set of installed add-ons, while 5.0 and 5.1 do not. The step is from "the path takes only major and minor" to "patch releases share"; no source states that in so many words.

### Ways to make two Blenders share the same add-ons anyway

All of these are documented; each is a lever, not a default.

> "The BLENDER_USER_RESOURCES environment variable can be set to a custom directory to replace the default user directory."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html`. Read 2026-07-30. The full list of variables:

> "BLENDER_USER_RESOURCES: Replace default directory of all user files. Other BLENDER_USER_* variables override when set."
>
> "BLENDER_USER_SCRIPTS: Directory for user scripts."
>
> "BLENDER_USER_EXTENSIONS: Directory for user extensions."
>
> "BLENDER_SYSTEM_SCRIPTS: Directories to add extra scripts."
>
> "BLENDER_SYSTEM_EXTENSIONS: Directory for system extensions repository."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, "Environment Variables". Read 2026-07-30. The same page gives `--env-system-scripts` and `--env-system-extensions` as command-line ways to set the system ones.

The system extensions route is spelled out for exactly this kind of setup:

> "When working offline or in a more controlled environment, it may be useful to provide a set of extensions to all users. For this there is a default read-only System repository. This repository could be located on a read-only network drive or in a system directory."
>
> "The $BLENDER_SYSTEM_EXTENSIONS environment variable controls the default location. This should point to a directory, within which a system directory should exist."
>
> "Extensions packages should be extracted in this system directory, with a resulting path like this: $BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml"

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html`, "Bundling Extensions". Read 2026-07-30.

**Inferred (one step).** The path given for the system extensions directory carries no version number, unlike the user directory paths. **Inferred:** two Blenders of different versions pointed at the same `BLENDER_SYSTEM_EXTENSIONS` would both load the same extensions from it. The step is from "the documented path has no version component" to "the directory is shared across versions"; no source says so.

A repository can also be pinned to a directory of your choosing:

> "use_custom_directory — Manually set the path for extensions to be stored. When disabled a user's extensions directory is created."

(a property of `UserExtensionRepo`). Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.UserExtensionRepo.html`. Read 2026-07-30. And the same repository has:

> "source — Select if the repository is in a user managed or system provided directory. USER: User – Repository managed by the user, stored in user directories. SYSTEM: System – Read-only repository provided by the system."

Same page and date. The manual adds that system repositories exist to travel with a Blender:

> "These repositories are intended to bundle extensions with Blender, to make it portable."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html`, "Local Repository". Read 2026-07-30.

**Documented, from a picture.** The manual's picture of the repositories list, `_images/editor_preferences_section_extensions_repositories.png` on `https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html` (read 2026-07-30), shows three entries, each with a tick box, all ticked: "extensions.blender.org" with a globe icon, "User Default", and "System". That matches the text: "By default Blender has a Remote Repository pointing towards the Official Blender Extensions Platform and two Local Repositories."

For legacy add-ons there is the script-directory route:

> "Add a Script Directories entry pointing to your script folder (e.g. my_scripts). Save the preferences and restart Blender for it to recognize the new add-on location."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html`. Read 2026-07-30.

And the portable route, which ties the setup to the installation rather than to the user:

> "When running Blender from a portable drive, it's possible to keep the configuration files on the same drive to take with you. To enable this, create a folder named portable at the following locations: Windows: Next to the Blender executable, in the unzipped folder. Linux: Next to the Blender executable, in the unzipped folder. macOS: Inside the application bundle at Blender.app/Contents/Resources. This folder will then store preferences, startup file, installed extensions and presets."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html`, "Portable Installation". Read 2026-07-30.

### If the other Blender does see the add-on, what then decides whether it works

**a. What the add-on declares about Blender versions.** For an extension, in its manifest:

> "blender_version_min: Minimum supported Blender version - use at least 4.2.0."
>
> "blender_version_max: Blender version that the extension does not support, earlier versions are supported."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html`, "Required values" and "Optional values". Read 2026-07-30. `blender_version_min` is required; `blender_version_max` is optional.

For a legacy add-on, in its `bl_info`:

> "is a dictionary containing add-on metadata such as the title, version and author to be displayed in the Preferences add-on list. It also specifies the minimum Blender version required to run the script; older versions won't display the add-on in the list."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/scripting/addon_tutorial.html`. Read 2026-07-30. That is a plain statement of behaviour: an older Blender does not even list an add-on whose declared minimum is above it.

**Observed**, from Cycles' own files: an installed legacy-style add-on carries that declaration in the file itself. Cycles 5.0, `__init__.py`, line 10:

```
    "blender": (2, 80, 0),
```

Read 2026-07-30.

**b. The version is used when fetching from a repository, too.**

> "When Blender fetches the extensions listing it passes the following arguments to make sure only compatible extensions are listed: platform, blender_version"

with the example query URL `https://extensions.blender.org/api/v1/extensions/?blender_version=4.2.0&platform=linux-x64`. Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/dynamic_repository.html`. Read 2026-07-30.

**c. Whether Blender's own API still behaves as the add-on expects.**

> "New Blender versions can sometimes introduce breaking changes in Blender's Python API or even change how entire features work. If this affects your extension, you may want to maintain two 'tracks' of your extension concurrently: one for Blender 'old' and one for Blender 'new'."

> "Make sure to correctly indicate the Blender versions that each version of your extension is compatible with in their manifest file. You can also update Blender version compatibility of already-uploaded versions of your add-on from the extensions website."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/extensions/version_number_guidelines.html`. Read 2026-07-30.

The 5.0 release notes are explicit that such breaks happened in this version. Blender 5.0 release notes, read 2026-07-30:

> "Properties defined by the bpy.props API are no longer stored in the same container as user-defined Custom Properties. As a consequence, it is no more possible to access them directly through the Python 'dict-like' syntax."

`https://developer.blender.org/docs/release_notes/5.0/python_api/`

> "Blender comes with various bundled Python modules, while these were not documented as part of the public API, scripts may have imported them. The following modules have been made private and should not be used by scripts."

Same page.

> "Since \"Copy Global Transform\" core add-on was integrated into Blender, any scripts accessing the add-on (trying to enable it, or check if its enabled) will break."

`https://developer.blender.org/docs/release_notes/5.0/animation_rigging/`

> "Python FBX import add-on is marked as legacy. Add-ons using `bpy.ops.import_scene.fbx` are advised to switch to `bpy.ops.wm.fbx_import` to use the new importer."

`https://developer.blender.org/docs/release_notes/5.0/pipeline_io/`

> "Because of that, custom themes from 4.5 LTS are expected to break in the new version."

`https://developer.blender.org/docs/release_notes/5.0/user_interface/` (about theme extensions rather than add-ons, but the same carrying-across problem).

**d. Python itself.**

> "Blender and the bpy module are only compatible with a single Python version. This makes it possible for add-ons and VFX software in general to only have to target a single Python version."

> "Blender bundles a complete Python installation and does not interact with the system Python by default."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html`, "Python Version". Read 2026-07-30. And, as quoted under question 13, a bundled compiled library may fail when "attempting to load a library built for a different version of Python" (`https://docs.blender.org/manual/en/5.0/troubleshooting/python.html`, read 2026-07-30).

**e. And the consequence of carrying it across anyway is named.**

> "Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this option may lead to errors on startup. If this occurs, the recommended first step is to try Loading Factory Settings."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/getting_started/configuration/defaults.html`. Read 2026-07-30.

> "Many crashes and errors may be caused by bad settings or add-ons that have not been updated to work with the newest release of Blender."

Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/troubleshooting/report_bug.html`. Read 2026-07-30.

**f. Which Blender wrote the preferences can be read back.**

> "version — Version of Blender the userpref.blend was saved with"

(a read-only property of `Preferences`). Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Preferences.html`. Read 2026-07-30.

### The other reading of the question

"Another Blender on the same computer" most likely means a different version, and that is what I answered. It could also mean a second copy of the *same* version — two separate 5.0 installations. On that reading: both read the same user directory, since it is named for the version and not for the installation (paths quoted above), so the same installed add-ons apply to both — unless one of them is a portable installation, or is given different `BLENDER_USER_*` variables, in which case it is not. That follows directly from the quoted paths and the quoted portable-installation rule.

### Where I looked

Searched the whole 5.0 manual, the whole 5.0 Python API documentation and the whole 5.0 release notes for: "previous version", "copy previous", "older version", "version compatib", "compatible with", "incompatible", "upgrade", "migrat", "co-exist", "side by side", "multiple versions", "blender_version_min", "blender_version_max", "bl_info", "BLENDER_USER_RESOURCES", "BLENDER_USER_EXTENSIONS", "BLENDER_SYSTEM_EXTENSIONS", "portable", "resource_path", "user_resource", "extension_path_user", "addon", "extension". Opened `advanced/blender_directory_layout.html`, `advanced/deploying_blender.html`, `advanced/command_line/arguments.html`, `advanced/command_line/extension_arguments.html`, `advanced/extensions/getting_started.html`, `advanced/extensions/addons.html`, `advanced/extensions/version_number_guidelines.html`, `advanced/extensions/python_wheels.html`, `advanced/extensions/creating_repository/static_repository.html`, `advanced/extensions/creating_repository/dynamic_repository.html`, `advanced/scripting/addon_tutorial.html`, `editors/preferences/addons.html`, `editors/preferences/extensions.html`, `editors/preferences/introduction.html`, `getting_started/configuration/defaults.html`, `getting_started/installing/windows.html`, `getting_started/installing/macos.html`, `getting_started/installing/linux.html`, `troubleshooting/python.html`, `troubleshooting/report_bug.html`, `troubleshooting/startup.html`; in the API `bpy.utils.html`, `bpy.app.html`, `bpy.types.Preferences.html`, `bpy.types.UserExtensionRepo.html`, `bpy.types.PreferencesExtensions.html`, `bpy.ops.extensions.html`, `bpy.ops.preferences.html`; the pictures `_images/editor_preferences_section_extensions_repositories.png` and `_images/advanced_deploying-blender_system-extensions.png`; the 5.0 release notes index and the `python_api`, `animation_rigging`, `pipeline_io`, `user_interface`, `core` and `bugfixes` pages; and Cycles 5.0 `__init__.py`.

Also checked Blender's requirements page (`https://www.blender.org/download/requirements/`, read 2026-08-01). It names Blender 5.0 in one place — "Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." — which is about hardware, not about add-ons, so it is ruled out for this question.

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

**Not covered.**

- What Blender actually *does* at startup with an already-installed extension whose `blender_version_min` is higher than the running Blender, or whose `blender_version_max` excludes it: hide it, grey it out, refuse to enable it, or report an error. The sources state this behaviour only for legacy add-ons ("older versions won't display the add-on in the list") and only for the minimum. For extensions the sources describe the manifest fields and the repository query, not what an already-installed mismatched extension does.
- Whether the version check is applied at install time, at startup, at enable time, or all three.
- The exact boundary of the versioned user directory — whether it is major-and-minor only. Marked inferred above.
- Whether a `BLENDER_SYSTEM_EXTENSIONS` directory is genuinely shared between versions. Marked inferred above.
- What happens if two Blender versions are pointed at one user directory by `BLENDER_USER_RESOURCES` — whether that is supported, and what the older one does with the newer one's preferences file.

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

The folder is named for the major and minor number only; the patch number never enters it, so 4.5.12 reads a folder called `4.5`. This is the macOS layout the sources describe, confirmed.

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


**Outside the interface.** **Settled by test on Linux, and on the Mac for everything but one item — see the two observed sections above.** All of it holds with no window, including the one part the sources leave interface-bound: `bpy.ops.preferences.copy_prev()` ran under `--background`, returned `{'FINISHED'}`, and carried the older version's installed extension into the newer version's folder. **On the Creator's Mac this does not hold for one item:** `bpy.ops.preferences.copy_prev()` refuses to run there in all four versions, with a window open as well as without one — see the Mac observed section above. What the sources say follows. The part that decides everything — which directories are read — holds outside the window, and the sources give command-line and environment-variable ways to control it: `BLENDER_USER_RESOURCES`, `BLENDER_USER_EXTENSIONS`, `BLENDER_SYSTEM_SCRIPTS`, `BLENDER_SYSTEM_EXTENSIONS`, and the arguments `--env-system-scripts` and `--env-system-extensions` (`https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, read 2026-07-30). Extension management has full command-line subcommands, and `--command` "implies --background mode" (same page). One thing does **not** carry over: "Import Preferences From Previous Version" is offered on the splash screen when a new version is first started (`https://docs.blender.org/manual/en/5.0/getting_started/configuration/defaults.html`, read 2026-07-30), and the sources give no equivalent for a Blender driven from outside its window. Copying the version directory with "standard file management tools" is mentioned for deployment, but not for this specific step. **Brought up to date 2026-08-07.** The one interface-bound item has now been pinned down further on Linux. `bpy.ops.preferences.copy_prev()` is reachable from a script there **only when the version folders sit in Blender's own layout**; when the settings folder is handed over with `BLENDER_USER_RESOURCES` — the way nearly every test in this round hands it over — it refuses in all four versions, `poll()` returning `False`. So on Linux the folder arrangement decides whether this route can be driven from outside the interface at all. See the observed section on the folder arrangements above.

**Operating system.** **Partly settled by test — see the observed section above.** The per-version folder, the shared-folder routes and the `portable` folder have now been confirmed on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with the macOS paths coming out as the sources describe them, including the `portable` folder inside `Blender.app/Contents/Resources`. **The two machines disagree on one point:** `bpy.ops.preferences.copy_prev()` runs on Linux and refuses to run on macOS. Two further macOS-only points: a stand-in `HOME` does not redirect Blender's user folder there, though `BLENDER_USER_RESOURCES` does. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The principle is the same on every operating system Blender runs on — setup lives in a per-version directory — but the paths differ, and the sources give all three: `$HOME/.config/blender/5.0/` (or `$XDG_CONFIG_HOME/blender/5.0/`) on Linux, `/Users/$USER/Library/Application Support/Blender/5.0/` on macOS, `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.0\` on Windows. The system directories differ too, including a separate path for a Microsoft Store installation. The portable folder sits next to the executable on Windows and Linux but "Inside the application bundle at Blender.app/Contents/Resources" on macOS. All from `https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html`, read 2026-07-30. Separately, a compiled library bundled with an add-on is tied to an operating system: "Unlike regular Python scripts, these are not portable between different platforms" (`https://docs.blender.org/manual/en/5.0/troubleshooting/python.html`, read 2026-07-30). **Brought up to date 2026-08-07.** The two Linux cells were run deliberately and the four cells now stand as a table in the observed section above: on Linux it runs under Blender's own layout and refuses when the settings folder is handed over; on macOS it refuses under both arrangements. The two machines still disagree, and the arrangement does not explain the disagreement away — only the Linux "runs" needed the arrangement named. The macOS cells were not run again in this sitting and are quoted from the Mac record. Windows is unchanged and was still not tried.

---

## Things a person with Blender open could settle by hand

The items on this list have since been run on **both** machines — a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores) and the Creator's own Mac (a MacBook Pro with an M1 Pro chip running macOS 26.5.2) — in all four Blender versions, and their results are written into the answers above as observed findings. They are kept here so those results can be traced back to what was asked. **Nothing on this list is still open.**

The two things those lists did not cover both needed a screen, which that machine does not have. Both have since been run on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — in all four versions, and their results are written into the answers above as observed findings. **Nothing on this list is still open.** The only thing left untested anywhere is Windows, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

Each is written so somebody who cannot see the reasoning above can carry it out, and each says what a negative result looks like.

**A. (Question 12) Does installing an extension from the command line disturb a render already running in a separate Blender?**

Start a long render with no window: `blender --background scene.blend --render-anim` on a scene set to take several minutes. While it runs, in a second terminal run `blender --command extension install-file -r user_default -e some-addon.zip`. Watch the first terminal.

Positive result: the render finishes normally, at the same rate, with the same frames written.
Negative result: the render stalls, slows noticeably, prints an error, exits early, or the install command hangs or reports it cannot proceed. Either outcome is informative, so the test carries information.

**B. (Question 12) Same thing, but the render is running inside a Blender with its window open, and the install is done in that same Blender.**

Start an animation render in the interface. While it is running, open Preferences ‣ Get Extensions and use Install from Disk on a `.zip`. Record whether the button can be pressed at all, whether the install completes, and whether the render output is unchanged.

Positive result: the install completes and the render finishes with the expected frames.
Negative result: the interface refuses the action while rendering, the render is cancelled, Blender crashes, or a frame is lost. Repeat with Render ‣ Lock Interface ticked and unticked, since the sources say that option exists to protect a render from interface activity.

**C. (Question 12) Do two Blenders fight over the extension repository lock?**

Run `blender --command extension install <package>` in two terminals at once, aimed at the same repository.

Positive result: one waits for the other and both succeed.
Negative result: one fails with a lock message, or both write and the repository is left broken (check with `blender --command extension list`).

**D. (Question 13) Does a failed enable name the add-on?**

Take a working extension `.zip`, edit its `__init__.py` so that `register()` raises an error, rebuild it with `blender --command extension build`, install it with `-e` to enable on install, and read what is printed.

Positive result: the output names the add-on and shows the traceback.
Negative result: the output is a bare traceback with no add-on name, or nothing at all and the add-on is silently left switched off.

**E. (Question 13) Can a script tell an add-on that registered from one that failed?**

After doing D, run `blender --background --python-expr "import bpy; print([a.module for a in bpy.context.preferences.addons])"`.

Positive result: the broken add-on's module is absent from the list, so the list is a usable signal.
Negative result: it is present anyway, meaning the list records intent rather than success and cannot be used as a health check.

**F. (Question 13) What does `extension list` actually print?**

Run `blender --command extension list` on a machine with one enabled extension, one installed-but-disabled extension, and one broken one.

Positive result: the three are distinguishable in the output.
Negative result: all three look the same, so the command cannot be used to check health.

**G. (Question 14) Is the user directory keyed to major and minor only?**

On a machine with two patch releases of the same major.minor (for example 5.0.x and 5.0.y), install an extension in one and start the other, then look at Preferences ‣ Add-ons.

Positive result: the second one lists it, confirming that patch releases share.
Negative result: it does not, meaning the directory is finer-grained than the documented path suggests.

**H. (Question 14) What does a Blender do with an installed extension whose declared minimum is above it?**

Build an extension with `blender_version_min = "5.1.0"`, install it into a directory shared by two Blender versions (set `BLENDER_USER_EXTENSIONS` to the same path for both), and start the 5.0 one. Look at Preferences ‣ Add-ons and at the console.

Positive result: it is hidden, or shown but not enableable, or shown with a warning — whichever it is, that is the behaviour the documentation does not state.
Negative result: it is listed and enables normally, meaning the declared minimum is not enforced at that moment.

**I. (Question 14) Do two versions really share a system extensions directory?**

Extract an extension to `$BLENDER_SYSTEM_EXTENSIONS/system/my-addon/`, then start two different Blender versions with the same variable set.

Positive result: both list it under the System repository.
Negative result: one or both do not, meaning the system path is version-scoped after all.

**J. (Question 14) Does pointing two versions at one user directory work at all?**

Set `BLENDER_USER_RESOURCES` to a single directory, start 5.0, save preferences, then start a different version with the same variable.

Positive result: the second starts and reads the same installed add-ons.
Negative result: it refuses, rewrites the preferences so the first version can no longer use them, or errors on startup.

---

2026-08-06 — Opus 5 (Claude Code) — Add-ons, Blender 5.0

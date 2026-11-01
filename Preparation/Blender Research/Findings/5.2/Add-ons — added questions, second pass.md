# Add-ons — added questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender's 5.2 manual, Blender's 5.2 Python API documentation, Blender's 5.2 release notes and Cycles' own files as shipped with 5.2 — all read from the copies taken on **2026-07-30**. Blender's requirements page, in the copy taken on **2026-08-01**, was opened and found not to name Blender 5.2 in so many words, so it is not cited as evidence for anything below.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

*Note added by the citation check (2026-08-06): unlike the other seven documents in this set, this one names its pages in words — "Get Extensions page", "Preferences ‣ Add-ons", "the tutorial" — rather than giving an address and a read date beside each quotation. Every quoted passage was found anyway, because the page names are unambiguous, but a later pass should supply the addresses so that checking does not depend on guessing which page is meant.*

Questions answered here: 12, 13 and 14 only. The rest of the sheet was left alone.

---

## 12. Does installing an add-on disturb a render already running on the same computer?

**LABEL: not in the sources.**

**That label answers the question from the sources only. A test has since been run and settled it — see the observed section below in this same answer.**

Nothing in the 5.2 manual, the 5.2 API documentation, the 5.2 release notes or Cycles' files says what happens to a render that is already under way when an add-on is installed. No page connects installing to rendering in either direction.

**The question has two readings, and neither is covered.** I read it as: a render is running, and an add-on is installed while it runs. That can mean (a) the install happens inside the very same Blender that is rendering, or (b) the install happens in a second Blender while a first Blender renders. I looked for both. The sources address neither.

### What is nearby, and why none of it answers the question

These are the closest things the sources do say. I list them so the gap is visible, not because any of them settles the question.

- Every documented way to install says nothing about rendering. The interface routes are described as "Install from the Website", "Install from Blender" and "Install from Disk" (Get Extensions page). The operators are `bpy.ops.extensions.package_install`, `bpy.ops.extensions.package_install_files` and `bpy.ops.preferences.addon_install`, whose only description is "Install an add-on". The command-line route is `blender --command extension install`. None of these carries a caution, a precondition, or any mention of a render, a job, or work in progress.

- Turning an add-on on is described as taking effect at once, but only in general terms: "The add-on functionality should be immediately available." (Preferences ‣ Add-ons.) Nothing says what "immediately" means for work already running.

- The API overview explains why add-ons can be toggled at all while Blender is up: "The register/unregister calls are used so it's possible to toggle add-ons and reload scripts while Blender runs." That is about Blender running, not about Blender rendering.

- Blender does document one hazard of changing things during a render, but it is a different thing. "While rendering the `frame_change_pre` and `frame_change_post` handlers are called from one thread and the viewport updates from a different thread. If the handler changes data that is accessed by the viewport, this can cause a crash of Blender. In such cases, lock the interface (Render → Lock Interface or `bpy.types.RenderSettings.use_lock_interface`) before starting a render." That warning is about a handler altering scene data mid-render. It does not mention installing or enabling anything.

- Lock Interface itself is documented only as "Lock interface during rendering in favor of giving more memory to the renderer" (Topbar ‣ Render, and the same wording on `RenderSettings.use_lock_interface`). The sources do not say what the interface being locked prevents a person or a script from doing.

- Extension repositories do have a lock of some kind, but it is never explained. The only trace of it is an operator description: `bpy.ops.extensions.repo_unlock()` — "Remove the repository file-system lock", alongside `repo_lock_all()` — "Lock repositories - to test locking" and `repo_unlock_all()` — "Unlock repositories - to test unlocking". Nothing says what takes the lock, when, or what a second Blender does if it finds one taken.

- Blender can be asked whether a render is running — `bpy.app.is_job_running(job_type)`, "Check whether a job of the given type is running", with `RENDER` — "Regular rendering" among the job types. So the state is readable. What installing does to that state is not written down.

- The one place the manual discusses several renders running at once is a hint about splitting an animation across machines: "Set up a shared directory over a network file system. Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory." It is about machines sharing frames, not about installing anything.

**A step I can take, and its limit.** The command-line install is `blender --command extension install`, i.e. it is carried out by launching Blender. So a command-line install of this kind is done by a Blender process, which is not the process that was already rendering. **LABEL for this step: inferred** — the source shows the command begins with `blender`, and I supplied the reading that this starts a Blender process. What that second process does to the first one's render — competing for processor, memory or the same files — is not in the sources at all.

### Where I looked

- Whole-manual and whole-API searches for: `while rendering`, `during rendering`, `during a render`, `render is running`, `render in progress`, `interrupt`, `abort`, `cancel the render`, `stop the render`, `affect the render`, `pause the render`, `disturb`, `interfere`, `busy`, `safe to install`, `multiple instances`, `another instance`, `second instance`, `simultaneously`, `at the same time`, `concurrent`, `while blender is running`, `running instance`, `lock`, `file-system lock`, `repository lock`, `restart`, `reload`, `render farm`, `headless`, `queue`, `batch render`, `job`.
- Pages opened in full: Preferences ‣ Add-ons; Preferences ‣ Get Extensions; Creating Extensions (index, Getting Started, Add-ons, Python Wheels, Version Number Guidelines); Extensions Command Line Arguments; Command Line Arguments; Blender's Directory Layout; Deploying Blender in Production; Defaults; Scripting & Security; Add-on tutorial; Operators (System Operators); Topbar; Cycles GPU Rendering FAQ; Output Properties; troubleshooting Python Errors; troubleshooting Network; API pages `bpy.ops.extensions`, `bpy.ops.preferences`, `bpy.ops.script`, `bpy.app`, `bpy.app.handlers`, `bpy.utils`, `bpy.types.Addon`, `bpy.types.Addons`, `bpy.types.AddonPreferences`, `bpy.types.PreferencesExtensions`, `bpy.types.UserExtensionRepo`, `bpy.types.RenderSettings`, the Window-Manager job-type list, and the API's overview, threading, crashes and best-practice notes.
- Release notes for 5.2: every subject file searched for `add-on`, `addon` and `extension`. The only hits are unrelated (a Virtual Reality manual link, a File Output "File Extension" checkbox, glTF material extensions, Loop Tools features becoming built in, a filter icon, and three bug-fix entries). Nothing about installing.
- Cycles' files: searched, and they are the sources of one add-on rather than documentation of the install system; they say nothing about installing.
- Ruled out as not bearing on the question: the whole GPU-troubleshooting tree, the render-engine and Cycles settings pages, and the asset-library pages — searched for the terms above and matched only on unrelated senses of the words.

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

**Not covered.** This answer does not settle: whether an install can change what a running render produces; whether it can end or stall one; whether the two compete for processor, memory or graphics memory; whether the extension repository lock can block either side; and whether an add-on enabled part-way through a render is picked up by that render or only by the next one.

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every part of this question was reached with no window at all: the render, the installs, the removals, and an install fired from inside the rendering Blender by a handler. None of it needed a screen. **The windowed half has since been run on the Creator's Mac and agrees with the no-window half:** with a window open, an install fired from inside the rendering Blender returned `{'FINISHED'}`, every frame was written, and every frame matched its control picture for picture — with the interface lock off and on alike. What the sources say follows. The gap is the same either way. There are documented ways to install without a person clicking — `blender --command extension install`, `blender --command extension install-file`, and enabling at startup with `--addons` or from a startup script — but no source says what any of them do to a render already running, in the interface or out of it. **Brought up to date 2026-08-07.** The interface-lock half has now been rebuilt with no window at all, with `use_lock_interface` read back at every step and from inside the render handler at the very frame the install fires — see the observed section on the rebuilt lock test above. With the lock genuinely on, the install returned `{'FINISHED'}`, all eight frames were written, and every frame matched its control picture for picture, in all four versions. Two further things came out of that read-back and bear on this line: `bpy.context.window_manager.is_interface_locked` and `bpy.app.is_job_running('RENDER')` both read `False` at every moment, including mid-render with the lock on, so neither gives a program outside the interface a view of the scene setting or of the running render. **The windowed form of the same test is still waiting on the Creator's Mac and stays sent back.**

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The sources do not say. The install commands and operators are documented without any per-system note, and I found no page that treats this question differently on Windows, macOS or Linux. **Brought up to date 2026-08-07.** The rebuilt interface-lock test was run on the rented Linux machine only, in all four versions, and came out as the Linux runs above. The windowed form of it needs the Creator's Mac and has not been run again. Nothing here changes what stays open for Windows.

---

## 13. Once an add-on is installed and switched on, what more can be learned about whether it will work?

**LABEL: documented**, for each of the checks below. Note the shape of what is documented: Blender documents places where trouble shows up, not a test that promises an add-on will work.

### What the sources say can be learned

**1. Whether it failed to start, from the console.** The same tip appears on two pages: "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred." (Preferences ‣ Add-ons, and Preferences ‣ Get Extensions; the Creating Extensions ‣ Add-ons page carries it too.) Of that console the manual says: "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors." and "Blender output is shown in the Console Window it is started from." (Using Blender From The Command Line.) *(Changed by the citation check: the first of these had been given as "an operating system text window that displays messages about what Blender is doing", which is a paraphrase and is not on the page; the page's own words are above.)*

**2. Whether Blender itself has anything to complain about, by rescanning.** "Refresh Local — Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting). Any issues are reported as warnings." The same thing exists as an operator: `bpy.ops.extensions.repo_refresh_all` — "Refresh extension & legacy add-ons, reloading modules & meta-data (similar to restarting)". For legacy add-ons there is also "Refresh — Scans the Add-on Directory for new add-ons" and `bpy.ops.preferences.addon_refresh()` — "Scan add-on directories for new modules".

**3. What the add-on says about itself, and where it lives.** "You can click the arrow at the left of the add-on box to see more information, such as its location, a description and a link to the documentation. Here you can also find a button to report a bug specific of this add-on." The listing is fed by the add-on's own declared data: "The only difference between add-on extensions and built-in Python modules is that add-ons must include a `blender_manifest.toml` which Blender uses to read metadata such as name, author, tags and project link. The User Preferences add-on listing uses this to display information about each add-on." *(Source named by the citation check: this sentence is on the Python API's Overview page, not on either Preferences page — `api_5.2/info_overview.txt`, `https://docs.blender.org/api/5.2/info_overview.html`.)*

**4. Whether it still needs setting up before it does anything.** "Some add-ons may have their own preferences which can be found in the Preferences section of the add-on information box. Some add-ons use this section for example to enable/disable certain functions of the add-on. Sometimes these might even all default to off. So it is important to check if the enabled add-on has any particular preferences." So switched on does not mean ready.

**5. Where it was actually put.** "When installing an add-on the source and destination paths are printed in the console. You can also find add-on path locations by running this in the Python Console: `import addon_utils` / `print(addon_utils.paths())`." Related readable paths: `bpy.utils.script_paths(...)` — "Returns a list of valid script paths"; `bpy.utils.script_path_user()`; `bpy.utils.resource_path(type, *, major=None, minor=None)` — "Return the base path for storing system files"; `bpy.utils.extension_path_user(package, *, path='', create=False)` — "Return a user writable directory associated with an extension."

**6. Whether the third-party code it carries actually loaded.** For an add-on that bundles Python wheels: "Once the extension has been installed you can check the module is being loaded by importing it in the Python console and printing its location: `import PIL` / `print(PIL.__file__)`."

**7. Whether the compiled parts suit this machine.** "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular Python scripts, these are not portable between different platforms. It is possible the library is incompatible with your Blender installation (attempting to load a library built for a different version of Python, or loading a 32-bit library on a 64-bit system). If the add-on contains `.pyd` or `.so` files, check that the distribution is compatible with your operating system." The same page gives a script for tracking down duplicate Python libraries on Windows and says where its output goes: "(The output will show in Command Line Window.)"

**8. Whether it will be allowed to reach the internet.** "If the add-on needs to use internet, it must check for the (read-only) property `bpy.app.online_access`." And the limit of that knowledge is stated outright: "Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule."

**9. What is installed at all, and from where.** `blender --command extension list` — "List packages from all enabled repositories"; `blender --command extension repo-list` — "List all repositories stored in Blender's preferences". In the API, `Preferences.addons` is a "Collection of add-ons", each with `module` — "Module name" — and `preferences`.

**10. That an error state exists and can be cleared.** `bpy.ops.extensions.status_clear_errors()` appears in the API listing. Its description is "Undocumented, consider contributing", so the sources show that extension errors are tracked but do not say what they contain or how to read them.

**One step beyond the quotes.** The tutorial says: "When the add-on is enabled, Blender executes the code and runs the `register()` function. When the add-on is disabled, Blender runs the `unregister()` function." From that, a switch-on that produces no console error tells you the add-on's code was imported and its `register()` ran. **LABEL for this step: inferred** — the reader has to supply that "no error printed" means "the code above ran to the end". Nothing in the sources says a clean enable guarantees anything past that point.

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

**Not covered.** None of this settles whether the add-on does its job correctly — whether it produces the right result, works on a particular file, or behaves the same on every frame. The sources describe reporting of failures, not a positive test of fitness. They also do not say what an extension's tracked "errors" hold, what Refresh Local's warnings look like, or whether any of these checks can be read back as data rather than as text on a screen.

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


**Outside the interface.** **Settled by test on both machines — see the two observed sections above.** Every Python and command-line check named in this answer was run in a Blender started with no window and returned rather than raising, including `bpy.ops.extensions.repo_refresh_all()`, `bpy.ops.preferences.addon_refresh()` and `bpy.ops.preferences.addon_expand()`, which the sources leave unstated. **The parts that are panels in the Preferences editor have since been reached on the Creator's Mac, which has a screen** — see the Mac observed section above. A declared warning is shown on the row and inside it; an add-on that failed to switch on is not marked at all. What the sources say follows. Mostly yes, with two exceptions.
- Works from outside: the console is where errors appear and it is the terminal Blender was started from; `--addons <addon(s)>` — "Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons"; enabling from a startup script, for which the manual gives a worked example — `import addon_utils` / `addon_utils.enable("my-addon")`; `blender --command extension list` and `repo-list`; and anything readable through Python (`addon_utils.paths()`, `bpy.utils.script_paths`, `Preferences.addons`, `bpy.app.online_access`). Blender knows whether it is running without an interface: `bpy.app.background` — "True when blender is running without a user interface (started with -b)".
- Interface only, as documented: the add-on information box with its location, description, documentation link and bug-report button; the Refresh Local and Refresh buttons — though the same work is exposed as `bpy.ops.extensions.repo_refresh_all` and `bpy.ops.preferences.addon_refresh()`. The sources do not say whether those operators can be called when Blender is running without an interface.

**Operating system.** **Partly settled by test — see the observed section above.** Everything above has now been run on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — and the two agreed. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. Two of these are stated to differ.
- The precompiled-library check is explicitly per-system: "these are not portable between different platforms… check that the distribution is compatible with your operating system", and the duplicate-library script is under a heading "Platform Specific ‣ Windows".
- The console differs: it is "an operating system text window"; on Windows the manual notes `--start-console` — "Start with the console window open (ignored if -b is set), (Windows only)" — and describes the console appearing and then being toggled off at startup.
The other checks are documented without any per-system note, and I found nothing saying they differ.

### Where I looked

Searched the whole manual and the whole API documentation for: `add-on`, `addon`, `extension`, `console`, `error`, `warning`, `traceback`, `refresh`, `validate`, `permission`, `wheel`, `precompiled`, `library`, `register`, `reload`, `restart`, `debug-python`, `online_access`, `addon_utils`, `is_job_running`. Opened in full: Preferences ‣ Add-ons; Preferences ‣ Get Extensions; Creating Extensions (index, Getting Started, Add-ons, Python Wheels); Extensions Command Line Arguments; Command Line Arguments (Python, debug, misc and environment sections); The Console Window; troubleshooting Python Errors; troubleshooting Network; Deploying Blender in Production; the add-on tutorial; API pages `bpy.ops.preferences`, `bpy.ops.extensions`, `bpy.ops.script`, `bpy.app`, `bpy.utils`, `bpy.types.Addon`, `bpy.types.Addons`, `bpy.types.AddonPreferences`, `bpy.types.Preferences`, `bpy.types.PreferencesExtensions`, `bpy.types.UserExtensionRepo`, the API overview and the tips-and-tricks, gotchas and best-practice notes. Pictures opened: the Add-ons preferences screenshot (`_images/editors_preferences_section_addons.webp`) and the Get Extensions screenshot (`_images/editors_preferences_section_extensions.webp`). **LABEL for what the pictures show: documented** — the Add-ons picture shows a searchable list with one tick-box per add-on and an expand arrow, greyed rows for the ones not ticked, a search field, an "Enabled Only" tick-box and a tag button; the Extensions picture shows the online-access prompt with "Continue Offline" and "Allow Online Access". Neither picture shows any per-add-on health, status or error indicator. Ruled out after searching: the whole troubleshooting GPU tree, and the release notes for 5.2, which say nothing about checking an installed add-on.

---

## 14. Does an add-on set up for one Blender work in another Blender on the same computer, and what decides that?

**LABEL: documented.**

Short answer from the sources: not by itself. Two Blenders of different versions on one computer keep their settings and their installed add-ons in separate folders, so an add-on set up in one is simply not set up in the other until it is brought across — and bringing it across is not a guarantee it works, because the add-on itself declares which Blender versions it supports.

### The parts, each with the words that carry it

**Two Blenders can sit on one computer.** "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

**Each keeps its own settings and its own installed add-ons, in a folder named for the version.** The user directory is per version — on Linux `$HOME/.config/blender/5.2/`, on macOS `/Users/$USER/Library/Application Support/Blender/5.2/`, on Windows `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.2\` — and it holds, among other things, `./extensions` — "Extension repositories" — and `./scripts` — "Add-ons, presets, templates, user interface, startup scripts". The reason is stated plainly on the first-run page: "The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders."

**Bringing them across is an offered step, and it is stated to carry risk.** "Import Preferences From Previous Version — Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and startup files from the previous version of Blender and load them. This will include installed add-ons and extensions." And, on the same page: "Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this option may lead to errors on startup. If this occurs, the recommended first step is to try Loading Factory Settings."

**What decides whether it works, for an extension: the add-on's own declared range.** `blender_version_min` is required — "Minimum supported Blender version - use at least 4.2.0" — and `blender_version_max` is optional — "Blender version that the extension does not support, earlier versions are supported." The manifest example carries the same note in its comments: "Optional: Blender version that the extension does not support, earlier versions are supported. This can be omitted and defined later on the extensions platform if an issue is found." Authors are told to keep this honest: "Make sure to correctly indicate the Blender versions that each version of your extension is compatible with in their manifest file. You can also update Blender version compatibility of already-uploaded versions of your add-on from the extensions website."

**What decides it for a legacy add-on: the version recorded in `bl_info`.** "`bl_info` is a dictionary containing add-on metadata such as the title, version and author to be displayed in the Preferences add-on list. It also specifies the minimum Blender version required to run the script; older versions won't display the add-on in the list."

**Observed, in Cycles' own files.** Cycles ships as an add-on and records exactly such a version. In `cycles_5.2/__init__.py`, lines 7–16:

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

There is no `blender_manifest.toml` anywhere in `cycles_5.2/`. **LABEL: observed** (file and place given; read 2026-07-30). What a render does with that recorded version is a step beyond the file and is not claimed here.

**A second thing that decides it: the Python inside each Blender.** "Blender and the `bpy` module are only compatible with a single Python version. This makes it possible for add-ons and VFX software in general to only have to target a single Python version. Blender bundles a complete Python installation and does not interact with the system Python by default." Add-ons that bundle third-party code bundle it built for a particular Python and a particular machine — the manual's own example downloads wheels with `--python-version=3.13` for `macosx_11_0_arm64`, `manylinux_2_28_x86_64` and `win_amd64`, and requires that "Wheels filenames must match Python's binary distribution specification". Release notes state a Python change did happen between releases: the 5.1 notes carry the heading "Python 3.13" and list "Python 3.13" among that version's items. The 5.2 release notes do not restate a Python version. *(Flagged by the citation check: this is the one place in this answer where a claim rests on another version's copy of the documentation — `release_notes_5.1/python_api.md` rather than anything under 5.2. The words are really there; the source is outside this version's own sources, and a reader should treat it as such.)*

**And a third: hand-compiled parts.** "Unlike regular Python scripts, these are not portable between different platforms. It is possible the library is incompatible with your Blender installation (attempting to load a library built for a different version of Python, or loading a 32-bit library on a 64-bit system)."

**Deliberately sharing one set of add-ons between Blenders is possible.** `BLENDER_USER_RESOURCES` — "Replace default directory of all user files"; also "The `BLENDER_USER_RESOURCES` environment variable can be set to a custom directory to replace the default user directory." For read-only bundling there is a system repository: "The `$BLENDER_SYSTEM_EXTENSIONS` environment variable controls the default location. This should point to a directory, within which a `system` directory should exist. Extensions packages should be extracted in this system directory, with a resulting path like this: `$BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml`" — and for older-style add-ons, "`$BLENDER_SYSTEM_SCRIPTS` can also be used to add script directories without modifying the preferences", with add-ons expected at e.g. `$BLENDER_SYSTEM_SCRIPTS/addons/simple_addon.py`. There is also portable use: "When running Blender from a portable drive, it's possible to keep the configuration files on the same drive to take with you… This folder will then store preferences, startup file, installed extensions and presets."

**One step beyond the quotes.** The user folder is named for the version — `…/Blender/5.2/` — and the reason given is that "the configuration files of each Blender version are stored in separate folders". From that: two Blender installations that are the same version, sitting in two places on one computer, share the one user folder by default, and an add-on installed in one is therefore already present in the other. **LABEL for this step: inferred** — the source names the folder and says folders are per version; I supplied the reading that two same-version installations therefore land on the same folder. The sources never discuss two installations of the same version. Consistent with it: `bpy.utils.resource_path(type, *, major=None, minor=None)` — "Return the base path for storing system files" — takes a major and minor version, and "None (the default) uses `bpy.app.version[0]`" and `[1]` respectively, so the path is built from the running Blender's version number, not from where it was installed.

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

**Not covered.** This does not settle what Blender actually does when it meets an add-on outside its declared range — for legacy add-ons the manual says only that older Blenders "won't display the add-on in the list", and for extensions the sources say nothing at all about what a Blender newer than `blender_version_max` does with it (hide it, refuse to switch it on, switch it on anyway, or report something). It also does not settle whether an add-on's own stored preferences survive being copied to another version, what happens if `BLENDER_USER_RESOURCES` is pointed at one folder by two different Blender versions, or whether the extension repository format is the same from version to version.

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


**Outside the interface.** **Settled by test on Linux, and on the Mac for everything but one item — see the two observed sections above.** All of it holds with no window, including the one part the sources leave interface-bound: `bpy.ops.preferences.copy_prev()` ran under `--background`, returned `{'FINISHED'}`, and carried the older version's installed extension into the newer version's folder. **On the Creator's Mac this does not hold for one item:** `bpy.ops.preferences.copy_prev()` refuses to run there in all four versions, with a window open as well as without one — see the Mac observed section above. What the sources say follows. The version-specific folders, the environment variables, the manifest range and the recorded `bl_info` version all hold regardless of whether anybody is at the screen — none of them are described as interface features. Add-ons can be switched on without the interface with `--addons <addon(s)>` or from a startup script (`import addon_utils` / `addon_utils.enable("my-addon")`), and installed with `blender --command extension install` / `install-file`. The one exception is the copy-across step: "Import Preferences From Previous Version" is offered in the first-run screen that replaces "the interactive region of the Splash Screen", and the sources name no command-line or Python equivalent for it. The sources do say installations themselves can be handled with ordinary file tools: "deployment can be automated using standard file management tools." **Brought up to date 2026-08-07.** The one interface-bound item has now been pinned down further on Linux. `bpy.ops.preferences.copy_prev()` is reachable from a script there **only when the version folders sit in Blender's own layout**; when the settings folder is handed over with `BLENDER_USER_RESOURCES` — the way nearly every test in this round hands it over — it refuses in all four versions, `poll()` returning `False`. So on Linux the folder arrangement decides whether this route can be driven from outside the interface at all. See the observed section on the folder arrangements above.

**Operating system.** **Partly settled by test — see the observed section above.** The per-version folder, the shared-folder routes and the `portable` folder have now been confirmed on **both** machines — the rented Ubuntu 22.04.5 LTS machine and the Creator's own Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with the macOS paths coming out as the sources describe them, including the `portable` folder inside `Blender.app/Contents/Resources`. **The two machines disagree on one point:** `bpy.ops.preferences.copy_prev()` runs on Linux and refuses to run on macOS. Two further macOS-only points: a stand-in `HOME` does not redirect Blender's user folder there, though `BLENDER_USER_RESOURCES` does. Windows was not tried at all — no Windows machine has run it — so this line stays open for Windows only. What the sources say follows. The per-version scheme holds on all three systems Blender's directory-layout page names — Linux, macOS and Windows — but the actual folders differ per system, as quoted above, and so do the system folders (`./5.2/` beside the executable, `/usr/share/blender/5.2/`, `./Blender.app/Contents/Resources/5.2/`, `%ProgramFiles%\Blender Foundation\Blender\5.2\`, `%ProgramFiles%\WindowsApps\BlenderFoundation.Blender<HASH>\Blender\5.2\`). Portable use differs too: the `portable` folder goes next to the executable on Windows and Linux, but "Inside the application bundle at `Blender.app/Contents/Resources`" on macOS. And what an add-on carries can be system-specific: the manifest's optional `platforms` list is "`["windows-x64", "windows-arm64", "macos-x64", "macos-arm64", "linux-x64"]`", and compiled parts "are not portable between different platforms". Blender's requirements page was checked for a per-system statement about 5.2 and does not name 5.2 in so many words, so it is not used here. **Brought up to date 2026-08-07.** The two Linux cells were run deliberately and the four cells now stand as a table in the observed section above: on Linux it runs under Blender's own layout and refuses when the settings folder is handed over; on macOS it refuses under both arrangements. The two machines still disagree, and the arrangement does not explain the disagreement away — only the Linux "runs" needed the arrangement named. The macOS cells were not run again in this sitting and are quoted from the Mac record. Windows is unchanged and was still not tried.

### Where I looked

Searched the whole manual, the whole API documentation and all 5.2 release-note files for: `blender_version_min`, `blender_version_max`, `bl_info`, `manifest`, `co-exist`, `multiple versions`, `previous version`, `copy previous`, `import preferences`, `compatible`, `compatibility`, `directory layout`, `user directory`, `portable`, `BLENDER_USER_RESOURCES`, `BLENDER_SYSTEM_EXTENSIONS`, `BLENDER_SYSTEM_SCRIPTS`, `wheel`, `python version`, `Python 3.`, `precompiled`, `platforms`, `resource_path`, `upgrade`, `legacy`. Opened in full: Blender's Directory Layout; Deploying Blender in Production; Defaults (first-run screen); Preferences ‣ Add-ons; Preferences ‣ Get Extensions; Preferences ‣ File Paths (scripts note); Creating Extensions (index, Getting Started, Add-ons, Python Wheels, Version Number Guidelines); Extensions Command Line Arguments; Command Line Arguments; the add-on tutorial; troubleshooting Python Errors; API pages `bpy.utils`, `bpy.app`, `bpy.types.Addon`, `bpy.types.UserExtensionRepo`, `bpy.ops.extensions`, `bpy.ops.preferences`. Cycles' files: listed the whole of `cycles_5.2/`, read `__init__.py`, and searched for any `.toml` manifest — there is none. Requirements page: opened `requirements/requirements_2026-08-01.txt` and searched it for "5.2"; it names 4.5 and speaks of "Blender 5.0 and later" but never names 5.2, so by its own limits it is not evidence here. Ruled out after searching: the render, modelling, sculpting and animation trees, which match these terms only in unrelated senses.

---

## Things a person at a computer with Blender open could settle in a few minutes

The items on this list have since been run on **both** machines — a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores) and the Creator's own Mac (a MacBook Pro with an M1 Pro chip running macOS 26.5.2) — in all four Blender versions, and their results are written into the answers above as observed findings. They are kept here so those results can be traced back to what was asked. **Nothing on this list is still open.**

The two things those lists did not cover both needed a screen, which that machine does not have. Both have since been run on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — in all four versions, and their results are written into the answers above as observed findings. **Nothing on this list is still open.** The only thing left untested anywhere is Windows, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

Each says what to do and what a negative result would look like.

**A. Does installing an extension while a render is running end or spoil that render? (question 12, same Blender)**
Open Blender 5.2. Set the render to Cycles with a high sample count so a still takes several minutes. Start the render (F12). While it runs, open Preferences ‣ Get Extensions ‣ Install from Disk and install any small extension `.zip`. Watch the render window and the console.
*Negative result:* the render carries on to completion and saves a normal image, and the console shows no new error — meaning the install did not disturb it. *Positive result:* the render stops, stalls, produces a different or corrupt image, or Blender reports an error or closes.

**B. Same question, second Blender. (question 12, separate processes)**
Start a render without an interface: `blender --background scene.blend --render-frame 1 --render-output /some/path`. While it runs, in a second terminal run `blender --command extension install-file -r <repo id> -e some_addon.zip`. Watch both.
*Negative result:* the background render finishes normally and writes its frame, and the install command reports success — neither disturbed the other. *Positive result:* either command fails, hangs, or reports a repository lock, or the rendered frame is missing or wrong.

**C. Can an install be blocked while a render holds the interface? (question 12)**
Turn on Render ‣ Lock Interface, start a long render, and try Install from Disk.
*Negative result:* the install proceeds as normal — Lock Interface does not stand in the way. *Positive result:* the menu item is unavailable, the operator refuses, or nothing happens until the render ends.

**D. What does a clean enable actually tell you? (question 13)**
Start Blender from a terminal so the console is visible. Install a small add-on whose `register()` prints a line, switch it on in Preferences ‣ Add-ons, then switch on an add-on that is deliberately broken (e.g. one whose first line imports a module that is not there).
*Negative result:* both switch on with nothing printed, and the tick-box stays ticked for the broken one — meaning the console tells you nothing and a ticked box means nothing. *Positive result:* the good one prints its line, the broken one prints an error and its tick-box does not stay ticked.

**E. What does Refresh Local report? (question 13)**
With an add-on installed and switched on, edit its files on disk to introduce a plain syntax error, then press Preferences ‣ Add-ons ‣ Refresh Local.
*Negative result:* nothing appears — no warning, no change in the listing — so Refresh Local's "issues are reported as warnings" does not cover this kind of breakage. *Positive result:* a warning appears naming the add-on.

**F. Are two Blender versions really separate? (question 14)**
Install any extension in Blender 5.2. Then start a different Blender version already on the same computer and open Preferences ‣ Add-ons. Then look on disk for the two version-named user folders side by side.
*Negative result:* the add-on is listed and ticked in the other version too, and only one user folder exists — meaning the folders are not in fact separate on this machine. *Positive result:* the other version does not list it, and two separately named folders exist.

**G. What happens past `blender_version_max`? (question 14)**
Build a trivial extension with `blender --command extension build`, with a manifest whose `blender_version_max` is set below 5.2 (for example `5.0.0`). Install it into Blender 5.2 with Install from Disk and look at the Add-ons list and the console.
*Negative result:* it installs, appears in the list, and switches on with no message — meaning `blender_version_max` is not enforced at install or enable time by Blender itself. *Positive result:* the install is refused, or it is installed but hidden, greyed, marked, or refuses to switch on, with a message naming the version.

**H. Two installations of the same version. (question 14, the inferred step)**
Put two copies of Blender 5.2 in two different folders on one computer. Install an extension using the first, then start the second and look at Preferences ‣ Add-ons.
*Negative result:* the second does not list it — meaning same-version installations do not share a user folder after all, and the inference above is wrong. *Positive result:* the second lists it as installed.

---

2026-08-06 — Opus 5 (Claude Code) — Add-ons, Blender 5.2

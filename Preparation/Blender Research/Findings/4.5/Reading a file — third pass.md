# Reading a file — Blender 4.5 — third pass

**Sources consulted:** the Blender 4.5 manual (`Blender Sources/manual_4.5`, including its `_images` folder), the Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), the Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All read on 2026-07-30 (the snapshot date), reported 2026-08-05.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 2 was assigned. It is answered below, keeping its number.

---

## 2. How many Blenders can be at work on one computer at the same time, and does one reading a file while another renders cause trouble?

### On the question itself

The question is ambiguous in two places, and I say so rather than choose silently.

**"How many"** can mean either (i) does Blender impose a cap on how many copies of itself may run at once, or (ii) how many is it sensible to run before the machine gives out. I answer (i) as the reading meant, because it is the one the sources can speak to at all. Reading (ii) would be a judgement about a particular computer, and no source I found offers a number for it.

**"One reading a file while another renders"** can mean either (i) two Blenders touching *the same* `.blend` — one opening it while the other is rendering it — or (ii) any two Blenders running at once, one doing file work and one rendering, on unrelated files. I answer (i) as the reading meant, since it is the sharper one, and cover (ii) where the sources touch it.

### Answer, first half: how many can be at work at once

**Several can. The sources say plainly that more than one Blender can be running on a machine at the same time, and they never state a maximum.**

Four separate places take concurrent Blenders for granted, and one of them is a feature that starts a second Blender itself.

**LABEL: documented** — that more than one can run at once.

Exact words, source, address, date:

> "As with the application it's possible to start multiple instances,
> each with their own bpy and therefor Blender state.
> Python provides the multiprocessing module to make communicating with sub-processes more convenient."

Python API documentation, https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html — read 2026-07-30. (The page is about Blender built as a Python module; the phrase "As with the application" is what carries the claim across to ordinary Blender.)

> "The location of Blender's executable, useful for utilities that open new instances."

Python API documentation, `bpy.app.binary_path`, https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30.

> "Opens the blend-file that contains the asset in a new Blender instance.
> When this instance is closed, the Asset Browser will be automatically refreshed."

Manual, Asset Browser, https://docs.blender.org/manual/en/4.5/editors/asset_browser.html — read 2026-07-30. This is Blender starting a second Blender while the first stays open and waits for it to close.

> "The Copy UVs tool currently uses an internal clipboard which is not shared between instances of blender."

Manual, UV Editing, https://docs.blender.org/manual/en/4.5/modeling/meshes/uv/editing.html — read 2026-07-30.

The word "instance" is defined by the manual, so it is not my own reading of it:

> "The timespan of a Blender instance. The session begins with starting an instance of Blender
> and ends with closing it."

Manual, Glossary, entry "Blender Session", https://docs.blender.org/manual/en/4.5/glossary/index.html — read 2026-07-30.

And the manual is careful to separate an extra *window* from an extra *instance*, which matters because the two are easy to confuse:

> "The new window is a fully functional window, which is part of the same instance of Blender."

Manual, Areas, https://docs.blender.org/manual/en/4.5/interface/window_system/areas.html — read 2026-07-30.

**LABEL for the number itself: not in the sources.** No page states a maximum number of Blenders per computer, nor says the number is unlimited. Where I looked for it is recorded below.

The nearest thing to a bound anywhere is about resources, not about a count of Blenders, and I record it as such rather than dress it up as an answer:

> "Use amount of <threads> for rendering and other operations
> [1-1024], 0 to use the systems processor count."

Manual, Command Line Arguments, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

> "Rendering takes all available CPU time; you should render overnight,
> when the computer is not needed, or set Blender to a low priority while rendering,
> and work on other things (be careful with the RAM space!)."

Manual, Rendering Animations, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30.

These say a render will take the whole processor and warn about memory. They do not say how many Blenders may run, and I do not treat them as if they did.

### Answer, second half: does one reading a file while another renders cause trouble

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes or Cycles' own files says what happens when one Blender opens a `.blend` while another Blender is rendering it. There is no mention anywhere of a `.blend` being locked, held open, claimed exclusively, or of any warning shown when a second Blender opens a file already in use. There is equally no statement that it is safe. The sources are simply silent on it. My record of looking is below, and it is long, because an absence is the claim that has to be worked hardest for.

Three things sit close to the question and are worth having, each clearly bounded so nothing is over-read.

**Closest thing 1 — autosave files are kept apart by process ID.** This is the only place in all four sources where concurrently running Blenders are separated by name rather than by luck.

> "Automatic saving of temporary files in temp directory, uses process ID.
> Warning: Sculpt and edit mode data won't be saved"

Python API documentation, `PreferencesFilePaths.use_auto_save_temporary_files`, https://docs.blender.org/api/4.5/bpy.types.PreferencesFilePaths.html — read 2026-07-30. The same property is documented as `boolean, default True`.

That quote is **documented**. The step beyond it — that two Blenders running at the same time therefore write autosave files under different names and do not overwrite each other's — is **inferred**, and here is the step: the source says the name uses the process ID but never says what a process ID is or that two live processes have different ones. The reader has to supply that from outside Blender. So it is inference, not documentation. Note also that the manual describes the naming differently and less firmly:

> "Auto-saved files typically have a name such as <filename>_autosave.blend or a random identifier,
> and use the .blend extension."

Manual, Opening & Saving, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — read 2026-07-30. The same page says "Only one auto-saved file is kept per project." The manual and the API documentation do not describe the naming in the same terms, and neither says which governs when two Blenders share a temporary directory.

**Closest thing 2 — concurrent Blenders share one temporary directory, and the sources do not say what happens there.**

> "The temporary directory is used to store various files at run-time
> (including render layers, physics cache, copy-paste buffer and crash logs)."

> "The temporary directory is selected based on the following priority:
> User Preference (see File Paths).
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
> The /tmp/ directory."

Manual, Blender's Directory Layout, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30.

Two Blenders started the same way on the same machine would resolve that to the same directory by those rules, and render layers and physics cache go into it. Whether they collide there is **not in the sources** — no page addresses it. I flag one discrepancy I found while checking this, because it bears on isolating instances: the directory-layout page names the variables as "TEMP on Windows, TMP & TMP_DIR on other platforms", while the command-line arguments page lists "TEMP: Store temporary files here (MS-Windows)" and "TMPDIR: Store temporary files here (UNIX Systems). The path must reference an existing directory or it will be ignored" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). The two pages do not agree on the name, and neither is presented as correcting the other.

**Closest thing 3 — several renderers writing to one directory is a documented, supported arrangement, but it is about output files, not about reading the `.blend`.**

> "Overwrite
> Overwrite existing files when rendering.
> Placeholders
> Create empty placeholder frames while rendering."

> "Primitive Render Farm
> An easy way to get multiple machines to share the rendering workload is to:
> Set up a shared directory over a network file system.
> Disable Overwrite, enable Placeholders in the Render Output panel.
> Start as many machines as you wish rendering to that directory."

Manual, Output properties, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

The matching properties, quoted from the API documentation (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30):

> "use_overwrite / Overwrite existing files while rendering / Type: boolean, default True"

> "use_placeholder / Create empty placeholder files while rendering frames (similar to Unix 'touch') / Type: boolean, default False"

I also opened the picture on that manual page, `_images/render_output_properties_output_panel.png`, referenced at the top of the Output page. **LABEL: documented** (it is part of the manual). It shows the Output panel with a file path field reading `/tmp/`, and a row of checkboxes: Overwrite (ticked) and Placeholders (unticked), alongside File Extensions (ticked) and Cache Result (unticked). That confirms both switches are present together in the panel and shows their state in that screenshot. I read nothing further into it.

Note carefully what this passage does and does not say. It says **machines**, not Blenders on one machine, and it concerns two renderers colliding over **output frames**, not over the `.blend` being read. Whether the same trick works for several Blenders on a single computer is not stated, and I do not extend it.

**Not covered.** This answer does not settle: any maximum number of Blenders per computer; what happens if a second Blender opens a `.blend` that a first is currently rendering; whether Blender takes any lock on a `.blend` while it is open or rendering; whether a `.blend` is read once at load and thereafter not touched, or re-read during a render; what happens if a `.blend` is opened while it is being written by something else; whether two Blenders sharing a temporary directory collide over render layers, physics cache or the copy-paste buffer; whether two Blenders quitting at different times overwrite each other's preferences; and whether the Placeholders and Overwrite arrangement behaves the same for several Blenders on one computer as it does for several machines on a shared drive.

**Outside the interface.** Partly holds, and the strongest evidence is itself from outside the interface. The statement that multiple instances can be started appears on the page about driving Blender as a Python module, which the same page describes as "For the most part using Blender as a Python module is equivalent to running a script in background-mode (passing the command-line arguments --background or -b)". `bpy.app.binary_path` is documented as being for "utilities that open new instances", which is script-driven, not clicked. Command-line rendering is documented throughout (https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30), including `blender -b file.blend -a`, and the security page notes such arguments "can be used to start a regular Blender instance". So the first half of the answer holds outside the interface. Two documented differences apply to the Python-module route specifically: "Functionality controlled by command line arguments (shown by calling blender --help aren't accessible)", with `--threads` named as an example having no equivalent; and "Blender's typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render and a crash log is not written in the event of a crash" (https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html, read 2026-07-30). The same page also warns: "It's possible other Python modules make use of the GPU in a way that prevents Blender/Cycles from accessing the GPU" — that is about other Python modules, not about another Blender, and I do not stretch it. One piece of my evidence is interface-only by its nature: the Asset Browser's "Open Blend File" is a button in a window. For the second half of the question the sources are silent inside the interface and outside it alike.

**Operating system.** The sources state no instance limit on any operating system, and no page singles out an operating system as behaving differently in this respect. I read all three launch pages — Windows, macOS and Linux (https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, .../macos.html, .../linux.html, read 2026-07-30) — and none mentions running a second copy, refusing a second copy, or any single-instance behaviour. Three documented operating-system differences sit nearby and are worth naming, though none of them is about how many Blenders may run: EEVEE's "Headless rendering is not supported on headless Windows systems" (https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html, read 2026-07-30), which bears on running a Blender with no display on Windows; the temporary-directory environment variables differ by operating system, as quoted above; and the user configuration directories differ by operating system while being per-version, so concurrently running Blenders of the same version on one machine resolve to the same configuration path — the directory-layout page gives `$HOME/.config/blender/4.5/` on Linux, `/Users/$USER/Library/Application Support/Blender/4.5/` on macOS and `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\4.5\` on Windows, and documents `BLENDER_USER_RESOURCES` as able "to be set to a custom directory to replace the default user directory". Whether concurrent Blenders tread on each other in that shared configuration directory is not stated.

**What the hand tests settled, on two machines.** The second half of this question has been tested on both machines the project has, and the two agreed. On the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — on 2026-08-04 and again on 2026-08-05; and on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — on 2026-08-06. In both places, with a 60-frame render of a file under way with no window, a second Blender opened the same file, printed its path and its three object names and quit with an exit code of 0; a third Blender opened a copy, renamed an object and saved over it mid-render and reported that it had finished; and in every case the render still finished with an exit code of 0, wrote all 60 frames, and printed no error and no warning. The full record of both runs, with the exact commands and the raw output, is in `Preparation/Blender Research/Findings/4.5/Reading a file.md` under question 2. The two are kept apart there and are not merged. So the second half is settled for macOS and for Linux, and **Windows stays open** because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**The first half is settled by neither machine, and by no machine.** Whatever number of Blenders a computer will start is a fact about that computer. The rented Linux machine cannot help: it is a container sharing someone else's hardware and does not own all of its own settings. One figure about it is recorded so nobody mistakes it for a Blender limit — the number of files one process on it may have open at once is **1024**, read with `ulimit -n`. That is a fact about the container.

### Where I looked

Searched the whole of `manual_4.5` and `api_4.5` at once, for every one of these terms, case-insensitively:

*Concurrency wording:* simultaneous, concurrent, "at the same time", "same time", "multiple instances", "another instance", "second instance", "single instance", "instances of Blender", "instance of Blender", "Blender instance", "multiple copies", "copies of Blender", "one Blender", "two Blender", "each instance", "per instance", "more than one", "one at a time", "only one", "at once", "in parallel", "as many", "as many as", "no limit", unlimited.

*File contention wording:* "file lock", "locked file", "lock the file", locking, "read-only", "read only", "in use by", "already open", "already in use", "exclusive access", "race condition", corrupt, conflict, clash, interfere, "data loss", "not safe", safely, "being written", "partially written", incomplete, "same file", "same blend", "another program", "modified externally", "changed on disk", "external program".

*How a file is held:* "into memory", "loaded into", "keeps the file open", "file open", "holds the file", "memory-map", mmap, streamed, "whole file", "re-read", reread, reload, "on demand", lazy, "kept open", "closed after".

*Running and processes:* "run Blender", "running Blender", "start Blender", headless, "batch render", queue, scheduler, spawn, fork, pid, "process id", "render farm", renderfarm, farm, placeholder, overwrite, "network render", distribute, "several machines", "multiple machines", "other computers", "same animation".

*Resources:* "out of memory", "shares the", "shared between", contention, compete, "same GPU", "same device", "while rendering", "during rendering", "during the render", "is rendering", unresponsive, blocked, "wait until".

*Temporary and configuration files:* "auto save", autosave, quit.blend, "temporary directory", "temp directory", BLENDER_USER_CONFIG, BLENDER_SYSTEM_RESOURCES, TEMP.

Pages opened and read, beyond the ones quoted above: Deploying Blender in Production; Blender's Directory Layout (in full); Recovering Data; Opening & Saving; Save & Load preferences; File Paths preferences; System preferences; Command Line Arguments; Rendering From The Command Line; the Windows, macOS and Linux launch pages; Cycles Performance settings; Cycles GPU Rendering (frequently asked questions); Rendering Animations; Areas; Status Bar; Asset Browser; Preferences introduction; Brush management; Pose copy/paste; the Glossary; `info_advanced_blender_as_bpy` (in full); `info_gotchas_threading`; `bpy.app`; `bpy.types.BlendDataLibraries`; `bpy.types.PreferencesFilePaths`; `bpy.types.PreferencesSystem`; `bpy.types.RenderSettings`; `bpy.types.RenderEngine`; `bpy.ops.asset`.

Release notes: grepped all nineteen 4.5 subject files for instance, concurren, simultan, parallel, lock, multiple, background and process. Nothing about running several Blenders at once, about file locking, or about reading a file another Blender is using.

Cycles' own files: grepped every Python file in `cycles_4.5` for thread, lock, temp, process, instance, concurren and simultan. The only matches were a temporary path swap in `operators.py`, temporary `.oso`/`.osl` compile files in `osl.py`, the `threads_mode` preset entry in `presets.py`, and the Threads panel in `ui.py`. Nothing about concurrent Blenders or file locking. `properties.py` line 499 describes Sample Subset as "Render a subset of the specified max samples. Typically used for distributed rendering across multiple devices" — that is about splitting one frame's samples across devices, not about several Blenders on one computer, so I set it aside.

Pictures: I checked which images the relevant pages reference and opened the one that could bear on the answer, the Output panel screenshot, reported above. The other candidates — the Status Bar right-side screenshot, the System preferences screenshot, the three launch-page console screenshots and the Asset Browser screenshots — show panels and terminals, and none of them could show how many Blenders may run or what happens when two touch one file, so I did not treat them as evidence.

Ruled out on inspection, and why:

- `bpy.types.RenderEngine` — "Multiple instances may exist at the same time, for example for a viewport and final render." This is about render-engine objects inside one Blender, a different sense of "instance" entirely. Not evidence about Blender processes.
- Shader Compilation Method in System preferences — "Uses multiple separate subprocesses to compile shaders in parallel", with a Threads / Subprocesses count "limited by the number of logical CPU cores on the system", and the 4.5 EEVEE release note that "A new user preference setting was added to choose between multi-threaded and subprocess compilation". These subprocesses compile shaders for one Blender; they are not separate Blenders reading or rendering files. Not an answer to the count.
- "Lock interface during rendering in favor of giving more memory to the renderer" (Topbar, and `RenderSettings`) — a lock on the interface within one Blender, not a lock on a file.
- Text Editor "Resolve Conflict … Resolves modified file conflicts when an external text file is updated from another program" — concerns text data-blocks and external text files, not `.blend` files, and not another Blender.
- EEVEE "Multiple GPU Support: There is currently no support for multiple GPU systems" and Cycles' "Can multiple GPUs be used for rendering? Yes" — about one Blender using several graphics cards, not about several Blenders.
- Cycles Sample Subset and the "Primitive Render Farm" hint — both about spreading work across **machines**. Kept as context, not counted as an answer about one computer.

---

## Things a person with Blender open could settle by hand in a few minutes

Precise steps, for someone who cannot see my reasoning. This list was written before its author knew the hand tests had already run. Items 3 and 4 have since been settled on both machines the project has, and are struck from the list; what they found is recorded above and in `Preparation/Blender Research/Findings/4.5/Reading a file.md` under question 2. Item 2 cannot be settled by any test, for the reason given above.

1. **Whether a second Blender can be started at all, per operating system.** Start Blender. Without closing it, start it again — on Windows from a Command Prompt with `blender.exe`, on Linux from a terminal with `./blender`, on macOS from a terminal with `/Applications/Blender.app/Contents/MacOS/Blender`. Note whether a second window appears or whether the existing one merely comes to the front. Repeat on each operating system available. On macOS also try double-clicking the application icon a second time and compare with the terminal result, since the two launch routes may differ. Only Windows is left here: several Blenders were started at once, with no window, on both macOS and Linux during the hand tests.

2. **Whether many can run at once.** From a terminal, start ten Blenders in background mode on a small scene: `blender -b test.blend -f 1 -o /tmp/out_1_####` and so on, each with its own output name, all at the same time. Note whether all ten start, whether any prints a refusal, and what the machine does. Raise the number until something breaks and record what breaks and what it says. **Not run, and not worth running:** whatever number one computer reaches is a fact about that computer.

5. **Whether the `.blend` is still needed after loading.** Start a long background render of `test.blend`. While it runs, rename or move `test.blend` on disk. Record whether the render carries on to completion or fails, and what it prints.

6. **Whether concurrent Blenders collide in the temporary directory.** Open the same `.blend` in two interactive Blenders at once, both with Auto Save on and both using the same temporary directory (leave Preferences ‣ File Paths ‣ Temporary Files blank in both). Wait past the auto-save interval, then list the temporary directory and record how many auto-save files appear and what they are named. This directly tests the process-ID naming quoted above.

7. **Whether concurrent Blenders collide over preferences.** Open two Blenders. In the first, change a preference; in the second, change a different one. Close the first, then the second. Reopen Blender and record which of the two changes survived.

8. **Whether Placeholders and Overwrite work for several Blenders on one computer.** In `test.blend`, set Output to a local folder, disable Overwrite and enable Placeholders. Start three background animation renders of that file at the same time on the one computer: `blender -b test.blend -a`, three times. When they finish, check whether every frame in the range exists exactly once, whether any frame is missing, and whether any frame was rendered twice.

---

2026-08-05 — Opus 5 (Claude Code) — Reading a file, Blender 4.5

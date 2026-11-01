# Reading a file — Blender 5.1 — third pass

**Sources consulted:** Blender 5.1 manual, Blender 5.1 Python API documentation, Blender 5.1 release notes and the Cycles 5.1 files, all in `Blender Sources/` and all taken on **2026-07-30** (my date read for all four). Also Blender's requirements page, taken on **2026-08-01** (my date read for it).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 2 was assigned to me. It is the only question answered below.

---

## 2. How many Blenders can be at work on one computer at the same time, and does one reading a file while another renders cause trouble?

### A note on the reading I took

Two small ambiguities, flagged rather than silently resolved.

"How many Blenders" — I read this as *how many copies of Blender can be running at the same moment on one machine*. The other reading is *how many versions of Blender can be installed side by side*. I answer the first, and because the second is cheap to settle I give it in one line at the end of the "how many" part.

"One reading a file while another renders" — I read this as *one Blender reading the same `.blend` that a second Blender is rendering from*. The wider reading is *any two Blenders on one machine, one of them doing a read, the other rendering*. I answer the narrow reading and say where the wider one differs.

---

### Part one — how many can be at work at the same time

**More than one can. That is stated outright, in several places.**

**LABEL: documented**

The API documentation, in its page on running Blender as a Python module, says plainly that several can be started:

> "Only a single .blend file can be edited at a time."
>
> "As with the application it's possible to start multiple instances, each with their own bpy and therefore Blender state.
> Python provides the multiprocessing module to make communicating with sub-processes more convenient."

Source: Python API documentation, https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — read 2026-07-30.

The first sentence is a limit *inside* one Blender: one file open per running Blender. The second sentence is the answer to this question: you get more files open by starting more Blenders, and each carries its own state.

The manual describes Blender starting a second Blender itself, and the first one carrying on while it runs:

> "A new Blender process will start, and automatically open the asset library
> file that contains the chosen pose."

and, three steps later:

> "The original Blender is still running in the background and notices that the new Blender has quit.
> It automatically refreshes the Asset Browser to show the newly added pose."

Source: manual, https://docs.blender.org/manual/en/5.1/animation/armatures/posing/editing/pose_library.html — read 2026-07-30.

The same behaviour is described from the Asset Browser side:

> "Opens the blend-file that contains the asset in a new Blender instance.
> When this instance is closed, the Asset Browser will be automatically refreshed."

Source: manual, https://docs.blender.org/manual/en/5.1/editors/asset_browser.html — read 2026-07-30.

The Python API says the executable's own path is exposed for exactly this purpose:

> "The location of Blender's executable, useful for utilities that open new instances."

Source: Python API documentation, `bpy.app.binary_path`, https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30.

Two more places treat several running Blenders as an ordinary state of affairs. The manual, on nodes:

> "Nodes can be copied and pasted between Blender instances."

Source: manual, https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html — read 2026-07-30.

and, on copying UVs, the same assumption stated as a limitation:

> "The Copy UVs tool currently uses an internal clipboard which is not shared between instances of blender."

Source: manual, https://docs.blender.org/manual/en/5.1/modeling/meshes/uv/editing.html — read 2026-07-30.

The 5.1 release notes record the node clipboard change as new in this version:

> "Copy-paste between Blender instances is now possible, also works between different types of node editors."

and

> "Cut-paste is now supported, also between Blender instances (No shortcut assigned by default)."

Source: release notes, https://developer.blender.org/docs/release_notes/5.1/user_interface/ — read 2026-07-30.

**How many, as a number: not in the sources.**

**LABEL: not in the sources**

No page states a maximum, a recommended maximum, or any count at all. Where a count would naturally appear it does not. The manual's "Working Limits" page (https://docs.blender.org/manual/en/5.1/advanced/limits.html) lists limits on space, time and text-field lengths and says nothing about processes. The requirements page names minimum and recommended CPU, RAM and GPU for one machine and never mentions running more than one copy — and in any case it names Blender 4.5 LTS, 5.0, 3.6, 3.4, 2.92, 2.90, 2.83, 2.79 and 2.76, and never names 5.1, so it does not speak for the version being researched here. Blender's licence page and the install pages set no such limit either.

The closest thing to a count anywhere in the sources is about machines, not copies on one machine, and it is unbounded:

> "Start as many machines as you wish rendering to that directory."

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

**What actually caps the number: inferred, and the step is mine.**

**LABEL: inferred**

The sources say a render "takes all available CPU time", that running out of memory is the most common cause of a crash, and that a GPU can only use its own memory and that overrunning it can take down the driver. Nothing in them says "therefore the number of Blenders you can run is set by RAM, GPU memory and cores" — that sentence is me joining those facts up, and a reader has to supply the joining, so this is inferred and not documented. The quotes it rests on:

> "Rendering takes all available CPU time; you should render overnight,
> when the computer is not needed, or set Blender to a low priority while rendering,
> and work on other things (be careful with the RAM space!)."

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

> "The most common causes of Blender crashes:
> Running out of memory."

Source: manual, https://docs.blender.org/manual/en/5.1/troubleshooting/crash.html — read 2026-07-30.

> "Typically, the GPU can only use the amount of memory that is on the GPU […] This is usually much smaller than the amount of system memory the CPU can access."

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html — read 2026-07-30.

> "In practice, using too much GPU memory can make the GPU driver crash, freeze, or kill the application.
> So be careful of what you ask."

Source: manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html — read 2026-07-30.

Blender does give handles for sharing one machine's processor between whatever is running on it, which is consistent with several copies at once but does not state it:

> "-t, --threads <threads>
>
> Use amount of <threads> for rendering and other operations
> [1-1024], 0 to use the systems processor count."

and

> "--qos <level>
>
> Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).
>
> default: Uses the default behavior of the OS.
> high: Always makes use of performance cores.
> eco: Schedules Blender threads exclusively to efficiency cores."

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

And in the interface, the same idea:

> "Manually choose the maximum number threads to use for rendering.
> This can be useful for example, if you want to use your computer while rendering,
> you can set the property to a thread count lower the amount of logical processors on your computer."

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html — read 2026-07-30.

**The other reading of "how many Blenders" — versions installed side by side.**

**LABEL: documented.** Several versions may be installed at once, on all three systems. Windows: "Note, you do not have to overwrite your existing Blender installation. It's perfectly possible to have multiple versions installed side by side." (https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html). Linux: "When using this method of installation, it is possible to have multiple versions of Blender installed." (https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html). macOS: "You can rename Blender.app or place it in a different folder to have more than one version at a time." (https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html). All read 2026-07-30. Each version keeps its own user and system directory named for the version number (https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html).

---

### Part two — does one reading a file while another renders cause trouble?

**The sources never address this. That is the honest answer.**

**LABEL: not in the sources**

Nothing in the 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes or the Cycles 5.1 files says what happens when one Blender opens a `.blend` while another Blender is rendering from it. There is no statement that Blender locks a `.blend` while it holds it, no statement that it does not, no warning against it, and no blessing of it. The record of where I looked is below, and it is the substance of this answer.

What the sources *do* say, which is next door to the question but does not settle it:

**1. Every running Blender shares one temporary directory, chosen the same way for each.**

**LABEL: documented**

> "The temporary directory is used to store various files at run-time
> (including render layers, physics cache, copy-paste buffer and crash logs)."
>
> "The temporary directory is selected based on the following priority:
> User Preference (see File Paths).
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
> The /tmp/ directory."

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30.

So render layers from a render, and the file a reading Blender might write, land in the same place by default. Whether that is a problem is not stated.

**2. The one collision the sources do address, they address by process ID.**

**LABEL: documented**

> "use_auto_save_temporary_files
>
> Automatic saving of temporary files in temp directory, uses process ID.
> Warning: Sculpt and edit mode data won't be saved
>
> (default True)"

Source: Python API documentation, `bpy.types.PreferencesFilePaths`, https://docs.blender.org/api/5.1/bpy.types.PreferencesFilePaths.html — read 2026-07-30.

This is the only place in any of my sources where Blender is documented as distinguishing itself from another running Blender. It names the mechanism — the process ID — and it names it for auto-save only. The manual's account of the same feature does not mention the process ID, and describes the names loosely: "Auto-saved files typically have a name such as <filename>_autosave.blend or a random identifier" (https://docs.blender.org/manual/en/5.1/files/blend/open_save.html).

**3. Blender's own memory readout is explicitly qualified to the case of one Blender on one machine.**

**LABEL: documented**

> "System Memory
>
> Shows an estimate of Blender's RAM consumption. On a single-instance single-machine scenario,
> this estimate provides a measurement against the hardware limit of the machine."

Source: manual, https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html — read 2026-07-30. The same sentence appears again at https://docs.blender.org/manual/en/5.1/editors/preferences/interface.html.

I also opened the picture the manual places beside that text, `_images/interface_window-system_status-bar_ui-right.png` (from the status bar page above). It shows the right-hand end of the status bar reading `Collection | Cube | Verts:8 | Faces:6 | Tris:12 | Objects:0/3 | Mem: 29.7 MiB | v2.81.16` — so the memory figure is a plain running total shown in the bar. **LABEL for the picture: documented.** Note the screenshot carries an older version string than 5.1; what I take from it is only that the figure is displayed there.

**Reading anything more into that qualifier is inferred, and here is the step.** The manual says the estimate measures against the machine's hardware limit *in the single-instance case*. It does not say what the estimate does in any other case. My step: if the qualifier were doing no work, it would not be there; so with a second Blender running, one Blender's figure no longer tells you where the machine as a whole stands. A reader has to supply that step, so: **LABEL: inferred**. What it does *not* tell you is whether anything goes wrong — only that this particular readout stops being a reading of the machine.

**4. Where the sources do expect several renderers at once, they expect them on separate machines, and they hand you a mechanism to stop them colliding over output.**

**LABEL: documented**

> "Hint
>
> Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

The mechanism, in the API:

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

and

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

Source: Python API documentation, `bpy.types.RenderSettings`, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

This is about several renderers not treading on each other's *output files*. It says nothing about several Blenders and one *input* `.blend`. The recipe is written for machines; the sources never say whether the same recipe works for several Blenders on one machine — that would be an inference and I am not making it.

Cycles has a second such arrangement, again written for machines:

> "A typical use case is to distribute the rendering of a single frame over multiple machines."

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

**5. A Blender that is only reading can be told to skip some of the work of loading.**

**LABEL: documented**

> "--disable-depsgraph-on-file-load
>
> Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
> when loading a blend-file in background mode (-b or -c options)."
>
> "NOTE: this is a temporary option, in the future depsgraph will never be
> automatically generated on file load in background mode."

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

The sources do not say how much work this saves, so this does not carry any claim that a reading Blender is light enough not to disturb a render.

**6. Contention over the graphics card is documented, but only between rendering and drawing, never between two Blenders.**

**LABEL: documented**

> "On older GPU generations, graphics cards can only either render or draw the user interface.
> This can make Blender unresponsive while it is rendering."
>
> "The only complete solution for this is to use a dedicated GPU for rendering, and another for display."

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html — read 2026-07-30.

And, in the API's page on Blender as a Python module, the nearest thing to an acknowledgement that something else on the machine can take the card away from Cycles:

> "Resource Sharing (GPU)
>
> It's possible other Python modules make use of the GPU in a way that prevents Blender/Cycles from accessing the GPU."

Source: Python API documentation, https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — read 2026-07-30. That sentence is about other Python modules in the same process, not about a second Blender.

**7. One more shared writable thing, for completeness.**

**LABEL: documented.** Every Blender of the same version reads and writes the same user directory, and preferences are written on exit by default: "By default changes to preferences are saved on exit" (https://docs.blender.org/manual/en/5.1/editors/preferences/introduction.html, read 2026-07-30). The user directory also holds `config/recent-files.txt`, "Recent file menu list" (https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html). The sources never say what happens when two running Blenders write these at once.

### Where I looked, for the "not in the sources" claim

Everything below was searched across the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1`, `cycles_5.1` and the requirements page unless narrowed, using whole-tree text search rather than opening files one at a time.

**Words for a lock or exclusive access:** `file lock`, `lock the file`, `locks the file`, `locking`, `exclusive access`, `exclusive lock`, `flock`, `read-only`, `read only`, `write access`, `permission denied`, `in use by`, `already open`, `being used by`, `file is open`, `held open`, `opened by`, `does not lock`, `no lock`, `file handle`, `open file descriptor`. Result: no hit anywhere is about one program holding a `.blend` against another. The `read-only` hits are about linked library data, extension repositories, and API properties. `Locked` hits are the Locked Track constraint and transform locks.

**Words for two things happening at once:** `simultaneous`, `simultaneously`, `concurrent`, `concurrently`, `in parallel`, `at the same time`, `at once`, `two instances`, `both instances`, `each instance`, `per instance`, `multiple instances`, `single instance`, `single-instance`, `single machine`, `single-machine`, `multi-instance`, `another instance`, `second instance`, `separate instance`, `new instance`, `instance of Blender`, `Blender instance`, `multiple copies`, `another copy`, `another process`, `running Blender`, `Blender is running`, `already running`, `only one Blender`, `a single Blender`, `one Blender at a time`, `as many … as you`, `any number of`. The hits that matter are all quoted above; the rest are about geometry instancing, node instances, and grease-pencil "Concurrent" build mode.

**Words for the trouble it might cause:** `corrupt`, `data loss`, `race condition`, `conflict`, `atomic`, `partially written`, `incomplete file`, `half-written`, `written to a temporary`, `safe to`, `is safe`, `not safe`, `thread-safe`, `thread safe`, `reentrant`, `shared state`, `shared resource`, `resource sharing`. Nothing about two Blenders. `Conflict` hits are keymap conflicts, node socket conflicts, material-name conflicts on import, and the Text Editor's "Resolve Conflict" — that last one is about an *external text file* changed by another program while Blender has it open, which is the nearest analogue in the whole documentation and is not about `.blend` files.

**Words for the machinery two Blenders would share:** `temporary directory`, `temp directory`, `temporary files`, `quit.blend`, `autosave`, `auto save`, `process ID`, `PID`, `IPC`, `daemon`, `watchdog`, `socket`, `port`, `server`, `job queue`, `queue up`, `mutex`, `singleton`, `subprocess`, `multiprocess`. The useful hits are quoted above. `daemon` is the 3D-mouse daemon socket and a Python example; `server` and `port` in the Cycles files are absent — the Cycles files supplied here are `__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py` plus `shader/` and a `source/` tree that holds only `util/` and `kernel/` headers. There is no session or device-management code in them, so they carry nothing on this question.

**Words for several renderers:** `render farm`, `renderfarm`, `network render`, `distributed render`, `batch render`, `farm`, `multiple machines`, `several machines`, `other machines`, `render nodes`, `multiple computers`, `different computers`, `another machine`, `across machines`, `multiple systems`, `network file system`, `shared directory`, `network drive`, `network share`, `NFS`, `samba`, `SMB`, `placeholder`, `overwrite`. Two real hits, both quoted above, both about *machines*.

**Words for the act itself:** `while rendering`, `while a render`, `during rendering`, `during a render`, `while Blender is rendering`, `another program`, `other program`, `external program`, `outside of Blender`, `outside Blender`, `changed on disk`, `modified externally`, `externally modified`, `loaded into memory`, `into memory`, `keeps the file open`, `holds the file`.

**Pages opened in full and ruled out**, each because it is where such a statement would sit and it is not there:

- Working Limits — https://docs.blender.org/manual/en/5.1/advanced/limits.html — limits on space, time, text-field lengths. Nothing on processes.
- Blender's Directory Layout — https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read whole. Names the shared directories, never says what happens when two Blenders use them at once.
- Opening & Saving — https://docs.blender.org/manual/en/5.1/files/blend/open_save.html — read whole. Nothing on opening a file another program has.
- Rendering From The Command Line — https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read whole. Nothing on running several.
- Command Line Arguments — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — searched whole; `-b`, `-t`, `-c`, `--qos`, `TEMP`, `TMPDIR`, `--disable-depsgraph-on-file-load` all read in place.
- Launching from the Command Line, and its Linux / macOS / Windows pages — https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/index.html and the three pages under it — read. Nothing about starting a second one.
- Troubleshooting: Startup, Crashes, Network, Recovering Data — https://docs.blender.org/manual/en/5.1/troubleshooting/startup.html, .../crash.html, .../network.html, .../recover.html — read whole. Startup lists the causes of a failure to start and a second running Blender is not among them.
- Crashes (again, for the crash log) — the crash file is named from the loaded blend-file on Windows (`test.crash.txt`) and `blender.crash.txt` in `/tmp` on Linux. Fixed names in a shared directory; the sources never say what two Blenders crashing would do to them.
- Installing on Windows / Linux / macOS — the three pages under https://docs.blender.org/manual/en/5.1/getting_started/installing/ — read.
- Preferences: Save & Load, File Paths, Interface, System, Introduction — https://docs.blender.org/manual/en/5.1/editors/preferences/ — read the relevant panels.
- Glossary, entry "Blender Session" — https://docs.blender.org/manual/en/5.1/glossary/index.html — "The timespan of a Blender instance. The session begins with starting an instance of Blender and ends with closing it." A definition, not a rule about how many.
- Python API: Blender as a Python Module, Python Threads are Not Supported, Tips and Tricks, `bpy.app`, `bpy.types.RenderSettings`, `bpy.types.PreferencesFilePaths`, `bpy.types.BlendData` (including `temp_data()`), `bpy.types.BlendDataLibraries` (including `load()`) — https://docs.blender.org/api/5.1/. The threading page is about Python threads *inside* one Blender ("Python threads cause Blender to crash in hard to diagnose ways"), and its recommended alternative is separate processes: "For running Python code independently of Blender, it is recommended to use the multiprocessing module." It says nothing about two Blenders and one file.
- All 21 release-note files for 5.1, swept for `instance`, `process`, `parallel`, `concurren`, `simultan`, `thread`, `lock`, `background`, `command line`, `multi-process`, `subprocess`, and file read/load/open. The only relevant entries are the two copy-paste-between-instances lines quoted above.
- Requirements page, `requirements/requirements_2026-08-01.txt`, read whole, read 2026-08-01. Gives per-machine minimums and recommendations and says nothing about running more than one copy. It names 4.5 LTS, 5.0, 3.6, 3.4, 2.92, 2.90, 2.83, 2.79, 2.76 — never 5.1 — so it cannot be leaned on for 5.1 at all.

### Not covered

- No number. The sources give no maximum, no recommended maximum and no rule of thumb for how many Blenders one machine should run.
- No verdict on the actual question of harm. Whether a second Blender reading a `.blend` disturbs a first Blender rendering from it — in either direction, and whether the harm would be a failed read, a failed render, a wrong result or nothing at all — is simply not in these sources.
- No statement either way on whether Blender holds a `.blend` open after loading it, or reads it once and lets go. Answering this question properly turns on that, and the sources do not settle it.
- Saving is untouched. Everything above is about *reading*. What happens if the second Blender *saves over* the file the first is rendering from is not addressed anywhere I looked, and I would not extend any of the above to it.
- The wider reading of the question — two Blenders on one machine, one reading some unrelated file, one rendering — is covered only by the shared-resource facts above (temp directory, memory, cores, graphics card). No source states an interaction.
- The "single-instance single-machine" qualifier tells you the memory readout is narrower than it looks. It does not tell you anything goes wrong.
- The render-farm recipe (Placeholders on, Overwrite off) is documented for machines. I do not claim it transfers to several Blenders on one machine; the sources do not say.
- I ran nothing. Every finding here is read, not observed. There is no Blender 5.1 among the files I was given — only Cycles' script files and headers — so nothing here could be tested.

### Outside the interface

Mostly yes, and in one place the outside-the-interface case is the *only* case documented.

- "As with the application it's possible to start multiple instances" is written on the page about Blender driven with no interface at all, as a Python module. So the claim that several can run at once is documented for the driven-from-outside case directly, not carried over from the interface. Same page, same address as above. **documented**
- `bpy.app.binary_path` exists "for utilities that open new instances" — a script starting further Blenders, which is the outside-the-interface case. **documented**
- Background mode is the documented way to render without an interface: "-b, --background — Run in background (often used for UI-less rendering)", and "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)." (https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html) **documented**
- Where it differs, documented: the node copy-paste between instances is an interface feature and has no meaning headless. Audio differs too — "The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards" (command line arguments page) — which matters if several headless Blenders would otherwise each want the machine's audio device. `--disable-depsgraph-on-file-load` applies only in background mode. As a Python module specifically, "Functionality controlled by command line arguments (shown by calling blender --help) isn't accessible" — `--threads` is named as one with no Python equivalent, so a Blender driven that way cannot be told how many cores to take. **documented**
- On the second half of the question the answer is the same outside as inside: the sources do not say, so they do not say it for either case. **not in the sources**

### Operating system

The fact that several can run holds without an operating system attached to it — none of the statements quoted above is qualified by system. The machinery around it is not uniform:

- The user directory differs per system: `$HOME/.config/blender/5.1/` on Linux (or `$XDG_CONFIG_HOME`), `/Users/$USER/Library/Application Support/Blender/5.1/` on macOS, `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.1\` on Windows. Every Blender of that version on that machine uses it. **documented**
- The temporary directory differs per system, and the two pages that describe it do not agree on the variable names. The directory layout page says "Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms)". The command line arguments page says "TEMP: Store temporary files here (MS-Windows)" and "TMPDIR: Store temporary files here (UNIX Systems)". I report both as they stand rather than pick one. **documented, and inconsistent between the two pages**
- Auto-save location differs: "Autosave blend-file location. (Windows only, temp directory used for other systems.)" **documented**
- `--qos`, the setting for placing threads on performance or efficiency cores, is "Windows only". **documented**
- Crash logs differ per system, with fixed names in shared places on Windows and Linux. **documented**
- Having more than one version at a time is documented separately for each of the three systems, with a different method on each (see above). **documented**
- Not in the sources: whether the *way you launch* a second Blender differs per system — in particular whether double-clicking Blender.app twice on macOS starts a second process or brings the first forward. The macOS launch page only shows running the executable inside the bundle from a terminal, and says nothing about a second one. This is on the hand-check list below.

**What the hand tests settled, on two machines.** The second half of this question has been tested on both machines the project has, and the two agreed. On the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — on 2026-08-04 and again on 2026-08-05; and on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — on 2026-08-06. In both places, with a 60-frame render of a file under way with no window, a second Blender opened the same file, printed its path and its three object names and quit with an exit code of 0; a third Blender opened a copy, renamed an object and saved over it mid-render and reported that it had finished; and in every case the render still finished with an exit code of 0, wrote all 60 frames, and printed no error and no warning. The full record of both runs, with the exact commands and the raw output, is in `Preparation/Blender Research/Findings/5.1/Reading a file.md` under question 2. The two are kept apart there and are not merged. So the second half is settled for macOS and for Linux, and **Windows stays open** because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**The first half is settled by neither machine, and by no machine.** Whatever number of Blenders a computer will start is a fact about that computer. The rented Linux machine cannot help: it is a container sharing someone else's hardware and does not own all of its own settings. One figure about it is recorded so nobody mistakes it for a Blender limit — the number of files one process on it may have open at once is **1024**, read with `ulimit -n`. That is a fact about the container.

---

## Things a person with Blender 5.1 open could settle in a few minutes

Each of these is a gap above that a short sitting at a computer would close. They are written to be carried out without reading anything I wrote. This list was written before its author knew the hand tests had already run. Items 4 and 6 have since been settled on both machines the project has and are struck from the list; what they found is recorded above and in `Preparation/Blender Research/Findings/5.1/Reading a file.md` under question 2. Items 12, 13 and 15 turn on timing or on filling a graphics card, so they need a machine to themselves and were not run on the shared rented one.

1. **Does a second Blender start at all, from the terminal?** Open two terminals. In each, run the Blender executable with no arguments (Windows: `blender.exe`; macOS: `/Applications/Blender.app/Contents/MacOS/Blender`; Linux: `./blender`). Note whether two Blender windows appear, and whether the machine's task manager / Activity Monitor / `top` shows two Blender processes. Try a third and a fourth.

2. **Does a second Blender start from the desktop, on macOS?** Double-click Blender.app in Finder. With it already running, double-click it again. Record whether a second window and a second process appear, or whether the running one is merely brought forward. Then run `open -n -a Blender` in a terminal and record whether *that* produces a second process. Report both results — they may differ.

3. **Does one Blender refuse a `.blend` another has open?** Open `test.blend` in Blender A. In Blender B, open the same `test.blend`. Record whether it opens, warns, or refuses, and quote any message exactly. Then in Blender B move an object and save. Record what Blender A does — nothing, a warning, anything.

5. **Does a read disturb a render on the other side?** Same as above but start the read first on a very large file, and start the render while the read is loading. Record whether the render's start-up is delayed or fails.

7. **What lands in the temporary directory when two run?** Find the temporary directory (Preferences ▸ File Paths ▸ Temporary Files, or the system default). Start two Blenders, both with unsaved changes, and wait past the auto-save interval (default two minutes). List the directory and record the exact file names. Confirm whether the auto-save names contain each Blender's process ID, and whether the two overwrote each other.

8. **What happens to `quit.blend` when two quit?** With two Blenders running and different unsaved scenes in each, quit the first, note the `quit.blend` in the temporary directory and its timestamp, then quit the second. Record whether the second overwrote the first, and which scene File ▸ Recover ▸ Last Session then brings back.

9. **Does giving each its own temporary directory change anything in 7 and 8?** Repeat both with each Blender started with a different `TEMP`/`TMPDIR` set in its environment, or with different Temporary Files preferences. Record any difference.

10. **What do two Blenders do to the preferences file?** Change a preference in Blender A and a different one in Blender B, then quit A, then quit B. Restart and record which change survived.

11. **Is the status bar memory figure a machine figure or a Blender figure?** Enable the status bar's System Memory readout (right-click the status bar). Run two Blenders with large scenes and compare each one's figure against the machine's total in Task Manager / Activity Monitor / `top`. Record whether the two figures sum to the machine's Blender total or whether either claims to be the whole.

12. **Two renders on one graphics card.** Set both Blenders to GPU rendering with the same card and start a heavy render in each at the same time. Record whether both complete, whether either reports an out-of-memory error, and whether the display or driver misbehaves. Then repeat with a scene sized so that two copies of it would not both fit in the card's memory, and record what happens.

13. **Two renders on one processor.** Start two headless CPU renders of the same scene at once and compare each one's per-frame time against the same render run alone. Repeat with `-t` set to half the logical processors on each.

14. **Does the render-farm recipe work within one machine?** Turn Overwrite off and Placeholders on, set the output to one folder on a local drive, and start two headless animation renders of the same file at the same time on the same machine. Record whether the two divide the frames between them, or whether both render the same frames, or whether any frame comes out damaged.

15. **How cheap is a read?** Time `blender -b big.blend --python-expr "pass"` against `blender -b big.blend --disable-depsgraph-on-file-load --python-expr "pass"` on a large file, several runs each. Record both timings and the peak memory of each.

---

2026-08-05 — Opus 5 (Claude Code) — Reading a file, Blender 5.1

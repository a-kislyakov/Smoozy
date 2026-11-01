# The machine — second pass, Blender 5.0

**Sources consulted, and when.**

- Blender 5.0 manual (plain text plus one picture from `_images`) — read 2026-07-30.
- Blender 5.0 Python API documentation (plain text) — read 2026-07-30.
- Blender 5.0 release notes — read 2026-07-30.
- Cycles' own files as shipped with Blender 5.0 (`cycles_5.0`) — read 2026-07-30.
- Blender's requirements page, the copy taken on 2026-08-01 — read 2026-08-01.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Answering questions 2, 3, 4, 5 and 6 only. Numbers below match the sheet.

A note on how I searched, once, so I do not repeat it in every answer. The manual and the API documentation were searched whole, by text, before any file was opened. Search terms used across the five questions: `CUDA`, `OptiX`, `HIP`, `oneAPI`, `Metal`, `GPU`, `device`, `compute device`, `render device`, `GPU Compute`, `driver`, `requirement`, `minimum requirements`, `Vulkan`, `OpenGL`, `snap`, `flatpak`, `steam`, `Microsoft Store`, `package manager`, `portable`, `startup`, `crash`, `error message`, `unsupported graphics`, `no device`, `device not`, `fall back`, `fallback`, `falls back`, `silently`, `exit code`, `return code`, `enumerate`, `list devices`, `query device`, `which GPU`, `get_devices`, `refresh_devices`, `available_devices`, `compute_device_type`, `CyclesPreferences`, `CyclesRenderSettings`, `System Info`, `NVLink`, `CYCLES_` (environment-variable prefix). Where a search found nothing, that is recorded under the question it belongs to.

---

## 2. What must a computer have on it before Blender will render using its graphics cards?

**Answer.** Five things, and the sources state them separately.

**(a) A card of a supported make and generation, matched to a supported operating system.** The manual gives one section per technology:

> "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher."

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."

> "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."

> "oneAPI is a computation library that is supported on Windows and Linux and requires a Intel® Arc™ graphics card with the Xe HPG architecture."

> "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

**(b) A driver at or above a stated version.** For AMD:

> "Minimum driver versions:
> Windows: Radeon Software 24.9.1 or Radeon PRO Software 24.Q4
> Linux: Radeon Software 23.40 or ROCm 6.0"

For Intel:

> "Minimum driver versions:
> Windows: Intel Graphics Driver XX.X.101.5518
> Linux: intel-level-zero-gpu package 1.3.27642, typically available through the intel-compute-runtime package XX.XX.34666.3"

For NVIDIA under OptiX, the driver floor is inside the quote in (a): "a driver version of at least 535".

The release notes state one of these floors as a change made in this version:

> "* The minimum supported compute capability is `sm_50` now (~GeForce 900 cards). (blender/blender!142212)
> * The minimum required Windows AMD GPU driver has increased to Adrenalin version 24.9.1 or Radeon Pro driver 24.Q4 (blender/blender!148305)"

**(c) Manufacturer drivers, not the ones the computer came with.**

> "In case of problems, be sure to install the official graphics drivers from the GPU manufacturers website, or through the package manager on Linux. The graphics drivers provided by the computer manufacturer can sometimes be outdated or incomplete."

**(d) Nothing else installed alongside — no separate toolkit.**

> "Normally users do not need to install the CUDA toolkit as Blender comes with precompiled kernels."

**(e) Three settings, which the manual treats as part of the requirement rather than as a separate step.**

> "To enable GPU rendering:
> Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.
> Enable the available devices for the chosen backend.
> Configure each scene to use the GPU Compute Device."

Two further points that belong here.

The Cycles add-on's own files carry the same requirement figures, and in two places the figures differ from the manual's. This is what the file says, quoted exactly:

> `col.label(text=rpt_("Requires Apple Silicon with macOS %s or newer") % mac_version,` — with `mac_version = "12.2"` set two lines above.

The manual's Metal section says "macOS 13.0 or newer is required to support all features." For Intel on Windows the add-on file sets `driver_version = "XX.X.101.8132"` where the manual says "XX.X.101.5518". I report both readings without choosing between them.

The requirements page carries a line about Cycles but names no version:

> "Rendering in Cycles using the GPU has specific requirements described in the manual. Always make sure to install the latest drivers from the graphics card manufacturer website."

Because that sentence names no version, I do not read it as a statement about 5.0; it points back to the manual in any case. The one place the requirements page does name this version is the macOS note, quoted under question 3.

**LABEL: documented** for (a) through (e); **observed** for the Cycles add-on figures.

**Sources and addresses.**
- Manual, GPU Rendering — https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.
- Release notes, Cycles, "GPU" section — https://developer.blender.org/docs/release_notes/5.0/cycles/ — read 2026-07-30.
- Cycles add-on file `cycles_5.0/properties.py`, inside `CyclesPreferences._draw_devices`, lines 1808–1871 — read 2026-07-30.
- Requirements page — https://www.blender.org/download/requirements/ — read 2026-08-01.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 5.0's **Edit ‣ Preferences ‣ System** page was opened and the **Cycles Render Devices** panel expanded. The row of technology buttons held exactly two, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — and two dropdown lists: **Kernel Optimization**, set to **Full**, offering **Off** and **Intersection Only**; and **MetalRT**, set to **Auto**, offering **Off** and **On**. Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed. No "Requires Apple Silicon with macOS … or newer" line appeared, because a Metal device was found, so this run records nothing about the macOS 12.2-against-13.0 disagreement above.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — `/opt/blender/blender-5.0.1-linux-x64/blender` (`Blender 5.0.1`) was put to three tests here. The machine is a container sharing somebody else's hardware, and other work ran on it at the same time, so nothing here says how long anything took.

**The technology row on Linux, and what it says when there is no card.** With a window open on the machine's own display, the row held five buttons — **None**, **CUDA**, **OptiX**, **HIP**, **oneAPI** — and no Metal. With **oneAPI** chosen, and no Intel card in the machine, 5.0 drew:

```
No compatible GPUs found for Cycles
Requires Intel GPU with Xe-HPG architecture and
  - intel-level-zero-gpu or intel-compute-runtime version
    XX.XX.34666.3 or newer
  - oneAPI Level-Zero Loader
```

With **HIP** chosen, and no AMD card:

```
No compatible GPUs found for Cycles
Requires AMD GPU with RDNA architecture
and ROCm HIP Runtime 6.0 or newer
or AMD driver version 23.40 or newer
```

So the Intel Linux figure a running 5.0 draws is the manual's `XX.XX.34666.3`. And the AMD text names both a ROCm HIP Runtime figure and a driver figure, as two alternatives — 4.5 on the same machine names only the driver. This records what the panel displays on a machine with no Intel or AMD card; it does not say what Blender would enforce against a real one, and no such card exists among the equipment.

**A Cycles graphics-card render works with no window and with no display reachable.** *(Wording corrected 2026-08-06 by the citation check of the two-card material. That machine had a real display server running and the runs were cut off from it; a machine with no display server at all is a different machine, recorded in the two-card block under question 5 and in 5.0's first-pass findings under question 1. Nothing observed changes.)* With the technology saved as `CUDA` and the card ticked, a background render exited `0` and reported `Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]`. With `OPTIX`, the OptiX line. With `DISPLAY` removed from the environment altogether, the same. So a Cycles graphics-card render needs no display attached and no desktop session on Linux.

**The compute-capability figure is out of reach here.** The requirement line for CUDA and OptiX is drawn only when no compatible card is found, and this machine's card is compatible — Blender's own system-information file reports its compute capability as major `8`, minor `9`, far above the `5.0` this version states. So the figure could not be tested.

**Not covered.** Whether a card below these floors is hidden, greyed out, or offered and then fails — needs a card below the floor. They do not name a minimum amount of graphics memory for Cycles GPU rendering — the "2 GB VRAM" figure on the requirements page sits under the general Windows and Linux tables, which name no version, so I do not read it as a Cycles requirement for 5.0. Whether a card must also be driving a display is now answered for Linux: it need not, since a card render worked with no display server at all. What happens if a requirement is met on paper but the card is refused at run time is still untested.

**Outside the interface.** All of (a) through (d) are hardware and driver facts and nothing in the sources ties them to the interface. Point (e) is written as a sequence of clicks; the equivalent from outside is covered under question 5. The sources do not say whether the three settings in (e) are still all required when `--cycles-device` is used — the add-on's own help text says that flag overrides both, and both halves of that are now observed on both machines (question 5). And the missing-requirement text quoted above never reaches a windowless run: with the technology saved as `ONEAPI` and no Intel card, the phrase `No compatible GPUs found for Cycles` appeared nought times in the whole output of a background render, which completed on the processor at exit `0`.

**Operating system.** No. This is the one answer where the operating system decides the outcome. CUDA, OptiX, HIP and oneAPI are stated as Windows and Linux only; Metal is stated as Apple only, and only on Apple Silicon. The driver floors in (b) are given per operating system for AMD and Intel. The manual gives no macOS option for NVIDIA or AMD cards, and no Windows or Linux option for Metal. Two systems are now tested and both bear it out: on the Creator's Apple Silicon Mac the technology row offered **None** and **Metal** and nothing else; on the rented Linux machine it offered **None**, **CUDA**, **OptiX**, **HIP** and **oneAPI** and no Metal, and drew the Linux driver figures. Windows remains untested, and the Windows figures cannot be read from Linux because the panel text branches on the operating system; a rented Windows machine would show them and none was used. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Record of looking.** Searched the whole 5.0 manual for `CUDA`, `OptiX`, `HIP`, `oneAPI`, `Metal` — five pages carry them, of which three are real hits: `render/cycles/gpu_rendering`, `editors/preferences/system`, `advanced/command_line/arguments` (the others are shader-node pages about metallic materials and a history page). Opened all three in full. Opened `render/cycles/render_settings/index` and `.../performance` and `.../sampling` for the scene-side device settings. Searched the release notes for the same five terms plus `driver` and `requirement`; the Cycles page's "GPU" section and the index page's "Compatibility" list were the only hits about requirements. Read the requirements page in full. Grepped the Cycles add-on's Python files for device and requirement strings and read `properties.py` lines 1558–1938. Ruled out `cycles_5.0/source` — it is kernel and OSL shader code (`source/kernel/device/{cuda,gpu,hip,hiprt,metal,optix}` holds compile-time kernel headers); grepping it for `compute capability`, `sm_50`, `__CUDA_ARCH__` and `__METAL_VERSION__` returned only kernel compilation guards, from which nothing about machine requirements can be read without adding steps of my own.

---

## 3. Does what a computer must have on it depend on how Blender was obtained and put there?

**Answer.** Yes, in several distinct ways, and one of them bears directly on rendering.

**The way that matters most.** A Blender installed from a Linux distribution's own package manager may not be able to render on graphics cards at all:

> "Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

**Administrator rights depend on the package chosen on Windows.** Of the installer:

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

Of the zip:

> "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

**The Microsoft Store copy is a different kind of thing, and Blender knows it.** The Python API documents a read-only flag for exactly this:

> "is_microsoft_store_install
> Whether this blender installation is a sandboxed Microsoft Store version
> Type: boolean, default False, (readonly)"

**A helper program is required for two routes.** Snap:

> "Assuming snap is already installed, Blender can be installed through snap with:
> snap install blender --classic"

Steam:

> "Download and install the Steam client for your operating system."

**Some routes need nothing at all beyond a folder.** The requirements page, in a section that names no version:

> "Whether it's on a USB stick, sitting on a folder on your desktop, or fully installed, Blender runs out of the box.
> No installation needed, no Internet connection required."

And the manual describes making a copy self-contained by creating a folder:

> "When running Blender from a portable drive, it's possible to keep the configuration"

**One route imposes an approval step.** On macOS:

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

**One route restricts the tool used to unpack it.**

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead."

**On the machine itself, one requirement is stated per version.** The requirements page names this version explicitly here, so I may use it:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."

The release notes state the same as a change:

> "* [Support for Intel Macs has been removed]"

I must record a plain conflict between two sources here, without resolving it. The 5.0 manual's macOS installation page says:

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture."

That sentence sits in the 5.0 manual and contradicts the version-named requirements note and the release note. I report all three and pick none.

**LABEL: documented.**

**Sources and addresses.**
- Manual, Installing on Linux — https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html — read 2026-07-30.
- Manual, Installing on Windows — https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html — read 2026-07-30.
- Manual, Installing on macOS — https://docs.blender.org/manual/en/5.0/getting_started/installing/macos.html — read 2026-07-30.
- Manual, Installing from Steam — https://docs.blender.org/manual/en/5.0/getting_started/installing/steam.html — read 2026-07-30.
- Manual, Blender's Directory Layout — https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html — read 2026-07-30.
- API, `bpy.types.PreferencesSystem` — https://docs.blender.org/api/5.0/bpy.types.PreferencesSystem.html — read 2026-07-30.
- Release notes, index, "Compatibility" — https://developer.blender.org/docs/release_notes/5.0/ — read 2026-07-30.
- Requirements page — https://www.blender.org/download/requirements/ — read 2026-08-01.

**Hand test on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, the copy under test was the official Linux x64 build from Blender's own download server, unpacked at `/opt/blender/blender-5.0.1-linux-x64/`. There is a way to tell from a running Blender which graphics-card technologies its copy carries, and this answer says the sources do not give one: `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed:

```
TYPES: (True, True, True, False, True, False)
```

Cycles' own file unpacks that as `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt`, so this copy carries CUDA, OptiX, HIP and oneAPI and not Metal. So the official Linux download does not leave graphics-card rendering out. A distribution's own package is a different copy and is untested; nothing may be installed on the rented machine.

**Not covered.** The sources do not say which distributions leave Cycles GPU rendering out — and none could be installed to find out. `bpy.app.build_options` reports `cycles=True` / `cycles_osl=True`, which is about Cycles itself and not about its GPU backends; `_cycles.get_device_types()` above is about the backends and is now observed on both machines, giving `(False, False, False, True, False, False)` on the Mac and the Linux answer above. They do not say whether the Snap, Steam or Microsoft Store copies can render on graphics cards, or whether the Store sandbox restricts driver access. They say nothing about Flatpak — a search of the manual for that word returned nothing.

**Outside the interface.** The administrator-rights and helper-program points are properties of installing, not of running, and hold however Blender is later started. The `is_microsoft_store_install` flag is a Python property and so readable from outside the interface. The macOS first-open approval is described as a prompt, and the sources do not say what happens when such a copy is first launched with no person present.

**Operating system.** No — every difference above is specific to one operating system. The package-manager and Snap points are Linux only; installer-versus-zip, the Store, and the 7-zip limitation are Windows only; the approval prompt and the Apple Silicon requirement are macOS only. Steam and the portable folder are described for all three.

**Record of looking.** Searched the whole manual for `snap`, `flatpak`, `steam`, `Microsoft Store`, `package manager`, `portable`. `flatpak` returned nothing anywhere. Read all five pages under `getting_started/installing/` in full plus `advanced/blender_directory_layout`. Searched the API documentation for `is_microsoft_store_install` and `build_options`. Read the requirements page in full and checked every place it names a version — only the macOS note and the "Requirements for Older Versions" list do, and that list names 4.5 LTS, 4.1, 3.6, 3.4, 2.92, 2.90, 2.83, 2.79 and 2.76, never 5.0. Searched the release notes for `Intel`, `Apple Silicon` and `requirement`.

---

## 4. Does Blender say anything about what is missing when it will not start, or when it will not render?

**Answer.** Partly, and the two halves are unequal. When it will not render on graphics cards, Blender names the missing thing in some detail. When it will not start, the sources describe what to check far more than what Blender itself reports.

### When it will not render on graphics cards

The Cycles preferences panel prints a named message plus the specific requirement that was not met. These strings are in the add-on's own file:

> `col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)`

and then, chosen by which backend is selected and on which operating system Blender is running:

> `col.label(text=rpt_("Requires NVIDIA GPU with compute capability %s") % compute_capability,` (CUDA and OptiX, with `compute_capability = "5.0"`)

> `col.label(text=rpt_("and NVIDIA driver version %s or newer") % driver_version,` (OptiX, with `driver_version = "535"`)

> `col.label(text=rpt_("Requires AMD GPU with RDNA architecture"),` followed by `rpt_("and AMD Adrenalin driver %s or newer")` on Windows or `rpt_("and ROCm HIP Runtime %s or newer")` / `rpt_("or AMD driver version %s or newer")` on Linux

> `col.label(text=rpt_("Requires Intel GPU with Xe-HPG architecture"), icon='BLANK1', translate=False)` plus a driver-version line, with a longer Linux variant listing `"  - intel-level-zero-gpu or intel-compute-runtime version"` and `"  - oneAPI Level-Zero Loader"`

> `col.label(text=rpt_("Requires Apple Silicon with macOS %s or newer") % mac_version,`

The same file also marks a device that is present but running unoptimised:

> `name += rpt_(" (Unoptimized Performance)")`

The manual carries a short list of render-time error messages under a heading of that name, giving the message and its meaning:

> "Error: Out of memory
> This usually means there is not enough memory to store the scene for use by the GPU."

> "The NVIDIA OpenGL driver lost connection with the display driver"

> "Unsupported GNU version
> On Linux, depending on your GCC version you might get this error."

> "CUDA Error: Kernel compilation failed
> This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed."

The manual also documents one silent recovery rather than a message — Blender changing course instead of reporting:

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

### When it will not start

The manual has a page for this. It lists causes for a person to check, and separately describes the console:

> "If Blender fails to start, there are a few common causes to check for:
> See if your computer meets the minimum requirements.
> Confirm that your graphics card is supported and that the drivers are up to date"

> "The Blender Console Window can display many different types of status and error messages. Some messages simply inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender non-responsive or shut down completely."

Only two startup messages are actually quoted on that page, and neither is about a missing requirement:

> "found bundled python: {DIR}"

> "Read prefs: {DIR}/userpref.blend"

One named startup-time error about a missing requirement does appear, in the graphics-hardware troubleshooting pages, repeated on each vendor page:

> "Unsupported Graphics Driver Error
> This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

I record without resolving that this figure and the requirements page's figure differ; the requirements page says, in a section naming no version, "OpenGL: Version 4.3 or newer, with mandatory support for GL_ARB_shader_draw_parameters and GL_ARB_clip_control extensions."

For a start that fails without a message, the manual's advice is to go looking rather than to expect one:

> "Try running Blender from the command line, to see if any helpful error messages are printed."

And on a hard failure:

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."

**LABEL: observed** for the Cycles preferences strings (read out of the add-on's file); **documented** for everything quoted from the manual.

**Sources and addresses.**
- Cycles add-on file `cycles_5.0/properties.py`, `CyclesPreferences._draw_devices`, lines 1808–1877 — read 2026-07-30.
- Manual, GPU Rendering, "Error Messages" — https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.
- Manual, Startup — https://docs.blender.org/manual/en/5.0/troubleshooting/startup.html — read 2026-07-30.
- Manual, Common Problems (graphics hardware) — https://docs.blender.org/manual/en/5.0/troubleshooting/gpu/common/other.html — read 2026-07-30.
- Manual, Crashes — https://docs.blender.org/manual/en/5.0/troubleshooting/crash.html — read 2026-07-30.
- Requirements page — https://www.blender.org/download/requirements/ — read 2026-08-01.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, every technology button in Blender 5.0's **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** row was clicked in turn. The row held **None** and **Metal**. Neither the "No compatible GPUs found for Cycles" line nor any "Requires…" line appeared under either, and the two tick boxes were labelled exactly **Apple M1 Pro (GPU - 14 cores)** and **Apple M1 Pro**. A Metal device was present on this machine, so the no-compatible-card case was never reached and this run records nothing about what it displays.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.0 in background mode was asked for devices it could not give. `-- --cycles-device OPTIX`: exit status `1`, no image written, and the line

```
00:01.263  reports          | ERROR Found no Cycles device of the specified type
```

`-- --cycles-device NOTADEVICE`: exit status `1`, no image written, and the line

```
00:00.231  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
```

So `set_device_override` does print before the process ends, and the exit status is the `1` the file shows.

The near-miss the answer names is now filled in from the other side too. Asked to render on graphics cards with every card switched off in the saved preferences, and again with the technology set to `NONE`, Blender exited `0`, wrote the image, and printed **nothing** naming a missing or unusable device — the run simply ended `Fra: 1 | Mem: 151M | Sample 16/16`, without the `(Using optimized kernels)` text a graphics-card render carries. Full commands and output are under question 5.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, five things bearing on this question were put to 5.0.

**Devices Cycles will not accept.** `-- --cycles-device METAL`, `-- --cycles-device HIP` and `-- --cycles-device ONEAPI` all ended at exit `1`, wrote no image, and printed the same one line, word for word the Mac's:

```
00:00.447  reports          | ERROR Found no Cycles device of the specified type
```

HIP and oneAPI are built into this copy and simply have no card; Blender says the same thing for that as for Metal, which is not built in at all. `-- --cycles-device NOTADEVICE` gave exit `1`, no image, and two lines:

```
00:00.187  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

The second line 4.5 does not print on the same machine.

**The real no-compatible-card case, which the Mac could not reach.** With the technology saved as `ONEAPI` and no Intel card present (`SAVED: ONEAPI ACTIVE: False NUMGPU: 0`), a background render exited `0`, wrote its image, and rendered on the processor. The whole output was searched for the phrase `No compatible GPUs found for Cycles` and it appeared **nought times**; no `Requires…` line appeared either. So that text is drawn in the preferences panel and nowhere else, and the near-miss above was a fair guide to the real case. What the panel says is quoted under question 2.

**Where the crash file lands on Linux.** A crash forced with `-b <file> --python-expr "import ctypes; ctypes.string_at(0)"` gave exit `139` and:

```
Writing: /tmp/machine-test.crash.txt
```

With no blend-file loaded, `Writing: /tmp/blender.crash.txt`. So the manual is right about the folder and incomplete about the name.

**What Blender says when it cannot start for want of a display server.** With `DISPLAY` removed and a window asked for, 5.0 printed and then exited `1`:

```
00:00.024  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.024  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4556, total unfreed memory 1.272339 MB
```

4.5 on the same machine printed the same information in a plainer shape and then hung rather than exiting.

**One warning this version prints on Linux that the Mac never showed**, but only when Cycles is asked for HIP:

```
00:00.287  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
```

That is Cycles reporting that AMD's library is not on the machine. Harmless here, but anything reading Blender's output has to expect it.

*(Corrected 2026-08-06 by the runner, after the check of the Linux hand tests could not reproduce this. The version split is right; "nearly every run" was not. Run again on the same machine with a plain Cycles render at factory settings, this line appeared **not at all**, in any of the four versions. It appears only when Cycles is actually asked for HIP — either the preferences are set to HIP, or the render is started with `--cycles-device HIP`. Then 5.0, 5.1 and 5.2 each print it exactly once and 4.5 prints nothing of the kind, failing instead with `Error: Found no Cycles device of the specified type`. So anything reading Blender's output has to expect this line when it asks for HIP, and not otherwise.)*

**Not covered.** This is the answer with the largest gap, and it is worth stating precisely.

The Cycles messages above are drawn on a preferences panel. Nothing in the sources says whether the same text, or any text, is emitted when there is no panel to draw on. The manual's "Error Messages" list gives message text but never says where the message goes — window, console, log, or all three. Nothing states an exit code for a failed render; a search for `exit code` and `return code` across the manual and API documentation returned nothing. Nothing says what Blender reports when a device selected in preferences has since disappeared, when a driver is below the floor at render time, or when a render is asked for on graphics cards and none are usable — in particular, whether it stops or quietly renders on the processor. A search for `fall back`, `fallback`, `falls back`, `no device`, `device not` and `silently` found nothing on that point; the only documented automatic change of course is the graphics-memory-to-system-memory one quoted above.

One near-miss is worth naming because it is the closest the sources come. The Cycles add-on's own command-line handling ends a bad device request by quitting outright, with no message written in the file:

> ```
> if args.cycles_device:
>     import _cycles
>     if not _cycles.set_device_override(args.cycles_device):
>         sys.exit(1)
> ```

What `set_device_override` prints before returning false, if anything, is not in any file I was given.

**Outside the interface.** The startup page assumes a console is available and tells the reader how to get one — "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window. To ensure you can see any error messages that appear at launch, open a command-line window" — which documents that the messages exist independently of the window, but says nothing about the Cycles device messages specifically. Those are now settled on the rented Linux machine: **they do not leave the window.** With a technology chosen that has no compatible card, a windowless render printed the phrase nought times and completed on the processor. The crash file is written without any interface involvement, and where it lands on Linux is now recorded above. Blender does have logging switches usable from outside — `--log <match>`, `--log-level <level>` with an `info` level described as "Information about devices, files, configuration, operations", and `--log-file <filepath>` — but the sources do not say what device information those levels actually produce.

**Operating system.** The Cycles preferences messages differ by operating system by construction: the add-on file branches on `sys.platform` to choose the HIP and oneAPI text, and prints the Metal requirement only for the Metal backend. That is now observed: the rented Linux machine drew the Linux Intel and AMD lines, quoted under question 2, and could never draw the Windows ones. Of the manual's render error messages, "Unsupported GNU version" is stated as Linux only and depends on the compiler on the machine, so it was not reached; "The NVIDIA OpenGL driver lost connection with the display driver" is described in terms of a Windows time limit and no Windows machine ran these tests. The crash page gives a different file name and location for each of Windows, macOS and Linux; two of the three are now observed — `blender.crash.txt` in the Mac's temporary directory, and `/tmp/<name of the loaded file>.crash.txt` on Linux.

**Record of looking.** Searched the whole manual for `error message`, `unsupported graphics`, `minimum requirements`, `crash`, `startup`, `fall back`, `fallback`, `falls back`, `no device`, `device not`, `silently`, `exit code`, `return code`. Listed and opened `troubleshooting/startup`, `troubleshooting/crash`, `troubleshooting/gpu/common/{introduction,other,laptops}`, `troubleshooting/gpu/windows/other`, and checked that the "Unsupported Graphics Driver Error" text repeats identically across all twelve vendor pages under `troubleshooting/gpu/`. Read the "Error Messages" section of the GPU Rendering page in full. Grepped every Cycles add-on Python file for `report(`, `rpt_`, `label(text=` and `sys.exit`; `operators.py` reports only denoising and merge failures, `ui.py` carries only "Cycles built without Embree support", "CPU raytracing performance will be poor", "Incompatible output node" and "No output node", none of which concern device requirements. Read `engine.py` lines 1–60 in full. Searched the release notes for message text and found none; the corrective-releases page lists device-related crashes by title only, for example "Crash: Blender segfaults with HIP RT, when rocm-hip-devel dependency is not met" and "Segfault when switching to HIP while HIP runtime 5.X is installed", which describe failures rather than messages.

---

## 5. How is it decided which of a computer's graphics cards a render uses, can more than one card be used for the same render, and how is that decided from outside Blender's own interface?

This question has three parts and I answer them in order.

### Which cards a render uses

**Answer.** Three choices combine, and all three must line up.

First, one backend is chosen, and it is a single choice for the whole of Blender:

> "Cycles Render Devices
> Changes the Computing Device the Cycles render engine uses to render images."

> "CUDA: If the system has a compatible NVIDIA CUDA device, it will be available as an option for rendering with Cycles."

Only backends the machine can actually support are offered. The picture of this panel on that page shows the row offering exactly three choices — `None`, `CUDA`, `HIP` — with `None` selected, on whatever machine the screenshot was taken. That is a picture of a panel, and what it shows is that the row's contents vary with the machine.

Second, individual cards are switched on or off within that backend:

> "Enable the available devices for the chosen backend."

The add-on file shows this is a per-card switch with a stated default, and that the processor is offered alongside the cards but starts switched off:

> ```
> id: StringProperty(name="ID", description="Unique identifier of the device")
> name: StringProperty(name="Name", description="Name of the device")
> use: BoolProperty(name="Use", description="Use device for rendering", default=True)
> type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
> ```

> `entry.use = entry.type != 'CPU'`

Third, the scene itself must ask for graphics cards at all:

> "GPU Compute: Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices."

with a default that is stated in the add-on file:

> ```
> device: EnumProperty(
>     name="Device",
>     description="Device to use for rendering",
>     items=enum_devices,
>     default='CPU',
> ```

One default is set by the machine rather than by the person. On Apple Silicon the backend starts as Metal:

> ```
> # Default to selecting the Metal compute device on Apple Silicon GPUs
> # (drivers are tightly integrated with macOS so pose no stability risk)
> if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
>     return 5
> return 0
> ```

The add-on file also shows that the list of cards is deliberately not rebuilt when Blender starts:

> `# Refresh device list. This does not happen automatically on Blender startup due to unstable drivers that can cause crashes.`

### Whether more than one card can be used for the same render

**Answer.** Yes, stated outright.

> "Can multiple GPUs be used for rendering?
> Yes, go to Preferences ‣ System ‣ Compute Device Panel, and configure it as you desire."

The processor can be added to the cards, depending on backend:

> "Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."

Memory is not pooled by default:

> "Would multiple GPUs increase available memory?
> Typically, no, each GPU can only access its own memory.
> The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."

> "Distribute Memory Across Devices
> Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes. Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol.
> Currently only NVLink on NVIDIA GPUs is supported."

The add-on file gives that setting's exact name, kind and default:

> ```
> peer_memory: BoolProperty(
>     name="Distribute memory across devices",
>     description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it",
>     default=False,
> )
> ```

One behaviour of mixed sets is stated only as a comment in the add-on file, and it concerns hardware ray tracing rather than device selection:

> `# Any device without RT support will disable it for all.`

### How it is decided from outside Blender's own interface

**Answer.** By one command-line switch, which selects a backend, and it overrides the two saved settings.

The manual:

> "--cycles-device <device>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
> Append +CPU to a GPU device to render on both CPU and GPU.
> Example:
> blender -b file.blend -f 20 -- --cycles-device OPTIX"

with the placement rule:

> "Cycles add-on options must be specified following a double dash."

The add-on's own help text for the same switch says what the manual does not — that it displaces the saved choices:

> `help="Set the device to use for Cycles, overriding user preferences and the scene setting."`
> `     "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."`
> `     "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",`

Note what this switch does and does not name. It names a backend, and optionally adds the processor. It does not name an individual card. Whether the per-card switches from saved preferences still apply when this flag is used is decided inside `_cycles.set_device_override`, which is not among the files I was given.

The other route from outside is Python. The settings above are ordinary properties — `scene.cycles.device` on the scene, and `compute_device_type`, `devices` and `peer_memory` on the Cycles add-on preferences — and the API documents the general way to reach any add-on's preferences:

> "addon_prefs = preferences.addons[__name__].preferences"

and documents the scene-side property's existence:

> "cycles
> Cycles render settings
> Type: CyclesRenderSettings, (readonly)"

**LABEL: documented** for everything quoted from the manual and the API documentation; **observed** for everything quoted from `cycles_5.0/properties.py` and `cycles_5.0/engine.py`; **documented** for what the picture of the preferences panel shows.

**Sources and addresses.**
- Manual, System preferences — https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30, including its picture `_images/editors_preferences_section_system.webp` from the same page.
- Manual, GPU Rendering — https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.
- Manual, Render Settings — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html — read 2026-07-30.
- Manual, Command Line Arguments — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.
- API, `bpy.types.AddonPreferences` — https://docs.blender.org/api/5.0/bpy.types.AddonPreferences.html — read 2026-07-30.
- API, `bpy.types.Scene` — https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30.
- Cycles add-on file `cycles_5.0/properties.py`, lines 30–33, 381–387, 1558–1606, 1668–1684, 1707–1716, 1907 — read 2026-07-30.
- Cycles add-on file `cycles_5.0/engine.py`, lines 8–41 — read 2026-07-30.

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 5.0.

**The panel.** **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** was expanded and every button in its technology row clicked in turn. The row held exactly two buttons, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — plus **Kernel Optimization** (**Full**; also **Off**, **Intersection Only**) and **MetalRT** (**Auto**; also **Off**, **On**). Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed.

**From Blender's Python Console** (Scripting workspace, window open). `list(bpy.context.preferences.addons.keys())` returned:

```
['io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'cycles', 'pose_library', 'bl_pkg']
```

so the add-on key is the literal string `cycles` on this machine. With `p = bpy.context.preferences.addons["cycles"].preferences`:

- `p.compute_device_type` returned `'METAL'`.
- `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned `[]` before anything else was called, and after `p.refresh_devices()` returned `[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]`.
- `p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, `p.has_multi_device()` returned `False`.
- `p.devices[0].use = False` was run — index 0 being the processor entry, unticked by default — and the panel then showed exactly what it shows by default. `p.devices[0].use = True` was run, and the **Apple M1 Pro** box in the panel became ticked.

This was done from a Python Console inside Blender's own window; it records nothing about a background run.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender` (`Blender 5.0.1`). Scene: `/tmp/machine-test.blend`, Cycles, scene Device `GPU`, 16 samples.

**The three choices, read and written with no window open.** `-b --python-expr` reading the preferences printed:

```
CYCLES KEYS: ['cycles']
BACKEND: METAL
BEFORE: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False)]
AFTER: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False)]
```

On this machine the card was switched off in 5.0's saved preferences — the state of one installation, not a fact about Blender. Writing worked too: setting every `use` to `False` and calling `bpy.ops.wm.save_userpref()` printed `SAVED STATE: METAL [('Apple M1 Pro', False), ('Apple M1 Pro (GPU - 14 cores)', False)]`.

**Whether the per-card switches still apply when `--cycles-device` is used — the question this answer left inside `_cycles.set_device_override`.** With every card switched off and saved, a background render ended:

```
00:05.388  render           | Fra: 1 | Mem: 151M | Finished
```

and the same render with `-- --cycles-device METAL` ended:

```
01:36.974  render           | Fra: 1 | Mem: 1583M | Sample 16/16 (Using optimized kernels)
```

So the flag displaced the saved switches. Against a copy of the scene saved with its own Device set to `CPU`, `-- --cycles-device METAL` gave `00:04.740  render           | Fra: 1 | Mem: 1583M | Sample 16/16 (Using optimized kernels)` — so it displaced the scene setting as well. Both halves of the add-on's help-text claim hold here.

**Whether preferences are read in background mode** — yes: switching the cards off and saving changed what a windowless render did, as the two runs above show. **`--factory-startup` discards those saved switches**: with it added, the stored list read `BEFORE: []` and after a refresh showed the card ticked and the processor unticked.

**A change made inside the render's own run** did not produce the graphics-card signature. Setting every `use` to `True` in the same process that then rendered printed `SET TO: [('Apple M1 Pro', True), ('Apple M1 Pro (GPU - 14 cores)', True)]` and ended `00:04.971  render           | Fra: 1 | Mem: 151M | Sample 16/16`, with no `(Using optimized kernels)`. The saved preferences were unchanged afterwards, and were restored to the state found.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), the whole of **Edit ‣ Preferences ‣ System** was looked at. The page held **Cycles Render Devices**, then **Network**, **Memory & Limits**, **Video Sequencer** and a collapsed **Sound** — and nothing else. **There was no Display Graphics section, and no graphics backend or device control anywhere on the page**, in any of the four versions. So on this machine there is no choice of drawing card outside the Cycles list. A test aimed at the Vulkan device list was mis-aimed and is recorded as such: Vulkan is not something a Mac offers, so the question assumed a control that does not exist on this operating system. Windows and Linux stay untested.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-5.0.1-linux-x64/blender`. Its settings folder started empty, so this version began from factory settings rather than from anything a person had set. One thing about that folder, said plainly rather than left out. The first runs were meant to point Blender at a folder of this topic's own with `BLENDER_USER_RESOURCES`, and a mistake in the shell meant the variable was never actually set, so those runs used Blender's ordinary default folder instead. That folder was also empty when this work began, so "began from factory settings" holds either way. Every test that turns on saved preferences was then run again with the variable properly set, and every result came back identical. Both runs are described in `Audits/Blender Research/The machine/Hand tests — Linux.md`.

**What Blender finds.** With an untouched settings folder:

```
CYCLES KEYS: ['cycles']
BACKEND: NONE
BEFORE: []
AFTER: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True)]
```

The key is `cycles` here too. The technology starts at `NONE`, where the Mac starts at `METAL`. **One physical card is listed twice**, once under CUDA and once under OptiX, with two identifiers. With preferences saved, `BEFORE` came back already populated; `--factory-startup` emptied it again. `has_active_device()` returned `True`, `get_num_gpu_devices()` returned `1`, `has_multi_device()` returned `False`.

**The saved switches are honoured, and the flag displaces them.** Every device switched off and saved, then rendered: exit `0`, image written, `Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]`. The same render with `-- --cycles-device OPTIX`: `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]`. So `--cycles-device` **ignores** the per-card switches and overrides them — one of the things this answer says the sources leave open, now observed on both machines.

**The flag also overrides the scene's own setting.** The scene saved with Device `CPU`, rendered with `-- --cycles-device OPTIX`, reported the card.

**`+CPU` uses both, and Linux names both.** With `-- --cycles-device CUDA+CPU`:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                            |                  AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (11 threads) [CPU]
```

**A change made by a script inside the render's own run — and here the two machines disagree.** On the rented Linux machine it took effect. Every device off: the processor. Every device on: the card and the processor both named. The technology set to `NONE`: the processor. All three exited `0`. On the Creator's Mac the same three runs carried no graphics-card marker at all. Both stand; neither is chosen between here.

**A control over which card draws the interface exists on Linux.** Read with no window: `GPU_BACKEND NOW: OPENGL | CHOICES: ['OPENGL', 'METAL', 'VULKAN']` and `GPU_PREFERRED_DEVICE NOW: AUTO | CHOICES: ['AUTO']`. With a window, the System page carries a **Display Graphics** section, which the Mac has not got, showing `Backend: OpenGL`; with the backend saved as `VULKAN` and Blender restarted, a **Device** control appears reading `Auto`, with a note this version and 4.5 carry and 5.1 and 5.2 do not:

```
Current Vulkan backend limitations:
  • Low performance in VR
```

`gpu_preferred_device` still held only `['AUTO']` with Vulkan running, there being one card.

**Blender's own `--help` on this machine** gives `--cycles-device` the same six values as every other version, and adds two arguments 4.5 has not: `--no-window-frame` ("Disable all window decorations (Linux only)") and `--gpu-vsync`. There is no `--gpu-device` argument in this version; 5.2 has one, and that is 5.2's finding.

**Not covered.** How work is divided between several cards, whether an uneven set is balanced, or what happens when one card in the set fails mid-render — all need a second card. Whether cards from two different makes can be used in one render — the backend being a single choice makes that look unlikely, but nothing states it and I will not infer it. What happens when a saved preference names a card that is no longer present. Note also that denoising and compositing choose their devices separately — "Denoising and Compositing have their own device selection" — so an answer about the render device does not settle those; every graphics-card render on the rented Linux machine reported denoising on the processor. Two things this list once held are answered: `--cycles-device` overrides the per-card switches, and there is still no way from the command line to name one particular Cycles render card, though 5.2 adds one for the *display* card.

**Outside the interface.** This is the part the sources cover best. `--cycles-device` exists precisely for use from outside, the manual's own example is a background render (`blender -b file.blend -f 20 -- --cycles-device OPTIX`), and the add-on's help text states it overrides the saved settings. The manual explicitly supports rendering with no graphical display at all: "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)." What is not settled is the saved-preferences route: the per-card switches live in the preferences file, and the sources do not say whether those preferences are read in background mode, nor how a saved preference gets onto a machine that is only ever driven from outside. The comment about the device list not being refreshed at startup is in the file, but the file does not say whether that also applies with no interface present. The hand tests settle this on one machine, and with no window open. The add-on key is the literal string `cycles`; the switches were read and written through `--python-expr` under `-b`; `bpy.ops.wm.save_userpref()` works there; and the saved file **is** read by a windowless render, since switching every card off and saving changed what the render did. How a saved preference gets onto a machine only ever driven from outside is answered the same way: by writing and saving it from a script. `--factory-startup` discards those saved switches. One thing came back the other way: a change made inside the render's own run did not produce the graphics-card signature, so a same-run change is not established as taking effect.

**Operating system.** The mechanism is the same everywhere; the choices available are not. Only the backends supported on that operating system appear, per the requirements quoted in question 2, and both machines bear that out. `--cycles-device` accepts the same six words on every platform, per the manual and per Blender's own `--help` on both machines; what happens when a platform is given a word it cannot support is now observed on Linux — `-- --cycles-device METAL` exits `1` with `ERROR Found no Cycles device of the specified type` and writes no image, which is the same answer the Mac gave for `OPTIX`. Two of the pieces are platform-bound by their own text: the Apple Silicon default in the add-on file — observed on both, `METAL` on the Mac and `NONE` on Linux — and NVLink memory distribution, which needs two NVIDIA cards and could not be tested.

### Hand tests on the two-card machine — observed
**Read this block against the *Not covered* paragraph just above it.** That paragraph was written before any two-card machine existed, so some of what it calls untested is answered here — how work is divided between two ticked cards, and the several-card form of the `--cycles-device` question above all. What it says about an NVLink pair, about two cards of different makes, about a slow card beside a fast one, and about anything needing a window still stands. *(Pointer added 2026-08-06 by the citation check of the two-card material.)*

Run on **2026-08-06** on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-5.0.1-linux-x64/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`. Label: **observed**. **A different machine from the one-card rented Linux machine above; nothing here is merged with that machine's results or with the Creator's Mac.** Settings folder `/root/smoozy/two cards/res-5.0`, empty at the start, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender. **Nothing else was running on the machine.** The full list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`.

**The two cards are not joined by NVLink** — `nvidia-smi topo -m` reports `PHB`, `Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)` — so an NVLink pair still cannot be tested here.

**Both cards start switched on, and one card is listed twice.** With an untouched settings folder and `--factory-startup`, after `refresh_devices()`:

```
BACKEND: NONE
BEFORE: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a', True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX', True)]
```

**Yes, more than one card is used for the same render, and without being asked.** With the technology saved as `OPTIX` and both ticked, a windowless render named both:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                 NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
```

Watched card by card in 4.5 with `nvidia-smi --query-gpu=index,utilization.gpu,memory.used --format=csv -l 1`, both cards sat between 91 % and 98 % for the whole render and both held the same 2683 MiB.

**Distribute Memory Across Devices on an unlinked pair.** The setting exists, is called `peer_memory`, and starts off:

```
peer_memory                      BOOLEAN    default=False        name='Distribute memory across devices'
      description='Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it'
```

It can be turned on and saved on an unlinked pair, and read back `True` in a later run. A render with it on exited 0, used both cards, and **Blender said nothing about it at all** — searching both streams for `peer`, `nvlink`, `interconnect` and `distribut` matched nothing but the output file's own name. Measured card by card in 4.5 on an 868 MB scene, each card reached the same 3557 MiB whether the setting was on or off; with one card only, 3565 MiB on card 0 and 3 MiB on card 1. So the scene is still put on both cards in full — **measured in 4.5 only**, and not repeated in this version. *(Qualifier added 2026-08-06 by the citation check of the two-card material: the sentence read as this version's own result.)*

**`--cycles-device OPTIX` takes every card of its technology, not only the ticked ones.** With only the card at `0000:00:07` ticked, a render with no flag named that card alone; the same state with `-- --cycles-device OPTIX` named both; and with **every** device unticked the same flag still named both. The flag wrote nothing back into the saved preferences.

**One card against two, on the same scene and the same samples**, on a machine with nothing else running: two cards `Time: 00:30.21 (Saving: 00:00.25)`, `Time: 00:31.31 (Saving: 00:00.32)`; one card `Time: 00:55.07 (Saving: 00:00.24)`, `Time: 00:55.10 (Saving: 00:00.26)`.

**Blender prints one memory figure, never two, and never names a card beside one.** On the timing scene its `Finished` line read `Mem: 9M` with two cards and `Mem: 1084M` with one — smaller with two, not larger and not doubled.

### Hand tests on the four-card machine — observed

Run on **2026-08-06** on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each, driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server, run over SSH** — against `/opt/blender/blender-5.0.1-linux-x64/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`. Label: **observed**. **A fourth machine, and a different one from every machine above; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine.** Settings folder `/root/smoozy/four cards/res-5.0`, emptied and remade before use, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender. **Nothing else was running on the machine.** The full list, with every command and every script written out, is `Audits/Blender Research/The machine/Hand tests — four cards.md`.

**Three things separate this machine from the two-card one, and each bears on something here.** Four cards rather than two, which is the point. A different and **smaller** card — a 4060 Ti of 16 GB against a 4090 of 24 GB — so **no timing below may be set beside a two-card timing, and none is**. And far more system memory, 197 GB against 98 GB, which matters only where a card is being filled.

**The four cards are not joined by NVLink** — `nvidia-smi topo -m` reports `PHB` between every pair, `Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)` — so an NVLink set still cannot be tested.

**All four cards start switched on, and each card is listed twice.** With an emptied settings folder and `--factory-startup`, after `refresh_devices()`, **nine entries**:

```
BACKEND: NONE
AFTER REFRESH: [('NVIDIA GeForce RTX 4060 Ti', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:07', True), ('NVIDIA GeForce RTX 4060 Ti', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:09', True), ('NVIDIA GeForce RTX 4060 Ti', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0b', True), ('NVIDIA GeForce RTX 4060 Ti', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0d', True), ('AMD EPYC 7R43 64-Core Processor', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4060 Ti', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:07_OptiX', True), ('NVIDIA GeForce RTX 4060 Ti', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:09_OptiX', True), ('NVIDIA GeForce RTX 4060 Ti', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0b_OptiX', True), ('NVIDIA GeForce RTX 4060 Ti', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0d_OptiX', True)]
```

Setting the technology to CUDA and then to OptiX and refreshing again printed the same nine entries each time. **The two-card machine found the same shape with two cards, and the two machines agree.**

**But the ticks are not the whole of it.** At factory settings no technology is chosen, and a scene whose own Device is `GPU` then renders on the processor, with nothing said about it:

```
Path tracing on: AMD EPYC 7R43 64-Core Processor (CPU) (61 threads) [CPU]
```

So "every card is switched on at factory settings" is true of the ticks and not of the render. The two-card machine's record rests on a run that chose OptiX first and does not say this.

**With a technology chosen and the ticks left exactly as they came, all four cards were used for one render, and nothing had to be asked for.** Blender named all four:

```
                            | Path tracing on: NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0d_OptiX]
                            |                  NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0b_OptiX]
                            |                  NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:09_OptiX]
                            |                  NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:07_OptiX]
```

and each card was watched separately with `nvidia-smi --query-gpu=index,utilization.gpu,memory.used --format=csv -l 1` during a render in Blender 4.5. All four sat between 97 % and 98 % for the whole render, and each held 854 or 856 MiB:

```
0, 97 %, 856 MiB
1, 97 %, 854 MiB
2, 97 %, 854 MiB
3, 98 %, 856 MiB
```

**One line reads differently from the two-card machine's.** There the rebalancer reported `Number of requested rebalances: 1` and `Number of performed rebalances: 0`. Here it reports 7 requested and 6 performed, so with four cards Blender moved the work between them repeatedly during one render. That is recorded as it printed and nothing is read into it.

**One card, two cards and four cards, on the same scene and the same samples.** Forty-eight renders in all across the four versions, one after another, never overlapping, on a machine with nothing else running. The scene is 1920 by 1080 at 3000 samples, adaptive sampling off, denoising off. **Cycles' own `Total render time:`, in seconds**, three runs of each: four cards `19.1505`, `19.3794`, `19.3043`; two cards `36.6886`, `36.8401`, `37.2728`; the card at `0000:00:07` alone `71.2772`, `71.3059`, `71.2877`. **Blender's own `Time:` line**, which counts the whole run: four cards `00:26.02`, `00:25.76`, `00:25.70`; two cards `00:42.89`, `00:43.11`, `00:43.52`; one card `01:15.83`, `01:15.75`, `01:15.70`.

**The four cards are as fast as each other.** The cards at `0000:00:09`, `0000:00:0b` and `0000:00:0d`, one run each, gave `70.2241`, `70.845` and `69.2099` seconds on Cycles' own figure — a spread of about 2 %.

**Doubling the cards did not halve the time, and quadrupling them did not quarter it.** On Cycles' own figure, one card to two is 1.93 times faster, two to four is 1.91, and one to four is 3.70. On Blender's whole-run figure, where the work of loading the file and building the scene does not shrink at all, the same three are 1.76, 1.66 and 2.92. So "more cards make it proportionally faster" holds in shape and not in arithmetic.

**Distribute Memory Across Devices, on four cards that are not linked.** The setting exists in this version, is called `peer_memory`, is a yes-or-no setting, and starts off — word for word what the two-card machine printed:

```
peer_memory                      BOOLEAN    default=False        name='Distribute memory across devices'
      description='Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it'
```

It can be turned on and saved with four unlinked cards — `SAVED STATE: OPTIX peer_memory= True`, `SAVE DONE` — and a separate later run printed `READ BACK: OPTIX peer_memory= True`. **With it on, nothing changed.** The highest memory each card reached during the same render of a 909 MB scene, sampled ten times a second in Blender 4.5, two runs of each case:

```
Distribute Memory OFF, four cards, run 1:   card 0 1710MiB   card 1 1946MiB   card 2 2190MiB   card 3 1710MiB
Distribute Memory OFF, four cards, run 2:   card 0 1710MiB   card 1 1710MiB   card 2 1710MiB   card 3 1710MiB
Distribute Memory ON,  four cards, run 1:   card 0 1710MiB   card 1 1946MiB   card 2 1946MiB   card 3 1946MiB
Distribute Memory ON,  four cards, run 2:   card 0 1946MiB   card 1 1946MiB   card 2 1710MiB   card 3 1946MiB
Distribute Memory OFF, one card,   run 1:   card 0 1722MiB   card 1    3MiB   card 2    3MiB   card 3    3MiB
Distribute Memory OFF, one card,   run 2:   card 0 1946MiB   card 1    3MiB   card 2    3MiB   card 3    3MiB
```

The figures move by as much with the setting off as with it on, and **a card working alone reached the same amount that each of four cards reached**. Blender's own memory figure was unchanged too — `Mem:480.88M, Peak:494.94M` with the setting off and with it on, to the hundredth of a megabyte. **Blender said nothing at all about the setting**: searching both streams of all eight runs for `peer`, `nvlink`, `interconnect` and `distribut` matched nothing. **The two machines agree**, and this is the strongest negative claim of the set holding on twice as many unlinked cards.

**Switching cards off, and what `-- --cycles-device OPTIX` then does.** With only the card at `0000:00:07` ticked and no flag, exit 0 and one card named. With the same one card ticked and the flag, exit 0 and **all four** named. With two cards ticked and the flag, all four named. With **every** device unticked and the flag, all four named again — the same four lines each time. And they really did the work: in Blender 4.5, with only `0000:00:07` ticked, the highest use and memory each card reached during a render of the timing scene:

```
no flag:       card 0 peak use 98% peak memory 878MiB
               card 1 peak use  0% peak memory   3MiB
               card 2 peak use  1% peak memory   3MiB
               card 3 peak use  0% peak memory   3MiB

with the flag: card 0 peak use 98% peak memory 872MiB
               card 1 peak use 98% peak memory 854MiB
               card 2 peak use 98% peak memory 854MiB
               card 3 peak use 98% peak memory 854MiB
```

with `Time: 01:06.23` against `Time: 00:18.12` — a one-card time against a four-card one. So the flag takes every card of its technology whatever the preferences said, and puts all of them to work. **The two machines agree**, and this run adds the card-by-card evidence the other did not capture. The flag wrote nothing back: read afterwards, the saved preferences still held what the last save had put there.

**What the memory readouts say with four cards.** Blender prints one figure, never four, and never names a card beside a memory number. Its `Finished` line for the timing scene rendered three ways:

```
four cards: 00:25.643  render           | Fra: 1 | Mem: 9M | Finished
two cards:  00:42.704  render           | Fra: 1 | Mem: 9M | Finished
one card:   01:15.714  render           | Fra: 1 | Mem: 392M | Finished
```

**The figure is far smaller with more than one card — and it does not go on shrinking.** Two cards and four cards printed exactly the same figure, on this scene and — in Blender 4.5, which is where that half was measured — on the 909 MB one. *(Narrowed by the four-card citation check, 2026-08-06: the heavier scene was measured in 4.5 only, and this sentence read as though it had been measured here.)* So it does not count memory divided by the number of cards. What it does count is not settled here and nothing is read into it.

**Not covered here.** Cards of two different makes; a slow card beside a fast one, all four being the same model and within 2 % of each other; an NVLink set; and anything needing eyes on a window, this machine having no display server either.

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on. So this block speaks for the outside case alone, and none of the two-card machine's window tests could be repeated here either.

**Operating system — this block.** Linux, on the machine named at the top of it. It says nothing about macOS, where the project's only machine has one set of built-in graphics, and nothing about Windows, which no Windows machine has run.

**Not covered here.** Cards of two different makes; a slow card beside a fast one; an NVLink pair; and anything needing eyes on a window, this machine having no display server.

**Record of looking.** Searched the whole manual for `device`, `compute device`, `render device`, `GPU Compute`, `multi-GPU`, `NVLink`, `which GPU`, `--cycles-device`. Read `editors/preferences/system`, `render/cycles/gpu_rendering`, `render/cycles/render_settings/index`, `render/cycles/render_settings/performance`, `advanced/command_line/arguments` and `advanced/command_line/render` in full. Opened the one picture on the preferences page. Read `cycles_5.0/properties.py` and `cycles_5.0/engine.py`. Searched the manual and API documentation for `CYCLES_` to see whether an environment variable can pick a device — the only such names anywhere are `CYCLES_CUDA_EXTRA_CFLAGS` (a compiler workaround on the GPU Rendering page) and, inside kernel source headers, `CYCLES_CUBIN_CC`, `CYCLES_HIPBIN_CC` and `WITH_CYCLES_DEBUG`, none of which select a device; the manual's "Environment Variables" section on the command-line page lists only `BLENDER_USER_*`, `BLENDER_SYSTEM_*`, `BLENDER_OCIO`, `BLENDER_CUSTOM_SPLASH*`, `TEMP` and `TMPDIR`. Searched the API documentation for `compute_device_type` — the only match is `PreferencesSystem.legacy_compute_device_type`, documented as "For backwards compatibility only".

---

## 6. Can Blender be asked which graphics cards it can see and can use?

**Answer.** Yes, by three different routes, and only one of them is properly documented.

**Route one, documented: the system information report.** The manual describes a report Blender writes out, and one of its sections is exactly this question:

> "Save System Info
> This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems.
> You will be prompted to save a text file called system-info.txt."

Of its sections:

> "Cycles
> The instruction sets and capabilities of each hardware render device available for use with Cycles."

and separately, for the card drawing the interface:

> "GPU
> Shows the GPU vendor, version and the capabilities of your hardware and driver."

The troubleshooting pages point at the same thing:

> "To find out which graphics card and driver Blender is using, use Help ‣ Save System Info inside Blender. The OpenGL section will have information about your graphics card, vendor and driver version."

This report has a Python operator, which is what makes it reachable without clicking:

> "bpy.ops.wm.sysinfo(*, filepath='')
> Generate system information, saved into a text file
> Parameters:
> filepath (string, (optional, never None)) – filepath"

**Route two, documented, but about the display card only.** The `gpu.platform` module answers what is drawing the interface, not what Cycles can render on:

> "gpu.platform.device_type_get()
> Get GPU device type.
> Returns: Device type ('APPLE', 'NVIDIA', 'AMD', 'INTEL', 'SOFTWARE', 'QUALCOMM', 'UNKNOWN')."

> "gpu.platform.renderer_get()
> Get GPU to be used for rendering.
> Returns: GPU name."

I flag a wording trap here. `renderer_get` says "GPU to be used for rendering", but the module is titled "GPU Platform Utilities" and described as "This module provides access to GPU Platform definitions", and every troubleshooting page that mentions finding your card describes this information as the OpenGL section. Reading `renderer_get` as naming a Cycles render device would require me to supply that step myself, so I do not.

**Route three, observed, not documented anywhere: the Cycles add-on's own functions.** These are in the file, with these exact names and shapes:

> ```
> def get_device_list(self, compute_device_type):
>     import _cycles
>     device_list = _cycles.available_devices(compute_device_type)
> ```

> `# Layout of the device tuples: (Name, Type, Persistent ID)`

> ```
> def get_device_types(self, context):
>     import _cycles
>     has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()
> ```

> ```
> def has_active_device(self):
>     return self.get_num_gpu_devices() > 0
> ```

> ```
> # Deprecated: use refresh_devices instead.
> def get_devices(self, compute_device_type=''):
>     self.refresh_devices()
>     return None
> ```

There is also a plain list of what has been seen, stored in the preferences, with a name and an on/off switch per entry — the `devices` collection and the `CyclesDeviceSettings` fields quoted under question 5 — and a second entry point that takes no backend argument:

> ```
> def available_devices():
>     import _cycles
>     return _cycles.available_devices()
> ```

So "can see" and "can use" are separable in this route: the list gives every device found for a backend, while `use` on each entry and `has_active_device()` / `get_num_gpu_devices()` say which of them are switched on.

**LABEL: documented** for routes one and two; **observed** for route three.

**Sources and addresses.**
- Manual, Help menu — https://docs.blender.org/manual/en/5.0/getting_started/help.html — read 2026-07-30.
- Manual, Common Problems (graphics hardware) — https://docs.blender.org/manual/en/5.0/troubleshooting/gpu/common/other.html — read 2026-07-30.
- API, `bpy.ops.wm` — https://docs.blender.org/api/5.0/bpy.ops.wm.html — read 2026-07-30.
- API, `gpu.platform` — https://docs.blender.org/api/5.0/gpu.platform.html — read 2026-07-30.
- Cycles add-on file `cycles_5.0/properties.py`, lines 1558–1566, 1581–1583, 1606, 1653–1660, 1687–1690, 1709–1721, 1728–1754 — read 2026-07-30.
- Cycles add-on file `cycles_5.0/engine.py`, lines 152–154 — read 2026-07-30.

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 5.0.

**Route one, the system information report.** **Help ‣ Save System Info** was used to write the file. Its section headed `Cycles:` read, in full:

```
CPU device capabilities: 

Metal device capabilities:
Number of devices: 1
		Device: Apple M1 Pro (GPU - 14 cores)
```

Its section headed `GPU:` read:

```
renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:
```

followed by a section headed `Implementation Dependent GPU Limits:` reading `Maximum Batch Vertices:	-2147483648` and `Maximum Batch Indices:	-2147483648`. Only those sections were copied back, so the rest of the file is not recorded here. This was the menu route with the window open; the operator route with no window is not covered by this run.

**Route two, `gpu.platform`,** from Blender's Python Console (Scripting workspace, window open). `backend_type_get()` returned `'METAL'`; `device_type_get()` returned `'APPLE'`; `renderer_get()` returned `'Metal API'`; `vendor_get()` returned `'Apple M1 Pro'`; `version_get()` returned `'1.2'`. `gpu.platform.devices_get()` raised:

```
AttributeError: module 'gpu.platform' has no attribute 'devices_get'
```

so that function is not present in 5.0.

**Route three, the Cycles add-on's own functions,** from the same console. The add-on key was found to be the literal string `cycles`. With `p = bpy.context.preferences.addons["cycles"].preferences`, `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned `[]` before anything else was called, and after `p.refresh_devices()` returned:

```
[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

`p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, `p.has_multi_device()` returned `False`. So on this machine the "can see" list and the "can use" flags came back as two entries, with the processor's flag off and the graphics card's on, and the identifier was `CPU` for the processor and `METAL_Apple M1 Pro (GPU - 14 cores)` for the graphics card. One run records nothing about whether those identifiers are stable.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender` (`Blender 5.0.1`).

**Route one with no window open.** `-b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='/tmp/sysinfo-terminal-50.txt'))"` printed `RESULT: {'FINISHED'}` and wrote the file. Its `Cycles` section was identical to the one the Help menu produced. Its `GPU` section was a single line:

```
GPU: missing, background mode
```

**Route two with no window open.** `import gpu; print('INIT:', gpu.init())` raised `AttributeError: module 'gpu' has no attribute 'init'`, and calling the platform functions raised `SystemError: GPU functions for drawing are not available in background mode`. So route two is unavailable without a window in this version — the `'NONE'` that `backend_type_get` can return was never reached, because the call itself refuses.

**Route three with no window open** answered. The key printed as `['cycles']`; the stored `devices` collection was **already populated** before `refresh_devices()` was called, which is the opposite of what the same reading showed inside the window; and the raw call gave:

```
TYPES: (False, False, False, True, False, False)
METAL: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False, True, True, False, True))
```

so the entries are eight fields long, not three. The second entry point this answer quotes — `available_devices()` with no argument — is refused:

```
TypeError: function takes exactly 1 argument (0 given)
```

**The whole file, both routes.** The file written from the Help menu has since been read end to end. Its headings, in order: `Blender:`, `Python:`, `paths:`, `Python (External Binary):`, `Directories:`, `scripts:`, `pref scripts:`, `FFmpeg:`, `Other Libraries:`, `GPU:`, `extensions:`, `Implementation Dependent GPU Limits:`, `GLSL:`, `Cycles:` (with `Metal device capabilities:` inside it), `Enabled add-ons:`. **There is no section headed `OpenGL`.** The background-written file drops `extensions:`, `Implementation Dependent GPU Limits:` and `GLSL:` altogether and replaces the `GPU:` section with the single line `GPU: missing, background mode`; its `Cycles:` section is identical to the menu-written one. So this report answers the Cycles half of the question with or without a window, and the drawing-card half only with one.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-5.0.1-linux-x64/blender`.

**Route one with no window open.** `-b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='…'))"` printed `RESULT: {'FINISHED'}` and wrote the file. Its `GPU` section is one line, as on the Mac:

```
GPU: missing, background mode
```

Its `Cycles` section is where Linux differs, and it answers what "instruction sets and capabilities" contains. It is long — about ninety lines. Its opening and the two lines that carry the answer:

```
Cycles:
====================================


CPU device capabilities: AVX2

CUDA device capabilities:
	NVIDIA GeForce RTX 4090
		CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK			1024
		…
		CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR			8
		CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR			9
		…
```

So it does name each card, and it names the processor's instruction set (`AVX2`) where the Mac's line was empty. It does **not** distinguish cards that are switched on from cards merely present: the technology was saved as OptiX at the time and the section is still headed `CUDA device capabilities`, listing the one card once.

**Route two with no window open** was refused exactly as on the Mac:

```
AttributeError: module 'gpu' has no attribute 'init'
```
```
SystemError: GPU functions for drawing are not available in background mode
```

**Route three with no window open** answered, and this machine can say what the Mac could not — what the list looks like for a technology with no supported card:

```
TYPES: (True, True, True, False, True, False)
CUDA: (('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', False, False, True, False, True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True))
OPTIX: (('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', False, False, True, True, True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True))
HIP: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
ONEAPI: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
METAL: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
```

So `available_devices` for a technology with no supported card returns **the processor alone, and no error**, whether that technology is built in (HIP, oneAPI) or not (Metal). Each entry is an eight-field record, as on the Mac, so entries are indeed longer than the three the comment names. The no-argument form was refused: `TypeError: function takes exactly 1 argument (0 given)`.

**Whether the call is safe.** It did not crash Blender on either machine, in any version, across every run of two days. That is not the same as the warning being stale, and it is recorded as what happened rather than as a verdict.

**What Blender was built with, against what it can find.** Hiding the card for one command with `CUDA_VISIBLE_DEVICES=` left `TYPES:` unchanged while both card lists came back holding the processor alone. So Blender answers *what am I built with* and *what can I see now* separately.

**Not covered.** Whether the stored identifier is stable across restarts, driver updates or reordering of cards — it held steady across every run of both days, and on Linux it visibly carries the card's place on the bus, which is a hint and not a statement. What any of this returns on a machine with several cards. The sources do not show the format of the report's Cycles section — the manual describes it in one sentence and gives no sample — but that format is now recorded above for both machines. Nothing states whether asking for the device list can be done safely at any moment — the file carries a warning that it may not be:

> `# Be careful when deciding when to call this function, as Blender can crash with `_cycles.available_devices()` on some drivers.`

Nothing in the sources gives a command-line switch that prints the device list and exits; searches for `list devices`, `enumerate` and `query device` found nothing, and the command-line page documents `-E help` for engines and `--log-list-categories` for logging categories but nothing equivalent for devices.

**Outside the interface.** All three routes are now tested with no window open on two machines, and they agree. Route one works: `bpy.ops.wm.sysinfo(filepath=…)` returned `{'FINISHED'}` and wrote a file whose `Cycles` section was fully populated, though its `GPU` section read only `GPU: missing, background mode`. Route three works, and on both machines the stored list was **already populated** before `refresh_devices()` was called when preferences had been saved, and empty under `--factory-startup`. Route two does **not** work on either machine: the platform calls raised `SystemError: GPU functions for drawing are not available in background mode`, and `gpu.init` does not exist in this version to enable them.

**Operating system.** The routes themselves are not stated to differ. What they report does: `device_type_get` can return `'APPLE'`, `'NVIDIA'`, `'AMD'`, `'INTEL'`, `'SOFTWARE'`, `'QUALCOMM'` or `'UNKNOWN'`, and the Cycles device list can only contain devices for backends that exist on that operating system. `_cycles.get_device_types()` returns six flags — CUDA, OptiX, HIP, Metal, oneAPI, HIP RT — on every platform, and which come back true is left to the machine: `(False, False, False, True, False, False)` on the Creator's Mac and `(True, True, True, False, True, False)` on the rented Linux machine. All three routes were run on both machines and answered the same way, except that `gpu.platform` refuses with no window on both and was reached only inside the window on the Mac, reporting `'METAL'` and `'APPLE'`. Windows remains untested.

### Hand tests on the two-card machine — observed

Run on **2026-08-06** on the second rented Linux machine described under question 5 — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s on driver 580.95.05, sixty-one cores, **no display server at all**, over SSH — against `/opt/blender/blender-5.0.1-linux-x64/blender`. Label: **observed**. Not merged with the one-card rented machine's results or the Mac's.

**Every route answered with no window and no display server, and every one named both cards:**

```
BACKEND: OPTIX
TYPES: (True, True, True, False, True, False)
CUDA: (('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', False, False, True, False, True), ('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a', False, False, True, False, True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False, False, True, False, True))
OPTIX: (('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', False, False, True, True, True), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX', False, False, True, True, True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False, False, True, False, True))
HAS ACTIVE: True
NUM GPU: 2
MULTI: False
```

**`get_num_gpu_devices()` counts the cards and returns 2**, and **`has_multi_device()` returns `False` with two cards ticked** — so that name is not about having more than one card.

`bpy.ops.wm.sysinfo(filepath=…)` returned `{'FINISHED'}` and wrote its file with no display server; its `Cycles` section carries one `CUDA device capabilities:` heading and then the card's name **twice**, at lines 83 and 172 of the file.

### Hand tests on the four-card machine — observed

Run on **2026-08-06** on the third rented Linux machine described under question 5 — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, sixty-one cores, 197 GB of memory, **no display server**, over SSH — against `/opt/blender/blender-5.0.1-linux-x64/blender`. Label: **observed**. A fourth machine; not merged with the Mac's results, the one-card machine's or the two-card machine's. The full list is `Audits/Blender Research/The machine/Hand tests — four cards.md`.

**Every asking route named four cards.** With all four ticked under OptiX and saved, `blender -b --python scripts/ask.py` printed:

```
BACKEND: OPTIX
TYPES: (True, True, True, False, True, False)
CUDA: (('NVIDIA GeForce RTX 4060 Ti', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:07', False, False, True, False, True), ('NVIDIA GeForce RTX 4060 Ti', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:09', False, False, True, False, True), ('NVIDIA GeForce RTX 4060 Ti', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0b', False, False, True, False, True), ('NVIDIA GeForce RTX 4060 Ti', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0d', False, False, True, False, True), ('AMD EPYC 7R43 64-Core Processor', 'CPU', 'CPU', False, False, True, False, True))
OPTIX: (('NVIDIA GeForce RTX 4060 Ti', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:07_OptiX', False, False, True, True, True), ('NVIDIA GeForce RTX 4060 Ti', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:09_OptiX', False, False, True, True, True), ('NVIDIA GeForce RTX 4060 Ti', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0b_OptiX', False, False, True, True, True), ('NVIDIA GeForce RTX 4060 Ti', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0d_OptiX', False, False, True, True, True), ('AMD EPYC 7R43 64-Core Processor', 'CPU', 'CPU', False, False, True, False, True))
HAS ACTIVE: True
NUM GPU: 4
MULTI: False
```

**`get_num_gpu_devices()` counts the cards and returns 4. `has_multi_device()` returns `False` with four cards ticked**, in this version and in the other three. The two-card machine found the same with two ticked, so that name is not about how many cards there are, and four ticked cards do not make it true either. **The two machines agree.**

The `TYPES` reading is the same split the two-card machine found: `(True, True, True, False, True, True)` in 4.5 and 5.2, `(True, True, True, False, True, False)` in 5.0 and 5.1.

**Not covered here.** Anything needing Blender's window, this machine having no display server: `--gpu-device help` was not run here, and the Vulkan **Device** control still cannot be seen with more than one card.

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on. So this block speaks for the outside case alone, and none of the two-card machine's window tests could be repeated here either.

**Operating system — this block.** Linux, on the machine named at the top of it. It says nothing about macOS, where the project's only machine has one set of built-in graphics, and nothing about Windows, which no Windows machine has run.

**Not covered here.** Cards of two different makes, an NVLink pair, and anything a window would show.

**Record of looking.** Searched the whole API documentation for `device`, `get_devices`, `refresh_devices`, `available_devices`, `compute_device_type`, `CyclesPreferences`, `CyclesDeviceSettings` and `CyclesRenderSettings`. The result is worth stating plainly: the Python API documentation for 5.0 contains no page for `CyclesPreferences`, `CyclesDeviceSettings` or `CyclesRenderSettings`, and no entry for them in the general index; `CyclesRenderSettings` appears only as a type name on `bpy.types.Scene` and `bpy.types.Object`, and in the change log. `bpy.ops.preferences` contains nothing about devices. The 4.5-to-5.0 change log has no Cycles preferences entries at all. So route three exists only in the add-on's file, not in the documentation. Searched the manual for `System Info` — thirteen hits, twelve of them the same troubleshooting sentence, one the Help menu page, which I read in full. Read `gpu.platform` in full and checked the rest of the `gpu.*` modules by name (`gpu.capabilities`, `gpu.matrix`, `gpu.select`, `gpu.shader`, `gpu.state`, `gpu.texture`, `gpu.types`) — all concern drawing, not device enumeration. Checked `bpy.app.build_options`, which reports `cycles=True` and `cycles_osl=True` but names no GPU backend. Searched the command-line page for a device-listing switch and found none.

---

## Things a person with Blender open could settle in a few minutes

This list has since been run on two machines. What the Creator's own MacBook Pro settled is written into the answers above as **Hand test — observed**, and what the rented Linux machine settled as **Hand tests on the rented Linux machine — observed**. The two lists live at `Audits/Blender Research/The machine/Hand tests.md` and `Audits/Blender Research/The machine/Hand tests — Linux.md`. **A third machine has since been used as well** — a second rented Linux machine with two graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the two-card machine — observed** and **Hand tests on a machine with no display server at all — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`. *(Added 2026-08-06 by the citation check of the two-card material: this closing list still said two machines.)* **A fourth machine has since been used as well** — a third rented Linux machine with **four** graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the four-card machine — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

### Still open

Nothing on this list is still open. All five items were run on both machines and what came back is recorded under questions 2, 3, 4, 5 and 6 above. Item 5 was answered only as far as the Mac reached — every device unticked, and the technology set to `NONE`. The rented Linux machine reached the real case: with the technology set to one for which it has no card at all, a windowless render still printed nothing naming a missing device and completed on the processor, and the missing-requirement text was found only in the preferences panel.

### Cannot be run on the equipment available

The equipment is one MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, whose graphics are Apple's own and built in, and one rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4090, driver 580.105.08 — reached over SSH. No Windows machine has run it, there is no AMD or Intel card and no second card, and nothing may be installed on the rented machine. **Brought up to date 2026-08-06 by the citation check of the two-card material.** A third machine has since been used — a second rented Linux machine, Ubuntu 22.04.5 LTS, **two** NVIDIA GeForce RTX 4090s on driver 580.95.05, sixty-one cores, **no display server at all**, over SSH — so "no second card" no longer holds, and what it settled is in the two-card block above. There is still no pair of cards joined by NVLink: this pair talks through the host bridge. The one-card rented Linux machine was destroyed on 2026-08-06 and nothing can be run on it again. And its driver *was* removed and reinstalled on the two-card machine, on purpose, for the broken-driver test recorded in the first-pass findings for this version.

**A fourth machine has since been used, and it is the only one still alive.** A **third rented Linux machine** — Ubuntu 22.04.5 LTS, **four** NVIDIA GeForce RTX 4060 Ti of **16 GB each** on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, **197 GB of memory**, **no display server**, over SSH. What it settled is written into the answers above as **Hand tests on the four-card machine — observed**, and its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. Three limits above are narrowed by it and none is removed: more than two cards is no longer out of reach, a card small enough to be filled is no longer out of reach, and the several-card form of every question under 5 and 6 has now been asked twice on two different machines. **Still out of reach on it as well:** any pair joined by NVLink, these four talking through the host bridge; cards of two different makes; cards of unequal speed, all four being the same model within about 2 % of each other; and anything needing eyes on a window, this machine having no display server either. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

- **What `--cycles-device` does to the per-card switches when there is more than one card to choose between.** The one-card form is now settled on both machines: it overrides them. The several-card form needs two cards of one make.
- **Whether the figures the panel draws are enforced.** What it draws for Intel and AMD on Linux is now recorded under question 2. Enforcement needs a real Intel or AMD card with a driver below the figure, and the Intel Windows figure needs a Windows machine.
- **Whether the Metal requirement is macOS 12.2 or 13.0.** Needs an Apple Silicon Mac running macOS 12.x.
- **Whether the OpenGL floor Blender enforces is 3.3 or 4.3.** Needs a machine whose graphics support falls between the two; the rented machine reports `4.6.0 NVIDIA 580.105.08`.
- **Whether the compute-capability floor of 5.0 is enforced.** Needs a card between 3.0 and 5.0; this one reports major 8, minor 9.

---

2026-08-01 — Claude Opus 5 (Claude Code) — The machine second pass, Blender 5.0

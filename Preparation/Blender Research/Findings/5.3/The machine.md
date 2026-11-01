# The machine — findings for Blender 5.3

- **Sources consulted.** Blender 5.3 manual, Blender 5.3 Python API documentation and Blender 5.3 release notes, all as taken on 2026-10-11. Cycles' own files for 5.3, as taken on 2026-10-16. Also, inside the same folder: the folder's own note, and Blender's requirements page as copied on 2026-08-01 (used only in one flagged note under question 1). Work done on 2026-10-16.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

**Disclosure.** Before I began, the tool I run in showed me, unasked, the text of the project's `CLAUDE.md` (a signpost telling Blender researchers to stop and ignore it), a one-line-per-entry index of the user's saved notes, and a git status line naming one file. I did not open anything they point to, and nothing below comes from them. Apart from reading the question sheet, my only contact with the project outside `Blender Sources/` was checking, without reading anything, that the output folder existed and that no file was already at the output path. `Blender Sources/` also holds folders for other Blender versions (4.5 to 5.2). I did not read them.

**About the snapshot.** The release notes' front page says: "Blender 5.3 is currently in **Beta** until November 4, 2026." (https://developer.blender.org/docs/release_notes/5.3/, read 2026-10-11). Everything below describes 5.3 as it was documented during that beta.

**About the Cycles files.** They are Cycles' Python files (settings, panels, start-up code) and the source of its rendering kernels. The compiled part of Cycles, which actually picks and talks to devices, is not among them. So in many places they show what the settings and panels say, but not what the render engine does with them. I flag every place where I take that extra step.

---

## 1. What must a computer have on it before Blender will run at all?

**Answer in short.** The 5.3 sources name these things:
- the right operating system and processor family: Apple silicon on Mac; an x64 or arm64 variant matching the processor on Windows; the build "for your architecture" on Linux
- a graphics card and driver that meet a minimum graphics standard. The manual names OpenGL 3.3. The 5.3 release notes name Vulkan 1.1 as Vulkan's minimum.
- on Linux, a window system (X11 or Wayland). Rendering from the command line needs no graphical display.
- suitable user permissions, and no antivirus software blocking Blender

Nothing else has to be installed: Python comes bundled and no internet connection is needed. The manual refers to a separate list of "minimum and recommended requirements" but does not copy it. So exact operating-system versions, processor features, memory and disk space are not in the named sources.

**1a. Mac. Documented.**
> "Blender 5.0 and later only supports Macs with Apple silicon."

Manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/macos.html, read 2026-10-11.

**1b. Windows. Documented.**
> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU’s architecture."

Manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/windows.html, read 2026-10-11.

**1c. Linux. Documented.**
> "Download the Linux version for your architecture and decompress the file to the desired location"

> "Extracting Blender’s archive using 7-zip is not supported. TAR must be used instead."

Manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/linux.html, read 2026-10-11.

> "On Linux Blender supports both X11 and Wayland for official releases. When Wayland is detected, it is the preferred system, otherwise X11 will be used."

Manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/linux_windowing_environment.html, read 2026-10-11.

**1d. Graphics card and driver. Documented and inferred, but the sources give different kinds of figure.**

The manual says this on every one of its twelve graphics troubleshooting pages (Windows, Linux and macOS, each for NVIDIA, AMD, Intel and Other). For example:
> "Unsupported Graphics Driver Error … This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

Manual, https://docs.blender.org/manual/en/5.3/troubleshooting/gpu/windows/nvidia.html, read 2026-10-11. The same paragraph goes on to mention Blender 2.79 and 2.8, and it names no 5.x version.

The 5.3 release notes say:
> "Vulkan backend is now default on Windows (not ARM) and Linux."

> "Vulkan minimum requirements has been lowered to Vulkan 1.1 and some GPU/driver features have been made optional to support more devices."

Release notes, https://developer.blender.org/docs/release_notes/5.3/eevee/, read 2026-10-11. This is a release note about a change. I quote it as a documented fact about 5.3.

On Linux with NVIDIA, the manual says:
> "Note, drivers before 550 do not support Vulkan!"

Manual, https://docs.blender.org/manual/en/5.3/troubleshooting/gpu/linux/nvidia.html, read 2026-10-11.

The manual's install page says:
> "Always check that your graphics drivers are up to date and that OpenGL is properly supported. Blender has a set of minimum and recommended requirements; so make sure these are met before trying to install Blender."

Manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/index.html, read 2026-10-11.

How Blender draws can be chosen:
> "--gpu-backend … Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."

Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.

> "gpu_backend … GPU backend to use (requires restarting Blender for changes to take effect) (default 'VULKAN')"

API, https://docs.blender.org/api/5.3/bpy.types.PreferencesSystem.html, read 2026-10-11. The API page does not say whether this default differs by operating system.

*Inferred:* what the graphics card and driver must support depends on which drawing system (OpenGL, Vulkan or Metal) Blender is using. The step: Blender offers more than one drawing system, can be told which to use, and the sources give a separate minimum for OpenGL (3.3) and for Vulkan (1.1). The sources do not say which one a given machine starts with, apart from the release note's defaults. They also do not say what Blender does when its default is missing.

**1e. Software. Documented.**
> "Blender bundles a complete Python installation and does not interact with the system Python by default."

Manual, https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.

> "Blender is designed to not require an internet connection, so it doesn’t have a built-in update system."

Manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/index.html, read 2026-10-11.

> "System directories store files that come bundled with Blender and are required for it to function. This includes scripts, presets, essential assets and more."

Manual, https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html, read 2026-10-11.

**1f. Permissions and security software. Documented.** The startup troubleshooting page lists causes to check when Blender will not start:
> "See if your computer meets the minimum requirements."
> "Confirm that your graphics card is supported and that the drivers are up to date"
> "Make sure any antivirus software is not preventing Blender from starting."
> "Ensure that you have appropriate user permissions on the computer."

Manual, https://docs.blender.org/manual/en/5.3/troubleshooting/startup.html, read 2026-10-11.

**1g. A display is not needed for command-line rendering. Documented.**
> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html, read 2026-10-11.

**1h. Flagged note: a source outside the four named ones.** `Blender Sources/` also holds a text copy of Blender's requirements page, dated 2026-08-01. It is not one of the four sources the brief names, and I was not given an address for it. The folder's own note says it "describes whichever release was current on that date" and "can only speak for a version it names in so many words". It does not name 5.3. Its only line whose version range includes 5.3 is:
> "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."

That matches 1a and adds macOS 13. Its other figures are not tied to 5.3 by name. They include a minimum Windows version, processor cores and features, memory, graphics memory, OpenGL 4.3 / Vulkan 1.3, and a Linux glibc version. Those graphics figures also differ from the manual's OpenGL 3.3 and the release notes' Vulkan 1.1. I report this so it is not lost. Whether it counts is for the reader to decide.

**Label summary:** 1a–1g documented; the backend point in 1d is inferred, with the step written out; 1h comes from outside the named sources.

**Not covered.**
- The named sources do not give the minimum Windows, macOS or Linux version, processor instruction sets, memory or disk space for 5.3.
- They do not say whether the graphics minimum (1d) still applies when Blender runs without its interface. *(Hand tests, 2026-10-17: not reached — the only machine available is far above every figure. See below.)*
- They do not say which drawing system a particular machine ends up on, or what happens when that system is not available. *(Hand tests, 2026-10-17: answered for one machine only, the Creator's Mac on macOS — a fresh 5.3 drew with Metal, and `--gpu-backend vulkan` or `opengl` was refused with `Error: Unrecognized GPU backend for '--gpu-backend', expected one of [metal].` and Blender carried on with Metal. Observed; see below.)*
- The OpenGL 3.3 figure sits in a paragraph that names no 5.x version.

**Where I looked for what is missing.** I read the installing pages (index, Linux, macOS, Windows, Steam, Linux windowing), the troubleshooting startup and crash pages, all twelve graphics troubleshooting pages, the hardware configuration page, "Deploying Blender" and "Blender's Directory Layout". I searched the whole manual, API and release notes for: "minimum requirement", "system requirements", "Ventura", "macOS 1x", "Windows 8/10/11", "glibc", "SSE4", "AVX", "8 GB", "RAM", "redistributable", "vcruntime", "Visual C", "fallback / fall back". The install pages send readers to "the Downloading Blender page" for the minimum requirements, and that page is not part of the manual. The only hits on Windows versions were about virtual-reality headsets (Windows Mixed Reality), a pen-tablet input option ("Windows Ink"), and an NVIDIA device note. They are not about running Blender at all.

**Outside the interface.** Command-line rendering needs no graphical display (1g). The sources do not say whether the graphics card and driver minimum (1d) applies in background mode. The operating-system and processor items (1a–1c) are stated for Blender in general, with no exception for background use. *Inferred:* they therefore apply however Blender is started. *(Hand tests on the Creator's Mac, 2026-10-17 — observed: started from a terminal with no window, 5.3 ran, rendered under all three engines on the card and on the processor, and ended by itself; after `gpu.init()` it reported drawing with Metal. With a window it drew with Metal too. The graphics minimum itself was not tested, that Mac being far above it, and no machine without a display was available. See the hand tests below.)*

**Operating system.** Each item is tied to its own system as quoted. The release note's Vulkan default covers Windows (not ARM) and Linux. Metal is one of the drawing options, but the named sources do not say when it is used. *(Hand tests, 2026-10-17 — observed on macOS 26.6.2 on Apple silicon only: a fresh 5.3 drew with Metal, its backend preference read `METAL` although the API gives `'VULKAN'` as the default, and its `--help` offered `'metal'` alone for `--gpu-backend`. No Windows or Linux machine was available, so the Vulkan default stated for them is untried here.)*

### Hand tests on the Creator's Mac — observed, 2026-10-17

All of this was run on **2026-10-17** on **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2** — against **Blender 5.3.0 Beta**, built 2026-10-16, hash `5d8151335b6b`, at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, started from a terminal. Every run was pointed at a settings folder of its own (`BLENDER_USER_RESOURCES`), empty before its test began and never the Creator's own, and Blender's temporary folder (`TMPDIR`) was pointed at the test's own work folder. **This covers macOS on Apple silicon only**: no Linux or Windows machine and no other make of graphics card was available. Entry numbers are those of the hand-test list for this topic and version; each result below says what was done, so it can be repeated from here.

**What this copy is, what it carries, and the engines it lists (entry 1).** `--version` printed `Blender 5.3.0 Beta`, `build hash: 5d8151335b6b`, `build platform: Darwin`. With no window, `-b --factory-startup --python-expr` printing the version, `_cycles.get_device_types()`, two of `bpy.app.build_options` and Blender's user folder gave, exit status 0:

```
VERSION 5.3.0 Beta b'5d8151335b6b'
TYPES (False, False, False, True, False, False)
BUILD CYCLES True OSL True
USER FOLDER /private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/config/e1
```

`-b -E help` printed, exit status 0:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

Blender's own `--help` on this Mac lists, under `GPU Options:`:

```
--gpu-backend 
	Force to use a specific GPU backend. Valid options: 'metal'.
```

So on this Mac the one drawing system offered by name is `metal`; the manual's arguments page lists `vulkan, metal, opengl` (1d). The same help text also lists `--gpu-device`, and `--gpu-device help`, run with no file, printed and exited with status 0:

```
Blender GPU Device Listing (Vulkan):
Pass an Index or Device-ID to '--gpu-device'.
  (Vulkan backend not built)
```

Blender started, ran and quit with no window in every run. This Mac is far above every graphics figure the sources give, so nothing here tests a minimum.

**Every engine renders with no window, on the card and on the processor (entry 2).** The factory scene (cube, camera, light) at 64 by 64 was rendered with no window under each engine the listing names, once with `-- --cycles-device METAL` and once with `-- --cycles-device CPU`, as `-b plain.blend -E <engine> --debug-cycles -o <path> -f 1 -- --cycles-device <D>`. All six exited 0 and wrote a picture:

```
METAL BLENDER_EEVEE EXIT 0
METAL BLENDER_WORKBENCH EXIT 0
METAL CYCLES EXIT 0
CPU BLENDER_EEVEE EXIT 0
CPU BLENDER_WORKBENCH EXIT 0
CPU CYCLES EXIT 0
-rw-r--r--@ 1 artist     wheel  3882 Oct 17 17:11 CPU-BLENDER_EEVEE-0001.png
-rw-r--r--@ 1 artist     wheel  4141 Oct 17 17:11 CPU-BLENDER_WORKBENCH-0001.png
-rw-r--r--@ 1 artist     wheel  5060 Oct 17 17:11 CPU-CYCLES-0001.png
-rw-r--r--@ 1 artist     wheel  3882 Oct 17 17:11 METAL-BLENDER_EEVEE-0001.png
-rw-r--r--@ 1 artist     wheel  4141 Oct 17 17:11 METAL-BLENDER_WORKBENCH-0001.png
-rw-r--r--@ 1 artist     wheel  5038 Oct 17 17:11 METAL-CYCLES-0001.png
```

What Cycles said about where it worked is quoted under question 2. EEVEE and Workbench printed nothing naming a device; the whole EEVEE run with `METAL`, word for word:

```
00:00.332  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/files/plain.blend"
00:00.337  render           | Rendering single frame
00:00.338  render           | Rendering frame 1
00:00.343  render           | Start rendering: Scene, ViewLayer
00:00.345  render           | Engine: EEVEE
00:00.521  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.717  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.881  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.974  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.080  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e2/METAL-BLENDER_EEVEE-0001.png'
00:01.080  render           | Time: 00:00.74 (Saving: 00:00.08)

Blender quit
```

No line in any of the six logs contains `EGL`.

**The drawing system with a window, from a fresh start (entry 24; the macOS half of the researcher's test 9).** 5.3 was opened with its window from a terminal, with `--factory-startup` and a never-used settings folder, and a script inside it waited three seconds and printed:

```
WINDOW BACKEND: METAL | DEVICE TYPE: APPLE | RENDERER: Metal API | VENDOR: Apple M1 Pro | VERSION: 1.2
WINDOW DEVICES: [(0, 'Metal API', 'METAL')]
SYSTEM PREF gpu_backend = 'METAL' | CHOICES: ['OPENGL', 'METAL', 'VULKAN']
SYSTEM PREF gpu_preferred_device = 'METAL' | CHOICES: ['AUTO']
```

So the drawing system in use was Metal, and the backend preference read `METAL` — not the API page's stated default `'VULKAN'` (1d) — while listing three choices. The preferred-device setting read `'METAL'` while its listed choices were only `['AUTO']`. The script then opened Preferences ‣ System and Blender's own screenshot command photographed it. The page held, top to bottom: **Cycles Render Devices**, **Network**, **Memory & Limits**, **Video Sequencer**, and **Sound** (collapsed). **There was no Display Graphics section and no backend or device control for drawing anywhere on the page.** (Two of the four pictures meant to cover the page came out plain grey, taken while the page was redrawing after a scroll; the other two cover the page from its top to its last section, Sound.) The main window behind it showed the splash screen with its Quick Setup.

**Asking for a drawing system this Mac's copy does not have (entry 25).** 5.3 was opened with its window, from a fresh settings folder and `--factory-startup`, three times, with `--gpu-backend metal`, `--gpu-backend vulkan` and `--gpu-backend opengl`; a script inside it reported the drawing system in use, photographed the window and quit. For `vulkan`, word for word:

```
Error: Unrecognized GPU backend for '--gpu-backend', expected one of [metal].
BACKEND IN USE: METAL | DEVICE TYPE: APPLE | RENDERER: Metal API | VERSION: 1.2
PREF gpu_backend: 'METAL'
SHOT {'FINISHED'}
00:03.726  blend            | Saved session recovery to "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/tmp/quit.blend"
Blender quit
```

exit status 0. `opengl` printed the same `Error:` line and then the same lines; `metal` printed no `Error:` line. The three pictures are identical byte for byte — the main window with its splash screen. The same three with no window, after `gpu.init()`, printed `BACKGROUND BACKEND IN USE: METAL Metal API`, exit status 0, with the same `Error:` line for `vulkan` and `opengl`. So on this Mac `vulkan` and `opengl` were refused with that line and Blender carried on with Metal; the "fail instead of falling back" option `--debug-gpu-backend-no-fallback` that its `--help` lists was not tried.

**The drawing system with no window (entry 7).** With no window, `gpu.init()` returned `None` without error, and after it `gpu.platform` answered `METAL APPLE Apple M1 Pro Metal API 1.2 [(0, 'Metal API', 'METAL')]` (backend, device type, vendor, renderer, version, devices). Without `gpu.init()` the same call raised `SystemError: GPU functions for drawing requires the gpu module to be initialized. See gpu.init.` The full record is under question 6.

---

## 2. What must a computer have on it before Blender will render using its graphics cards?

**Answer in short.** It depends on the render engine.

**Cycles** needs four things:
- a supported card from one of five families, with a recent enough driver, on the right operating system: NVIDIA through CUDA or OptiX, AMD through HIP, Intel through oneAPI, or Apple through Metal
- a Blender build that includes GPU rendering
- a backend chosen in the preferences, with cards ticked there
- the scene set to "GPU Compute", or a command-line override instead of the last two

Users normally do not need the CUDA toolkit. Where the manual and Cycles' own files give different driver minimums, both are quoted below.

**EEVEE** renders only on a graphics card. Beyond the general graphics minimum under question 1, the sources give it no figures of its own.

**Workbench**: the sources say nothing about what it needs.

**2a. Cycles, per card family. Documented (manual) and observed (Cycles' files).**

From the manual, https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11:
> "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher."

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 575."

> "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."
> "Minimum driver versions: Windows: Radeon Software 24.9.1 or Radeon PRO Software 24.Q4 Linux: Radeon Software 24.30 or ROCm HIP Runtime 6.3"

> "oneAPI is a computation library that is supported on Windows and Linux and requires an Intel® Arc™ graphics card with the Xe HPG architecture."
> "Windows: Intel Graphics Driver XX.X.101.5518 Linux: intel-level-zero-gpu package 1.3.27642, typically available through the intel-compute-runtime package XX.XX.34666.3"

> "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

From Cycles' own files, `properties.py`, class `CyclesPreferences`, method `_draw_devices`, lines 1958–1965, read 2026-10-16:
```
        optix_minimum_driver_version = "580"
        cuda_minimum_driver_version = "580"
        hip_minimum_adrenalin_driver_version = "24.9.1"
        hip_minimum_pro_driver_version = "24.Q4"
        hip_minimum_linux_driver_version = "24.30"
        hip_rocm_minimum_version = "6.3"
        oneapi_minimum_windows_driver_version = "XX.X.101.8306"
        oneapi_minimum_linux_driver_version = "XX.XX.37435.3"
```
The same method shows these requirement lines (lines 1972–2041). It uses `compute_capability = "5.0"` for both NVIDIA backends and `mac_version = "13.0"`:
- "Requires NVIDIA GPU with compute capability %s" / "and NVIDIA driver version %s or newer" (CUDA and OptiX)
- "Requires AMD GPU with RDNA architecture" (HIP, Windows and Linux)
- "Requires Intel(R) Arc(TM) GPUs or newer Intel(R) Graphics" (oneAPI, Windows and Linux); on Linux it also lists "  - oneAPI Level-Zero Loader"
- "Requires Apple Silicon with macOS %s or newer" (Metal)

*Observed difference between the two sources.* The figures do not all agree, and I do not choose between them:
- NVIDIA OptiX driver: manual 575; Cycles files 580.
- NVIDIA CUDA driver: no figure in the manual; Cycles files 580.
- Intel driver: manual Windows "XX.X.101.5518" and Linux "1.3.27642 / XX.XX.34666.3"; Cycles files "XX.X.101.8306" and "XX.XX.37435.3".
- Intel cards: manual "Intel® Arc™ … Xe HPG"; Cycles files "Intel(R) Arc(TM) GPUs or newer Intel(R) Graphics".
- AMD and Apple figures agree.

The two sets were taken five days apart (2026-10-11 and 2026-10-16).

The 5.3 release notes add:
> "Support for NVIDIA RTX Spark devices running Windows 11, including unified memory optimizations."

Release notes, https://developer.blender.org/docs/release_notes/5.3/cycles/, read 2026-10-11.

**2b. No extra toolkit normally. Documented.**
> "Normally users do not need to install the CUDA toolkit as Blender comes with precompiled kernels."

Manual, https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11.

**2c. The build must include GPU rendering. Documented.** Some Linux distribution packages leave it out. See 3a.

**2d. Settings that must be in place. Documented, observed for the defaults, and inferred.**
> "To enable GPU rendering: Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference. Enable the available devices for the chosen backend. Configure each scene to use the GPU Compute Device."

Manual, https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11.

> "None: When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."

Manual, https://docs.blender.org/manual/en/5.3/editors/preferences/system.html, read 2026-10-11.

Defaults, observed in Cycles' own files, read 2026-10-16:
- The scene's device setting starts at CPU. `properties.py`, class `CyclesRenderSettings`, lines 430–436: `device: EnumProperty( name="Device", … items=enum_devices, default='CPU', …)`.
- The preferences' backend starts at Metal on Apple silicon Macs and at None everywhere else. `properties.py`, `CyclesPreferences.default_device`, lines 1683–1690:
```
        # Default to selecting the Metal compute device on Apple Silicon GPUs
        # (drivers are tightly integrated with macOS so pose no stability risk)
        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
            return 5
        return 0
```
  (5 is Metal and 0 is None, from the list at lines 1696–1706.)

*Inferred:* on Windows and Linux, a fresh setup renders Cycles on the processor until both the preference and the scene are changed, or until the command line overrides them (see 5b). The step: both defaults point away from the GPU, and the manual says None means the CPU is used.

**2e. Denoising on the graphics card. Documented.** The manual gives separate conditions:
- "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards."
- "GPU accelerated denoising is available on discrete Radeon RX 6000 and newer GPUs on Linux, and discrete Radeon RX 7000 and newer GPUs on Windows."
- "GPU accelerated denoising is available on all supported GPUs." (Intel)
- "GPU accelerated ray-tracing and denoising is available on Apple Silicon."

Manual, https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11.

**2f. EEVEE. Documented and inferred.**
> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient."

Manual, https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html, read 2026-10-11.

*Inferred:* EEVEE needs at least the general graphics minimum under question 1d. The step: EEVEE renders only on the graphics card, and 1d is the only graphics minimum the sources give. The sources state no EEVEE-specific card or driver minimum.

**2g. Workbench. Not in the sources.**

**Not covered.**
- Which of the differing driver figures in 2a is the one 5.3 enforces. *(Hand tests, 2026-10-17: cannot be run — needs NVIDIA or Intel cards, and a Windows machine for the Windows lines; only the Creator's Mac was available, where a Metal device is present and no requirement line is drawn.)*
- Whether a command-line render leaves out cards that fail the driver check. The preferences code skips them (see 6a); the render engine's own code is not supplied. *(Hand tests, 2026-10-17: cannot be run — needs a card that fails the check.)*
- Any minimum amount of graphics memory.
- What Workbench and EEVEE need beyond the general minimum.

**Where I looked for Workbench and an EEVEE minimum.** I read every Workbench page (index, performance, sampling, lighting, color, options, display settings, grease pencil), the EEVEE index, limitations and performance pages, and the Viewport and System preference pages. I searched the manual, API and release notes for "EEVEE" or "Workbench" on the same line as "gpu", "graphics", "opengl", "vulkan", "driver", "cpu" or "background". The only Workbench hardware statement I found is about an optional shadow feature: "Requires Vulkan and a GPU with hardware raytracing support." (https://docs.blender.org/manual/en/5.3/editors/preferences/viewport.html)

**Outside the interface.** The card and driver needs are the same however Blender is started. The settings in 2d can be replaced from outside by the command-line option described in 5b. The requirement lines in 2a are text drawn in the Preferences window (see 4 and 6). *(Hand tests on the Creator's Mac, 2026-10-17 — observed: from a terminal with no window, Cycles rendered on the Metal card, named by its `Path tracing on:` report; but with a settings folder nothing had been saved into, a GPU scene rendered on the processor until a run had refreshed the card list and saved the preferences. `-- --cycles-device METAL` put it on the card whatever was saved. EEVEE and Workbench rendered with no window. See the hand tests below and under question 5.)*

**Operating system.** CUDA, OptiX, HIP and oneAPI: Windows and Linux only (manual). Metal: macOS on Apple silicon only. In the Cycles files, the AMD and Intel requirement lines are only written for Windows (`sys.platform[:3] == "win"`) and Linux (`sys.platform.startswith("linux")`) (`properties.py`, lines 1983–2037). The sources do not say whether any GPU backend works on Windows on arm64. *(Hand tests, 2026-10-17 — observed on macOS on Apple silicon only: the copy carries Metal and no other card technology, the panel row offers None and Metal only, and the technology accepts no other value. No Windows or Linux machine and no NVIDIA, AMD or Intel card was available.)*

### Hand tests on the Creator's Mac — observed, 2026-10-17

Run on **2026-10-17** on **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2** — against **Blender 5.3.0 Beta**, hash `5d8151335b6b`, at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, from a terminal, each run with an empty settings folder of its own and Blender's temporary folder pointed at the test's work folder. macOS on Apple silicon only.

**Which card technologies this copy carries (entry 1).** With no window, `_cycles.get_device_types()` printed `TYPES (False, False, False, True, False, False)`, which Cycles' own file unpacks as `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt` (6b): Metal only.

**Cycles renders on the card with no window, and says where (entry 2).** The factory scene at 64 by 64, with an empty settings folder, rendered with no window as `-b plain.blend -E CYCLES --debug-cycles -o <path> -f 1 -- --cycles-device METAL`, exit status 0, picture written. Its report, word for word:

```
00:00.340  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
00:00.340  cycles           | Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
…
00:00.628  render           | Fra: 1 | Remaining: 04:44.98 | Mem: 1393M | Sample 1/4096 (7 kernels to optimize)
…
00:00.830  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                            |                    Hardware Ray-Tracing: Off
                            | 
                            | Denoising on: Apple M1 Pro (CPU) (8 threads) [CPU]
                            | 
                            | Render Scheduler Summary
                            | 
                            | Mode: Headless
                            | Resolution: 64x64
```

The same with `-- --cycles-device CPU`, exit status 0, picture written:

```
00:00.336  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
…
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
                            |                    Hardware Ray-Tracing: Off
                            | 
                            | Denoising on: Apple M1 Pro (CPU) (8 threads) [CPU]
```

So the line that names where a render ran is `Path tracing on:`, printed with `--debug-cycles`; the `Added device` line appears whether or not the card is then used. EEVEE and Workbench also rendered with no window (question 1), printing nothing that names a device.

**The starting technology on this Mac (entry 4).** With no saved preferences, the Cycles technology read `BACKEND: METAL`, and the only values it accepted were `NONE` and `METAL` (`enum "ONEAPI" not found in ('NONE', 'METAL')`). A refresh listed the card ticked and the processor unticked. The full record is under question 5. The Windows and Linux half of 2d — a start at None — could not be tried.

**The Cycles panel on this Mac (entry 24).** In a fresh 5.3 opened with its window, Preferences ‣ System ‣ Cycles Render Devices, photographed by Blender's own screenshot command, showed a row of two buttons, **None** and **Metal**, with Metal selected; two tick boxes, **Apple M1 Pro (GPU - 14 cores)** ticked and **Apple M1 Pro** unticked; **Kernel Optimization** showing **Full**; and **MetalRT** showing **Auto**. No "No compatible GPUs found for Cycles" line and no "Requires…" line was on the page, a Metal device being present; so the Metal requirement line and its macOS figure were not drawn. The full record is under question 5.

**A fresh setup on this Mac rendered a GPU scene on the processor (entry 5).** With a settings folder nothing had ever been saved into, a Cycles scene whose own Device is `GPU` rendered with no window on `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]`, exit status 0, saying nothing about it. Once a run had refreshed the card list and saved the preferences, the same render ran on `Apple M1 Pro (GPU - 14 cores) (Metal) (display)`. With `-- --cycles-device METAL` it ran on the card whatever was saved. The full sequence is under question 5.

---

## 3. Does what a computer must have on it depend on how Blender was obtained and put there?

**Answer in short.** Yes, in several documented ways:
- some Linux distribution packages are built without Cycles GPU rendering
- the Windows installer needs administrator rights and the zip does not
- the Windows download must match the processor (x64 or arm64)
- the Steam route needs the Steam client and a Steam account
- the Python-module route (from PIP) needs a matching Python, ignores user preferences, and has its own GPU caveat
- building from source can involve the CUDA developer toolkit

The sources do not say whether the official, Steam, Microsoft Store and Snap builds differ in GPU support.

**3a. Linux distribution packages. Documented.**
> "Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

Snap:
> "snap install blender --classic"
> "Installing from this method has a benefit that updates to Blender are automatically installed."

Manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/linux.html, read 2026-10-11.

**3b. Windows. Documented.**
> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

> "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights." (the zip route)

Manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/windows.html, read 2026-10-11. The same page says to pick an x64 or arm64 variant (see 1b). The release note's Vulkan default is "Windows (not ARM)" (see 1d).

Microsoft Store:
> "is_microsoft_store_install … Whether this blender installation is a sandboxed Microsoft Store version (default False, readonly)"

API, https://docs.blender.org/api/5.3/bpy.types.PreferencesSystem.html, read 2026-10-11.

The bundled files sit in a different place for each route (zip, installer, Microsoft Store):
https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html, read 2026-10-11.

**3c. Mac. Documented.**
> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

Manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/macos.html, read 2026-10-11. This is said for the disk-image route.

**3d. Steam. Documented.**
> "Download and install the Steam client for your operating system."
> "Once it is installed, open the client and login to your Steam account, or create an account if you don’t already have one."

Manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/steam.html, read 2026-10-11.

**3e. Blender as a Python module. Documented, with two inferences.**
> "Blender as a Python Module isn’t provided on Blender’s official download page."
> "A pre-compiled bpy module is available via PIP."
> "Functionality controlled by command line arguments (shown by calling blender --help) isn’t accessible."
> "The user’s startup and preferences are ignored to prevent your local configuration from impacting script behavior. The Python module behaves as if --factory-startup was passed as a command line argument."
> "It’s possible other Python modules make use of the GPU in a way that prevents Blender/Cycles from accessing the GPU."
> "Blender’s typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render and a crash log is not written in the event of a crash."

API, https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html, read 2026-10-11.

> "Blender and the bpy module are only compatible with a single Python version."

Manual, https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.

*Inferred:*
- The computer must have that one Python version installed for this route. The step: the module runs inside an outside Python, and it works with only one Python version.
- The Cycles card choices saved in the user's preferences are not applied on this route unless the preferences are loaded on purpose. The step: Cycles' per-card settings are "Properties saved in preferences" (`properties.py`, line 1673), and this route ignores user preferences. The same page shows how to load them: "bpy.ops.wm.read_userpref()".

The command-line option `--cycles-device` is also unavailable on this route, by the line quoted above.

**3f. Building from source. Documented.**
> "If you build Blender yourself, try to download and install a newer CUDA developer toolkit."

Manual, https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11. Compare 2b ("Normally users do not need to install the CUDA toolkit …").

**3g. What a build contains varies. Documented and observed.**
> "Formats that can be compiled into Blender, not available on all systems:"

Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.

The API lists a `bpy.app.build_options` value with entries such as `cycles=True` and `cycles_osl=True`, shown as one example build (https://docs.blender.org/api/5.3/bpy.app.html, read 2026-10-11).

Cycles' own files report "OSL support disabled in this build" when a build lacks it (`__init__.py`, class `CyclesRender`, line 108, read 2026-10-16).

**Not covered.**
- Whether the official blender.org downloads for each system include every GPU backend. The manual describes the backends as supported but never says "included in every download". *(Hand tests, 2026-10-17: one copy only — the 5.3.0 Beta installed on the Creator's Mac, whose route is not recorded, carries Metal and no other card technology. Observed; see below. No Windows or Linux copy was available.)*
- Whether the Steam, Microsoft Store and Snap builds differ from the official ones in GPU support. *(Hand tests, 2026-10-17: cannot be run — none of those routes was on the machine.)*
- Whether the PIP module supports GPU rendering at all, or which backends it has. *(Hand tests, 2026-10-17: cannot be run — the module was not on the machine and nothing but Blender itself could be run.)*

**Where I looked.** All installing pages, "Deploying Blender", "Blender's Directory Layout", "Blender as a Python Module", the API pages for `bpy.app` and the system preferences, and the 5.3 release notes (all subjects). I searched for "Steam", "Snap", "Microsoft Store", "package manager", "PIP", "build", "compiled" and "toolkit".

**Outside the interface.** The Python-module route has no interface at all, and command-line options do not work on it (3e). The other routes give the same program. The sources do not say whether the Store's "sandboxed" status or Snap's packaging changes anything when Blender is started by another program. *(Hand tests, 2026-10-17: only one route could be tried, the macOS copy on the Creator's Mac, which ran and rendered when started from a terminal with no window — question 1.)*

**Operating system.** Snap and distribution packages: Linux. Installer, zip and Microsoft Store: Windows. Disk image: macOS. Steam: "on Linux, macOS, or Windows". The Python module: the sources do not say which systems. *(Hand tests, 2026-10-17 — observed on macOS only: the copy on the Creator's Mac carries Metal alone. No Windows or Linux copy was available.)*

### Hand tests on the Creator's Mac — observed, 2026-10-17

Run on **2026-10-17** on **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2** — against **Blender 5.3.0 Beta**, hash `5d8151335b6b`, the copy installed at `/Applications/Blender 5.3b.app`. How that copy was obtained is not recorded here, so this speaks for that one copy and not for a route.

**What that copy carries (entry 1).** `_cycles.get_device_types()` printed `TYPES (False, False, False, True, False, False)` — Metal and no other card technology — and `bpy.app.build_options` printed `BUILD CYCLES True OSL True`. Its `--help` offers one drawing system by name, `'metal'`, and `--gpu-device help` printed `(Vulkan backend not built)`. No other route (Steam, a package manager, the Python module, a build from source) was on the machine, so none was compared.

---

## 4. Does Blender say anything about what is missing when it will not start, or when it will not render?

**Answer in short.** Partly. The sources document:
- one named start-up error about graphics: "Unsupported Graphics Driver Error"
- console messages you see when Blender is started from a terminal
- crash log files
- a set of Cycles error messages
- requirement messages in the Preferences window that name the missing card or driver

Most exact console wording is not documented. When the command-line device override fails, Cycles' files show that Blender quits with exit status 1. Whether it prints why is not in the sources.

**When it will not start. Documented.**

> "Unsupported Graphics Driver Error … This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."
(Every graphics troubleshooting page, for example https://docs.blender.org/manual/en/5.3/troubleshooting/gpu/windows/nvidia.html, read 2026-10-11.)

> "Crash on Startup … Try running Blender from the command line, to see if any helpful error messages are printed."
(Same pages.)

> "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window. To ensure you can see any error messages that appear at launch, open a command-line window and navigate to the location where the Blender executable is, and run blender from that location."

> "found bundled python: {DIR} … This message indicates that Blender was able to find the Python library for the Python interpreter embedded within Blender. If this folder is missing or unable to be found, it is likely that an error will occur, and this message will not appear."

Manual, https://docs.blender.org/manual/en/5.3/troubleshooting/startup.html, read 2026-10-11.

> "When running on Intel 3rd, 4th or 5th gen iGPUs, the latest Intel driver will crash on startup."

Manual, https://docs.blender.org/manual/en/5.3/troubleshooting/gpu/windows/intel.html, read 2026-10-11. This describes a crash, not a message.

Crash logs:
> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash."
- Windows: "On a crash, a file is written based on the name of the currently loaded blend-file, so test.blend will create a file called test.crash.txt."
- Linux: "After a crash, a file named blender.crash.txt is written to the /tmp directory."
- macOS: "After a crash, the macOS Crash Reporter shows a window with backtrace after some time, or when Blender is opened again."
- Windows also has: "blender_debug_gpu.cmd and blender_debug_gpu_workaround.cmd log GPU-related errors."

Manual, https://docs.blender.org/manual/en/5.3/troubleshooting/crash.html, read 2026-10-11.

Pictures, documented for what they show only. Three manual pictures show console output at launch:
- `_images/advanced_command-line_launch_macos_mac.png` on https://docs.blender.org/manual/en/5.3/advanced/command_line/launch/macos.html shows the line "ndof: 3Dx driver not found". The path in the picture shows Blender 2.64.
- `_images/advanced_command-line_launch_linux_console.png` on https://docs.blender.org/manual/en/5.3/advanced/command_line/launch/linux.html shows lines such as "jack server is not running or cannot be started" and "AL lib: (WW) alc_initconfig: Failed to initialize backend "jack"". No version is visible.
- `_images/advanced_command-line_launch_windows_console.png` on https://docs.blender.org/manual/en/5.3/advanced/command_line/launch/windows.html shows "found bundled python: c:\blender\2.77\python". The path shows Blender 2.77.

The pictures show that Blender prints notes about missing optional pieces at launch and keeps going. They do not show 5.3's exact wording.

GPU module, when used from a script without the interface:
> "gpu.init() … Initializes the GPU module for background use. If the initialization fails, a SystemError will be raised."

API, https://docs.blender.org/api/5.3/gpu.html, read 2026-10-11.

**When it will not render. Documented, observed, inferred, and not in the sources.**

*Requirement messages in Preferences. Observed in Cycles' own files* (`properties.py`, `CyclesPreferences._draw_devices`, read 2026-10-16):
- When a backend has no usable card (line 1969): "No compatible GPUs found for Cycles". It is followed by the requirement lines quoted in 2a.
- When a card is listed but its driver is too old (lines 2053–2080), the card's name gets " (Disabled)", followed by one of these:
  - "Requires NVIDIA driver version %s or newer"
  - "Requires AMD Adrenalin driver %s or newer, or AMD Radeon Pro %s driver or newer"
  - "Requires ROCm HIP Runtime %s or newer, or AMD driver version %s or newer"
  - "Requires Windows driver version %s or newer"
  - "Requires intel-level-zero-gpu or intel-compute-runtime version %s or newer"
  - "Driver upgrade required" when no other line applies
- A card can also be marked " (Unoptimized Performance)" (line 2048).

*Cycles error messages. Documented.* The manual lists messages a user may see:
- "Error: Out of memory … This usually means there is not enough memory to store the scene for use by the GPU."
- "The NVIDIA OpenGL driver lost connection with the display driver"
- "Unsupported GNU version"
- "CUDA Error: Kernel compilation failed … This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed."

Manual, https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11.

*Output. Documented.*
> "If no valid output method is available, rendering operations will report errors."

Manual, https://docs.blender.org/manual/en/5.3/render/output/animation.html, read 2026-10-11.

*EEVEE. Documented.* "Increasing the size might fix the Shadow buffer full error." (https://docs.blender.org/manual/en/5.3/render/eevee/render_settings/performance.html, read 2026-10-11.)

*Command-line device override. Observed, inferred, and not in the sources.* `engine.py`, function `_parse_command_line`, lines 38–41, read 2026-10-16:
```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```
*Inferred:* if the requested device cannot be set, Blender ends with exit status 1 instead of rendering. The step: `sys.exit(1)` runs when the override call reports failure. What makes the call fail, and whether a message is printed first, sit in compiled code that is not supplied. **Not in the sources.**

*Ways to make Blender say more. Documented* (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11):
- `--log "render,cycles"` (an example from the page)
- `--log-level`, where "info: Information about devices, files, configuration, operations"
- `--log-file <filepath>`
- `--debug-cycles` ("Enable debug messages from Cycles.")
- `--debug-gpu`
- `--cycles-print-stats` ("Log statistics about render memory and time usage.")
- Help ‣ Save System Info (see question 6)

**Not covered.**
- The exact console wording for most start-up and render failures. *(Hand tests, 2026-10-17: the render-failure wordings reachable on the Creator's Mac are now quoted below — a refused `--cycles-device`, a crash, an unwritable output, a missing image texture. A start-up failure for want of a good enough card could not be reached there.)*
- Whether anything is printed when the scene asks for GPU Compute but no usable card is configured. The manual says the CPU is then used (see 2d), but not whether Blender says so. *(Hand tests, 2026-10-17 — observed on the Creator's Mac: nothing is printed; the render runs on the processor, which only the `Path tracing on:` report names. See below.)*
- Whether `--cycles-device` prints a reason when it fails. *(Hand tests, 2026-10-17 — observed on the Creator's Mac: it does — `ERROR Found no Cycles device of the specified type` or `ERROR NOTADEVICE is not a valid Cycles device.` — and exits with status 1. See below.)*

**Where I looked.** The troubleshooting section (index, startup, crash, 3D view, Python, all graphics pages), the Cycles GPU page, the output pages, the EEVEE limitations and performance pages, and the command-line pages. I opened the three launch pictures. In Cycles' files I read `engine.py` in full, `__init__.py` in full, and every message string in `properties.py` and `ui.py`. I searched the manual for "error", "message", "missing", "warning" and "fallback".

**Outside the interface.**
- *Inferred:* the requirement and "(Disabled)" messages appear only in the Preferences window. The step: they are produced by `_draw_devices`, which is called from the preferences panel's drawing code (`draw_impl`, lines 2099–2108) and from nowhere else in the supplied files. A script can still read whether each card passes the driver check (see 6a). *(Hand tests, 2026-10-17: not settled — on the Creator's Mac the no-card and old-driver cases cannot be reached, so neither the panel text nor its absence from a windowless run could be seen. The windowless fall-back runs there printed no such text.)*
- Console messages are visible when Blender is started from a terminal. On Windows, when Blender is started normally, "the Console Window will then be toggled off" (https://docs.blender.org/manual/en/5.3/advanced/command_line/launch/windows.html).
- Exit status: 1 after a failed `--cycles-device` (inferred above; *observed on the Creator's Mac, 2026-10-17, with the reason printed first*). Also observed there with no window: 0 for a render that fell back to the processor and said nothing, 1 for an output that could not be written, 139 for a crash, and 0 for a script that raised a Python error when `--python-exit-code` was not given. `--python-exit-code` sets the exit status for "a Python exception … (only for scripts executed from the command line)".
- In the Python-module route, "a crash log is not written in the event of a crash" (3e).

**Operating system.** Crash file names and places differ by system, as quoted. The batch files are Windows only. The AMD and Intel requirement lines exist only for Windows and Linux (see 2). The "lost connection with the display driver" entry describes a Windows time limit. *(Hand tests, 2026-10-17 — observed on macOS only: with no window, a crash wrote `<name of the loaded file>.crash.txt`, or `blender.crash.txt` with no file loaded, into Blender's temporary folder rather than beside the blend-file, and macOS's own crash reporter also wrote a `Blender-…ips` file and started its Problem Reporter. No Windows or Linux machine was available.)*

### Hand tests on the Creator's Mac — observed, 2026-10-17

Run on **2026-10-17** on **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2** — against **Blender 5.3.0 Beta**, built 2026-10-16, hash `5d8151335b6b`, at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, from a terminal. Every run was pointed at a settings folder of its own, empty before its test began and never the Creator's own, and Blender's temporary folder was pointed at the test's own work folder. macOS on Apple silicon only; this Mac carries no NVIDIA, AMD or Intel card, and its graphics are far above every minimum the sources give, so **nothing here reaches a start-up failure or a card below a requirement.**

**A failed `--cycles-device` says why, and exits 1 (entry 3).** A Cycles scene (scene Device `GPU`, 16 samples, 640 by 360) was rendered with no window as `-b machine-test-gpu.blend --debug-cycles -o <path> -f 1 -- --cycles-device <V>`. For `OPTIX`, `CUDA`, `HIP` and `ONEAPI` — none of which this copy carries (question 2) — each exited with status 1 and wrote no picture. The whole of the `OPTIX` run, word for word:

```
00:00.322  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/files/machine-test-gpu.blend"
00:00.331  render           | Rendering single frame
00:00.333  render           | Rendering frame 1
00:00.333  render           | Start rendering: Scene, ViewLayer
00:00.333  render           | Engine: Cycles
00:00.335  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
00:00.338  cycles           | Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
00:00.343  cycles           | Using default CPU kernels.
00:00.346  render           | Fra: 1 | Mem: 0M | Initializing
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.346  reports          | ERROR Found no Cycles device of the specified type

Blender quit
```

`CUDA`, `HIP` and `ONEAPI` printed the same lines, the same last line `ERROR Found no Cycles device of the specified type`, and only the times differ. A value that is not a device at all, `NOTADEVICE`, exited with status 1, wrote no picture, and printed only:

```
00:00.312  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

So on this Mac a refused value ends the run with exit status 1 after a line naming the reason, and a technology this copy lacks did not fall back quietly to the processor.

**A scene that asks for the card and gets the processor says nothing about it (entry 5; the researcher's test 3).** A Cycles scene whose own Device is `GPU` was rendered with no window in five states that each ended on the processor — no settings ever saved, every device saved off, the same with `--factory-startup`, the card saved ticked but `--factory-startup` given, and the technology saved as `NONE` — each exiting with status 0, writing its picture, and reporting `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]` (the states and commands are under question 5). The `NONE` render was given `--log-level info --log "cycles,render" --debug-cycles`; its whole output, 145 lines, was searched for `device`, `gpu`, `metal`, `none`, `fall` and `compute`, and every match was an ordinary progress line such as `Fra: 1 | Mem: 1M | Updating Mesh | Copying Mesh to device` or `Number of lights sent to the device: 1`. No line said a card was unusable, missing or switched off, or that the processor was used instead. The same render with `--log-level info --log "*" --log-file <file>` printed only three lines to the terminal and wrote 149 lines to the file, opening:

```
00:00.021  color_management | Using Blender config: "/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio"
00:00.106  translation      | Locale en_US used for translation
00:00.114  blend            | Read prefs: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/config/e5/config/userpref.blend"
```

and the only line in it naming the device that rendered was again `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]`. In none of these runs was the device used named anywhere but that line.

**A crash writes a file named after the loaded blend-file, into the temporary folder (entry 9).** A crash was forced with no window by `--python-expr "import ctypes; ctypes.string_at(0)"`, with Blender's temporary folder pointed at a work folder, three ways. With a copy of the factory scene named `crashme.blend` loaded — exit status `139`, and the whole output:

```
00:00.335  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e9/crashme.blend"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Writing: /private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/tmp/crashme.crash.txt
```

With no blend-file — exit status `139`, and `Writing: /private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/tmp/blender.crash.txt`. With `--disable-crash-handler` added straight after `-b` and the file loaded — exit status `139`, the `Read blend:` line and nothing else: no `Writing:` line, and no crash file written (the folder's `crashme.crash.txt` kept the time of the first run). The crash file sat in the temporary folder, not beside the blend-file, and opens:

```
# Blender 5.3.0, Commit date: 2026-10-16 00:30, Hash 5d8151335b6b

# backtrace
0   Blender                             0x000000010385f5c4 _ZN7blender33BLI_system_backtrace_with_os_infoEP7__sFILEPKv + 56
…
# Python backtrace
  File "/Applications/Blender 5.3b.app/Contents/Resources/5.3/python/lib/python3.13/ctypes/__init__.py", line 548 in string_at
  File "<string>", line 1 in <module>
```

macOS's own crash reporter also recorded all three: three new files appeared in `~/Library/Logs/DiagnosticReports`, `Blender-2026-10-17-171801.ips`, `Blender-2026-10-17-171801.000.ips` and `Blender-2026-10-17-171801.0002.ips`, including one for the run with `--disable-crash-handler`. And macOS's Problem Reporter (`com.apple.ProblemReporter`) started in the same second, `17:18:01`, and was found to be the front program fifteen minutes later (entry 23, question 7). Its window was not looked at.

**What the Cycles panel says on this Mac (entry 24).** With 5.3's window open, Preferences ‣ System ‣ Cycles Render Devices showed the two buttons None and Metal and, under Metal, the labels **Apple M1 Pro (GPU - 14 cores)** and **Apple M1 Pro**, with nothing added to either — no " (Disabled)", no " (Unoptimized Performance)" — and no "No compatible GPUs found for Cycles" or "Requires…" line under either button. A Metal device being present, the no-card and old-driver cases were not reached. The record is under question 5.

**A missing image texture is named, and the render still finishes (entry 17).** With a material's image file moved away and the texture shown by a control to reach the picture, a windowless EEVEE render printed `00:00.470  image.gpu        | ERROR Image file "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e17/tex.png" does not exist` and a Cycles render printed `00:00.349  cycles           | WARNING Image file /private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e17/tex.png does not exist.` and `00:00.350  cycles           | ERROR Failed to load 1 image files`. Both then saved a frame and exited with status 0. The full record is under question 7.

**An output it cannot write is an error and exit status 1 (entry 18).** A windowless render aimed at `/usr/smoozy-x_`, a folder the user cannot write to, printed `ERROR OpenImageIO write failed: Could not open file "/usr/smoozy-x_0001.png"` and `ERROR Render error (Undefined error: 0) cannot save: '/usr/smoozy-x_0001.png'`, wrote nothing, and exited with status 1. The full record is under question 7.

---

## 5. How is it decided which of a computer's graphics cards a render uses, can more than one card be used for the same render, and how is that decided from outside Blender's own interface?

**Answer in short.**

*Cycles.* Three settings decide together:
- the backend picked in Preferences ‣ System ‣ Cycles Render Devices (one of None, CUDA, OptiX, HIP, oneAPI, Metal)
- a tick per card in that list
- the scene's Device setting (CPU or GPU Compute)

More than one card can be used for one render. Memory is not pooled, except for NVIDIA cards joined by NVLink. The processor can be added to the cards.

From outside, there are two ways:
- the `--cycles-device` option, placed after `--`, overrides both the preferences and the scene. It picks a backend, not individual cards.
- a Python script run by the command line can set the same preferences and scene setting.

The saved preferences are still used for command-line renders.

*EEVEE* has no support for more than one card. The sources do not say how its card is chosen. The nearest setting is the drawing device under "Display Graphics".

*Workbench*: how its card is chosen and whether it can use multiple cards are not in the sources.

### 5a. Cycles, inside the interface

**Documented.**
> "Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference. Enable the available devices for the chosen backend. Configure each scene to use the GPU Compute Device."

> "Can multiple GPUs be used for rendering? Yes, go to Preferences ‣ System ‣ Compute Device Panel, and configure it as you desire."

> "Would multiple GPUs increase available memory? Typically, no, each GPU can only access its own memory. The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."

Manual, https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11.

> "GPU Compute: Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices. Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."

Manual, https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/index.html, read 2026-10-11.

> "Distribute Memory Across Devices … Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol. Currently only NVLink on NVIDIA GPUs is supported."

Manual, https://docs.blender.org/manual/en/5.3/editors/preferences/system.html, read 2026-10-11.

The picture `_images/editors_preferences_section_system.webp` on that page shows the "Cycles Render Devices" panel as a row of buttons, "None", "CUDA" and "HIP", with "None" selected. Only one button in the row is shown highlighted. With None selected, no card list is visible. Documented for what it shows.

**Observed, Cycles' own files, read 2026-10-16:**
- One backend at a time. In `properties.py`, lines 1710–1715, `compute_device_type: EnumProperty(name="Compute Device Type", description="Device to use for computation (rendering with Cycles)", …)` holds a single value.
- Per-card settings are kept in preferences. Lines 1669–1677:
  ```
  class CyclesDeviceSettings(bpy.types.PropertyGroup):
      # Runtime properties
      __slots__ = ("is_optimized", "meets_driver_requirement")

      # Properties saved in preferences
      id: StringProperty(name="ID", description="Unique identifier of the device")
      name: StringProperty(name="Name", description="Name of the device")
      use: BoolProperty(name="Use", description="Use device for rendering", default=True)
      type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
  ```
- A newly found card is ticked, unless it is the processor. Line 1791: `entry.use = entry.type != 'CPU'`.
- Cards are matched by a lasting ID, not by name. Lines 1773–1776 match on `device[2]` and the type. Line 1799 reads "# Layout of the device tuples: (Name, Type, Persistent ID)". Lines 1792–1794 update the name "in case it changed".
- Cards that fail the driver check are left out of the preferences list (lines 2120–2125):
  ```
            # device[8] == DeviceInfo.meets_driver_requirement
            # For more details see available_devices_func function in python.cpp
            if not device[8]:
                # Devices that do not meet the driver requirement are not used;
                # skip them.
                continue
  ```
- The processor is offered as an extra to the cards (lines 2090–2094):
  ```
                    # CPU is always listed last (by convention in get_devices()),
                    # see get_devices_for_type implementation.
                    # Grey it out if no GPU device is enabled, because CPU here
                    # is meant as a supplement to GPU rendering. For CPU-only
                    # rendering, select the "None" compute device type instead.
  ```
- How the panels decide between processor and cards. `ui.py`, lines 107–116:
  ```
  def use_cpu(context):
      cscene = context.scene.cycles

      return (get_device_type(context) == 'NONE' or cscene.device == 'CPU' or not backend_has_active_gpu(context))


  def use_gpu(context):
      cscene = context.scene.cycles

      return (get_device_type(context) != 'NONE' and cscene.device == 'GPU' and backend_has_active_gpu(context))
  ```

*Inferred:*
- Cards from two different backends cannot be used in the same render (for example an NVIDIA card through CUDA together with an AMD card through HIP). The step: the backend setting holds one value, and only cards of that type count as active GPUs (`get_num_gpu_devices`, lines 1840–1856).
- The render engine follows the same rule as the panels in `ui.py`. That rule is: cards are used only when the backend is not None, the scene says GPU, and at least one ticked card passes the driver check. This is supported for the None case by the manual's line in 2d. Otherwise it is only the panels' logic, because the engine's own code is not supplied.

### 5b. Cycles, from outside the interface

**Documented.**
> "Cycles add-on options must be specified following a double dash."
> "--cycles-device <device> … Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. Append +CPU to a GPU device to render on both CPU and GPU."
> "Example: blender -b file.blend -f 20 -- --cycles-device OPTIX"

Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.

**Observed.** `engine.py`, lines 16–20, read 2026-10-16:
```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```
Lines 28–29 skip the option entirely when there is no `--`:
```
    if "--" not in argv:
        return
```
The option is read when Cycles registers itself: `__init__.py`, line 146, calls `engine.init()`, which calls `_parse_command_line()` (`engine.py`, line 53). The supplied files do not say when, during Blender's start-up, that registration happens.

**Not in the sources:** when `--cycles-device` names a backend, which cards of that type it uses. All of them? Only those ticked in the saved preferences? The override itself is compiled code that is not supplied. *(Hand tests, 2026-10-17 — observed, one-card form only: on the Creator's Mac, with every device saved unticked, `-- --cycles-device METAL` rendered on the card. Which of several cards it takes needs a machine with several, which was not available. See the hand tests below.)*

**Documented.** Saved preferences carry into command-line work:
> "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them:"

Manual, https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html, read 2026-10-11. The sentence is about script settings, but its wording is general.

**Ambiguous in the sources.** `--factory-startup` is described in two ways:
- the arguments page: "Skip reading the startup.blend in the users home directory."
- the Python-module page: "The user’s startup and preferences are ignored … The Python module behaves as if --factory-startup was passed as a command line argument."

*Inferred, weakly:* `--factory-startup` may also skip the saved card choices. The step: the second page links ignoring preferences to that option. The sources do not settle this. *(Hand tests, 2026-10-17 — observed on the Creator's Mac: with the card saved ticked, a windowless render with `--factory-startup` ran on the processor while the same render without it ran on the card, and a script run with `--factory-startup` found the stored card list empty. See the hand tests below.)*

**Documented and observed.** A script can set these values. The API shows how an add-on's preferences are reached:
> "addon_prefs = preferences.addons[__name__].preferences"

API, https://docs.blender.org/api/5.3/bpy.types.AddonPreferences.html, read 2026-10-11.

The command line can run a script with `--python <filepath>` or `--python-expr <expression>` (arguments page). The values a script would set are named in Cycles' files:
- `compute_device_type` (lines 1710–1715)
- each entry of `devices` with its `use` flag (line 1717, lines 1669–1677)
- the scene's `cycles.device`, 'CPU' or 'GPU' (lines 32–36 and 430–436)
- the method `refresh_devices()` (lines 1819–1828)

The sources do not say whether a change made this way in background mode is saved for later runs. *(Hand tests, 2026-10-17 — observed on the Creator's Mac: `bpy.ops.wm.save_userpref()` called with no window wrote `userpref.blend` into the settings folder and the next run read the saved ticks back and rendered by them; a change made in the render's own run without saving took effect for that render and was not kept. See the hand tests below.)*

**Not in the sources:** any environment variable that limits which cards Cycles sees. I searched for "VISIBLE_DEVICES", "ZE_AFFINITY" and "CYCLES_". The only Cycles environment variable documented is `CYCLES_CUDA_EXTRA_CFLAGS`, a compiler option.

### 5c. EEVEE and Workbench

**Documented.**
> "There is currently no support for multiple GPU systems."

Manual, https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html, read 2026-10-11.

> "Device Vulkan … Specifies which GPU device to use for display drawing operations. This setting is useful for systems with multiple GPUs (e.g., integrated + discrete) where you want to force Blender to use a specific GPU for UI rendering. Changing the backend requires restarting Blender for the change to take effect. Auto: Automatically selects the most appropriate GPU based on system configuration and driver support."

> "Backend … Selects the graphics API used for drawing the interface and rendering display content."

Manual, https://docs.blender.org/manual/en/5.3/editors/preferences/system.html, read 2026-10-11.

> "gpu_preferred_device … Preferred device to select during detection (requires restarting Blender for changes to take effect) (default 'AUTO') … AUTO Auto – Auto detect best GPU for running Blender."

API, https://docs.blender.org/api/5.3/bpy.types.PreferencesSystem.html, read 2026-10-11. The page lists only "AUTO" as a value.

> "For the best performance, the dedicated GPU should be used for Blender. Which GPU to use for which application can be configured in your graphics driver or operating system settings."

Manual, for example https://docs.blender.org/manual/en/5.3/troubleshooting/gpu/windows/nvidia.html, read 2026-10-11.

**Not in the sources:** whether EEVEE's or Workbench's final render runs on the card chosen by "Device Vulkan" or `gpu_preferred_device`. The manual says these choose the card for "display drawing operations" and "UI rendering". It never says they choose the card for a final EEVEE render. It also gives no matching setting for OpenGL or Metal.

**Workbench. Not in the sources.** The sources do not say how Workbench chooses its graphics card or whether it supports more than one card.

**Outside the interface for EEVEE.**
- `--gpu-backend` picks the drawing system (documented).
- `gpu_preferred_device` can be set from a script, but it "requires restarting Blender" (documented). *Inferred:* it cannot take effect within the run that sets it.

**A second reading of "the same render".** If "the same render" means "the same image, split across separate runs", Cycles' files have a setting for that. "Use Sample Subset", described as "Render a subset of the specified max samples. Typically used for distributed rendering across multiple devices" (`properties.py`, lines 548–566). The API documents a "merge_images" operator that combines "OpenEXR multi-layer images rendered with different sample ranges into one image with reduced noise" (https://docs.blender.org/api/5.3/bpy.ops.cycles.html). Observed and documented, respectively. I note it only because the question can be read this way.

**Not covered.** Which cards `--cycles-device` uses *(one-card form observed on the Creator's Mac, 2026-10-17; the several-card form cannot be run there)*; whether `--factory-startup` drops saved card choices *(observed on the Creator's Mac, 2026-10-17: it set them aside)*; how EEVEE picks its card; how the compositor's "GPU" device picks a card (the manual offers "CPU" and "GPU" with no card choice).

**Where I looked.** The Cycles GPU, render settings and performance pages, the System and Viewport preference pages and the picture above, the command-line pages, "Scripting & Security", the Python-module page, the EEVEE limitations and performance pages, all Workbench pages, and the graphics troubleshooting pages. In Cycles' files: all of `properties.py`'s device code, `ui.py`'s device helpers, and `engine.py` and `__init__.py` in full. I searched for "multiple GPU", "multi-gpu", "more than one", "device index", "persistent id", "VISIBLE_DEVICES", "integrated", "discrete" and "dedicated".

**Outside the interface — summary.** Covered in 5b and 5c. A command-line render uses the saved preferences unless `--cycles-device` overrides them, and the override picks a backend, not single cards. A script can set everything the Preferences window can. For EEVEE, only the drawing system can be forced from the command line. *(Hand tests on the Creator's Mac, 2026-10-17 — observed with no window: the saved ticks decided the device, an empty saved list meant the processor, `--cycles-device METAL` displaced both the saved ticks and the scene's own Device and was not written into the preferences, `--factory-startup` set the saved ticks aside, and a script's change in the render's own run took effect. On that Mac `--gpu-backend` accepts `metal` alone. See below.)*

**Operating system.** Backends are per system, as in question 2. NVLink is an NVIDIA feature; no system is named for it. "Device Vulkan" applies where Vulkan is used. The release note makes it the default on Windows (not ARM) and Linux. The laptop advice appears on all four Windows graphics troubleshooting pages and on the Linux NVIDIA, AMD and Other pages. It is not on the Linux Intel page or on any macOS page. *(Hand tests, 2026-10-17 — observed on macOS on Apple silicon only, on a machine with one graphics device: every selection route above behaved as recorded below. Windows, Linux, several cards, NVLink and the Vulkan Device control could not be tried; the System page there has no Display Graphics section.)*

### Hand tests on the Creator's Mac — observed, 2026-10-17

All of this was run on **2026-10-17** on **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2** — against **Blender 5.3.0 Beta**, built 2026-10-16, hash `5d8151335b6b`, at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, from a terminal. Every run was pointed at a settings folder of its own (`BLENDER_USER_RESOURCES`), empty before its test began and never the Creator's own, and Blender's temporary folder was pointed at the test's own work folder. **This Mac has one graphics device**, so nothing here says which of several cards is used, how work is shared between them, or anything about NVLink; and it is macOS on Apple silicon only. "The card" below is the `Apple M1 Pro (GPU - 14 cores)` device; "the processor" is the `Apple M1 Pro` CPU device. Where a render ran is read from the `Path tracing on:` lines Cycles prints with `--debug-cycles`, quoted under question 2.

**`+CPU` uses the card and the processor together (entry 3).** The Cycles scene (scene Device `GPU`, 16 samples, 640 by 360) rendered with no window with `-- --cycles-device METAL+CPU` exited with status 0, wrote its picture, and reported:

```
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                            |                    Hardware Ray-Tracing: Off
                            |                  Apple M1 Pro (CPU) (7 threads) [CPU]
                            |                    Hardware Ray-Tracing: Off
                            | 
                            | Denoising on: Apple M1 Pro (CPU) (8 threads) [CPU]
```

Seven processor threads when sharing with the card; eight when the processor works alone (question 2). The values this copy cannot give — `OPTIX`, `CUDA`, `HIP`, `ONEAPI` — each stopped the run with exit status 1 and `ERROR Found no Cycles device of the specified type` (question 4).

**What the card settings hold, read by a script with no window, and what `--factory-startup` does to them (entry 4).** A script run with `-b --python` printed the add-on keys, `compute_device_type`, the row of technologies the panel would offer (`get_device_types`), the stored `devices` list before and after `refresh_devices()`, three counts, the `peer_memory` setting, and then tried to set the technology to `ONEAPI` and put it back. It was run three times on the same settings folder: empty; after a separate run had called `refresh_devices()` and `bpy.ops.wm.save_userpref()`; and with `--factory-startup` after that save. The first run, word for word (exit status 0):

```
USER FOLDER: /private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/config/e4
ADDON KEYS: ['io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'cycles', 'pose_library', 'bl_pkg']
BACKEND: METAL
ROW: ['NONE', 'METAL']
BEFORE: []
AFTER: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
HAS ACTIVE: True | NUM GPU: 1 | MULTI: False
…
PEER_MEMORY: ('Distribute memory across devices', False, 'Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it', False)
…
SET ONEAPI REFUSED: TypeError('bpy_struct: item.attr = val: enum "ONEAPI" not found in (\'NONE\', \'METAL\')')
BACKEND PUT BACK: METAL
```

The saving run printed `Writing user preferences: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/config/e4/config/userpref.blend"`, `Info: Preferences saved` and `SAVED STATE: METAL [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`, and `userpref.blend` then sat in that folder. After the save, the second run's `BEFORE:` was already filled — `[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]` — before any refresh. **With `--factory-startup` on the same folder, `BEFORE:` was `[]` again**, and the refresh filled it as in the first run. So with no window: the technology starts as `METAL` on this Mac with no saved preferences; the stored card list is empty until something refreshes it; a refresh lists the card ticked and the processor unticked; a save made with no window is read back by the next run; and `--factory-startup` starts from an empty list whatever was saved. The add-on key is the literal string `cycles`. The only technologies the setting accepts here are `NONE` and `METAL`. How `--factory-startup` acts on a render is in entry 5, below.

**What the saved settings make a windowless render do, and what overrides them (entry 5).** One settings folder, empty at the start, and one Cycles scene whose own Device is `GPU` (16 samples, 640 by 360), rendered with no window as `-b machine-test-gpu.blend --debug-cycles -o <path> -f 1`. Between renders the settings were saved by separate windowless runs that called `refresh_devices()`, changed the ticks or the technology, and called `bpy.ops.wm.save_userpref()`; each printed what it saved. Every render below exited with status 0 and wrote its picture. In run order, with each render's `Path tracing on:` line and last line:

- **a — nothing ever saved:** `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]`, ending `Fra: 1 | Mem: 18M | Finished`. With the technology at its starting `METAL` and the stored card list still empty (entry 4), the GPU scene rendered on the processor.
- **b — after a save of the refreshed list,** `SAVED STATE: METAL [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`: `Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]`, ending `Fra: 1 | Mem: 1410M | Finished`.
- **c — every device saved off,** `SAVED STATE: METAL [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', False)]`: `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]`, `Mem: 18M`.
- **c2 — the same saved state, with `--factory-startup`:** the processor, `Mem: 18M`. *This one cannot tell anything apart*: both the saved state and an empty list (a) give the processor.
- **c3 — added to settle what c2 could not.** The card saved ticked again (`SAVED STATE: METAL [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`), then rendered with `--factory-startup`: `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]`, `Mem: 18M`. The same saved state without `--factory-startup`: `Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) […]`, `Mem: 1410M`. **So `--factory-startup` set the saved card choice aside for the render.**
- **d — every device saved off, with `-- --cycles-device METAL` added:** `Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]`, `Mem: 1410M`.
- **e — the same, on a copy of the scene whose own Device is `CPU`:** `Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]`, `Mem: 1410M`. So the flag displaced both the saved ticks and the scene's own setting.
- **f — the saved state read back after d and e:** `READ BACK: METAL [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', False)]` — the flag was not written into the preferences.
- **g — the technology saved as `NONE` with the card ticked,** `SAVED STATE: NONE [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`, rendered with `--log-level info --log "cycles,render"`: `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]`, `Mem: 18M`, and no `Added device` line.

What was said about the fall-back to the processor in a, c, c2, c3 and g is under question 4: nothing. The card was named by `Added device` in every run except g, whether or not it was then used.

**A change a script makes inside the render's own run, never saved, took effect (entry 6).** A fresh settings folder; the same GPU scene; each render given `--python-expr` changing the preferences, then `--debug-cycles -o <path> -f 1`, with no window. Every run exited with status 0 and wrote its picture. In each, the script's own `SET TO:` line was printed after the render's lines, near the end of the output.

- **Every device saved off; the script set every device on:** `SET TO: METAL [('Apple M1 Pro', True), ('Apple M1 Pro (GPU - 14 cores)', True)]`, and
  ```
                              | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                              |                    Hardware Ray-Tracing: Off
                              |                  Apple M1 Pro (CPU) (7 threads) [CPU]
  ```
  ending `Fra: 1 | Mem: 18M | Finished`.
- **The card saved on; the script set every device off:** `SET TO: METAL [('Apple M1 Pro', False), ('Apple M1 Pro (GPU - 14 cores)', False)]`, and `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]`.
- **The card saved on; the script set the technology to `NONE`:** `SET TO: NONE [('Apple M1 Pro', False), ('Apple M1 Pro (GPU - 14 cores)', True)]`, and `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]`.
- Read back afterwards, the saved state was unchanged: `READ BACK: METAL [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`.

In the first run, with the card and the processor both named, the last line read `Mem: 18M` — the figure the processor-only renders above printed, not the `1410M` the card-only renders printed.

**The panel, with a window (entry 24).** 5.3 was opened with its window from a terminal, `--factory-startup`, a never-used settings folder, and a script inside it that printed the settings, opened Preferences ‣ System, changed the Cycles settings one at a time, and had Blender photograph the window after each. Nothing was clicked. Printed, with the window open:

```
ADDON KEYS: ['io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'cycles', 'pose_library', 'bl_pkg']
CYCLES BACKEND: METAL | ROW: ['NONE', 'METAL']
BEFORE REFRESH: []
AFTER REFRESH: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
CYCLES PREF bl_idname = '\x06'
CYCLES PREF compute_device_type = 'METAL' | CHOICES: []
CYCLES PREF peer_memory = False
CYCLES PREF metalrt = 'AUTO' | CHOICES: ['OFF', 'ON', 'AUTO']
CYCLES PREF use_hiprt = True
CYCLES PREF use_oneapirt = True
CYCLES PREF kernel_optimization_level = 'FULL' | CHOICES: ['OFF', 'INTERSECT', 'FULL']
…
SET ONEAPI REFUSED: TypeError('bpy_struct: item.attr = val: enum "ONEAPI" not found in (\'NONE\', \'METAL\')')
```

What the pictures show, read off them:

- **As found:** under **Cycles Render Devices**, a row of two buttons, **None** and **Metal**, Metal highlighted; a box with **Apple M1 Pro (GPU - 14 cores)** ticked and **Apple M1 Pro** unticked; **Kernel Optimization: Full**; **MetalRT: Auto**. Nothing else in the panel — no requirement line, no " (Disabled)" or " (Unoptimized Performance)" after either name, and no Distribute Memory Across Devices box.
- **The card unticked (Metal still selected):** both tick boxes empty; the **Apple M1 Pro** line and the **MetalRT** control drawn greyed; **Kernel Optimization** not greyed.
- **None selected:** the row alone, None highlighted; no tick boxes and no controls under it.
- **Metal selected again with the card ticked:** as found.

*(In a first run of this entry, the pictures taken after the card was unticked and after `ONEAPI` was tried still showed the None state of the picture before them: the window had not been redrawn after a change made by a script. The changes were run again in a second fresh Blender with a redraw asked for before each picture; the list above is from that second run, except the "as found" view, which is from the first run — the second run's own "as found" picture came out plain grey.)*

---

## 6. Can Blender be asked which graphics cards it can see and can use?

**Answer in short.** Yes, in three ways:
- the Preferences window lists, for the chosen backend, each card Cycles found. A card whose driver is too old is marked "(Disabled)" with the reason, and "No compatible GPUs found for Cycles" appears when there are none.
- "Save System Info" writes a file with a GPU section and a Cycles section, and can be run as an operator.
- from a script, Cycles' preferences give the list of cards, with name, type, lasting ID and whether each passes the driver check. The `gpu.platform` module names the card used for drawing.

Cycles' files warn that asking can crash Blender on some drivers. For that reason the list is not refreshed automatically at start-up.

**6a. Cycles' list of cards. Observed in Cycles' own files, `properties.py`, read 2026-10-16.**
- Lines 1762–1771:
  ```
      # Be careful when deciding when to call this function,
      # as Blender can crash with `_cycles.available_devices()` on some drivers.
      def get_device_list(self, compute_device_type):
          import _cycles
          device_list = _cycles.available_devices(compute_device_type)
  ```
- Lines 1796–1801: "# Gets all devices types to display in the preferences for a compute device type." / "# This includes the CPU device." / "# Layout of the device tuples: (Name, Type, Persistent ID)".
- Lines 1807–1809 copy two more facts onto each card: `entry.is_optimized = device[7]` and `entry.meets_driver_requirement = device[8]`. Other positions are named in comments and code: `device[3] == DeviceInfo.has_peer_memory` (line 2127), `device[4] == DeviceInfo.use_hardware_raytracing` (line 2131), and positions 5, 6 and 9 for denoiser and DLSS support (lines 1892, 1932, 1912).
- Lines 1819–1828:
  ```
      # Refresh device list. This does not happen automatically on Blender
      # startup due to unstable drivers that can cause crashes.
      def refresh_devices(self):
  ```
- Line 1830: "# Deprecated: use refresh_devices instead." This is about `get_devices`.
- `has_active_device()` and `get_num_gpu_devices()` (lines 1840–1876) count ticked cards of the chosen backend that pass the driver check.
- `engine.py`, lines 158–160, also offers `available_devices()`, which returns `_cycles.available_devices()`.

**6b. The Preferences window. Documented and observed.**
> "CUDA: If the system has a compatible NVIDIA CUDA device, it will be available as an option for rendering with Cycles."
(The same wording is used for OptiX, HIP, oneAPI and Metal.) https://docs.blender.org/manual/en/5.3/editors/preferences/system.html, read 2026-10-11.

The backend buttons appear only for backends that Cycles reports. `properties.py`, lines 1692–1708: `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()`, then one button per true value, after "None". Observed.

The per-card markings "(Disabled)", "(Unoptimized Performance)" and the requirement lines are quoted in question 4. Observed.

**6c. Save System Info. Documented.**
> "Save System Info … This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems."
> "GPU … Shows the GPU vendor, version and the capabilities of your hardware and driver."
> "Cycles … The instruction sets and capabilities of each hardware render device available for use with Cycles."

Manual, https://docs.blender.org/manual/en/5.3/getting_started/help.html, read 2026-10-11.

The graphics troubleshooting pages call the graphics part "The OpenGL section":
> "To find out which graphics card and driver Blender is using, use Help ‣ Save System Info inside Blender. The OpenGL section will have information about your graphics card, vendor and driver version."

(for example https://docs.blender.org/manual/en/5.3/troubleshooting/gpu/windows/nvidia.html, read 2026-10-11). The two pages name that section differently.

> "bpy.ops.wm.sysinfo(*, filepath='') … Generate system information, saved into a text file"

API, https://docs.blender.org/api/5.3/bpy.ops.wm.html, read 2026-10-11.

Cycles' files have `system_info()`, which returns `_cycles.system_info()` (`engine.py`, lines 178–180). Observed.

**6d. The card used for drawing. Documented.**

From https://docs.blender.org/api/5.3/gpu.platform.html, read 2026-10-11:
- "gpu.platform.devices_get() … Get all available GPU devices. … List of GPUDevice objects for each device."
- "gpu.platform.renderer_get() … Get GPU to be used for rendering."
- "gpu.platform.vendor_get()" and "gpu.platform.version_get()" give the vendor and the driver version.
- "gpu.platform.device_type_get() … Device type (‘APPLE’, ‘NVIDIA’, ‘AMD’, ‘INTEL’, ‘SOFTWARE’, ‘QUALCOMM’, ‘UNKNOWN’)."
- "gpu.platform.backend_type_get() … Backend type (‘OPENGL’, ‘VULKAN’, ‘METAL’, ‘NONE’, ‘UNKNOWN’)."

A `GPUDevice` has "index", "identifier" and "name" (https://docs.blender.org/api/5.3/gpu.types.html).

The page does not say whether "rendering" in `renderer_get` means EEVEE's final renders or the drawing of the screen.

**Not covered.**
- The exact layout of the Cycles section in the system-info file. *(Hand tests, 2026-10-17: recorded below for the Creator's Mac, one Metal device; nothing for several cards or another make.)*
- Whether `bpy.ops.wm.sysinfo` works in background mode. *(Hand tests, 2026-10-17 — observed on the Creator's Mac: it does, returning `{'FINISHED'}` and writing a file whose graphics part is one line, `GPU: missing, background mode`, with or without `gpu.init()` first. See below.)*
- How often the "can crash on some drivers" risk arises.
- Whether `gpu.platform.devices_get()` lists cards that Cycles can use but that cannot draw. *(Hand tests, 2026-10-17: cannot be run — the only machine available has one device, which both draws and renders.)*

**Where I looked.** The help page, the System preference page, the graphics troubleshooting pages, the API pages for `gpu`, `gpu.platform`, `gpu.types`, `gpu.capabilities`, `bpy.ops.wm`, the system preferences, `bpy.app`, `Addon` and `AddonPreferences`, and the API change log. I searched for "compute_device", "refresh_devices", "available_devices", "CyclesPreferences", "sysinfo" and "system_info". The API has no page for Cycles' preferences class. Its change log only lists a new member, "bpy.types.CyclesPreferences.has_dlss_gpu_devices".

**Outside the interface.** The Preferences listing (6b) is inside the interface. The script methods (6a) and `gpu.platform` (6d) can be called from a script started on the command line. For `gpu`, the API warns of "background mode where the GPU module can't be imported by default" (https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html). It offers "gpu.init() … Initializes the GPU module for background use. If the initialization fails, a SystemError will be raised." (https://docs.blender.org/api/5.3/gpu.html).

*Inferred:* a background script must call `gpu.init()` before asking `gpu.platform`. A `SystemError` there is itself an answer, meaning no drawing device could be started. The step: the two quoted lines together. *(Hand tests on the Creator's Mac, 2026-10-17 — observed: with no window, asking `gpu.platform` without `gpu.init()` raised `SystemError: GPU functions for drawing requires the gpu module to be initialized. See gpu.init.`; `gpu.init()` returned `None`, and after it every `gpu.platform` call answered. With a window open, the same calls answered without `gpu.init()`. Cycles' lists, `bpy.ops.wm.sysinfo` and `--gpu-device help` all answered from a terminal with no window. See below.)*

Cycles' list (6a) is not refreshed on start-up, so a script has to call `refresh_devices()` or `get_devices_for_type(...)` first. Observed from the comment above.

**Operating system.** The sources state no difference between systems for these queries. Which backends can appear depends on the system (see 2). "QUALCOMM" and "APPLE" appear among the device types. *(Hand tests, 2026-10-17 — observed on macOS on Apple silicon only: every route below answered there, naming one Metal device; `gpu.platform.device_type_get()` returned `'APPLE'`. No Windows or Linux machine was available.)*

### Hand tests on the Creator's Mac — observed, 2026-10-17

Run on **2026-10-17** on **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2** — against **Blender 5.3.0 Beta**, hash `5d8151335b6b`, at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, from a terminal, each run with an empty settings folder of its own and Blender's temporary folder pointed at the test's work folder. macOS on Apple silicon only; this machine has one graphics device.

**A command-line listing of drawing cards (entry 1).** Blender's own `--help` on this Mac lists `--gpu-device`, whose text says `'help' prints supported Vulkan devices and exits`. Run as `--gpu-device help`, it printed, with exit status 0:

```
Blender GPU Device Listing (Vulkan):
Pass an Index or Device-ID to '--gpu-device'.
  (Vulkan backend not built)
```

So on this Mac that route names no card.

**Cycles' own lists, asked by a script with no window (entry 4).** With an empty settings folder, a script run with `-b --python` printed, after `refresh_devices()` (exit status 0):

```
AFTER: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
HAS ACTIVE: True | NUM GPU: 1 | MULTI: False
DEVICE 'Apple M1 Pro' CPU | is_optimized: (not set) | meets_driver_requirement: (not set)
DEVICE 'Apple M1 Pro (GPU - 14 cores)' METAL | is_optimized: (not set) | meets_driver_requirement: (not set)
PEER_MEMORY: ('Distribute memory across devices', False, 'Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it', False)
TYPES: (False, False, False, True, False, False)
CUDA: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True, True, False),)
OPTIX: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True, True, False),)
HIP: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True, True, False),)
ONEAPI: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True, True, False),)
METAL: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True, True, False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False, True, True, False, True, True, False))
NOARG ERROR: TypeError('function takes exactly 1 argument (0 given)')
```

The `(not set)` marks are the script's own: `is_optimized` and `meets_driver_requirement` could not be read off the stored entries after a refresh. Each raw entry from `_cycles.available_devices(<type>)` is a record of ten values. A technology this copy lacks returns the processor alone and no error; `available_devices()` with no argument is refused. Before any refresh, in an empty folder, the stored list is `[]`; after a saved refresh it is filled at start (question 5). Nothing here could show what the lists do with several cards, cards that fail a driver check, or a card of another make.

**The drawing module with no window (entry 7; the desktop half of the researcher's test 4).** With no window and no `gpu.init()`, `-b --factory-startup --python-expr "from gpu import platform as g; print('NO INIT BACKEND:', g.backend_type_get())"` raised, and Blender still exited with status 0:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
SystemError: GPU functions for drawing requires the gpu module to be initialized. See gpu.init.
```

`print('INIT:', gpu.init())` printed `INIT: None`, exit status 0, with no error. After it, the researcher's own line — `import gpu; gpu.init(); from gpu import platform as g; print(g.backend_type_get(), g.device_type_get(), g.vendor_get(), g.renderer_get(), g.version_get(), [(d.index, d.name, d.identifier) for d in g.devices_get()])` — printed, exit status 0:

```
METAL APPLE Apple M1 Pro Metal API 1.2 [(0, 'Metal API', 'METAL')]
```

So with no window, after `gpu.init()`, the drawing side names one device, `Metal API` with identifier `METAL`, where Cycles' list names the card `Apple M1 Pro (GPU - 14 cores)` with identifier `METAL_Apple M1 Pro (GPU - 14 cores)` and the processor `CPU`. One device on one Mac; this says nothing about a machine with several.

**Save System Info with no window (entry 8; the researcher's test 5).** `-b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='<work folder>/sysinfo-bg.txt'))"` printed `RESULT: {'FINISHED'}`, exit status 0, and wrote the file. Its headings — the lines underlined with `=` signs — were:

```
= Blender 5.3.0 Beta System Information =
Blender:
Python:
Python (External Binary):
Directories:
FFmpeg:
Other Libraries:
Cycles:
Enabled add-ons:
```

Its graphics part is one line, at line 74: `GPU: missing, background mode`. Its `Cycles` section, in full:

```
Cycles:
=========================================


CPU device capabilities: 

Metal device capabilities:
Number of devices: 1
		Device: Apple M1 Pro (GPU - 14 cores)

```

The same with `gpu.init()` called first printed `INIT: None` and `RESULT: {'FINISHED'}`, exit status 0, and wrote a file that differs from the first in one line only — the temporary folder's name (`tempdir: '…/tmp/blender_trDQrx/'` against `'…/tmp/blender_oGPBuq/'`). So `gpu.init()` did not fill in the `GPU` section.

**Save System Info with a window open (entry 24).** In a fresh 5.3 opened with its window, a script called `bpy.ops.wm.sysinfo(filepath=…)`, which returned `{'FINISHED'}`. That file's headings were:

```
= Blender 5.3.0 Beta System Information =
Blender:
Python:
Python (External Binary):
Directories:
FFmpeg:
Other Libraries:
GPU:
Implementation Dependent GPU Limits:
Cycles:
Enabled add-ons:
```

and its `GPU` part, where the windowless file had the one line `GPU: missing, background mode`:

```
GPU:
=========================================

renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:

Implementation Dependent GPU Limits:
=========================================

Maximum Batch Vertices:	-2147483648
Maximum Batch Indices:	-2147483648

GLSL:
Maximum Varying Floats:	60
Maximum Vertex Attributes:	31
Maximum Vertex Uniform Components:	1024
Maximum Fragment Uniform Components:	1024
Maximum Vertex Image Units:	64
Maximum Fragment Image Units:	64
Maximum Pipeline Image Units:	64
Maximum Image Units:	64
```

Its `Cycles` section was the same as the windowless one's: `CPU device capabilities: `, then `Metal device capabilities:`, `Number of devices: 1`, `Device: Apple M1 Pro (GPU - 14 cores)`. There is no section headed `OpenGL`. It was written by the operator from a script with the window open, not by choosing Help ‣ Save System Info from the menu. The same script, with the window open, printed `WINDOW BACKEND: METAL | DEVICE TYPE: APPLE | RENDERER: Metal API | VENDOR: Apple M1 Pro | VERSION: 1.2` and `WINDOW DEVICES: [(0, 'Metal API', 'METAL')]` from `gpu.platform` without `gpu.init()`.

---

## 7. Can anything a render does wait for a person, and what happens on a computer with nobody at it?

*The question is ambiguous.* "Nobody at it" can mean a computer that is running but unattended, with a user logged in. It can also mean a computer with no display or no logged-in session ("headless"). I answer both where the sources allow.

**Answer in short.** The sources never say that a render itself stops to wait for an answer. They document several nearby things that do involve a person:
- a dialog when a file's scripts are blocked
- macOS asking approval the first time Blender opens
- a first-start setup on the splash screen
- still images from the interface that a person must save
- a crash window on macOS

They also document what to use when nobody is there:
- background mode, which needs no display
- an option that renders a frame and saves it
- options that allow scripts to run without asking

EEVEE cannot render on "headless Windows systems".

**7a. Scripts in the file. Documented.**
> "When a blend-file tries to execute a script and is not allowed, a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts."

> "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them: Enable with -y or --enable-autoexec Disable with -Y or --disable-autoexec"

> "To render an animation in background mode, allowing drivers and other scripts to run: blender --background --enable-autoexec my_movie.blend --render-anim"

> "There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off) … Rendering with Freestyle, because Freestyle uses scripts to control line styles."

Manual, https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html, read 2026-10-11.

The picture `_images/advanced_scripting_security_autorun-scripts-dialog.png` on that page shows a box reading "For security reasons, automatic execution of Python scripts in this file was disabled:", a text name, "This may lead to unexpected behavior", a checkbox "Permanently allow execution of scripts", and two buttons, "Allow Execution" and "Ignore". Documented for what it shows.

> "-Y, --disable-autoexec … Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.

**Not in the sources:** whether this dialog appears, or holds anything up, in background mode. *(Hand tests on the Creator's Mac, 2026-10-17 — observed: with no window it did not appear and held nothing up; the blocked driver became three lines of text, the run went on and wrote its frame, exit status 0. With a window it appeared as a dialog in the middle of the main window — worded differently from the manual's picture, with "Continue Safely" and no checkbox — and a render ran and wrote its frame with it unanswered. See below.)*

*Inferred:* in a background render without `-y`, and without the preference allowing it, drivers and registered scripts do not run. The step: "(default)" on `-Y`, plus "the Preferences are still used". The sources do not say whether the render then simply carries on, or how the image is affected. *(Hand tests on the Creator's Mac, 2026-10-17 — observed: it carried on, `bpy.app.autoexec_fail` read `True`, the driven value stayed at 0.0 where `-y` gave 4.0, and the two pictures differ. See below.)*

**7b. Still images in the interface need a person to save them. Documented.**
> "By default, animation renders are saved automatically, while still renders are not. Still images must be saved manually from the Image Editor or written using a File Output Node."

Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html, read 2026-10-11.

From outside:
> "-f, --render-frame <frame> … Render frame <frame> and save it."

(arguments page)

> "write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled)"

API, https://docs.blender.org/api/5.3/bpy.ops.render.html, read 2026-10-11.

**7c. First opening on a Mac. Documented.** "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time." (https://docs.blender.org/manual/en/5.3/getting_started/installing/macos.html). This is about opening Blender, not about a render.

**7d. First start. Documented.** "When starting Blender for the first time or updating to a new version, the “interactive region” contains a Quick Set Up Process." (https://docs.blender.org/manual/en/5.3/interface/window_system/splash.html). This is in the interface. The sources do not say whether background mode is affected. *(Hand tests on the Creator's Mac, 2026-10-17 — observed: every windowless run there used a never-used settings folder and none waited; with a window, a fresh start showed the splash and Quick Setup, and neither held up a render asked for from the command line or by a script. See below.)*

**7e. Typed input to scripts. Documented, with one inference.**
> "-d, --debug … Keeps Python’s sys.stdin rather than setting it to None"

(arguments page)

*Inferred:* without `--debug`, a script cannot sit waiting for typed input on the console. The step: the text says Python's input is "set to None" unless `--debug` is given. *(Hand tests on the Creator's Mac, 2026-10-17 — observed, and not what this inference says: without `--debug`, `sys.stdin` printed as `<_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>`, and a script calling `input()` while something held standard input open was still waiting when a 60-second limit ended it. See below.)*

**7f. Crashes. Documented.** "After a crash, the macOS Crash Reporter shows a window with backtrace after some time, or when Blender is opened again." (https://docs.blender.org/manual/en/5.3/troubleshooting/crash.html). Windows and Linux write files instead (see question 4). The command line offers "--disable-crash-handler … Disable the crash handler." and "--disable-abort-handler". The sources say nothing more about what these change. *(Hand tests on the Creator's Mac, 2026-10-17 — observed: with `--disable-crash-handler` a crash wrote no Blender crash file and printed no `Writing:` line, while macOS still wrote its own crash report. `--disable-abort-handler` was not tried. See question 4.)*

**7g. A computer with nobody at it. Documented.**
> "-b, --background … Run in background (often used for UI-less rendering). The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards."

(arguments page)

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

(https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html)

> "Headless Rendering … Headless rendering is not supported on headless Windows systems."

(EEVEE limitations, https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html)

> "The Blender application icon no longer shows up in the Dock when running in background mode via the command line."

(Release notes, https://developer.blender.org/docs/release_notes/5.3/user_interface/, macOS)

> "Tip … Closing the Console Window will also close Blender, losing any unsaved work."

(https://docs.blender.org/manual/en/5.3/advanced/command_line/index.html)

> "If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations. If you have a particularly heavy scene, Cycles can take up too much GPU time."

(https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, under "The NVIDIA OpenGL driver lost connection with the display driver")

> "Viewport renders are automatically paused during final renders."

(Release notes, https://developer.blender.org/docs/release_notes/5.3/cycles/)

In the Python-module route, "there is no special handling for Control-C to cancel a render" (3e).

**Not in the sources:**
- whether any part of a render waits for a person in background mode *(Hand tests on the Creator's Mac, 2026-10-17 — observed: nothing on the render path tried there waited; what did hold a windowless run was a script waiting on standard input and a script hooked into the render. See below.)*
- what happens to a render when the screen locks, the computer sleeps, or the user logs out *(cannot be run in this sitting: it needs the Creator's Mac to itself, put to sleep on purpose, and other work was going on there)*
- whether Cycles or Workbench render on a headless Windows system (only EEVEE is named) *(cannot be run: no Windows machine was available)*
- whether a graphics card with no monitor counts as "headless" *(cannot be run: no separate card was available)*

**Where I looked.** I searched the manual, API and release notes for "prompt", "confirm", "confirmation", "dialog", "wait", "waiting for", "interactive", "unattended", "user input", "without user", "headless", "no display", "graphical display", "X server", "remote", "SSH", "sleep", "screen saver", "lock screen", "logged in", "logout", "service", "daemon", "stdin" and "input()". I read "Scripting & Security" (and its picture), the Save & Load preferences, the splash and defaults pages, the output and animation pages, the Render menu entries (including "Lock Interface", described as "Lock interface during rendering in favor of giving more memory to the renderer."), the crash page and the command-line pages. The confirmation dialogs I found (the save prompt, deleting items, saving modified images) belong to working in the interface, not to rendering.

**Outside the interface.**
- Background mode (`-b`) is the documented way to render with nobody present. It needs no display.
- `-f` saves the frame.
- `-y` lets scripts run without the dialog.
- Audio is off by default.
- On macOS, no Dock icon appears.

Inside the interface, the script dialog, the splash setup and the manual saving of stills involve a person.

*(Hand tests on the Creator's Mac, 2026-10-17 — observed: with no window, the blocked-scripts warning, the save with modified images at `ASK`, writing over a frame, a missing texture, an unwritable output and Render Audio over an existing file each went ahead or failed without asking; Blender ended by itself after every windowless run. Two things held a windowless run: a script reading standard input while it was held open, and a script hooked into the render. The render outlived the shell it was started from, and was ended by a hang-up sent to its process group. A windowless render was not in the Mac's list of registered programs. With a window, the warning dialog, the splash and an unanswered file chooser held nothing up, but a Blender started with a window never quit by itself. See below.)*

**Operating system.**
- macOS: approval on first opening, Crash Reporter window, Dock change.
- Windows: closing the console window closes Blender, the GPU time limit, and EEVEE cannot render on headless Windows systems.
- Linux: no X server is needed for command-line rendering.

The sources say nothing about headless EEVEE on Linux or macOS. *(Hand tests, 2026-10-17 — observed on macOS on Apple silicon only: EEVEE rendered from a terminal with no window. That Mac had its screen on and a session running, so this is not a computer with nobody at it. No Windows or Linux machine was available.)*

### Hand tests on the Creator's Mac — observed, 2026-10-17

All of this was run on **2026-10-17** on **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2, with its built-in screen and the Creator's session logged in** — against **Blender 5.3.0 Beta**, built 2026-10-16, hash `5d8151335b6b`, at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, started from a terminal. Every run was pointed at a settings folder of its own (`BLENDER_USER_RESOURCES`), empty before its test began and never the Creator's own, and Blender's temporary folder (`TMPDIR`) was pointed at the test's own work folder. Runs that could hang were started under a time limit that ends Blender after a set number of seconds; exit status `142` means the limit ended it. "No window" means started with `-b`. Window tests were driven by a script inside Blender and photographed with Blender's own screenshot command; nothing was clicked. **This covers macOS on Apple silicon only.** No Windows or Linux machine was available, and **this is not a computer with nobody at it**: the screen was on and a session was running throughout.

**With no window, all three engines render and the run ends by itself (entry 2).** The factory scene at 64 by 64 rendered with no window under `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`, each once with `-- --cycles-device METAL` and once with `-- --cycles-device CPU`. All six exited with status 0, wrote their picture, and gave the prompt back by themselves; the six exit lines and pictures are quoted under question 1. No line in any of the six logs contains `EGL`.

**A crash with no window ends the run and leaves files behind; it does not wait (entry 9, 7f).** Three crashes forced with no window each ended at once with exit status `139`. Blender wrote its own crash file into its temporary folder unless `--disable-crash-handler` was given, and macOS's crash reporter wrote a `Blender-…ips` file into `~/Library/Logs/DiagnosticReports` for all three, with the handler disabled too. The record is under question 4. macOS's Problem Reporter started in the same second and was still the front program fifteen minutes later (entry 23, below); its window was not looked at, so what it showed and whether it waits for an answer is not recorded.

**A still, an animation, and a file with nothing to do: each ends by itself with no window (entry 10).** The settings folder, asked first, answered `use_scripts_auto_execute = False` and named itself as Blender's user folder. Under a 120-second limit: `-b plain.blend -o <path> -f 1` printed through `Saved: '…/s_0001.png'` and `Blender quit`, `EXIT 0 SECONDS 1`; `-b anim3.blend -o <path> -a` printed `Rendering animation (frames 1..3)`, saved `a_0001.png`, `a_0002.png` and `a_0003.png`, and `Blender quit`, `EXIT 0 SECONDS 2`; `-b plain.blend` with nothing asked of it printed only:

```
00:00.355  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/files/plain.blend"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

`EXIT 0 SECONDS 0`. The engine was EEVEE in all three, the factory one.

**A blocked scripted-expression driver, with no window: text on the terminal, and the render carries on (entry 11; the researcher's test 8).** A file holding the factory cube with a scripted-expression driver on its X position, `__import__('math').floor(4.5)`, was rendered with no window three ways under a 120-second limit: with no flag (so the preference decides, and it starts off), with `-Y`, and with `-y` before the file name. All three exited with status 0 and wrote a frame. With no flag, word for word:

```
00:00.322  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/files/driver2.blend"
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support

Error in PyDriver: expression failed: __import__('math').floor(4.5)
For target: (type=Object, name="Cube", property=location, property_index=0)

00:00.329  render           | Rendering single frame
00:00.329  render           | Rendering frame 1
00:00.330  render           | Start rendering: Scene, ViewLayer
00:00.330  render           | Engine: EEVEE
00:00.471  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.666  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.815  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.899  render           | Fra: 1 | Rendering 64 / 64 samples
00:00.980  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e11/none_0001.png'
00:00.981  render           | Time: 00:00.65 (Saving: 00:00.06)

Blender quit
```

`-Y` printed the same lines. `-y` printed no driver lines and saved its frame. Asked after `frame_set(1)` in each of the three conditions:

```
--- flag []
AUTOEXEC_FAIL True
AUTOEXEC_FAIL_MESSAGE "Driver '__import__('math').floor(4.5)'"
AUTOEXEC_FAIL_QUIET False
CUBE_X 0.0
--- flag [-Y]
AUTOEXEC_FAIL True
AUTOEXEC_FAIL_MESSAGE "Driver '__import__('math').floor(4.5)'"
AUTOEXEC_FAIL_QUIET False
CUBE_X 0.0
--- flag [-y]
AUTOEXEC_FAIL False
AUTOEXEC_FAIL_MESSAGE ''
AUTOEXEC_FAIL_QUIET False
CUBE_X 4.0
```

The no-flag frame and the `-y` frame, compared as pictures rather than as files: `PIXELS_DIFFER max_channel_difference=0.517647`. So with no window the blocked driver was not a question to anybody: it became three lines of text, the render went on with the driver not run, and the picture differs from the one with the driver run.

**A blocked registered text, with no window (entry 12).** A file holding a text named `Text.py` with Register on, whose only line prints `REGISTERED TEXT RAN`, rendered with no window and no flag, exit status 0, frame written, printing among the render lines:

```
scripts disabled for "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/files/regtext.blend", skipping 'Text.py'
```

With `-y` before the file name, `REGISTERED TEXT RAN` was printed straight after the `Read blend:` line, exit status 0, frame written. Neither run waited.

**Changes never saved, a second file loaded over them, and a save with a render result in memory, all with no window (entry 13).** Copies of the factory scene were used, under a 120-second limit each.

- *A change never saved, reaching Blender's own exit.* `-b u.blend --python-expr` moving the cube (`location.x = 3`) printed `CHANGED, NOT SAVED; is_dirty = False`, then `Blender quit`, exit status 0. The file's checksum was `429bdaab07ae506e382c7d08e2a28b375de6eaf3` before and after. **Blender's own changed-file flag, `bpy.data.is_dirty`, read `False` after the change.** Added to see whether an operator sets it: the same with `bpy.ops.mesh.primitive_cube_add(location=(3,0,0))` printed `OPERATOR CHANGE, NOT SAVED; is_dirty = False | objects 4`, exit status 0, the same checksum after. So with no window no change made here was counted by Blender as unsaved, and nothing was asked.
- *A second file loaded over that change.* `-b u.blend -o <work folder>/given_ --python-expr "<move the cube>; bpy.ops.wm.open_mainfile(filepath='<work folder>/plain2.blend'); …" -f 1`, where `plain2.blend` had been saved with its own output path `<work folder>/fromfile_`. It printed the second `Read blend:` line, rendered, and saved `'/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e13/fromfile_0001.png'` — the second file's own path, not the one given with `-o` before the script — then `is_dirty = False`, `SECOND FILE LOADED /private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e13/plain2.blend | OUTPUT PATH NOW /private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e13/fromfile_`, `Blender quit`, exit status 0. No `given_` file was written. The same with the operator change instead gave the same lines.
- *A save made with a render result in memory.* `-b c.blend --python-expr` rendering a still with `bpy.ops.render.render()`, moving the cube, and calling `bpy.ops.wm.save_mainfile()` printed:

  ```
  SAVE MODIFIED IMAGES PREF ASK
  IMAGES IN MEMORY ['Render Result', 'Viewer Node']
  Info: Saved "c.blend"
  SAVE RETURNED
  ```

  then `Blender quit`, exit status 0. The file's written time and size went from `Oct 17 17:19:59 2026 95754` to `Oct 17 17:20:04 2026 95816`, and a `c.blend1` of the old size appeared beside it. With the Save Modified Images preference at `ASK`, nothing was asked.

**Standard input is a real stream without `--debug`, and a script reading it can hold a windowless run open (entry 14; bears on the inference in 7e).** `-b --python-expr "import sys; print('STDIN IS', sys.stdin)"` printed, without `--debug` and with it:

```
STDIN IS <_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>
```

(the `--debug` run also echoed its own arguments, `argv[4] = import sys; print('STDIN IS', sys.stdin)`). So without `--debug`, standard input here was not `None`. A script calling `input()` with standard input taken from `/dev/null` failed at once, under a 120-second limit, `EXIT 0 SECONDS 0`:

```
ASKING NOW
type something: Traceback (most recent call last):
  File "<string>", line 1, in <module>
EOFError: EOF when reading a line
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

The same script with standard input held open by `sleep 75 |` and never sent anything, under a 60-second limit, printed `ASKING NOW` and `type something: ` and nothing more, and was ended by the limit: `EXIT 142`. It was still waiting at 60 seconds. (The whole command took 75 seconds because the shell waited for `sleep` to finish.)

**A still render with no window writes nothing unless asked (entry 15; bears on 7b).** With the output path set to an empty folder, `bpy.ops.render.render()` from a windowless script printed `OUTPUT PATH IS /private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e15a/s_` and `RENDER RETURNED`, exit status 0, and the folder stayed empty (`total 0`). `bpy.ops.render.render(write_still=True)` into a second empty folder printed `Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e15b/s_.png'`, exit status 0, and `s_.png` was there.

**A frame already on disk is written over without a question, unless Overwrite is off (entry 16).** `-b plain.blend -o <work folder>/o_ -f 1` was run twice, two seconds apart, each exiting with status 0. The frame's written time went from `Oct 17 17:22:36 2026` to `Oct 17 17:22:40 2026`, and a copy of the first frame compared with the second as pictures: `PIXELS_IDENTICAL max_channel_difference=0.000000`. A third run with `use_overwrite` and `use_placeholder` both set off by `--python-expr` before `-f 1` printed:

```
00:00.326  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/files/plain.blend"
00:00.331  render           | Rendering single frame
00:00.331  render           | Rendering frame 1
00:00.331  render           | Skipping existing frame "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e16/o_0001.png"
00:00.331  reports          | No frames rendered, skipped to not overwrite
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

exit status 0, and the written time stayed `Oct 17 17:22:40 2026`.

**A missing texture, with a control: named, and nothing waits (entry 17).** The file was first asked what it holds: `SLOTS ['M']`, `FACE MATERIAL INDEXES [0]`, `IMAGES [('Render Result', '', False), ('tex.png', '//tex.png', False), ('Viewer Node', '', False)]` — one material, the one every face uses, with an image texture at `//tex.png`. It was rendered with no window under a 120-second limit five times: with its 8-by-8 green-toned `tex.png` (EEVEE, the file's engine); with a plain red image copied over `tex.png` (EEVEE, and again with `-E CYCLES`); and with `tex.png` moved away (EEVEE, and again with `-E CYCLES`). All five exited with status 0 and wrote a frame. Compared as pictures:

```
built vs red: PIXELS_DIFFER max_channel_difference=0.517647
built vs gone: PIXELS_DIFFER max_channel_difference=0.576471
red vs gone: PIXELS_DIFFER max_channel_difference=0.694118
cred vs cgone: PIXELS_DIFFER max_channel_difference=0.678431
```

So the texture reached the picture, and its absence changed the picture, in both engines. With the image moved away, EEVEE printed, in full:

```
00:00.339  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e17/missingtex.blend"
00:00.343  render           | Rendering single frame
00:00.343  render           | Rendering frame 1
00:00.344  render           | Start rendering: Scene, ViewLayer
00:00.344  render           | Engine: EEVEE
00:00.470  image.gpu        | ERROR Image file "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e17/tex.png" does not exist
00:00.484  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.715  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.747  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.766  render           | Fra: 1 | Rendering 64 / 64 samples
00:00.836  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e17/gone_0001.png'
00:00.836  render           | Time: 00:00.49 (Saving: 00:00.06)

Blender quit
```

and Cycles printed, among its progress lines:

```
00:00.349  cycles           | WARNING Image file /private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e17/tex.png does not exist.
…
00:00.350  render           | Fra: 1 | Mem: 1M | Updating Images | Loading tex.png
00:00.350  cycles           | ERROR Failed to load 1 image files
```

then saved its frame. The three runs with an image present were searched for `tex`, `image`, `not loaded`, `fail`, `missing`, `error` and `warn`; the only matches were the `Read blend:` line, and in the Cycles run `Updating Images` and `Updating Images | Loading tex.png`.

**An output folder that cannot be written to: an error, exit status 1, nothing written, nobody waited for (entry 18).** The output was aimed at `/usr/smoozy-x_`; `/usr` is owned by `root` (`drwxr-xr-x@ 11 root  wheel`) and the test ran as an ordinary user. *(The list's first way — a folder of the test's own made unwritable with `chmod 555` — was refused by the tool running the tests before anything ran, so this way was used instead.)* The windowless render printed:

```
00:00.909  render           | Fra: 1 | Rendering 64 / 64 samples
00:00.963  image.write      | ERROR OpenImageIO write failed: Could not open file "/usr/smoozy-x_0001.png"
00:00.963  reports          | ERROR Render error (Undefined error: 0) cannot save: '/usr/smoozy-x_0001.png'
00:00.963  render           | Time: 00:00.61

Blender quit
```

exit status 1, under a 120-second limit, and `ls` afterwards answered `/usr/smoozy-x_0001.png: No such file or directory`. The word in brackets, `Undefined error: 0`, does not name the cause; the `OpenImageIO write failed` line above it names the file.

**A script hooked into the render holds the render for as long as it runs (entry 19).** With no window, the same still render was run from a script twice. With nothing hooked in: `Saved: '…/plain_.png'`, `RENDER RETURNED`, `EXIT 0 SECONDS 1`. With a function added to `bpy.app.handlers.render_pre` that prints, sleeps 45 seconds and prints again:

```
00:00.331  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/files/plain.blend"
HANDLER STARTED 17:24:49
HANDLER FINISHED 17:25:34
00:46.022  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e19/h_.png'
RENDER RETURNED 17:25:35
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

`EXIT 0 SECONDS 46`. The frame was saved only after the handler finished.

**A windowless render outlives the shell it was started from being killed — but not a hang-up signal sent to its process group (entry 20).** Both were done with an animation render of Cycles on the processor, 1024 samples with adaptive sampling off, 256 by 256, frames 1 to 400, about 2.6 seconds a frame.

*(A first attempt used a lighter file, 64 samples at 128 by 128 with adaptive sampling on; it rendered all 400 frames in 18.5 seconds, so by the time the shell was killed at 45 seconds both it and Blender were already gone — `kill 60403 failed: no such process` — and that attempt tells nothing. The heavier file was made from it and the test run again.)*

- *The shell killed.* Blender was started in the background from a `zsh -c` shell, which was killed outright (`kill -9`) 45 seconds in. Before, after 3 seconds, and after 33 seconds:

  ```
    PID  PPID STAT COMMAND
  61282 61280 SN   zsh -c BLENDER_USER_RESOURCES="$W/config/e20" "$B" -b "$W/files/long2.blend" -o "$W/runs/e20a2/f_" -a >
  61284 61282 RN   /Applications/Blender 5.3b.app/Contents/MacOS/Blender -b /private/tmp/claude-501/smoozy-hand-tests-5.3/
  16
  --- 3 s after killing the shell:
    PID  PPID STAT COMMAND
  61284     1 RN   /Applications/Blender 5.3b.app/Contents/MacOS/Blender -b /private/tmp/claude-501/smoozy-hand-tests-5.3/
  17
  --- 33 s after:
    PID  PPID STAT COMMAND
  61284     1 RN   /Applications/Blender 5.3b.app/Contents/MacOS/Blender -b /private/tmp/claude-501/smoozy-hand-tests-5.3/
  28
  ```

  (the numbers alone are the frames on disk). Blender went on, now under process 1, and the frame count rose from 16 to 28. It was then ended by the test.

- *A hang-up to the whole group.* Blender was started as the leader of a process group of its own (`perl -e 'setpgrp(0,0); exec @ARGV' "$B" -b long2.blend -o … -a`), and 30 seconds in a hang-up signal was sent to the group (`kill -HUP -<group>`):

  ```
    PID  PGID STAT COMMAND
  61478 61478 RN   /Applications/Blender 5.3b.app/Contents/MacOS/Blender -b /private/tmp/claude-501/smoozy-hand-tests-5.3/
  11
  --- 3 s after the hang-up:
    PID  PGID STAT COMMAND
  11
  --- 23 s after:
    PID  PGID STAT COMMAND
  11
  (eval):kill:2: kill 61478 failed: no such process
  ```

  Blender was gone 3 seconds after the hang-up and the count stayed at 11. Its output stopped part-way through frame 12, at `Fra: 12 | Remaining: 00:02.14 | Mem: 2M | Sample 1/1024`, with no line about the signal.

**Render Audio from a windowless script writes straight over a file already there (entry 21).** A 24-byte file stood at the target path (`24 … mix.flac`, checksum `f7228ea6b178df32077280927f544cf46831a5e7`). `-b plain.blend --python-expr "import bpy; print('MIXDOWN RETURNED', bpy.ops.sound.mixdown(filepath='<that path>'))"` printed `MIXDOWN RETURNED {'FINISHED'}`, `Blender quit`, `EXIT 0 SECONDS 0`. Afterwards the file was `9816` bytes with checksum `85a9acb1b3d34a135b7d1cd4fa21275e67bec313`. Nothing was asked.

**Freestyle changes the picture with automatic script execution off, and nothing asks (entry 22; bears on the Freestyle line in 7a).** The same factory scene saved twice, with `scene.render.use_freestyle` and the view layer's `use_freestyle` both on and both off, was rendered with no window as `-b --disable-autoexec <file> -o <path> -f 1`. Both exited with status 0 and wrote a frame. Searched for `freestyle` and `script`, the Freestyle-off log matched only its `Read blend:` line; the Freestyle-on log printed nothing about scripts and showed Freestyle as steps of their own:

```
00:00.923  render           | Rendering freestyle
00:00.923  render           | Fra: 1 | Freestyle: Mesh loading
00:00.925  render           | Fra: 1 | Freestyle: View map creation
00:00.926  render           | Fra: 1 | Freestyle: Visibility computations 0%
…
00:00.926  render           | Fra: 1 | Freestyle: Visibility computations 100%
00:00.926  render           | Fra: 1 | Freestyle: Stroke rendering
```

Compared as pictures: `PIXELS_DIFFER max_channel_difference=0.658824`.

**A windowless render does not appear in the Mac's list of running programs (entry 23).** An eight-frame windowless Cycles render (about 2.6 seconds a frame) was started in the background, and every 2 seconds, eight times, the Mac's own list of registered programs was asked for that process by its number (`lsappinfo find pid=<number>`), and for the front program. Every look, word for word apart from the number:

```
--- look 1, pid 62269
ASN=[]
FRONT: "LSDisplayName"="Problem Reporter"
```

The render then ended by itself (`BLENDER EXIT 0`, 8 frames on disk). So the process was never in the list. That the same question does find a registered program was checked straight afterwards on the Creator's own open Blender, read only: `ASN=[ASN:0x0-0x483c838-"Blender":]`, `"LSDisplayName"="Blender"`, `"pid"=12919`. This bears on the release note above that Blender's icon no longer shows in the Dock in background mode, without testing the Dock itself. The front program throughout was macOS's Problem Reporter, which is entry 9's (below).

**After the crashes of entry 9, macOS's Problem Reporter stayed in front.** The front program read `"LSDisplayName"="Problem Reporter"`, `"CFBundleIdentifier"="com.apple.ProblemReporter"`, process `43059`, started `Sat Oct 17 17:18:01 2026` — the same second as the three `Blender-…ips` crash reports — and it was still the front program at the end of entry 23, about fifteen minutes later. Its window was not looked at.

**With a window, the blocked-scripts warning is a dialog in the middle of the main window, and a render runs with it unanswered (entry 26; bears on 7a and 7d).** The driver file above was opened with a window, from a never-used settings folder whose Auto Run Python Scripts starts off, and a script inside it, three seconds after start, printed and photographed:

```
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Error in PyDriver: expression failed: __import__('math').floor(4.5)
For target: (type=Object, name="Cube", property=location, property_index=0)
AUTOEXEC_FAIL True
AUTOEXEC_FAIL_MESSAGE "Driver '__import__('math').floor(4.5)'"
CUBE_X 0.0
FRAMES BEFORE: []
SHOT 1-opened.png {'FINISHED'}
```

The picture shows, over the splash screen and its Quick Setup, a dialog in the middle of the main window with a red cross, reading word for word:

```
For security reasons, automatic execution of Python scripts in this file was disabled:
Driver '__import__('math').floor(4.5)'
This may lead to unexpected behavior. Allow at your own risk.
        Allow Execution            Continue Safely
```

with **Continue Safely** highlighted and no checkbox. This is not the dialog the manual's picture shows (7a: a "Permanently allow execution of scripts" checkbox and an "Ignore" button). Nobody answered it. The script then switched the scene to Cycles on the processor, 1024 samples, 200 by 200, and started a still render as F12 would, writing the frame (`bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)`). It printed:

```
RENDER CALL {'RUNNING_MODAL'}
00:03.938  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e26/frames/r_.png'
FRAMES TWO SECONDS AFTER STARTING: ['r_.png']
SHOT 2-while-rendering.png {'FINISHED'}
WINDOWS [('Layout', ['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']), ('temp', ['IMAGE_EDITOR'])]
FRAMES TWENTY-TWO SECONDS AFTER STARTING: ['r_.png']
AUTOEXEC_FAIL NOW True
```

and Blender quit when the script told it to, exit status 0. So the frame was written within two seconds with the dialog and the Quick Setup both unanswered, and a second window, its screen named `temp` and holding one `IMAGE_EDITOR`, had opened. The second and third pictures, taken two and twenty-two seconds after the render started, are identical byte for byte and both show the dialog still up and `Rendering... 0%` in the status bar — though the frame was already saved when the first of them was taken. Entry 24 showed this window does not redraw by itself after a script's change, so these two pictures are not read as the window's state at those moments.

**With a window, a still render nobody asks to be written writes nothing, and opens a window of its own (entry 27; bears on 7b).** The factory scene was opened with a window from a never-used settings folder, and a script inside it started a still render as F12 would, first with `write_still=False`, then with `write_still=True`, into an empty folder:

```
RENDER DISPLAY TYPE PREF: WINDOW
FRAMES BEFORE: []
WINDOWS BEFORE: [('Layout', ['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D'])]
RENDER CALL write_still=False: {'RUNNING_MODAL'}
FRAMES AFTER FIRST RENDER: []
WINDOWS AFTER FIRST RENDER: [('Layout', ['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']), ('temp', ['IMAGE_EDITOR'])]
…
RENDER CALL write_still=True: {'RUNNING_MODAL'}
00:09.528  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e27/frames/s_.png'
FRAMES AFTER SECOND RENDER: ['s_.png']
WINDOWS AFTER SECOND RENDER: [('Layout', ['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']), ('temp', ['IMAGE_EDITOR'])]
```

Blender then quit when the script told it to, exit status 0. So the first render wrote nothing and a second window, screen `temp`, one `IMAGE_EDITOR`, appeared; it was still there after the second render, which wrote `s_.png`. Blender's photograph of that second window is a plain grey picture 320 by 256 pixels, and of the main window shows the splash screen and Quick Setup still up, so neither picture shows a rendered image.

**With a window, a file chooser opened on top of a running animation render stops nothing (entry 28).** A Cycles animation on the processor (about 2.6 seconds a frame) was opened with a window from a never-used settings folder, and a script inside it started the animation render as Ctrl F12 would (`bpy.ops.render.render('INVOKE_DEFAULT', animation=True)`), then ten seconds later opened a Save As file chooser (`bpy.ops.wm.save_as_mainfile('INVOKE_DEFAULT')`) and never answered it. Printed, with the per-sample lines left out:

```
ANIMATION RENDER CALL: {'RUNNING_MODAL'}
00:07.729  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e28/frames/f_0001.png'
00:11.780  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e28/frames/f_0002.png'
FRAMES BEFORE THE CHOOSER OPENED: 2
CHOOSER CALL: {'RUNNING_MODAL'}
FRAMES JUST AFTER THE CHOOSER OPENED: 2
00:15.564  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e28/frames/f_0003.png'
FRAMES TWO SECONDS LATER: 3
WINDOWS: [('Layout', ['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']), ('temp', ['IMAGE_EDITOR']), ('temp.001', ['FILE_BROWSER'])]
…
00:19.747  render           | Saved: '…/f_0004.png'
…
00:43.062  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e28/frames/f_0010.png'
FRAMES THIRTY SECONDS AFTER THAT: 10
WINDOWS: [('Layout', ['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']), ('temp', ['IMAGE_EDITOR']), ('temp.001', ['FILE_BROWSER'])]
```

So with the chooser standing unanswered in a window of its own, frames went on being written, about one every four seconds before and after it opened, from 2 to 10. Blender's photograph of that third window shows a file browser at the test's files folder with the name `long2.blend` and the buttons **Cancel** and **Overwrite**. Blender quit when the script told it to, with the render still going, exit status 0.

**A render asked for on the command line with a window open, from a never-used settings folder, is written within two seconds — and then that Blender does not quit (entry 29; bears on 7d).** No picture was taken in this entry; every other fresh start with a window in this sitting showed the splash screen and its Quick Setup. With a settings folder holding nothing (`ls -A … | wc -l` gave `0`) and no `-b`, `"$B" plain.blend -o <path> -f 1` was started at `17:41:24` under a 120-second limit. It printed:

```
00:00.631  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/files/plain.blend"
00:00.640  render           | Rendering single frame
00:00.640  render           | Rendering frame 1
00:00.640  render           | Start rendering: Scene, ViewLayer
00:00.640  render           | Engine: EEVEE
00:01.226  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e29/w_0001.png'
00:01.226  render           | Time: 00:00.58 (Saving: 00:00.06)
```

and nothing more. The frame's written time was `Oct 17 17:41:26 2026`. The run ended `EXIT 142 SECONDS 120`: still open when the limit ended it. Nothing was clicked or answered.

**With a window, Render Audio with a path named writes over the file without asking; with no path named it opens a chooser and writes nothing (entry 30).** A copy of the factory scene, `mixwin.blend`, was opened with a window from a never-used settings folder, and a script inside it wrote a 24-byte `named.flac` beside it and called `bpy.ops.sound.mixdown` as a menu would (`'INVOKE_DEFAULT'`), first with that path named, then with no path. Printed, word for word:

```
BLEND FILE: /private/tmp/claude-501/smoozy-hand-tests-5.3/The machine/runs/e30/mixwin.blend
FOLDER BEFORE: ['mixwin.blend', 'named.flac']
NAMED BEFORE: 24 bytes sha1 f7228ea6b178df32077280927f544cf46831a5e7
MIXDOWN WITH A PATH RETURNED: {'FINISHED'}
NAMED AFTER: 24 bytes sha1 f7228ea6b178df32077280927f544cf46831a5e7
WINDOWS AFTER THE FIRST CALL: [('Layout', ['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D'])]
FOLDER BEFORE THE SECOND CALL: [('mixwin.blend', '95754 bytes sha1 429bdaab07ae506e382c7d08e2a28b375de6eaf3'), ('named.flac', '9816 bytes sha1 85a9acb1b3d34a135b7d1cd4fa21275e67bec313')]
MIXDOWN WITH NO PATH RETURNED: {'RUNNING_MODAL'}
WINDOWS AFTER THE SECOND CALL: [('Layout', ['PROPERTIES', 'OUTLINER', 'DOPESHEET_EDITOR', 'VIEW_3D']), ('temp', ['FILE_BROWSER'])]
SHOT after-no-path-window0.png {'FINISHED'}
SHOT after-no-path-window1.png {'FINISHED'}
FOLDER AFTER THE SECOND CALL: [('after-no-path-window0.png', …), ('after-no-path-window1.png', …), ('mixwin.blend', '95754 bytes sha1 429bdaab07ae506e382c7d08e2a28b375de6eaf3'), ('named.flac', '9816 bytes sha1 85a9acb1b3d34a135b7d1cd4fa21275e67bec313')]
```

and Blender quit when the script told it to, exit status 0. So the call with a path returned `FINISHED`, opened no window, and the 24-byte file — still 24 bytes the instant the call returned — was 9816 bytes two seconds later, with the same checksum as the windowless mixdown (entry 21). The call with no path returned `RUNNING_MODAL`, a window of its own holding a file browser opened, and nothing new was written in the folder apart from the two pictures. Blender's photograph of that file-browser window came out plain grey, so what it offered is not recorded.

---

## 8. On a computer that has a graphics card but whose screen is drawn by the processor rather than by that card, what does a render do under each render engine?

*The question is ambiguous.* "Drawn by the processor" can mean two things:
- **(a)** graphics built into the processor chip (the manual calls this an "onboard GPU integrated into the main CPU") draws the screen, while a separate card sits unused for display
- **(b)** no graphics hardware draws the screen at all, and a software stand-in running on the processor does it

I answer both.

**Answer in short.**
- **Cycles:** the sources treat the card that renders and the card that draws the screen as separate. The manual even recommends one card for each. Cycles picks its cards through its own backends. *Inferred:* Cycles can render on the card either way, provided the card meets question 2. No source states this case outright.
- **EEVEE:** it renders only on a graphics device and never on the processor. The sources do not say which device it uses when the screen is drawn by something else. They give only a setting that picks the device for "display drawing" and "UI rendering". It does not run on headless Windows systems.
- **Workbench:** the sources say nothing.

The precise answer for EEVEE and Workbench is **not in the sources**.

**8a. What the sources say about display and rendering on different devices. Documented.**
> "The only complete solution for this is to use a dedicated GPU for rendering, and another for display."

> "the only real solution is to use separate graphics cards for display and rendering."

Manual, https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11. Both lines are about Cycles.

> "Laptops often have two GPUs for power saving purposes. One slower onboard GPU integrated into the main CPU (typically Intel or AMD) that uses lower power, and one faster dedicated GPU for better performance (AMD or NVIDIA) that uses more power. For the best performance, the dedicated GPU should be used for Blender. Which GPU to use for which application can be configured in your graphics driver or operating system settings."

Manual, for example https://docs.blender.org/manual/en/5.3/troubleshooting/gpu/windows/nvidia.html, read 2026-10-11.

> "Blender uses OpenGL for the 3D Viewport and user interface."

(Same pages.)

> "Running Blender inside a virtual machine is known to have problems when OpenGL drawing calls are forwarded to the host operating system. To resolve this, configure the system to use PCI passthrough."

(Same pages.)

**8b. Cycles. Documented, observed, and inferred.**

Documented: Cycles' devices come from its compute backends (questions 2 and 5), and the manual advises "a dedicated GPU for rendering, and another for display" (8a).

Observed: Cycles' render engine does not ask for a drawing context. `__init__.py`, lines 44–51, read 2026-10-16:
```
class CyclesRender(bpy.types.RenderEngine):
    bl_idname = 'CYCLES'
    bl_label = "Cycles"
    bl_use_eevee_viewport = True
    bl_use_preview = True
    bl_use_exclude_layers = True
    bl_use_spherical_stereo = True
    bl_use_custom_freestyle = True
```
It does not set `bl_use_gpu_context`, which the API describes as "Enable OpenGL context for the render method, for engines that render using OpenGL (default False)" (https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html, read 2026-10-11).

*Inferred:* in both readings, (a) and (b), a Cycles render set to GPU Compute uses the card through its backend, whatever draws the screen. The card must still meet question 2, and its driver must be installed. The step: Cycles' final render does not take Blender's drawing context, its cards are chosen separately, and the manual recommends separate cards for display and rendering. No source states this case directly.

The Cycles files also reveal one more thing: when Cycles registers, it is told whether Blender is in background mode, through `_cycles.init(path, user_path, bpy.app.background)` (`engine.py`, line 52). What it does with that is not in the supplied files.

**8c. EEVEE. Documented, then not in the sources.**
> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient."

> "There is currently no support for multiple GPU systems."

> "Headless rendering is not supported on headless Windows systems."

> "In EEVEE, GPU Memory management is done by the GPU driver."

Manual, https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html, read 2026-10-11.

The drawing device can be chosen:
> "Device Vulkan … Specifies which GPU device to use for display drawing operations. This setting is useful for systems with multiple GPUs (e.g., integrated + discrete) where you want to force Blender to use a specific GPU for UI rendering. … Auto: Automatically selects the most appropriate GPU based on system configuration and driver support."

(https://docs.blender.org/manual/en/5.3/editors/preferences/system.html)

The API lists "SOFTWARE" among the device types Blender's drawing layer can report (`gpu.platform.device_type_get`, https://docs.blender.org/api/5.3/gpu.platform.html).

**Not in the sources:**
- For reading (a): whether EEVEE renders on the processor's built-in graphics (the one drawing the screen) or on the separate card. Whether "Device Vulkan" or `gpu_preferred_device` also decides EEVEE's final render. What "Auto" chooses here.
- For reading (b): whether EEVEE runs at all on a "SOFTWARE" device. The manual's "no plan to support CPU (software) rendering" is about EEVEE having no processor mode of its own. It does not say what happens when Blender's drawing layer itself runs in software.
- For both: anything on Linux or macOS headless EEVEE.

**8d. Workbench. Not in the sources.** The manual describes Workbench as "a render engine optimized for fast rendering during modeling and animation preview" whose "primary task is to display a scene in the 3D Viewport" (https://docs.blender.org/manual/en/5.3/render/workbench/index.html). It says nothing about which device a Workbench render uses.

**Label summary.** 8a documented. 8b documented, observed and inferred, with the step written out. 8c documented as far as quoted, then not in the sources. 8d not in the sources.

**Not covered.** EEVEE and Workbench in both readings, as listed. Whether the compositor's "GPU" setting uses the card or the drawing device.

**Where I looked.** I searched the manual, API and release notes for "software", "software render", "llvmpipe", "software opengl", "swiftshader", "lavapipe", "Microsoft Basic", "xvfb", "virtual display", "offscreen", "remote desktop", "EGL", "headless", "integrated", "discrete", "dedicated", "onboard", "iGPU", "for display", "display gpu" and "monitor". I read the EEVEE pages (index, limitations, performance), the Workbench pages, the System and Viewport preferences, the GPU-module API pages (`gpu`, `gpu.platform`, `gpu.types`, `gpu.capabilities`), the RenderEngine API page, the command-line pages, and all graphics troubleshooting pages. "Offscreen" hits are about a Python drawing feature, not rendering devices.

**Outside the interface.**
- Command-line rendering needs no graphical display (1g).
- EEVEE cannot render on headless Windows systems.
- `--gpu-backend` forces the drawing system.
- A background script must call `gpu.init()` before using the `gpu` module, and that call raises a `SystemError` if it fails (question 6).
- For Cycles, `--cycles-device` picks the backend without regard to the screen (question 5).
- *(Hand tests on the Creator's Mac, 2026-10-17 — observed: with no window, all three engines rendered, Cycles on the card that also draws that Mac's screen, reported as `(display)`; after `gpu.init()` the drawing module answered for the one Metal device. Neither reading of this question was reachable there. See below.)*

**Operating system.** EEVEE's headless limit is stated for Windows only. The laptop text appears on the Windows pages and on most Linux pages, but on no macOS page. "Device Vulkan" applies where Vulkan is used, which by default is Windows (not ARM) and Linux. Nothing else here is tied to a system. *(Hand tests, 2026-10-17: neither reading could be tried — the only machine available, the Creator's Mac, has one graphics device that both draws and renders, and no Windows or Linux machine was available. See below.)*

### Hand tests on the Creator's Mac — observed, 2026-10-17

**Neither reading of this question can be reached on the Creator's Mac**, and nothing below answers it. That Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — has one graphics device, which both draws its screen and renders; there is no separate card, and no software drawing could be chosen. No machine with a separate card, or with a screen drawn by the processor's built-in graphics or by software, was available. Run there against Blender 5.3.0 Beta (hash `5d8151335b6b`) on 2026-10-17, two things bear on the question's terms without answering it:

- **Cycles marks the card that draws the screen.** A windowless Cycles render on that card reported `Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]` (entry 2; the full report is under question 2). What `(display)` is set from, and whether a card that does not draw a screen loses it, could not be seen with one device.
- **EEVEE and Workbench rendered with no window and printed nothing naming a device** (entry 2, question 1).
- **The desktop half of the researcher's test 4 was run**: with no window, after `gpu.init()`, the drawing module answered `METAL APPLE Apple M1 Pro Metal API 1.2 [(0, 'Metal API', 'METAL')]` (entry 7, question 6). Its other half — the same line on a machine whose screen is drawn by the processor's built-in graphics or by software — needs a machine that was not available.

---

## Things a person at a computer could settle in a few minutes

**Pruned 2026-10-17 after the hand tests on the Creator's Mac.** Tests 2, 3, 5 and 8 were run there and are answered in the questions above (questions 4, 2, 6 and 7), and so are the macOS half of test 9 and the normal-desktop half of test 4. What is left needs a machine that sitting did not have — its only machine was a MacBook Pro with an Apple M1 Pro chip and one graphics device, on macOS — so every item below is still open. Each says what it needs. The numbers are this list's own, kept from before the pruning.

Each test says what to open, what to do and what to look at. "file.blend" means any small scene saved with a camera. Where an engine name is needed, first run `blender -E help` and use the exact name it prints.

1. **Which cards does `--cycles-device` use?** (Questions 5, 6.) This needs a machine with two NVIDIA cards. *(Not run: no NVIDIA card and no second card was available. The one-card form is answered under question 5: on the Mac, with the card saved unticked, the flag rendered on the card, and `--factory-startup` set the saved ticks aside.)*
   - In Preferences ‣ System ‣ Cycles Render Devices, choose CUDA and untick one of the two cards. Save the preferences and quit.
   - In a terminal, run `blender -b file.blend -E CYCLES -f 1 -- --cycles-device CUDA`.
   - While it renders, run `nvidia-smi` in a second terminal and note which cards show Blender using them.
   - Repeat with `--factory-startup` added before `-f`.
   - This settles whether the override uses all cards of the type or only the ticked ones, and whether `--factory-startup` drops the saved ticks.

4. **What the drawing layer sees, from a script — the second half.** (Questions 6, 8.) Run:
   `blender -b --python-expr "import gpu; gpu.init(); from gpu import platform as g; print(g.backend_type_get(), g.device_type_get(), g.vendor_get(), g.renderer_get(), g.version_get(), [(d.index, d.name, d.identifier) for d in g.devices_get()])"`
   - Copy the output, or the error if `gpu.init()` fails.
   - Do this on a machine where the screen is drawn by the processor's built-in graphics or by software. *(Not run: no such machine was available. The normal-desktop half is answered under question 6.)*

6. **EEVEE when the screen is drawn by the processor.** (Question 8.) This needs a machine with a separate card, where the screen is drawn by the processor's built-in graphics. *(Not run: no such machine was available.)*
   - Run `blender -b file.blend -E <EEVEE name from -E help> -f 1 -o //eevee_test_####`.
   - While it runs, watch the separate card's activity with the vendor's tool (for NVIDIA, `nvidia-smi`).
   - Note whether the frame was written and which device was busy.
   - Repeat after setting Preferences ‣ System ‣ Display Graphics ‣ Device to the separate card (Vulkan only), restarting Blender, and saving the preferences.
   - Repeat with `--gpu-backend opengl`.

7. **EEVEE on a headless Windows machine.** (Questions 7, 8.) *(Not run: no Windows machine was available. When one is rented, its edition must be named.)*
   - On a Windows machine with no monitor connected and nobody logged in at the screen, connected over SSH or a remote shell, run the command from test 6.
   - Copy all output and note whether a frame was written.
   - Repeat with `-E CYCLES` and with the Workbench name from `-E help`.

9. **Which drawing system does a fresh 5.3 start on — the other systems.** (Question 1.) On Windows (x64), Windows (arm64) and Linux: *(Not run: no such machine was available. The macOS half is answered under question 1: Metal, with no Display Graphics section on the System page.)*
   - run `blender --factory-startup --python-expr "from gpu import platform as g; print(g.backend_type_get(), g.device_type_get(), g.renderer_get())"`. This is without `-b`, so the window opens; close it after the line prints.
   - also look at Preferences ‣ System ‣ Display Graphics ‣ Backend
   - note the backend reported in both places.

**Two more, from the earlier versions' records, not run in that sitting either.** Whether the screen locking or the display sleeping interrupts a render needs the Mac to itself, put to sleep on purpose, and other work was going on there. Two Blenders rendering into one output folder at once was not run because that Mac's rules allow one Blender at a time, for want of memory.

---

2026-10-16 — Claude Opus 5.5 (Claude Code) — The machine, Blender 5.3

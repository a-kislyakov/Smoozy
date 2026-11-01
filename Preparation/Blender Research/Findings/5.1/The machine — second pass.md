# The machine — second pass — Blender 5.1

**Sources consulted:** Blender 5.1 manual, Blender 5.1 Python API documentation, Blender 5.1 release notes, and Cycles' own files for 5.1 — all read on 2026-07-30. Blender's requirements page, read on 2026-08-01.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

---

## 1. What must a computer have on it before Blender will run at all?

### Short answer

The 5.1 manual states, in words, what must be in place: a supported operating system (Windows, macOS or Linux), a graphics card and driver that support OpenGL, up-to-date drivers, on Linux a windowing system (X11 or Wayland), permission to run the program, and nothing blocking it (such as antivirus software). The manual gives **one** number: OpenGL 3.3 as the minimum.

The manual does **not** carry the table of minimum operating-system versions, processor, memory or graphics-memory figures. It sends the reader off to a download page for those. That table exists on Blender's requirements page, but that page names no version for its tables, so it cannot be read as speaking for 5.1 — with one exception, the macOS note, which does name "Blender 5.0 and later".

So the answer is part **documented**, part **not in the sources**. I have split it by claim below.

### Label

**documented** for the claims in (a) to (f) below.
**not in the sources** for the numeric minimums (operating-system version, processor, memory, graphics memory) as they apply to 5.1 specifically — see (g).

### Not covered

- No numeric minimum for operating-system version, processor generation, core count, system memory or graphics memory can be attached to 5.1 from these sources, except the macOS one in (f).
- Nothing in these sources says how much disk space Blender needs.
- Nothing in these sources says what is required to run Blender with no display attached, or in background mode. The OpenGL requirement is stated for "the 3D Viewport and user interface"; whether it still applies with the interface switched off is not stated anywhere I searched.
- Nothing states what happens between "minimum" and "not supported" — whether Blender refuses to start, starts and misbehaves, or starts with reduced function.
- The manual and the requirements page give different numbers for the minimum OpenGL version (3.3 and 4.3). I report both as they stand; the sources do not reconcile them.
- The manual and the requirements page also differ on macOS processor architecture (see (e) and (f)). Again, both are reported as they stand.

---

### (a) A supported operating system, and a build matching the processor architecture

> "Blender is available for download on Windows, macOS, and Linux."

Manual, Installing Blender — https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — read 2026-07-30. Label: **documented**.

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

Manual, Installing on Windows — https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — read 2026-07-30. Label: **documented**.

### (b) A graphics card and driver with OpenGL, at least version 3.3

> "Blender uses OpenGL for the 3D Viewport and user interface. The graphics processing unit (GPU) and driver have a big impact on Blender's behavior and performance."

> "This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

> "Installing the latest version of the correct driver can help upgrade the OpenGL version, though some graphics cards are simply too old to run the latest Blender."

Manual, Troubleshooting Graphics Hardware — https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/index.html — and the per-platform pages that repeat the same wording, for example https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/windows/nvidia.html and https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/linux/nvidia.html — read 2026-07-30. Label: **documented**.

The same sentence, word for word, appears on all twelve per-platform pages (Windows, Linux, macOS, each with NVIDIA / AMD / Intel / Other). No platform is given a different number.

### (c) Up-to-date graphics drivers

> "Always check that your graphics drivers are up to date and that OpenGL is properly supported."

Manual, Installing Blender — https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — read 2026-07-30. Label: **documented**.

> "Upgrading to the latest graphics drivers often solves problems. Newer drivers have bug fixes that help Blender function correctly."

Manual, Troubleshooting Graphics Hardware — https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/index.html — read 2026-07-30. Label: **documented**.

### (d) The four things the manual tells you to check when Blender will not start

> "If Blender fails to start, there are a few common causes to check for:
>
> See if your computer meets the minimum requirements.
>
> Confirm that your graphics card is supported and that the drivers are up to date (see Troubleshooting Graphics Hardware).
>
> Make sure any antivirus software is not preventing Blender from starting.
>
> Ensure that you have appropriate user permissions on the computer."

Manual, Startup — https://docs.blender.org/manual/en/5.1/troubleshooting/startup.html — read 2026-07-30. Label: **documented**.

This is the closest thing in the 5.1 manual to a list of what a machine must have. Note that two of the four items are not about hardware at all: no antivirus software blocking Blender, and appropriate user permissions.

### (e) On Linux, a windowing system — and under Gnome-Shell, one extra library

> "On Linux Blender supports both X11 and Wayland for official releases."

> "When Wayland is detected, it is the preferred system, otherwise X11 will be used."

> "Under Gnome-Shell the libdecor library is required. This is available as a package on most Linux distribution."

> "If the library isn't found X11 will be used as a fallback."

Manual, Linux Windowing Environment — https://docs.blender.org/manual/en/5.1/getting_started/installing/linux_windowing_environment.html — read 2026-07-30. Label: **documented**.

A 5.1 release note bears directly on that library requirement:

> "Support opening windows without decorations with the: `--no-window-frame` argument (Linux only) On Wayland this removes the dependence on LIBDECOR."

Release notes, Core — https://developer.blender.org/docs/release_notes/5.1/core/ — read 2026-07-30. Label: **documented**.

### (f) macOS: the two sources say different things

The 5.1 manual says:

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture."

Manual, Installing on macOS — https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html — read 2026-07-30. Label: **documented**.

The requirements page says, and this is the **only** place on that page that names a version covering 5.1:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."

Requirements page — https://www.blender.org/download/requirements/ — read 2026-08-01. Label: **documented**. This names "Blender 5.0 and later" in so many words, which covers 5.1.

I am reporting both as they stand. I have not decided between them, and the sources give no way to.

### (g) The numbers the requirements page carries, and why they cannot be attached to 5.1

The requirements page carries minimum and recommended figures for Windows, macOS and Linux. Its Windows and Linux tables read, in part:

> "OS  Windows 8.1 (64-bit) … CPU  4 cores with SSE4.2 support … RAM  8 GB … GPU  2 GB VRAM with OpenGL 4.3, Vulkan 1.3 (see below)"

> "OS  Distribution with glibc 2.28 or newer (64-bit)"

> "OpenGL: Version 4.3 or newer, with mandatory support for GL_ARB_shader_draw_parameters and GL_ARB_clip_control extensions."

> "NVIDIA: GeForce 900 and newer, Quadro Tesla GPU architecture and newer, including RTX-based cards, with NVIDIA drivers."

> "AMD: GCN 4th gen and newer."

> "Intel: Kaby Lake architecture and newer. Always make sure to install the latest drivers from the graphics card manufacturer website."

Requirements page — https://www.blender.org/download/requirements/ — read 2026-08-01.

**These name no Blender version.** They describe whichever release was current on the day the page was taken. The page's own "Requirements for Older Versions" section names 4.5 LTS, 3.6 LTS, 3.4, 2.92, 2.90, 2.83 LTS, 2.79 and 2.76 — it does not name 5.1 or 5.0 anywhere in that section. So none of the figures above may be read as an answer for 5.1. Label for these as they apply to 5.1: **not in the sources**.

This is exactly the case the sheet's framing warns about: the page would have answered the question, but names no version. I searched the 5.1 manual, the 5.1 API documentation and the 5.1 release notes for every one of these terms and found none of them — no "glibc", no "SSE4", no "SSE2", no "Vulkan 1.3", no "OpenGL 4.3" as a requirement (the only hit for "OpenGL 4.3" in the manual is a debugging switch), no "Windows 8.1", no "Windows 11", no "Ventura", no "8 GB", no memory or core-count figure of any kind.

The manual is explicit that it does not carry them itself:

> "Blender has a set of minimum and recommended requirements; so make sure these are met before trying to install Blender."

Manual, Installing Blender — https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — read 2026-07-30. The manual points at a download page for the figures rather than reproducing them. Label: **documented** that the requirements exist and must be met; **not in the sources** what they are for 5.1.

### (h) What a computer does *not* need — stated outright

> "Whether it's on a USB stick, sitting on a folder on your desktop, or fully installed, Blender runs out of the box."

> "No installation needed, no Internet connection required."

Requirements page — https://www.blender.org/download/requirements/ — read 2026-08-01. This names no version, so I flag it the same way as (g): it speaks for whichever release was current on 2026-08-01, not demonstrably for 5.1.

The 5.1 manual says the same thing about the internet, and does speak for 5.1:

> "Blender is designed to not require an internet connection, so it doesn't have a built-in update system."

Manual, Installing Blender — https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — read 2026-07-30. Label: **documented**.

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

> "Blender bundles a complete Python installation and does not interact with the system Python by default. This can be changed with the `--python-use-system-env` command line argument, if care is taken to set up a compatible Python version."

Manual, Deploying Blender in Production — https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30. Label: **documented**. A separate Python installation is therefore not something the computer must already have.

The 5.1 release notes state which Python version is bundled:

> "Libraries have been upgraded to match VFX platform 2026, including: Python 3.13"

Release notes, Blender 5.1 Release Notes — https://developer.blender.org/docs/release_notes/5.1/ — read 2026-07-30. Label: **documented**.

### (i) Ruled out: Cycles' own files

I searched Cycles' 5.1 files for anything bearing on running Blender at all. The only "requires" lines there are about graphics cards for rendering — "Requires NVIDIA GPU with compute capability %s", "Requires AMD GPU with RDNA architecture", "Requires Intel GPU with Xe-HPG architecture", "Requires Apple Silicon with macOS %s or newer" (`properties.py`, lines 1814–1869). Those are about GPU rendering, which is a different question on the sheet, so I have not used them here. `__init__.py` line 7 onward declares `"blender": (2, 80, 0)` in `bl_info`, which is the add-on's own declared Blender version, not anything about the computer. Nothing in Cycles' files answers question 1.

### (j) Hand test — observed

On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 5.1 started and ran normally, and **Help ‣ Save System Info** was used to write `system-info.txt`. The file's section headed `GPU:` read:

```
renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:
```

and was followed by sections headed `Implementation Dependent GPU Limits:` and `GLSL:` before the section headed `Cycles:`. (An earlier version of this note said the text went straight from `extensions:` to `Cycles:`. That was written from a partial copy of the file; the whole file has since been read and is recorded in (k).)

Two limits on what this settles. Only the `GPU` and `Cycles` sections of the file were copied back, so nothing is recorded about the rest of it — in particular whether it carries any section naming an OpenGL version, which is what hand-test item 2 below was aimed at. And a machine that starts successfully says nothing about the floor: this does not resolve the 3.3-against-4.3 disagreement in (b) and (g), nor the macOS architecture disagreement in (f), since this machine is Apple Silicon.

Label: **observed.**

### (k) Hand tests from a terminal — observed

Run on **2026-08-01** on the same machine, against `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, which reports `Blender 5.1.2`.

**Blender starts and renders with no window.** `-b` starts were used all day without incident, and a background render of `/tmp/machine-test.blend` completed with both Cycles and EEVEE. This machine meets whatever graphics floor applies, so it tests neither the 3.3-against-4.3 disagreement in (b) and (g) nor the macOS architecture disagreement in (f) — it is Apple Silicon.

**One thing background mode does change.** The `gpu` module's drawing side is not available there at all in this version:

```
SystemError: GPU functions for drawing are not available in background mode
```

and `gpu.init` does not exist in 5.1 (`AttributeError: module 'gpu' has no attribute 'init'`). So whatever the OpenGL requirement is for, it is not being met by the same code path when there is no window.

**What the system-information file holds when written with no window.** `bpy.ops.wm.sysinfo(filepath='/tmp/sysinfo-terminal-51.txt')` returned `{'FINISHED'}`. The file's section headings were, in order: `Blender:`, `Python:`, `paths:`, `Python (External Binary):`, `Directories:`, `scripts:`, `pref scripts:`, `FFmpeg:`, `Other Libraries:`, then `GPU:`, then `Cycles:` (with `Metal device capabilities:` inside it), then `Enabled add-ons:`. **There is no section headed `OpenGL`** in that file, and the only occurrence of the word anywhere in it is inside a build linkflags line. The `GPU:` section, written with no window, was the single line `GPU: missing, background mode`. The file written from the Help menu has since been read end to end as well. It carries the same headings **plus** `extensions:`, `Implementation Dependent GPU Limits:` and `GLSL:`, and its `GPU:` heading introduces a real section (renderer, vendor, version, device type, backend type) rather than the one-line `GPU: missing, background mode`. Its `Cycles:` section is identical to the background-written one. **Neither file carries a section headed `OpenGL`.** So the only Blender-written readout of the graphics stack on this machine names Metal at version 1.2, and names no OpenGL version at all — which means it cannot be set against either the manual's 3.3 or the requirements page's 4.3.

### (l) Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — against `/opt/blender/blender-5.1.2-linux-x64/blender`, which reports `Blender 5.1.2`. The machine is a container sharing somebody else's hardware, not a whole computer, and other work was running on it at the same time, so nothing here says how long anything took or how much memory was free.

One thing about the settings folder, said plainly rather than left out. The first runs were meant to point Blender at a folder of this topic's own with `BLENDER_USER_RESOURCES`, and a mistake in the shell meant the variable was never actually set, so those runs used Blender's ordinary default folder instead. That folder was also empty when this work began, so every version still started from factory settings. Every test that turns on saved preferences was then run again with the variable properly set, and every result came back identical. Both runs are described in `Audits/Blender Research/The machine/Hand tests — Linux.md`. Nothing in this answer turns on saved preferences at all.

**Blender starts and renders on Linux with no windowing system running at all.** This is the question (e) leaves open. `DISPLAY` was removed from the environment and a background Cycles render of the test file completed at exit `0`, writing its PNG, with the graphics card doing the work. An EEVEE render did the same. So neither X11 nor Wayland is needed to run and render; the requirement in (e) is about opening a window.

**Asked for a window with no windowing system, Blender refuses, and this is exactly what it says.** Running `env -u DISPLAY <blender> --python-expr "import bpy; print('WINDOW OPENED')"` never printed `WINDOW OPENED`. It printed, and then exited with status `1`:

```
00:00.029  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.029  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4645, total unfreed memory 1.212318 MB
```

So Blender names both windowing systems it tried and what went wrong with each — which is what "what does Blender report about its own windowing system" was asking. Note that 4.5 on the same machine printed the same information in a plainer shape and then **hung**, having to be killed.

**The libdecor half of (e) is still out of reach.** The rented machine runs Xorg, not Wayland, and nothing on it may be installed or removed, so Gnome-Shell without libdecor cannot be built. What is settled is that the argument that bears on it exists: Blender's own `--help` on this machine lists `--no-window-frame` / "Disable all window decorations (Linux only)" in 5.0, 5.1 and 5.2, and not in 4.5.

**The graphics floor in (b) and (g) is still untested.** With no window, `gpu.platform` refuses on this machine exactly as on the Mac, and `gpu.init` does not exist. Reached another way — inside a window, this machine reports OpenGL version `4.6.0 NVIDIA 580.105.08`, above both the manual's 3.3 and the requirements page's 4.3. So it settles neither.

**What the system-information file holds on Linux.** Written with no window, the `GPU:` section is again the single line `GPU: missing, background mode`, and there is again **no section headed `OpenGL`**. The `Cycles:` section is where Linux differs: instead of the Mac's three-line Metal block it carries about ninety lines of NVIDIA card capabilities, opening `CPU device capabilities: AVX2` and including `CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR 8` and `CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR 9`. So on neither machine does Blender's own readout name an OpenGL version, and the 3.3-against-4.3 disagreement cannot be settled from it.

**One line Linux prints that the Mac never did**, but only when Cycles is asked for HIP:

```
00:00.344  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
```

Cycles reporting that AMD's library is not on the machine. Harmless here.

*(Corrected 2026-08-06 by the runner, after the check of the Linux hand tests could not reproduce this. The version split is right; "nearly every run" was not. Run again on the same machine with a plain Cycles render at factory settings, this line appeared **not at all**, in any of the four versions. It appears only when Cycles is actually asked for HIP — either the preferences are set to HIP, or the render is started with `--cycles-device HIP`. Then 5.0, 5.1 and 5.2 each print it exactly once and 4.5 prints nothing of the kind, failing instead with `Error: Found no Cycles device of the specified type`. So anything reading Blender's output has to expect this line when it asks for HIP, and not otherwise.)*

---

## Outside the interface

The sources do not say, and I will not guess.

Blender 5.1 can be driven from outside its own window — the command line page is headed "Blender 5.1" and lists:

> "-b, --background
>
> Run in background (often used for UI-less rendering)."

Manual, Command Line Arguments — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. Label: **documented**.

But nothing I found states whether the OpenGL requirement in (b), or the Linux windowing requirement in (e), still applies when Blender runs that way. The OpenGL sentence is scoped to "the 3D Viewport and user interface", and it would be easy to conclude that background mode escapes it — but that conclusion is a step the source does not take, so it would be **inferred**, and I am not making it. The startup checklist in (d) draws no distinction between the two ways of starting Blender either.

**One half of that is now observed rather than reasoned.** The **Linux windowing requirement in (e) does not apply in background mode**: on the rented Linux machine, with `DISPLAY` removed and neither X11 nor Wayland reachable, 5.1 started, rendered with Cycles on the graphics card and rendered with EEVEE, all at exit `0`. Asked to open a window in the same state, it refused and exited `1`, naming both windowing systems. See (l). The **OpenGL requirement in (b) is still not tested either way**, both machines being far above any floor the sources name.

One related thing the sources do say, which bears on running Blender on a machine that is not a desk:

> "Running Blender inside a virtual machine is known to have problems when OpenGL drawing calls are forwarded to the host operating system."

> "To resolve this, configure the system to use PCI passthrough."

> "Some VM hosts may require turning on GPU paravirtualization. Some GPU vendors restrict this function to higher-priced cards or models."

Manual, Troubleshooting Graphics Hardware, on all twelve per-platform pages, for example https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/windows/nvidia.html — read 2026-07-30. Label: **documented**.

## Operating system

It does not hold identically everywhere. The differences the sources state:

- **All three** — Windows, macOS and Linux are the platforms Blender is available for; the OpenGL 3.3 minimum, the driver advice and the startup checklist are stated in the same words for all three.
- **Windows** — the build must match the processor architecture, x64 or arm64. Installing from the Windows Installer file "requires administrator rights"; installing from the zip does not. (Manual, Installing on Windows, https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html, read 2026-07-30.)
- **macOS** — drivers are not a separate thing to install: "On macOS, graphics drivers are built into the operating system and the only way to get newer drivers is to upgrade macOS as a whole to the latest version." (Manual, Troubleshooting macOS, e.g. https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/apple/amd.html, read 2026-07-30.) Also: "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time." (Manual, Installing on macOS, read 2026-07-30.) And the processor-architecture disagreement in (f).
- **Linux** — X11 or Wayland is needed; under Gnome-Shell the libdecor library is required, or Blender falls back to X11. Both halves are now partly tested on a real Linux machine, and the results are in (l): with neither X11 nor Wayland reachable, Blender refuses to open a window and says so by name, while still running and rendering perfectly well with `-b`. The libdecor half needs Wayland and a machine whose libraries can be changed, and the rented machine is neither. Also stated: "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead." (Manual, Installing on Linux, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html, read 2026-07-30.) And, on Wayland with NVIDIA hardware: "Currently NVIDIA drivers don't fully support features needed for Wayland. Graphical glitches and flickering are common problems. In some cases, there can be crashes on startup." (Manual, Linux Windowing Environment, read 2026-07-30.) That last one is untested: the rented machine has NVIDIA hardware but runs Xorg.

Two of the three systems are now tested. Windows is not, no Windows machine having run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Hand tests on a machine with no display server at all — observed.** On **2026-08-06**, on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-5.1.2-linux-x64/blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`. Label: **observed**. Not merged with the one-card rented machine's results or the Creator's Mac.

**This is a different claim from the block above.** That machine had a display server running and its runs were cut off from it by removing `DISPLAY`. This machine has none, and nothing was installed or started to change that:

```
DISPLAY=[] WAYLAND_DISPLAY=[] XDG_SESSION_TYPE=[tty]
--- /tmp/.X11-unix:
total 8
drwxrwxrwt  2 root root 4096 Aug  6 10:11 .
drwxrwxrwt 11 root root 4096 Aug  6 11:38 ..
--- X server processes:
(none)
```

The X socket directory is empty, no X or Wayland process is running, and the session type is `tty`.

**Blender started and rendered there, on every engine this version has, on the cards and on the processor.** The starting scene at 64 by 64, four samples, with `-b` and `-E <engine>`, once with `-- --cycles-device OPTIX` and once with `-- --cycles-device CPU`:

```
GPU  CYCLES exit=0  png=5065
GPU  BLENDER_EEVEE_NEXT exit=0  png=3431
GPU  BLENDER_EEVEE exit=0  png=3431
GPU  BLENDER_WORKBENCH exit=0  png=3669
CPU  CYCLES exit=0  png=5062
CPU  BLENDER_EEVEE_NEXT exit=0  png=3431
CPU  BLENDER_EEVEE exit=0  png=3431
CPU  BLENDER_WORKBENCH exit=0  png=3669
```

The same runs repeated with `--debug-cycles` showed the work really was where it was asked for:

```
on the card:      Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                                       Hardware Ray-Tracing: Off
                                   NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
on the processor: Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]
                                       Hardware Ray-Tracing: Off
```

**Asked for a window on a machine with no display server, Blender said so and stopped:**

```
00:00.058  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.058  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4645, total unfreed memory 1.212318 MB
```

It never printed the line the script asked it to print, wrote this to **standard output**, and exited **1**. Nothing was written to standard error. Word for word what the one-card machine printed when cut off from its display, with the same exit status, plus the memory-leak line — so for this version the two cases are indistinguishable in what Blender says and does. `-b -E help` worked here, exit 0, listing `BLENDER_EEVEE`, `BLENDER_WORKBENCH`, `CYCLES`.
**Not covered.** Windows. Nothing about the OpenGL floor: this machine's cards are far above it.

---

## Where I looked

**Whole-tree searches.** I searched all of `Blender Sources/manual_5.1`, `Blender Sources/api_5.1` and `Blender Sources/release_notes_5.1` at once, rather than opening files and hoping.

Terms searched, and what came back:

| Term | Manual 5.1 | API 5.1 | Release notes 5.1 |
| --- | --- | --- | --- |
| `system requirement` | 2 files (`getting_started/index.txt`, `getting_started/installing/index.txt`) | none | none |
| `requirement` | many, nearly all unrelated (add-on manifests, Rigify rig types, stereoscopy, colour management); the relevant ones are the installing pages, `troubleshooting/startup.txt`, `getting_started/configuration/hardware.txt`, `getting_started/installing/linux_windowing_environment.txt` | none relevant | one, about node-tool idnames |
| `minimum` | many; relevant only in the GPU troubleshooting pages and `troubleshooting/startup.txt` — the rest are brush, sampling and modifier settings | none relevant | none relevant |
| `glibc` | **no hits anywhere** | no hits | no hits |
| `SSE4`, `SSE2`, `AVX` | **no hits anywhere** | no hits | no hits |
| `OpenGL 3.3` | 12 hits, all the same sentence on the per-platform GPU troubleshooting pages | none | none |
| `OpenGL 4.3` | 1 hit, and it is a debug switch on the command line page, not a requirement | none | none |
| `Vulkan 1.3` | **no hits anywhere** | no hits | no hits |
| `Vulkan` | 4 files: command line arguments, System preferences, colour management, Linux NVIDIA troubleshooting | none relevant | EEVEE benchmarks, VR |
| `Metal` | many, nearly all the word "metallic" in shader pages; the real ones are command line arguments, System preferences, Cycles GPU rendering | none relevant | EEVEE benchmarks, VR |
| `Windows 8.1`, `Windows 11`, `Ventura`, `8 GB` | **no hits anywhere** | no hits | no hits |
| `Windows 10` | 2 hits, both about virtual-reality headsets | none | none |
| `RAM`, `VRAM` | many, all about memory use during work, none a requirement | 2 hits, EEVEE settings | none |
| `64-bit` | 3 hits: Python add-on libraries, USD, Blender's history page | none | none |
| `driver` | many, nearly all about animation drivers; the graphics-driver ones are the GPU troubleshooting pages and Cycles GPU rendering | — | animation drivers only |
| `libdecor` | 1 file, `getting_started/installing/linux_windowing_environment.txt` | none | 1 hit in `core.md` |
| `fails to start`, `will not start`, `cannot run` | 1 hit, `troubleshooting/startup.txt` | none | none |
| `Unsupported Graphics Driver` | 13 hits across the GPU troubleshooting pages | none | none |
| `Apple Silicon` | 5 hits: Cycles GPU rendering (2), colour management (2), Installing on macOS (1) | none | none |
| `prerequisite`, `must be installed`, `dependenc` | all hits about Python add-ons and extensions, none about running Blender | — | — |

**Pages opened in full:** `getting_started/index.txt`, `getting_started/installing/index.txt`, `getting_started/installing/windows.txt`, `getting_started/installing/macos.txt`, `getting_started/installing/linux.txt`, `getting_started/installing/linux_windowing_environment.txt`, `getting_started/configuration/hardware.txt`, `troubleshooting/startup.txt`, `troubleshooting/gpu/index.txt`, `troubleshooting/gpu/common/introduction.txt`, `troubleshooting/gpu/common/other.txt`, all twelve per-platform pages under `troubleshooting/gpu/`, `troubleshooting/python.txt` (first part), `advanced/deploying_blender.txt`, `advanced/command_line/arguments.txt` (the background, GPU and misc sections). All release notes for 5.1 were listed and `index.md`, `core.md`, `eevee.md` and `cycles.md` were read. The requirements page was read in full.

**Cycles files:** `__init__.py`, `properties.py`, `engine.py` searched for version and requirement declarations — see (i).

**Ruled out, and why:**

- **`getting_started/configuration/hardware.txt`** ("Configuring Peripherals") — I opened it because the installing page points at it for hardware. It covers displays, mice, keyboards, tablets, touchpads, 3D mice and VR headsets. None of it is required to start Blender; it is about what makes Blender pleasant to use, and it says so ("A full HD display (1920x1080) or higher is recommended", "recommended: three button mouse with scroll wheel"). The VR section does list requirements, but for virtual-reality sessions, not for starting Blender.
- **The Python API documentation (1,899 files)** — searched for `system requirement`, `glibc`, `SSE4`, `minimum required OpenGL`, `OpenGL 4.3`, `Vulkan 1.3`, `requirements page`. No hits. The API documentation describes Blender's own data and functions; it does not describe the machine. Ruled out as a source for this question.
- **Cycles' own files** — the requirement lines there are about graphics cards for rendering, not about starting Blender. Ruled out for this question.
- **The pictures in the manual** — I searched `_images` for filenames containing `requir`, `system_info` and `install`. What came back was a Windows-installer screenshot for the manual's own contributor guide and four Rigify rig diagrams. None shows a requirements table. No picture was worth opening for this question.
- **`troubleshooting/crash.txt`** — mentioned upgrading RAM or the graphics card as a response to crashes, not as a condition for starting. Not used.

**I did not use the web.** Every quotation above comes from the files supplied.

---

## Things a person with Blender open could settle by hand in a few minutes

This list has since been run on two machines. What the Creator's own MacBook Pro settled is written into (j) and (k) above, and what the rented Linux machine settled into (l). The two lists live at `Audits/Blender Research/The machine/Hand tests.md` and `Audits/Blender Research/The machine/Hand tests — Linux.md`. **A third machine has since been used as well** — a second rented Linux machine with two graphics cards and no display server — and what it settled is written into this answer as **Hand tests on a machine with no display server at all — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`. *(Added 2026-08-06 by the citation check of the two-card material: this closing list still said two machines.)* **A fourth machine has since been used as well** — a third rented Linux machine with **four** graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the four-card machine — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

### Still open

Nothing on this list is still open. Both system-information files on the Mac have been read end to end and are in (k), and the Linux one is in (l). Two items that needed a Linux machine are now answered in (l): Blender does start and render on Linux with no X11 and no Wayland running, and what it reports about its own windowing system when asked to open a window is quoted word for word. What is left is below.

### Cannot be run on the equipment available

The equipment is one MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, whose graphics are Apple's own and built in and whose driver arrives with the operating system, and one rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores — reached over SSH, running Xorg, on which nothing may be installed or removed. **Brought up to date 2026-08-06 by the citation check of the two-card material.** A third machine has since been used — a second rented Linux machine, Ubuntu 22.04.5 LTS, **two** NVIDIA GeForce RTX 4090s on driver 580.95.05, sixty-one cores, **no display server at all**, over SSH — so "no second card" no longer holds, and what it settled is in the two-card block above. There is still no pair of cards joined by NVLink: this pair talks through the host bridge. The one-card rented Linux machine was destroyed on 2026-08-06 and nothing can be run on it again. And its driver *was* removed and reinstalled on the two-card machine, on purpose, for the broken-driver test recorded in the first-pass findings for this version.

**A fourth machine has since been used, and it is the only one still alive.** A **third rented Linux machine** — Ubuntu 22.04.5 LTS, **four** NVIDIA GeForce RTX 4060 Ti of **16 GB each** on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, **197 GB of memory**, **no display server**, over SSH. What it settled is written into the answers above as **Hand tests on the four-card machine — observed**, and its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. Three limits above are narrowed by it and none is removed: more than two cards is no longer out of reach, a card small enough to be filled is no longer out of reach, and the several-card form of every question under 5 and 6 has now been asked twice on two different machines. **Still out of reach on it as well:** any pair joined by NVLink, these four talking through the host bridge; cards of two different makes; cards of unequal speed, all four being the same model within about 2 % of each other; and anything needing eyes on a window, this machine having no display server either. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

- **Does Blender 5.1 start with no graphics card driver support?** Needs a machine with a very old card or a rolled-back driver. Neither machine can be put in that state.
- **Does Blender 5.1 start in background mode with no display and no graphics driver?** The no-display half is answered in (l) — it does. The no-graphics-driver half needs a machine without one.
- **Does Blender 5.1 start under Gnome-Shell with libdecor removed, and what does `--no-window-frame` change on Wayland?** Needs a Wayland session and a machine whose libraries can be changed. That the argument exists in this version is settled in (l).
- **Which processor architectures does the 5.1 macOS download come in?** This is a question about Blender's download page rather than about a machine, so no test here can settle it; it belongs with research.

---

2026-08-01 — Claude Opus 5 (Claude Code) — The machine second pass, Blender 5.1

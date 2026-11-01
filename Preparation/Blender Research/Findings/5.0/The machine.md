# The machine — Blender 5.0

**Sources consulted:** Blender 5.0 manual (`manual_5.0`, including one image from `_images`), Blender 5.0 Python API documentation (`api_5.0`), Blender 5.0 release notes (`release_notes_5.0`), and Cycles' own files (`cycles_5.0`). All read on **2026-07-30** (the snapshot date given to me), which is the date read for every finding below.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

Two notes on that line, so it is exact:

- Inside `Blender Sources/` I also opened `About this folder.md`, the folder's own note. It links to a project document; I did not follow that link or open anything it points to.
- `Blender Sources/` also contains loose files at its top level (`crashes.txt`, `cycles_device.txt`, `devices_api.txt`, `eevee_render.txt`, `eevee_results.txt`, `missing.txt`, `missing_filtered.txt`) that are not part of the four source sets I was told to use and are not described in the folder's own note. I did not open them and nothing below comes from them. If they were meant to be sources, this work does not cover them.

---

## 1. What must a computer have on it before Blender will run at all?

**Label: documented** except where a section is marked **inferred**, and **not in the sources** for the numeric minimums (memory, disk space, processor, operating-system versions, supported card lists).

### An operating system Blender is built for, and a matching processor architecture

> "Blender is available for download on Windows, macOS, and Linux."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html, read 2026-07-30.

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html, read 2026-07-30.

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/macos.html, read 2026-07-30.

The release notes state a change that bears directly on the macOS line:

> "Support for Intel Macs has been removed"

— Release notes, https://developer.blender.org/docs/release_notes/5.0/, read 2026-07-30. (The manual page above still names Intel; both are quoted as found. The release-notes entry links out to a discussion page that is not in the sources.)

### A graphics card and driver that reach OpenGL 3.3

> "This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

— Manual, https://docs.blender.org/manual/en/5.0/troubleshooting/gpu/windows/nvidia.html (the same sentence appears on every page under Troubleshooting Graphics Hardware), read 2026-07-30.

That sentence is the explanation of the "Unsupported Graphics Driver Error". The same section says which way out exists:

> "Installing the latest version of the correct driver can help upgrade the OpenGL version, though some graphics cards are simply too old to run the latest Blender."

— same page, read 2026-07-30.

The release notes record a change here without giving the figures:

> "Increase Minimum GPU requirements for Blender"

— Release notes, https://developer.blender.org/docs/release_notes/5.0/, read 2026-07-30. It links out to a discussion page that is not in the sources, so the new figures are not available to me.

### The files that ship with Blender, in place

> "System directories store files that come bundled with Blender and are required for it to function. This includes scripts, presets, essential assets and more."

— Manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html, read 2026-07-30.

That includes Python, which Blender brings with it rather than finding on the machine:

> "Blender bundles a complete Python installation and does not interact with the system Python by default."

— Manual, https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html, read 2026-07-30.

> "found bundled python: {DIR}
>
> This message indicates that Blender was able to find the Python library for the Python interpreter embedded within Blender. If this folder is missing or unable to be found, it is likely that an error will occur, and this message will not appear."

— Manual, https://docs.blender.org/manual/en/5.0/troubleshooting/startup.html, read 2026-07-30.

### On Linux, X11 and Wayland are the documented windowing choices — **inferred requirement**

> "On Linux Blender supports both X11 and Wayland for official releases. When Wayland is detected, it is the preferred system, otherwise X11 will be used."

and, under Requirements:

> "Under Gnome-Shell the libdecor library is required. This is available as a package on most Linux distribution. If the library isn't found X11 will be used as a fallback."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/linux_windowing_environment.html, read 2026-07-30.

**The step:** the page names X11 and Wayland as the supported choices and calls libdecor a requirement under Gnome Shell, with X11 as the fallback. It does not directly say that every windowed Linux start requires one of X11 or Wayland. The command-line quote below separately says a render can run without a graphical display.

### Checks the manual recommends after a failed start

> "See if your computer meets the minimum requirements.
>
> Confirm that your graphics card is supported and that the drivers are up to date […]
>
> Make sure any antivirus software is not preventing Blender from starting.
>
> Ensure that you have appropriate user permissions on the computer."

— Manual, https://docs.blender.org/manual/en/5.0/troubleshooting/startup.html, read 2026-07-30.

### What the sources do not give: the actual minimums

The manual states that minimums exist and sends the reader off-site for them:

> "Blender has a set of minimum and recommended requirements; so make sure these are met before trying to install Blender."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html, read 2026-07-30. Each of the three install pages repeats "Check the Downloading Blender page to find the minimum requirements". That page is on blender.org, not in the documentation snapshot, so no figure for memory, disk space, processor generation, or operating-system version is available here.

**Where I looked, to be able to say that.** I searched the whole manual, the whole API documentation and the whole release notes for: `system requirement`, `hardware requirement`, `minimum spec`, `required to run`, `requirement`, `minimum`, `RAM`, `disk space`, `processor`, `64-bit`, `SSE`, `AVX`, `glibc`, `redistributable`, `runtime library`, `dependencies`, `prerequisite`, `OpenGL 3.3`, `OpenGL 4.3`, `Vulkan 1.`. The only hits that carry a requirement are the ones quoted above. I read in full: `getting_started/installing/` (all five pages), `getting_started/index.txt`, `getting_started/configuration/hardware.txt` and `introduction.txt`, `troubleshooting/startup.txt`, `troubleshooting/crash.txt`, all sixteen pages under `troubleshooting/gpu/`, `advanced/deploying_blender.txt`, `advanced/blender_directory_layout.txt`. `getting_started/configuration/hardware.txt` covers displays, mice, tablets and 3D mice as recommendations, not requirements, so I ruled it out for this question.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 5.0 started and ran normally, and **Help ‣ Save System Info** was used to write `system-info.txt`. The file's section headed `GPU:` read:

```
renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:
```

followed by a section headed `Implementation Dependent GPU Limits:` reading `Maximum Batch Vertices:	-2147483648` and `Maximum Batch Indices:	-2147483648`. Only the `GPU` and `Cycles` sections were copied back, so nothing here records the rest of the file, or whether it carries any section naming an OpenGL version. This records what Blender reported on a machine that started successfully; it does not test the OpenGL 3.3 floor.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — `/opt/blender/blender-5.0.1-linux-x64/blender` (`Blender 5.0.1`) was put to three tests bearing on this question. The machine is a container sharing somebody else's hardware, not a whole computer, and other work was running on it at the same time, so nothing here says how long anything took or how much memory was free.

**With no display reachable, Blender started and rendered.** `DISPLAY` was removed from the environment and a background Cycles render of the test file completed at exit `0`, writing its PNG, with the graphics card doing the work:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
                            | Denoising on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
00:07.070  render           | Saved: '/root/smoozy/The machine/work/nd-5.0-0001.png'
```

*(Corrected 2026-08-06 by the runner. This originally said "with no display server at all", which the machine it was run on did not match — that machine had a real display running and the run was merely cut off from it. The two are different claims and the difference is the whole of what a later machine settled: a computer with genuinely no display server is recorded separately, on the second rented Linux machine, which has none. Nothing observed here changes; only the description of the machine does.)*

An EEVEE render did the same, also with `DISPLAY` removed, at exit `0`. So the X11-or-Wayland requirement is about opening a window, not about running.

**Asked for a window with no display reachable, Blender refused and exited.** `env -u DISPLAY <blender> --python-expr "import bpy; print('WINDOW OPENED')"` never printed `WINDOW OPENED`. It printed:

```
00:00.024  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.024  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4556, total unfreed memory 1.272339 MB
```

and exited with status `1`. Worth recording against the version above it: 4.5 on the same machine printed the same information in its own plainer shape and then **hung**, having to be killed. 5.0 exits.

**One line 5.0 prints on this machine that 4.5 does not**, but only when Cycles is asked for HIP:

```
00:00.287  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
```

That is Cycles reporting that AMD's library is not on the machine. It is harmless here and the render proceeds, but anything reading Blender's output has to expect it.

*(Corrected 2026-08-06 by the runner, after the check of the Linux hand tests could not reproduce this. The version split is right; "nearly every run" was not. Run again on the same machine with a plain Cycles render at factory settings, this line appeared **not at all**, in any of the four versions. It appears only when Cycles is actually asked for HIP — either the preferences are set to HIP, or the render is started with `--cycles-device HIP`. Then 5.0, 5.1 and 5.2 each print it exactly once and 4.5 prints nothing of the kind, failing instead with `Error: Found no Cycles device of the specified type`. So anything reading Blender's output has to expect this line when it asks for HIP, and not otherwise.)*

The OpenGL floor is still untested: the rented machine reports `4.6.0 NVIDIA 580.105.08`, above both figures the sources give.

**Not covered.** No numbers for memory, storage, processor or operating-system version. Nothing on whether a machine that falls below OpenGL 3.3 refuses to start or starts and then fails. Nothing on what a virtual machine or container needs beyond the note that "Running Blender inside a virtual machine is known to have problems when OpenGL drawing calls are forwarded to the host operating system" and that PCI passthrough resolves it (manual, https://docs.blender.org/manual/en/5.0/troubleshooting/gpu/windows/other.html, read 2026-07-30).

**Outside the interface.** Partly different. The manual says a display is not needed when rendering from the command line:

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

— Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html, read 2026-07-30. The sources do not say whether the OpenGL 3.3 minimum still applies when Blender is started with `-b` and never opens a window, and no machine below that floor exists to test it. What is now observed is the manual's own claim: on the rented Linux machine, with no display server reachable at all, 5.0 started, rendered with Cycles on the graphics card, and rendered with EEVEE. What does **not** hold outside the window is opening one, and what Blender says then is quoted in the Linux tests above.

**Operating system.** The architecture requirement differs by system (x64/arm64 on Windows; Apple Silicon on macOS, with Intel support removed in 5.0). The windowing requirement (X11/Wayland/libdecor) is Linux-only, and Linux is now tested: with neither running, Blender refuses to open a window, names both systems it tried, and exits `1`. The OpenGL 3.3 sentence is repeated identically on the Windows, Linux and macOS troubleshooting pages, so it is stated for all three; two of the three now have a machine that is far above it, and none has a machine below it. On macOS the driver cannot be updated separately: "On macOS, graphics drivers are built into the operating system and the only way to get newer drivers is to upgrade macOS as a whole to the latest version." (manual, https://docs.blender.org/manual/en/5.0/troubleshooting/gpu/apple/nvidia.html, read 2026-07-30).

**Hand tests on a machine with no display server at all — observed.** On **2026-08-06**, on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-5.0.1-linux-x64/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`. Label: **observed**. Nothing here is merged with the one-card rented machine's results or the Creator's Mac.

**This is a different claim from the block above.** The one-card machine had a display server running and its runs were cut off from it by removing `DISPLAY` from the environment. This machine has none, and nothing was installed or started to change that:

```
DISPLAY=[] WAYLAND_DISPLAY=[] XDG_SESSION_TYPE=[tty]
--- /tmp/.X11-unix:
total 8
drwxrwxrwt  2 root root 4096 Aug  6 10:11 .
drwxrwxrwt 11 root root 4096 Aug  6 11:38 ..
--- X server processes:
(none)
--- /dev/dri:
crw-rw----  1 root video  226,   0 Aug  6 10:11 card0
crw-rw----  1 root video  226,   1 Aug  6 10:11 card1
crw-rw----  1 root render 226, 128 Aug  6 10:11 renderD128
crw-rw----  1 root render 226, 129 Aug  6 10:11 renderD129
```

The X socket directory is empty, no X or Wayland process is running, and the session type is `tty`.

**Blender started and rendered there, on every engine this version has, on the cards and on the processor.** The starting scene at 64 by 64, four samples, rendered with `-b` and `-E <engine>`, once with `-- --cycles-device OPTIX` and once with `-- --cycles-device CPU`:

```
GPU  CYCLES exit=0  png=4992
GPU  BLENDER_EEVEE_NEXT exit=0  png=3358
GPU  BLENDER_EEVEE exit=0  png=3358
GPU  BLENDER_WORKBENCH exit=0  png=3596
CPU  CYCLES exit=0  png=4994
CPU  BLENDER_EEVEE_NEXT exit=0  png=3358
CPU  BLENDER_EEVEE exit=0  png=3358
CPU  BLENDER_WORKBENCH exit=0  png=3596
```

`BLENDER_EEVEE_NEXT` and `BLENDER_EEVEE` are both accepted by this version and give the same file size. The same runs repeated with `--debug-cycles` showed the work really was where it was asked for:

```
on the card:      Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                                   NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
on the processor: Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]
```

**Asked for a window on a machine with no display server, Blender said so and stopped.** `blender --python-expr "import bpy; print('RAN WITH A WINDOW')"` under `timeout 120` never printed `RAN WITH A WINDOW`, wrote to **standard output**, and exited **1**:

```
00:00.036  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.036  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4556, total unfreed memory 1.272339 MB
```

That run also wrote to **standard error**, fourteen times over:

```
Freeing memory after the leak detector has run. This can happen when using static variables in C++ that are defined outside of functions. To fix this error, use the 'construct on first use' idiom.
```

Word for word what the one-card machine printed when cut off from its display, with the same exit status, plus the memory-leak line. **So for this version the two cases are indistinguishable in what Blender says and does.**

**`-b -E help` worked here**, exit 0, printing:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

**Not covered.** Windows. Nothing about the OpenGL floor: this machine's cards are far above it and no card below the floor can be reached.

---

## 2. What must a computer have on it before Blender will render using its graphics cards?

**Ambiguity flagged.** I read this as Cycles rendering on the graphics card, because that is what the sources treat as an optional capability with prerequisites. The other reading is EEVEE, which uses the graphics card always and has no separate requirements page; I give what the sources say about it at the end of this answer.

**Label: documented**, with **observed** additions from Cycles' own files.

### The three things to set

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

— Manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html, read 2026-07-30.

### What each backend requires

All quotes from the same page and date.

> "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher."

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."

> "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards."

> "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."

> "Minimum driver versions:
>
> Windows: Radeon Software 24.9.1 or Radeon PRO Software 24.Q4
>
> Linux: Radeon Software 23.40 or ROCm 6.0"

> "oneAPI is a computation library that is supported on Windows and Linux and requires a Intel® Arc™ graphics card with the Xe HPG architecture."

> "Minimum driver versions:
>
> Windows: Intel Graphics Driver XX.X.101.5518
>
> Linux: intel-level-zero-gpu package 1.3.27642, typically available through the intel-compute-runtime package XX.XX.34666.3"

> "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

### What the release notes record as changed

> "The minimum supported compute capability is `sm_50` now (~GeForce 900 cards)."

> "The minimum required Windows AMD GPU driver has increased to Adrenalin version 24.9.1 or Radeon Pro driver 24.Q4"

— Release notes, https://developer.blender.org/docs/release_notes/5.0/cycles/, read 2026-07-30.

### Drivers, and what is not needed

> "In case of problems, be sure to install the official graphics drivers from the GPU manufacturers website, or through the package manager on Linux. The graphics drivers provided by the computer manufacturer can sometimes be outdated or incomplete."

> "Normally users do not need to install the CUDA toolkit as Blender comes with precompiled kernels."

— Manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html, read 2026-07-30.

### The same requirements, as Cycles itself states them — **observed**

Cycles' own preferences panel carries requirement text that is shown when no usable card is found. It differs from the manual in two places, so both are given.

> ```
>             elif device_type == 'HIP':
>                 import sys
>                 if sys.platform[:3] == "win":
>                     adrenalin_driver_version = "24.9.1"
>                     pro_driver_version = "24.Q4"
>                     col.label(
>                         text=rpt_("Requires AMD GPU with RDNA architecture"),
> ```
>
> ```
>                 elif sys.platform.startswith("linux"):
>                     rocm_version = "6.0"
>                     driver_version = "23.40"
> ```
>
> ```
>             elif device_type == 'ONEAPI':
>                 import sys
>                 if sys.platform.startswith("win"):
>                     driver_version = "XX.X.101.8132"
> ```
>
> ```
>                     col.label(
>                         text=rpt_("  - intel-level-zero-gpu or intel-compute-runtime version"),
>                         icon='BLANK1',
>                         translate=False)
>                     col.label(text=rpt_("    %s or newer") % driver_version, icon='BLANK1', translate=False)
>                     col.label(text=rpt_("  - oneAPI Level-Zero Loader"), icon='BLANK1', translate=False)
> ```
>
> ```
>             elif device_type == 'METAL':
>                 mac_version = "12.2"
>                 col.label(text=rpt_("Requires Apple Silicon with macOS %s or newer") % mac_version,
> ```

— File `Blender Sources/cycles_5.0/properties.py`, inside `CyclesPreferences._draw_devices`, lines 1823–1870 (the HIP branch begins at 1823, oneAPI at 1848, Metal at 1867). Read 2026-07-30.

Two differences worth stating plainly, without reconciling them:

- The Windows oneAPI driver figure in Cycles' own file is `XX.X.101.8132`; the manual says `XX.X.101.5518`.
- The Metal figure in Cycles' own file is macOS **12.2**; the manual says macOS **13.0 or newer is required to support all features**. A related comment sits in Cycles' interface file: `# The MNEE kernel doesn't compile on macOS < 13.` (`Blender Sources/cycles_5.0/ui.py`, line 197, in `use_mnee`). Read 2026-07-30.

The oneAPI Level-Zero Loader appears only in Cycles' own file, not in the manual.

### The build itself must have GPU rendering in it

> "For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html, read 2026-07-30.

### The other reading of the question — EEVEE

EEVEE has no requirements page. What the sources say about it as a demand on the machine:

> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering"

> "Headless rendering is not supported on headless Windows systems."

— Manual, https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html, read 2026-07-30.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 5.0's **Edit ‣ Preferences ‣ System** page was opened and the **Cycles Render Devices** panel expanded. The row of technology buttons held exactly two, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — and two dropdown lists: **Kernel Optimization**, set to **Full**, offering **Off** and **Intersection Only**; and **MetalRT**, set to **Auto**, offering **Off** and **On**. Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed. No "Requires Apple Silicon with macOS … or newer" line appeared, because a Metal device was found, so this run records nothing about the macOS 12.2-against-13.0 disagreement above.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), a background Cycles render with `-- --cycles-device METAL` used the graphics card, printing `00:01.295  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".` and ending `01:36.974  render           | Fra: 1 | Mem: 1583M | Sample 16/16 (Using optimized kernels)`. EEVEE also rendered with no window open: `-b -E help` listed `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`, and `-b /tmp/machine-test.blend -E BLENDER_EEVEE -o /tmp/eevee-50- -f 1` exited `0`, ending `00:05.976  render           | Saved: '/tmp/eevee-50-0001.png'`. The manual's headless limitation is stated for Windows; macOS is now tested and EEVEE rendered. Full commands and output are under question 5.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, four things were put to 5.0.

**What this copy was built with.** `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed:

```
TYPES: (True, True, True, False, True, False)
```

Read against Cycles' own file, which unpacks that as `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt`, the official Linux x64 build carries CUDA, OptiX, HIP and oneAPI and not Metal.

**A graphics-card render works, with no window and with no display reachable.** With the technology saved as `CUDA` and the card ticked, a background render exited `0`, wrote its PNG and printed:

```
00:00.273  cycles           | Added device "NVIDIA GeForce RTX 4090" with id "CUDA_NVIDIA GeForce RTX 4090_0000:00:07".
```

ending, in the report `--debug-cycles` prints:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                            | 
                            | Denoising on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
```

With `OPTIX` saved, `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]`. Denoising ran on the processor in every graphics-card render here. The `(Using optimized kernels)` text the Mac carried did not appear at all on Linux; the report line above is what names the device.

**What the panel says when the chosen technology has no compatible card.** This is the case the Mac could never reach. With a window open on the machine's own display, the technology set from a script and the System page photographed with `bpy.ops.screen.screenshot`, 5.0 drew, under **oneAPI**:

```
No compatible GPUs found for Cycles
Requires Intel GPU with Xe-HPG architecture and
  - intel-level-zero-gpu or intel-compute-runtime version
    XX.XX.34666.3 or newer
  - oneAPI Level-Zero Loader
```

and under **HIP**:

```
No compatible GPUs found for Cycles
Requires AMD GPU with RDNA architecture
and ROCm HIP Runtime 6.0 or newer
or AMD driver version 23.40 or newer
```

So the Intel Linux figure a running 5.0 draws is `XX.XX.34666.3`, which is the manual's number. And the ROCm-against-Radeon-Software distinction the sources leave hanging is drawn plainly here as two alternatives, `and ROCm HIP Runtime 6.0 or newer` / `or AMD driver version 23.40 or newer` — 4.5 on the same machine names only the driver. This records what the panel displays on a machine with no Intel or AMD card; it does not say what Blender would enforce against a real one.

The requirement line for CUDA and OptiX was never drawn, a compatible NVIDIA card being present. The card's compute capability, as Blender's own system-information file reports it, is major `8`, minor `9` — far above the `5.0` this version states — so nothing here tests that figure.

**EEVEE renders headless on Linux.** With `DISPLAY` removed from the environment, `-b <file> -E BLENDER_EEVEE -f 1` exited `0` and wrote its PNG:

```
00:00.800  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.035  render           | Saved: '/root/smoozy/The machine/work/eeveend-5.0-0001.png'
```

The manual's headless limitation is stated for Windows. macOS and Linux are now both tested and EEVEE rendered on both; Windows is untested.

**Observed — what Blender does when the graphics driver itself is broken, 2026-08-06.** Every earlier answer here is about a technology having no card behind it. This is a different case and it had never been tested: the cards are present and the **driver** is gone or wrong. It was run on a second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, no display server, over SSH — which was broken on purpose for it, and which is not the machine the other rented-machine results in this document came from.

*What was done, to what.* Two states were made, and this version was asked in each to render on the card through OptiX. First the driver packages were purged outright, so `nvidia-smi` and the graphics library were gone. Then the driver was reinstalled, which left the kernel module already running at `580.95.05` while the new library arrived as `580.105.08` — a mismatch, and the state an unasked overnight upgrade leaves behind until the machine is restarted.

*The exact commands.* The scene was one script, `s.py`, written to the machine and used unchanged for every run:

```python
import bpy, sys
s = bpy.context.scene
s.render.engine="CYCLES"; s.cycles.device="GPU"; s.cycles.samples=16
s.render.resolution_x=s.render.resolution_y=128
s.render.filepath=sys.argv[-1]
p=bpy.context.preferences.addons["cycles"].preferences
p.compute_device_type="OPTIX"; p.refresh_devices()
for d in p.devices: d.use=(d.type=="OPTIX")
bpy.ops.render.render(write_still=True)
```

The driver was removed with:

```
apt-get purge -y "libnvidia-compute-580*" "nvidia-compute-utils-580*" "libnvidia-gl-580*" \
                 "nvidia-utils-580*" "nvidia-kernel-common-580*" "nvidia-dkms-580*"
```

which removed fifteen packages, after which `nvidia-smi` was gone (`command not found`) and no `libcuda.so*` remained. It was put back with `apt-get install -y cuda-drivers-580`. Each render was then run as:

```
BLENDER_USER_RESOURCES=<this version's own folder> <this version's blender> \
  -b --factory-startup --python s.py -- <output path>
```

*What came back, with the driver removed.* This version's own output, and the device list read straight afterwards:

```
4.5:  exit 0   (no line about the driver at all)
5.0:  exit 0   00:00.298  cycles  | WARNING CUEW initialization failed: Error opening the library
5.1:  exit 0   00:00.363  cycles  | WARNING CUEW initialization failed: Error opening the library
5.2:  exit 0   00:00.460  cycles  | WARNING CUEW initialization failed: Error opening the library

DEVICES: [('AMD EPYC 7B13 64-Core Processor', 'CPU')]
```

Each wrote its picture — 15,331 bytes in 4.5, and 15,390, 15,437 and 15,550 in the later three.

*What came back, with the driver mismatched.* Putting the driver back left the kernel module already running at one version and the new library at another, which the machine reports itself:

```
kernel module loaded: 580.95.05
library installed:    /usr/lib/x86_64-linux-gnu/libcuda.so.580.105.08

$ nvidia-smi
Failed to initialize NVML: Driver/library version mismatch
NVML library version: 580.105
```

Blender, in every version, on the same scene:

```
4.5:  exit 0   CUDA cuInit: Unknown CUDA error value                    devices seen: ['CPU']
5.0:  exit 0   00:03.201  cycles | ERROR CUDA cuInit: Unknown CUDA error value   devices seen: ['CPU']
5.1:  exit 0   00:03.841  cycles | ERROR CUDA cuInit: Unknown CUDA error value   devices seen: ['CPU']
5.2:  exit 0   00:03.737  cycles | ERROR CUDA cuInit: Unknown CUDA error value   devices seen: ['CPU']
```

A restart is what resolves it; reinstalling does not.

*So in both states the render was asked for on the card and happened on the processor, and the program reported success.* Nothing in the exit status separates a job that used the card from one that did not. Anything needing to know must read the run's own words, or ask Blender which devices it can see before starting.

*What a broken driver does not settle.* In both states the only device Blender listed was the processor, so no CUDA or OptiX device was found on a machine whose cards are physically present. Whether that is the same case as "no compatible card" for the purposes of the preferences panel was not seen and is not claimed here: this machine has no display server, so no panel could be drawn on it, and neither run captured the "No compatible GPUs found for Cycles" heading or the compute-capability line beneath it. **The disagreement over the compute-capability figure therefore stands exactly where it did.** *(Added 2026-08-06 by the citation check of the two-card material, which found the broken-driver block silent on this and the question easy to read as closed.)*

*One thing this version does differently:* with the driver removed it prints `WARNING CUEW initialization failed: Error opening the library`, where Blender 4.5 prints nothing at all.

*On what kind of computer.* The second rented Linux machine described above. **This settles Linux on an NVIDIA machine only.** It was not run on the Creator's Mac, and Windows is untried everywhere.

**Not covered.** No list of specific card models beyond the series names quoted. No figure for how much card memory a given scene needs. Nothing about the requirements for the graphics-card denoisers beyond the compute-capability figure quoted. What happens if a requirement is unmet is now partly observed rather than unknown: with the chosen technology having no card at all, the render moves to the processor silently, and with the technology named on the command line it refuses and exits `1` (question 4). What is still unknown is a card that is present but below a stated figure, which needs an AMD or Intel card, or an NVIDIA card older than this one.

**Outside the interface.** The requirements themselves are properties of the machine and read the same either way, but the manual states them as steps in the preferences window. The one point the sources make about setting the device without the window is the `--cycles-device` argument (question 5). **Whether a headless Linux machine with no display reachable can still reach a CUDA, HIP or oneAPI device is now observed: it can.** With `DISPLAY` removed from the environment, a background Cycles render on the one-card rented Linux machine used the NVIDIA card in every version tried, 5.0 among them. That machine had a display server running; the same on a machine with none at all is recorded under question 1. And the "No compatible GPUs found for Cycles" text does not reach a windowless run at all: with the technology saved as `ONEAPI`, for which this machine has no card, that phrase appeared nought times in the whole output of a background render, which completed on the processor at exit `0`.

**Operating system.** Every backend is named with its systems: CUDA, OptiX, HIP and oneAPI on Windows and Linux; Metal on macOS with Apple Silicon. Driver minimums for HIP and oneAPI are given separately for Windows and Linux. There is no backend listed for macOS other than Metal. Two systems are now tested and both match. On the Creator's Apple Silicon Mac the technology row offered **None** and **Metal** and nothing else. On the rented Linux machine it offered five buttons — **None**, **CUDA**, **OptiX**, **HIP**, **oneAPI** — and no Metal, and drew the Linux driver figures quoted above. Windows remains untested, and its figures cannot be read from Linux because the panel text branches on the operating system; a rented Windows machine would show them and none was used. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 3. Does what a computer must have on it depend on how Blender was obtained and put there?

**Label: documented.**

Yes, in several ways.

### The build may or may not contain graphics-card rendering

> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system […] Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html, read 2026-07-30.

The API documentation publishes build options for the documented build:

> "Constant value bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, …)"

— API documentation, https://docs.blender.org/api/5.0/bpy.app.html, read 2026-07-30.

### Rights and prerequisites at install time differ

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

> "When choosing the zip-file, you have to manually extract Blender to the desired folder […] No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html, read 2026-07-30.

> "Assuming snap is already installed, Blender can be installed through snap with: `snap install blender --classic`"

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html, read 2026-07-30.

> "Download and install the Steam client for your operating system."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/steam.html, read 2026-07-30.

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/installing/macos.html, read 2026-07-30.

### The Microsoft Store copy is a different kind of installation

> "is_microsoft_store_install
>
> Whether this blender installation is a sandboxed Microsoft Store version"

— API documentation, https://docs.blender.org/api/5.0/bpy.types.PreferencesSystem.html, read 2026-07-30.

Its files also sit elsewhere:

> "Microsoft Store installation:
>
> `%ProgramFiles%\WindowsApps\BlenderFoundation.Blender<HASH>\Blender\5.0\`"

— Manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html, read 2026-07-30. The same page lists a different system directory for Linux distribution packages (`/usr/share/blender/5.0/`) than for the blender.org archive (`./5.0/`).

### File association is handled differently again

> "On Linux, if Blender is installed from a package manager such as Snap, file association is handled by the package manager."

— Manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html, read 2026-07-30.

### Building it yourself can bring a toolkit troubleshooting step back into play

> "If you build Blender yourself, try to download and install a newer CUDA developer toolkit."

— Manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html, read 2026-07-30. Set against "Normally users do not need to install the CUDA toolkit as Blender comes with precompiled kernels" on the same page, this documents a troubleshooting recommendation for a self-built copy. It does not establish that the toolkit is always required when building Blender.

**Hand test on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, the copy under test was the official Linux x64 build from Blender's own download server, unpacked at `/opt/blender/blender-5.0.1-linux-x64/`. Asked what it was built with, `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed `TYPES: (True, True, True, False, True, False)` — CUDA, OptiX, HIP and oneAPI present, Metal absent. So the sharpest case in this answer, a copy that leaves Cycles graphics-card rendering out, is not what the official Linux download does; it carries all four of the technologies its operating system supports. A distribution's own package is a different copy and is untested, because nothing may be installed on the rented machine.

**Not covered.** The sources do not say that the hardware or driver minimums themselves change with the source of the copy — only the features present, the install-time rights, and the file locations. Nothing on whether the Microsoft Store sandbox restricts access to graphics cards. Nothing on the Steam copy's contents. Nothing on whether Snap's confinement (`--classic` is given without explanation) affects anything at run time. None of those could be tested: one Mac and one rented Linux machine, one install route each.

**Outside the interface.** The differences above are all properties of the installed copy and hold however Blender is started. Two are specifically about driving it from outside: the file-association arguments `-r`, `--register`, `--register-allusers`, `--unregister`, `--unregister-allusers` are marked "(Windows & Linux only)" (manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html), and the path to the executable differs per install method (https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html, `.../launch/macos.html`, `.../launch/linux.html`), read 2026-07-30. On macOS the executable is inside the bundle: "The path to the executable in the .app folder is ./Blender.app/Contents/MacOS/Blender."

**Operating system.** Every item above is tied to one system: package manager and Snap and TAR to Linux, installer/zip/Store to Windows, DMG and Gatekeeper approval to macOS. Steam spans all three.

---

## 4. Does Blender say anything about what is missing when it will not start, or when it will not render?

**Label: documented**, with **observed** additions from Cycles' own files.

Yes, in several places.

### A console window with startup messages

> "The Blender Console Window can display many different types of status and error messages. Some messages simply inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender non-responsive or shut down completely."

and the warning that you may not see it:

> "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window. To ensure you can see any error messages that appear at launch, open a command-line window and navigate to the location where the Blender executable is, and run blender from that location."

— Manual, https://docs.blender.org/manual/en/5.0/troubleshooting/startup.html, read 2026-07-30. The two examples the page gives are `found bundled python: {DIR}` and `Read prefs: {DIR}/userpref.blend`.

The same advice is repeated for crashes:

> "Crash on Startup
>
> Try running Blender from the command line, to see if any helpful error messages are printed."

— Manual, https://docs.blender.org/manual/en/5.0/troubleshooting/gpu/windows/other.html, read 2026-07-30.

On Windows the console is hidden by default: "The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." (manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html, read 2026-07-30).

### A named error for a graphics driver that is too old

> "Unsupported Graphics Driver Error
>
> This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

— Manual, https://docs.blender.org/manual/en/5.0/troubleshooting/gpu/windows/nvidia.html, read 2026-07-30.

### Named errors for graphics-card rendering

The Cycles page has an "Error Messages" section listing these by their text:

> "Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU."

> "The NVIDIA OpenGL driver lost connection with the display driver"

> "Unsupported GNU version"

> "CUDA Error: Kernel compilation failed
>
> This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed."

— Manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html, read 2026-07-30.

### Cycles states what is missing, in its own words — **observed**

When no usable card is found for the chosen backend, the preferences panel says so and names the requirement:

> ```
>         if not found_device:
>             col = box.column(align=True)
>             col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)
>
>             if device_type == 'CUDA':
>                 compute_capability = "5.0"
>                 col.label(text=rpt_("Requires NVIDIA GPU with compute capability %s") % compute_capability,
>                           icon='BLANK1', translate=False)
>             elif device_type == 'OPTIX':
>                 compute_capability = "5.0"
>                 driver_version = "535"
>                 col.label(text=rpt_("Requires NVIDIA GPU with compute capability %s") % compute_capability,
>                           icon='BLANK1', translate=False)
>                 col.label(text=rpt_("and NVIDIA driver version %s or newer") % driver_version,
>                           icon='BLANK1', translate=False)
> ```

— File `Blender Sources/cycles_5.0/properties.py`, `CyclesPreferences._draw_devices`, lines 1808–1822 (the HIP, oneAPI and Metal branches follow, quoted under question 2). Read 2026-07-30.

A device whose `is_optimized` value is false is labelled too:

> ```
>         for device in devices:
>             name = self._format_device_name(device.name)
>             if not device.is_optimized:
>                 name += rpt_(" (Unoptimized Performance)")
> ```

— Same file, lines 1873–1877. Read 2026-07-30.

Two other messages sit in Cycles' own files, for a build without Open Shading Language:

> ```
>             self.report({'ERROR'}, "OSL support disabled in this build")
> ```

— File `Blender Sources/cycles_5.0/__init__.py`, lines 111 and 118. Read 2026-07-30.

### A log that names the devices it found

Blender 5.0's release notes record a rebuilt logging system and show what it prints:

> "`--log-level info` enables all info level log messages, with a moderate amount of output. This may be useful to enable in production for troubleshooting issues."

and the example output includes:

> ```
> 00:00.359  gpu.platform     | Using GPU "Apple M3 Max Metal API 1.2"
> 00:00.359  gpu.platform     | Using Backend "Metal"
> 00:01.041  cycles           | Added device "Apple M3 Max (GPU - 40 cores)" with id "METAL_Apple M3 Max (GPU - 40 cores)".
> ```

— Release notes, https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30.

The manual describes the `info` level as covering exactly this:

> "info: Information about devices, files, configuration, operations"

— Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30. The same page gives `--log-file <filepath>` ("Set a file to output the log to") and `--log-list-categories`.

### A crash file

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."

— Manual, https://docs.blender.org/manual/en/5.0/troubleshooting/crash.html, read 2026-07-30. On Windows the file is named after the blend-file (`test.blend` → `test.crash.txt`), and the same page names the helper scripts shipped in the installation directory: `blender_debug_log.cmd`, `blender_debug_gpu.cmd`, `blender_debug_gpu_workaround.cmd`, `blender_factory_startup.cmd`.

### One thing that is stated only as a behaviour, not a message — **observed**

> ```
>     if args.cycles_device:
>         import _cycles
>         if not _cycles.set_device_override(args.cycles_device):
>             sys.exit(1)
> ```

— File `Blender Sources/cycles_5.0/engine.py`, `_parse_command_line`, lines 38–41. Read 2026-07-30. This is Cycles' own file saying that a rejected `--cycles-device` value ends the process with status 1. Whether anything is printed when that happens is decided inside `_cycles.set_device_override`, which is not one of the files I have.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, every technology button in Blender 5.0's **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** row was clicked in turn. The row held **None** and **Metal**. Neither the "No compatible GPUs found for Cycles" line nor any "Requires…" line appeared under either. The two tick boxes were labelled exactly **Apple M1 Pro (GPU - 14 cores)** and **Apple M1 Pro**. A Metal device was present on this machine, so the no-compatible-card case was never reached and this run records nothing about what it displays.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.0 in background mode was asked for devices it could not give. With `-- --cycles-device OPTIX` it exited `1`, wrote no image, and printed:

```
00:01.263  reports          | ERROR Found no Cycles device of the specified type
```

With `-- --cycles-device NOTADEVICE` it exited `1`, wrote no image, and printed:

```
00:00.231  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
```

So the `sys.exit(1)` in `engine.py` is accompanied by a message, and the message names the fault.

The other half is silent. Asked to render on graphics cards with every card switched off in the saved preferences, and again with the technology set to `NONE`, it exited `0`, wrote the image, and printed nothing naming a missing or unusable device. Full commands and output are under question 5.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, five things bearing on this question were put to 5.0.

**A device Cycles will not accept.** `-- --cycles-device METAL`, `-- --cycles-device HIP` and `-- --cycles-device ONEAPI` all ended at exit `1`, wrote no image, and printed one line:

```
00:00.447  reports          | ERROR Found no Cycles device of the specified type
```

HIP and oneAPI are built into this copy and simply have no card; Blender gives the same message for that as for Metal, which it was not built with at all. `-- --cycles-device NOTADEVICE` gave exit `1`, no image, and two lines:

```
00:00.187  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

The second line is one 4.5 does not print on the same machine. The messages themselves are word for word the Mac's.

**The quiet cases.** With every card switched off in the saved preferences, again with the technology set to `NONE`, and again with the technology set to `ONEAPI` for which there is no card, the render exited `0`, wrote its image, and printed nothing naming a missing or unusable device, falling back to `Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]`.

**Whether the Cycles panel text ever reaches a background session — settled, and the answer is no.** With the technology saved as `ONEAPI` (`SAVED: ONEAPI ACTIVE: False NUMGPU: 0`), the whole output of a background render was searched for `No compatible GPUs found for Cycles` and it appeared **nought times**. No `Requires…` line appeared either. What that text says in the panel is quoted under question 2.

**Where the crash file lands on Linux.** A crash forced with `-b <file> --python-expr "import ctypes; ctypes.string_at(0)"` gave exit `139` and one line:

```
Writing: /tmp/machine-test.crash.txt
```

With no blend-file loaded:

```
Writing: /tmp/blender.crash.txt
```

So the manual's Linux sentence is right about the folder and incomplete about the name: with a file loaded, the crash file is named after that file. It still goes to `/tmp` and not beside the blend-file — the test file was elsewhere. The file opens with the version and a backtrace, ending in a `# Python backtrace` section naming the line that caused it.

**What Blender says when it cannot open a window** is quoted in full under question 1: it names both windowing systems it tried and exits `1`.

**Not covered.** I cannot say whether a dialog box is ever shown, nor what Blender prints when it will not start for want of a good enough graphics card, no machine below the floor existing. I cannot say what is printed when a graphics card is present but its driver is below the minimum, as opposed to no card being found. What the Cycles panel messages look like in full is now recorded (question 2), and where they appear is settled: the panel only.

**Outside the interface.** The Cycles preferences messages are drawn into a panel and so belong to the window — now observed rather than reasoned, on the rented Linux machine. The log lines, the crash file, the `sys.exit(1)`, and the console messages are all reachable without one — `--log`, `--log-level`, `--log-file` and `--log-list-categories` are command-line arguments, and the manual explicitly recommends the command line as the way to see startup errors at all. On Windows the console is hidden unless Blender is started from one, which no machine used here could check.

**Operating system.** The console behaviour differs: hidden by default on Windows, printed to the terminal on Linux and macOS (manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/linux.html and `.../launch/macos.html`). Both of the latter are now observed. The crash-file naming and the `.cmd` helper scripts are described for Windows only; Linux naming is now observed and is above, and the Mac's was `blender.crash.txt` in its own temporary directory. Everything else quoted is stated without an operating system attached. One Linux-only line appears in this version's output that the Mac never showed: `WARNING HIPEW initialization failed: Error opening HIP dynamic library`, printed at the start of very nearly every run because AMD's library is not on the machine.

---

## 5. How is it decided which of a computer's graphics cards a render uses, can more than one card be used for the same render, and how is that decided from outside Blender's own interface?

**Label: documented** for the decision and for multi-card use; **observed** for the mechanics in Cycles' own files; **inferred** where marked.

### The decision has three levels

**One — which backend.** A single choice, held in the preferences, not in the scene:

> "Cycles Render Devices
>
> Changes the Computing Device the Cycles render engine uses to render images. […]
>
> None: When set to None or when the only option is None: the CPU will be used as the computing device for Cycles.
>
> CUDA: If the system has a compatible NVIDIA CUDA device, it will be available as an option for rendering with Cycles."

— Manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html, read 2026-07-30 (the OptiX, HIP, oneAPI and Metal entries follow in the same "If the system has a compatible …" form).

The picture on that page shows the panel with only the backends that machine offers:

**Documented, from the picture.** The image `_images/editors_preferences_section_system.webp`, shown on https://docs.blender.org/manual/en/5.0/editors/preferences/system.html, shows a "Cycles Render Devices" row with exactly three buttons — **None**, **CUDA**, **HIP** — with None selected. Read 2026-07-30.

**Inferred:** the backend choice is one row of mutually exclusive buttons, and the row is not a fixed list. **The step:** the picture shows one selected button and only three choices, while the quoted manual text names further backends that are available on compatible systems.

Cycles' own file shows how that list is built — **observed**:

> ```
>     def get_device_types(self, context):
>         import _cycles
>         has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()
>
>         list = [('NONE', "None", n_("Do not use compute device"), 0)]
>         if has_cuda:
>             list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))
> ```

— File `Blender Sources/cycles_5.0/properties.py`, lines 1581–1587. Read 2026-07-30.

And that the default is not the same everywhere — **observed**:

> ```
>     @staticmethod
>     def default_device():
>         import platform
>         # Default to selecting the Metal compute device on Apple Silicon GPUs
>         # (drivers are tightly integrated with macOS so pose no stability risk)
>         if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
>             return 5
>         return 0
> ```

— Same file, lines 1572–1579. Read 2026-07-30. Value 5 is `METAL` and 0 is `NONE` in `enum_device_type` (same file, lines 172–179).

**Two — which individual cards, within that backend.** Each detected device carries a switch — **observed**:

> ```
> class CyclesDeviceSettings(bpy.types.PropertyGroup):
>     # Runtime properties
>     __slots__ = ("is_optimized")
>
>     # Properties saved in preferences
>     id: StringProperty(name="ID", description="Unique identifier of the device")
>     name: StringProperty(name="Name", description="Name of the device")
>     use: BoolProperty(name="Use", description="Use device for rendering", default=True)
>     type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
> ```

— File `Blender Sources/cycles_5.0/properties.py`, lines 1558–1566. Read 2026-07-30.

New entries are added switched on, except the processor — **observed**:

> ```
>                 entry.use = entry.type != 'CPU'
> ```

— Same file, line 1680, in `update_device_entries`. Read 2026-07-30.

**Three — whether this scene uses them at all.** A per-scene choice with only two values:

> "Device
>
> Specifies the compute device used for rendering in Cycles. […]
>
> CPU: Render using the system's central processing unit. This mode is compatible with all Cycles features. […]
>
> GPU Compute: Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices. Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."

— Manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html, read 2026-07-30.

Confirmed in Cycles' own file — **observed**:

> ```
> enum_devices = (
>     ('CPU', "CPU", "Use CPU for rendering"),
>     ('GPU', "GPU Compute",
>         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
> )
> ```

— File `Blender Sources/cycles_5.0/properties.py`, lines 30–34; the property itself is `device: EnumProperty(name="Device", description="Device to use for rendering", items=enum_devices, default='CPU', …)` at lines 381–387. Read 2026-07-30.

### Can more than one card be used for the same render?

Yes, for Cycles.

> "Can multiple GPUs be used for rendering?
>
> Yes, go to Preferences ‣ System ‣ Compute Device Panel, and configure it as you desire."

> "Would multiple GPUs increase available memory?
>
> Typically, no, each GPU can only access its own memory.
>
> The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."

— Manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html, read 2026-07-30.

> "Distribute Memory Across Devices
>
> Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes. Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol.
>
> Currently only NVLink on NVIDIA GPUs is supported."

— Manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html, read 2026-07-30.

**No, for EEVEE:**

> "Multiple GPU Support
>
> There is currently no support for multiple GPU systems."

— Manual, https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html, read 2026-07-30.

One further constraint on mixing cards — **observed**:

> ```
>         # Any device without RT support will disable it for all.
>         has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt
> ```

— File `Blender Sources/cycles_5.0/properties.py`, `draw_impl`, lines 1907–1908. Read 2026-07-30. This governs whether the hardware ray-tracing option is offered; what it does to a render is not visible in these files.

### From outside Blender's own interface

**The backend, by argument.** Documented:

> "Cycles Render Options
>
> Cycles add-on options must be specified following a double dash.
>
> `--cycles-device <device>`
>
> Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU.
>
> Example:
>
> `blender -b file.blend -f 20 -- --cycles-device OPTIX`"

— Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30.

Cycles' own file states what that argument overrides — **observed**:

> ```
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)
> ```

— File `Blender Sources/cycles_5.0/engine.py`, `_configure_argument_parser`, lines 16–20. Read 2026-07-30.

**Inferred:** `--cycles-device` chooses a backend and whether the processor joins in, but cannot name one card out of several. **The step:** the list of accepted values contains only backend names and `+CPU`, with no form for naming an individual device; therefore selecting among several cards of the same backend must come from somewhere else — the per-device switches in the preferences. A reader has to supply that last inference, so this is inferred, not documented.

**Inferred:** those preference switches are saved in the user's preferences file, which is on disk and outside the running program. **The step:** Cycles' file marks `use` as a property saved in preferences, and the manual names `userpref.blend` as the preferences file:

> "`./config/userpref.blend`"

— Manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html, read 2026-07-30. That page also gives the containing folder per system (`$HOME/.config/blender/5.0/`, `/Users/$USER/Library/Application Support/Blender/5.0/`, `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.0\`), the `portable` folder that moves it beside the executable, and the `BLENDER_USER_RESOURCES` environment variable that replaces it.

**By script.** The manual documents `-P, --python <filepath>`, `--python-expr <expression>`, `--python-text <name>` and `-c, --command <command>` (which "implies --background mode"), all on https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30. Cycles' own files show the object a script would reach — **observed**:

> ```
> def get_device_type(context):
>     return context.preferences.addons[__package__].preferences.compute_device_type
> ```

— File `Blender Sources/cycles_5.0/ui.py`, lines 96–97. Read 2026-07-30. `bpy.ops.cycles.*` entries in the API documentation give the add-on's file paths as `addons_core/cycles/…` (https://docs.blender.org/api/5.0/bpy.ops.cycles.html, read 2026-07-30). The exact string `__package__` resolves to at run time is not stated in any source I have, so I will not assert the key to use.

**Also relevant to a run from outside:** `-E, --engine <engine>` ("Specify the render engine. Use `-E help` to list available engines"), `-t, --threads <threads>` ("Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count"), and `--cycles-print-stats` ("Log statistics about render memory and time usage") — same arguments page, read 2026-07-30. Note the manual's warning on the same subject: "Arguments are executed in the order they are given!" (https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html).

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 5.0.

**The panel.** **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** was expanded and every button in its technology row clicked in turn. The row held exactly two buttons, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — plus **Kernel Optimization** (**Full**; also **Off**, **Intersection Only**) and **MetalRT** (**Auto**; also **Off**, **On**). Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed.

**From Blender's Python Console** (Scripting workspace, window open). `list(bpy.context.preferences.addons.keys())` returned:

```
['io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'cycles', 'pose_library', 'bl_pkg']
```

so the add-on key is the literal string `cycles` on this machine. With `p = bpy.context.preferences.addons["cycles"].preferences`:

- `p.compute_device_type` returned `'METAL'`.
- `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned `[]` before anything else was called.
- After `p.refresh_devices()`, the same expression returned:

```
[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

- `p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, `p.has_multi_device()` returned `False`.
- `p.devices[0].use = False` was run — index 0 being the processor entry, unticked by default — and the panel then showed exactly what it shows by default. `p.devices[0].use = True` was run, and the **Apple M1 Pro** box in the panel became ticked.

This was done from a Python Console inside Blender's own window; it records nothing about a background run.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, which reports `Blender 5.0.1`. Scene: `/tmp/machine-test.blend`, Cycles, scene Device `GPU`, 16 samples. Output quoted as printed.

**The saved state found on this machine, recorded first because it shapes what follows.** Reading the preferences with no window open printed:

```
CYCLES KEYS: ['cycles']
BACKEND: METAL
BEFORE: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False)]
AFTER: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False)]
```

In this version, on this machine, the graphics card was switched **off** in the saved preferences — unlike 4.5, 5.1 and 5.2 on the same machine, where it was on. That is the state of one person's installation, not a statement about Blender. The stored list was already populated before `refresh_devices()` was called. The same command with `--factory-startup` added printed `BEFORE: []` and then `AFTER:` with the card ticked and the processor unticked, so `--factory-startup` discards the saved per-card switches.

**The raw list.** `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types()); print('METAL:', _cycles.available_devices('METAL')); print('NOARG:', _cycles.available_devices())"` printed:

```
TYPES: (False, False, False, True, False, False)
METAL: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False, True, True, False, True))
```

and the no-argument call raised `TypeError: function takes exactly 1 argument (0 given)`. So each entry is an eight-field tuple, and the no-argument form that `engine.py` wraps is refused.

**A background render with the saved preferences as found** (card off) exited `0`, wrote its PNG, and printed:

```
00:01.295  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
00:01.295  cycles           | Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
00:01.297  cycles           | Using default CPU kernels.
00:01.300  cycles           | Using default CPU kernels.
00:06.097  render           | Fra: 1 | Mem: 151M | Sample 16/16
00:06.097  render           | Fra: 1 | Mem: 151M | Finished
```

The scene asked for `GPU Compute`; the run completed and produced an image, with no message naming a missing or unusable device.

**Every card switched off deliberately, then rendered.** The setting command printed `SAVED STATE: METAL [('Apple M1 Pro', False), ('Apple M1 Pro (GPU - 14 cores)', False)]` — so `bpy.ops.wm.save_userpref()` runs with no window — and the render ended `00:05.388  render           | Fra: 1 | Mem: 151M | Finished`, again with no such message.

**The same state with `-- --cycles-device METAL` added** exited `0` and printed:

```
01:32.826  render           | Fra: 1 | Mem: 1433M | Sample 0/16 (Using optimized kernels)
01:36.974  render           | Fra: 1 | Mem: 1583M | Sample 16/16 (Using optimized kernels)
```

**With the scene's own Device set to `CPU`** in a saved copy of the file, `-- --cycles-device METAL` still gave `00:04.740  render           | Fra: 1 | Mem: 1583M | Sample 16/16 (Using optimized kernels)`. So the flag displaced both the saved preferences and the scene setting.

**The technology set to `NONE` in the saved preferences** produced a render on the processor with no message, ending `00:05.296  render           | Fra: 1 | Mem: 151M | Sample 16/16`.

**Preference changes made inside the render's own run.** Setting every device's `use` to `True` in the same process that then rendered printed `SET TO: [('Apple M1 Pro', True), ('Apple M1 Pro (GPU - 14 cores)', True)]` and ended `00:04.971  render           | Fra: 1 | Mem: 151M | Sample 16/16` — without the `(Using optimized kernels)` text that the `--cycles-device METAL` run carried. Setting them all to `False`, and setting `compute_device_type` to `'NONE'`, gave the same shape of result. The saved preferences were unchanged afterwards, and were restored to the state found.

**One thing about this machine.** Its Blenders carry third-party add-ons. Several runs in this version ended in exit status `139` with a line like `Writing: /var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/blender.crash.txt`; the crash file's `# Python backtrace` named a background thread in a third-party add-on (`poliigon-addon-blender`), not device handling, and re-running the same command completed normally. One of those crash files was named `machine-test.crash.txt` — after the loaded blend-file — rather than `blender.crash.txt`.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), the whole of **Edit ‣ Preferences ‣ System** was looked at. The page held **Cycles Render Devices**, then **Network**, **Memory & Limits**, **Video Sequencer** and a collapsed **Sound** — and nothing else. **There was no Display Graphics section, and no graphics backend or device control anywhere on the page**, in any of the four versions. So on this machine there is no choice of drawing card outside the Cycles list. A test aimed at the Vulkan device list was mis-aimed and is recorded as such: Vulkan is not something a Mac offers, so the question assumed a control that does not exist on this operating system. Windows and Linux stay untested.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-5.0.1-linux-x64/blender`. Its settings folder started empty, so this version began from factory settings rather than from anything a person had set. One thing about that folder, said plainly rather than left out. The first runs were meant to point Blender at a folder of this topic's own with `BLENDER_USER_RESOURCES`, and a mistake in the shell meant the variable was never actually set, so those runs used Blender's ordinary default folder instead. That folder was also empty when this work began, so "began from factory settings" holds either way. Every test that turns on saved preferences was then run again with the variable properly set, and every result came back identical. Both runs are described in `Audits/Blender Research/The machine/Hand tests — Linux.md`.

**What Blender finds, and how the one card is listed.** With an untouched settings folder:

```
CYCLES KEYS: ['cycles']
BACKEND: NONE
BEFORE: []
AFTER: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True)]
```

The add-on key is `cycles` here too. The technology starts at `NONE` on this machine, where it starts at `METAL` on the Mac — exactly as Cycles' own `default_device` says. Every graphics entry starts ticked and the processor unticked. And **one physical card is listed twice**, once under CUDA and once under OptiX, with two stored identifiers, each carrying the card's place on the bus.

With preferences already saved, `BEFORE` came back already populated, and `--factory-startup` emptied it again — the same pattern as the Mac. `has_active_device()` returned `True`, `get_num_gpu_devices()` returned `1`, `has_multi_device()` returned `False`.

**The saved switches are honoured, and the flag displaces them.** With the technology saved as `OPTIX` and every device switched off, a background render exited `0`, wrote its image and reported the processor. The same render with `-- --cycles-device OPTIX` appended reported `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]`. With the devices back on and the technology set to `NONE`, the processor again, and no `Added device` line at all.

**The flag also displaces the scene's own setting.** A copy of the scene saved with its Device set to `CPU`, rendered with `-- --cycles-device OPTIX`, reported the card.

**`+CPU` really does use both.** With `-- --cycles-device CUDA+CPU`, exit `0`, image written, and the report named two devices:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                            |                  AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (11 threads) [CPU]
```

Eleven threads for the processor there and twelve when it works alone.

**A change made by a script inside the render's own run — and here the two machines disagree.** Three runs, each changing the preferences and then rendering in the same command, never saving them. Every device off:

```
SET TO: [('NVIDIA GeForce RTX 4090', False), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', False), ('NVIDIA GeForce RTX 4090', False)]
                            | Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
```

Every device on:

```
SET TO: [('NVIDIA GeForce RTX 4090', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', True), ('NVIDIA GeForce RTX 4090', True)]
                            | Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
                            |                  AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (11 threads) [CPU]
```

The technology set to `NONE`: `BACKEND NOW: NONE`, and the processor. All three exited `0` and wrote an image. On the rented Linux machine the change made inside the run **took effect** — the report named exactly the devices the script had asked for. On the Creator's Mac the same three runs carried no graphics-card marker at all. Both stand; neither is chosen between here.

**A control over which card draws Blender's interface — it exists on Linux.** Read with no window, `bpy.context.preferences.system` gave `GPU_BACKEND NOW: OPENGL | CHOICES: ['OPENGL', 'METAL', 'VULKAN']` and `GPU_PREFERRED_DEVICE NOW: AUTO | CHOICES: ['AUTO']`. With a window open, the System page carries a **Display Graphics** section — which the Mac has not got at all — holding a **Backend** dropdown reading `OpenGL`. With the backend saved as `VULKAN` and Blender restarted, a **Device** control appears beneath it reading `Auto`, with a note this version and 4.5 carry and 5.1 and 5.2 do not:

```
Current Vulkan backend limitations:
  • Low performance in VR
```

The Device list offered only `Auto`, and the property confirms it: `gpu_preferred_device` held `['AUTO']` and nothing else, even with Vulkan actually running. That is one card, not a claim about machines with several. Changing the backend without restarting shows `A restart of Blender is required`.

The Linux System page also carries an **Operating System Settings** section — `Open blend files with this Blender version`, with `Register` and `Unregister` buttons and a `For All Users` box — which the Mac has not got.

**One command-line argument this version has and 4.5 has not.** Blender's own `--help` on this machine lists, in 5.0, 5.1 and 5.2 and not in 4.5:

```
--no-window-frame 
	Disable all window decorations (Linux only).
```

There is no `--gpu-device` argument in this version; 5.2 has one.

**Not covered.** Nothing in the sources says how work is divided between several enabled cards, whether they are given equal shares, or what happens when they are of unequal speed, and no machine with two cards exists. Nothing says whether `--cycles-device` is remembered or affects the saved preferences. The compositor and the denoiser have their own separate device settings ("Denoising and Compositing have their own device selection", https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html) and this answer does not cover those — though every graphics-card render on the rented Linux machine reported denoising on the processor. Two things this list once held are now answered: what happens when the chosen backend has no usable card at render time (it moves to the processor silently), and whether the `use` switches can be changed without the preferences window and take effect in the same session (on Linux they did; on the Mac they did not).

**Outside the interface.** The backend and processor participation can be set by argument. **Inferred:** the saved per-card switches can be supplied through a preferences file placed with an environment variable or a `portable` folder — and that inference is now observed on the rented Linux machine. Pointed at a folder of its own with `BLENDER_USER_RESOURCES`, Blender reported that folder as its preferences path, wrote a `userpref.blend` into it, read the saved technology and switches back from it on the next run, and rendered accordingly. This is now tested on two machines and they agree on every part but one. The add-on key is the literal string `cycles` on both. The preferences object was read and written through `--python-expr` with no window on both. `bpy.ops.wm.save_userpref()` works with no window on both. A saved preferences file **is** read by a windowless render on both. `--cycles-device` displaced both the saved switches and the scene's own Device setting on both. The one disagreement is the same-run change, recorded above. What no test could reach on either machine is a computer with more than one card.

**Operating system.** The backend list differs by system (see question 2), and the default backend differs on Apple Silicon — both halves now observed, `METAL` on the Mac and `NONE` on the rented Linux machine. The preferences file location differs by system (quoted above). On Linux the System page carries a Display Graphics section and an Operating System Settings section that macOS has not, and it carries neither the Kernel Optimization nor the MetalRT control. EEVEE's single-card limitation is stated without an operating system and could not be tested. Its headless limitation is stated for Windows only, and macOS and Linux are now both tested and both render.

### Hand tests on the two-card machine — observed
**Read this block against the *Not covered* paragraph just above it.** That paragraph was written before any two-card machine existed, so some of what it calls untested is answered here — how work is divided between two ticked cards, and the several-card form of the `--cycles-device` question above all. What it says about an NVLink pair, about two cards of different makes, about a slow card beside a fast one, and about anything needing a window still stands. *(Pointer added 2026-08-06 by the citation check of the two-card material.)*

Run on **2026-08-06** on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-5.0.1-linux-x64/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`. Label: **observed**.

**This is a different machine from the one-card rented Linux machine above, and nothing here is merged with that machine's results or with the Creator's Mac.** Its settings folder was `/root/smoozy/two cards/res-5.0`, empty when this work began, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender every time. **Nothing else was running on the machine**, which is why the timings are quoted at all. The full list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`.

**The two cards are not joined by NVLink.** `nvidia-smi topo -m` reports `PHB` between them — `Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)` — so this machine still cannot test an NVLink pair.

**Both cards are switched on at factory settings, and one card is listed twice.** With an untouched settings folder, `--factory-startup`, and a script that calls `refresh_devices()` and then prints every device as `(name, type, id, use)`:

```
PREFS PATH: /root/smoozy/two cards/res-5.0
BACKEND: NONE
BEFORE: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a', True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX', True)]
```

The label `BEFORE` is the script's own word and is misleading — the refresh had already happened. Five entries for two cards: each card once under CUDA and once under OptiX, plus the processor. Every graphics entry starts ticked.

**A render uses both, without being asked to.** With the technology saved as `OPTIX` and both cards ticked, a windowless render of a Cycles scene at 1920 by 1080 and 6000 samples named both cards, one to a line:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                 NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
```

The card-by-card watch was run in 4.5 rather than here, and it showed both cards between 91 % and 98 % for the whole render and both holding the same 2683 MiB.

**Two cards against one, on the same scene and the same samples.** Nine renders in each of three states, one after another, never overlapping, on a machine with nothing else running. This version's `Finished` line carries a timestamp counting from Blender's start rather than a render time, so the same scene was rendered twice more in each of two states afterwards with the `Time:` summary line captured. Both sets, unedited:

| | two cards | the card at `0000:00:07` alone | the card at `0000:00:0a` alone |
|---|---|---|---|
| timestamp on `Finished` | `00:29.243`, `00:28.607`, `00:28.301` | `00:52.871`, `00:52.861`, `00:52.657` | `00:52.776`, `00:53.326`, `00:53.176` |
| the `Time:` summary line | `Time: 00:30.21 (Saving: 00:00.25)`, `Time: 00:31.31 (Saving: 00:00.32)` | `Time: 00:55.07 (Saving: 00:00.24)`, `Time: 00:55.10 (Saving: 00:00.26)` | not run in this form |

The two cards are the same model and either alone gave the same time as the other alone.

**Distribute Memory Across Devices, on a pair that is not linked.** The setting exists in this version, is called `peer_memory`, is a yes-or-no setting, and starts off:

```
peer_memory                      BOOLEAN    default=False        name='Distribute memory across devices'
      description='Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it'
```

It can be turned on and saved even though the cards are not linked — `SAVED STATE: OPTIX peer_memory= True …` — and a separate later run printed `PEER READ BACK: True`. A render with it on exited 0 and used both cards, and **Blender said nothing about the setting at all**: searching both streams of the run for `peer`, `nvlink`, `interconnect` and `distribut` matched nothing but the output file's own name. The card-by-card memory measurement was made in 4.5 and is recorded there: with the setting on and with it off, each card reached the same 3557 MiB on the same scene, so the scene was still put on both cards in full.

**Switching one card off, and what the flag then does.** Three saved states, each rendered with `--debug-cycles`. With only the card at `0000:00:07` ticked and no flag, exit 0 and one card named:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
```

With the same one card ticked and `-- --cycles-device OPTIX`, exit 0 and **both** named. With **every** device unticked and the same flag, exit 0 and both named again:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                 NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
```

So the flag does not merely re-enable the one card the preferences switched off — **it takes every card of that technology**, whatever the preferences said.

**What the memory readouts say with two cards.** Blender prints one figure, never two, and never names a card beside a memory number. On the timing scene its `Finished` line read `Mem: 9M` with two cards and `Mem: 1084M` with one — **smaller with two cards, not larger and not doubled**. What that field counts is not settled here and nothing is read into it.

**Frame order and output.** A four-frame animation was rendered with both cards and then with one. The order was ascending both ways:

```
00:04.319  render           | Saved: '/root/smoozy/two cards/work/ord_5.0/f_0001.png'
00:05.207  render           | Saved: '/root/smoozy/two cards/work/ord_5.0/f_0002.png'
00:06.064  render           | Saved: '/root/smoozy/two cards/work/ord_5.0/f_0003.png'
00:06.942  render           | Saved: '/root/smoozy/two cards/work/ord_5.0/f_0004.png'
```

The files were not byte-identical — but neither were two runs on the same cards, and Blender stamps a date into the PNG, so bytes cannot answer this alone. Comparing the pixels instead, of 921,600 values: **11 differed between two cards and one, 9 between two runs on two cards, and 12 between two runs on one card**, and the largest difference in every one was `0.003921568393707275` — one step in 255.

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

**Not covered.** Whether cards of two different makes can be ticked at once — both cards here are the same NVIDIA model. Whether a slow card holds a fast one back. What Distribute Memory Across Devices does when the cards **are** joined by NVLink. Whether that setting's box is drawn, greyed or hidden on an unlinked pair, and what the Vulkan **Device** control offers with two cards — both need a window, and this machine has no display server. Taking a card away mid-render was tried in 4.5 only and is recorded there.

**Outside the interface.** Everything above was driven from a terminal with no window and no display server. The sharp new point is that **the individual cards are still only reachable through the saved preferences or Python**, and that `--cycles-device` is all-or-nothing: it takes every card of its technology rather than the ones that were ticked.

**Operating system.** Linux only, as with the one-card machine, which this second machine agrees with on everything the first could reach. macOS was never asked any of the two-card questions, the Creator's Mac having one set of graphics. Windows is untested.

---

## 6. Can Blender be asked which graphics cards it can see and can use?

**Label: documented** for the two published ways; **observed** for the richer machinery in Cycles' own files.

Yes.

### The published way: Save System Info

> "Save System Info
>
> This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems.
>
> You will be prompted to save a text file called system-info.txt."

and among the sections it lists:

> "GPU
>
> Shows the GPU vendor, version and the capabilities of your hardware and driver."

> "Cycles
>
> The instruction sets and capabilities of each hardware render device available for use with Cycles."

— Manual, https://docs.blender.org/manual/en/5.0/getting_started/help.html, read 2026-07-30.

The troubleshooting pages point at the same thing:

> "To find out which graphics card and driver Blender is using, use Help ‣ Save System Info inside Blender. The OpenGL section will have information about your graphics card, vendor and driver version."

— Manual, https://docs.blender.org/manual/en/5.0/troubleshooting/gpu/windows/nvidia.html, read 2026-07-30.

It has an operator, so it is reachable from a script:

> "bpy.ops.wm.sysinfo(*, filepath='')
>
> Generate system information, saved into a text file"

— API documentation, https://docs.blender.org/api/5.0/bpy.ops.wm.html, read 2026-07-30.

### The published way: the log

> "`00:01.041  cycles           | Added device "Apple M3 Max (GPU - 40 cores)" with id "METAL_Apple M3 Max (GPU - 40 cores)".`"

— Release notes, https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30, in the example output for the new logging system. The manual's description of the `info` level is "Information about devices, files, configuration, operations", and `--log "render,cycles"` is given as an example of selecting categories (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30).

### The display card, through Python

> "gpu.platform.backend_type_get() — Get actuve GPU backend. Returns: Backend type ('OPENGL', 'VULKAN', 'METAL', 'NONE', 'UNKNOWN')."
>
> "gpu.platform.device_type_get() — Get GPU device type. Returns: Device type ('APPLE', 'NVIDIA', 'AMD', 'INTEL', 'SOFTWARE', 'QUALCOMM', 'UNKNOWN')."
>
> "gpu.platform.renderer_get() — Get GPU to be used for rendering. Returns: GPU name."
>
> "gpu.platform.vendor_get() — Get GPU vendor."
>
> "gpu.platform.version_get() — Get GPU driver version."

— API documentation, https://docs.blender.org/api/5.0/gpu.platform.html, read 2026-07-30. This is the card Blender draws with, not the list of cards Cycles could render on. The two need not be the same card.

### A Cycles device list for a requested type, in Cycles' own files — **observed**

> ```
>     # Be careful when deciding when to call this function,
>     # as Blender can crash with `_cycles.available_devices()` on some drivers.
>     def get_device_list(self, compute_device_type):
>         import _cycles
>         device_list = _cycles.available_devices(compute_device_type)
> ```

> ```
>     # Gets all devices types to display in the preferences for a compute device type.
>     # This includes the CPU device.
>     def get_devices_for_type(self, compute_device_type, device_list=None):
>         # Layout of the device tuples: (Name, Type, Persistent ID)
> ```

> ```
>     # Refresh device list. This does not happen automatically on Blender
>     # startup due to unstable drivers that can cause crashes.
>     def refresh_devices(self):
> ```

> ```
>     # Deprecated: use refresh_devices instead.
>     def get_devices(self, compute_device_type=''):
>         self.refresh_devices()
>         return None
> ```

— File `Blender Sources/cycles_5.0/properties.py`, lines 1651–1721. Read 2026-07-30.

Alongside these, the same class carries `get_num_gpu_devices` (line 1728), `has_multi_device` (1741), `has_active_device` (1753, `return self.get_num_gpu_devices() > 0`), `has_oidn_gpu_devices` (1756) and `has_optixdenoiser_gpu_devices` (1772), and the add-on's engine module exposes the raw call — **observed**:

> ```
> def available_devices():
>     import _cycles
>     return _cycles.available_devices()
> ```

— File `Blender Sources/cycles_5.0/engine.py`, lines 152–154. Read 2026-07-30.

What each entry in the list carries can be read off the code that consumes it: index 0 name, 1 type, 2 persistent id (stated in the comment quoted above), 3 used for `has_peer_memory` (line 1900), 4 for hardware ray tracing (1902), 5 for `has_device_oidn_support` (1766), 6 for `has_device_optixdenoiser_support` (1782), 7 for `is_optimized` (1697) — all in `Blender Sources/cycles_5.0/properties.py`. Read 2026-07-30.

"Can see" and "can use" are separate here: `_cycles.available_devices(compute_device_type)` supplies entries for the requested type; `CyclesDeviceSettings.use` (quoted under question 5) is the saved switch named "Use device for rendering", and `self.devices` is the saved collection of entries.

### Where the sources are silent

None of this Cycles machinery appears in the published API documentation. **Where I looked:** I listed every file in `api_5.0` matching "cycles" (only `bpy.ops.cycles.txt` and `bpy.types.FModifierCycles.txt` exist — there is no `bpy.types.CyclesPreferences` page). I searched the whole of `api_5.0` and `manual_5.0` for `get_devices`, `compute_device_type`, `CyclesPreferences`, `available_devices`, `device list`, `list of devices`, `detected device`, `enumerate.*device`. The only hits in the API documentation were `bpy.types.PreferencesSystem` (which has `gpu_preferred_device` — "Preferred device to select during detection (requires restarting Blender for changes to take effect)", enum currently listing only `AUTO`, and `legacy_compute_device_type` — "For backwards compatibility only", read-only), and the `genindex` files. I also searched the release notes for `refresh_devices`, `get_devices`, `available_devices`, `compute_device` and found nothing. So the answer "yes" rests on Save System Info, the log, `gpu.platform`, and Cycles' own files — not on a documented device-listing API.

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 5.0.

**What `system-info.txt` says about cards.** **Help ‣ Save System Info** was used to write the file. Its section headed `Cycles:` read, in full:

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

followed by a section headed `Implementation Dependent GPU Limits:` reading `Maximum Batch Vertices:	-2147483648` and `Maximum Batch Indices:	-2147483648`.**The whole file, and how the two routes differ.** The file written from the Help menu has been read end to end. Its section headings, in order, are: `Blender:`, `Python:`, `paths:`, `Python (External Binary):`, `Directories:`, `scripts:`, `pref scripts:`, `FFmpeg:`, `Other Libraries:`, `GPU:`, `extensions:`, `Implementation Dependent GPU Limits:`, `GLSL:`, `Cycles:` (with `Metal device capabilities:` inside it), and `Enabled add-ons:`. **There is no section headed `OpenGL`**, on this machine, in this version.

After the `GPU:` block quoted above the file continues:

```
Implementation Dependent GPU Limits:
=========================================

Maximum Batch Vertices:	-2147483648
Maximum Batch Indices:	-2147483648

GLSL:
Maximum Varying Floats:	60
Maximum Vertex Attributes:	31
Maximum Vertex Uniform Components:	1024
Maximum Fragment Uniform Components:	1024
Maximum Vertex Image Units:	128
Maximum Fragment Image Units:	128
Maximum Pipeline Image Units:	128
Maximum Image Units:	128
```

The same file written with no window open drops all of that. There the `GPU:` heading is replaced by the single line `GPU: missing, background mode`, and the next heading is `Cycles:` — no `extensions:`, no `Implementation Dependent GPU Limits:`, no `GLSL:`. The `Cycles:` section is identical between the two routes. So the report answers question 6 for Cycles' render devices either way, and only for the drawing card does it need the window.

**Asking Cycles directly, from Blender's Python Console** (Scripting workspace, window open). The add-on key was found to be the literal string `cycles`. With `p = bpy.context.preferences.addons["cycles"].preferences`, `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned `[]` before anything else was called, and after `p.refresh_devices()` returned:

```
[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

`p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, and `p.has_multi_device()` returned `False`. So on this machine the identifier called "Unique identifier of the device" was the string `CPU` for the processor and `METAL_Apple M1 Pro (GPU - 14 cores)` for the graphics card. One run records nothing about whether those identifiers stay the same across restarts, driver changes or machines.

**`gpu.platform`, from the same console.** `backend_type_get()` returned `'METAL'`; `device_type_get()` returned `'APPLE'`; `renderer_get()` returned `'Metal API'`; `vendor_get()` returned `'Apple M1 Pro'`; `version_get()` returned `'1.2'`. `gpu.platform.devices_get()` raised:

```
AttributeError: module 'gpu.platform' has no attribute 'devices_get'
```

so that function is not present in 5.0.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender` (`Blender 5.0.1`).

**Save System Info with no window open.** `-b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='/tmp/sysinfo-terminal-50.txt'))"` printed `RESULT: {'FINISHED'}` and wrote the file. Its `GPU` section was one line:

```
GPU: missing, background mode
```

Its `Cycles` section, in full:

```
Cycles:
====================================


CPU device capabilities: 

Metal device capabilities:
Number of devices: 1
		Device: Apple M1 Pro (GPU - 14 cores)
```

So the operator works with no window, the Cycles section is the same as the menu route's, and the GPU section is not.

**`gpu.platform` with no window open.** `-b --factory-startup --python-expr "import gpu; print('INIT:', gpu.init())"` raised:

```
AttributeError: module 'gpu' has no attribute 'init'
```

and calling the platform functions without it raised:

```
SystemError: GPU functions for drawing are not available in background mode
```

So in 5.0 there is no `gpu.init`, and `gpu.platform` does not answer with no window.

**The Cycles queries with no window open** answered — full output under question 5. The add-on key printed as `['cycles']`; the stored list was already populated before `refresh_devices()` was called; each raw entry was an eight-field tuple; and `_cycles.available_devices()` with no argument raised `TypeError: function takes exactly 1 argument (0 given)`. Identifiers were identical in every run of the day.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-5.0.1-linux-x64/blender`.

**Save System Info with no window open.** `-b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='…'))"` printed `RESULT: {'FINISHED'}` and wrote the file. Its whole `GPU` section is one line, as on the Mac:

```
GPU: missing, background mode
```

Its `Cycles` section is where Linux differs, and it is long — about ninety lines of card capabilities. Its opening and the two lines that carry the answer:

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

So the report names the card's compute capability outright, and the processor's instruction set (`AVX2`) where the Mac's line was empty. Only the technology whose cards were found gets a section: with the technology saved as OptiX the file still headed the section `CUDA device capabilities`. The headings in the file, in order, were `Blender:`, `Python:`, `paths:`, `Python (External Binary):`, `Directories:`, `scripts:`, `pref scripts:`, `FFmpeg:`, `Other Libraries:`, `GPU: missing, background mode`, `Cycles:`, `Enabled add-ons:` — and, as on the Mac, there is **no section headed `OpenGL`**.

**Asking Cycles' compiled part directly — including for a machine with no supported card, which the Mac could not do.** `-b --factory-startup --python-expr "import _cycles; …"` printed:

```
TYPES: (True, True, True, False, True, False)
CUDA: (('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', False, False, True, False, True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True))
OPTIX: (('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', False, False, True, True, True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True))
HIP: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
ONEAPI: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
METAL: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
```

and the no-argument form was refused, as on the Mac:

```
TypeError: function takes exactly 1 argument (0 given)
```

So `available_devices` for a technology with no supported card returns **the processor alone, and no error** — whether that technology is built in (HIP, oneAPI) or not (Metal). Each entry is an eight-field record. And the call did not crash Blender on either machine, despite the warning in Cycles' own comments; that is not the same as the warning being stale.

**What Blender was built with, against what it can find.** Hiding the card for one command with `CUDA_VISIBLE_DEVICES=` left `TYPES:` unchanged while both card lists came back holding the processor alone, and a render then asked for CUDA exited `1` with `ERROR Found no Cycles device of the specified type`. So Blender answers *what am I built with* and *what can I see now* separately.

**The graphics module with no window open.** `-b --python-expr "import gpu; print('INIT:', gpu.init())"` raised:

```
AttributeError: module 'gpu' has no attribute 'init'
```

and the platform calls raised:

```
SystemError: GPU functions for drawing are not available in background mode
```

Word for word what the Mac gave. So `gpu.init` is absent and `gpu.platform` refuses with no window, on both machines, in this version.

**Not covered.** Whether the stored identifier is stable across restarts, driver updates or reordering of cards — it held steady across every run of the day on both machines, and on Linux it visibly carries the card's place on the bus, which is a hint and not a statement; a driver update and a move are untested. Whether asking for the list has a side effect on which devices are enabled. What any of this returns on a machine with several cards.

**Outside the interface.** Save System Info has an operator, `bpy.ops.wm.sysinfo(filepath='')`, and the API documentation shows the general pattern for driving Blender by argument (`-P`, `--python-expr`, `-c`). The log route is entirely a command-line one (`--log`, `--log-level`, `--log-file`). The sources do not state that `wm.sysinfo` works in background mode, only that it is an operator with a filepath, so I am not claiming it. The Cycles calls quoted are ordinary Python and carry no stated dependence on a window, but the warning that the device list is not refreshed at startup and can crash on some drivers is the one thing to weigh before relying on them unattended. Both are now tested with no window open on one machine. `bpy.ops.wm.sysinfo(filepath=…)` returned `{'FINISHED'}` and wrote its file, whose `Cycles` section matched the menu route's while its `GPU` section read only `GPU: missing, background mode`. The Cycles queries answered, and there the stored list was already populated before `refresh_devices()` was called — the opposite of what the same reading showed inside the window. `gpu.platform` does **not** answer with no window in this version: it raised `SystemError: GPU functions for drawing are not available in background mode`, and `gpu.init` does not exist here.

**Operating system.** No operating-system limit is stated for the published calls or Save System Info. The `gpu.platform` return values name Apple, NVIDIA, AMD, Intel, Qualcomm and a software renderer, but that list alone does not establish the result on every operating system. The Cycles device list is requested by backend, and the documented available backends differ by system (question 2). Two systems are now tested. On the Creator's Mac all the routes answered, `gpu.platform` reporting backend `'METAL'` and device type `'APPLE'`. On the rented Linux machine the same routes answered, the report's Cycles section carrying a long CUDA capability dump instead of the Mac's short Metal one, and `gpu.platform.devices_get` did not exist on either. Windows remains untested.

### Hand tests on the two-card machine — observed

Run on **2026-08-06** on the second rented Linux machine described under question 5 — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s on driver 580.95.05, sixty-one cores, **no display server at all**, over SSH — against `/opt/blender/blender-5.0.1-linux-x64/blender`. Label: **observed**. Not merged with the one-card rented machine's results or the Mac's.

**Every route answered, and every one named both cards.** Asked with no window and no display server:

```
BACKEND: OPTIX
TYPES: (True, True, True, False, True, False)
CUDA: (('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', False, False, True, False, True), ('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a', False, False, True, False, True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False, False, True, False, True))
OPTIX: (('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', False, False, True, True, True), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX', False, False, True, True, True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False, False, True, False, True))
HAS ACTIVE: True
NUM GPU: 2
MULTI: False
```

Three things in that. Each technology's list holds **both** cards and the processor. **`get_num_gpu_devices()` counts the cards and returns 2.** And **`has_multi_device()` returns `False` with two cards ticked** — so that name is not about having more than one card, whatever it reads like.

**The system-information file lists the card twice.** `bpy.ops.wm.sysinfo(filepath=…)` returned `{'FINISHED'}` and wrote its file with no display server. Its `Cycles` section carries `CPU device capabilities: AVX2`, one heading `CUDA device capabilities:`, and then the card's name twice — once at line 83 of the file and once at line 172 — each followed by its own block of capabilities.

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

**Not covered.** What any of this returns on a machine with cards of two different makes, or with an NVLink pair. What the Status Bar or any panel shows, this machine having no display server at all. `--gpu-device help`, which this version has not got; 5.2 has it and it lists both cards there.

**Outside the interface.** Settled again, on a machine with two cards and no display server: `wm.sysinfo` wrote its file, the Cycles queries answered, and both cards were named by every route. Nothing needed a window.

**Operating system.** Linux, as before. This second machine agrees with the one-card machine wherever the two could be asked the same thing, and adds only what a second card shows. Windows is untested.

---

## Things a person with Blender 5.0 open could settle by hand in a few minutes

This list has since been run on three machines. What the Creator's own MacBook Pro settled is written into the answers above as **Hand test — observed**, what the one-card rented Linux machine settled as **Hand tests on the rented Linux machine — observed**, and what a second rented Linux machine with two graphics cards and no display server settled as **Hand tests on the two-card machine — observed**. The three lists live at `Audits/Blender Research/The machine/Hand tests.md`, `Audits/Blender Research/The machine/Hand tests — Linux.md` and `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`. **A fourth machine has since been used as well** — a third rented Linux machine with **four** graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the four-card machine — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

### Still open

Nothing on this list is still open. Every item either machine could reach has been run, and what came back is recorded under questions 1, 2, 3, 4, 5 and 6 above. The item that stood after the Mac run — what the preferences panel says on a machine with no usable card — is now closed by the Linux machine for HIP and oneAPI, and is recorded under question 2.

### Cannot be run on the equipment available

The equipment is one MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, whose graphics are Apple's own and built in; one rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores — reached over SSH; and a second rented Linux machine — Ubuntu 22.04.5 LTS, **two** NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, **no display server at all** — also over SSH. No Windows machine has run it, there is no AMD or Intel graphics card and no pair of cards joined by NVLink, and nothing on either rented machine may be installed or removed. **Two corrections to that sentence, made 2026-08-06 by the citation check of the two-card material.** The one-card rented Linux machine was destroyed on 2026-08-06 and nothing can ever be run on it again, so every result from it stands as a record and none of it can be repeated. And something *was* removed and reinstalled on the two-card machine: its NVIDIA driver was purged on purpose, and then reinstalled into a mismatched state, for the broken-driver test recorded under question 2. That machine has since been restarted and did not answer when the citation check tried it on 2026-08-06.

**A fourth machine has since been used, and it is the only one still alive.** A **third rented Linux machine** — Ubuntu 22.04.5 LTS, **four** NVIDIA GeForce RTX 4060 Ti of **16 GB each** on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, **197 GB of memory**, **no display server**, over SSH. What it settled is written into the answers above as **Hand tests on the four-card machine — observed**, and its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. Three limits above are narrowed by it and none is removed: more than two cards is no longer out of reach, a card small enough to be filled is no longer out of reach, and the several-card form of every question under 5 and 6 has now been asked twice on two different machines. **Still out of reach on it as well:** any pair joined by NVLink, these four talking through the host bridge; cards of two different makes; cards of unequal speed, all four being the same model within about 2 % of each other; and anything needing eyes on a window, this machine having no display server either. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

- **The Metal minimum, on the two readings.** Needs an Apple Silicon Mac running macOS 12.x. On macOS 26.5.2 a Metal device is found and no requirement line is shown, and the Linux machine offers no Metal button at all.
- **Whether the Intel and AMD figures the panel draws are enforced.** What it draws on Linux is now recorded under question 2. Testing enforcement needs a real Intel or AMD card with a driver below the figure.
- **The Intel Windows driver figure.** Needs a Windows machine; the panel text branches on the operating system.
- **Whether the compute-capability floor of 5.0 is enforced.** Needs a card between 3.0 and 5.0. This machine's card reports major 8, minor 9.
- **Cards of two different makes ticked at once.** The two-card machine has two of the same NVIDIA model. Most of what needed a second card is now answered under questions 5 and 6; this is not.
- **What Distribute Memory Across Devices does when the cards are joined by NVLink.** The two-card machine's pair talks through the host bridge. What the setting does on an unlinked pair is now recorded under question 5.
- **Whether a slow card holds a fast one back.** Both cards on the two-card machine are the same model and gave the same time as each other.
- **Anything needing eyes on a window with two cards** — whether the Distribute Memory Across Devices box is drawn or greyed, and what the Vulkan **Device** control offers. The machine that has two cards has no display server; the machine that had a display had one card.
- **Whether EEVEE really cannot render headless on Windows.** macOS and Linux are now both tested and it renders on both.

---

2026-08-01 — Claude Opus 5 (Claude Code) — The machine, Blender 5.0

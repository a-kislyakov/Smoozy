# The machine — Blender 4.5

**Sources consulted, and when.** Blender 4.5 manual (`manual_4.5`, including one image from `_images`), Blender 4.5 Python API documentation (`api_4.5`), Blender 4.5 release notes (`release_notes_4.5`), and Cycles' own files (`cycles_4.5`). All read on **2026-07-30** (the snapshot date given to me).

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

A note on one thing I did open, so it is on the record: the folder `Blender Sources/` also holds a handful of loose text files at its top level (`crashes.txt`, `cycles_device.txt`, `devices_api.txt`, `eevee_render.txt`, `eevee_results.txt`, `missing.txt`, `missing_filtered.txt`) and a short note called `About this folder.md`. I glanced at them because they sit inside the folder I was told I may read. They turned out to be saved search results from other people's searches, almost all of them against Blender 4.5's *later* versions, not 4.5. I took nothing from them. `About this folder.md` links to a method document elsewhere in the project; I did not follow that link.

---

## 1. What must a computer have on it before Blender will run at all?

**Label: documented (in part) and not in the sources (in part).**

The manual states plainly that there *is* a list of minimum requirements, but it does not contain the list. It sends the reader to a page on blender.org that is not part of my sources.

> "Blender is available for download on Windows, macOS, and Linux. Always check that your graphics drivers are up to date and that OpenGL is properly supported. Blender has a set of minimum and recommended requirements; so make sure these are met before trying to install Blender."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30

Each of the three install pages repeats the redirection. The Linux page's wording, identical on the Windows and macOS pages:

> "Check the Downloading Blender page to find the minimum requirements and the different versions that are available for Blender (if you have not done so yet)."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30
(also https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html and https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html)

So **no figure for memory, disk space, processor model or processor instruction set appears anywhere in my sources.** What the sources *do* state:

**A graphics card and driver providing OpenGL 3.3 or better.** This is stated as the meaning of a named error:

> "This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

— Manual, https://docs.blender.org/manual/en/4.5/troubleshooting/gpu/common/other.html — read 2026-07-30
(the same sentence is repeated verbatim on the per-vendor, per-system pages under `troubleshooting/gpu/`)

**A processor architecture matching the download.**

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html — read 2026-07-30

**On Linux, a windowing system — X11 or Wayland — when Blender opens a window.**

> "On Linux Blender supports both X11 and Wayland for official releases."
> "When Wayland is detected, it is the preferred system, otherwise X11 will be used."
> "Under Gnome-Shell the libdecor library is required. This is available as a package on most Linux distribution."
> "If the library isn't found X11 will be used as a fallback."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux_windowing_environment.html — read 2026-07-30

**No separate system Python is needed for the program's own scripting by default.** Blender carries its own Python:

> "Blender bundles a complete Python installation and does not interact with the system Python by default. This can be changed with the --python-use-system-env command line argument, if care is taken to set up a compatible Python version."

— Manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30

**No internet connection is needed.**

> "Blender is designed to not require an internet connection, so it doesn't have a built-in update system."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30

**Two non-hardware conditions,** from the page on Blender failing to start:

> "Make sure any antivirus software is not preventing Blender from starting."
> "Ensure that you have appropriate user permissions on the computer."

— Manual, https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html — read 2026-07-30

**Where I looked, to justify the "not in the sources" half.** I searched the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` for: `requirement`, `minimum requirements`, `system requirements`, `recommended requirements`, `64-bit`, `SSE2`, `SSE4`, `AVX`, `RAM`, `GB of RAM`, `glibc`, `libstdc`, `libX11`, `libxkb`, `shared librar`, `prerequisite`, `dependenc`, `redistributable`, `Visual C++`, `.NET`, `runtime`. The only hits that bear on the question are the ones quoted above. There is no page in the 4.5 manual carrying a requirements table.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 4.5 was opened and **Help ‣ Save System Info** was used to write `system-info.txt`. Blender started and ran normally on that machine. The file's section headed `GPU:` read:

```
renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:
```

and was followed by a section headed `Implementation Dependent GPU Limits:` reading `Maximum Batch Vertices:	-2147483648` and `Maximum Batch Indices:	-2147483648`. Only the `GPU` and `Cycles` sections of that file were copied back, so nothing here records the rest of it — in particular, whether it carries any section naming an OpenGL version. This records what Blender reported on a machine that started successfully; it does not test the OpenGL 3.3 floor.

**Not covered.** How much memory, disk or processor Blender needs; which processor instruction sets it is built against; which minimum operating system versions it runs on (no Windows or Linux version floor appears anywhere; a macOS floor appears only for Cycles-on-Metal, see question 2); whether the OpenGL 3.3 floor still applies when the Vulkan backend is chosen instead. The manual's pointer to the "Downloading Blender page" on blender.org is where the real list lives, and I did not go there.

**Outside the interface.** Partly different, and the difference is stated. A render started from a command line needs no graphical display at all:

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30

That statement covers rendering from the command line in general. A separate source says EEVEE cannot run headless on Windows (see question 2). The sources searched for this answer do not establish whether the OpenGL 3.3 floor is enforced when Blender is started with `--background`.

Both halves are now tested on two machines. Started from outside its own window, 4.5 ran and rendered on the Creator's Mac and on the rented Linux machine, and on the Linux machine it did so with no display reachable. What still holds only inside the window is opening a window: with no display reachable, a windowed start is refused (see the Linux hand tests below). The genuine no-display-server case is a different machine and is recorded separately, further down this answer. *(Wording corrected 2026-08-06 by the citation check of the two-card material, to match the correction the runner had already made further down.)*

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender` started with `-b` and rendered without a window on every attempt of the day, both with Cycles and with EEVEE. `-b -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

and `-b /tmp/machine-test.blend -E BLENDER_EEVEE_NEXT -o /tmp/eevee2-45- -f 1` exited `0`, printing:

```
Fra:1 Mem:75.29M (Peak 75.29M) | Time:00:05.37 | Rendering 64 / 64 samples
Saved: '/tmp/eevee2-45-0001.png'
Time: 00:05.55 (Saving: 00:00.15)
```

Note the engine's name in this version is `BLENDER_EEVEE_NEXT`. This says nothing about the OpenGL floor, which this machine meets; it records that a windowless start and a windowless EEVEE render both worked on macOS.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — `/opt/blender/blender-4.5.12-linux-x64/blender` (which `--version` reports as `Blender 4.5.12 LTS`) was put to three tests bearing on this question. The machine is a container sharing somebody else's hardware, not a whole computer, and other work was running on it at the same time, so nothing here says how long anything took.

**With no display reachable, Blender started and rendered.** `DISPLAY` was removed from the environment and a background Cycles render of the test file completed at exit `0`, writing its PNG, with the graphics card doing the work:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
Denoising on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
Saved: '/root/smoozy/The machine/work/nd-4.5-0001.png'
```

*(Corrected 2026-08-06 by the runner. This originally said "with no display server at all", which the machine it was run on did not match — that machine had a real display running and the run was merely cut off from it. The two are different claims and the difference is the whole of what a later machine settled: a computer with genuinely no display server is recorded separately, on the second rented Linux machine, which has none. Nothing observed here changes; only the description of the machine does.)*

An EEVEE render did the same, also with `DISPLAY` removed, at exit `0`. So on Linux neither X11 nor Wayland is needed for a render, in this version. The manual's statement about needing X11 or Wayland is about opening a window, and the next test is that case.

**Asked for a window with no display reachable, Blender refused — and then did not stop.** The command was `env -u DISPLAY /opt/blender/blender-4.5.12-linux-x64/blender --python-expr "import bpy; print('WINDOW OPENED')"`, wrapped in `timeout 60`. No window opened, `WINDOW OPENED` was never printed, and Blender printed:

```
GHOST: failed to initialize display for back-end(s): ['WAYLAND', 'X11']
  'WAYLAND': unable to connect to display!
  'X11': unable to open a display!
GHOST: unable to initialize, exiting!
```

It then hung. The sixty-second timeout had to kill it, giving exit status `124`. That is worth recording exactly: in 4.5, on this machine, a Blender that cannot open a window says so and never returns.

**`-b -E help` also hung in this version, on this machine.** The command printed nothing at all and the sixty-second timeout killed it, exit `124`. It was tried as it stands, with `--factory-startup`, and with `DISPLAY` removed, and hung all three times. On the Creator's Mac the same command in 4.5 printed its engine listing and stopped. The two machines disagree and neither result is chosen between here. The engine names were got another way instead, from `bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items`, which printed `ENGINES: ['BLENDER_EEVEE_NEXT']` — the same name the Mac gave.

None of this tests the OpenGL floor. The rented machine reports OpenGL version `4.6.0 NVIDIA 580.105.08`, well above both figures the sources give.

**Operating system.** The sources cover Windows, macOS and Linux, and the answer differs by system on three counts: the architecture warning is worded per system; the Linux windowing requirement (X11/Wayland/libdecor) has no counterpart on Windows or macOS; and the compatibility section of the 4.5 release notes states:

> "**macOS Intel support is deprecated.** Due to the high maintenance cost of tracking down and fixing graphics related issues specific to Intel and AMD GPU Macs, it was decided that this will be the last release with an official Intel macOS build of Blender."

— Release notes, https://developer.blender.org/docs/release_notes/4.5/ — read 2026-07-30

Two of the three systems are now tested. macOS is settled by the Mac tests above and Linux by the rented-machine tests above: on Linux, 4.5 starts and renders with no display server, and refuses to open a window without one. **Observed 2026-08-06 on a second rented Linux machine that has no display server at all** — see the block below, which is the same claim made on a machine where there is nothing to be cut off from. Windows is untested, no Windows machine having run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Hand tests on a machine with no display server at all — observed.** On **2026-08-06**, on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-4.5.12-linux-x64/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`. Label: **observed**. Nothing here is merged with the one-card rented machine's results or the Creator's Mac.

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
GPU  CYCLES exit=0  png=4957
GPU  BLENDER_EEVEE_NEXT exit=0  png=3400
GPU  BLENDER_EEVEE exit=124  png=no file  Error: engine not found 'BLENDER_EEVEE'
GPU  BLENDER_WORKBENCH exit=0  png=3613
CPU  CYCLES exit=0  png=4957
CPU  BLENDER_EEVEE_NEXT exit=0  png=3400
CPU  BLENDER_EEVEE exit=124  png=no file  Error: engine not found 'BLENDER_EEVEE'
CPU  BLENDER_WORKBENCH exit=0  png=3613
```

`BLENDER_EEVEE` is not an engine this version has, and asked for it 4.5 printed its error and then did not end, killed at 120 seconds. The same runs repeated with `--debug-cycles` showed the work really was where it was asked for:

```
on the card:      Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                                   NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
on the processor: Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]
```

**Asked for a window on a machine with no display server, 4.5 said so and then did not end.** `blender --python-expr "import bpy; print('RAN WITH A WINDOW')"` under `timeout 120` never printed `RAN WITH A WINDOW`, wrote to **standard error**, and was killed at 120 seconds, exit status **124**:

```
GHOST: failed to initialize display for back-end(s): ['WAYLAND', 'X11']
  'WAYLAND': unable to connect to display!
  'X11': unable to open a display!
GHOST: unable to initialize, exiting!
```

Word for word what the one-card machine printed when cut off from its display, and the same exit status. **So for this version the two cases are indistinguishable in what Blender says and does.**

**`-b -E help` hung here too**, exit `124` at sixty seconds, but on this machine it printed its listing first:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

On the one-card machine it printed nothing before hanging; on the Creator's Mac it printed and stopped. All three results stand and none is chosen between here.

**Not covered.** Windows. Nothing about the OpenGL floor: this machine's cards are far above it and no card below the floor can be reached.

---

## 2. What must a computer have on it before Blender will render using its graphics cards?

**Label: documented, plus observed in supplied Cycles files.**

The manual's GPU Rendering page is the main answer. For the technologies listed below, it names a supported card family and, except for CUDA, a driver or operating-system floor. It separately says that users normally do not need to install the CUDA toolkit.

**How it is turned on:**

> "To enable GPU rendering, go into the Preferences ‣ System ‣ Cycles Render Devices, and select either CUDA, OptiX, HIP, oneAPI, or Metal. Next, you must configure each scene to use GPU rendering in Properties ‣ Render ‣ Device."

**NVIDIA, CUDA:**

> "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 3.0 and higher."

**NVIDIA, OptiX:**

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."
> "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards."

**AMD, HIP:**

> "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."
> "Minimum driver versions:"
> "Windows: Radeon Software 24.6.1 or Radeon PRO Software 24.Q2"
> "Linux: Radeon Software 23.40 or ROCm 6.0"
> "Hardware ray-tracing support is available with the most recent drivers. This can be enabled in the preferences, and is supported on Radeon RX 6000 and newer."
> "GPU accelerated denoising is available on discrete Radeon RX 6000 and Radeon RX 7000 GPUs."
> "Shadow caustics are not supported with HIP."

**Intel, oneAPI:**

> "oneAPI is a computation library that is supported on Windows and Linux and requires a Intel® Arc™ graphics card with the Xe HPG architecture."
> "Minimum driver versions:"
> "Windows: Intel Graphics Driver XX.X.101.5518"
> "Linux: intel-level-zero-gpu package 1.3.27642, typically available through the intel-compute-runtime package XX.XX.27642.38"

**Apple, Metal:**

> "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

**And what is *not* needed:**

> "Normally users do not need to install the CUDA toolkit as Blender comes with precompiled kernels."

All of the above — Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html — read 2026-07-30

**The release notes state one of these changed:**

> "* **Cycles:** The minimum driver version required for OptiX has increased to 535."

— Release notes, https://developer.blender.org/docs/release_notes/4.5/ — read 2026-07-30

**Observed in a supplied Cycles file, and it does not match the manual everywhere.** The file defines requirement text under the branch used when no compatible card is found. For three of the five technologies, it names different versions from the manual page above. The file alone does not establish what a running copy displays or checks.

Metal, `Blender Sources/cycles_4.5/properties.py`, lines 1849–1852:

```
            elif device_type == 'METAL':
                mac_version = "12.2"
                col.label(text=rpt_("Requires Apple Silicon with macOS %s or newer") % mac_version,
                          icon='BLANK1', translate=False)
```

oneAPI, same file, lines 1830–1848:

```
            elif device_type == 'ONEAPI':
                import sys
                if sys.platform.startswith("win"):
                    driver_version = "XX.X.101.6557"
                    col.label(text=rpt_("Requires Intel GPU with Xe-HPG architecture"), icon='BLANK1', translate=False)
                    col.label(text=rpt_("and Windows driver version %s or newer") % driver_version,
                              icon='BLANK1', translate=False)
                elif sys.platform.startswith("linux"):
                    driver_version = "XX.XX.31740.15"
```

and, further down the same branch:

```
                    col.label(text=rpt_("  - oneAPI Level-Zero Loader"), icon='BLANK1', translate=False)
```

CUDA and OptiX, same file, lines 1798–1808, agree with the manual:

```
            if device_type == 'CUDA':
                compute_capability = "3.0"
            elif device_type == 'OPTIX':
                compute_capability = "5.0"
                driver_version = "535"
```

HIP, same file, lines 1809–1829, also agrees with the manual (Adrenalin 24.6.1 / Radeon Pro 24.Q2 on Windows, 23.40 on Linux).

— `Blender Sources/cycles_4.5/properties.py`, lines 1794–1853 — read 2026-07-30

I am not judging which of the two is right. Both are 4.5 sources and they differ on Metal (macOS 12.2 against 13.0 — though the manual's wording is "to support all features", which is not quite the same claim), on the Intel Windows driver (XX.X.101.6557 against XX.X.101.5518) and on the Intel Linux package (XX.XX.31740.15 against 1.3.27642). The Cycles file also names a piece the manual does not: the **oneAPI Level-Zero Loader**.

**One prerequisite is about the copy of Blender, not the machine:**

> "For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30

**EEVEE is a different case.** EEVEE always uses the graphics card and cannot fall back to the processor:

> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient."
> "Headless rendering is not supported on headless Windows systems."

— Manual, https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html — read 2026-07-30

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 4.5's **Edit ‣ Preferences ‣ System** page was opened and the **Cycles Render Devices** panel expanded. The row of technology buttons offered exactly two: **None** and **Metal**, with Metal selected on arrival. Selecting **None** hid every other setting in that panel. With Metal selected, two tick boxes were listed: **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked. Two dropdown lists were shown as well: **Kernel Optimization**, set to **Full**, with **Off** and **Intersection Only** as its other choices; and **MetalRT**, set to **Auto**, with **Off** and **On**. Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control, and nothing else changed. No "No compatible GPUs found for Cycles" line and no "Requires…" line appeared at any point on this machine, so this run records nothing about what those lines say.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), a background Cycles render of `/tmp/machine-test.blend` (scene Device `GPU`, 16 samples) with the saved preferences as they stood exited `0` and printed `Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".` followed by `Sample 16/16 (Using optimized kernels)` at about 1558M of device memory. The full commands and output are under question 5. EEVEE also rendered with no window open on this machine — the manual's headless limitation is stated for Windows, and macOS is now tested; the engine name here is `BLENDER_EEVEE_NEXT`.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, four things were put to 4.5.

**What this copy of Blender was built with.** `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed:

```
TYPES: (True, True, True, False, True, True)
```

Reading it against Cycles' own file, which unpacks that answer as `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt`, the official Linux x64 build carries CUDA, OptiX, HIP and oneAPI and not Metal.

**A graphics-card render works, with no window and with no display reachable.** A background render of the test scene, with the technology saved as `CUDA` and the card ticked, exited `0`, wrote its PNG and printed:

```
I0806 04:26:54.288223 21179 device.cpp:212] Added device "NVIDIA GeForce RTX 4090" with id "CUDA_NVIDIA GeForce RTX 4090_0000:00:07".
```

and ended, in the report `--debug-cycles` prints:

```
Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]

Denoising on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
```

The same with the technology saved as `OPTIX` gave `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]`. Denoising ran on the processor in every graphics-card render on this machine. The `(Using optimized kernels)` text the Mac carried did not appear at all here; on Linux the report line above is what names the device.

**What the panel says when the chosen technology has no compatible card.** This is the case the Mac could never reach. Blender was started with a window on the machine's own display, the technology set from a script, and the System page photographed with Blender's own `bpy.ops.screen.screenshot`. With **oneAPI** chosen, and no Intel card in the machine, 4.5 drew:

```
No compatible GPUs found for Cycles
Requires Intel GPU with Xe-HPG architecture and
  - intel-level-zero-gpu or intel-compute-runtime version
    XX.XX.31740.15 or newer
  - oneAPI Level-Zero Loader
```

With **HIP** chosen, and no AMD card:

```
No compatible GPUs found for Cycles
Requires AMD GPU with RDNA architecture
and AMD driver version 23.40 or newer
```

So the figure 4.5 actually draws for Intel on Linux is `XX.XX.31740.15`, which is the number in Cycles' own shipped file and not the manual's `XX.XX.27642.38`. This records what the panel displays on a machine with no Intel card; it does not say what Blender would enforce against a real Intel card, and no Intel or AMD card exists among the equipment.

The requirement line for CUDA and for OptiX was never drawn, because a compatible NVIDIA card is present. The card's compute capability, as Blender's own system-information file reports it, is major `8`, minor `9` — far above both the 3.0 this version states and the 5.0 later versions state — so nothing here tells the two figures apart.

**EEVEE renders headless on Linux.** With `DISPLAY` removed from the environment, `-b <file> -E BLENDER_EEVEE_NEXT -f 1` exited `0` and wrote its PNG:

```
Fra:1 Mem:70.89M (Peak 70.92M) | Time:00:00.57 | Rendering 64 / 64 samples
Saved: '/root/smoozy/The machine/work/eeveend-4.5-0001.png'
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

*One thing this version does differently:* with the driver removed it printed **nothing at all** — no warning, no error, no sign anything was wrong. The three later versions each print one line about a library that would not open.

*On what kind of computer.* The second rented Linux machine described above. **This settles Linux on an NVIDIA machine only.** It was not run on the Creator's Mac, and Windows is untried everywhere.

**Not covered.** Whether Blender checks the driver version itself or merely fails; what happens between the two conflicting sets of Intel and Apple version numbers when a card of that make is actually present; whether an unsupported card causes a refusal to render or a silent fall back to the processor; anything about cards from vendors other than NVIDIA, AMD, Intel and Apple; the exact meaning of the placeholder "XX" in the Intel driver strings; whether AMD's ROCm and Radeon Software are alternatives or both needed on Linux — 4.5's panel names only the Radeon driver, where later versions name both.

**Outside the interface.** The hardware and driver requirements are properties of the machine and say nothing about who started Blender, so they hold either way — but note that the *first* step quoted above ("go into the Preferences") is an interface instruction, and question 5 covers what replaces it. One difference is documented and matters: EEVEE cannot render headless on Windows. This is now settled for the other half of the question on two machines: a Cycles graphics-card render and an EEVEE render both completed with no window on macOS and on Linux, and on Linux with no display reachable. On a machine that has no display server at all, the same is recorded under question 1. And the "No compatible GPUs found for Cycles" text does **not** reach a windowless run: with the technology saved as `ONEAPI`, for which this machine has no card, the phrase appeared nought times in the whole output of a background render, which completed on the processor at exit `0`. That text is drawn in the preferences panel and nowhere else.

**Operating system.** Strongly system-dependent, and this is the whole shape of the answer: CUDA, OptiX, HIP and oneAPI are stated as Windows and Linux only; Metal is stated as macOS with Apple Silicon only; driver minimums are given separately for Windows and Linux; and the EEVEE headless limitation is named for Windows alone. Two systems are now tested and they bear this out. On the Creator's Apple Silicon Mac the technology row offered **None** and **Metal** and nothing else. On the rented Linux machine it offered five buttons — **None**, **CUDA**, **OptiX**, **HIP**, **oneAPI** — and no Metal. Windows remains untested.

---

## 3. Does what a computer must have on it depend on how Blender was obtained and put there?

**Label: documented.**

Yes, on several counts — though none of them is a *hardware* difference. What changes is what else must already be on the machine, what rights are needed, and in one case whether GPU rendering is present at all.

**Whether Cycles can use the graphics cards at all** can depend on where the copy came from:

> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration. Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

**Snap needs Snap already installed:**

> "Snap is a universal package manager designed to work across a range of distributions. Assuming snap is already installed, Blender can be installed through snap with:"
> "snap install blender --classic"

**And the archive must be unpacked with the right tool:**

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead."

All three — Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30

**Administrator rights are needed for one Windows route and not the other:**

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender."
> "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30

**The Microsoft Store copy is sandboxed,** and Blender itself knows which it is. The Python API exposes a read-only flag:

> "is_microsoft_store_install"
> "Whether this blender installation is a sandboxed Microsoft Store version"
> "Type:"
> "boolean, default False, (readonly)"

— Python API, https://docs.blender.org/api/4.5/bpy.types.PreferencesSystem.html — read 2026-07-30

**Steam needs the Steam client and an account:**

> "Download and install the Steam client for your operating system."
> "Once it is installed, open the client and login to your Steam account, or create an account if you don't already have one."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/steam.html — read 2026-07-30

**macOS asks the operating system's permission the first time:**

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html — read 2026-07-30

**A copy built from source can need a piece an official copy does not.** The manual's advice for a CUDA kernel-compilation failure:

> "If you build Blender yourself, try to download and install a newer CUDA developer toolkit."
> "Normally users do not need to install the CUDA toolkit as Blender comes with precompiled kernels."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html — read 2026-07-30

**And the download must match the machine's architecture** — the two "Important" notes quoted under question 1.

**Hand test on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, the copy under test was the official Linux x64 build taken from Blender's own download server and unpacked at `/opt/blender/blender-4.5.12-linux-x64/`. Asked what it was built with, `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed `TYPES: (True, True, True, False, True, True)` — CUDA, OptiX, HIP and oneAPI present, Metal absent. So the sharpest case in the answer above, a copy that leaves Cycles graphics-card rendering out, is **not** what the official Linux download does; that copy carries all four of the technologies its operating system supports. A distribution's own package is a different copy and is untested, because nothing may be installed on this machine.

**Not covered.** Whether the memory/disk/processor minimums differ by route — no source states any such figure at all, so no comparison is possible. Whether the Microsoft Store sandbox restricts anything relevant to rendering. Whether the Snap or Steam copies build Cycles with GPU support. Whether a distribution's own package leaves graphics-card rendering out. Whether the Flatpak route exists — it is not named in the manual.

**Outside the interface.** The route also changes *where the program file is* and therefore what a script must invoke, but my sources do not spell that out beyond one Windows example path (`C:\Program Files\Blender Foundation\Blender\blender.exe`, at https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html). The Steam page notes that on Linux and Windows the Steam copy does not register the blend-file association automatically. Nothing in the sources says a route prevents command-line use.

**Operating system.** Every difference listed above is named for a specific system: distribution packages, Snap and 7-zip are Linux; administrator rights and the Microsoft Store are Windows; the approval prompt is macOS; Steam is all three.

---

## 4. Does Blender say anything about what is missing when it will not start, or when it will not render?

**Label: documented, plus observed in supplied Cycles files.**

Yes, in several forms, but the coverage is patchy: some messages are named and their meaning given, others are described only in outline.

### When it will not start

**A named error with a stated meaning.** This is the clearest case in the sources:

> "Unsupported Graphics Driver Error"
> "This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

— Manual, https://docs.blender.org/manual/en/4.5/troubleshooting/gpu/common/other.html — read 2026-07-30

**Messages go to a console the user may not see, and the manual says so:**

> "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window. To ensure you can see any error messages that appear at launch, open a command-line window and navigate to the location where the Blender executable is, and run blender from that location."

> "The Blender Console Window can display many different types of status and error messages. Some messages simply inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender non-responsive or shut down completely."

**Two start-up messages are documented by name.** Notably, the manual describes one missing piece by its *absence* rather than by a message:

> "found bundled python: {DIR}"
> "This message indicates that Blender was able to find the Python library for the Python interpreter embedded within Blender. If this folder is missing or unable to be found, it is likely that an error will occur, and this message will not appear."

> "Read prefs: {DIR}/userpref.blend"
> "The preferences use this path."

All the above — Manual, https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html — read 2026-07-30

**When it stops rather than fails to start, a file is written:**

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."

with the file named per system: `test.crash.txt` beside the loaded blend-file on Windows, the macOS Crash Reporter plus `~/Library/Logs/DiagnosticReports/`, and `blender.crash.txt` in `/tmp` on Linux.

— Manual, https://docs.blender.org/manual/en/4.5/troubleshooting/crash.html — read 2026-07-30

### When it will not render on the graphics cards

**A supplied Cycles file defines missing-requirement text for its preferences panel.** It holds the exact strings under the branch used when no compatible card is found. This establishes the text and where the file places it, but not what a running copy displays:

```
        if not found_device:
            col = box.column(align=True)
            col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)
```

and directly beneath, per technology:

```
            if device_type == 'CUDA':
                compute_capability = "3.0"
                col.label(text=rpt_("Requires NVIDIA GPU with compute capability %s") % compute_capability,
                          icon='BLANK1', translate=False)
            elif device_type == 'OPTIX':
                compute_capability = "5.0"
                driver_version = "535"
                col.label(text=rpt_("Requires NVIDIA GPU with compute capability %s") % compute_capability,
                          icon='BLANK1', translate=False)
                col.label(text=rpt_("and NVIDIA driver version %s or newer") % driver_version,
                          icon='BLANK1', translate=False)
```

with equivalents for HIP ("Requires AMD GPU with RDNA architecture", "and AMD Adrenalin driver 24.6.1 or newer", "or AMD Radeon Pro 24.Q2 driver or newer", "and AMD driver version 23.40 or newer"), oneAPI ("Requires Intel GPU with Xe-HPG architecture", "and Windows driver version XX.X.101.6557 or newer", "  - intel-level-zero-gpu or intel-compute-runtime version", "    XX.XX.31740.15 or newer", "  - oneAPI Level-Zero Loader") and Metal ("Requires Apple Silicon with macOS 12.2 or newer").

— `Blender Sources/cycles_4.5/properties.py`, lines 1794–1853 — read 2026-07-30

There is also a per-card note appended to each device's name in the same panel:

```
        for device in devices:
            name = self._format_device_name(device.name)
            if not device.optimized:
                name += rpt_(" (Unoptimized Performance)")
```

— `Blender Sources/cycles_4.5/properties.py`, lines 1855–1859 — read 2026-07-30

**Four render-time errors are named and explained in the manual,** under a heading called "Error Messages":

> "Error: Out of memory"
> "This usually means there is not enough memory to store the scene for use by the GPU."

> "The NVIDIA OpenGL driver lost connection with the display driver"
> "If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations."

> "Unsupported GNU version"
> "On Linux, depending on your GCC version you might get this error."

> "CUDA Error: Kernel compilation failed"
> "This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html — read 2026-07-30

**One release note describes a message that is wrong,** which is worth having because it tells you the message exists:

> "GPU hardware ray-tracing package from the Ubuntu repository does not work on Ubuntu 25.04 due to incompatible TBB versions. There is a message saying that the installation of a package is required, even when the package is installed."

— Release notes, https://developer.blender.org/docs/release_notes/4.5/cycles/ — read 2026-07-30

**A supplied Cycles file turns denoiser exceptions into error reports.** It catches the exception and passes its text to `report`:

```
        except Exception as e:
            self.report({'ERROR'}, str(e))
```

— `Blender Sources/cycles_4.5/operators.py`, lines 110–111 (the same two lines again at 149–150) — read 2026-07-30

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 4.5's **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** panel was expanded and every technology button in its row was clicked in turn. The row held two buttons, **None** and **Metal**. With Metal selected the panel listed the machine's two devices; with None selected every other setting in the panel was hidden. Neither the "No compatible GPUs found for Cycles" line nor any "Requires…" line appeared. A Metal device was present on this machine, so this run does not reach the missing-requirement case and records nothing about what that case displays.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), three things were put to `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender` in background mode, with `/tmp/machine-test.blend` set to Cycles and GPU.

Asked for a technology this machine does not have, `-- --cycles-device OPTIX`, it exited with status `1`, wrote no image file, and printed:

```
Error: Found no Cycles device of the specified type
```

Given a value that is not a device name at all, `-- --cycles-device NOTADEVICE`, it exited with status `1`, wrote no image file, and printed:

```
Error: NOTADEVICE is not a valid Cycles device.
```

But asked to render on graphics cards when **every card had been switched off in the saved preferences**, it exited with status `0`, wrote the image, and printed nothing naming a missing or unusable device — the run ended `Fra:1 Mem:225.15M (Peak 320.08M) | Time:00:04.85 | Mem:150.38M, Peak:150.38M | Scene, ViewLayer | Sample 16/16`, without the `(Using optimized kernels)` text that the same render carried when the card was in use. The same silence followed setting the technology to `NONE`. Full commands and output are under question 5.

Crash files were produced during the day's runs and are worth recording for their naming. In this version the line printed was `Writing: /var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/blender.crash.txt` — that is, `blender.crash.txt` in the temporary directory, not a file named after the loaded blend-file. Those crashes were traced by the crash file's own `# Python backtrace` to a background thread in a third-party add-on (`poliigon-addon-blender`), not to device handling, and re-running the same command completed normally.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, five things bearing on this question were put to 4.5.

**A device Cycles will not accept.** With `-- --cycles-device METAL`, `-- --cycles-device HIP` and `-- --cycles-device ONEAPI` — the first not built into this copy, the other two built in but with no matching card — all three ended the same way: exit status `1`, no image file, and one line:

```
Error: Found no Cycles device of the specified type
```

So a technology built in but with no card gives the same message as one not built in at all. With `-- --cycles-device NOTADEVICE`, exit `1`, no image, and:

```
Error: NOTADEVICE is not a valid Cycles device.
```

Both messages are word for word what the Mac gave.

**The quiet cases are quiet here too.** With every card switched off in the saved preferences, and again with the technology set to `NONE`, and again with the technology set to `ONEAPI` for which this machine has no card, the render exited `0`, wrote its image, and printed nothing naming a missing or unusable device. It fell back to the processor:

```
Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
```

**Whether "No compatible GPUs found for Cycles" ever reaches the console — settled here, and the answer is no.** With the technology saved as `ONEAPI` (`SAVED: ONEAPI ACTIVE: False NUMGPU: 0`), the whole output of a background render was searched for that phrase and it appeared **nought times**. No `Requires…` line appeared either. The same text *is* drawn in the preferences panel, and what it says there is quoted under question 2.

**Where the crash file lands on Linux.** A crash was forced with `-b <file> --python-expr "import ctypes; ctypes.string_at(0)"`. Exit status `139`, and one line:

```
Writing: /tmp/machine-test.crash.txt
```

With no blend-file loaded, the same crash gave:

```
Writing: /tmp/blender.crash.txt
```

So on Linux the file goes into the temporary directory — `/tmp`, which is also what Python inside Blender reports as the system temporary directory, Blender's own temporary-directory preference being empty — and it is named after the loaded blend-file when there is one. It does not sit beside the blend-file: the test file was in the work folder and the crash file was in `/tmp`. The file opens:

```
# Blender 4.5.12, Commit date: 2026-07-20 13:37, Hash 84afd5f785f7

# backtrace
/opt/blender/blender-4.5.12-linux-x64/blender() [0xf82b50]
…

# Python backtrace
  File "/opt/blender/blender-4.5.12-linux-x64/4.5/python/lib/python3.11/ctypes/__init__.py", line 519 in string_at
  File "<string>", line 1 in <module>
```

**What Blender says when it cannot open a window.** Quoted in full under question 1: it names both windowing systems it tried and then, in this version on this machine, does not exit.

**Not covered.** The exact wording Blender prints when it will not start for want of a good enough graphics card — the manual names the *class* of error ("Unsupported Graphics Driver Error") and explains its meaning, but does not print the literal line, and no machine below the floor exists to reach it. (What Blender prints when there is no *display server* is settled under question 1; that is a different case.) Whether any of these messages are machine-readable, or carry a code. What Blender says when a card is present and supported but its driver is below the stated minimum, as opposed to no card being found. Whether a render exits with a distinguishable status code beyond the `0` and `1` recorded here.

**Outside the interface.** The "No compatible GPUs found for Cycles" text and the requirement lines beneath it are produced inside `_draw_devices`, which is panel-drawing code — from the file alone it was reasonable to expect them not to be printed when nothing is drawn, and that step made the answer **inferred**. It is now observed on the rented Linux machine and it holds: with the technology set to one that has no compatible card, a windowless render printed that phrase nought times and completed on the processor. The manual does say the console carries error messages at launch and recommends starting from a command line precisely so they can be seen. `--debug-cycles` ("Enable debug messages from Cycles") and `--log`/`--log-level`/`--log-file` exist for getting more out (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html). What Cycles prints to the console when it cannot use a card in background mode is now recorded on two machines: a refused `--cycles-device` value names the fault and exits `1`, and everything else is silent.

**Operating system.** The crash-file behaviour is documented separately per system, as quoted, and Linux is now tested: `/tmp/<name of the loaded file>.crash.txt`, or `/tmp/blender.crash.txt` with nothing loaded. The Mac wrote `blender.crash.txt` into its own temporary directory. The "Unsupported GNU version" error is stated as Linux-only and was not reached, it depending on the compiler on the machine; the display-driver timeout is stated for Windows, which is untested. The per-technology requirement lines branch on `sys.platform` in the Cycles file itself, so the Linux machine draws the Linux lines and can never draw the Windows ones — the Intel and AMD Linux figures 4.5 draws are quoted under question 2. Windows additionally ships helper scripts named in the manual — `blender_debug_log.cmd`, `blender_debug_gpu.cmd`, `blender_debug_gpu_workaround.cmd`, `blender_factory_startup.cmd` — with no counterparts named for macOS or Linux, and none of them could be run.

One more thing the two machines showed differently, and it is about shape rather than content: on Linux the messages are worded exactly as on the Mac in this version, plain and untimestamped.

---

## 5. How is it decided which of a computer's graphics cards a render uses, can more than one card be used for the same render, and how is that decided from outside Blender's own interface?

**Label: documented, plus observed in supplied Cycles files. Inferred parts are marked.**

**A note on reading.** I read "a render" as a Cycles render, since that is the engine the sources discuss cards for; EEVEE is covered at the end because it answers the middle part differently. I read "which of a computer's graphics cards" as *which physical cards*, not *which kind of computation*. Both readings matter here, because the answer splits along exactly that line.

### How it is decided

There are **three separate switches**, and all three must line up.

**First, a technology.** One is chosen for the whole of Blender, in the preferences:

> "Cycles Render Device"
> "Changes the computing device the Cycles render engine uses to render images. Cycles can use either the CPU or certain GPUs to render images"
> "None: When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."
> "CUDA: If the system has a compatible NVIDIA CUDA device, it will be available as an option for rendering with Cycles."

(and the same sentence pattern for OptiX, HIP, oneAPI and Metal)

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html — read 2026-07-30

In Cycles' own file that is one setting, and it is a single choice, not a set:

```
    compute_device_type: EnumProperty(
        name="Compute Device Type",
        description="Device to use for computation (rendering with Cycles)",
        default=CyclesPreferences.default_device(),
        items=CyclesPreferences.get_device_types,
    )
```

Its default is chosen at run time by machine:

```
    @staticmethod
    def default_device():
        import platform
        # Default to selecting the Metal compute device on Apple Silicon GPUs
        # (drivers are tightly integrated with macOS so pose no stability risk)
        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
            return 5
        return 0
```

Value 5 is `METAL` and value 0 is `NONE`, per the list built in `get_device_types` (lines 1567–1583) and the fixed table at lines 176–183.

— `Blender Sources/cycles_4.5/properties.py`, lines 1558–1590 — read 2026-07-30

**Second, the individual cards.** Each card Blender finds gets its own on/off flag:

```
class CyclesDeviceSettings(bpy.types.PropertyGroup):
    id: StringProperty(name="ID")
    name: StringProperty(name="Name")
    use: BoolProperty(name="Use", default=True)
    type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
```

and, in `CyclesPreferences`:

```
    devices: CollectionProperty(type=CyclesDeviceSettings)
```

The flag's starting state when a card is first seen is set here — every graphics card starts switched **on**, and the processor starts switched **off**:

```
            if not entry:
                # Create new entry if no existing one was found
                entry = self.devices.add()
                entry.id = device[2]
                entry.name = device[0]
                entry.type = device[1]
                entry.use = entry.type != 'CPU'
```

— `Blender Sources/cycles_4.5/properties.py`, lines 1548–1552, 1592, 1654–1669 — read 2026-07-30

Only cards matching the chosen technology are offered, plus the processor:

```
        for device in device_list:
            entry = self.find_existing_device_entry(device)
            entry.optimized = device[7]
            if entry.type == compute_device_type:
                devices.append(entry)
            elif entry.type == 'CPU':
                cpu_devices.append(entry)
        # Extend all GPU devices with CPU.
        if len(devices) and compute_device_type != 'CPU':
            devices.extend(cpu_devices)
```

— `Blender Sources/cycles_4.5/properties.py`, lines 1673–1691 — read 2026-07-30

**Third, the scene must ask for it.** This is per blend-file, not per machine:

> "Next, you must configure each scene to use GPU rendering in Properties ‣ Render ‣ Device."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html — read 2026-07-30

and in the file, with only two choices and a default of processor:

```
enum_devices = (
    ('CPU', "CPU", "Use CPU for rendering"),
    ('GPU', "GPU Compute",
        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
)
```
```
    device: EnumProperty(
        name="Device",
        description="Device to use for rendering",
        items=enum_devices,
        default='CPU',
    )
```

— `Blender Sources/cycles_4.5/properties.py`, lines 30–33 and 376–381 — read 2026-07-30

A supplied Cycles file also defines a helper named `use_gpu` that checks all three:

```
def use_gpu(context):
    cscene = context.scene.cycles

    return (get_device_type(context) != 'NONE' and cscene.device == 'GPU' and backend_has_active_gpu(context))
```

where `backend_has_active_gpu` is `has_active_device()`, which is `get_num_gpu_devices() > 0`.

— `Blender Sources/cycles_4.5/ui.py`, lines 96–113, and `properties.py` lines 1714–1740 — read 2026-07-30

### Can more than one card be used for the same render

**Yes.** Stated outright:

> "Can multiple GPUs be used for rendering?"
> "Yes, go to Preferences ‣ System ‣ Compute Device Panel, and configure it as you desire."

**But their memory is not pooled by default:**

> "Would multiple GPUs increase available memory?"
> "Typically, no, each GPU can only access its own memory."
> "The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html — read 2026-07-30

> "Distribute Memory Across Devices"
> "Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes. Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol."
> "Currently only NVLink on NVIDIA GPUs is supported."

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html — read 2026-07-30

That option is off to begin with:

```
    peer_memory: BoolProperty(
        name="Distribute memory across devices",
        description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it",
        default=False,
    )
```

— `Blender Sources/cycles_4.5/properties.py`, lines 1594–1598 — read 2026-07-30

**Cards and processor together are possible** (see the command-line switch below), and Cycles has a name for that state:

```
    def has_multi_device(self):
        compute_device_type = self.get_compute_device_type()
        if compute_device_type != 'NONE':
            for device in self.get_device_list(compute_device_type):
                if device[1] == compute_device_type:
                    continue
                for dev in self.devices:
                    if dev.use and dev.id == device[2]:
                        return True

        return False
```

— `Blender Sources/cycles_4.5/properties.py`, lines 1727–1737 — read 2026-07-30

**Mixing technologies is not offered.** Because `get_devices_for_type` only lists entries whose `type` equals the single chosen `compute_device_type` (plus the processor), a machine holding both an NVIDIA and an AMD card cannot have both ticked at once. **Inferred** — the step is that the panel and the counting functions both filter on that one type, so no second type's cards can ever be in the used set.

**A supplied Cycles file applies one hardware-ray-tracing condition across the used cards.** Its own comment states:

```
        # Any device without RT support will disable it for all.
        has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt
```

— `Blender Sources/cycles_4.5/properties.py`, line 1890 — read 2026-07-30

### From outside Blender's own interface

**The technology can be set on the command line. The individual cards cannot.**

> "Cycles Render Options"
> "Cycles add-on options must be specified following a double dash."
> "--cycles-device <device>"
> "Set the device used for rendering."
> "Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL."
> "Append +CPU to a GPU device to render on both CPU and GPU."
> "Example:"
> "blender -b file.blend -f 20 -- --cycles-device OPTIX"

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

I searched the whole of the command-line arguments page for `device` and `gpu`. The complete list of hits is: `--cycles-device`, the audio `-setaudio` option, the `--debug-gpu*` family, `--gpu-backend` ("Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."), `--gpu-compilation-subprocesses`, `--profile-gpu` and `--debug-gpu-vulkan-local-read`. **There is no argument that names or selects an individual graphics card.**

So, to pick *which* cards from outside, the sources leave two routes, and both are the same underlying data:

- **The saved preferences file. Inferred.** The manual says preferences are stored at `./config/userpref.blend` (https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html), are read at start-up — `Read prefs: {DIR}/userpref.blend` (https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html) — and by default are saved on exit (https://docs.blender.org/manual/en/4.5/editors/preferences/introduction.html). The supplied Cycles file places the per-card `use` flags in preferences. The reasoning step is that these flags therefore travel through the saved preferences file.
- **Python. Inferred.** Cycles reaches its preferences as `context.preferences.addons[__package__].preferences` throughout its supplied files (for example `ui.py` line 97, `properties.py` line 272, `version_update.py` lines 69 and 88), and `CyclesPreferences.bl_idname = __package__` (`properties.py` line 1556). The reasoning step is that the properties on that object can therefore be set by an outside script. The Python API change log names one method on this class:

> "bpy.types.CyclesPreferences.get_devices_for_type (self, compute_device_type, device_list), was (self, compute_device_type)"

— Python API, https://docs.blender.org/api/4.5/change_log.html — read 2026-07-30

**A caution stated in Cycles' own file, which bears directly on scripting this:**

```
    # Be careful when deciding when to call this function,
    # as Blender can crash with `_cycles.available_devices()` on some drivers.
    def get_device_list(self, compute_device_type):
```
```
    # Refresh device list. This does not happen automatically on Blender
    # startup due to unstable drivers that can cause crashes.
    def refresh_devices(self):
```

— `Blender Sources/cycles_4.5/properties.py`, lines 1637–1639 and 1693–1695 — read 2026-07-30

**Splitting one frame over several machines** is documented as a separate mechanism, not a device setting:

> "A typical use case is to distribute the rendering of a single frame over multiple machines. Say you want to render 2048 samples in total, but split this work over two machines because doing it on one would take too long:"
> "On both machines, set the (Max) Samples to 2048, disable Denoise, and enable Sample Subset."
> "On the first machine, set Offset to 0 and Length to 1024."
> "On the second machine, set Offset to 1024 and Length to 1024."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read 2026-07-30

### EEVEE, and the card that draws

EEVEE answers the middle part differently:

> "Multiple GPU Support"
> "There is currently no support for multiple GPU systems."

— Manual, https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html — read 2026-07-30

For the card Blender draws with, there is a preference, but in 4.5 it offers only one value:

> "gpu_preferred_device"
> "Preferred device to select during detection (requires restarting Blender for changes to take effect)"
> "AUTO — Auto detect best GPU for running Blender."
> "Type:"
> "enum in ['AUTO'], default 'AUTO'"

— Python API, https://docs.blender.org/api/4.5/bpy.types.PreferencesSystem.html — read 2026-07-30

> "Device Vulkan"
> "Specifies which GPU device to use for display drawing operations."
> "This setting is useful for systems with multiple GPUs (e.g., integrated + discrete) where you want to force Blender to use a specific GPU for UI rendering."
> "Auto: Automatically selects the most appropriate GPU based on system configuration and driver support."

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html — read 2026-07-30

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), the whole of **Edit ‣ Preferences ‣ System** was looked at, in all four versions. The page held, top to bottom: **Cycles Render Devices** (two buttons, `None` and `Metal`, with `Metal` selected; two tick boxes, `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked; `Kernel Optimization` showing `Full`; `MetalRT` showing `Auto`), **Network** (`Allow Online Access` ticked, `Time Out` 10, `Connection Limit` 5), **Memory & Limits** (`Undo Steps` 256, `Undo Memory Limit` 0, `Global Undo` ticked, `Console Scrollback Lines` 256, `Texture Time Out` 120, `Garbage Collection Rate` 60, `VBO Time Out` 120, `Garbage Collection Rate` 60, `Geometry Nodes Stack Limit` 100), **Video Sequencer** (`Memory Cache Limit` 16384, `Proxy Setup` showing `Automatic`), and **Sound**, collapsed.

**There was no Display Graphics section, and no graphics backend or device control anywhere on the page** — in any of the four versions. So on this machine the setting quoted above was not present to be read.

One thing to say plainly rather than let it read as a failed search: the test that produced this was mis-aimed. It asked what the **Device** list offers once the backend is set to **Vulkan**, and Vulkan is not something a Mac offers at all — the question assumed a control that does not exist on this operating system. What a Windows or Linux machine shows here is untested, because neither exists among the equipment available.

And on laptops the choice is handed to the system, not to Blender:

> "For the best performance, the dedicated GPU should be used for Blender. Which GPU to use for which application can be configured in your graphics driver or operating system settings."

— Manual, https://docs.blender.org/manual/en/4.5/troubleshooting/gpu/common/laptops.html — read 2026-07-30

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 4.5.

**In the preferences.** **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** was expanded and every button in its technology row was clicked in turn. The row held two buttons, **None** and **Metal**, with Metal selected on arrival. Selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — plus **Kernel Optimization** (set to **Full**; other choices **Off** and **Intersection Only**) and **MetalRT** (set to **Auto**; other choices **Off** and **On**). Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed.

**In Blender's Python Console** (Scripting workspace, window open). `list(bpy.context.preferences.addons.keys())` returned:

```
['io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'cycles', 'pose_library', 'bl_pkg']
```

so the add-on key is the literal string `cycles` on this machine. With `p = bpy.context.preferences.addons["cycles"].preferences`:

- `p.compute_device_type` returned `'METAL'`.
- `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned `[]` — an empty list — before anything else was called.
- After `p.refresh_devices()`, the same expression returned:

```
[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

- `p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, `p.has_multi_device()` returned `False`.
- `p.devices[0].use = False` was then run — index 0 being the processor entry, which is unticked by default. The preferences panel at that moment showed exactly what it shows by default. `p.devices[0].use = True` was then run, and the **Apple M1 Pro** box in the panel became ticked.

This was done from a Python Console inside Blender's own window. It does not record what happens in a background run.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, which reports `Blender 4.5.12 LTS`. The scene was `/tmp/machine-test.blend`: Cycles engine, scene Device `GPU`, 16 samples. Output is quoted as the machine printed it.

**The device list with no window open.** Command:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --python-expr "import bpy; ks=[k for k in bpy.context.preferences.addons.keys() if 'cycles' in k.lower()]; print('CYCLES KEYS:', ks); p=bpy.context.preferences.addons[ks[0]].preferences; print('BACKEND:', p.compute_device_type); print('BEFORE:', [(d.name, d.type, d.id, d.use) for d in p.devices]); p.refresh_devices(); print('AFTER:', [(d.name, d.type, d.id, d.use) for d in p.devices])"
```

printed:

```
CYCLES KEYS: ['cycles']
BACKEND: METAL
BEFORE: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
AFTER: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

So with no window open the stored list was already populated before `refresh_devices()` was called. The same command with `--factory-startup` added after `-b` printed instead:

```
CYCLES KEYS: ['cycles']
BACKEND: METAL
BEFORE: []
AFTER: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

**The raw list from `_cycles`.** Command:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types()); print('METAL:', _cycles.available_devices('METAL')); print('NOARG:', _cycles.available_devices())"
```

printed:

```
TYPES: (False, False, False, True, False, False)
METAL: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False, True, True, False, True))
```

and the third call raised:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
TypeError: function takes exactly 1 argument (0 given)
```

(The traceback appears above the two printed lines in the combined terminal capture; the two `print` calls ran first and the error came from the third.) So each entry is an eight-field tuple, and `_cycles.available_devices()` called with no argument is refused.

**A background render with the saved preferences as they stood** (the graphics card ticked). Command:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b /tmp/machine-test.blend --debug-cycles -o /tmp/render-45- -f 1
```

exited `0`, wrote `/tmp/render-45-0001.png`, and printed among other lines:

```
I0801 18:18:54.595114 15783194 device.mm:105] Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
I0801 18:18:54.595146 15783194 device.mm:108] Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
I0801 18:18:54.598387 15783194 device_impl.cpp:47] Using default CPU kernels.
Fra:1 Mem:74.69M (Peak 169.62M) | Time:00:39.15 | Mem:1558.44M, Peak:1558.44M | Scene, ViewLayer | Sample 16/16 (Using optimized kernels)
Fra:1 Mem:74.69M (Peak 169.62M) | Time:00:39.15 | Mem:1558.44M, Peak:1558.44M | Scene, ViewLayer | Finished
```

**Every card switched off in the saved preferences, then rendered.** First:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --python-expr "import bpy; p=bpy.context.preferences.addons['cycles'].preferences; p.refresh_devices(); [setattr(d,'use',False) for d in p.devices]; bpy.ops.wm.save_userpref(); print('SAVED STATE:', p.compute_device_type, [(d.name,d.use) for d in p.devices])"
```

which printed `SAVED STATE: METAL [('Apple M1 Pro', False), ('Apple M1 Pro (GPU - 14 cores)', False)]`, so `bpy.ops.wm.save_userpref()` runs with no window. The same render command then exited `0`, wrote its PNG, and printed:

```
I0801 18:23:08.476790 15786642 device.mm:105] Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
I0801 18:23:08.477234 15786642 device_impl.cpp:47] Using default CPU kernels.
I0801 18:23:08.478374 15786642 device_impl.cpp:47] Using default CPU kernels.
Fra:1 Mem:225.15M (Peak 320.08M) | Time:00:04.85 | Mem:150.38M, Peak:150.38M | Scene, ViewLayer | Sample 16/16
Fra:1 Mem:225.15M (Peak 320.08M) | Time:00:04.85 | Mem:150.38M, Peak:150.38M | Scene, ViewLayer | Finished
```

No line naming a missing or unusable device appeared anywhere in that output.

**The same saved state, with `--cycles-device METAL` added.** Command:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b /tmp/machine-test.blend --debug-cycles -o /tmp/r8b-45- -f 1 -- --cycles-device METAL
```

exited `0`, wrote its PNG, and printed:

```
Fra:1 Mem:74.69M (Peak 169.62M) | Time:00:03.65 | Mem:1558.44M, Peak:1558.44M | Scene, ViewLayer | Sample 16/16 (Using optimized kernels)
```

**The technology set to `NONE` in the saved preferences, with the card ticked back on.** The render exited `0`, wrote its PNG, printed no `Added device` line at all, and ended:

```
Fra:1 Mem:225.16M (Peak 320.08M) | Time:00:04.87 | Mem:150.38M, Peak:150.38M | Scene, ViewLayer | Sample 16/16
```

**`--cycles-device` with values this machine cannot provide.** With `-- --cycles-device OPTIX`, exit status `1`, no image file written, and:

```
Error: Found no Cycles device of the specified type
```

With `-- --cycles-device NOTADEVICE`, exit status `1`, no image file written, and:

```
Error: NOTADEVICE is not a valid Cycles device.
```

With `-- --cycles-device METAL+CPU`, exit status `0`, an image file written, and:

```
Fra:1 Mem:308.71M (Peak 403.63M) | Time:00:04.26 | Mem:150.40M, Peak:150.40M | Scene, ViewLayer | Sample 16/16
```

**Preference changes made by a script inside the render's own run.** Three commands of the form `-b /tmp/machine-test.blend --python-expr "…" --debug-cycles -o … -f 1`, changing the preferences before the frame is rendered and never saving them. Setting every device's `use` to `False` printed `SET TO: [('Apple M1 Pro', False), ('Apple M1 Pro (GPU - 14 cores)', False)]` and ended `Fra:1 Mem:225.16M (Peak 320.08M) | Time:00:04.72 | Mem:150.38M, Peak:150.38M | Scene, ViewLayer | Sample 16/16`. Setting every device's `use` to `True` printed `SET TO: [('Apple M1 Pro', True), ('Apple M1 Pro (GPU - 14 cores)', True)]` and ended `Fra:1 Mem:309.96M (Peak 404.88M) | Time:00:04.31 | Mem:150.38M, Peak:150.38M | Scene, ViewLayer | Sample 16/16`. Setting `compute_device_type` to `'NONE'` printed `BACKEND NOW: NONE` and ended `Fra:1 Mem:225.15M (Peak 320.08M) | Time:00:04.80 | Mem:150.38M, Peak:150.38M | Scene, ViewLayer | Sample 16/16`. All three exited `0` and wrote an image. None of the three carried the `(Using optimized kernels)` text that the saved-preferences GPU render above carried. Reading the saved preferences afterwards showed them unchanged.

**Two things about this machine that bear on reading the above.** The saved preferences were restored to exactly the state found before these runs. And the Blenders on this machine carry third-party add-ons; several runs ended in exit status `139` with `Writing: /var/folders/…/T/blender.crash.txt`, and the crash file's `# Python backtrace` named a background thread in a third-party add-on (`poliigon-addon-blender`), not anything to do with devices. Re-running the same command completed normally each time.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-4.5.12-linux-x64/blender`. Its settings folder started empty, so this version began from factory settings rather than from anything a person had set. One thing about that folder, said plainly rather than left out. The first runs were meant to point Blender at a folder of this topic's own with `BLENDER_USER_RESOURCES`, and a mistake in the shell meant the variable was never actually set, so those runs used Blender's ordinary default folder instead. That folder was also empty when this work began, so "began from factory settings" holds either way. Every test that turns on saved preferences was then run again with the variable properly set, and every result came back identical. Both runs are described in `Audits/Blender Research/The machine/Hand tests — Linux.md`.

**What Blender finds, and how the one card is listed.** The device-list command of the Mac test, `-b --python-expr "…print('BEFORE'…); p.refresh_devices(); print('AFTER'…)"`, printed with an untouched settings folder:

```
CYCLES KEYS: ['cycles']
BACKEND: NONE
BEFORE: []
AFTER: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True)]
```

Four things in that. The add-on key is the literal string `cycles`, as on the Mac. The technology starts as `NONE` here, where it starts as `METAL` on the Mac — which is exactly what Cycles' `default_device` says it should do. Every graphics entry starts ticked and the processor starts unticked, as the file says. And **one physical card is listed twice**, once under CUDA and once under OptiX, with two different stored identifiers; the identifier carries the card's place on the bus.

With preferences already saved, `BEFORE` came back already populated, and with `--factory-startup` added it came back `[]` again — the same pattern as the Mac. `has_active_device()` returned `True`, `get_num_gpu_devices()` returned `1`, and `has_multi_device()` returned `False`.

**The saved switches are honoured, and the flag displaces them.** With the technology saved as `OPTIX` and every device switched off, a background render exited `0`, wrote its image and reported `Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]`. The same render with `-- --cycles-device OPTIX` appended exited `0` and reported `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]`. With the devices back on and the technology set to `NONE`, the processor again, and this time no `Added device` line at all.

**The flag also displaces the scene's own setting.** A copy of the scene saved with its own Device set to `CPU`, rendered with `-- --cycles-device OPTIX`, reported `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) […]`.

**`+CPU` really does use both, and Linux shows it plainly.** With `-- --cycles-device CUDA+CPU`, exit `0`, image written, and the report named two devices:

```
Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                 AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (11 threads) [CPU]
```

Note eleven threads for the processor there and twelve when it works alone. `-- --cycles-device OPTIX+CPU` did the same.

**A preference change made by a script inside the render's own run — and here the two machines disagree.** Three runs, each changing the preferences and then rendering in the same command, never saving them. Switching every device off:

```
SET TO: [('NVIDIA GeForce RTX 4090', False), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', False), ('NVIDIA GeForce RTX 4090', False)]
Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
```

Switching every device on:

```
SET TO: [('NVIDIA GeForce RTX 4090', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', True), ('NVIDIA GeForce RTX 4090', True)]
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
                 AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (11 threads) [CPU]
```

Setting the technology to `NONE`:

```
BACKEND NOW: NONE
Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
```

All three exited `0` and wrote an image. On the rented Linux machine the change made inside the run **took effect** — the report named exactly the devices the script had asked for. On the Creator's Mac the same three runs carried no graphics-card marker at all. Both results stand and neither is chosen between here.

**There is a control over which card draws Blender's interface, and on Linux it exists.** Read with no window, `bpy.context.preferences.system` gave `GPU_BACKEND NOW: OPENGL | CHOICES: ['OPENGL', 'METAL', 'VULKAN']` and `GPU_PREFERRED_DEVICE NOW: AUTO | CHOICES: ['AUTO']`. With a window open, the System page carries a **Display Graphics** section — which the Mac has not got at all — holding a **Backend** dropdown reading `OpenGL`. With the backend saved as `VULKAN` and Blender restarted, a second control appears beneath it, **Device**, reading `Auto`, together with a note this version and 5.0 carry and the later two do not:

```
Current Vulkan backend limitations:
  • Low performance in VR
```

The Device list offered only `Auto`, and the property confirms it: `gpu_preferred_device` held `['AUTO']` and nothing else, even with Vulkan actually running. That is one card, not a claim about machines with several. Changing the backend without restarting shows `A restart of Blender is required`.

The System page on Linux also carries an **Operating System Settings** section — `Open blend files with this Blender version`, with `Register` and `Unregister` buttons and a `For All Users` box — which the Mac has not got.

**`--gpu-device` does not exist in this version.** Given `--gpu-device 0`, Blender treated it as a file name:

```
unknown argument, loading as file: --gpu-device
Error: Cannot read file "/root/--gpu-device": No such file or directory
```

**Not covered.** How Cycles divides work between two ticked cards, and whether a slow card holds back a fast one. Whether ticking cards plus the processor is faster or slower — that is a timing question, and the rented machine is shared, so no timing claim can be made on it. What the "Persistent ID" of a card is made of — the code calls it that (comment "Layout of the device tuples: (Name, Type, Persistent ID)", line 1674) but nothing states what it contains; on Linux it visibly carries the card's place on the bus, which is a hint and not a statement. Whether it survives the card being moved to another slot or a driver update. Whether `--cycles-device` re-enables a card the preferences had switched off when there is more than one card of that technology to choose between — the one-card form is now settled on both machines and the several-card form is not. What the Vulkan **Device** control offers on a machine where more than one card exists.

**Outside the interface.** Answered above and it is the sharp edge of this question: the *technology* is settable by command line, the *scene's* CPU/GPU choice is a file setting, but **the individual cards are only reachable through the saved preferences or Python** — no command-line switch names one. The Cycles code's own warnings about when the device list may safely be refreshed apply just as much to a script; asking for the list did not crash Blender on either machine. This is now tested on two machines and they agree on every part but one. On the Creator's Mac, with every card switched off in the saved preferences, adding `-- --cycles-device METAL` produced a render carrying the `(Using optimized kernels)` text and about 1558M of device memory where the same render without it carried neither; on the rented Linux machine, with every device switched off, adding `-- --cycles-device OPTIX` produced `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX)` where the same render without it reported the processor. The add-on key is the literal string `cycles` on both. The per-card switches were read and set with no window on both, `bpy.ops.wm.save_userpref()` ran with no window on both, and a saved preferences file was honoured by a windowless render on both. The one disagreement is the same-run change, recorded above: it took effect on Linux and showed no graphics-card marker on the Mac. What is still untested anywhere is a machine with more than one card, where the question of *which* card becomes real.

**Operating system.** The available technologies differ by system (question 2), and the default technology differs by system in code: Metal on Apple Silicon, none elsewhere. Both halves of that are now observed — `METAL` on the Mac, `NONE` on the rented Linux machine. The MetalRT setting is drawn only when `platform.machine() == 'arm64'` (`properties.py` lines 1897–1907), and the Linux panel carries neither it nor Kernel Optimization; in their place it carries a greyed `Embree on GPU` box under oneAPI and a greyed `HIP RT` box under HIP. Linux also has a **Display Graphics** section and an **Operating System Settings** section that macOS has not. Otherwise the mechanism — one technology, per-card flags, per-scene choice — behaved the same on both machines. Windows is untested.

### Hand tests on the two-card machine — observed
**Read this block against the *Not covered* paragraph just above it.** That paragraph was written before any two-card machine existed, so some of what it calls untested is answered here — how work is divided between two ticked cards, and the several-card form of the `--cycles-device` question above all. What it says about an NVLink pair, about two cards of different makes, about a slow card beside a fast one, and about anything needing a window still stands. *(Pointer added 2026-08-06 by the citation check of the two-card material.)*

Run on **2026-08-06** on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-4.5.12-linux-x64/blender`, which `--version` reports as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`. Label: **observed**.

**This is a different machine from the one-card rented Linux machine above, and nothing here is merged with that machine's results or with the Creator's Mac.** Its settings folder was `/root/smoozy/two cards/res-4.5`, empty when this work began, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender every time. **Nothing else was running on the machine**, which is why the timings below are quoted at all. The full list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`.

**The two cards are not joined by NVLink.** `nvidia-smi topo -m` reports `PHB` between them, whose own legend reads `PHB  = Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)`. So this machine still cannot test an NVLink pair, and that limit is repeated wherever it bears on a result.

**Both cards are switched on at factory settings, and one card is listed twice.** With an untouched settings folder, `--factory-startup` and a script that calls `refresh_devices()` and then prints every device as `(name, type, id, use)`:

```
PREFS PATH: /root/smoozy/two cards/res-4.5
BACKEND: NONE
BEFORE: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a', True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX', True)]
```

The label `BEFORE` is the script's own word and is misleading — the refresh had already happened when that line was printed. Five entries for two cards: each card once under CUDA and once under OptiX, plus the processor. Every graphics entry starts ticked. The stored identifier carries each card's place on the bus, `0000:00:07` and `0000:00:0a`.

**A render uses both, without being asked to.** With the technology saved as `OPTIX` and both cards ticked, a windowless render of a Cycles scene at 1920 by 1080 and 6000 samples was watched card by card with `nvidia-smi --query-gpu=index,utilization.gpu,memory.used --format=csv -l 1`:

```
index, utilization.gpu [%], memory.used [MiB]
0, 0 %, 1 MiB
1, 2 %, 1 MiB
0, 0 %, 2683 MiB
1, 0 %, 2683 MiB
0, 98 %, 2683 MiB
1, 98 %, 2683 MiB
0, 97 %, 2683 MiB
1, 98 %, 2683 MiB
…
0, 98 %, 2683 MiB
1, 98 %, 2683 MiB
0, 0 %, 5 MiB
1, 89 %, 3 MiB
```

Both cards sat between 91 % and 98 % for the whole render and both held the same 2683 MiB. Blender named both, one to a line:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                 NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
```

and `--debug-cycles` added `geometry_bvh.cpp:116] Using MULTI layout.` and one `Added device` line per card.

**Two cards against one, on the same scene and the same samples.** Thirty-six renders, one after another, never overlapping, on a machine with nothing else running. Blender 4.5's own `Time:` summary line, every figure:

| two cards | the card at `0000:00:07` alone | the card at `0000:00:0a` alone |
|---|---|---|
| `00:26.43`, `00:26.70`, `00:25.75` | `00:49.49`, `00:49.90`, `00:49.13` | `00:50.24`, `00:48.77`, `00:48.89` |

The two cards are the same model and either alone gave the same time as the other alone.

**Distribute Memory Across Devices, on a pair that is not linked.** The setting exists in this version, is called `peer_memory`, is a yes-or-no setting, and starts off:

```
peer_memory                      BOOLEAN    default=False        name='Distribute memory across devices'
      description='Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it'
```

It can be turned on and saved even though the cards are not linked — `SAVED STATE: OPTIX peer_memory= True …`, and a separate later run printed `PEER READ BACK: True`. **With it on, nothing changed.** The highest memory each card reached during the same render of an 868 MB scene, sampled five times a second:

```
Distribute Memory OFF, two cards:   card 0: 3557 MiB   card 1: 3557 MiB
Distribute Memory ON,  two cards:   card 0: 3557 MiB   card 1: 3557 MiB
Distribute Memory OFF, one card:    card 0: 3565 MiB   card 1: 3 MiB
```

Render times `Time: 00:08.18`, `Time: 00:08.13` and `Time: 00:11.58`, all exit 0. **Blender said nothing about the setting at all**: searching both streams of every run for `peer`, `nvlink`, `interconnect` and `distribut` matched nothing but the output file's own name. So the setting is offered, accepted, stored and read back on an unlinked pair, and the scene is still put on both cards in full.

**Switching one card off, and what the flag then does.** Three saved states, each rendered with `--debug-cycles`. With only the card at `0000:00:07` ticked and no flag, exit 0 and one card named:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
```

With the same one card ticked and `-- --cycles-device OPTIX`, exit 0 and **both** named. With **every** device unticked and the same flag, exit 0 and both named again:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                 NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
```

So the flag does not merely re-enable the one card the preferences switched off — **it takes every card of that technology**, whatever the preferences said. The flag wrote nothing back; read afterwards the saved preferences still held `[('CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True), ('CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX', False)]`.

**Taking a card away.** A card cannot be unplugged from a machine reached over the network, so what was tried is everything the machine's own tools offer. Hiding one card for one run with `CUDA_VISIBLE_DEVICES=0`: exit 0, one `Added device` line, one card named, `Time: 00:52.63 (Saving: 00:00.26)`. Hiding both with the preferences still naming both: exit 0, image written, and the render fell to the processor with no error — `Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]`. Hiding both **and** asking for the cards anyway: exit 1, no image, `Error: Found no Cycles device of the specified type`. Putting one card into drain state **before** a render: `Successfully set GPU 00000000:00:0A.0 drain state to: draining.`, and the render then named one card and took `Time: 00:49.58 (Saving: 00:00.24)` — the one-card time.

Doing the same **during** a render that was using both cards:

```
--- while the render is running:
index, utilization.gpu [%], memory.used [MiB]
0, 98 %, 2683 MiB
1, 98 %, 2684 MiB
--- attempt 1: nvidia-smi --gpu-reset -i 1
The following GPUs could not be reset:
  GPU 00000000:00:0A.0: In use by another client

1 device is currently being used by one or more other processes (e.g., Fabric Manager, CUDA application, graphics application such as an X server, or a monitoring application such as another instance of nvidia-smi). Please first kill all processes using this device and all compute applications running in the system.
reset exit=255
--- attempt 2: nvidia-smi drain -p 0000:00:0a.0 -m 1
Successfully set GPU 00000000:00:0A.0 drain state to: draining.
drain exit=0
--- attempt 3: nvidia-smi -i 1 -c EXCLUSIVE_PROCESS
No devices were found
compute-mode exit=6
--- attempt 4: nvidia-smi -i 1 -pm 0
No devices were found
persistence exit=6
render exit=0
Saved: '/root/smoozy/two cards/work/out/mid_0001.png'
Time: 00:27.19 (Saving: 00:00.23)
```

A reset is refused outright while the card is in use. The drain state is accepted and takes the card out of `nvidia-smi`'s own listing immediately. **The running render neither failed nor slowed** — it finished in its usual two-card time and wrote its image. The card was put back afterwards.

**What the memory readouts say with two cards.** Blender prints one figure, never two, and never names a card beside a memory number. Its own `Finished` line for the same scene rendered three ways:

```
two cards, Distribute Memory off:  Fra:1 Mem:2524.36M (Peak 2850.07M) | Time:00:07.72 | Mem:480.88M, Peak:494.94M | Scene, ViewLayer | Finished
two cards, Distribute Memory on:   Fra:1 Mem:2524.39M (Peak 2850.07M) | Time:00:07.73 | Mem:480.88M, Peak:494.94M | Scene, ViewLayer | Finished
one card:                          Fra:1 Mem:2512.30M (Peak 2850.07M) | Time:00:11.29 | Mem:1925.82M, Peak:1925.82M | Scene, ViewLayer | Finished
```

and on the timing scene, `Mem:6.98M, Peak:6.98M` with two cards against `Mem:1064.28M, Peak:1064.28M` with one. **The second `Mem:` field is smaller with two cards than with one**, not larger and not doubled. What it counts is not settled here and nothing is read into it. `--debug-cycles` gives one memory block and calls it system memory outright — `scene.cpp:391] System memory statistics after full device sync:` — with `Mapped host memory limit set to 101,923,811,328 bytes. (94.92G)` printed once per card. The cards themselves showed the scene on both, in full, at the same size.

**Frame order and output.** A four-frame animation was rendered with both cards and then with one. The order was ascending both ways:

```
Saved: '/root/smoozy/two cards/work/ord_4.5/f_0001.png'
Saved: '/root/smoozy/two cards/work/ord_4.5/f_0002.png'
Saved: '/root/smoozy/two cards/work/ord_4.5/f_0003.png'
Saved: '/root/smoozy/two cards/work/ord_4.5/f_0004.png'
```

The files were not byte-identical — but neither were two runs on the same cards, and Blender stamps a date into the PNG (`2026/08/06 11:09:48` against `2026/08/06 11:09:58`), so bytes cannot answer this alone. Comparing the pixels instead, of 921,600 values: **57 differed between two cards and one, 14 between two runs on two cards, and 10 between two runs on one card**, and the largest difference in every one of those was `0.003921568393707275` — one step in 255.

### Hand tests on the four-card machine — observed

Run on **2026-08-06** on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each, driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server, run over SSH** — against `/opt/blender/blender-4.5.12-linux-x64/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`. Label: **observed**. **A fourth machine, and a different one from every machine above; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine.** Settings folder `/root/smoozy/four cards/res-4.5`, emptied and remade before use, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender. **Nothing else was running on the machine.** The full list, with every command and every script written out, is `Audits/Blender Research/The machine/Hand tests — four cards.md`.

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
Path tracing on: NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0d_OptiX]
                 NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0b_OptiX]
                 NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:09_OptiX]
                 NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:07_OptiX]
```

and each card was watched separately with `nvidia-smi --query-gpu=index,utilization.gpu,memory.used --format=csv -l 1` during a render in Blender 4.5. All four sat between 97 % and 98 % for the whole render, and each held 854 or 856 MiB:

```
0, 97 %, 856 MiB
1, 97 %, 854 MiB
2, 97 %, 854 MiB
3, 98 %, 856 MiB
```

**One line reads differently from the two-card machine's.** There the rebalancer reported `Number of requested rebalances: 1` and `Number of performed rebalances: 0`. Here it reports 8 requested and 7 performed, so with four cards Blender moved the work between them repeatedly during one render. That is recorded as it printed and nothing is read into it.

**One card, two cards and four cards, on the same scene and the same samples.** Forty-eight renders in all across the four versions, one after another, never overlapping, on a machine with nothing else running. The scene is 1920 by 1080 at 3000 samples, adaptive sampling off, denoising off. **Cycles' own `Total render time:`, in seconds**, three runs of each: four cards `16.9384`, `16.9645`, `17.007`; two cards `32.5951`, `31.8998`, `32.4968`; the card at `0000:00:07` alone `61.9968`, `61.9989`, `62.0295`. **Blender's own `Time:` line**, which counts the whole run: four cards `00:23.34`, `00:23.35`, `00:23.43`; two cards `00:38.81`, `00:38.14`, `00:38.73`; one card `01:08.18`, `01:08.15`, `01:08.22`.

**The four cards are as fast as each other.** The cards at `0000:00:09`, `0000:00:0b` and `0000:00:0d`, one run each, gave `61.2805`, `61.7175` and `60.5389` seconds on Cycles' own figure — a spread of about 2 %.

**Doubling the cards did not halve the time, and quadrupling them did not quarter it.** On Cycles' own figure, one card to two is 1.91 times faster, two to four is 1.91, and one to four is 3.65. On Blender's whole-run figure, where the work of loading the file and building the scene does not shrink at all, the same three are 1.76, 1.66 and 2.92. So "more cards make it proportionally faster" holds in shape and not in arithmetic.

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
four cards: Fra:1 Mem:103.60M (Peak 166.89M) | Time:00:22.74 | Mem:6.98M, Peak:38.62M | Scene, ViewLayer | Finished
two cards:  Fra:1 Mem:95.44M (Peak 158.72M) | Time:00:38.39 | Mem:6.98M, Peak:38.62M | Scene, ViewLayer | Finished
one card:   Fra:1 Mem:91.37M (Peak 154.65M) | Time:01:07.78 | Mem:384.23M, Peak:384.23M | Scene, ViewLayer | Finished
```

**The figure is far smaller with more than one card — and it does not go on shrinking.** Two cards and four cards printed exactly the same figure, on this scene and on the 909 MB one. So it does not count memory divided by the number of cards. What it does count is not settled here and nothing is read into it.

**Not covered here.** Cards of two different makes; a slow card beside a fast one, all four being the same model and within 2 % of each other; an NVLink set; and anything needing eyes on a window, this machine having no display server either.

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on. So this block speaks for the outside case alone, and none of the two-card machine's window tests could be repeated here either.

**Operating system — this block.** Linux, on the machine named at the top of it. It says nothing about macOS, where the project's only machine has one set of built-in graphics, and nothing about Windows, which no Windows machine has run.

**Not covered.** Whether cards of two different makes can be ticked at once — both cards here are the same NVIDIA model. Whether a slow card holds a fast one back — the two are the same model and gave the same time as each other. What Distribute Memory Across Devices does when the cards **are** joined by NVLink. Whether that setting's box is drawn, greyed or hidden on an unlinked pair, and what the Vulkan **Device** control offers with two cards — both need a window, and this machine has no display server. EEVEE's stated lack of support for machines with several graphics cards: EEVEE rendered here with no window on a two-card machine, but nothing in that says which card it used. A card physically removed mid-render.

**Outside the interface.** Everything above was driven from a terminal with no window and no display server: the per-card switches were read and set, the preferences were saved and honoured, `--cycles-device` overrode them, and both cards were used. The sharp new point is that **the individual cards are still only reachable through the saved preferences or Python** — no command-line switch names one for Cycles — and that `--cycles-device` is all-or-nothing: it takes every card of its technology rather than the ones that were ticked.

**Operating system.** Linux only, as with the one-card machine. This second machine agrees with the first on everything the first could reach — the five-entry device list shape, `NONE` as the starting technology, the saved switches being honoured, `--cycles-device` displacing them — and adds only what a second card makes possible. macOS was never asked any of the two-card questions, the Creator's Mac having one set of graphics. Windows is untested.

---

## 6. Can Blender be asked which graphics cards it can see and can use?

**Label: documented, plus observed in supplied Cycles files. Inferred parts are marked.**

Yes, in three different ways, and one of them explicitly covers Cycles' render devices.

**A file Blender writes on request.** The manual lists its sections, and two of them are exactly this question:

> "Save System Info"
> "This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems."
> "You will be prompted to save a text file called system-info.txt."

> "GPU"
> "Shows the GPU vendor, version and the capabilities of your hardware and driver."

> "Cycles"
> "The instruction sets and capabilities of each hardware render device available for use with Cycles."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/help.html — read 2026-07-30

The troubleshooting pages point at the same thing for the narrower question of which card is in use:

> "To find out which graphics card and driver Blender is using, use Help ‣ Save System Info inside Blender. The OpenGL section will have information about your graphics card, vendor and driver version."

— Manual, https://docs.blender.org/manual/en/4.5/troubleshooting/gpu/common/other.html — read 2026-07-30

**The same file can be produced without touching the menu.** It is an operator, so it can be called from a script:

> "bpy.ops.wm.sysinfo(*, filepath='')"
> "Generate system information, saved into a text file"
> "Parameters:"
> "filepath (string, (optional, never None)) – filepath"

— Python API, https://docs.blender.org/api/4.5/bpy.ops.wm.html — read 2026-07-30

**A supplied Cycles file defines direct ways to ask.** It shows the available calls and how their results are used.

Which technologies this copy of Blender was built with:

```
    def get_device_types(self, context):
        import _cycles
        has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()
```

The list of actual cards for a technology, and what each entry holds:

```
    def get_device_list(self, compute_device_type):
        import _cycles
        device_list = _cycles.available_devices(compute_device_type)
```
```
    def get_devices_for_type(self, compute_device_type, device_list=None):
        # Layout of the device tuples: (Name, Type, Persistent ID)
```

Reading the rest of the code shows the tuple carries more than those three: index 3 is used for peer memory, index 4 for hardware ray tracing, index 5 for OpenImageDenoise support, index 6 for the OptiX denoiser, index 7 for whether the kernels are optimised (`properties.py` lines 1683, 1752, 1768, 1882–1887). Naming what those later fields *mean* is **inferred** from how they are used, not documented — the step is reading each index's single use site and taking the name of the variable it is assigned to.

The stored, persistent view of each card:

```
class CyclesDeviceSettings(bpy.types.PropertyGroup):
    id: StringProperty(name="ID")
    name: StringProperty(name="Name")
    use: BoolProperty(name="Use", default=True)
    type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
```

And ready-made questions with plain answers:

```
    def has_active_device(self):
        return self.get_num_gpu_devices() > 0
```

alongside `get_num_gpu_devices()`, `has_multi_device()`, `has_oidn_gpu_devices()`, `has_optixdenoiser_gpu_devices()` and `get_compute_device_type()`.

— `Blender Sources/cycles_4.5/properties.py`, lines 1548–1552, 1567–1569, 1639–1641, 1673–1674, 1709–1772 — read 2026-07-30

**But the list is not gathered by itself, and asking for it carries a stated risk:**

```
    # Refresh device list. This does not happen automatically on Blender
    # startup due to unstable drivers that can cause crashes.
    def refresh_devices(self):
```
```
    # Be careful when deciding when to call this function,
    # as Blender can crash with `_cycles.available_devices()` on some drivers.
```

— `Blender Sources/cycles_4.5/properties.py`, lines 1693–1695 and 1637–1638 — read 2026-07-30

The older way of asking has been kept but hollowed out:

```
    # Deprecated: use refresh_devices instead.
    def get_devices(self, compute_device_type=''):
        self.refresh_devices()
        return None
```

— `Blender Sources/cycles_4.5/properties.py`, lines 1704–1707 — read 2026-07-30

The Python API change log quoted under question 5 names `get_devices_for_type`. The searches recorded below found no full Python API documentation for these functions.

**The card Blender is drawing with can be asked separately** — one card, not a list:

> "gpu.platform.backend_type_get()"
> "Get actuve GPU backend."
> "Backend type ('OPENGL', 'VULKAN', 'METAL', 'NONE', 'UNKNOWN')."

> "gpu.platform.device_type_get()"
> "Device type ('APPLE', 'NVIDIA', 'AMD', 'INTEL', 'SOFTWARE', 'QUALCOMM', 'UNKNOWN')."

> "gpu.platform.renderer_get()"
> "Get GPU to be used for rendering."

> "gpu.platform.vendor_get()"
> "Get GPU vendor."

> "gpu.platform.version_get()"
> "Get GPU driver version."

— Python API, https://docs.blender.org/api/4.5/gpu.platform.html — read 2026-07-30

**Where I looked, for what is missing.** I searched `manual_4.5`, `api_4.5` and `release_notes_4.5` for `available_devices`, `get_devices`, `refresh_devices`, `has_active_device`, `No compatible GPUs found`. The only hit in any of the three is the change-log line for `get_devices_for_type`. **These functions are not written up in the manual or in the Python API documentation at all** — they exist and are usable, but the only description of them is the Cycles file itself. I also searched the full command-line arguments page for `device` and `gpu`; there is **no command-line switch that lists graphics cards** (`-E help` lists render engines, not devices).

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 4.5.

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

**Asking Cycles directly, from Blender's Python Console** (Scripting workspace, window open). The add-on key was found to be the literal string `cycles`. With `p = bpy.context.preferences.addons["cycles"].preferences`, the expression `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned `[]` before anything else was called, and after `p.refresh_devices()` returned:

```
[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

`p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, and `p.has_multi_device()` returned `False`. So on this machine the identifier the sources call the "Persistent ID" was the string `CPU` for the processor and `METAL_Apple M1 Pro (GPU - 14 cores)` for the graphics card. One run in one version records nothing about whether those identifiers stay the same across restarts or machines.

**`gpu.platform`, from the same console.** `backend_type_get()` returned `'METAL'`; `device_type_get()` returned `'APPLE'`; `renderer_get()` returned `'Metal API'`; `vendor_get()` returned `'Apple M1 Pro'`; `version_get()` returned `'1.2'`. `gpu.platform.devices_get()` raised:

```
AttributeError: module 'gpu.platform' has no attribute 'devices_get'
```

so that function is not present in 4.5.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, which reports `Blender 4.5.12 LTS`.

**Save System Info with no window open.** Command:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='/tmp/sysinfo-terminal-45.txt'))"
```

printed `RESULT: {'FINISHED'}` and the file was written. Its `GPU` section, in full, was one line:

```
GPU: missing, background mode
```

Its `Cycles` section, in full:

```
Cycles:
=========================================


CPU device capabilities: 

Metal device capabilities:
Number of devices: 1
		Device: Apple M1 Pro (GPU - 14 cores)
```

**The `gpu` module with no window open.** `-b --factory-startup --python-expr "import gpu; print('INIT:', gpu.init())"` raised:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
AttributeError: module 'gpu' has no attribute 'init'
```

and `-b --factory-startup --python-expr "import gpu; print('BACKEND:', gpu.platform.backend_type_get()); print('RENDERER:', gpu.platform.renderer_get()); print('VENDOR:', gpu.platform.vendor_get()); print('VERSION:', gpu.platform.version_get())"` raised:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
SystemError: GPU functions for drawing are not available in background mode
```

So in 4.5 there is no `gpu.init`, and `gpu.platform` does not answer with no window.

**The Cycles device list with no window open** is quoted in full under question 5. Its shape here: the add-on key printed as `['cycles']`, the stored list was already populated before `refresh_devices()` was called, and each raw `_cycles.available_devices('METAL')` entry was an eight-field tuple — `('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False, True, True, False, True)` for the card. `_cycles.available_devices()` with no argument raised `TypeError: function takes exactly 1 argument (0 given)`. The identifiers were the same in every run of the day.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-4.5.12-linux-x64/blender`.

**Save System Info with no window open.** `-b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='…'))"` printed `RESULT: {'FINISHED'}` and wrote the file. Its whole `GPU` section is one line, exactly as on the Mac:

```
GPU: missing, background mode
```

Its `Cycles` section is where Linux differs, and it is long — about ninety lines of card capabilities. Its opening and the two lines that carry the answer:

```
Cycles:
=========================================


CPU device capabilities: AVX2

CUDA device capabilities:
	NVIDIA GeForce RTX 4090
		CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK			1024
		…
		CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR			8
		CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR			9
		…
```

So on Linux with an NVIDIA card the report names the card's compute capability outright, and the processor's instruction set (`AVX2`) where the Mac's line was empty. Only the technology whose cards were found gets a section: with the technology saved as OptiX the file still headed the section `CUDA device capabilities` and carried nothing headed OptiX. The headings in the file, in order, were `Blender:`, `Python:`, `paths:`, `Python (External Binary):`, `Directories:`, `scripts:`, `pref scripts:`, `FFmpeg:`, `Other Libraries:`, `GPU: missing, background mode`, `Cycles:`, `Enabled add-ons:` — and, as on the Mac, **there is no section headed `OpenGL`**.

**Asking Cycles' compiled part directly.** `-b --factory-startup --python-expr "import _cycles; …"` printed:

```
TYPES: (True, True, True, False, True, True)
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

Three things. Asking for a technology with no card returns the processor alone and no error, whether that technology is built in (HIP, oneAPI) or not (Metal). Each entry is an eight-field record, matching the Mac. And this copy of Blender did not crash on the call, despite the warning in Cycles' own comments.

**What Blender was built with, against what it can find.** Hiding the card for one command with `CUDA_VISIBLE_DEVICES=` left `TYPES:` unchanged — CUDA and OptiX still reported as built in — while both lists came back holding the processor alone. A render then asked for CUDA exited `1`, wrote no image, and printed `Error: Found no Cycles device of the specified type`. So Blender answers *what am I built with* and *what can I see now* separately.

**The graphics module with no window open.** `-b --python-expr "import gpu; print('INIT:', gpu.init())"` raised:

```
AttributeError: module 'gpu' has no attribute 'init'
```

and the platform calls raised:

```
SystemError: GPU functions for drawing are not available in background mode
```

Word for word what the Mac gave. So `gpu.init` is absent and `gpu.platform` refuses with no window, on both machines, in this version.

**There is no `--gpu-device` argument in this version** — see question 5 for what Blender did with it.

**Not covered.** Whether Cycles' device query can be run without first settling on a technology, since `get_device_list` takes one as an argument and `refresh_devices` simply loops over all five. What the "Persistent ID" is made of — on Linux it visibly carries the card's place on the bus, which is a hint and not a statement, and whether it survives a driver update or a move is untested on both machines. Whether the crash risk named in the comments is still live in 4.5 — it did not fire on either machine, which is not the same as it being gone. What any of this returns on a machine with several cards.

**Outside the interface.** Documented: `bpy.ops.wm.sysinfo(filepath=...)` can be called from a script to write system information. Inferred: the direct Cycles queries in the supplied file can also be called from an outside script; the reasoning step is that they are methods on the preferences object reached through `bpy.context`. Two cautions are stated in the file: refreshing the device list "does not happen automatically on Blender startup", and refreshing can crash Blender on some drivers. The file does not establish that an unrefreshed list is empty or stale. Both halves are now tested on two machines and they agree. `bpy.ops.wm.sysinfo(filepath=…)` returned `{'FINISHED'}` and wrote its file with no window open on both, its `GPU` section reduced to `GPU: missing, background mode` on both while its `Cycles` section was fully populated on both. The Cycles queries answered with no window on both. `gpu.platform` does **not** answer with no window in this version on either machine, and `gpu.init` does not exist on either to fix that.

**Operating system.** Not distinguished in the sources. `Save System Info`, `wm.sysinfo`, the Cycles queries and the `gpu.platform` module are described without any per-system qualification. Two systems are now tested and the sources hold for both. On macOS all the routes answered, `gpu.platform` reporting backend `'METAL'` and device type `'APPLE'`. On the rented Linux machine the same routes answered, the Cycles section of the report carrying a long CUDA capability dump instead of the Mac's short Metal one, and `gpu.platform.devices_get` did not exist on either. Windows remains untested.

### Hand tests on the two-card machine — observed

Run on **2026-08-06** on the second rented Linux machine described under question 5 — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s on driver 580.95.05, sixty-one cores, **no display server at all**, over SSH — against `/opt/blender/blender-4.5.12-linux-x64/blender`. Label: **observed**. Not merged with the one-card rented machine's results or the Mac's.

**Every route answered, and every one named both cards.** Asked with no window and no display server:

```
BACKEND: OPTIX
TYPES: (True, True, True, False, True, True)
CUDA: (('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', False, False, True, False, True), ('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a', False, False, True, False, True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False, False, True, False, True))
OPTIX: (('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', False, False, True, True, True), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX', False, False, True, True, True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False, False, True, False, True))
HAS ACTIVE: True
NUM GPU: 2
MULTI: False
```

Three things in that. Each technology's list holds **both** cards and the processor. **`get_num_gpu_devices()` counts the cards and returns 2.** And **`has_multi_device()` returns `False` with two cards ticked** — so that name is not about having more than one card, whatever it reads like.

**The system-information file lists the card twice.** `bpy.ops.wm.sysinfo(filepath=…)` returned `{'FINISHED'}` and wrote its file with no display server. Its `Cycles` section carries `CPU device capabilities: AVX2`, one heading `CUDA device capabilities:`, and then the card's name twice — once at line 83 of the file and once at line 172 — each followed by its own block of capabilities. There is still no section headed `OpenGL`.

### Hand tests on the four-card machine — observed

Run on **2026-08-06** on the third rented Linux machine described under question 5 — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, sixty-one cores, 197 GB of memory, **no display server**, over SSH — against `/opt/blender/blender-4.5.12-linux-x64/blender`. Label: **observed**. A fourth machine; not merged with the Mac's results, the one-card machine's or the two-card machine's. The full list is `Audits/Blender Research/The machine/Hand tests — four cards.md`.

**Every asking route named four cards.** With all four ticked under OptiX and saved, `blender -b --python scripts/ask.py` printed:

```
BACKEND: OPTIX
TYPES: (True, True, True, False, True, True)
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

**Not covered.** What any of this returns on a machine with cards of two different makes, or with an NVLink pair. What the Status Bar or any panel shows, this machine having no display server at all.

**Outside the interface.** Settled again, on a machine with two cards and no display server: `wm.sysinfo` wrote its file, the Cycles queries answered, and both cards were named by every route. Nothing needed a window.

**Operating system.** Linux, as before. This second machine agrees with the one-card machine wherever the two could be asked the same thing, and adds only what a second card shows. Windows is untested.

---

## Things a person with Blender 4.5 open could settle in a few minutes

This list has since been run on three machines. What the Creator's own MacBook Pro settled is written into the answers above as **Hand test — observed**, what the one-card rented Linux machine settled as **Hand tests on the rented Linux machine — observed**, and what a second rented Linux machine with two graphics cards and no display server settled as **Hand tests on the two-card machine — observed**. The three lists of tests live at `Audits/Blender Research/The machine/Hand tests.md`, `Audits/Blender Research/The machine/Hand tests — Linux.md` and `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`. What follows is what is still open, and what cannot be settled on the equipment available. **A fourth machine has since been used as well** — a third rented Linux machine with **four** graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the four-card machine — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

### Still open

Nothing on this list is still open. Every item either machine could reach has been run, and what came back is recorded under questions 1, 2, 3, 4, 5 and 6 above. Three of the four items that stood after the Mac run are now closed by the Linux machine: which Intel and AMD requirement lines the panel shows, whether the "No compatible GPUs found for Cycles" text ever reaches the console, and what Blender prints when it cannot open a window. What is left is below.

### Cannot be run on the equipment available

The equipment is one MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, whose graphics are Apple's own and built in; one rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores — reached over SSH; and a second rented Linux machine — Ubuntu 22.04.5 LTS, **two** NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, **no display server at all** — also over SSH. No Windows machine has run it, there is no AMD or Intel graphics card and no pair of cards joined by NVLink, and nothing on either rented machine may be installed or removed. **Two corrections to that sentence, made 2026-08-06 by the citation check of the two-card material.** The one-card rented Linux machine was destroyed on 2026-08-06 and nothing can ever be run on it again, so every result from it stands as a record and none of it can be repeated. And something *was* removed and reinstalled on the two-card machine: its NVIDIA driver was purged on purpose, and then reinstalled into a mismatched state, for the broken-driver test recorded under question 2. That machine has since been restarted and did not answer when the citation check tried it on 2026-08-06.

**A fourth machine has since been used, and it is the only one still alive.** A **third rented Linux machine** — Ubuntu 22.04.5 LTS, **four** NVIDIA GeForce RTX 4060 Ti of **16 GB each** on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, **197 GB of memory**, **no display server**, over SSH. What it settled is written into the answers above as **Hand tests on the four-card machine — observed**, and its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. Three limits above are narrowed by it and none is removed: more than two cards is no longer out of reach, a card small enough to be filled is no longer out of reach, and the several-card form of every question under 5 and 6 has now been asked twice on two different machines. **Still out of reach on it as well:** any pair joined by NVLink, these four talking through the host bridge; cards of two different makes; cards of unequal speed, all four being the same model within about 2 % of each other; and anything needing eyes on a window, this machine having no display server either. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

- **Which Metal requirement line the panel shows.** Needs an Apple Silicon Mac running macOS 12.x. On macOS 26.5.2 a Metal device is always found. The Intel and AMD Linux lines are now recorded under question 2; the Intel **Windows** line needs a Windows machine, the panel text branching on the operating system.
- **Whether the figures the panel draws are enforced.** Needs a real AMD or Intel card, and a driver below the stated figure.
- **Whether the compute-capability floor is 3.0.** Needs a card between 3.0 and 5.0. Every card the project can reach reports major 8, minor 9, so the requirement line is never drawn for CUDA or OptiX.
- **Whether two cards of different makes can both be used.** The two-card machine has two of the same NVIDIA model. Most of what needed a second card is now answered under question 5; this is not.
- **What Distribute Memory Across Devices does when the cards are joined by NVLink.** The two-card machine's pair talks through the host bridge. What the setting does on an unlinked pair is now recorded under question 5.
- **Whether a slow card holds a fast one back.** Both cards on the two-card machine are the same model and gave the same time as each other.
- **Anything needing eyes on a window with two cards** — whether the Distribute Memory Across Devices box is drawn or greyed, and what the Vulkan **Device** control offers. The machine that has two cards has no display server; the machine that had a display had one card.
- **What Blender prints when it will not start for want of OpenGL 3.3.** Needs a machine below that floor. What it prints when there is no display server at all is now settled under question 1, and that is a different case.
- **Whether EEVEE really cannot render headless on Windows.** macOS and Linux are now both tested, including on a machine with no display server at all, and it renders on all of them. Windows needs a Windows machine and none was rented for it.

---

2026-08-01 — Claude Opus 5 (Claude Code) — The machine, Blender 4.5

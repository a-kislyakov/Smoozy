# The machine — second pass, Blender 4.5

**Sources consulted, and when.** Blender's 4.5 manual (plain text plus one picture from its `_images` folder), Blender's 4.5 Python API documentation, Blender's 4.5 release notes, and Cycles' own files as shipped with 4.5 — all read on **2026-07-30**. Blender's requirements page, read on **2026-08-01**.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

Answering questions 1, 2, 3, 4 and 5 only. Question 6 is not answered here.

---

## A note that applies to questions 1, 2 and 3

The requirements page carries no version in its address. Under the rule I was given, it speaks for 4.5 **only where it names 4.5 in so many words**. I checked every line of the file. Exactly four lines name Blender 4.5:

1. "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."
2. Under *Requirements for Older Versions → Windows*: "Blender 4.5 LTS supports NVIDIA GeForce 400 and newer. AMD GCN 1st gen and newer. Intel Broadwell architecture and newer."
3. Under *Requirements for Older Versions → macOS*: "Blender 4.5 LTS supports Intel chips, macOS 11.2 (Big Sur), requires GPU with Metal 2.2 (AMD, Apple Silicon, or Intel Skylake and newer)"
4. Under *Requirements for Older Versions → Linux*: "Blender 4.5 LTS supports NVIDIA GeForce 400 and newer. AMD GCN 1st gen and newer. Intel Broadwell architecture and newer."

Everything else on that page — the whole minimum/recommended tables for Windows, macOS and Linux (operating system versions, processor cores, SSE4.2, memory, video memory, OpenGL 4.3, Vulkan 1.3, glibc 2.28), and the "Runs Anywhere", "Truly Portable" and "Compatible" prose at the top — **names no version at all**. So none of it can be reported as 4.5's requirements. This is the single biggest gap in questions 1 and 2, and I say so again inside each answer rather than expecting the reader to carry it down.

---

## 1. What must a computer have on it before Blender will run at all?

**Answer.** The sources give a partial list, and the most obvious source of a complete list — the requirements tables — cannot be used for 4.5 (see the note above).

What the sources *do* say for 4.5:

**Graphics hardware and driver.** Blender needs a graphics card and driver that reach a minimum OpenGL level, and the manual names that level while explaining an error message Blender shows:

> "This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

The same page adds why: "Blender uses OpenGL for the 3D Viewport and user interface. The graphics processing unit (GPU) and driver have a big impact on Blender's behavior and performance."

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/troubleshooting/gpu/windows/nvidia.html (the identical wording appears on all twelve of the per-platform, per-vendor troubleshooting pages under `troubleshooting/gpu/`, and in the shared fragment `troubleshooting/gpu/common/other.txt`). Read 2026-07-30. The "Blender uses OpenGL…" sentence is also at https://docs.blender.org/manual/en/4.5/troubleshooting/gpu/index.html.

Note plainly: the requirements page's table row says "2 GB VRAM with OpenGL 4.3, Vulkan 1.3" — a different number from the manual's 3.3 — but that row names no version, so it cannot be attributed to 4.5. I am reporting both and resolving neither.

**Supported graphics card generation, for 4.5 by name.** From the requirements page, in the section that does name the version:

> "Blender 4.5 LTS supports NVIDIA GeForce 400 and newer. AMD GCN 1st gen and newer. Intel Broadwell architecture and newer."

That exact sentence appears twice — once under Windows, once under Linux.

For macOS, again naming the version:

> "Blender 4.5 LTS supports Intel chips, macOS 11.2 (Big Sur), requires GPU with Metal 2.2 (AMD, Apple Silicon, or Intel Skylake and newer)"

and

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur)."

*Label: documented.* Source: requirements page, https://www.blender.org/download/requirements/, read 2026-08-01.

Ambiguity I am flagging rather than resolving: those "supports …" lines sit under a heading about older versions and do not say whether they describe what is needed to *run* Blender, or what is needed to *render on the card*. I read them as the former because they sit in a requirements page beside operating system and memory rows, but the other reading is available.

**A running-environment on Linux.** The manual is explicit:

> "On Linux Blender supports both X11 and Wayland for official releases."
> "When Wayland is detected, it is the preferred system, otherwise X11 will be used."

and, for one desktop:

> "Under Gnome-Shell the libdecor library is required. This is available as a package on most Linux distribution."
> "If the library isn't found X11 will be used as a fallback."

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux_windowing_environment.html. Read 2026-07-30.

**Things that are *not* needed.** The manual states Blender carries its own Python and does not need an internet connection:

> "Blender bundles a complete Python installation and does not interact with the system Python by default."

(https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html)

> "Blender is designed to not require an internet connection, so it doesn't have a built-in update system."

(https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html)

> "Blender downloads can be extracted to any directory on the system, as a self contained installation."

(https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html)

*Label: documented.* Read 2026-07-30.

**Two non-hardware conditions Blender's own troubleshooting names.** The startup page lists, among the common causes of Blender failing to start:

> "Make sure any antivirus software is not preventing Blender from starting."
> "Ensure that you have appropriate user permissions on the computer."

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html. Read 2026-07-30.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 4.5 started and ran normally, and **Help ‣ Save System Info** was used to write `system-info.txt`. The file's section headed `GPU:` read:

```
renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:
```

followed by a section headed `Implementation Dependent GPU Limits:` reading `Maximum Batch Vertices:	-2147483648` and `Maximum Batch Indices:	-2147483648`. Only the `GPU` and `Cycles` sections were copied back, so nothing here records the rest of the file, and in particular nothing records whether it carries any section naming an OpenGL version. This does not test the OpenGL floor, and does not settle the 3.3-against-4.3 disagreement above.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — `/opt/blender/blender-4.5.12-linux-x64/blender` (`Blender 4.5.12 LTS`) started and ran. The machine is a container sharing somebody else's hardware, not a whole computer, and other work was running on it at the same time, so nothing here says how long anything took or how much memory was free.

Three things bearing on this question.

**With no display reachable, Blender still renders.** *(Wording corrected 2026-08-06 by the citation check of the two-card material. This machine had a real display server running; the runs were cut off from it by removing `DISPLAY`. A machine with genuinely no display server is a different machine, recorded in the block at the end of this answer. Nothing observed changes.)* `DISPLAY` was removed from the environment and a background Cycles render completed at exit `0` on the graphics card, and so did an EEVEE render. So the manual's X11-or-Wayland requirement is about opening a window, not about running.

**Asked for a window with no display reachable, it refuses, and in this version it then hangs.** The command `env -u DISPLAY <blender> --python-expr "import bpy; print('WINDOW OPENED')"`, wrapped in `timeout 60`, never printed `WINDOW OPENED` and printed instead:

```
GHOST: failed to initialize display for back-end(s): ['WAYLAND', 'X11']
  'WAYLAND': unable to connect to display!
  'X11': unable to open a display!
GHOST: unable to initialize, exiting!
```

It then did not exit; the timeout killed it, status `124`. In 5.0, 5.1 and 5.2 the same test exits with status `1` — but that is another version's finding and is recorded there.

**The OpenGL floor is still not tested.** The rented machine reports `4.6.0 NVIDIA 580.105.08`, above both figures the sources give, so the 3.3-against-4.3 disagreement stands.

**Not covered.** The sources I was given never state, for 4.5 specifically, a minimum operating system version, a minimum processor or instruction set, a minimum amount of memory, a minimum amount of video memory, or a minimum system library version. The manual defers those to the download page ("Check the Downloading Blender page to find the minimum requirements"), and the requirements page I hold does not name 4.5 anywhere it gives those figures. Nothing here settles whether Blender 4.5 will start on a machine with no graphics card at all, nor what happens on a machine below the stated OpenGL level (the manual describes an error message but not whether Blender then exits or continues).

**Outside the interface.** Now observed on Linux as well as documented: with `DISPLAY` removed from the environment, 4.5 rendered both with Cycles on the graphics card and with EEVEE, at exit `0`, on the rented Linux machine. So the manual's claim holds here in fact and not only on paper. Partly different, and the manual says so. Rendering from the command line "we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)" (https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, documented, read 2026-07-30). Against that, the manual also states "Headless rendering is not supported on headless Windows systems" (https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html, documented, read 2026-07-30) — though that sentence sits inside EEVEE's limitations, so it is a statement about EEVEE and not plainly about Blender as a whole. The sources do not say whether the OpenGL requirement above still applies when Blender is started with `--background`.

**Operating system.** The requirements page's 4.5-naming lines differ by system: the same graphics-generation sentence for Windows and Linux, a separate Metal-and-macOS sentence for macOS. The Linux windowing requirements (X11/Wayland/libdecor) are Linux-only by their own wording. The OpenGL-3.3 sentence appears identically on the Windows, Linux and macOS troubleshooting pages, so it is stated for all three.

**Hand tests on a machine with no display server at all — observed.** On **2026-08-06**, on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-4.5.12-linux-x64/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`. Label: **observed**. Not merged with the one-card rented machine's results or the Creator's Mac.

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
GPU  CYCLES exit=0  png=4957
GPU  BLENDER_EEVEE_NEXT exit=0  png=3400
GPU  BLENDER_EEVEE exit=124  png=no file  Error: engine not found 'BLENDER_EEVEE'
GPU  BLENDER_WORKBENCH exit=0  png=3613
CPU  CYCLES exit=0  png=4957
CPU  BLENDER_EEVEE_NEXT exit=0  png=3400
CPU  BLENDER_EEVEE exit=124  png=no file  Error: engine not found 'BLENDER_EEVEE'
CPU  BLENDER_WORKBENCH exit=0  png=3613
```

`BLENDER_EEVEE` is not an engine this version has, and asked for it 4.5 printed its error and then did not end, killed at 120 seconds.

The same runs repeated with `--debug-cycles` showed the work really was where it was asked for:

```
on the card:      Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                                   NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
on the processor: Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]
```

**Asked for a window on a machine with no display server, Blender said so and then did not end:**

```
GHOST: failed to initialize display for back-end(s): ['WAYLAND', 'X11']
  'WAYLAND': unable to connect to display!
  'X11': unable to open a display!
GHOST: unable to initialize, exiting!
```

It never printed the line the script asked it to print, wrote this to **standard error**, and was killed at 120 seconds, exit status **124**. Word for word what the one-card machine printed when cut off from its display, and the same exit status, so for this version the two cases are indistinguishable in what Blender says and does.

`-b -E help` hung here too, exit `124` at sixty seconds, but on this machine it printed its listing first — `BLENDER_EEVEE_NEXT`, `BLENDER_WORKBENCH`, `CYCLES`. On the one-card machine it printed nothing before hanging; on the Creator's Mac it printed and stopped. All three results stand and none is chosen between here.
**Not covered.** Windows. Nothing about the OpenGL floor: this machine's cards are far above it.

**Where I looked.** Searched the whole 4.5 manual for: `requirements`, `minimum`, `glibc`, `SSE4`, `64-bit`, `x86_64`, `arm64`, `OpenGL`, `Vulkan`, `driver version`, `prerequisite`, `dependenc`, `administrator`, `X server`, `display server`, `headless`, `environment variable`. Searched the 4.5 API documentation and the 4.5 release notes for the same terms. Opened and read in full: `getting_started/installing/index.txt`, `.../linux.txt`, `.../macos.txt`, `.../windows.txt`, `.../steam.txt`, `.../linux_windowing_environment.txt`, `troubleshooting/startup.txt`, `troubleshooting/crash.txt`, `troubleshooting/gpu/index.txt` and all files under `troubleshooting/gpu/`, `advanced/deploying_blender.txt`, `advanced/blender_directory_layout.txt`, `advanced/command_line/index.txt`, `.../render.txt`, `.../arguments.txt`, `.../launch/{linux,macos,windows}.txt`. Read the whole requirements file line by line. **Ruled out:** the release notes carry nothing on minimum system requirements (searched all nineteen files for `requirement`, `minimum`, `glibc`, `OpenGL 4.3`, `Vulkan 1`; the only "minimum" hits are the OptiX driver bump, a Grease Pencil thickness change and a Timeline marker height). The API documentation carries no requirements page. Cycles' own files carry nothing about running Blender itself.

---

## 2. What must a computer have on it before Blender will render using its graphics cards?

**Answer.** For Cycles the manual is detailed and explicit. For EEVEE it says less.

**Cycles — the switch.**

> "To enable GPU rendering, go into the Preferences ‣ System ‣ Cycles Render Devices, and select either CUDA, OptiX, HIP, oneAPI, or Metal. Next, you must configure each scene to use GPU rendering in Properties ‣ Render ‣ Device."

**Cycles — per technology, quoted exactly:**

> "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 3.0 and higher."

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."

> "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."

with supported cards listed as "Radeon RX 5000 Series / Radeon RX 6000 Series / Radeon RX 7000 Series / Radeon RX 9000 series / Radeon Pro W6000 Series / Radeon Pro W7000 Series" and minimum drivers "Windows: Radeon Software 24.6.1 or Radeon PRO Software 24.Q2 / Linux: Radeon Software 23.40 or ROCm 6.0".

> "oneAPI is a computation library that is supported on Windows and Linux and requires a Intel® Arc™ graphics card with the Xe HPG architecture."

with supported cards "Intel® Arc™ A-Series / Intel® Arc™ B-Series" and minimum drivers "Windows: Intel Graphics Driver XX.X.101.5518 / Linux: intel-level-zero-gpu package 1.3.27642, typically available through the intel-compute-runtime package XX.XX.27642.38".

> "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

And on drivers generally:

> "In case of problems, be sure to install the official graphics drivers from the GPU manufacturers website, or through the package manager on Linux. The graphics drivers provided by the computer manufacturer can sometimes be outdated or incomplete."

And, importantly, a statement that the toolkit is *not* needed:

> "Normally users do not need to install the CUDA toolkit as Blender comes with precompiled kernels."

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html. Read 2026-07-30. The switch is also stated at https://docs.blender.org/manual/en/4.5/render/cycles/introduction.html ("For GPU accelerated rendering, enable compatible devices in Preferences ‣ System ‣ Cycles Render Devices.") and https://docs.blender.org/manual/en/4.5/editors/preferences/system.html.

**The OptiX driver number is a stated change in 4.5.** The release notes say:

> "The minimum driver version required for OptiX has increased to 535."

*Label: documented.* Source: release notes, https://developer.blender.org/docs/release_notes/4.5/cycles/ and https://developer.blender.org/docs/release_notes/4.5/. Read 2026-07-30.

**Cycles' own files carry the same requirements as on-screen text.** When no usable card is found, Cycles builds these lines:

```
1796:            col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)
1798:            if device_type == 'CUDA':
1799:                compute_capability = "3.0"
1802:            elif device_type == 'OPTIX':
1803:                compute_capability = "5.0"
1804:                driver_version = "535"
```
and, for the remaining technologies, "Requires AMD GPU with RDNA architecture" with `adrenalin_driver_version = "24.6.1"` / `pro_driver_version = "24.Q2"` on Windows and `driver_version = "23.40"` on Linux; "Requires Intel GPU with Xe-HPG architecture" with `driver_version = "XX.X.101.6557"` on Windows and `"XX.XX.31740.15"` plus "oneAPI Level-Zero Loader" on Linux; and "Requires Apple Silicon with macOS %s or newer" with `mac_version = "12.2"`.

*Label: observed.* File: `Blender Sources/cycles_4.5/properties.py`, lines 1794–1852 (inside `CyclesPreferences._draw_devices`). Read 2026-07-30.

Two of these numbers differ from the manual, and I am reporting the difference without resolving it: the file says macOS **12.2** for Metal where the manual says "macOS 13.0 or newer is required to support all features"; and the file says Intel Windows driver **XX.X.101.6557** where the manual says **XX.X.101.5518**.

**Which technologies a given build even offers is decided at run time.** Cycles asks the compiled part of itself which back-ends exist, and only lists those:

```
1567:    def get_device_types(self, context):
1568:        import _cycles
1569:        has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()
1570:
1571:        list = [('NONE', "None", n_("Do not use compute device"), 0)]
1572:        if has_cuda:
1573:            list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))
```

*Label: observed.* File: `Blender Sources/cycles_4.5/properties.py`, lines 1567–1583. Read 2026-07-30. The manual states the same idea in words: "If the system has a compatible NVIDIA CUDA device, it will be available as an option for rendering with Cycles." (https://docs.blender.org/manual/en/4.5/editors/preferences/system.html)

**EEVEE.** The manual says EEVEE has no CPU path at all:

> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient."

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html. Read 2026-07-30.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 4.5's **Edit ‣ Preferences ‣ System** page was opened and the **Cycles Render Devices** panel expanded. The row of technology buttons held exactly two, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — and two dropdown lists: **Kernel Optimization**, set to **Full**, offering **Off** and **Intersection Only**; and **MetalRT**, set to **Auto**, offering **Off** and **On**. Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed. No "Requires Apple Silicon with macOS … or newer" line appeared, because a Metal device was found, so this run records nothing about the macOS 12.2-against-13.0 disagreement above.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, three things were put to 4.5.

**A Cycles graphics-card render works with no window and with no display reachable.** With the technology saved as `CUDA` and the card ticked, a background render exited `0`, wrote its PNG and reported `Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]`. With `OPTIX` saved, `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) […]`. With `DISPLAY` removed from the environment entirely, the same thing. That machine still had a display server running; the same on a machine with none at all is recorded under question 1. So a Cycles GPU render needs no display attached and no desktop session on Linux — one of the things this answer said the sources never state.

**The missing-requirement lines, which the Mac could never reach.** With a window open on the machine's own display and the technology set to one this machine has no card for, 4.5 drew, under **oneAPI**:

```
No compatible GPUs found for Cycles
Requires Intel GPU with Xe-HPG architecture and
  - intel-level-zero-gpu or intel-compute-runtime version
    XX.XX.31740.15 or newer
  - oneAPI Level-Zero Loader
```

and under **HIP**:

```
No compatible GPUs found for Cycles
Requires AMD GPU with RDNA architecture
and AMD driver version 23.40 or newer
```

So the Intel Linux figure a running 4.5 draws is `XX.XX.31740.15` — Cycles' own shipped number, not the manual's `XX.XX.27642.38`. This records what is displayed on a machine with no Intel card; it does not test what Blender enforces against a real one. The 4.5 HIP text names only the Radeon driver, where 5.0 onwards also name a ROCm HIP Runtime figure.

**The technology row on Linux.** Five buttons — **None**, **CUDA**, **OptiX**, **HIP**, **oneAPI** — and no Metal, so the Metal minimum-version conflict cannot be reached here either.

**Not covered.** The sources give no minimum amount of video memory for GPU rendering that can be attributed to 4.5 (the requirements page's "2 GB VRAM" row names no version). They do not say what Blender needs installed for EEVEE to render on a card beyond having a card — no driver versions, no minimum card generation for EEVEE specifically. The Metal minimum version conflict above is unresolved and needs a Mac on macOS 12.x.

**Outside the interface.** The manual's requirements are stated for the settings, not for the window, and the same page documents a command-line way to reach the setting (`--cycles-device`, see question 5) — so on the face of it they hold either way. But the sources never say so, and never say whether a background render needs the same operating-system graphics environment. One documented Linux difference does bear on it: the "Unsupported GNU version" fix on that same page is given as a command-line environment variable, `CYCLES_CUDA_EXTRA_CFLAGS="-ccbin gcc-x.x" blender`, which only makes sense outside the interface.

**Operating system.** Sharply different, and stated. CUDA, OptiX, HIP and oneAPI are each said to be "supported on Windows and Linux". Metal is Apple Silicon and macOS only. So on macOS the only Cycles GPU route named is Metal, and on Windows and Linux the only routes named are CUDA, OptiX, HIP and oneAPI. Minimum driver numbers for HIP and oneAPI are given separately for Windows and Linux. Two systems are now tested and both match. On the Creator's Apple Silicon Mac the technology row offered **None** and **Metal** and nothing else. On the rented Linux machine it offered **None**, **CUDA**, **OptiX**, **HIP** and **oneAPI** and no Metal, and drew the Linux driver figures. Windows remains untested, and the Windows figures cannot be read from Linux because the panel text branches on the operating system; a rented Windows machine would show them and none was used. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Where I looked.** Read `render/cycles/gpu_rendering.txt` in full, plus `render/cycles/introduction.txt`, `render/cycles/index.txt`, `render/cycles/render_settings/index.txt`, `render/cycles/render_settings/performance.txt`, `render/cycles/render_settings/sampling.txt`, `editors/preferences/system.txt`, `render/eevee/limitations/limitations.txt`, `render/eevee/limitations/index.txt`, `render/eevee/render_settings/performance.txt`, `render/eevee/introduction.txt`. Searched the whole manual for `compute capability`, `driver version`, `VRAM`, `CUDA`, `OptiX`, `HIP`, `oneAPI`, `Metal`, `NVLink`, `Device`, `GPU Compute`. Searched all of Cycles' own files for `device`, `Requires`, `driver`. Searched the release notes for `driver`, `requirement`, `minimum`. Read the requirements page's "Other Requirements → Cycles" paragraph, which only says "Rendering in Cycles using the GPU has specific requirements described in the manual" and names no version. **Ruled out:** the API documentation has no Cycles device page at all (see question 5's record).

---

## 3. Does what a computer must have on it depend on how Blender was obtained and put there?

**Answer.** Yes, in several ways the sources state outright.

**A build obtained from a Linux distribution may not be able to render on graphics cards at all.**

> "Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html. Read 2026-07-30.

This is not only a Linux point in principle: Blender exposes what a given build contains. The API documentation shows a build-options list including `cycles=True` and `cycles_osl=True` among about fifty flags (`bpy.app.build_options`), so whether Cycles is present at all is a property of the build.

*Label: documented.* Source: API documentation, https://docs.blender.org/api/4.5/bpy.app.html. Read 2026-07-30.

**The processor architecture of the download must match the machine.**

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture." (macOS page)

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture." (Windows page)

*Label: documented.* Sources: https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html and https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html. Read 2026-07-30.

**Administrator rights are needed for one Windows route and not the other.**

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

> "When choosing the zip-file, you have to manually extract Blender to the desired folder … No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

*Label: documented.* Source: https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html. Read 2026-07-30.

**Some routes need other software present first.**

> "Assuming snap is already installed, Blender can be installed through snap with: snap install blender --classic" (Linux page)

> "Download and install the Steam client for your operating system." (Steam page)

*Label: documented.* Sources: https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html and https://docs.blender.org/manual/en/4.5/getting_started/installing/steam.html. Read 2026-07-30.

**One route needs a particular unpacking tool.**

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead."

*Label: documented.* Source: https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html, under "Known Limitations". Read 2026-07-30.

**One route needs an approval from the operating system.**

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

*Label: documented.* Source: https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html. Read 2026-07-30.

**Where Blender's own files end up differs by route**, which matters for anything that has to find them. The manual gives distinct system-directory paths for a blender.org archive (`./4.5/`), Linux distribution packages (`/usr/share/blender/4.5/`), the Windows installer (`%ProgramFiles%\Blender Foundation\Blender\4.5\`) and the Microsoft Store (`%ProgramFiles%\WindowsApps\BlenderFoundation.Blender<HASH>\Blender\4.5\`).

*Label: documented.* Source: https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html. Read 2026-07-30.

**And one thing does *not* depend on the route:** file association on Linux is handled by the package manager when installed that way — "On Linux, if Blender is installed from a package manager such as Snap, file association is handled by the package manager" (https://docs.blender.org/manual/en/4.5/editors/preferences/system.html, documented, read 2026-07-30).

**Hand test on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, the copy under test was the official Linux x64 build from Blender's own download server, unpacked at `/opt/blender/blender-4.5.12-linux-x64/`. Asked what it was built with, `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed:

```
TYPES: (True, True, True, False, True, True)
```

Read against Cycles' own file, which unpacks that as `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt`, the official Linux download carries CUDA, OptiX, HIP and oneAPI and not Metal. So the sharpest case in this answer — a copy with Cycles graphics-card rendering left out — is not what the official Linux download does. A distribution's own package is a different copy and is untested; nothing may be installed on the rented machine.

**Not covered.** The sources do not say whether the *hardware* requirements (graphics card generation, driver version, memory) change with the route — everything above is about software, rights, tools and paths. They give no requirements at all for the "Build from Source" route, which the installing page lists as an option without stating what building needs. They do not say which distributions ship Cycles GPU support and which do not, and none could be installed to find out. They do not say whether a Microsoft Store or Steam or Snap copy can render on graphics cards.

**Outside the interface.** The route affects what a script or launcher must know: the executable's path differs by route (the command-line launch pages give `C:\Program Files\Blender Foundation\Blender\blender.exe` for the installer, `/Applications/Blender.app/Contents/MacOS/Blender` on macOS, and note that a distribution package usually puts `blender` on the PATH — https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, `.../macos.html`, `.../linux.html`, documented, read 2026-07-30). The manual also states the general case: "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools." (https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html).

**Operating system.** Every point above is stated for a named system: the Cycles-GPU-omission and 7-zip points are Linux, the administrator-rights and store-path points are Windows, the approval point is macOS, the architecture point is stated separately for Windows and macOS. Nothing here is stated as holding on all three.

**Where I looked.** Read all six files under `getting_started/installing/` in full, plus `advanced/deploying_blender.txt` and `advanced/blender_directory_layout.txt` in full. Searched the whole manual for `portable`, `snap`, `Steam`, `Microsoft Store`, `package manager`, `administrator`, `build from source`, `compile`. Searched the API documentation for `build_options`. Searched the release notes for install-route wording. **Ruled out:** the release notes say nothing about installation routes (searched all nineteen files); Cycles' own files say nothing about how Blender was obtained; the requirements page's "Truly Portable" paragraph would be relevant but names no version, so it cannot be used for 4.5.

---

## 4. Does Blender say anything about what is missing when it will not start, or when it will not render?

**Answer.** Yes for several specific cases, all of them named in the manual as messages, plus one message built by Cycles itself. But the sources describe *messages that exist*, not *what Blender does in general* when something is missing.

**When it will not start.**

The manual names a startup message about the graphics driver and says what it means:

> "Unsupported Graphics Driver Error
> This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/troubleshooting/gpu/windows/nvidia.html (and every other page under `troubleshooting/gpu/`). Read 2026-07-30.

The manual also lists two "Common Startup Messages" by their exact text, both about things Blender *found* rather than things missing:

> "found bundled python: {DIR}
> This message indicates that Blender was able to find the Python library for the Python interpreter embedded within Blender. If this folder is missing or unable to be found, it is likely that an error will occur, and this message will not appear."

> "Read prefs: {DIR}/userpref.blend
> The preferences use this path."

And it warns that you may not see any of it:

> "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window. To ensure you can see any error messages that appear at launch, open a command-line window and navigate to the location where the Blender executable is, and run blender from that location."

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html. Read 2026-07-30.

The same advice is repeated for crashes: "Try running Blender from the command line, to see if any helpful error messages are printed." (`troubleshooting/gpu/…`, documented.)

On a crash rather than a failure to start, Blender writes a file: "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash." — named `test.crash.txt` alongside a loaded `test.blend` on Windows, `blender.crash.txt` in `/tmp` on Linux, and via the macOS Crash Reporter on macOS. *Label: documented.* Source: https://docs.blender.org/manual/en/4.5/troubleshooting/crash.html. Read 2026-07-30. The release notes add for 4.5: "Blender on Windows now shows a crash popup with recovery options" (https://developer.blender.org/docs/release_notes/4.5/core/, documented, read 2026-07-30).

**When it will not render on graphics cards.**

The manual names four error messages under a heading "Error Messages" and explains each: "Error: Out of memory" ("This usually means there is not enough memory to store the scene for use by the GPU"); "The NVIDIA OpenGL driver lost connection with the display driver"; "Unsupported GNU version"; and "CUDA Error: Kernel compilation failed" ("This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed").

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html. Read 2026-07-30.

**Cycles builds an explicit "what is missing" message.** When no compatible card is found for the chosen technology, it prints the heading and then the requirement it failed:

```
1794:        if not found_device:
1795:            col = box.column(align=True)
1796:            col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)
1797:
1798:            if device_type == 'CUDA':
1799:                compute_capability = "3.0"
1800:                col.label(text=rpt_("Requires NVIDIA GPU with compute capability %s") % compute_capability,
```

followed by the OptiX, HIP, oneAPI and Metal branches quoted under question 2.

*Label: observed.* File: `Blender Sources/cycles_4.5/properties.py`, lines 1794–1852. Read 2026-07-30.

One line to be careful with: these are drawn into the preferences panel (`_draw_devices` is called from `draw_impl`, which is called from `draw`, lines 1861–1921 of the same file). That they appear on screen in the preferences is what the file shows. Whether the same words reach a terminal when Blender is run without its window is a step beyond the file, and the file does not take it.

**Cycles also refuses to start when told to use a device name it does not accept**, exiting rather than continuing:

```
38:    if args.cycles_device:
39:        import _cycles
40:        if not _cycles.set_device_override(args.cycles_device):
41:            sys.exit(1)
```

*Label: observed.* File: `Blender Sources/cycles_4.5/engine.py`, lines 38–41. Read 2026-07-30. That it exits is what the file says. What text, if any, is printed alongside is decided inside `_cycles.set_device_override`, which is not among the files I was given — so I cannot say whether anything is printed.

**Cycles avoids asking about devices at startup on purpose**, which bears on whether a missing card is reported early:

```
1637:    # Be careful when deciding when to call this function,
1638:    # as Blender can crash with `_cycles.available_devices()` on some drivers.
```
```
1693:    # Refresh device list. This does not happen automatically on Blender
1694:    # startup due to unstable drivers that can cause crashes.
```

*Label: observed.* File: `Blender Sources/cycles_4.5/properties.py`, lines 1637–1638 and 1693–1694. Read 2026-07-30.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, every technology button in Blender 4.5's **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** row was clicked in turn. The row held **None** and **Metal**. Neither the "No compatible GPUs found for Cycles" line nor any "Requires…" line appeared under either. A Metal device was present on this machine, so the no-compatible-card case was never reached and this run records nothing about what it displays.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 4.5 in background mode was asked for devices it could not give. With `-- --cycles-device OPTIX` it exited `1`, wrote no image, and printed `Error: Found no Cycles device of the specified type`. With `-- --cycles-device NOTADEVICE` it exited `1`, wrote no image, and printed `Error: NOTADEVICE is not a valid Cycles device.` So the `sys.exit(1)` in `engine.py` is accompanied by a message, and the message names the fault.

The other half is the quiet one. Asked to render on graphics cards with **every card switched off in the saved preferences**, it exited `0`, wrote the image, and printed nothing naming a missing or unusable device. Setting the technology to `NONE` behaved the same way. Full commands and output are under question 5.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, four things were put to 4.5.

**A device it cannot give.** `-- --cycles-device METAL`, `-- --cycles-device HIP` and `-- --cycles-device ONEAPI` all ended at exit `1` with no image and one line, `Error: Found no Cycles device of the specified type`. Worth noting: HIP and oneAPI *are* built into this copy and simply have no card, and Blender gives the same message for that as for a technology it was not built with at all. `-- --cycles-device NOTADEVICE` gave exit `1`, no image, and `Error: NOTADEVICE is not a valid Cycles device.` Both messages are word for word the Mac's.

**Whether the panel text ever reaches the console — settled, and the answer is no.** With the technology saved as `ONEAPI`, for which this machine has no card (`SAVED: ONEAPI ACTIVE: False NUMGPU: 0`), a background render exited `0`, wrote its image and rendered on the processor. The whole output was searched for `No compatible GPUs found for Cycles` and the phrase appeared **nought times**, and no `Requires…` line appeared either. So the missing-requirement text is drawn in the panel and nowhere else. What it says in the panel is quoted under question 2.

**Where the crash file lands on Linux.** A crash forced with `-b <file> --python-expr "import ctypes; ctypes.string_at(0)"` gave exit `139` and:

```
Writing: /tmp/machine-test.crash.txt
```

With no file loaded:

```
Writing: /tmp/blender.crash.txt
```

So the manual's Linux sentence is right about the folder and incomplete about the name: with a blend-file loaded it is named after that file, not `blender.crash.txt`. And it goes into `/tmp`, not beside the blend-file — the test file was elsewhere.

**What Blender says when it cannot open a window**, quoted in full under question 1: it names both windowing systems it tried, and in this version it then hangs rather than exiting.

**Not covered.** The sources describe a handful of named messages; they nowhere state a general rule about whether Blender reports what is missing. Several of the gaps this answer listed are now closed on two machines: what Blender does when the chosen technology or card is unavailable at render time (it names the fault and exits `1` when the technology was asked for on the command line, and quietly renders on the processor otherwise), whether these messages appear with no window (they do), and what exit status a failed render returns (`1`, with no image file). What is still not settled: whether any message carries a code a program could match on; what Blender says when a card is present and supported but its driver is too old; and the exact wording when Blender will not start for want of a good enough card, which needs a machine below the floor.

**Outside the interface.** The manual is explicit that the console is where messages go and that you should start Blender from a terminal to see them ("Blender output is shown in the Console Window it is started from", https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html, documented). The Cycles device-override exit above is a command-line behaviour by construction. The "No compatible GPUs found for Cycles" text is drawn into a panel, and it is now observed on the rented Linux machine that it has **no** equivalent outside the window.

**Operating system.** The crash-file behaviour is stated differently for each system (Windows writes `*.crash.txt` beside the blend-file and now shows a popup; Linux writes `blender.crash.txt` to `/tmp`; macOS uses its own Crash Reporter). Two of the three are now observed. On the Creator's Mac the line was `Writing: /var/folders/…/T/blender.crash.txt`. On the rented Linux machine it was `/tmp/<name of the loaded file>.crash.txt`, or `/tmp/blender.crash.txt` with nothing loaded. Windows is untested. The Windows console is hidden by default and must be re-shown from "Window ‣ Toggle System Console", which no machine used here could check. The Cycles "requires…" messages branch on the system in the file itself (`sys.platform` tests at lines 1811, 1822, 1832, 1837), so the Linux machine draws the Linux lines and can never draw the Windows ones. The "Unsupported Graphics Driver Error" explanation is given identically on all three systems' pages and was reached on neither machine.

**Where I looked.** Read in full: `troubleshooting/startup.txt`, `troubleshooting/crash.txt`, `troubleshooting/index.txt`, `troubleshooting/gpu/index.txt` and all twelve files beneath it, `render/cycles/gpu_rendering.txt`, `advanced/command_line/index.txt`. Searched the whole manual for `error`, `message`, `console`, `fall back`, `fallback`, `falls back`, `warning`, `Save System Info`. Searched all of Cycles' own files for `error`, `report`, `label`, `exit`, `warning`, `No compatible`. Searched the release notes for crash and error wording. **Ruled out:** the API documentation has no page on startup or render failure messages; the requirements page says nothing about messages; the only `fallback` hits anywhere in the manual's render sections are about compositor denoise nodes ("Use the GPU to process the denoise node if available, otherwise fallback to CPU") and EEVEE ray-tracing, neither of which is about a missing Cycles device.

---

## 5. How is it decided which of a computer's graphics cards a render uses, can more than one card be used for the same render, and how is that decided from outside Blender's own interface?

This question has three parts and I answer them in order.

### 5a. How it is decided which cards are used

**Two settings must agree, plus a tick per card.**

First, one technology is chosen for the whole of Blender, in the preferences:

> "Cycles Render Device
> Changes the computing device the Cycles render engine uses to render images. Cycles can use either the CPU or certain GPUs to render images"

with the options None, CUDA, OptiX, HIP, oneAPI and Metal, each described as available only "If the system has a compatible … device".

Second, each scene has its own setting: "Next, you must configure each scene to use GPU rendering in Properties ‣ Render ‣ Device."

*Label: documented.* Sources: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html and https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html. Read 2026-07-30.

I also opened the picture on the preferences page, `_images/editors_preferences_section_system.png` (https://docs.blender.org/manual/en/4.5/editors/preferences/system.html). It shows a panel headed "Cycles Render Devices" at the top of the System section — but the panel is collapsed in the screenshot, so it shows only that the panel exists, not what is inside it. *Label: documented, and deliberately limited to what is visible.*

Third — and this is the part the manual does not spell out — each individual card carries its own on/off tick, stored in the preferences:

```
1548:class CyclesDeviceSettings(bpy.types.PropertyGroup):
1549:    id: StringProperty(name="ID")
1550:    name: StringProperty(name="Name")
1551:    use: BoolProperty(name="Use", default=True)
1552:    type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
```

and the collection of them hangs off the Cycles preferences:

```
1592:    devices: CollectionProperty(type=CyclesDeviceSettings)
```

New entries are created with the tick on for graphics cards and off for the processor:

```
1666:                entry.use = entry.type != 'CPU'
```

*Label: observed.* File: `Blender Sources/cycles_4.5/properties.py`, lines 1548–1552, 1592 and 1654–1669. Read 2026-07-30.

The scene setting has exactly two values:

```
30:enum_devices = (
31:    ('CPU', "CPU", "Use CPU for rendering"),
32:    ('GPU', "GPU Compute",
33:        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
34:)
```
```
378:    device: EnumProperty(
379:        name="Device",
380:        description="Device to use for rendering",
381:        items=enum_devices,
382:        default='CPU',
383:    )
```

*Label: observed.* Same file, lines 30–34 and 378–383. Read 2026-07-30. Note the default is `CPU`.

The technology setting's default is not fixed either:

```
1558:    @staticmethod
1559:    def default_device():
1560:        import platform
1561:        # Default to selecting the Metal compute device on Apple Silicon GPUs
1562:        # (drivers are tightly integrated with macOS so pose no stability risk)
1563:        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
1564:            return 5
1565:        return 0
```

*Label: observed.* Same file, lines 1558–1565. Read 2026-07-30. The number 5 is Metal and 0 is None, by the numbering in `get_device_types` at lines 1571–1581. That mapping is a step the reader has to make, so treat "the default is Metal on Apple Silicon and None elsewhere" as *inferred* from those two passages read together.

Putting the three together, Cycles counts a card as in use only when the technology matches and the tick is on:

```
1714:    def get_num_gpu_devices(self):
1715:        compute_device_type = self.get_compute_device_type()
1716:
1717:        num = 0
1718:        if compute_device_type != 'NONE':
1719:            for device in self.get_device_list(compute_device_type):
1720:                if device[1] != compute_device_type:
1721:                    continue
1722:                for dev in self.devices:
1723:                    if dev.use and dev.id == device[2]:
1724:                        num += 1
1725:        return num
```

*Label: observed.* Same file, lines 1714–1725. Read 2026-07-30.

And Cycles' own interface code treats a render as being on graphics cards only when all three conditions hold:

```
108:def use_gpu(context):
109:    cscene = context.scene.cycles
110:
111:    return (get_device_type(context) != 'NONE' and cscene.device == 'GPU' and backend_has_active_gpu(context))
```

*Label: observed.* File: `Blender Sources/cycles_4.5/ui.py`, lines 108–111 (with `backend_has_active_gpu` at lines 100–101 calling `has_active_device`, which is `get_num_gpu_devices() > 0` at `properties.py` lines 1739–1740). Read 2026-07-30. What this shows is how Cycles' *panels* decide what to display. Reading it as "this is how the render itself picks devices" is a further step and would be **inferred**, not observed — the code that starts the render is inside `_cycles`, which I was not given.

**For the screen rather than the render**, there is a separate and separately documented choice:

> "Device Vulkan
> Specifies which GPU device to use for display drawing operations.
> This setting is useful for systems with multiple GPUs (e.g., integrated + discrete) where you want to force Blender to use a specific GPU for UI rendering.
> Changing the backend requires restarting Blender for the change to take effect.
> Auto: Automatically selects the most appropriate GPU based on system configuration and driver support."

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html. Read 2026-07-30. The API documentation names the same setting: "gpu_preferred_device — Preferred device to select during detection (requires restarting Blender for changes to take effect) / AUTO Auto – Auto detect best GPU for running Blender. / Type: enum in ['AUTO'], default 'AUTO'" (https://docs.blender.org/api/4.5/bpy.types.PreferencesSystem.html, documented, read 2026-07-30) — the listed choices there are just `AUTO`, with no card names.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), the whole of **Edit ‣ Preferences ‣ System** was looked at, in all four versions. The page held, top to bottom: **Cycles Render Devices** (two buttons, `None` and `Metal`, with `Metal` selected; two tick boxes, `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked; `Kernel Optimization` showing `Full`; `MetalRT` showing `Auto`), **Network** (`Allow Online Access` ticked, `Time Out` 10, `Connection Limit` 5), **Memory & Limits** (`Undo Steps` 256, `Undo Memory Limit` 0, `Global Undo` ticked, `Console Scrollback Lines` 256, `Texture Time Out` 120, `Garbage Collection Rate` 60, `VBO Time Out` 120, `Garbage Collection Rate` 60, `Geometry Nodes Stack Limit` 100), **Video Sequencer** (`Memory Cache Limit` 16384, `Proxy Setup` showing `Automatic`), and **Sound**, collapsed.

**There was no Display Graphics section, and no graphics backend or device control anywhere on the page** — in any of the four versions. So on this machine the setting quoted above was not present to be read.

One thing to say plainly rather than let it read as a failed search: the test that produced this was mis-aimed. It asked what the **Device** list offers once the backend is set to **Vulkan**, and Vulkan is not something a Mac offers at all — the question assumed a control that does not exist on this operating system. What a Windows or Linux machine shows here is untested, because neither exists among the equipment available.

On laptops the manual points outside Blender altogether: "Which GPU to use for which application can be configured in your graphics driver or operating system settings." (https://docs.blender.org/manual/en/4.5/troubleshooting/gpu/windows/nvidia.html, under "Laptops"; the same wording appears on the other per-vendor pages and in the shared fragment `troubleshooting/gpu/common/laptops.txt` — documented, read 2026-07-30).

### 5b. Whether more than one card can be used for the same render

**For Cycles, yes, and the manual says so plainly.**

> "Can multiple GPUs be used for rendering?
> Yes, go to Preferences ‣ System ‣ Compute Device Panel, and configure it as you desire."

With a stated limit on what that buys you:

> "Would multiple GPUs increase available memory?
> Typically, no, each GPU can only access its own memory.
> The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."

And the preferences page on that setting:

> "Distribute Memory Across Devices
> Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes. Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol.
> Currently only NVLink on NVIDIA GPUs is supported."

*Label: documented.* Sources: https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html and https://docs.blender.org/manual/en/4.5/editors/preferences/system.html. Read 2026-07-30. In Cycles' own files the setting is:

```
1594:    peer_memory: BoolProperty(
1595:        name="Distribute memory across devices",
1596:        description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it",
1597:        default=False,
1598:    )
```

*Label: observed.* `Blender Sources/cycles_4.5/properties.py`, lines 1594–1598. Read 2026-07-30.

Cards and the processor can also be combined: "Append +CPU to a GPU device to render on both CPU and GPU." (manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, documented).

One 4.5-specific caution is stated in the release notes:

> "When more than one Intel GPU is present and used for rendering, there may be crashes. A solution to this is already in Blender 5.0 alpha and planned for Blender 4.5.1, with the root cause to be fixed in a future Intel Driver release."

*Label: documented.* Source: release notes, https://developer.blender.org/docs/release_notes/4.5/cycles/. Read 2026-07-30.

**For EEVEE, no.**

> "Multiple GPU Support
> There is currently no support for multiple GPU systems."

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html. Read 2026-07-30.

### 5c. How it is decided from outside Blender's own interface

**There is one command-line switch, and it is documented.**

> "Cycles Render Options
> Cycles add-on options must be specified following a double dash.
> --cycles-device <device>
> Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
> Append +CPU to a GPU device to render on both CPU and GPU.
> Example:
> blender -b file.blend -f 20 -- --cycles-device OPTIX"

*Label: documented.* Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html. Read 2026-07-30.

Cycles' own file states the same switch's reach in stronger words than the manual does:

```
16:    parser.add_argument("--cycles-device",
17:                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
18:                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
19:                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
20:                        default=None)
```

and it only looks at arguments after a double dash:

```
27:    argv = sys.argv
28:    if "--" not in argv:
29:        return
```

*Label: observed.* File: `Blender Sources/cycles_4.5/engine.py`, lines 16–20 and 24–32. Read 2026-07-30. That the help text claims it overrides the preferences and the scene setting is what the file says; whether it does is decided in `_cycles.set_device_override`, which I was not given.

**What that switch cannot do:** every valid value it lists is a *technology*, not a particular card. There is no documented command-line way to name card one rather than card two. I looked for one specifically (see the record below) and found nothing.

**For picking individual cards from outside, the pieces exist but are not documented together.** The per-card tick lives in the Cycles add-on's preferences (`properties.py` line 1592, observed), and the API documentation shows the general way to reach any add-on's preferences: "addon_prefs = preferences.addons[__name__].preferences" (https://docs.blender.org/api/4.5/bpy.types.AddonPreferences.html, documented), with the manual adding "Add-ons can define their own preferences which use the package name as an identifier. This can be accessed using __package__." (https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html, documented). Cycles' own file sets exactly that: `bl_idname = __package__` (`properties.py` line 1556, observed).

Putting those together — that the ticks can therefore be set from a script run by `--python` — is a step the reader must take. *Label: inferred.* The step: the ticks are ordinary add-on preference properties, the API documents how add-on preference properties are reached, and Blender documents `--python <filepath>` and `--python-expr <expression>` as ways to run a script (manual, `advanced/command_line/arguments.html`); therefore the ticks are reachable that way. What makes it inferred rather than documented is that no source I was given shows this being done for Cycles, or names the key to use.

And the manual explicitly warns against guessing that key: "The value of __package__ will vary between systems so it should never be replaced with a literal string." (https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html, documented). So the sources do not give me a literal name to put in a script, and I will not invent one.

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 4.5.

**The panel.** **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** was expanded and every button in its technology row clicked in turn. The row held exactly two buttons, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — plus **Kernel Optimization** (**Full**; also **Off**, **Intersection Only**) and **MetalRT** (**Auto**; also **Off**, **On**). Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed.

**The key the sources would not give.** In Blender's Python Console (Scripting workspace, window open), `list(bpy.context.preferences.addons.keys())` returned:

```
['io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'cycles', 'pose_library', 'bl_pkg']
```

so the key is the literal string `cycles` on this machine. This is the string the manual warns must not be guessed; it is recorded here as what one machine returned, not as a value that holds everywhere.

**Reading and setting the ticks.** With `p = bpy.context.preferences.addons["cycles"].preferences`:

- `p.compute_device_type` returned `'METAL'`.
- `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned `[]` before anything else was called, and after `p.refresh_devices()` returned `[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]`.
- `p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, `p.has_multi_device()` returned `False`.
- `p.devices[0].use = False` was run — index 0 being the processor entry, unticked by default — and the panel then showed exactly what it shows by default. `p.devices[0].use = True` was run, and the **Apple M1 Pro** box in the panel became ticked.

This was all done from a Python Console inside Blender's own window; it records nothing about a background run.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, which reports `Blender 4.5.12 LTS`. Scene: `/tmp/machine-test.blend`, Cycles, scene Device `GPU`, 16 samples. Output quoted as printed.

**The ticks are read with no window open, and they decide the render.** With the saved preferences as found (the card ticked), a background render printed `Sample 16/16 (Using optimized kernels)` at `Mem:1558.44M`. After running

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --python-expr "import bpy; p=bpy.context.preferences.addons['cycles'].preferences; p.refresh_devices(); [setattr(d,'use',False) for d in p.devices]; bpy.ops.wm.save_userpref(); print('SAVED STATE:', p.compute_device_type, [(d.name,d.use) for d in p.devices])"
```

— which printed `SAVED STATE: METAL [('Apple M1 Pro', False), ('Apple M1 Pro (GPU - 14 cores)', False)]`, so `bpy.ops.wm.save_userpref()` works with no window — the same render instead ended:

```
Fra:1 Mem:225.15M (Peak 320.08M) | Time:00:04.85 | Mem:150.38M, Peak:150.38M | Scene, ViewLayer | Sample 16/16
```

with no `(Using optimized kernels)` and no message of any kind about the switched-off card. So the saved ticks are honoured by a windowless render.

**`--cycles-device` overrides the ticks.** With those same ticks still off, adding `-- --cycles-device METAL` gave:

```
Fra:1 Mem:74.69M (Peak 169.62M) | Time:00:03.65 | Mem:1558.44M, Peak:1558.44M | Scene, ViewLayer | Sample 16/16 (Using optimized kernels)
```

**`--cycles-device` overrides the scene setting too.** A copy of the scene saved with its own Device set to `CPU` (`/tmp/machine-test-cpu.blend`), rendered with `-- --cycles-device METAL`, gave:

```
Fra:1 Mem:74.69M (Peak 169.62M) | Time:00:03.69 | Mem:1558.44M, Peak:1558.44M | Scene, ViewLayer | Sample 16/16 (Using optimized kernels)
```

Both halves of the add-on's help-text claim — "overriding user preferences and the scene setting" — therefore hold on this machine.

**The key, from outside the window.** `-b --python-expr "…bpy.context.preferences.addons.keys()…"` printed `CYCLES KEYS: ['cycles']`, and reading the ticks through `bpy.context.preferences.addons['cycles'].preferences.devices` worked with no window:

```
BACKEND: METAL
BEFORE: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
AFTER: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

With `--factory-startup` added, `BEFORE:` was `[]` and `AFTER:` showed the card ticked and the processor unticked — so `--factory-startup` discards the saved per-card switches.

**A change made by a script inside the render's own run did not produce the graphics-card signature.** Setting every device's `use` to `True` in the same process that then rendered printed `SET TO: [('Apple M1 Pro', True), ('Apple M1 Pro (GPU - 14 cores)', True)]` and ended `Fra:1 Mem:309.96M (Peak 404.88M) | Time:00:04.31 | Mem:150.38M, Peak:150.38M | Scene, ViewLayer | Sample 16/16` — without the `(Using optimized kernels)` text the saved-preference GPU render carried. Setting them all to `False`, and setting `compute_device_type` to `'NONE'`, gave the same shape of result. The saved preferences were unchanged afterwards.

The preferences were restored to the state found before these runs.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-4.5.12-linux-x64/blender`. Its settings folder started empty, so this version began from factory settings rather than from anything a person had set. One thing about that folder, said plainly rather than left out. The first runs were meant to point Blender at a folder of this topic's own with `BLENDER_USER_RESOURCES`, and a mistake in the shell meant the variable was never actually set, so those runs used Blender's ordinary default folder instead. That folder was also empty when this work began, so "began from factory settings" holds either way. Every test that turns on saved preferences was then run again with the variable properly set, and every result came back identical. Both runs are described in `Audits/Blender Research/The machine/Hand tests — Linux.md`.

**What the card list looks like on this machine.** With an untouched settings folder:

```
CYCLES KEYS: ['cycles']
BACKEND: NONE
BEFORE: []
AFTER: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True)]
```

The key is `cycles` here too. The technology starts at `NONE` where the Mac starts at `METAL`. **One physical card is listed twice**, once under CUDA and once under OptiX, with two identifiers. With preferences saved, `BEFORE:` came back already populated, and `--factory-startup` emptied it again — the same pattern as the Mac.

**The saved ticks are honoured, and the flag overrides them.** Every device switched off and saved, then rendered: exit `0`, image written, `Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]`. The same render with `-- --cycles-device OPTIX`: exit `0`, `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]`.

**The flag overrides the scene setting too.** The scene saved with its own Device `CPU`, rendered with `-- --cycles-device OPTIX`, reported the card.

**`+CPU` uses both, and Linux names both.** `-- --cycles-device CUDA+CPU`:

```
Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                 AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (11 threads) [CPU]
```

**A change made by a script inside the render's own run — and the two machines disagree.** On the rented Linux machine it took effect. Every device off: `Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]`. Every device on: the card and the processor both named. The technology set to `NONE`: the processor. All three exited `0`. On the Creator's Mac the same three runs carried no graphics-card marker at all. Both stand; neither is chosen between here.

**The Vulkan display device — the control exists on Linux and offers one value here.** Read with no window, `gpu_backend` held `OPENGL` with choices `['OPENGL', 'METAL', 'VULKAN']`, and `gpu_preferred_device` held `AUTO` with choices `['AUTO']`. With a window, the System page carries a **Display Graphics** section — which macOS has not got at all — showing `Backend: OpenGL`. With the backend saved as `VULKAN` and Blender restarted, a **Device** control appears reading `Auto`, and beneath it a note this version carries and 5.1 and 5.2 do not:

```
Current Vulkan backend limitations:
  • Low performance in VR
```

`gpu_preferred_device` still held only `['AUTO']` with Vulkan actually running, because there is one card. What it offers with several cards is untested.

**There is no command-line way to name a display device in this version.** `--gpu-device 0` was treated as a file name:

```
unknown argument, loading as file: --gpu-device
Error: Cannot read file "/root/--gpu-device": No such file or directory
```

5.2 does have such an argument; that is another version's finding.

**Not covered.** Nothing in the sources settles: whether `--cycles-device` picks particular cards of a given technology or all of them; what happens if a ticked card is absent at render time; how EEVEE's single card is chosen when a machine has several. Two things this answer once listed are now observed on both machines: the preference ticks *are* read when Blender renders without its window, and a saved preferences file is what a background render reads. And how a Vulkan display device is chosen from the command line is answered for this version by the absence of any such argument.

**Outside the interface.** This is what part 5c asks, so the summary is: one documented switch exists (`--cycles-device`), it selects a technology and optionally adds the processor, and it is stated in the add-on's own file to override both the preferences and the scene. Per-card selection from outside is not documented anywhere in the sources; it is inferred as possible through the add-on's preferences and a startup script. The identifying key is no longer unstated: it returned as the literal string `cycles` on one machine, and the per-card ticks were read and set through it — from a Python Console inside the window, with the panel following, and from `--python-expr` with no window at all. The terminal tests above settle the rest of this line on that machine: the saved ticks are honoured by a windowless render, `bpy.ops.wm.save_userpref()` works with no window, `--cycles-device` overrides both the saved ticks and the scene's own setting, and `--factory-startup` discards the saved ticks. What no test here could reach is a machine with more than one card, where *which* card becomes the question.

**Operating system.** The list of technologies the switch accepts is the same everywhere, and Blender's own `--help` gives the identical six values in all four versions on both machines. Which of them exist on a machine is decided by system and hardware (question 2), and the starting technology differs by system in fact as well as in code — `METAL` on the Mac, `NONE` on the rented Linux machine. The Vulkan display-device setting is documented as part of a backend chosen per system, and `--gpu-backend` lists `vulkan, metal, opengl`; on Linux the Display Graphics section exists and shows a Device control once the backend is Vulkan, and on macOS the section is not there at all. The multi-card Intel warning in the release notes is not limited to one system in its wording. The NVLink exception is stated for NVIDIA cards without naming a system, and no machine with two cards exists to test it.

### Hand tests on the two-card machine — observed
**Read this block against the *Not covered* paragraph just above it.** That paragraph was written before any two-card machine existed, so some of what it calls untested is answered here — how work is divided between two ticked cards, and the several-card form of the `--cycles-device` question above all. What it says about an NVLink pair, about two cards of different makes, about a slow card beside a fast one, and about anything needing a window still stands. *(Pointer added 2026-08-06 by the citation check of the two-card material.)*

Run on **2026-08-06** on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-4.5.12-linux-x64/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`. Label: **observed**. **A different machine from the one-card rented Linux machine above; nothing here is merged with that machine's results or with the Creator's Mac.** Settings folder `/root/smoozy/two cards/res-4.5`, empty at the start, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender. **Nothing else was running on the machine.** The full list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`.

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

It can be turned on and saved on an unlinked pair, and read back `True` in a later run. A render with it on exited 0, used both cards, and **Blender said nothing about it at all** — searching both streams for `peer`, `nvlink`, `interconnect` and `distribut` matched nothing but the output file's own name. Measured card by card in 4.5 on an 868 MB scene, each card reached the same 3557 MiB whether the setting was on or off; with one card only, 3565 MiB on card 0 and 3 MiB on card 1. So the scene is still put on both cards in full.

**`--cycles-device OPTIX` takes every card of its technology, not only the ticked ones.** With only the card at `0000:00:07` ticked, a render with no flag named that card alone; the same state with `-- --cycles-device OPTIX` named both; and with **every** device unticked the same flag still named both. The flag wrote nothing back into the saved preferences.

**One card against two, on the same scene and the same samples**, on a machine with nothing else running: two cards `Time: 00:26.43`, `Time: 00:26.70`, `Time: 00:25.75`; the card at `0000:00:07` alone `Time: 00:49.49`, `Time: 00:49.90`, `Time: 00:49.13`; the card at `0000:00:0a` alone `Time: 00:50.24`, `Time: 00:48.77`, `Time: 00:48.89`.

**Blender prints one memory figure, never two, and never names a card beside one.** On the timing scene its `Finished` line read `Mem:6.98M, Peak:6.98M` with two cards and `Mem:1064.28M, Peak:1064.28M` with one — smaller with two, not larger and not doubled.

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

**Not covered here.** Cards of two different makes; a slow card beside a fast one; an NVLink pair; and anything needing eyes on a window, this machine having no display server.

**Where I looked.** Read in full: `render/cycles/gpu_rendering.txt`, `editors/preferences/system.txt`, `advanced/command_line/arguments.txt`, `advanced/command_line/render.txt`, `advanced/command_line/index.txt`, `render/eevee/limitations/limitations.txt`, `advanced/extensions/addons.txt` (the preferences section). Read in Cycles' own files: all of `engine.py`'s argument handling, `properties.py` lines 1548–1921 (the whole `CyclesPreferences` class and the device settings before it), `properties.py` lines 30–34 and 176–183 and 378–383 (the device enums and the scene setting), `ui.py` lines 92–200 and 2450–2480 (the device helpers and the panel that draws the scene setting), plus searches of `operators.py`, `presets.py` and `version_update.py` for anything device-related. Searched the whole manual for `Device` (only eleven hits in `render/`, all listed in my notes), `compute device` (one file), `Cycles Render Devices` (two files), `multiple GPU`, `NVLink`, `hybrid`, `+CPU`. Searched the whole API documentation for `compute_device_type` (only `bpy.types.PreferencesSystem`, and only as `legacy_compute_device_type`, "For backwards compatibility only"), `gpu_preferred_device`, `gpu_backend`, `addons[`. Searched the API documentation and the manual for `CUDA_VISIBLE_DEVICES` — **no hits anywhere in any source**, so no environment-variable route to card selection is documented. **Ruled out:** the API documentation has no `bpy.types.CyclesPreferences` page and no Cycles device page at all — `bpy.ops.cycles.html` documents only three operators (`denoise_animation`, `merge_images`, `use_shading_nodes`), none about devices; this is expected since Cycles is an add-on, and it means the per-card ticks are documented nowhere on the web addresses I was given. The manual's Cycles "Render Settings" contents page lists no "Device" entry, so the scene-level Device setting is documented only in passing on the GPU Rendering page, not on a page of its own.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

This list has since been run on two machines. What the Creator's own MacBook Pro settled is written into the answers above as **Hand test — observed**, and what the rented Linux machine settled as **Hand tests on the rented Linux machine — observed**. The two lists live at `Audits/Blender Research/The machine/Hand tests.md` and `Audits/Blender Research/The machine/Hand tests — Linux.md`. **A third machine has since been used as well** — a second rented Linux machine with two graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the two-card machine — observed** and **Hand tests on a machine with no display server at all — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`. *(Added 2026-08-06 by the citation check of the two-card material: this closing list still said two machines.)* **A fourth machine has since been used as well** — a third rented Linux machine with **four** graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the four-card machine — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

### Still open

Nothing on this list is still open. The Vulkan display-device item was run twice. On the Mac it turned out to be mis-aimed — that machine's System page carries no Display Graphics section and no backend or device control at all, Vulkan being something a Mac does not offer. On the rented Linux machine the section does exist, and its Device control appears once the backend is set to Vulkan and offers only `Auto` on a one-card machine. Both are recorded under question 5 above.

Items 1 to 4 of the original list — whether a background render honours the ticks, whether `--cycles-device` overrides both the preferences and the scene, what is printed for a device name Blender will not accept, and what happens when the chosen technology has no usable card — have all been run on both machines. What came back is recorded under questions 4 and 5 above. The last of the four is now fully answered rather than approximated: on the rented Linux machine the chosen technology really did have no usable card, and the render fell back to the processor without a word.

### Cannot be run on the equipment available

The equipment is one MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, whose graphics are Apple's own and built in, and one rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4090, driver 580.105.08 — reached over SSH. No Windows machine has run it, there is no AMD or Intel card and no second card, and nothing may be installed on the rented machine. **Brought up to date 2026-08-06 by the citation check of the two-card material.** A third machine has since been used — a second rented Linux machine, Ubuntu 22.04.5 LTS, **two** NVIDIA GeForce RTX 4090s on driver 580.95.05, sixty-one cores, **no display server at all**, over SSH — so "no second card" no longer holds, and what it settled is in the two-card block above. There is still no pair of cards joined by NVLink: this pair talks through the host bridge. The one-card rented Linux machine was destroyed on 2026-08-06 and nothing can be run on it again. And its driver *was* removed and reinstalled on the two-card machine, on purpose, for the broken-driver test recorded in the first-pass findings for this version.

**A fourth machine has since been used, and it is the only one still alive.** A **third rented Linux machine** — Ubuntu 22.04.5 LTS, **four** NVIDIA GeForce RTX 4060 Ti of **16 GB each** on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, **197 GB of memory**, **no display server**, over SSH. What it settled is written into the answers above as **Hand tests on the four-card machine — observed**, and its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. Three limits above are narrowed by it and none is removed: more than two cards is no longer out of reach, a card small enough to be filled is no longer out of reach, and the several-card form of every question under 5 and 6 has now been asked twice on two different machines. **Still out of reach on it as well:** any pair joined by NVLink, these four talking through the host bridge; cards of two different makes; cards of unequal speed, all four being the same model within about 2 % of each other; and anything needing eyes on a window, this machine having no display server either. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

- **Whether the figures Blender draws for Intel and AMD are enforced.** What the panel *draws* on Linux is now recorded under question 2. Testing whether Blender acts on those figures needs a real Intel or AMD card and a driver below them.
- **The Intel Windows driver figure.** Needs a Windows machine; the panel text branches on the operating system.
- **Which OpenGL level Blender 4.5 actually demands.** Needs a machine whose card supports OpenGL 3.3 but not 4.3. The rented machine reports `4.6.0 NVIDIA 580.105.08`.
- **The macOS 12.2-against-13.0 conflict.** Needs a Mac running macOS 12.x.
- **Anything needing a second graphics card**, including how work is divided and what the Vulkan Device control offers with more than one card.

---

2026-08-01 — Claude Opus 5 (Claude Code) — The machine second pass, Blender 4.5

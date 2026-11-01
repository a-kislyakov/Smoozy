# The machine — Blender 5.2 — second pass

**Sources consulted, and dates.**
Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files as shipped with Blender 5.2 (`Blender Sources/cycles_5.2`) — all read on **2026-07-30**. Blender's requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`, https://www.blender.org/download/requirements/) — read on **2026-08-01**.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One disclosure, so the line above is exact. I ran a directory listing of `Preparation/Blender Research/Findings/` — one `ls`, to confirm the folder I was told to write into existed before writing. It returned four folder names (`4.5`, `5.0`, `5.1`, `5.2`) and nothing else. I opened no file there, and in particular did not open `Preparation/Blender Research/Findings/5.2/The machine.md`. I also read `Blender Sources/About this folder.md`, which is inside `Blender Sources/` and therefore permitted; it describes the folder's own contents and contains no claims about Blender's behaviour.

**A note on the requirements page.** It carries no version in its address. Below, I use it as evidence for 5.2 only where I say so and explain why, and I flag every place where it would have answered a question but names no version.

---

## 1. What must a computer have on it before Blender will run at all?

**LABEL: documented, with one large gap (see "Not covered").**

The manual does not itself state hardware requirements. It points outward to the requirements page:

> "Blender is available for download on Windows, macOS, and Linux. Always check that your graphics drivers are up to date and that OpenGL is properly supported. Blender has a set of minimum and recommended requirements; so make sure these are met before trying to install Blender."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html (read 2026-07-30)

The requirements page gives the actual figures. **It does not name Blender 5.2 anywhere**, so by the rule I was given it speaks for whichever release was current on 2026-08-01, not necessarily for 5.2. I quote it here and mark the limit rather than omitting it, because it is the only place in the sources where these numbers appear at all.

> "Windows / Minimum / OS Windows 8.1 (64-bit) / CPU 4 cores with SSE4.2 support / RAM 8 GB / GPU 2 GB VRAM with OpenGL 4.3, Vulkan 1.3 (see below)"
> "macOS / Minimum / OS macOS 13 (Ventura) / CPU Apple Silicon / RAM 8 GB"
> "Linux / Minimum / OS Distribution with glibc 2.28 or newer (64-bit) / CPU 4 cores with SSE4.2 support / RAM 8 GB / GPU 2 GB VRAM with OpenGL 4.3, Vulkan 1.3 (see below)"
> "OpenGL: Version 4.3 or newer, with mandatory support for GL_ARB_shader_draw_parameters and GL_ARB_clip_control extensions."

— Blender requirements page, https://www.blender.org/download/requirements/ (read 2026-08-01). **These words name no Blender version.** They sit in the page's main tables, which describe the release current on the day the page was taken.

Two statements on the same page *do* name versions, and both are about releases other than 5.2:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."

— same page, same date. This one names "Blender 5.0 and later". 5.2 is later than 5.0. **Read as covering 5.2, this is inferred, not documented** — the step is: the page names a range ("5.0 and later"), and I place 5.2 inside that range. A reader has to supply that placement, so it is inferred.

The manual states the same macOS restriction directly, and this one *is* documented for 5.0-and-later:

> "Blender 5.0 and later only supports Macs with Apple silicon."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html (read 2026-07-30). That it appears in the 5.2 manual is itself evidence it holds for 5.2; that it covers 5.2 specifically is still the same range-placement step, so: **inferred** for 5.2 exactly.

Three further things the sources say Blender needs, independent of the requirements page:

**A supported graphics driver.** The manual's own figure differs from the requirements page's:

> "This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/troubleshooting/gpu/windows/nvidia.html (read 2026-07-30). This text is shared across every per-vendor troubleshooting page (it lives in an included fragment, `troubleshooting/gpu/_common/other.txt`, which has no page of its own — I cite the Windows/NVIDIA page, which I confirmed contains it verbatim). **I report both figures without reconciling them**: the manual says OpenGL 3.3 here, the requirements page says OpenGL 4.3. The sources do not say which governs 5.2.

**A windowing system on Linux.**

> "On Linux Blender supports both X11 and Wayland for official releases. When Wayland is detected, it is the preferred system, otherwise X11 will be used."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux_windowing_environment.html (read 2026-07-30)

**Nothing else — no installer, no internet.**

> "Whether it's on a USB stick, sitting on a folder on your desktop, or fully installed, Blender runs out of the box. No installation needed, no Internet connection required."

— Blender requirements page, https://www.blender.org/download/requirements/ (read 2026-08-01). Names no version.

> "Blender downloads can be extracted to any directory on the system, as a self contained installation."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html (read 2026-07-30). This one *is* the 5.2 manual, so it is documented for 5.2.

**Where I looked.** Searched the whole of `manual_5.2` and `api_5.2` for: `minimum requirements`, `system requirements`, `requirements`, `glibc`, `64-bit`, `RAM`, `VRAM`, `SSE4`, `Apple Silicon`, `OpenGL 4.3`, `OpenGL 3.3`, `Vulkan 1.3`. `glibc`, `SSE4`, `VRAM` and `Vulkan 1.3` return **zero hits anywhere in the 5.2 manual** — those numbers exist only on the requirements page. `OpenGL 4.3` appears in the manual only in the `--debug-gpu` argument description, not as a requirement. Read in full: `getting_started/installing/index.txt`, `linux.txt`, `macos.txt`, `windows.txt`, `steam.txt`, `linux_windowing_environment.txt`, `getting_started/configuration/hardware.txt`, `getting_started/index.txt`, `troubleshooting/index.txt`, `troubleshooting/startup.txt`, `troubleshooting/crash.txt`, all of `troubleshooting/gpu/`, `advanced/deploying_blender.txt`, `advanced/blender_directory_layout.txt`. Searched all of `release_notes_5.2` for `driver`, `requirement`, `minimum`, `Vulkan`, `OpenGL`, `Metal`, `CUDA`, `OptiX`, `HIP`, `oneAPI`, `macOS`, `glibc`, `Intel`, `compute capability`, `device` — **`requirement`, `glibc` and `compute capability` return zero hits, and nothing in the 5.2 release notes announces any change to hardware or driver requirements.** Ruled out: `getting_started/configuration/hardware.txt` despite its name — it is about mice, keyboards, tablets and VR headsets, not about what Blender needs to run.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 5.2 started and ran normally, and **Help ‣ Save System Info** was used to write `system-info.txt`. The file's section headed `GPU:` read:

```
renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:
```

and was followed by sections headed `Implementation Dependent GPU Limits:` and `GLSL:`, then `Cycles:`. The whole file has since been read; there is **no section headed `OpenGL`** anywhere in it, so it reports no OpenGL version to set against either figure. This is a machine that meets the Apple Silicon condition; it does not test any floor, and does not resolve the OpenGL 3.3-against-4.3 disagreement recorded above.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — `/opt/blender/blender-5.2.0-linux-x64/blender` (`Blender 5.2.0 LTS`) started and ran. The machine is a container sharing somebody else's hardware, and other work ran on it at the same time, so nothing here says how long anything took or how much memory was free.

**With no display reachable, Blender still runs and renders.** *(Wording corrected 2026-08-06 by the citation check of the two-card material. This machine had a real display server running and the runs were cut off from it by removing `DISPLAY`. A machine with genuinely no display server is a different machine, recorded in the block at the end of this answer. Nothing observed changes.)* `DISPLAY` was removed from the environment; a background Cycles render completed at exit `0` on the graphics card, and so did an EEVEE render.

**Asked for a window in the same state, it refuses and exits `1`:**

```
00:00.047  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.047  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4890, total unfreed memory 1.260777 MB
```

**The OpenGL 3.3-against-4.3 disagreement is still unsettled.** With `gpu.init()` called first, this machine reports version `4.6.0 NVIDIA 580.105.08` — above both figures, so it tests neither.

**Not covered.** The sources never state 5.2's own minimum CPU, RAM, operating-system version or graphics requirements in a place that names 5.2. The requirements page names no version for its main tables. The manual carries one requirement figure (OpenGL 3.3) that disagrees with the requirements page (OpenGL 4.3), and nothing resolves that. Nothing says what happens below the minimum — whether Blender refuses to start, starts degraded, or crashes. Nothing states a disk-space requirement.

**Outside the interface.** The sources do not say. Every requirement quoted above is stated without reference to how Blender is started. The one relevant remark I found points the other way — that a graphical display is *not* needed for rendering: "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example)" (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30). Whether the OpenGL/Vulkan requirement is relaxed when Blender runs with `--background` is **not in the sources**; I searched the manual for `headless`, `X server`, `background mode` and found only the two remarks quoted here and the EEVEE one in answer 2. The remark that points the other way is now observed as well: on the rented Linux machine, with no display server reachable at all, 5.2 rendered with Cycles on the graphics card and with EEVEE, both at exit `0`. Whether a machine *below* the graphics floor could do the same is still untested, both machines being far above it.

**Operating system.** No — the requirements differ per operating system by design, and the tables above are given separately for Windows, macOS and Linux. macOS is the sharpest difference: Apple Silicon only, from Blender 5.0 onward. Two systems are now tested: macOS on the Creator's Mac and Linux on the rented machine, where Blender runs and renders with neither X11 nor Wayland and refuses to open a window without them, naming both. Windows is untested.

**Hand tests on a machine with no display server at all — observed.** On **2026-08-06**, on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-5.2.0-linux-x64/blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`. Label: **observed**. Not merged with the one-card rented machine's results or the Creator's Mac.

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
GPU  CYCLES exit=0  png=5067
GPU  BLENDER_EEVEE_NEXT exit=0  png=3478
GPU  BLENDER_EEVEE exit=0  png=3478
GPU  BLENDER_WORKBENCH exit=0  png=3669
CPU  CYCLES exit=0  png=5065
CPU  BLENDER_EEVEE_NEXT exit=0  png=3478
CPU  BLENDER_EEVEE exit=0  png=3478
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
00:00.059  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.059  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4890, total unfreed memory 1.260777 MB
```

It never printed the line the script asked it to print, wrote this to **standard output**, and exited **1**. Nothing was written to standard error. Word for word what the one-card machine printed when cut off from its display, with the same exit status, plus the memory-leak line — so for this version the two cases are indistinguishable in what Blender says and does.
**Not covered.** Windows. Nothing about the OpenGL floor: this machine's cards are far above it.

---

## 2. What must a computer have on it before Blender will render using its graphics cards?

**LABEL: documented (Cycles), and documented-plus-observed for the exact figures.**

I read the question as being about Cycles GPU rendering, which is what the sources treat at length. I note at the end what EEVEE adds, since EEVEE also renders on the graphics card.

**Cycles.** Three things must be true. The manual states them as steps:

> "To enable GPU rendering: Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference. Enable the available devices for the chosen backend. Configure each scene to use the GPU Compute Device."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html (read 2026-07-30)

The hardware and driver each backend needs, from the same page and date:

> "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher."

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."

> "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."
> "Minimum driver versions: Windows: Radeon Software 24.9.1 or Radeon PRO Software 24.Q4 / Linux: Radeon Software 24.30 or ROCm HIP Runtime 6.3"

> "oneAPI is a computation library that is supported on Windows and Linux and requires an Intel® Arc™ graphics card with the Xe HPG architecture."
> "Minimum driver versions: Windows: Intel Graphics Driver XX.X.101.5518 / Linux: intel-level-zero-gpu package 1.3.27642, typically available through the intel-compute-runtime package XX.XX.34666.3"

> "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

And on what is *not* needed:

> "Normally users do not need to install the CUDA toolkit as Blender comes with precompiled kernels."

The same requirements are written into Cycles' own files, as the text shown when no usable card is found. **LABEL for this part: observed.**

```
                col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)

            if device_type == 'CUDA':
                compute_capability = "5.0"
                col.label(text=rpt_("Requires NVIDIA GPU with compute capability %s") % compute_capability,
```
— `Blender Sources/cycles_5.2/properties.py`, lines 1866–1870, inside `CyclesPreferences._draw_devices`

```
            elif device_type == 'OPTIX':
                compute_capability = "5.0"
                driver_version = "535"
```
— same file, lines 1873–1875

```
                    adrenalin_driver_version = "24.9.1"
                    pro_driver_version = "24.Q4"
```
— same file, lines 1886–1887 (the Windows branch of `device_type == 'HIP'`)

```
                    rocm_version = "6.3"
                    driver_version = "24.30"
```
— same file, lines 1894–1895 (the Linux branch of `HIP`)

```
                    driver_version = "XX.X.101.8306"
```
— same file, line 1912 (the Windows branch of `ONEAPI`)

```
                    driver_version = "XX.XX.37435.3"
```
— same file, line 1924 (the Linux branch of `ONEAPI`)

```
            elif device_type == 'METAL':
                mac_version = "12.2"
                col.label(text=rpt_("Requires Apple Silicon with macOS %s or newer") % mac_version,
```
— same file, lines 1936–1938

All read 2026-07-30. **Two of these numbers differ from the manual's, and I report the difference without resolving it**: the manual says oneAPI needs Windows driver `XX.X.101.5518` while the file says `XX.X.101.8306`; the manual says oneAPI needs Linux `intel-compute-runtime package XX.XX.34666.3` while the file says `XX.XX.37435.3`; and the manual says Metal "requires" macOS 13.0 "to support all features" while the file's message says macOS 12.2. The sources do not say which is authoritative.

**Cycles GPU support must also be present in the build.** The manual warns this is not guaranteed:

> "some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html (read 2026-07-30)

Cycles' own files confirm the backend list is decided by the build, not only by the hardware. **LABEL: observed.**

```
    def get_device_types(self, context):
        import _cycles
        has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()

        list = [('NONE', "None", n_("Do not use compute device"), 0)]
        if has_cuda:
            list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))
```
— `Blender Sources/cycles_5.2/properties.py`, lines 1637–1643 (read 2026-07-30)

That the *choices offered in the preferences* are filtered by these flags is what the file shows, so that much is observed. That an absent flag means the machine cannot render on that backend at all is a step beyond the file: **inferred**, and the step is that the enum built here is the only place the manual says a backend is chosen from.

**EEVEE.** EEVEE renders only on the graphics card, and one thing is stated about where it will not run:

> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering"
> "Headless Rendering — Headless rendering is not supported on headless Windows systems."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html (read 2026-07-30)

**Where I looked.** Read in full: `render/cycles/gpu_rendering.txt`, `render/cycles/index.txt`, `render/cycles/render_settings/index.txt`, `render/cycles/render_settings/performance.txt`, `editors/preferences/system.txt`, `render/render_engines.txt`, `render/eevee/index.txt`, `render/eevee/limitations/limitations.txt`, and all of `Blender Sources/cycles_5.2/properties.py`, `ui.py`, `engine.py`, `__init__.py`, `operators.py`, `version_update.py`. Searched `manual_5.2` + `api_5.2` + `release_notes_5.2` for `compute capability`, `CUDA`, `OptiX`, `HIP`, `oneAPI`, `Metal`, `NVLink`, `driver`. Searched all of `release_notes_5.2` for the same — **the 5.2 release notes announce no change to GPU rendering requirements**; the only Cycles entries are the texture cache, Raycast node attributes, subsurface scattering and world cast-shadows (https://developer.blender.org/docs/release_notes/5.2/cycles/, read 2026-07-30). Ruled out: `render/eevee/` pages other than `limitations` — searched for `requir`, `hardware`, `GPU`, `driver`, `backend` in `render/eevee/index.txt` and found no hardware statements at all. Ruled out the whole `Blender Sources/cycles_5.2/source/` tree — it is the render kernel's headers (`kernel/bvh`, `kernel/closure`, and so on), and contains no device-requirement or environment-variable text (searched it for `getenv`, zero hits).

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 5.2's **Edit ‣ Preferences ‣ System** page was opened and the **Cycles Render Devices** panel expanded. The row of technology buttons held exactly two, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — and two dropdown lists: **Kernel Optimization**, set to **Full**, offering **Off** and **Intersection Only**; and **MetalRT**, set to **Auto**, offering **Off** and **On**. Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed. No "Requires Apple Silicon with macOS … or newer" line appeared, because a Metal device was found, so this run records nothing about the macOS 12.2-against-13.0 disagreement above, and nothing about either oneAPI figure.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), a background Cycles render used the graphics card and ended `00:06.307  render           | Fra: 1 | Mem: 1607M | Sample 16/16 (1 kernels to optimize)`, so the three steps quoted above do not need the window: the backend was already `METAL` in the saved preferences, the card's switch was on, and the scene asked for GPU Compute. EEVEE also rendered with no window: `-b -E help` listed `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`, and a background EEVEE render exited `0`, ending `00:02.462  render           | Saved: '/tmp/eevee-52-0001.png'`. The headless limitation quoted above is stated for Windows; macOS is now tested and EEVEE rendered. Full commands and output are under question 5.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, three things were put to 5.2.

**The technology row on Linux, and the requirement text the Mac could never reach.** With a window open on the machine's own display, the row held five buttons — **None**, **CUDA**, **OptiX**, **HIP**, **oneAPI** — and no Metal. With **oneAPI** chosen and no Intel card present, 5.2 drew:

```
No compatible GPUs found for Cycles
Requires Intel® Arc™ GPUs or newer Intel® Graphics
  - intel-level-zero-gpu or intel-compute-runtime version
    XX.XX.37435.3 or newer
  - oneAPI Level-Zero Loader
```

With **HIP** chosen and no AMD card:

```
No compatible GPUs found for Cycles
Requires AMD GPU with RDNA architecture
and ROCm HIP Runtime 6.3 or newer
or AMD driver version 24.30 or newer
```

**This settles which of the two oneAPI figures a running 5.2 actually puts on screen for Linux: `XX.XX.37435.3`, the shipped file's number, not the manual's.** The wording of the first line is the shipped file's too. On the same machine, 5.0 and 5.1 draw `XX.XX.34666.3` and the older `Xe-HPG` wording — so the difference between the sources is a real change between versions and not an error, at least as far as what Blender displays. It says nothing about what Blender enforces against a real Intel card, of which there is none.

**A Cycles graphics-card render works with no window and with no display reachable.** With the technology saved as `CUDA`, and again as `OPTIX`, a background render exited `0` and named the card. With `DISPLAY` removed entirely, the same. That machine still had a display server running; the same on a machine with none at all is in the block at the end of question 1.

**The compute-capability figure is out of reach.** The `Requires NVIDIA GPU with compute capability` line is drawn only when no compatible card is found, and this machine's card is compatible: Blender's own system-information file reports its compute capability as major `8`, minor `9`.

**Not covered.** How much graphics memory is needed — the manual says only that running out is the usual cause of a GPU render failing, never a figure, and reaching that case would need the rented machine to itself. Whether a card below the stated compute capability is simply not offered, or is offered and then fails. What "to support all features" excludes on Metal below macOS 13.0. Whether the differing **Metal** numbers between the manual and the shipped file reflect a change or an error — the Linux oneAPI half of that is now answered above, and the Metal half needs a Mac on macOS 12.x. Nothing about EEVEE's minimum card.

**Outside the interface.** Partly different, and this is the one place the sources are explicit. The backend and the per-card choices live in the preferences, which are the same preferences whether Blender is driven by a person or not; but a command-line argument exists that overrides them — see answer 5. The one requirement that is explicitly about running without a window is EEVEE's: headless rendering is not supported on headless Windows systems (quoted above). For Cycles, the sources say nothing about whether the requirements differ in background mode; a Cycles graphics-card render and an EEVEE render both completed in background mode on the Creator's Mac and on the rented Linux machine, and on the latter with no display server at all. Windows stays open. And the requirement text quoted above never reaches a windowless run: with the technology saved as `ONEAPI` and no Intel card, the phrase `No compatible GPUs found for Cycles` appeared nought times in the whole output of a background render, which completed on the processor at exit `0`.

**Operating system.** No. The available backends are decided by operating system: CUDA, OptiX, HIP and oneAPI are stated as "supported on Windows and Linux"; Metal is stated as Apple-only. Driver minimums for HIP and oneAPI are given separately for Windows and Linux. Two systems are now tested and both bear it out: on the Creator's Apple Silicon Mac the technology row offered **None** and **Metal** and nothing else; on the rented Linux machine it offered **None**, **CUDA**, **OptiX**, **HIP** and **oneAPI** and no Metal, and drew the Linux figures. Windows remains untested, and the Windows figures cannot be read from Linux because the panel text branches on the operating system; a rented Windows machine would show them and none was used. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 3. Does what a computer must have on it depend on how Blender was obtained and put there?

**LABEL: documented — yes, in several specific ways.**

The requirements themselves are stated once, not per download. But the sources name five ways the obtaining method changes what the machine must have or must permit.

**Administrator rights — installer yes, zip no.**

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."
> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender. No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html (read 2026-07-30)

**CPU architecture must match the download, on Windows.**

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

— same page, same date.

**A distribution package may lack Cycles GPU rendering entirely.**

> "Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html (read 2026-07-30). This is the sharpest instance: the same machine may or may not be able to render on its cards depending purely on where Blender came from.

**Extra software must be present for Steam and for Snap.**

> "Download and install the Steam client for your operating system. Once it is installed, open the client and login to your Steam account, or create an account if you don't already have one."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/steam.html (read 2026-07-30)

> "Assuming snap is already installed, Blender can be installed through snap with: snap install blender --classic"

— https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html (read 2026-07-30)

**The Microsoft Store build is sandboxed, and Blender knows it.** The Python API exposes a read-only flag for exactly this:

> "is_microsoft_store_install — Whether this blender installation is a sandboxed Microsoft Store version (default False, readonly)"

— Blender 5.2 Python API, https://docs.blender.org/api/5.2/bpy.types.PreferencesSystem.html (read 2026-07-30). That the flag exists and says "sandboxed" is documented. What the sandbox actually restricts is **not in the sources**.

**A macOS approval step.**

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

— https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html (read 2026-07-30)

**Extraction tool matters on Linux.**

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead."

— https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html (read 2026-07-30)

**And one thing that does *not* depend on it:** the build's feature set is queryable regardless of source. `bpy.app.build_options` is documented with a sample value that includes `cycles=True, cycles_osl=True` (https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30). That the sample shows `cycles=True` is documented; that this key is how one detects a build without Cycles is **inferred** — the step is joining the manual's statement that some distributions omit Cycles GPU support to the existence of a `cycles` build flag, which the API page does not connect for you.

**Where I looked.** Read in full every page under `getting_started/installing/`, plus `advanced/deploying_blender.txt` and `advanced/blender_directory_layout.txt` (the "Portable Installation" section, which describes creating a folder named `portable` next to the executable). Searched `manual_5.2` and `api_5.2` for `Microsoft Store`, `Steam`, `Snap`, `package manager`, `administrator`, `arm64`, `portable`, `build_options`. Ruled out the release notes: searched all of `release_notes_5.2` for `install`, `Store`, `Steam`, `package` — nothing about obtaining or installing Blender appears in them.

**Hand test on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, the copy under test was the official Linux x64 build from Blender's own download server, unpacked at `/opt/blender/blender-5.2.0-linux-x64/`. There is a way to ask a running Blender which graphics-card technologies its copy carries: `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed:

```
TYPES: (True, True, True, False, True, True)
```

which Cycles' own file unpacks as `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt`. The same call on the Creator's Mac printed `(False, False, False, True, False, False)`. So the blender.org Linux x64 build carries CUDA, OptiX, HIP and oneAPI and no Metal, and the blender.org macOS build carries Metal and nothing else — which is what the operating systems allow, not a difference between downloads. A distribution package, a Snap or a Steam copy is a different copy and none could be tested; nothing may be installed on the rented machine.

**Not covered.** Whether the minimum CPU, RAM or graphics requirements themselves differ per download — the sources give one set of figures, on a page that names no version and no download method. What the Microsoft Store sandbox restricts. Whether a Steam or Snap build has the same backends compiled in as the blender.org build — the question can now be *asked* of a running Blender, as above, but no such build could be installed to ask it of. Whether the arm64 Windows build has the same Cycles backends as x64.

**Outside the interface.** The differences above are all properties of the installed files and the operating system, not of how Blender is driven, so they hold either way — but the sources do not say so, and I am not asserting it. One item is explicitly about being driven from outside: the Windows zip has no file association registered, and the manual says it can be registered with `blender -r` from the command line (`getting_started/installing/windows.html`, read 2026-07-30).

**Operating system.** No — every difference listed is specific to one operating system. Administrator rights and architecture matching are Windows; package-manager builds and Snap and TAR extraction are Linux; the approval prompt is macOS; Steam is all three.

---

## 4. Does Blender say anything about what is missing when it will not start, or when it will not render?

**LABEL: mixed — documented for several specific messages, not in the sources for the general case.**

**When it will not start.** The manual lists causes to check, but does not promise Blender names them:

> "If Blender fails to start, there are a few common causes to check for: See if your computer meets the minimum requirements. Confirm that your graphics card is supported and that the drivers are up to date … Make sure any antivirus software is not preventing Blender from starting. Ensure that you have appropriate user permissions on the computer."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html (read 2026-07-30)

The same page says messages exist and where to see them:

> "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window. To ensure you can see any error messages that appear at launch, open a command-line window and navigate to the location where the Blender executable is, and run blender from that location."

> "The Blender Console Window can display many different types of status and error messages. Some messages simply inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender non-responsive or shut down completely."

— same page, same date. It then documents exactly **two** message texts, and only two:

> "found bundled python: {DIR} — This message indicates that Blender was able to find the Python library for the Python interpreter embedded within Blender. If this folder is missing or unable to be found, it is likely that an error will occur, and this message will not appear."

> "Read prefs: {DIR}/userpref.blend — The preferences use this path."

— same page, same date.

One startup-failure message *is* documented by name, and it does name what is missing:

> "Unsupported Graphics Driver Error — This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

— https://docs.blender.org/manual/en/5.2/troubleshooting/gpu/windows/nvidia.html (read 2026-07-30; the same text appears on every per-vendor page, from a shared include).

And for a crash rather than a refusal, files are written naming the fault:

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash."

— https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html (read 2026-07-30), which then gives the per-platform filenames (`test.crash.txt` on Windows, `~/Library/Logs/DiagnosticReports/` on macOS, `/tmp/blender.crash.txt` on Linux).

**When it will not render.** Here the sources are much better. The manual documents four named Cycles error messages, each with the missing thing spelled out:

> "Error: Out of memory — This usually means there is not enough memory to store the scene for use by the GPU."

> "The NVIDIA OpenGL driver lost connection with the display driver — If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations."

> "Unsupported GNU version — On Linux, depending on your GCC version you might get this error."

> "CUDA Error: Kernel compilation failed — This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed."

— all from https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html (read 2026-07-30)

And Cycles' own files carry the message shown when no usable card is found — which names the specific missing thing per backend. **LABEL: observed.**

```
            col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)
```
— `Blender Sources/cycles_5.2/properties.py`, line 1866; followed at lines 1870–1938 by per-backend lines such as `"Requires NVIDIA GPU with compute capability %s"`, `"and NVIDIA driver version %s or newer"`, `"Requires AMD GPU with RDNA architecture"`, `"Requires Intel(R) Arc(TM) GPUs or newer Intel(R) Graphics"`, `"Requires Apple Silicon with macOS %s or newer"` (read 2026-07-30). What the file shows is that these strings are built and put into the preferences panel when no device of the chosen type is found. That a user staring at the preferences would therefore see the reason is one small step further: **inferred**.

One failure has a documented consequence rather than a message — a bad `--cycles-device` value stops Blender. **LABEL: observed.**

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```
— `Blender Sources/cycles_5.2/engine.py`, lines 37–41, in `_parse_command_line` (read 2026-07-30). The file shows the exit. What text, if any, is printed first is inside `_cycles.set_device_override`, which is not among the files I was given, so **that part is not in the sources**.

**Turning messages on.** The manual documents controls for how much Blender says:

> "--log-level <level> … info: Information about devices, files, configuration, operations"
> "--debug-cycles — Enable debug messages from Cycles."
> "--log-list-categories — List all available logging categories for --log, and exit."

— https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30). Note the wording of `info`: it explicitly includes "Information about devices". What that information contains is not shown.

**Where I looked.** Read in full: `troubleshooting/index.txt`, `startup.txt`, `crash.txt`, `network.txt` heading list, all of `troubleshooting/gpu/` (index, `_common/introduction`, `_common/laptops`, `_common/mesa`, `_common/other`, and the twelve per-vendor pages), `render/cycles/gpu_rendering.txt`, `advanced/command_line/index.txt`, `advanced/command_line/arguments.txt` in full, `advanced/command_line/launch/{linux,macos,windows}.txt`. Searched `manual_5.2` for `exit code`, `exit status`, `return code` — **zero hits**. Searched `manual_5.2` for `will not start`, `refuse` — **zero hits**. Searched `manual_5.2` + `api_5.2` + the Cycles files for `fall back`, `fallback`, `falls back` — many hits, none about a device failing over to another. Searched all Cycles Python files for `report(`, `label(text=`, `Unsupported`, `not supported`, `error` — the full set of user-facing device messages is the block quoted above plus `"Cycles built without Embree support"` / `"CPU raytracing performance will be poor"` (`ui.py` lines 959–960) and denoising errors in `operators.py`.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, every technology button in Blender 5.2's **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** row was clicked in turn. The row held **None** and **Metal**. Neither the "No compatible GPUs found for Cycles" line nor any "Requires…" line appeared under either. A Metal device was present on this machine, so the no-compatible-card case was never reached, and this run records nothing about what it displays, nor about whether that text ever reaches a console.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.2 in background mode gave an exit code and a message for a refused device, and neither for an unusable one.

`-- --cycles-device OPTIX`: exit status `1`, no image file, and the line `00:01.557  reports          | ERROR Found no Cycles device of the specified type`. `-- --cycles-device NOTADEVICE`: exit status `1`, no image file, and the line `00:00.257  cycles           | ERROR NOTADEVICE is not a valid Cycles device.` So `set_device_override` does print before the process ends, which this answer records as not in the sources.

Against that: a Cycles render asked for graphics cards with every card switched off in the saved preferences exited `0`, wrote the image, and printed nothing naming a missing or unusable device. The same held with the technology set to `NONE`. Full commands and output are under question 5.

**Crash files.** Several runs of the day ended in exit status `139` with `Writing: /var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/blender.crash.txt`. The crash file's `# Python backtrace` named a background thread in a third-party add-on installed on this machine, not device handling, and re-running the same command completed normally.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, four things were put to 5.2.

**Whether the "No compatible GPUs found" text ever reaches a console — settled, and the answer is no.** With the technology saved as `ONEAPI` and no Intel card present (`SAVED: ONEAPI ACTIVE: False NUMGPU: 0`), a background render exited `0`, wrote its image and rendered on the processor. The whole output was searched for that phrase and it appeared **nought times**; no `Requires…` line appeared either. What the panel shows instead is quoted under question 2.

**Devices Cycles will not accept, including one built in with no card.** `-- --cycles-device METAL`, `-- --cycles-device HIP` and `-- --cycles-device ONEAPI` all gave exit `1`, no image, and:

```
00:00.671  reports          | ERROR Found no Cycles device of the specified type
```

and `-- --cycles-device NOTADEVICE` gave exit `1`, no image, and:

```
00:00.224  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

**Where the crash file lands on Linux.** A crash forced with `-b <file> --python-expr "import ctypes; ctypes.string_at(0)"` gave exit `139` and `Writing: /tmp/machine-test.crash.txt`. With no blend-file loaded, `Writing: /tmp/blender.crash.txt`. So the manual's `/tmp/blender.crash.txt` is right about the folder and only right about the name when nothing is loaded.

**Two messages this version has and no earlier one does**, both about the card that draws rather than the card that renders:

```
00:00.627  ghost.context    | ERROR Requested Vulkan GPU '99' is unavailable or unsupported. Run with '--gpu-device help' to list available devices.
```
```
Warning: '--gpu-device' is only supported for the Vulkan backend. Ignoring the device override for the selected OpenGL backend and falling back to the stored GPU preference.
```

**Not covered.** No documented message for a machine below the CPU, RAM, glibc or operating-system minimum — the sources name a message only for the graphics driver, and no machine below any floor exists. No statement of what Blender prints when a chosen GPU backend is compiled in but the driver is too old at render time. Exit codes are undocumented but four are now recorded on the rented Linux machine: `0` for a render that completed, `1` for a refused device and for a windowed start with no display server, and `139` for a crash.

**Outside the interface.** The `--cycles-device` failure path exits the process (observed, above), which is a signal available outside the interface. The console messages are described as visible when Blender is started from a terminal, which the manual actively recommends: "Try running Blender from the command line, to see if any helpful error messages are printed" (`troubleshooting/gpu/windows/nvidia.html`, read 2026-07-30). The "No compatible GPUs found for Cycles" message is drawn into a preferences panel by `_draw_devices`, and whether it reaches anything at all when there is no window **is now observed on the rented Linux machine: it does not.** A windowless render whose technology has no compatible card printed the phrase nought times and completed on the processor.

**Operating system.** Partly. The crash-file mechanism is documented separately for each: `test.crash.txt` plus `blender_debug_log.cmd` and related scripts on Windows; the macOS Crash Reporter and `~/Library/Logs/DiagnosticReports/` on macOS; `/tmp/blender.crash.txt` on Linux. Two of the three are now observed — `blender.crash.txt` in the Mac's own temporary directory, and on Linux `/tmp/<name of the loaded file>.crash.txt`, or `/tmp/blender.crash.txt` with nothing loaded. The "NVIDIA OpenGL driver lost connection" message is stated as a Windows behaviour and no Windows machine ran these tests. "Unsupported GNU version" is stated as Linux and depends on the compiler on the machine, so it was not reached. The per-backend "Requires…" texts branch on `sys.platform` in `properties.py` for HIP and oneAPI (observed, `properties.py` lines 1883–1935), and the Linux wording is now quoted under question 2 while the Windows wording stays unread. One line appears on Linux that the Mac never showed: `WARNING HIPEW initialization failed: Error opening HIP dynamic library`, printed at the start of very nearly every run.

---

## 5. How is it decided which of a computer's graphics cards a render uses, can more than one card be used for the same render, and how is that decided from outside Blender's own interface?

**LABEL: documented for the first two parts; documented for the third with a real limit; the limit itself is observed.**

This question has three parts and I answer them in order.

### Which cards a render uses

Two settings, in two different places, both must be right.

**In the preferences** — a backend, then individual cards within it:

> "Cycles Render Devices — Changes the Computing Device the Cycles render engine uses to render images."
> "None: When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."
> "CUDA: If the system has a compatible NVIDIA CUDA device, it will be available as an option for rendering with Cycles."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html (read 2026-07-30)

**In the scene** — CPU or GPU:

> "Device — Specifies the compute device used for rendering in Cycles."
> "GPU Compute: Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices. Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/index.html (read 2026-07-30)

The manual's picture of the System preferences shows the backend row as a set of mutually exclusive tabs — on the machine pictured, `None`, `CUDA`, `HIP`, with `None` highlighted — and shows no per-card checkboxes beneath, consistent with `None` being selected. **LABEL: documented** (it is part of the manual). Picture: `_images/editors_preferences_section_system.webp`, on https://docs.blender.org/manual/en/5.2/editors/preferences/system.html (read 2026-07-30). I take from it only what is visible: the backends offered are shown as one exclusive row, and only backends the machine supports appear (this machine offers no OptiX, Metal or oneAPI tab).

The per-card switch itself is in Cycles' own files. **LABEL: observed.**

```
    id: StringProperty(name="ID", description="Unique identifier of the device")
    name: StringProperty(name="Name", description="Name of the device")
    use: BoolProperty(name="Use", description="Use device for rendering", default=True)
    type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
```
— `Blender Sources/cycles_5.2/properties.py`, lines 1619–1622, class `CyclesDeviceSettings` (read 2026-07-30). So each card has an id, a name, a `use` switch and a type; the preferences hold a collection of these (`devices: CollectionProperty(type=CyclesDeviceSettings)`, line 1662).

New cards are switched on by default, except the CPU. **LABEL: observed.**

```
                entry.use = entry.type != 'CPU'
```
— same file, line 1736, in `update_device_entries` (read 2026-07-30). Note this is the value given to a *newly discovered* device entry; it is not applied to an entry that already exists. That reading is what the surrounding `if not entry:` branch shows.

The valid device types are fixed: **observed**, `enum_device_type` at lines 172–179 of the same file — `CPU`, `CUDA`, `OPTIX`, `HIP`, `METAL`, `ONEAPI`.

### Whether more than one card can be used for the same render

**Yes, for Cycles.** Documented:

> "Can multiple GPUs be used for rendering? — Yes, go to Preferences ‣ System ‣ Compute Device Panel, and configure it as you desire."

— https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html (read 2026-07-30)

With a stated limit on what that buys you:

> "Would multiple GPUs increase available memory? — Typically, no, each GPU can only access its own memory."
> "The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."

— same page, same date. And from the preferences page:

> "Distribute Memory Across Devices — Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes. Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol. Currently only NVLink on NVIDIA GPUs is supported."

— https://docs.blender.org/manual/en/5.2/editors/preferences/system.html (read 2026-07-30)

CPU and GPU together is also possible:

> "Append +CPU to a GPU device to render on both CPU and GPU."

— https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30)

**No, for EEVEE.** Documented:

> "Multiple GPU Support — There is currently no support for multiple GPU systems."

— https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html (read 2026-07-30)

One thing the sources say about mixing cards, which is easy to miss. **LABEL: observed.**

```
        # Any device without RT support will disable it for all.
        has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt
```
— `Blender Sources/cycles_5.2/properties.py`, lines 1976–1977, in `draw_impl` (which begins at line 1948) (read 2026-07-30). This is a comment plus the line it describes, in the code that decides whether the hardware-ray-tracing checkbox is active. That the checkbox greys out is what the surrounding `row.active = has_hardware_rt` lines show; that a render therefore runs without hardware ray tracing when one selected card lacks it is a step beyond the file, so: **inferred**.

### How it is decided from outside Blender's interface

There is one command-line argument, and it sets the **backend, not the individual card**:

> "--cycles-device <device> — Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. Append +CPU to a GPU device to render on both CPU and GPU."
> "Example: blender -b file.blend -f 20 -- --cycles-device OPTIX"
> "Cycles add-on options must be specified following a double dash."

— https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30)

Cycles' own files say what it overrides. **LABEL: observed.**

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```
— `Blender Sources/cycles_5.2/engine.py`, lines 16–20 (read 2026-07-30). The words "overriding user preferences and the scene setting" are the file's own. Note this means the argument covers *both* of the two settings described above — you do not additionally need to set the scene to GPU Compute. That reading is what the help string says.

The argument is only read when `--` is present:

```
    argv = sys.argv
    if "--" not in argv:
        return
```
— same file, lines 27–29 (read 2026-07-30).

**The limit, and it is the important part of this answer.** `--cycles-device` names a *backend type*. Nothing I found lets you name individual cards from the command line. The per-card `use` switches live in the preferences, which persist in `userpref.blend`, and are reachable from Python:

> "addons — (default None, readonly) — Type: Addons[Addon]"
> "use_preferences_save — Save preferences on exit when modified (unless factory settings have been loaded) (default True)"

— Blender 5.2 Python API, https://docs.blender.org/api/5.2/bpy.types.Preferences.html (read 2026-07-30)

> "bpy.ops.wm.save_userpref() — Make the current preferences default"

— https://docs.blender.org/api/5.2/bpy.ops.wm.html (read 2026-07-30)

That one could therefore set individual cards from a `--python` script by writing to `preferences.addons['cycles'].preferences.devices[…].use` is **inferred**, and I write the step out plainly: the API documents that add-on preferences are reachable through `Preferences.addons`, and Cycles' own file shows a `devices` collection whose members carry a `use` switch — but **no source I was given writes that path out or shows an example of doing it**, and `bpy.types.CyclesPreferences` has no page in the 5.2 API documentation at all.

**Where I looked.** Read in full: `advanced/command_line/arguments.txt`, `advanced/command_line/render.txt`, `advanced/command_line/index.txt`, `editors/preferences/system.txt`, `render/cycles/render_settings/index.txt`, `render/cycles/gpu_rendering.txt`, `render/eevee/limitations/limitations.txt`, `Blender Sources/cycles_5.2/engine.py` and the device sections of `properties.py` and `ui.py`. Searched `manual_5.2` + `api_5.2` + `release_notes_5.2` for `cycles-device` (two hits, both in `arguments.txt`), `CYCLES_DEVICE`, `BLENDER_DEVICE`, `CYCLES_OPENCL` (**zero hits** — the only `CYCLES_*` environment variable anywhere in the 5.2 manual is `CYCLES_CUDA_EXTRA_CFLAGS`, and that is a compiler flag, not a device selector). Searched all Cycles Python files for `getenv`, `os.environ` — **zero hits**, so Cycles' own files set no device from an environment variable. Searched `cycles_5.2/source/` for `getenv` — **zero hits**. Searched for `NVLink`, `multiple GPU`, `multi-device`, `hybrid`. Ruled out `bpy.ops.cycles` (https://docs.blender.org/api/5.2/bpy.ops.cycles.html, read 2026-07-30) — it holds only `denoise_animation`, `merge_images` and `use_shading_nodes`, nothing about devices.

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 5.2.

**The panel.** **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** was expanded and every button in its technology row clicked in turn. The row held exactly two buttons, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — plus **Kernel Optimization** (**Full**; also **Off**, **Intersection Only**) and **MetalRT** (**Auto**; also **Off**, **On**). Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed.

**From Blender's Python Console** (Scripting workspace, window open). `list(bpy.context.preferences.addons.keys())` returned:

```
['io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'cycles', 'pose_library', 'bl_pkg']
```

so the key is the literal string `cycles` on this machine — the path this answer could only mark inferred. With `p = bpy.context.preferences.addons["cycles"].preferences`:

- `p.compute_device_type` returned `'METAL'`, matching the button shown as selected and the Apple Silicon default recorded above.
- `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned, **before `refresh_devices()` was called**, `[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]`, and returned the same two entries after `p.refresh_devices()`.
- `p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, `p.has_multi_device()` returned `False`.
- `p.devices[0].use = False` was run — index 0 being the processor entry, unticked by default — and the panel then showed exactly what it shows by default. `p.devices[0].use = True` was run, and the **Apple M1 Pro** box in the panel became ticked.

So the per-card `use` switch was both read and written from Python, and writing it changed the panel. This was done from a Python Console inside Blender's own window; it records nothing about a background run, and `bpy.ops.wm.save_userpref()` was not called.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender.app/Contents/MacOS/Blender` (`Blender 5.2.0 LTS`). Scene: `/tmp/machine-test.blend`, Cycles, scene Device `GPU`, 16 samples.

**The inferred script route, run for real with no window open.** The key printed as `['cycles']`, and

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b --python-expr "import bpy; p=bpy.context.preferences.addons['cycles'].preferences; p.refresh_devices(); [setattr(d,'use',False) for d in p.devices]; bpy.ops.wm.save_userpref(); print('SAVED STATE:', p.compute_device_type, [(d.name,d.use) for d in p.devices])"
```

printed `SAVED STATE: METAL [('Apple M1 Pro', False), ('Apple M1 Pro (GPU - 14 cores)', False)]`. So writing to `preferences.addons['cycles'].preferences.devices[…].use` from a `--python-expr` script, and saving with `bpy.ops.wm.save_userpref()`, both work with no window — the route this answer could only mark inferred.

**And the saved switch decides the render.** With the card switched off as above, a background render ended:

```
00:05.930  render           | Fra: 1 | Mem: 151M | Sample 16/16
```

and with the card switched back on it ended:

```
00:06.307  render           | Fra: 1 | Mem: 1607M | Sample 16/16 (1 kernels to optimize)
```

**`--cycles-device` covers both settings, as the help text claims.** With every card still switched off, `-- --cycles-device METAL` ended `00:05.484  render           | Fra: 1 | Mem: 1607M | Sample 16/16 (1 kernels to optimize)`. Against a copy of the scene saved with its own Device set to `CPU`, the same flag ended `00:05.700  render           | Fra: 1 | Mem: 1607M | Sample 16/16 (1 kernels to optimize)`.

**`--factory-startup` and the per-card switches.** Reading the stored list with `--factory-startup` gave `BEFORE: []` and, after a refresh, the card ticked and the processor unticked — regardless of what was saved. So it does discard them, which this answer records as not in the sources.

**What it did not settle.** A change made inside the render's own run — setting every `use` to `True` in the same process that then rendered — printed `SET TO: [('Apple M1 Pro', True), ('Apple M1 Pro (GPU - 14 cores)', True)]` and ended `00:05.417  render           | Fra: 1 | Mem: 151M | Sample 16/16`, without the text a graphics-card render carries. The saved preferences were unchanged afterwards, and were restored to the state found.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-5.2.0-linux-x64/blender`. Its settings folder started empty, so this version began from factory settings rather than from anything a person had set. One thing about that folder, said plainly rather than left out. The first runs were meant to point Blender at a folder of this topic's own with `BLENDER_USER_RESOURCES`, and a mistake in the shell meant the variable was never actually set, so those runs used Blender's ordinary default folder instead. That folder was also empty when this work began, so "began from factory settings" holds either way. Every test that turns on saved preferences was then run again with the variable properly set, and every result came back identical. Both runs are described in `Audits/Blender Research/The machine/Hand tests — Linux.md`.

**What Blender finds, and how the one card is listed.** With an untouched settings folder:

```
CYCLES KEYS: ['cycles']
BACKEND: NONE
BEFORE: []
AFTER: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True)]
```

The key is `cycles` here too. The technology starts at `NONE`, where the Mac starts at `METAL`. **One physical card is listed twice**, once under CUDA and once under OptiX, and the stored identifier visibly carries the card's place on the bus.

**Whether `--factory-startup` discards the saved per-card switches — answered, and it does.** With preferences saved, `BEFORE` came back already populated; with `--factory-startup` added, `BEFORE` came back `[]` and the refresh filled it from scratch. Both on both machines.

**What happens if `--cycles-device` names a backend the machine cannot provide — answered.** `METAL` (not built into this copy), `HIP` and `ONEAPI` (built in, no card) all gave exit `1`, no image, and `ERROR Found no Cycles device of the specified type`.

**The saved switches are honoured, the flag displaces them, and the flag also displaces the scene's own setting.** Every device switched off and saved, then rendered: `Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]`. The same with `-- --cycles-device OPTIX`: `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) […]`. A scene saved with Device `CPU`, rendered with the same flag: the card. The saved preferences read back unchanged afterwards, so the flag is not written into them.

**`+CPU` uses both, and names both:**

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                            |                      Hardware Ray-Tracing: Off
                            |                  AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (11 threads) [CPU]
                            |                      Hardware Ray-Tracing: Off
```

**Whether the same script works with no window open — yes, and here the two machines part company on one point.** Reading and writing `bpy.context.preferences.addons["cycles"].preferences.devices[…]` through `--python-expr` with no window worked on both machines, and `bpy.ops.wm.save_userpref()` worked with no window on both. But a change made *inside the render's own run*, never saved, **took effect on the rented Linux machine** — the report named exactly the devices the script had asked for, in all three variants — and produced **no graphics-card marker at all on the Creator's Mac**. Both results stand and neither is chosen between here.

**A way to name individual cards from the command line does exist in this version, for the card that draws.** Blender's own `--help` lists, in 5.2 alone:

```
--gpu-device <device>
	Select a specific GPU device, overriding the GPU device from user preferences for this
	run only. …
	* 'help' prints supported Vulkan devices and exits.

	Only used with the Vulkan backend. Other backends print a warning and ignore this option.
```

and `--gpu-device help` printed, with no window and no display reachable:

```
Blender GPU Device Listing (Vulkan):
Pass an Index or Device-ID to '--gpu-device'.
  Index  Device-ID    Name
  0      10de/2684/0  NVIDIA GeForce RTX 4090
  1      10de/2684/1  NVIDIA GeForce RTX 4090
  2      10005/0/2    llvmpipe (LLVM 15.0.7, 256 bits)
```

This does not touch `--cycles-device` and does not select a Cycles render device.

**The Display Graphics section.** With no window, `GPU_BACKEND NOW: OPENGL | CHOICES: ['OPENGL', 'METAL', 'VULKAN']` and `GPU_PREFERRED_DEVICE NOW: AUTO | CHOICES: ['AUTO']`. With a window, the System page carries a **Display Graphics** section — which the Mac has not got — showing `Backend: OpenGL`; with the backend saved as `VULKAN` and Blender restarted, a **Device** control appears reading `Auto`. This version does not show the "Current Vulkan backend limitations" note that 4.5 and 5.0 show on the same machine.

**Not covered.** No way documented to select individual **Cycles** cards from the command line — only the backend; this version's `--gpu-device` names the drawing card instead. No documented ordering or priority when several cards of the same backend are enabled, and no second card to try. Nothing on how work is divided between two enabled cards. Nothing on whether `--cycles-device` affects the Compositor's or the denoiser's separate device settings — the manual notes those exist ("Denoising and Compositing have their own device selection", `render_settings/index.html`) but does not connect them to the argument; every graphics-card render on the rented Linux machine reported denoising on the processor, which is a result about denoising rather than about the argument.

**Outside the interface.** Directly addressed, and the answer is: partly. The backend and CPU/GPU choice can be forced with `--cycles-device` (documented). The per-card choice cannot, and depends on saved preferences that were set from inside the interface, or on a Python script (inferred). Both machines turn that inference into an observation, and the Linux one closes the gap the Mac left: the key is the literal string `cycles`, and the `use` switches were read and written through `bpy.context.preferences.addons["cycles"].preferences.devices[…]` from a Python Console inside the window on the Mac, with the panel's tick following, and from `--python-expr` **with no window open** on both machines. `bpy.ops.wm.save_userpref()` works with no window. `--factory-startup` **does** discard the saved preferences and therefore the per-card switches, which the manual describes only in terms of the startup file — observed on both machines.

**Operating system.** The mechanism is the same everywhere; which backends are on the menu is not, and that is stated per operating system in answer 2. One default differs by machine. **LABEL: observed:**

### Hand tests on the two-card machine — observed
**Read this block against the *Not covered* paragraph just above it.** That paragraph was written before any two-card machine existed, so some of what it calls untested is answered here — how work is divided between two ticked cards, and the several-card form of the `--cycles-device` question above all. What it says about an NVLink pair, about two cards of different makes, about a slow card beside a fast one, and about anything needing a window still stands. *(Pointer added 2026-08-06 by the citation check of the two-card material.)*

Run on **2026-08-06** on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-5.2.0-linux-x64/blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`. Label: **observed**. **A different machine from the one-card rented Linux machine above; nothing here is merged with that machine's results or with the Creator's Mac.** Settings folder `/root/smoozy/two cards/res-5.2`, empty at the start, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender. **Nothing else was running on the machine.** The full list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`.

**The two cards are not joined by NVLink** — `nvidia-smi topo -m` reports `PHB`, `Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)` — so an NVLink pair still cannot be tested here.

**Both cards start switched on, and one card is listed twice.** With an untouched settings folder and `--factory-startup`, after `refresh_devices()`:

```
BACKEND: NONE
BEFORE: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a', True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX', True)]
```

**Yes, more than one card is used for the same render, and without being asked.** With the technology saved as `OPTIX` and both ticked, a windowless render named both:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                     Hardware Ray-Tracing: Off
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

**One card against two, on the same scene and the same samples**, on a machine with nothing else running: two cards `Time: 00:33.58 (Saving: 00:00.45)`, `Time: 00:33.62 (Saving: 00:00.33)`; one card `Time: 00:59.53 (Saving: 00:00.37)`, `Time: 00:59.78 (Saving: 00:00.42)`.

**Blender prints one memory figure, never two, and never names a card beside one.** On the timing scene its `Finished` line read `Mem: 8M` with two cards and `Mem: 1101M` with one — smaller with two, not larger and not doubled.

### Hand tests on the four-card machine — observed

Run on **2026-08-06** on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each, driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server, run over SSH** — against `/opt/blender/blender-5.2.0-linux-x64/blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`. Label: **observed**. **A fourth machine, and a different one from every machine above; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine.** Settings folder `/root/smoozy/four cards/res-5.2`, emptied and remade before use, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender. **Nothing else was running on the machine.** The full list, with every command and every script written out, is `Audits/Blender Research/The machine/Hand tests — four cards.md`.

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
                            |                      Hardware Ray-Tracing: Off
                            |                  NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:0b_OptiX]
                            |                      Hardware Ray-Tracing: Off
                            |                  NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:09_OptiX]
                            |                      Hardware Ray-Tracing: Off
                            |                  NVIDIA GeForce RTX 4060 Ti (OptiX) [CUDA_NVIDIA GeForce RTX 4060 Ti_0000:00:07_OptiX]
                            |                      Hardware Ray-Tracing: Off
```

and each card was watched separately with `nvidia-smi --query-gpu=index,utilization.gpu,memory.used --format=csv -l 1` during a render in Blender 4.5. All four sat between 97 % and 98 % for the whole render, and each held 854 or 856 MiB:

```
0, 97 %, 856 MiB
1, 97 %, 854 MiB
2, 97 %, 854 MiB
3, 98 %, 856 MiB
```

**One line reads differently from the two-card machine's.** There the rebalancer reported `Number of requested rebalances: 1` and `Number of performed rebalances: 0`. Here it reports 9 requested and 8 performed, so with four cards Blender moved the work between them repeatedly during one render, where with two it did not move it at all. **The count itself moves from run to run and is not this version's figure.** The run recorded here printed 9 requested and 8 performed; a repeat on the same machine printed 7 requested and 6 performed. Both are recorded as they printed and nothing is read into either. *(Narrowed by the four-card citation check, 2026-08-06: this gave a single run's count as the version's, and the repeat disagreed.)*

**One card, two cards and four cards, on the same scene and the same samples.** Forty-eight renders in all across the four versions, one after another, never overlapping, on a machine with nothing else running. The scene is 1920 by 1080 at 3000 samples, adaptive sampling off, denoising off. **Cycles' own `Total render time:`, in seconds**, three runs of each: four cards `20.4372`, `20.4797`, `20.5198`; two cards `39.1108`, `39.0499`, `39.017`; the card at `0000:00:07` alone `74.1969`, `74.2503`, `74.2102`. **Blender's own `Time:` line**, which counts the whole run: four cards `00:26.92`, `00:26.96`, `00:27.02`; two cards `00:43.72`, `00:45.36`, `00:45.40`; one card `01:20.43`, `01:20.57`, `01:18.89`.

**The four cards are as fast as each other.** The cards at `0000:00:09`, `0000:00:0b` and `0000:00:0d`, one run each, gave `73.085`, `73.9701` and `72.1093` seconds on Cycles' own figure — a spread of about 2 %.

**Doubling the cards did not halve the time, and quadrupling them did not quarter it.** On Cycles' own figure, one card to two is 1.90 times faster, two to four is 1.91, and one to four is 3.62. On Blender's whole-run figure, where the work of loading the file and building the scene does not shrink at all, the same three are 1.80, 1.63 and 2.98. So "more cards make it proportionally faster" holds in shape and not in arithmetic.

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
four cards: 00:26.687  render           | Fra: 1 | Mem: 8M | Finished
two cards:  00:43.612  render           | Fra: 1 | Mem: 8M | Finished
one card:   01:20.385  render           | Fra: 1 | Mem: 397M | Finished
```

**The figure is far smaller with more than one card — and it does not go on shrinking.** Two cards and four cards printed exactly the same figure, on this scene and — in Blender 4.5, which is where that half was measured — on the 909 MB one. *(Narrowed by the four-card citation check, 2026-08-06: the heavier scene was measured in 4.5 only, and this sentence read as though it had been measured here.)* So it does not count memory divided by the number of cards. What it does count is not settled here and nothing is read into it.

**Not covered here.** Cards of two different makes; a slow card beside a fast one, all four being the same model and within 2 % of each other; an NVLink set; and anything needing eyes on a window, this machine having no display server either.

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on. So this block speaks for the outside case alone, and none of the two-card machine's window tests could be repeated here either.

**Operating system — this block.** Linux, on the machine named at the top of it. It says nothing about macOS, where the project's only machine has one set of built-in graphics, and nothing about Windows, which no Windows machine has run.

**Not covered here.** Cards of two different makes; a slow card beside a fast one; an NVLink pair; and anything needing eyes on a window, this machine having no display server.

```
    def default_device():
        import platform
        # Default to selecting the Metal compute device on Apple Silicon GPUs
        # (drivers are tightly integrated with macOS so pose no stability risk)
        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
            return 5
        return 0
```
— `Blender Sources/cycles_5.2/properties.py`, lines 1629–1635 (read 2026-07-30). Value 5 is `METAL` and value 0 is `NONE` in the list built by `get_device_types` (same file, lines 1641–1651). So on Apple Silicon the backend starts as Metal, and everywhere else it starts as None. That the file selects these values is observed; that a first-run user on Windows therefore gets CPU rendering until they change it was a step further — **inferred** — and the Linux half of it is now observed outright. On the rented Linux machine, with an untouched settings folder, `compute_device_type` read `NONE`, and a background render of a scene whose own Device is `GPU` completed on the processor at exit `0`, reporting `Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]` and saying nothing about it. On the Creator's Mac the same reading gave `METAL`. Windows is untested.

Two more things differ by system, from those two machines. Linux carries a **Display Graphics** section and an **Operating System Settings** section on the System page that macOS has not, and macOS carries `Kernel Optimization` and `MetalRT` controls that Linux has not. And `--gpu-device`, this version's way of naming the drawing card from outside, is documented as working only with the Vulkan backend, which macOS does not offer.

---

## 6. Can Blender be asked which graphics cards it can see and can use?

**LABEL: yes — documented for the display card, observed for the Cycles cards.**

Three separate answers, because the sources give three separate mechanisms.

### The card Blender draws its interface with — documented, and in the Python API

> "gpu.platform.devices_get() — Get all available GPU devices. Returns: List of GPUDevice objects for each device."
> "gpu.platform.renderer_get() — Get GPU to be used for rendering. Returns: GPU name."
> "gpu.platform.device_type_get() — Get GPU device type. Returns: Device type ('APPLE', 'NVIDIA', 'AMD', 'INTEL', 'SOFTWARE', 'QUALCOMM', 'UNKNOWN')."
> "gpu.platform.vendor_get() — Get GPU vendor."
> "gpu.platform.version_get() — Get GPU driver version."
> "gpu.platform.backend_type_get() — Get active GPU backend. Returns: Backend type ('OPENGL', 'VULKAN', 'METAL', 'NONE', 'UNKNOWN')."

— Blender 5.2 Python API, https://docs.blender.org/api/5.2/gpu.platform.html (read 2026-07-30)

And what each returned device carries:

> "class gpu.types.GPUDevice — Represents a GPU device. Variables: index – Device index. identifier – Device identifier. name – Device name."

— https://docs.blender.org/api/5.2/gpu.types.html (read 2026-07-30)

Note carefully: `devices_get()` is documented as "all available GPU devices" and `renderer_get()` as the "GPU to be used for rendering" — but this is the `gpu` module, which the API describes as "Python wrappers for the GPU implementation in Blender" for drawing (https://docs.blender.org/api/5.2/gpu.html). Reading `renderer_get()` as naming the card *Cycles* will render on would be a leap the sources do not support, and I do not make it.

One thing needed to use this without a window, new in 5.2 — a documented change:

> "Added `gpu.init()` to initialize gpu backend when running Blender in background mode `--background`."

— Blender 5.2 release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ (read 2026-07-30), and in the API:

> "gpu.init() — Initializes the GPU module for background use. If the initialization fails, a SystemError will be raised."

— https://docs.blender.org/api/5.2/gpu.html (read 2026-07-30)

### The cards Cycles can see and use — observed, in Cycles' own files, not in the API documentation

```
    def get_device_list(self, compute_device_type):
        import _cycles
        device_list = _cycles.available_devices(compute_device_type)
```
— `Blender Sources/cycles_5.2/properties.py`, lines 1709–1711 (read 2026-07-30). Preceded by a comment worth quoting because it is a warning:

```
    # Be careful when deciding when to call this function,
    # as Blender can crash with `_cycles.available_devices()` on some drivers.
```
— same file, lines 1707–1708.

And a second, simpler entry point:

```
def available_devices():
    import _cycles
    return _cycles.available_devices()
```
— `Blender Sources/cycles_5.2/engine.py`, lines 152–154 (read 2026-07-30).

What comes back is a list of tuples, and the file states the first three fields:

```
        # Layout of the device tuples: (Name, Type, Persistent ID)
```
— `properties.py`, line 1744, in `get_devices_for_type`. Later fields are used by index rather than named: `device[3]` for peer memory, `device[4]` for hardware ray tracing, `device[5]` for OpenImageDenoise support, `device[6]` for OptiX-denoiser support, `device[7]` for whether the kernel is optimized (`properties.py` lines 1753, 1968–1974, 1822, 1838). **What those later fields mean is inferred from how they are used**, and I write the step out: the file assigns `has_device_oidn_support = device[5]` at line 1822 and `has_device_optixdenoiser_support = device[6]` at line 1838, so the names come from the file's own variables, not from any declaration of the tuple's shape.

Ready-made questions Cycles asks of itself, all in `properties.py` (read 2026-07-30) — **observed**:

- `refresh_devices()` (line 1765) — re-queries every backend. Its comment: `# Refresh device list. This does not happen automatically on Blender startup due to unstable drivers that can cause crashes.` (lines 1763–1764). **This matters and I flag it: the device list is not populated by merely starting Blender.**
- `get_devices(compute_device_type='')` (line 1775) — marked `# Deprecated: use refresh_devices instead.` (line 1774). It calls `refresh_devices()` and returns `None`.
- `has_active_device()` (line 1809) — `return self.get_num_gpu_devices() > 0`
- `get_num_gpu_devices()` (line 1784) — counts enabled cards of the chosen backend
- `has_multi_device()` (line 1797), `has_oidn_gpu_devices()` (line 1812), `has_optixdenoiser_gpu_devices()` (line 1828)
- `get_compute_device_type()` (line 1779) — which backend is chosen

And the enabled/disabled state per card is the `use` switch on each `CyclesDeviceSettings` entry, quoted in answer 5.

**Crucially: none of this is in the 5.2 Python API documentation.** I searched all of `api_5.2` for `CyclesPreferences`, `available_devices`, `cycles.preferences` — **zero hits**. There is no `bpy.types.CyclesPreferences` page. The only Cycles page in the API is `bpy.ops.cycles`, which lists three operators, none about devices (https://docs.blender.org/api/5.2/bpy.ops.cycles.html, read 2026-07-30). So the Cycles device list is askable, but only via the add-on's own preferences object and the internal `_cycles` module, neither of which the published API documents.

### Whatever Blender itself reports — documented

> "To find out which graphics card and driver Blender is using, use Help ‣ Save System Info inside Blender. The OpenGL section will have information about your graphics card, vendor and driver version."

— Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/troubleshooting/gpu/windows/nvidia.html (read 2026-07-30; same text on all twelve per-vendor pages)

With an operator behind it:

> "bpy.ops.wm.sysinfo(*, filepath='') — Generate system information, saved into a text file"

— https://docs.blender.org/api/5.2/bpy.ops.wm.html (read 2026-07-30)

And a logging level that says it covers devices:

> "info: Information about devices, files, configuration, operations"

— https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30)

**Where I looked.** Read in full: `api_5.2/gpu.platform.txt`, the `GPUDevice` section of `api_5.2/gpu.types.txt`, `api_5.2/gpu.txt` (including `gpu.init()`), `api_5.2/bpy.types.PreferencesSystem.txt`, `api_5.2/bpy.types.Preferences.txt`, `api_5.2/bpy.app.txt` (`build_options`, `binary_path`, `portable`), the `sysinfo` sections of `api_5.2/bpy.ops.wm.txt` and `api_5.2/bpy.utils.txt`, `api_5.2/bpy.ops.cycles.txt`, and all of `Blender Sources/cycles_5.2/properties.py`, `engine.py`, `ui.py`. Searched `api_5.2` + `manual_5.2` + `cycles_5.2` for `get_devices`, `available_devices`, `compute_device_type`, `GPUDevice`, `devices_get`, `sysinfo`, `Save System Info`, `system_info`. Searched `release_notes_5.2` for `device` — one hit, about the compositor's device setting in the sequencer, unrelated. Ruled out: `gpu.capabilities`, `gpu.state`, `gpu.compute`, `gpu.shader` — listed in `gpu.txt` and checked; none enumerate devices.

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 5.2, from Blender's Python Console and the Help menu with the window open.

**The card Blender draws with.** `gpu.platform.backend_type_get()` returned `'METAL'`; `device_type_get()` returned `'APPLE'`; `renderer_get()` returned `'Metal API'`; `vendor_get()` returned `'Apple M1 Pro'`; `version_get()` returned `'1.2'`. `gpu.platform.devices_get()` returned:

```
[<GPUDevice index=0 identifier="METAL" name="Metal API">]
```

— a single entry with `index` 0, `identifier` `METAL` and `name` `Metal API`. `gpu.init()` was not called in this run.

**The cards Cycles can see and use.** In the same session, with the add-on key found to be the literal string `cycles` and `p = bpy.context.preferences.addons["cycles"].preferences`, the expression `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned, **before `refresh_devices()` was called**:

```
[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

and the same two entries after `p.refresh_devices()`. `p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, and `p.has_multi_device()` returned `False`. `_cycles.available_devices()` itself was not called, so the raw tuple's shape is still unrecorded.

So on this machine the two routes returned different names for the same hardware in the same session: `gpu.platform` gave one entry named `Metal API` with identifier `METAL`, and Cycles gave two entries named `Apple M1 Pro` and `Apple M1 Pro (GPU - 14 cores)` with identifiers `CPU` and `METAL_Apple M1 Pro (GPU - 14 cores)`. This is one machine with one graphics card; it records nothing about a machine with several.

**What Blender itself reports.** **Help ‣ Save System Info** wrote a file whose section headed `Cycles:` read, in full:

```
CPU device capabilities: 

Metal device capabilities:
Number of devices: 1
		Device: Apple M1 Pro (GPU - 14 cores)
```

and whose section headed `GPU:` read:

```
renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:
```

Only those two sections were copied back, so the rest of the file's layout is not recorded here.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender.app/Contents/MacOS/Blender` (`Blender 5.2.0 LTS`), all with no window open.

**`gpu.init()` does what the release notes say.** `-b --factory-startup --python-expr "import gpu; print('INIT:', gpu.init())"` printed `INIT: None`. Calling the platform functions *without* it first raised:

```
SystemError: GPU functions for drawing requires the gpu module to be initialized. See gpu.init.
```

and calling them after it printed:

```
BACKEND: METAL
DEVICETYPE: APPLE
RENDERER: Metal API
VENDOR: Apple M1 Pro
VERSION: 1.2
DEVICES: [<GPUDevice index=0 identifier="METAL" name="Metal API">]
```

**The Cycles route with no window open** also answered, and the raw call gave:

```
TYPES: (False, False, False, True, False, False)
METAL: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False, True, True, False, True))
```

so the entries carry eight fields, not the three the comment names. `_cycles.available_devices()` with no argument — the form `engine.py` wraps — was refused:

```
TypeError: function takes exactly 1 argument (0 given)
```

**Whether the stored list must be refreshed first.** With no window open, `[(d.name, d.type, d.id, d.use) for d in p.devices]` was **already populated** before `refresh_devices()` was called, on this machine and in this version. With `--factory-startup` it printed `[]` first and filled in after the refresh.

**The report route with no window open.** `bpy.ops.wm.sysinfo(filepath='/tmp/sysinfo-terminal-52.txt')` returned `{'FINISHED'}` and wrote the file. Its `Cycles` section matched the one written from the Help menu. Its `GPU` section was a single line:

```
GPU: missing, background mode
```

**The boundary this answer refused to guess at**, seen from one machine: in the same session `gpu.platform.devices_get()` returned one entry named `Metal API`, while Cycles returned two entries named `Apple M1 Pro` and `Apple M1 Pro (GPU - 14 cores)`. That is what came back; with one graphics card in the machine it does not settle what either list does where there are several.

**The whole file, both routes.** The file written from the Help menu has since been read end to end. Its headings, in order: `Blender:`, `Python:`, `paths:`, `Python (External Binary):`, `Directories:`, `scripts:`, `pref scripts:`, `FFmpeg:`, `Other Libraries:`, `GPU:`, `extensions:`, `Implementation Dependent GPU Limits:`, `GLSL:`, `Cycles:` (with `Metal device capabilities:` inside it), `Enabled add-ons:`. **There is no section headed `OpenGL`.** The background-written file drops `extensions:`, `Implementation Dependent GPU Limits:` and `GLSL:` altogether and replaces the `GPU:` section with the single line `GPU: missing, background mode`; its `Cycles:` section is identical to the menu-written one. So this report answers the Cycles half of the question with or without a window, and the drawing-card half only with one.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-5.2.0-linux-x64/blender`.

**`wm.sysinfo` with no window open, and what it actually writes.** It returned `RESULT: {'FINISHED'}` and wrote the file. Its whole `GPU` section is one line, as on the Mac:

```
GPU: missing, background mode
```

Its `Cycles` section on Linux is about ninety lines of card capabilities. Its opening and the two lines that carry the answer:

```
Cycles:
========================================


CPU device capabilities: AVX2

CUDA device capabilities:
	NVIDIA GeForce RTX 4090
		CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK			1024
		…
		CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR			8
		CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR			9
		…
```

The headings, in order, were `Blender:`, `Python:`, `paths:`, `Python (External Binary):`, `Directories:`, `scripts:`, `pref scripts:`, `FFmpeg:`, `Other Libraries:`, `GPU: missing, background mode`, `Cycles:`, `Enabled add-ons:`, with **no section headed `OpenGL`** — which is the section the manual describes.

**What `gpu.platform.devices_get()` names, against what Cycles names.** With no window, `gpu.init()` returned `None` and afterwards:

```
BACKEND: OPENGL
DEVICETYPE: NVIDIA
RENDERER: NVIDIA GeForce RTX 4090/PCIe/SSE2
VENDOR: NVIDIA Corporation
VERSION: 4.6.0 NVIDIA 580.105.08
DEVICES: [<GPUDevice index=0 identifier="OPENGL" name="NVIDIA GeForce RTX 4090/PCIe/SSE2">]
```

One entry, named for the drawing backend. Cycles, on the same machine, named the same physical card twice — `CUDA_NVIDIA GeForce RTX 4090_0000:00:07` and `…_OptiX` — and `--gpu-device help` listed three entries: the card twice and a software renderer. So the three routes count and name differently, which is as far as one card can take the question of whether `devices_get()` lists cards Cycles could use.

**A documented plain-text way to print a device list from the command line, in this version only.** `--gpu-device help` printed a list and exited, with no window and no display reachable (the machine with no display server at all is under question 6):

```
Blender GPU Device Listing (Vulkan):
Pass an Index or Device-ID to '--gpu-device'.
  Index  Device-ID    Name
  0      10de/2684/0  NVIDIA GeForce RTX 4090
  1      10de/2684/1  NVIDIA GeForce RTX 4090
  2      10005/0/2    llvmpipe (LLVM 15.0.7, 256 bits)
```

It is documented in Blender's own `--help` and it lists the drawing side's cards, not Cycles'.

**Whether the Cycles device list can be got without the crash the comment warns about.** It was asked for in every version, many times over two days, on both machines, and Blender did not crash on it once. That is what happened, not a verdict on the warning.

**What the `Persistent ID` looks like.** On Linux, `CUDA_NVIDIA GeForce RTX 4090_0000:00:07` and `CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX` — the technology, the card's name, and its place on the bus. On the Mac, `METAL_Apple M1 Pro (GPU - 14 cores)`. It was identical across every run of the day on both machines. Whether it survives a driver update or a card being moved is untested; neither machine allows either.

**One more thing the Mac could not show:** `_cycles.available_devices('HIP')` and `_cycles.available_devices('ONEAPI')` — technologies built into this copy with no card present — returned the processor entry alone and no error, exactly as `'METAL'` did, which is not built in at all.

**Not covered.** Whether `gpu.platform.devices_get()` lists cards Cycles could use, or only cards the drawing backend can use — one card cannot settle it, though the three routes did give different names and counts. Whether the stored identifier survives a driver update or a card being moved. What any of this returns on a machine with several cards.

**Outside the interface.** Yes, with a caveat and a gap. `gpu.init()` exists specifically so the `gpu` module works under `--background` (documented, above), and `bpy.ops.wm.sysinfo` takes a filepath so it can be driven by a script. For Cycles, the device list comes from a preferences object reachable from Python, and the file explicitly says the list is **not** built at startup and needs `refresh_devices()` first — so a script that reads `preferences.addons['cycles'].preferences.devices` without refreshing first may find it empty. That is **inferred**, and the step is: the comment at lines 1763–1764 says the refresh "does not happen automatically on Blender startup", and `devices` is only ever added to inside `update_device_entries`, which only runs from `get_device_list`. The hand tests bear on that inference and now cover the background case. With no window open the `devices` collection was **already populated** before `refresh_devices()` was called, as it was inside the window — so a script reading it without refreshing did not find it empty on this machine. Adding `--factory-startup` did make it print `[]` before the refresh. `gpu.init()` returned `None` with no window and made the whole `gpu.platform` module answer there, `devices_get()` included; without it the calls raise a `SystemError` naming `gpu.init`. `bpy.ops.wm.sysinfo` returned `{'FINISHED'}` and wrote its file from the operator with no window, with the `Cycles` section intact and the `GPU` section reduced to `GPU: missing, background mode`.

**Operating system.** The mechanisms are the same on all three. What they return is not, and two systems are now tested. On the Creator's Mac `backend_type_get()` returned `'METAL'` and `device_type_get()` returned `'APPLE'`; on the rented Linux machine, after `gpu.init()`, `'OPENGL'` and `'NVIDIA'`. The Cycles device query is per backend, and which backends exist is per operating system (answer 2) — `(False, False, False, True, False, False)` on the Mac against `(True, True, True, False, True, True)` on Linux. The manual's "Save System Info" instruction is given identically on the Windows, Linux and macOS troubleshooting pages, and the file it writes differs in content: a short Metal block on the Mac and a long CUDA capability dump on Linux. A fourth route, `--gpu-device help`, exists only where the Vulkan backend does, so not on macOS. Windows remains untested.

### Hand tests on the two-card machine — observed

Run on **2026-08-06** on the second rented Linux machine described under question 5 — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s on driver 580.95.05, sixty-one cores, **no display server at all**, over SSH — against `/opt/blender/blender-5.2.0-linux-x64/blender`. Label: **observed**. Not merged with the one-card rented machine's results or the Mac's.

**Every route answered with no window and no display server, and every one named both cards:**

```
BACKEND: OPTIX
TYPES: (True, True, True, False, True, True)
CUDA: (('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', False, False, True, False, True), ('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a', False, False, True, False, True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False, False, True, False, True))
OPTIX: (('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', False, False, True, True, True), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX', False, False, True, True, True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False, False, True, False, True))
HAS ACTIVE: True
NUM GPU: 2
MULTI: False
```

**`get_num_gpu_devices()` counts the cards and returns 2**, and **`has_multi_device()` returns `False` with two cards ticked** — so that name is not about having more than one card.

`bpy.ops.wm.sysinfo(filepath=…)` returned `{'FINISHED'}` and wrote its file with no display server; its `Cycles` section carries one `CUDA device capabilities:` heading and then the card's name **twice**, at lines 83 and 172 of the file.

**`--gpu-device help` lists both cards, with no window and no display server:**

```
Blender GPU Device Listing (Vulkan):
Pass an Index or Device-ID to '--gpu-device'.
  Index  Device-ID    Name
  0      10de/2684/0  NVIDIA GeForce RTX 4090
  1      10de/2684/1  NVIDIA GeForce RTX 4090
```

### Hand tests on the four-card machine — observed

Run on **2026-08-06** on the third rented Linux machine described under question 5 — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, sixty-one cores, 197 GB of memory, **no display server**, over SSH — against `/opt/blender/blender-5.2.0-linux-x64/blender`. Label: **observed**. A fourth machine; not merged with the Mac's results, the one-card machine's or the two-card machine's. The full list is `Audits/Blender Research/The machine/Hand tests — four cards.md`.

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

**Not covered here.** Cards of two different makes, an NVLink pair, and anything a window would show.

---

## Things a person with Blender 5.2 open could settle by hand in a few minutes

This list has since been run on two machines. What the Creator's own MacBook Pro settled is written into the answers above as **Hand test — observed**, and what the rented Linux machine settled as **Hand tests on the rented Linux machine — observed**. The two lists live at `Audits/Blender Research/The machine/Hand tests.md` and `Audits/Blender Research/The machine/Hand tests — Linux.md`. **A third machine has since been used as well** — a second rented Linux machine with two graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the two-card machine — observed** and **Hand tests on a machine with no display server at all — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`. *(Added 2026-08-06 by the citation check of the two-card material: this closing list still said two machines.)* **A fourth machine has since been used as well** — a third rented Linux machine with **four** graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the four-card machine — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

### Still open

Nothing on this list is still open. All six items were run on both machines and what came back is recorded under questions 1, 2, 3, 4, 5 and 6 above. Item 1 was answered only as far as the Mac reached — every device unticked, and the technology set to `NONE`. The rented Linux machine reached the real case: with the technology set to one for which it has no card at all, a windowless render still printed nothing naming a missing device and completed on the processor, and the missing-requirement text was found only in the preferences panel.

**And one item that could not be run at all is now closed.** The rented Linux machine is exactly what "a machine without a supported Intel card but with the oneAPI backend on offer" describes, and it drew the requirement text. The figure 5.2 puts on screen is `XX.XX.37435.3` — the shipped file's number, not the manual's `XX.XX.34666.3`. That is recorded under question 2.

### Cannot be run on the equipment available

The equipment is one MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, whose graphics are Apple's own and built in, and one rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores — reached over SSH, on which nothing may be installed or removed. **Brought up to date 2026-08-06 by the citation check of the two-card material.** A third machine has since been used — a second rented Linux machine, Ubuntu 22.04.5 LTS, **two** NVIDIA GeForce RTX 4090s on driver 580.95.05, sixty-one cores, **no display server at all**, over SSH — so "no second card" no longer holds, and what it settled is in the two-card block above. There is still no pair of cards joined by NVLink: this pair talks through the host bridge. The one-card rented Linux machine was destroyed on 2026-08-06 and nothing can be run on it again. And its driver *was* removed and reinstalled on the two-card machine, on purpose, for the broken-driver test recorded in the first-pass findings for this version.

**A fourth machine has since been used, and it is the only one still alive.** A **third rented Linux machine** — Ubuntu 22.04.5 LTS, **four** NVIDIA GeForce RTX 4060 Ti of **16 GB each** on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, **197 GB of memory**, **no display server**, over SSH. What it settled is written into the answers above as **Hand tests on the four-card machine — observed**, and its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. Three limits above are narrowed by it and none is removed: more than two cards is no longer out of reach, a card small enough to be filled is no longer out of reach, and the several-card form of every question under 5 and 6 has now been asked twice on two different machines. **Still out of reach on it as well:** any pair joined by NVLink, these four talking through the host bridge; cards of two different makes; cards of unequal speed, all four being the same model within about 2 % of each other; and anything needing eyes on a window, this machine having no display server either. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

- **Watching a switched-off card go idle during a render.** That the per-card switch can be set from a script, and that setting it moves the panel's tick and changes what a windowless render does, is settled on both machines and recorded under question 5. Watching one card fall idle while another works needs two or more cards of the same backend.
- **Whether the manual's OpenGL 3.3 or the requirements page's OpenGL 4.3 governs.** Needs a machine or virtual machine whose driver reports OpenGL between the two. The rented machine reports `4.6.0 NVIDIA 580.105.08`.
- **Whether the shipped Metal number or the manual's is shown.** Needs a Mac old enough for the requirement line to be drawn; on macOS 26.5.2 a Metal device is found and no requirement text appears, and Linux offers no Metal button at all. The oneAPI half of this item is now closed, above.
- **Whether the figures the panel draws for Intel and AMD are enforced**, and the Intel **Windows** figure. The first needs a real Intel or AMD card with an old driver; the second needs a Windows machine, the panel text branching on the operating system.

---

2026-08-01 — Claude Opus 5 (Claude Code) — The machine second pass, Blender 5.2

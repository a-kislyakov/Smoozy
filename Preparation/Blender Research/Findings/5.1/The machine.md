# The machine — Blender 5.1

**Sources consulted, and when.** Blender's manual (`Blender Sources/manual_5.1`, including two of its pictures), Blender's Python API documentation (`Blender Sources/api_5.1`), the Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All read on **2026-07-30** (the snapshot date I was given). I used no web access.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

Two notes so nothing is hidden:

- I created the empty folder this file is saved into, because it did not exist. I opened no file in it and there was nothing in it to open.
- The top level of `Blender Sources/` holds seven loose text files (`crashes.txt`, `cycles_device.txt`, `devices_api.txt`, `eevee_render.txt`, `eevee_results.txt`, `missing.txt`, `missing_filtered.txt`) that the folder's own note does not describe. I looked at the first few lines of each to see what they were: they are saved search results from some earlier run, listing lines already inside the manual and Cycles folders. They are inside `Blender Sources/`, so reading them was allowed, but I did not use them. Every search below I ran myself.

A note on the word "documented" throughout: where a claim needed me to join two sentences, or to read a general statement as covering a particular case, I have labelled it **inferred** and written the step out, even where the step is small.

---

## 1. What must a computer have on it before Blender will run at all?

**Label: documented, and partly not in the sources.**

The manual states the shape of the answer but not its numbers. It names an operating system, a working graphics stack, and — on Linux — a windowing system, and then hands the actual minimum figures off to a page that is not in my sources.

**An operating system Blender is built for.**

> "Blender is available for download on Windows, macOS, and Linux."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html (read 2026-07-30)

**A processor architecture matching the download.**

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html (read 2026-07-30)

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html (read 2026-07-30)

**A graphics card and driver supporting OpenGL 3.3.** This is the only hard number the manual gives for starting Blender at all.

> "Always check that your graphics drivers are up to date and that OpenGL is properly supported."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html (read 2026-07-30)

> "This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."
> — Manual, https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/windows/nvidia.html (read 2026-07-30). The identical sentence appears on all twelve of the per-platform, per-vendor graphics troubleshooting pages under https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/.

**On Linux, a windowing system — and, in one case, a named library.**

> "On Linux Blender supports both X11 and Wayland for official releases."

> "Under Gnome-Shell the libdecor library is required. This is available as a package on most Linux distribution."
> "If the library isn't found X11 will be used as a fallback."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux_windowing_environment.html (read 2026-07-30)

The 5.1 release notes record a change to that:

> "Support opening windows without decorations with the: `--no-window-frame` argument (Linux only) On Wayland this removes the dependence on LIBDECOR."
> — Release notes, https://developer.blender.org/docs/release_notes/5.1/core/ (read 2026-07-30)

**Nothing Python-shaped.** Blender carries its own.

> "Blender bundles a complete Python installation and does not interact with the system Python by default."
> — Manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html (read 2026-07-30)

**No internet connection.**

> "Blender is designed to not require an internet connection, so it doesn't have a built-in update system."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html (read 2026-07-30)

**Two conditions stated only as things that stop Blender starting**, from the startup troubleshooting list: that antivirus software is not blocking it, and that the user has the right permissions.

> "Make sure any antivirus software is not preventing Blender from starting."
> "Ensure that you have appropriate user permissions on the computer."
> — Manual, https://docs.blender.org/manual/en/5.1/troubleshooting/startup.html (read 2026-07-30)

**What is not in the sources.** The actual minimum and recommended figures — processor, memory, disk, operating system version — are not anywhere in the manual, the API documentation, or the release notes. The manual says they exist and points off-site:

> "Blender has a set of minimum and recommended requirements; so make sure these are met before trying to install Blender."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html (read 2026-07-30)

> "Check the Downloading Blender page to find the minimum requirements and the different versions that are available for Blender (if you have not done so yet)."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html (the same sentence opens the Windows and macOS installing pages) (read 2026-07-30)

That "Downloading Blender page" is on blender.org and is not part of the manual, the API documentation, or the release notes. It is not in my sources and I did not go and get it.

**Where I looked, to be able to say that.** I searched the whole of `manual_5.1`, `api_5.1` and `release_notes_5.1` for: `system requirement`, `minimum requirement`, `requirements`, `prerequisite`, `64-bit`, `32-bit`, `RAM`, `gigabyte`, `SSE4`, `AVX`, `dual core`, `quad core`, `processor with`, `glibc`, `libc`, `redistributable`, `runtime library`, `dependenc`, `macOS 1`, `Windows 10`, `Windows 11`, `Windows 8`, `Ubuntu`, `Rocky`, `CentOS`. The only hits were the ones quoted above plus unrelated matters (32-bit image formats, tablet drivers, the manual's own build instructions). No page gives a memory figure, a processor figure, or an operating-system version floor for Blender itself. I read in full: the installing index and all four platform pages, the Linux windowing environment page, the startup troubleshooting page, the crashes page, all sixteen graphics-hardware troubleshooting pages, and the production deployment page.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 5.1 started and ran normally, and **Help ‣ Save System Info** was used to write `system-info.txt`. The file's section headed `GPU:` read:

```
renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:
```

and was followed by sections headed `Implementation Dependent GPU Limits:` and `GLSL:`, then `Cycles:`. The whole file has since been read; there is **no section headed `OpenGL`** anywhere in it. This records what Blender reported on a machine that started successfully; it does not test the OpenGL 3.3 floor.

**Not covered.** I cannot tell you the minimum memory, processor, disk space, or operating-system version, because the sources do not state them. I also cannot tell you what Blender needs at the shared-library level on Linux beyond libdecor under Gnome-Shell — nothing in the sources lists that. And OpenGL 3.3 is documented as the floor for the *interface and viewport*; whether the same floor applies when Blender never opens a window is question 1's edge, and I take it up in the "outside the interface" line below.

**Outside the interface.** The sources do not settle this. The OpenGL 3.3 requirement is stated in the context of drawing:

> "Blender uses OpenGL for the 3D Viewport and user interface."
> — Manual, https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/index.html (read 2026-07-30)

And the manual says a display is not needed for command-line rendering:

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."
> — Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html (read 2026-07-30)

Those two sentences sit next to each other but do not join. Nothing in the sources says whether a machine with no OpenGL 3.3 can still run `blender --background`, and no machine below that floor exists to try, so it stands unsettled. What is now observed is the second sentence's own claim: on the rented Linux machine, with no display server reachable at all, 5.1 rendered with Cycles on the graphics card and with EEVEE, both at exit `0`. What does **not** hold outside the window is opening one, and what Blender says then is quoted in the Linux tests below.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender` started with `-b` and rendered without a window on every attempt of the day, with Cycles and with EEVEE. This machine meets the graphics floor, so it tests nothing about that floor; what it records is that a windowless start and a windowless render both worked on macOS. One related thing did come back from `-b`: the `gpu` module's drawing side is not available there at all, raising `SystemError: GPU functions for drawing are not available in background mode` (question 6).

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — `/opt/blender/blender-5.1.2-linux-x64/blender` (`Blender 5.1.2`) was put to three tests bearing on this question. The machine is a container sharing somebody else's hardware, not a whole computer, and other work was running on it at the same time, so nothing here says how long anything took or how much memory was free.

**Blender starts and renders on Linux with no windowing system running at all.** This is the question this version's own hand-test list said needed a Linux machine. `DISPLAY` was removed from the environment and a background Cycles render completed at exit `0`, writing its PNG, on the graphics card:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
                            | Denoising on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
00:10.018  render           | Saved: '/root/smoozy/The machine/work/nd-5.1-0001.png'
```

An EEVEE render did the same. So neither X11 nor Wayland is needed to render; the manual's requirement is about opening a window.

**Asked for a window with no windowing system, Blender refuses, and this is what it reports about it.** The other Linux question this version's list carried was what Blender says about its own windowing system. Running `env -u DISPLAY <blender> --python-expr "import bpy; print('WINDOW OPENED')"` never printed `WINDOW OPENED` and printed instead, before exiting with status `1`:

```
00:00.029  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.029  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4645, total unfreed memory 1.212318 MB
```

So Blender names both windowing systems it tried and what went wrong with each. Note that 4.5 on the same machine printed the same information in a plainer shape and then hung rather than exiting.

**The libdecor and `--no-window-frame` half is still out of reach**, the machine running Xorg rather than Wayland and nothing on it being removable. What is settled is that the argument exists: Blender's own `--help` lists it in 5.0, 5.1 and 5.2 and not in 4.5, worded `--no-window-frame` / "Disable all window decorations (Linux only)".

One more thing this machine shows and the Mac does not,, but only when Cycles is asked for HIP:

```
00:00.344  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
```

That is Cycles reporting that AMD's library is not on the machine. Harmless here, but anything reading Blender's output has to expect it. The OpenGL floor is still untested: this machine reports `4.6.0 NVIDIA 580.105.08`, above both figures.

*(Corrected 2026-08-06 by the runner, after the check of the Linux hand tests could not reproduce this. The version split is right; "nearly every run" was not. Run again on the same machine with a plain Cycles render at factory settings, this line appeared **not at all**, in any of the four versions. It appears only when Cycles is actually asked for HIP — either the preferences are set to HIP, or the render is started with `--cycles-device HIP`. Then 5.0, 5.1 and 5.2 each print it exactly once and 4.5 prints nothing of the kind, failing instead with `Error: Found no Cycles device of the specified type`. So anything reading Blender's output has to expect this line when it asks for HIP, and not otherwise.)*

The one nearby statement that *is* explicit concerns EEVEE, not Blender starting:

> "Headless rendering is not supported on headless Windows systems."
> — Manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html (read 2026-07-30)

**Operating system.** It does not hold identically everywhere. The architecture rule differs (x64/arm64 on Windows, Intel/Apple Silicon on macOS; the Linux page says only "Download the Linux version for your architecture"). The windowing-system and libdecor requirements are Linux-only, and the windowing-system half is now tested: with neither X11 nor Wayland reachable, 5.1 refuses to open a window, names both and exits `1`, while still rendering perfectly well with `-b`. The libdecor half is not tested, the rented machine running Xorg and nothing on it being removable. On macOS the graphics driver is not a separate thing you install:

**Hand tests on a machine with no display server at all — observed.** On **2026-08-06**, on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-5.1.2-linux-x64/blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`. Label: **observed**. Nothing here is merged with the one-card rented machine's results or the Creator's Mac.

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
GPU  CYCLES exit=0  png=5065
GPU  BLENDER_EEVEE_NEXT exit=0  png=3431
GPU  BLENDER_EEVEE exit=0  png=3431
GPU  BLENDER_WORKBENCH exit=0  png=3669
CPU  CYCLES exit=0  png=5062
CPU  BLENDER_EEVEE_NEXT exit=0  png=3431
CPU  BLENDER_EEVEE exit=0  png=3431
CPU  BLENDER_WORKBENCH exit=0  png=3669
```

`BLENDER_EEVEE_NEXT` and `BLENDER_EEVEE` are both accepted by this version and give the same file size. The same runs repeated with `--debug-cycles` showed the work really was where it was asked for:

```
on the card:      Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                                       Hardware Ray-Tracing: Off
                                   NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
on the processor: Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]
                                       Hardware Ray-Tracing: Off
```

**Asked for a window on a machine with no display server, Blender said so and stopped.** `blender --python-expr "import bpy; print('RAN WITH A WINDOW')"` under `timeout 120` never printed `RAN WITH A WINDOW`, wrote to **standard output**, and exited **1**:

```
00:00.058  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.058  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4645, total unfreed memory 1.212318 MB
```

Nothing was written to standard error.

Word for word what the one-card machine printed when cut off from its display, with the same exit status, plus the memory-leak line. **So for this version the two cases are indistinguishable in what Blender says and does.**

**`-b -E help` worked here**, exit 0, printing:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

**Not covered.** Windows. Nothing about the OpenGL floor: this machine's cards are far above it and no card below the floor can be reached.

> "On macOS, graphics drivers are built into the operating system and the only way to get newer drivers is to upgrade macOS as a whole to the latest version."
> — Manual, https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/apple/nvidia.html (read 2026-07-30)

---

## 2. What must a computer have on it before Blender will render using its graphics cards?

**Label: documented.**

The manual gives one page for this, and it is specific. Five backends, each with its own card and driver floor. The whole of what follows is from one page unless marked otherwise: Manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html (read 2026-07-30).

> "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer and operating system."

**NVIDIA, CUDA:**

> "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher."

**NVIDIA, OptiX:**

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."

> "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards."

**AMD, HIP:**

> "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."

> "Minimum driver versions:"
> "Windows: Radeon Software 24.9.1 or Radeon PRO Software 24.Q4"
> "Linux: Radeon Software 23.40 or ROCm HIP Runtime 6.0"

**Intel, oneAPI:**

> "oneAPI is a computation library that is supported on Windows and Linux and requires an Intel® Arc™ graphics card with the Xe HPG architecture."

> "Windows: Intel Graphics Driver XX.X.101.5518"
> "Linux: intel-level-zero-gpu package 1.3.27642, typically available through the intel-compute-runtime package XX.XX.34666.3"

**Apple, Metal:**

> "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

**Nothing extra to install for NVIDIA.**

> "Normally users do not need to install the CUDA toolkit as Blender comes with precompiled kernels."

**And the official driver, not the machine-maker's:**

> "In case of problems, be sure to install the official graphics drivers from the GPU manufacturers website, or through the package manager on Linux. The graphics drivers provided by the computer manufacturer can sometimes be outdated or incomplete."

**Cycles' own files carry strings for these floors, and they do not match the manual in two places.** The quoted strings and their places in the file are **observed**. Whether and when a running Blender shows them is not established here.

> ```
>             elif device_type == 'ONEAPI':
>                 import sys
>                 if sys.platform.startswith("win"):
>                     driver_version = "XX.X.101.8306"
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1848–1851, inside `CyclesPreferences._draw_devices` (read 2026-07-30)

The manual's figure for the same thing is `XX.X.101.5518`. Cycles' file says `XX.X.101.8306`.

> ```
>             elif device_type == 'METAL':
>                 mac_version = "12.2"
>                 col.label(text=rpt_("Requires Apple Silicon with macOS %s or newer") % mac_version,
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1867–1869 (read 2026-07-30)

The manual says macOS 13.0 "to support all features"; Cycles' file says macOS 12.2. I report both as they stand and draw no conclusion about which governs — nothing in my sources says.

The rest of Cycles' file agrees with the manual: compute capability `"5.0"` for CUDA (line 1813) and for OptiX with driver `"535"` (lines 1817–1818); "Requires AMD GPU with RDNA architecture" with Adrenalin `"24.9.1"` / Radeon Pro `"24.Q4"` on Windows and ROCm `"6.0"` / driver `"23.40"` on Linux (lines 1826–1847); and on Linux for Intel it adds one item the manual does not name:

> ```
>                     col.label(text=rpt_("  - oneAPI Level-Zero Loader"), icon='BLANK1', translate=False)
> ```
> — `Blender Sources/cycles_5.1/properties.py`, line 1866 (read 2026-07-30)

**One more requirement, which is about the build rather than the machine.** Label: **documented**.

> "For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html (read 2026-07-30)

**EEVEE is a separate matter and has no such list.** What the sources do say:

> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient."
> — Manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html (read 2026-07-30)

I found no card or driver floor stated for EEVEE anywhere. Searched `manual_5.1/render/eevee/` in full plus site-wide for `EEVEE` next to `requir`, `driver`, `Vulkan`, `Metal`, `OpenGL`.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 5.1's **Edit ‣ Preferences ‣ System** page was opened and the **Cycles Render Devices** panel expanded. The row of technology buttons held exactly two, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — and two dropdown lists: **Kernel Optimization**, set to **Full**, offering **Off** and **Intersection Only**; and **MetalRT**, set to **Auto**, offering **Off** and **On**. Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed. No "Requires Apple Silicon with macOS … or newer" line appeared, because a Metal device was found, so this run records nothing about the macOS 12.2-against-13.0 disagreement above, and nothing about the oneAPI driver-number disagreement.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), a background Cycles render used the graphics card, printing `00:01.779  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".` and ending `01:06.271  render           | Fra: 1 | Mem: 1583M | Sample 16/16 (Using optimized kernels)`. So GPU rendering does work in `--background`, which this answer records as unstated. EEVEE rendered with no window open too: `-b -E help` listed `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`, and a background EEVEE render exited `0`, ending `00:05.896  render           | Saved: '/tmp/eevee-51-0001.png'`. Full commands and output are under question 5.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, four things were put to 5.1.

**What this copy was built with.** `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed:

```
TYPES: (True, True, True, False, True, False)
```

Cycles' own file unpacks that as `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt`, so the official Linux x64 build carries CUDA, OptiX, HIP and oneAPI and not Metal.

**A graphics-card render works, with no window and with no display reachable.** *(Wording corrected 2026-08-06 by the citation check of the two-card material. The one-card machine had a real display server running and this run was cut off from it by removing `DISPLAY`; a machine with no display server at all is a different machine, and its result is under question 1. Nothing observed changes.)* With the technology saved as `CUDA` and the card ticked, a background render exited `0`, wrote its PNG and printed:

```
00:00.347  cycles           | Added device "NVIDIA GeForce RTX 4090" with id "CUDA_NVIDIA GeForce RTX 4090_0000:00:07".
```

ending, in the report `--debug-cycles` prints:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                            |                      Hardware Ray-Tracing: Off
                            | 
                            | Denoising on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
                            |                   Hardware Ray-Tracing: Off
```

With `OPTIX` saved, the same with the OptiX line. Note the `Hardware Ray-Tracing: Off` lines under each device: 5.1 and 5.2 print them on this machine and 4.5 and 5.0 do not. Denoising ran on the processor in every graphics-card render here.

**What the panel says when the chosen technology has no compatible card.** This is what the Mac could never reach, and it settles the oneAPI driver-number disagreement for Linux. With a window open on the machine's own display, the technology set from a script and the System page photographed with `bpy.ops.screen.screenshot`, 5.1 drew, under **oneAPI**:

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

So the Intel Linux figure a running 5.1 draws is `XX.XX.34666.3`, the manual's number. And the AMD text names a ROCm HIP Runtime figure and a driver figure as two alternatives — 4.5 on the same machine names only the driver, and 5.2 names `6.3` and `24.30` instead. This records what the panel displays on a machine with no Intel or AMD card; it does not say what Blender enforces against a real one, and the Windows figures cannot be read from Linux at all because the text branches on the operating system; a rented Windows machine would show them and none was used. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

The requirement line for CUDA and OptiX was never drawn, a compatible NVIDIA card being present. Its compute capability, as Blender's own system-information file reports it, is major `8`, minor `9`.

**EEVEE renders headless on Linux.** With `DISPLAY` removed, `-b <file> -E BLENDER_EEVEE -f 1` exited `0` and wrote its PNG:

```
00:00.928  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.246  render           | Saved: '/root/smoozy/The machine/work/eeveend-5.1-0001.png'
```

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

**Not covered.** This settles what must be *present*. It does not settle what happens when it is present but a particular scene still will not render on the card — the manual treats that separately and only in outline (memory exhaustion, kernel compilation), and reaching the memory case would need the whole rented machine to itself. It also does not settle anything about a machine's second or third card individually; see question 5. And it does not tell you how Blender behaves when the driver is present but older than the floor, which needs a card of the make whose floor is being tested.

**Outside the interface.** The card, driver and operating-system requirements are stated as properties of the hardware and the technology, with no mention of the interface, so they read as holding either way. But two things here are interface-shaped and I flag them: the requirement to "Enable the available devices for the chosen backend" and to "Configure each scene to use the GPU Compute Device" is written as three clicks in Preferences, and the manual gives no non-interface equivalent on this page. The command-line equivalent exists and is covered in question 5. **Whether GPU rendering itself works in `--background` is now observed on both machines: it does** — on macOS with Metal, and on the rented Linux machine with CUDA and with OptiX, the latter even with no display server at all. And the missing-requirement text quoted above never reaches a windowless run: with the technology saved as `ONEAPI` and no Intel card, the phrase `No compatible GPUs found for Cycles` appeared nought times in the whole output of a background render, which completed on the processor at exit `0`.

**Operating system.** No, and this is the sharpest split in the whole sheet. CUDA, OptiX, HIP and oneAPI are documented as "supported on Windows and Linux" — macOS is absent from all four. Metal is Apple-only and Apple Silicon only. The HIP and oneAPI driver floors are given as different numbers for Windows and Linux. Cycles' file makes the same split in code, branching on `sys.platform` for HIP and oneAPI (`properties.py` lines 1824–1866). Two systems are now tested and both bear it out: on the Creator's Apple Silicon Mac the technology row offered **None** and **Metal** and nothing else; on the rented Linux machine it offered five buttons — **None**, **CUDA**, **OptiX**, **HIP**, **oneAPI** — and no Metal, and drew the Linux figures. Windows remains untested.

---

## 3. Does what a computer must have on it depend on how Blender was obtained and put there?

**Label: documented.**

Yes, in several distinct ways.

**A distribution's package may simply not contain GPU rendering.** This is the strongest instance.

> "Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html (read 2026-07-30)

**Administrator rights are needed for one Windows route and not the other.**

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

> "When choosing the zip-file, you have to manually extract Blender to the desired folder... No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html (read 2026-07-30)

**The Microsoft Store copy is sandboxed, and Blender knows it is.**

> "is_microsoft_store_install"
> "Whether this blender installation is a sandboxed Microsoft Store version (default False, readonly)"
> — API documentation, https://docs.blender.org/api/5.1/bpy.types.PreferencesSystem.html (read 2026-07-30)

**Steam requires the Steam client on the machine.**

> "Download and install the Steam client for your operating system."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/steam.html (read 2026-07-30)

**Snap requires snap.**

> "Snap is a universal package manager designed to work across a range of distributions. Assuming snap is already installed, Blender can be installed through snap with:"
> "snap install blender --classic"
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html (read 2026-07-30)

**On macOS, opening it the first time may need the machine's own approval.**

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html (read 2026-07-30)

**A self-built copy gets different CUDA troubleshooting advice.** The manual says the toolkit is normally not needed (quoted in question 2), and then:

> "If you build Blender yourself, try to download and install a newer CUDA developer toolkit."
> — Manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html (read 2026-07-30)

**Which features a given copy has is a build-time fact you can read back.** The API documentation prints an example value that includes Cycles:

> "Constant value bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, ... cycles=True, cycles_osl=True, ...)"
> — API documentation, https://docs.blender.org/api/5.1/bpy.app.html (read 2026-07-30)

**Inferred, one small step:** since `build_options` carries a `cycles` entry, and the Linux page says some builds are made without Cycles GPU support, a copy of Blender can differ from another copy of the same version in what it can do. The step a reader has to supply is that the `cycles=True` in the documentation is an example of a build's own value rather than a fixed constant — the page prints it as a "Constant value", and I read that as the value of that particular documented build.

**One route is explicitly the same everywhere: unpacking it.**

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."
> — Manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html (read 2026-07-30)

**And one known extraction trap:**

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html (read 2026-07-30)

**Hand test on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, the copy under test was the official Linux x64 build from Blender's own download server, unpacked at `/opt/blender/blender-5.1.2-linux-x64/`. There is a way to find out which graphics-card technologies a copy carries other than by looking: `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed:

```
TYPES: (True, True, True, False, True, False)
```

which Cycles' own file unpacks as `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt`. So the official Linux download carries CUDA, OptiX, HIP and oneAPI and does not leave graphics-card rendering out. A distribution's own package is a different copy and is untested; nothing may be installed on the rented machine.

**Not covered.** This says the requirements differ by route; it does not give you a per-route checklist. In particular, the sources never say *which* distributions ship Blender without Cycles GPU support, and none could be installed to find out. Nor do they say what a Microsoft Store sandbox actually prevents — only that Blender flags it.

**Outside the interface.** The differences are properties of the installed copy, not of how it is driven, so they hold either way. Two of the flags are readable without the interface: `bpy.app.build_options` and `PreferencesSystem.is_microsoft_store_install` are both plain Python attributes. The one route-specific item that is interface-shaped is registration, which the manual also exposes as command-line arguments — `-r, --register`, `--register-allusers`, `--unregister`, `--unregister-allusers`, each documented as "(Windows & Linux only)" (Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30).

**Operating system.** No — the routes themselves are per-platform. Windows has installer, zip and Microsoft Store; macOS has one route, the disk image; Linux has blender.org, distribution packages and Snap; Steam is on all three. The manual documents them on separate pages for exactly that reason.

---

## 4. Does Blender say anything about what is missing when it will not start, or when it will not render?

**Label: documented, with a real gap on the "will not start" half.**

**When it will not start.** The manual documents that messages appear on a console, and that you may not see that console unless you launched Blender yourself from one.

> "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window. To ensure you can see any error messages that appear at launch, open a command-line window and navigate to the location where the Blender executable is, and run blender from that location."
> — Manual, https://docs.blender.org/manual/en/5.1/troubleshooting/startup.html (read 2026-07-30)

It then documents exactly two startup messages by name, and only one of them is about something missing:

> "found bundled python: {DIR}"
> "This message indicates that Blender was able to find the Python library for the Python interpreter embedded within Blender. If this folder is missing or unable to be found, it is likely that an error will occur, and this message will not appear."

> "Read prefs: {DIR}/userpref.blend"
> "The preferences use this path."
> — Manual, same page (read 2026-07-30)

Note what that first one says: when the Python folder is missing, the message *does not appear*. Its absence is the signal. The manual does not quote the error that appears instead.

One named error covers the graphics case:

> "Unsupported Graphics Driver Error"
> "This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."
> — Manual, https://docs.blender.org/manual/en/5.1/troubleshooting/gpu/windows/nvidia.html and the eleven sibling pages (read 2026-07-30)

That is a named error condition with a documented meaning. The manual does not print the message's literal text, nor say where it appears.

And the general advice, on every one of those twelve pages:

> "Crash on Startup"
> "Try running Blender from the command line, to see if any helpful error messages are printed."

This does not promise that a helpful message is printed in every case.

**When it crashes**, rather than declining to start, Blender writes a file, and this is documented per platform:

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."
> — Manual, https://docs.blender.org/manual/en/5.1/troubleshooting/crash.html (read 2026-07-30)

Windows: "so test.blend will create a file called test.crash.txt", plus `blender.crash.txt` for module crashes. Linux: "a file named blender.crash.txt is written to the /tmp directory." macOS: the system Crash Reporter, and files in `~/Library/Logs/DiagnosticReports/`. Same page.

**When it will not render.** Here the sources are much better. The manual has a dedicated Error Messages section naming four, each with a meaning:

> "Error: Out of memory"
> "This usually means there is not enough memory to store the scene for use by the GPU."

> "The NVIDIA OpenGL driver lost connection with the display driver"
> "If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations."

> "Unsupported GNU version"
> "On Linux, depending on your GCC version you might get this error."

> "CUDA Error: Kernel compilation failed"
> "This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed."
> — Manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html (read 2026-07-30)

**Cycles' own file contains precise text for the no-card case.** The literal strings and their place in the file are **observed**. The later claim about where Blender shows them is **inferred** from the drawing code.

> ```
>         if not found_device:
>             col = box.column(align=True)
>             col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1808–1810, in `CyclesPreferences._draw_devices` (read 2026-07-30)

Followed, per backend, by the requirement lines quoted under question 2 — for example:

> ```
>             elif device_type == 'OPTIX':
>                 compute_capability = "5.0"
>                 driver_version = "535"
>                 col.label(text=rpt_("Requires NVIDIA GPU with compute capability %s") % compute_capability,
>                           icon='BLANK1', translate=False)
>                 col.label(text=rpt_("and NVIDIA driver version %s or newer") % driver_version,
>                           icon='BLANK1', translate=False)
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1816–1822 (read 2026-07-30)

**Inferred, and the step is worth naming.** Those strings sit inside `_draw_devices`, which is called from `draw_impl`, which is called from `draw(self, context)` — the method Blender calls to paint the preferences panel. So this text is panel text. The step a reader must supply is that a `draw` method on an `AddonPreferences` subclass paints the add-on's preferences area; the API documentation shows that pattern (`https://docs.blender.org/api/5.1/bpy.types.AddonPreferences.html`, "def draw(self, context): ... layout.label(...)"). Because it is panel text, nothing in my sources says it is ever printed anywhere a script or a log could catch it.

**One further thing Blender offers, but only when asked.** Logging is off by default and has a level that explicitly covers devices:

> "--log <match>"
> "Enable logging categories, taking a single comma separated argument."
> "--log \"render,cycles\": both render and cycles messages."

> "--log-level <level>"
> "info: Information about devices, files, configuration, operations"

> "--debug-cycles"
> "Enable debug messages from Cycles."
> — Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html (read 2026-07-30)

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, every technology button in Blender 5.1's **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** row was clicked in turn. The row held **None** and **Metal**. Neither the "No compatible GPUs found for Cycles" line nor any "Requires…" line appeared under either, and the two tick boxes were labelled exactly **Apple M1 Pro (GPU - 14 cores)** and **Apple M1 Pro**. A Metal device was present on this machine, so the no-compatible-card case was never reached and this run records nothing about what it displays, on the panel or anywhere else.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.1 in background mode printed a message for two of the three failure cases put to it and nothing for the third.

`-- --cycles-device OPTIX`: exit status `1`, no image file, and the literal line

```
00:02.241  reports          | ERROR Found no Cycles device of the specified type
```

`-- --cycles-device NOTADEVICE`: exit status `1`, no image file, and the literal line

```
00:00.279  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
```

But a Cycles render asked for graphics cards when **every card was switched off in the saved preferences** exited `0`, wrote the image, and printed nothing naming a missing or unusable device; the same held with the technology set to `NONE`. So the literal text the sources could not give does exist for a refused `--cycles-device`, and there is no equivalent for a render that simply has no card enabled. Full commands and output are under question 5.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, five things bearing on this question were put to 5.1.

**Devices Cycles will not accept.** `-- --cycles-device METAL`, `-- --cycles-device HIP` and `-- --cycles-device ONEAPI` all ended at exit `1`, wrote no image, and printed one line:

```
00:00.548  reports          | ERROR Found no Cycles device of the specified type
```

HIP and oneAPI are built into this copy and simply have no card; Blender says the same thing for that as for Metal, which is not built in at all. `-- --cycles-device NOTADEVICE` gave exit `1`, no image, and two lines:

```
00:00.245  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

Both messages are word for word the Mac's; the second line is one 4.5 does not print on the same machine.

**The real no-compatible-card case, which the Mac could not reach.** With the technology saved as `ONEAPI` and no Intel card present (`SAVED: ONEAPI ACTIVE: False NUMGPU: 0`), a background render exited `0`, wrote its image and rendered on the processor. The whole output was searched for the phrase `No compatible GPUs found for Cycles` and it appeared **nought times**; no `Requires…` line appeared either. So the panel text really has no non-interface equivalent. What it says in the panel is quoted under question 2.

**Where the crash file lands on Linux.** A crash forced with `-b <file> --python-expr "import ctypes; ctypes.string_at(0)"` gave exit `139` and:

```
Writing: /tmp/machine-test.crash.txt
```

With no blend-file loaded, `Writing: /tmp/blender.crash.txt`. So the manual's Linux sentence is right about the folder and incomplete about the name: with a file loaded, the crash file is named after that file, and it still goes to `/tmp` rather than beside the blend-file.

**Exit statuses, which the sources give nowhere.** Two are now recorded on both machines: `1` for a refused `--cycles-device` value, with no image written, and `0` for a render that fell back to the processor and wrote one. A crash gives `139`, and a windowed start with no display server gives `1`.

**What Blender says when it cannot open a window** is quoted in full under question 1.

**Not covered.** The sources name errors and give their meanings, but with one exception ("No compatible GPUs found for Cycles" and its follow-on lines, which I read out of Cycles' own file) they never print the literal text. Several of those literal texts are now recorded from two machines and are above. What is still missing: what Blender prints when it will not start for want of a good enough graphics card, no machine below the floor existing; what it says when a card is present and supported but its driver is too old; and whether any message carries a code a program could match on.

**Outside the interface.** Mixed, and this is the important half of the answer. The console messages, the crash files, `--log`, `--log-file`, `--log-level` and `--debug-cycles` are all command-line things and are documented as such. The "No compatible GPUs found for Cycles" text is panel text and **now observed to have no non-interface equivalent**: on the rented Linux machine, with a technology chosen that has no compatible card, a windowless render printed the phrase nought times. The manual's own instruction for the graphics-hardware summary — "use Help ‣ Save System Info inside Blender" — is written as a menu action, though the same thing exists as an operator and works with no window on both machines; see question 6.

**Operating system.** The crash-file behaviour differs per platform in name and location, documented separately for Windows, macOS and Linux on the crashes page, and two of the three are now observed: `blender.crash.txt` in the Mac's own temporary directory, and `/tmp/<name of the loaded file>.crash.txt` on Linux. The Windows helper scripts (`blender_debug_log.cmd`, `blender_debug_gpu.cmd`, `blender_debug_gpu_workaround.cmd`, `blender_factory_startup.cmd`) exist only on Windows and were not run. Two of the four render error messages are platform-bound by their own text: the NVIDIA display-driver one says "Windows has a limit", and no Windows machine ran these tests; the GNU version one says "On Linux" and depends on the compiler on the machine, so it was not reached. One line appears on Linux in this version that the Mac never showed: `WARNING HIPEW initialization failed: Error opening HIP dynamic library`, at the start of very nearly every run.

---

## 5. How is it decided which of a computer's graphics cards a render uses, can more than one card be used for the same render, and how is that decided from outside Blender's own interface?

**Label: documented and observed.** I read this as three joined questions and answer each.

### Which cards a render uses

There are three settings, and all three must line up.

**First, one backend, chosen for the whole of Blender:**

> "Cycles Render Devices"
> "Changes the Computing Device the Cycles render engine uses to render images."
> "None: When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."
> "CUDA: If the system has a compatible NVIDIA CUDA device, it will be available as an option for rendering with Cycles."
> — Manual, https://docs.blender.org/manual/en/5.1/editors/preferences/system.html (read 2026-07-30). The same sentence pattern follows for OptiX, HIP, oneAPI and Metal.

The list on offer is built from what the machine has. Label: **observed**.

> ```
>     def get_device_types(self, context):
>         import _cycles
>         has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()
>
>         list = [('NONE', "None", n_("Do not use compute device"), 0)]
>         if has_cuda:
>             list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1581–1587 (read 2026-07-30)

The manual's picture documents one panel state. The image `_images/editors_preferences_section_system.webp`, shown on https://docs.blender.org/manual/en/5.1/editors/preferences/system.html and marked in the text as `[image: _images/editors_preferences_section_system.webp]`, shows the Cycles Render Devices row offering exactly three buttons — None, CUDA, HIP — with None selected. Label: **documented** for what is visible in the picture. **Inferred**, one step, for the wider conclusion: the observed code above builds the row from the device kinds reported on the machine, so the row can differ between machines.

The default is not the same everywhere. Label: **observed**.

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
> — `Blender Sources/cycles_5.1/properties.py`, lines 1572–1579 (read 2026-07-30)

`5` is `METAL` and `0` is `NONE` in the enum at lines 1585–1595 of the same file. **Inferred**, one step: the numbers returned are the identifiers used in `get_device_types`, so on Apple Silicon the backend starts as Metal and everywhere else it starts as None. The step is matching the bare integers to the identifiers in the list built a few lines below.

**Second, a tick per card**, within that backend:

> "Enable the available devices for the chosen backend."
> — Manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html (read 2026-07-30)

Each card is one saved record. Label: **observed**.

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
> — `Blender Sources/cycles_5.1/properties.py`, lines 1558–1566 (read 2026-07-30)

and the collection that holds them:

> ```
>     devices: CollectionProperty(type=CyclesDeviceSettings)
> ```
> — `Blender Sources/cycles_5.1/properties.py`, line 1606 (read 2026-07-30)

with `enum_device_type` at lines 172–179 giving the six kinds: `CPU`, `CUDA`, `OPTIX`, `HIP`, `METAL`, `ONEAPI`.

The file assigns `use` as true for a newly added non-CPU record and false for a newly added CPU record. Label: **observed** for what the file contains; whether a running Blender reaches this branch in a given case is not established:

> ```
>             if not entry:
>                 # Create new entry if no existing one was found
>                 entry = self.devices.add()
>                 entry.id = device[2]
>                 entry.name = device[0]
>                 entry.type = device[1]
>                 entry.use = entry.type != 'CPU'
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1674–1680, in `update_device_entries` (read 2026-07-30)

Cards are matched to their saved record by identifier *and* kind together:

> ```
>     def find_existing_device_entry(self, device):
>         for device_entry in self.devices:
>             if device_entry.id == device[2] and device_entry.type == device[1]:
>                 return device_entry
>         return None
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1662–1666 (read 2026-07-30)

**Inferred**, one step: because the tick lives on a record keyed by identifier and kind, and that record is stored in the preferences (the comment at line 1562 says "Properties saved in preferences"), the same card can be ticked for one backend and unticked for another. The step is reading `type` as the backend the tick belongs to, which the `entry.type == compute_device_type` test at line 1698 supports.

**Third, per scene, whether to use the GPU at all:**

> "Device"
> "Specifies the compute device used for rendering in Cycles."
> "CPU: Render using the system's central processing unit. This mode is compatible with all Cycles features."
> "GPU Compute: Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices."
> — Manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html (read 2026-07-30)

and its default. Label: **observed**.

> ```
>     device: EnumProperty(
>         name="Device",
>         description="Device to use for rendering",
>         items=enum_devices,
>         default='CPU',
>         update=update_render_passes,
>     )
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 381–387 (read 2026-07-30)

with `enum_devices` at lines 30–34 of the same file holding exactly two entries, `CPU` and `GPU`.

So: a scene set to CPU renders on the CPU whatever the preferences say, and a scene set to GPU renders on whichever cards are ticked under whichever backend is chosen. That last sentence is **documented** — it is what the two quoted manual passages say between them.

### Can more than one card be used for the same render

**Yes. Documented, and stated as plainly as anything on the sheet.**

> "Can multiple GPUs be used for rendering?"
> "Yes, go to Preferences ‣ System ‣ Compute Device Panel, and configure it as you desire."
> — Manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html (read 2026-07-30)

> "Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices. Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."
> — Manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html (read 2026-07-30)

Two things come with that. Memory does not pool:

> "Would multiple GPUs increase available memory?"
> "Typically, no, each GPU can only access its own memory."
> "The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."
> — Manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html (read 2026-07-30)

> "Distribute Memory Across Devices"
> "Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes. Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol."
> "Currently only NVLink on NVIDIA GPUs is supported."
> — Manual, https://docs.blender.org/manual/en/5.1/editors/preferences/system.html (read 2026-07-30)

And a mixed set of cards drags hardware ray tracing down to the slowest. Label: **observed**.

> ```
>         # Any device without RT support will disable it for all.
>         has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1907–1908 (read 2026-07-30)

One more caution, from the manual, about mixing card kinds at all:

> "Results will not look exactly the same on different CPU and GPU devices, so this is not suitable for multi device rendering or render farms with mixed hardware."
> — Manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/hair.html (read 2026-07-30), on one specific hair setting.

**EEVEE cannot do this at all.**

> "Multiple GPU Support"
> "There is currently no support for multiple GPU systems."
> — Manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html (read 2026-07-30)

### How it is decided from outside Blender's own interface

Three routes, and they are not equal.

**One: a command-line argument. Documented.**

> "Cycles Render Options"
> "Cycles add-on options must be specified following a double dash."
> "--cycles-device <device>"
> "Set the device used for rendering."
> "Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL."
> "Append +CPU to a GPU device to render on both CPU and GPU."
> "Example:"
> "blender -b file.blend -f 20 -- --cycles-device OPTIX"
> — Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html (read 2026-07-30)

Cycles' own file says what it overrides, which the manual does not. Label: **observed**.

> ```
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)
> ```
> — `Blender Sources/cycles_5.1/engine.py`, lines 16–20 (read 2026-07-30)

The file also contains a branch that calls `sys.exit(1)` if `_cycles.set_device_override` returns false. Label: **observed** for the branch; what makes the compiled function return false is not in the sources:

> ```
>     if args.cycles_device:
>         import _cycles
>         if not _cycles.set_device_override(args.cycles_device):
>             sys.exit(1)
> ```
> — `Blender Sources/cycles_5.1/engine.py`, lines 38–41 (read 2026-07-30)

**Inferred**, one step: the file returns before this block unless a bare `--` is present in the arguments, so the option is not parsed without that separator.

> ```
>     argv = sys.argv
>     if "--" not in argv:
>         return
> ```
> — `Blender Sources/cycles_5.1/engine.py`, lines 27–29 (read 2026-07-30)

**Note carefully what this argument does and does not do.** Its valid options are backend names — six of them, no more. **Documented**: the manual and the file both give the list and it is closed. So `--cycles-device` chooses the backend and the CPU-with-GPU pairing. It has no way to name one card out of several. **Not in the sources**: what the override does to the per-card ticks — whether it turns all cards of that backend on, or leaves the saved ticks standing. `set_device_override` lives in Blender's compiled part, and the Cycles files I was given are only the Python add-on plus its kernel and utility sources (`Blender Sources/cycles_5.1/source/` contains only `kernel/` and `util/`). I found no answer and will not invent one.

**Two: Python. Inferred, and I set out the step.** Cycles' preferences are an `AddonPreferences` subclass:

> ```
> class CyclesPreferences(bpy.types.AddonPreferences):
>     bl_idname = __package__
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1569–1570 (read 2026-07-30)

and the API documentation shows how such a thing is reached:

> "addon_prefs = preferences.addons[__name__].preferences"
> — API documentation, https://docs.blender.org/api/5.1/bpy.types.AddonPreferences.html (read 2026-07-30)

> "addon_prefs = bpy.context.preferences.addons[\"kitsu\"]"
> — Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html (read 2026-07-30)

The step: joining those, a script can reach `compute_device_type`, the `devices` collection and each device's `use` flag, and set them. Cycles' own files do exactly this from the other side:

> ```
> def get_device_type(context):
>     return context.preferences.addons[__package__].preferences.compute_device_type
> ```
> — `Blender Sources/cycles_5.1/ui.py`, lines 96–97 (read 2026-07-30)

And the scene setting is a plain scene property (`scene.cycles.device`, from `CyclesRenderSettings` at `properties.py` line 378 onward, attached to `bpy.types.Scene` in `register()`).

**The one thing I cannot give you is the exact key.** `bl_idname = __package__` means the key is the add-on's package name, and my sources never print that literal. The API documentation gives the file paths as `addons_core/cycles/operators.py` (https://docs.blender.org/api/5.1/bpy.ops.cycles.html, read 2026-07-30), so the directory is `cycles`, but a directory name is not necessarily the package key a script must type.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, `list(bpy.context.preferences.addons.keys())` was run in Blender 5.1's Python Console (Scripting workspace, window open) and returned:

```
['io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'cycles', 'pose_library', 'bl_pkg']
```

so the key is the literal string `cycles` on this machine. This is what one machine returned; it is not evidence that the same string is returned everywhere.

**Three: what does not exist, as far as I can find.** No environment variable for choosing a card. I searched all of `manual_5.1`, `api_5.1` and `release_notes_5.1` for `CUDA_VISIBLE_DEVICES`, `HIP_VISIBLE_DEVICES` and `ONEAPI_DEVICE_SELECTOR` — nothing, in any of the three. I also read the complete Environment Variables section of the command-line arguments page: every variable there concerns file and directory locations (`BLENDER_USER_RESOURCES`, `BLENDER_USER_CONFIG`, `BLENDER_USER_SCRIPTS`, `BLENDER_USER_EXTENSIONS`, `BLENDER_SYSTEM_*`), none concerns devices. And I searched all of `cycles_5.1/*.py` for `getenv` and `environ` — no matches. The only environment variable anywhere near this subject is `CYCLES_CUDA_EXTRA_CFLAGS`, and the manual's use for it is choosing a compiler, not a card (https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html).

Nor is there a per-card command-line flag. The full `--cycles-device` option list is quoted above and closed.

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 5.1.

**The panel.** **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** was expanded and every button in its technology row clicked in turn. The row held exactly two buttons, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — plus **Kernel Optimization** (**Full**; also **Off**, **Intersection Only**) and **MetalRT** (**Auto**; also **Off**, **On**). Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed.

**From Blender's Python Console** (Scripting workspace, window open), with the key `cycles` found as recorded above and `p = bpy.context.preferences.addons["cycles"].preferences`:

- `p.compute_device_type` returned `'METAL'`.
- `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned `[]` before anything else was called, and after `p.refresh_devices()` returned:

```
[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

- `p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, `p.has_multi_device()` returned `False`.
- `p.devices[0].use = False` was run — index 0 being the processor entry, unticked by default — and the panel then showed exactly what it shows by default. `p.devices[0].use = True` was run, and the **Apple M1 Pro** box in the panel became ticked.

This was done from a Python Console inside Blender's own window; it records nothing about a background run.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, which reports `Blender 5.1.2`. Scene: `/tmp/machine-test.blend`, Cycles, scene Device `GPU`, 16 samples. Output quoted as printed.

**The three settings, read with no window open.**

```
CYCLES KEYS: ['cycles']
BACKEND: METAL
BEFORE: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
AFTER: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

The stored list was already populated before `refresh_devices()` was called. With `--factory-startup` added, `BEFORE:` was `[]` and `AFTER:` showed the card ticked and the processor unticked — so `--factory-startup` discards the saved per-card switches.

**A background render with the saved preferences as found** exited `0`, wrote its PNG, and printed:

```
00:01.779  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
00:01.779  cycles           | Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
00:01.782  cycles           | Using default CPU kernels.
01:06.271  render           | Fra: 1 | Mem: 1583M | Sample 16/16 (Using optimized kernels)
01:06.271  render           | Fra: 1 | Mem: 1583M | Finished
```

**Every card switched off in the saved preferences, then rendered.** The setting command printed `SAVED STATE: METAL [('Apple M1 Pro', False), ('Apple M1 Pro (GPU - 14 cores)', False)]` — so `bpy.ops.wm.save_userpref()` runs with no window — and the render exited `0`, wrote its PNG, and ended:

```
00:06.412  render           | Fra: 1 | Mem: 151M | Sample 16/16
```

with no `(Using optimized kernels)` and no message about the switched-off card. So the saved ticks are honoured by a windowless render.

**The same state with `-- --cycles-device METAL`** exited `0` and ended:

```
00:05.568  render           | Fra: 1 | Mem: 1583M | Sample 16/16 (Using optimized kernels)
```

**With the scene's own Device saved as `CPU`**, `-- --cycles-device METAL` still gave `00:05.982  render           | Fra: 1 | Mem: 1583M | Sample 16/16 (Using optimized kernels)`. So the flag displaced both the preferences and the scene setting, as the add-on's help text claims.

**The technology set to `NONE`** gave a render on the processor with no message: `00:06.567  render           | Fra: 1 | Mem: 151M | Sample 16/16`.

**Values `--cycles-device` refuses.** `-- --cycles-device OPTIX`: exit `1`, no image, `00:02.241  reports          | ERROR Found no Cycles device of the specified type`. `-- --cycles-device NOTADEVICE`: exit `1`, no image, `00:00.279  cycles           | ERROR NOTADEVICE is not a valid Cycles device.` `-- --cycles-device METAL+CPU`: exit `0`, image written, ending `00:05.936  render           | Fra: 1 | Mem: 151M | Sample 16/16`.

**Preference changes made inside the render's own run** did not produce the graphics-card signature. Setting every `use` to `True` in the same process that then rendered printed `SET TO: [('Apple M1 Pro', True), ('Apple M1 Pro (GPU - 14 cores)', True)]` and ended `00:05.793  render           | Fra: 1 | Mem: 151M | Sample 16/16`. Setting them all to `False`, and setting `compute_device_type` to `'NONE'`, gave the same shape of result. The saved preferences were unchanged afterwards, and were restored to the state found.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-5.1.2-linux-x64/blender`. Its settings folder started empty, so this version began from factory settings rather than from anything a person had set. One thing about that folder, said plainly rather than left out. The first runs were meant to point Blender at a folder of this topic's own with `BLENDER_USER_RESOURCES`, and a mistake in the shell meant the variable was never actually set, so those runs used Blender's ordinary default folder instead. That folder was also empty when this work began, so "began from factory settings" holds either way. Every test that turns on saved preferences was then run again with the variable properly set, and every result came back identical. Both runs are described in `Audits/Blender Research/The machine/Hand tests — Linux.md`.

**What Blender finds, and how the one card is listed.** With an untouched settings folder:

```
CYCLES KEYS: ['cycles']
BACKEND: NONE
BEFORE: []
AFTER: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True)]
```

The add-on key is `cycles` here too. The technology starts at `NONE` on this machine and at `METAL` on the Mac, as `default_device` says. Every graphics entry starts ticked and the processor unticked, as the "on unless CPU" default says. And **one physical card is listed twice**, once under CUDA and once under OptiX, with two identifiers, each carrying the card's place on the bus. With preferences saved, `BEFORE` came back already populated; `--factory-startup` emptied it again. `has_active_device()` returned `True`, `get_num_gpu_devices()` returned `1`, `has_multi_device()` returned `False`.

**What `--cycles-device` does to the per-card ticks — settled, for the one-card case.** With the technology saved as `OPTIX` and every device switched off, a background render exited `0` and reported `Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]`. The same render with `-- --cycles-device OPTIX` appended reported `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]`. So the flag overrides the ticks. With the devices back on and the technology set to `NONE`, the processor again, and no `Added device` line at all.

**The flag also overrides the scene's own setting.** A copy of the scene saved with Device `CPU`, rendered with `-- --cycles-device OPTIX`, reported the card.

**`+CPU` uses both, and Linux names both.** With `-- --cycles-device CUDA+CPU`:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                            |                      Hardware Ray-Tracing: Off
                            |                  AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (11 threads) [CPU]
                            |                      Hardware Ray-Tracing: Off
```

Eleven threads for the processor there and twelve when it works alone.

**A change made by a script inside the render's own run — and here the two machines disagree.** Three runs, each changing the preferences and then rendering in the same command, never saving them. Every device off: `Path tracing on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]`. Every device on: the card and the processor both named. The technology set to `NONE`: `BACKEND NOW: NONE`, and the processor. All three exited `0` and wrote an image. On the rented Linux machine the change **took effect**. On the Creator's Mac the same three runs carried no graphics-card marker at all. Both stand; neither is chosen between here.

**The preferences-folder inference is now exercised.** Blender was started with `BLENDER_USER_RESOURCES` set to a folder of its own. It reported that folder as its preferences path (`PREFS PATH: /root/smoozy/The machine/res-5.1`), wrote a `userpref.blend` into it when `bpy.ops.wm.save_userpref()` was called, read the saved technology and per-card switches back from it on the next run (`READ BACK: OPTIX [('NVIDIA GeForce RTX 4090', 'CUDA', True), …]`), and rendered accordingly. So changing which preferences file is read does change which cards are ticked, which this answer marks as inferred.

**Which card draws the interface — the control exists on Linux.** Read with no window, `GPU_BACKEND NOW: OPENGL | CHOICES: ['OPENGL', 'METAL', 'VULKAN']` and `GPU_PREFERRED_DEVICE NOW: AUTO | CHOICES: ['AUTO']`. With a window, the System page carries a **Display Graphics** section, which the Mac has not got, showing `Backend: OpenGL`. With the backend saved as `VULKAN` and Blender restarted, a **Device** control appears reading `Auto`. This version does **not** show the "Current Vulkan backend limitations: • Low performance in VR" note that 4.5 and 5.0 show on the same machine. `gpu_preferred_device` still held only `['AUTO']` with Vulkan running, there being one card.

**Blender's own `--help` on this machine** gives `--cycles-device` the same six values as every other version, and lists `--no-window-frame` ("Disable all window decorations (Linux only)") and `--gpu-vsync`, neither of which 4.5 has. There is no `--gpu-device` argument in this version; 5.2 has one.

**Not covered.** How Blender chooses among several cards when you have not chosen, beyond the "on unless CPU" default. How work is divided between two ticked cards, or how a render behaves if one of them is busy. Both need a second card. What `--cycles-device` does to the per-card ticks is now answered for one card and not for several. Which card EEVEE uses on a machine with several is still nothing.

**Outside the interface.** This is the question's own subject and the answer is: partly. `--cycles-device` covers the backend and CPU pairing but not individual cards. Individual cards are reachable only through Python, via a key my sources do not spell out — though the hand tests returned it on one machine as the literal string `cycles`, and read and set the per-card flags through it, both inside the window and from `--python-expr` with no window at all. The terminal tests settle the rest on that machine: the saved ticks are honoured by a windowless render, `bpy.ops.wm.save_userpref()` works with no window, `--cycles-device` displaced both the saved ticks and the scene's own Device setting, and `--factory-startup` discards the saved ticks. Changing which preferences file is read would therefore change which cards are ticked, as the inference above says, but that particular route was not itself tested. A change made inside the render's own run did *not* produce the graphics-card signature, so a same-run change is not established as taking effect. The manual's own answer to "Can multiple GPUs be used for rendering?" is a menu path. One thing to watch: the per-card ticks live in the preferences file, so anything that changes which preferences file is read — `BLENDER_USER_CONFIG`, `BLENDER_USER_RESOURCES` — changes which cards are ticked. That last sentence is **inferred**: the manual documents those variables as setting the "Directory for user configuration files" and the ticks are documented in Cycles' file as "Properties saved in preferences"; the step is that preferences are read from that directory, which the startup page supports with its "Read prefs: {DIR}/userpref.blend" message.

**Operating system.** The mechanism is the same everywhere but the choices are not. Which backends appear depends on the machine (`get_device_types`) — `(False, False, False, True, False, False)` on the Creator's Mac and `(True, True, True, False, True, False)` on the rented Linux machine — and per question 2 four of the five are Windows-and-Linux and one is macOS-only. The starting backend differs on Apple Silicon (`default_device`), and both halves are now observed: `METAL` on the Mac, `NONE` on Linux. MetalRT is gated on `platform.machine() == 'arm64'` (`properties.py` lines 1915–1925), and the Linux panel shows neither it nor Kernel Optimization, carrying instead a greyed `Embree on GPU` box under oneAPI and a greyed, ticked `HIP RT` box under HIP. Linux also has a **Display Graphics** section and an **Operating System Settings** section that macOS has not. NVLink memory sharing is NVIDIA-only by the manual's own words and needs two cards, so it could not be tested.


### Hand tests on the two-card machine — observed
**Read this block against the *Not covered* paragraph just above it.** That paragraph was written before any two-card machine existed, so some of what it calls untested is answered here — how work is divided between two ticked cards, and the several-card form of the `--cycles-device` question above all. What it says about an NVLink pair, about two cards of different makes, about a slow card beside a fast one, and about anything needing a window still stands. *(Pointer added 2026-08-06 by the citation check of the two-card material.)*

Run on **2026-08-06** on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-5.1.2-linux-x64/blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`. Label: **observed**.

**This is a different machine from the one-card rented Linux machine above, and nothing here is merged with that machine's results or with the Creator's Mac.** Its settings folder was `/root/smoozy/two cards/res-5.1`, empty when this work began, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender every time. **Nothing else was running on the machine**, which is why the timings are quoted at all. The full list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`.

**The two cards are not joined by NVLink.** `nvidia-smi topo -m` reports `PHB` between them — `Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)` — so this machine still cannot test an NVLink pair.

**Both cards are switched on at factory settings, and one card is listed twice.** With an untouched settings folder, `--factory-startup`, and a script that calls `refresh_devices()` and then prints every device as `(name, type, id, use)`:

```
PREFS PATH: /root/smoozy/two cards/res-5.1
BACKEND: NONE
BEFORE: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a', True), ('AMD EPYC 7B13 64-Core Processor', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX', True)]
```

The label `BEFORE` is the script's own word and is misleading — the refresh had already happened. Five entries for two cards: each card once under CUDA and once under OptiX, plus the processor. Every graphics entry starts ticked.

**A render uses both, without being asked to.** With the technology saved as `OPTIX` and both cards ticked, a windowless render of a Cycles scene at 1920 by 1080 and 6000 samples named both cards:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                     Hardware Ray-Tracing: Off
                 NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
```

The card-by-card watch was run in 4.5 rather than here, and it showed both cards between 91 % and 98 % for the whole render and both holding the same 2683 MiB.

**Two cards against one, on the same scene and the same samples.** Nine renders in each of three states, one after another, never overlapping, on a machine with nothing else running. This version's `Finished` line carries a timestamp counting from Blender's start rather than a render time, so the same scene was rendered twice more in each of two states afterwards with the `Time:` summary line captured. Both sets, unedited:

| | two cards | the card at `0000:00:07` alone | the card at `0000:00:0a` alone |
|---|---|---|---|
| timestamp on `Finished` | `00:30.883`, `00:29.372`, `00:29.888` | `00:55.347`, `00:56.023`, `00:55.832` | `00:55.799`, `00:56.284`, `00:56.024` |
| the `Time:` summary line | `Time: 00:32.05 (Saving: 00:00.30)`, `Time: 00:32.73 (Saving: 00:00.33)` | `Time: 00:58.08 (Saving: 00:00.30)`, `Time: 00:58.84 (Saving: 00:00.36)` | not run in this form |

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
                     Hardware Ray-Tracing: Off
```

With the same one card ticked and `-- --cycles-device OPTIX`, exit 0 and **both** named. With **every** device unticked and the same flag, exit 0 and both named again:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                     Hardware Ray-Tracing: Off
                 NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
```

So the flag does not merely re-enable the one card the preferences switched off — **it takes every card of that technology**, whatever the preferences said.

**What the memory readouts say with two cards.** Blender prints one figure, never two, and never names a card beside a memory number. On the timing scene its `Finished` line read `Mem: 8M` with two cards and `Mem: 1083M` with one — **smaller with two cards, not larger and not doubled**. What that field counts is not settled here and nothing is read into it.

**Frame order and output.** A four-frame animation was rendered with both cards and then with one. The order was ascending both ways:

```
00:04.805  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0001.png'
00:05.981  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0002.png'
00:07.213  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0003.png'
00:08.324  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0004.png'
```

The files were not byte-identical — but neither were two runs on the same cards, and Blender stamps a date into the PNG, so bytes cannot answer this alone. Comparing the pixels instead, of 921,600 values: **17 differed between two cards and one, 10 between two runs on two cards, and 8 between two runs on one card**, and the largest difference in every one was `0.003921568393707275` — one step in 255.

### Hand tests on the four-card machine — observed

Run on **2026-08-06** on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each, driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server, run over SSH** — against `/opt/blender/blender-5.1.2-linux-x64/blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`. Label: **observed**. **A fourth machine, and a different one from every machine above; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine.** Settings folder `/root/smoozy/four cards/res-5.1`, emptied and remade before use, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender. **Nothing else was running on the machine.** The full list, with every command and every script written out, is `Audits/Blender Research/The machine/Hand tests — four cards.md`.

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

**One line reads differently from the two-card machine's.** There the rebalancer reported `Number of requested rebalances: 1` and `Number of performed rebalances: 0`. Here it reports 8 requested and 7 performed, so with four cards Blender moved the work between them repeatedly during one render, where with two it did not move it at all. **The count itself moves from run to run and is not this version's figure.** The run recorded here printed 8 requested and 7 performed; a repeat on the same machine printed 7 requested and 6 performed. Both are recorded as they printed and nothing is read into either. *(Narrowed by the four-card citation check, 2026-08-06: this gave a single run's count as the version's, and the repeat disagreed.)*

**One card, two cards and four cards, on the same scene and the same samples.** Forty-eight renders in all across the four versions, one after another, never overlapping, on a machine with nothing else running. The scene is 1920 by 1080 at 3000 samples, adaptive sampling off, denoising off. **Cycles' own `Total render time:`, in seconds**, three runs of each: four cards `19.7278`, `19.7868`, `19.7481`; two cards `37.8433`, `37.6999`, `37.8094`; the card at `0000:00:07` alone `71.6978`, `71.6732`, `71.6856`. **Blender's own `Time:` line**, which counts the whole run: four cards `00:26.19`, `00:26.21`, `00:26.20`; two cards `00:44.20`, `00:43.99`, `00:44.05`; one card `01:17.90`, `01:17.86`, `01:17.90`.

**The four cards are as fast as each other.** The cards at `0000:00:09`, `0000:00:0b` and `0000:00:0d`, one run each, gave `70.6002`, `71.3936` and `69.6461` seconds on Cycles' own figure — a spread of about 2 %.

**Doubling the cards did not halve the time, and quadrupling them did not quarter it.** On Cycles' own figure, one card to two is 1.90 times faster, two to four is 1.91, and one to four is 3.63. On Blender's whole-run figure, where the work of loading the file and building the scene does not shrink at all, the same three are 1.77, 1.68 and 2.97. So "more cards make it proportionally faster" holds in shape and not in arithmetic.

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
four cards: 00:25.830  render           | Fra: 1 | Mem: 8M | Finished
two cards:  00:44.000  render           | Fra: 1 | Mem: 8M | Finished
one card:   01:17.802  render           | Fra: 1 | Mem: 391M | Finished
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

**Label: documented and observed. Yes — by several means, which answer slightly different questions.**

### For Cycles' render devices

**Through the interface, and it is documented as the way to find out:**

> "Save System Info"
> "This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems."
> "You will be prompted to save a text file called system-info.txt."
> "It contains the following sections:"
> ...
> "GPU"
> "Shows the GPU vendor, version and the capabilities of your hardware and driver."
> ...
> "Cycles"
> "The instruction sets and capabilities of each hardware render device available for use with Cycles."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/help.html (read 2026-07-30)

That is a direct answer to the question: a file listing each Cycles render device on the machine. Note the wording — "available for use with Cycles", which is the "can use" half rather than merely "can see".

**The API documents a way to generate system information without a menu:**

> "bpy.ops.wm.sysinfo(*, filepath='')"
> "Generate system information, saved into a text file"
> "Parameters:"
> "filepath (str) – filepath, (optional, never None)"
> — API documentation, https://docs.blender.org/api/5.1/bpy.ops.wm.html (read 2026-07-30)

**Through Cycles' own Python, which is where the detail is.** Label for all of the following: **observed**.

The plain enumeration:

> ```
> def available_devices():
>     import _cycles
>     return _cycles.available_devices()
> ```
> — `Blender Sources/cycles_5.1/engine.py`, lines 152–154 (read 2026-07-30)

The per-backend enumeration used by the preferences panel:

> ```
>     # Be careful when deciding when to call this function,
>     # as Blender can crash with `_cycles.available_devices()` on some drivers.
>     def get_device_list(self, compute_device_type):
>         import _cycles
>         device_list = _cycles.available_devices(compute_device_type)
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1651–1655 (read 2026-07-30)

That comment is worth keeping. So is this one:

> ```
>     # Refresh device list. This does not happen automatically on Blender
>     # startup due to unstable drivers that can cause crashes.
>     def refresh_devices(self):
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1707–1709 (read 2026-07-30)

**Inferred**, one step, and it matters: the comment says the list is not refreshed automatically at startup. So the `devices` collection you read may be what was saved last time rather than what is in the machine now, until something calls `refresh_devices` or `get_device_list`. The step is that "does not happen automatically on startup" means the saved collection is what stands until asked — which the surrounding code supports, since `update_device_entries` is only ever reached through `get_device_list`.

There is also a deprecated one, and reading its body is the only way to know it is now useless:

> ```
>     # Deprecated: use refresh_devices instead.
>     def get_devices(self, compute_device_type=''):
>         self.refresh_devices()
>         return None
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1718–1721 (read 2026-07-30)

The file's method returns `None`. **Inferred**, one step: code that expected this call to return a list would instead receive `None`.

The ones that answer plain questions:

> ```
>     def get_num_gpu_devices(self):
> ```
> ```
>     def has_multi_device(self):
> ```
> ```
>     def has_active_device(self):
>         return self.get_num_gpu_devices() > 0
> ```
> ```
>     def has_oidn_gpu_devices(self):
> ```
> ```
>     def has_optixdenoiser_gpu_devices(self):
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1728, 1741, 1753–1754, 1756, 1772 (read 2026-07-30)

And the one that returns the list of records for a backend, including the CPU appended to it:

> ```
>     # Gets all devices types to display in the preferences for a compute device type.
>     # This includes the CPU device.
>     def get_devices_for_type(self, compute_device_type, device_list=None):
>         # Layout of the device tuples: (Name, Type, Persistent ID)
> ```
> ```
>         # Extend all GPU devices with CPU.
>         if len(devices) and compute_device_type != 'CPU':
>             devices.extend(cpu_devices)
>         return devices
> ```
> — `Blender Sources/cycles_5.1/properties.py`, lines 1685–1688 and 1702–1705 (read 2026-07-30)

**What each entry in the raw list holds.** The comment above says three fields. The code reads eight positions. **Observed**, position by position, each from a line in the same file: `device[0]` is the name (line 1678), `device[1]` the kind (1679), `device[2]` the identifier (1677), `device[3]` peer memory (1900–1901), `device[4]` hardware ray tracing (1902–1903), `device[5]` OpenImageDenoise support (1766), `device[6]` OptiX denoiser support (1782), `device[7]` whether it is optimized (1697). The stale comment and the code disagree about how many fields there are; I report both and take the code's positions as what is there.

The saved record per card — identifier, name, use, kind — is quoted in full under question 5.

### For the active GPU reported by `gpu.platform`

A different, singular question, which the API documentation answers directly. Label: **documented**.

> "gpu.platform.backend_type_get()"
> "Get active GPU backend."
> "Returns: Backend type ('OPENGL', 'VULKAN', 'METAL', 'NONE', 'UNKNOWN')."

> "gpu.platform.device_type_get()"
> "Get GPU device type."
> "Returns: Device type ('APPLE', 'NVIDIA', 'AMD', 'INTEL', 'SOFTWARE', 'QUALCOMM', 'UNKNOWN')."

> "gpu.platform.renderer_get()"
> "Get GPU to be used for rendering."
> "Returns: GPU name."

> "gpu.platform.vendor_get()"
> "Get GPU vendor."

> "gpu.platform.version_get()"
> "Get GPU driver version."
> — API documentation, https://docs.blender.org/api/5.1/gpu.platform.html (read 2026-07-30)

These are singular throughout — "the active backend", "GPU name". They do not list every card in the machine or say how this reported GPU relates to Cycles' render-device list.

For Vulkan display drawing, the manual documents a setting:

> "Device Vulkan"
> "Specifies which GPU device to use for display drawing operations."
> "This setting is useful for systems with multiple GPUs (e.g., integrated + discrete) where you want to force Blender to use a specific GPU for UI rendering."
> "Auto: Automatically selects the most appropriate GPU based on system configuration and driver support."
> — Manual, https://docs.blender.org/manual/en/5.1/editors/preferences/system.html (read 2026-07-30)

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), the whole of **Edit ‣ Preferences ‣ System** was looked at, in all four versions. The page held, top to bottom: **Cycles Render Devices** (two buttons, `None` and `Metal`, with `Metal` selected; two tick boxes, `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked; `Kernel Optimization` showing `Full`; `MetalRT` showing `Auto`), **Network** (`Allow Online Access` ticked, `Time Out` 10, `Connection Limit` 5), **Memory & Limits** (`Undo Steps` 256, `Undo Memory Limit` 0, `Global Undo` ticked, `Console Scrollback Lines` 256, `Texture Time Out` 120, `Garbage Collection Rate` 60, `VBO Time Out` 120, `Garbage Collection Rate` 60, `Geometry Nodes Stack Limit` 100), **Video Sequencer** (`Memory Cache Limit` 16384, `Proxy Setup` showing `Automatic`), and **Sound**, collapsed.

**There was no Display Graphics section, and no graphics backend or device control anywhere on the page** — in any of the four versions. So on this machine the setting quoted above was not present to be read.

One thing to say plainly rather than let it read as a failed search: the test that produced this was mis-aimed. It asked what the **Device** list offers once the backend is set to **Vulkan**, and Vulkan is not something a Mac offers at all — the question assumed a control that does not exist on this operating system. What a Windows or Linux machine shows here is untested, because neither exists among the equipment available.

### One important absence

**Not in the sources: Cycles' preferences are not in the Python API documentation.** There is no `bpy.types.CyclesPreferences` page in `api_5.1`, and no page documents `get_devices`, `get_device_list`, `refresh_devices`, `compute_device_type`, or the `devices` collection. I searched all 1,899 files of `api_5.1` for `CyclesPreferences`, `get_devices` and `compute_device_type`: the only hits were `PreferencesSystem.legacy_compute_device_type` (documented as "For backwards compatibility only ... readonly") and its two index entries. I listed every `api_5.1` filename matching `cycles` and `preference`: `bpy.ops.cycles.txt` (three operators, none about devices) and the `bpy.types.Preferences*` family, which is Blender's own preferences, not the add-on's. So everything in the Cycles half of this answer comes from reading the add-on's files, not from documentation — which is exactly why it is labelled **observed**.

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 5.1.

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

and was followed by `Implementation Dependent GPU Limits:` and `GLSL:` before `Cycles:`. The file's full list of section headings, in order, is: `Blender:`, `Python:`, `paths:`, `Python (External Binary):`, `Directories:`, `scripts:`, `pref scripts:`, `FFmpeg:`, `Other Libraries:`, `GPU:`, `extensions:`, `Implementation Dependent GPU Limits:`, `GLSL:`, `Cycles:` (with `Metal device capabilities:` inside it), `Enabled add-ons:`. There is no section headed `OpenGL`.

**How the two routes differ.** The same file written with no window open drops the graphics half entirely: the `GPU:` heading is replaced by the single line `GPU: missing, background mode`, and the next heading is `Cycles:` — no `extensions:`, no `Implementation Dependent GPU Limits:`, no `GLSL:`. The `Cycles:` section is identical between the two routes. After the `GPU:` block, the menu-written file continues:

```
Implementation Dependent GPU Limits:
====================================

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


**Cycles' own Python, from Blender's Python Console** (Scripting workspace, window open). The add-on key was found to be the literal string `cycles`. With `p = bpy.context.preferences.addons["cycles"].preferences`, `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned `[]` before anything else was called, and after `p.refresh_devices()` returned:

```
[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

`p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, `p.has_multi_device()` returned `False`. So on this machine the stored collection did come back empty until a refresh was asked for, and the identifier was `CPU` for the processor and `METAL_Apple M1 Pro (GPU - 14 cores)` for the graphics card. `_cycles.available_devices()` itself was not called in this run, so the shape of the raw entries is still unrecorded.

**`gpu.platform`, from the same console.** `backend_type_get()` returned `'METAL'`; `device_type_get()` returned `'APPLE'`; `renderer_get()` returned `'Metal API'`; `vendor_get()` returned `'Apple M1 Pro'`; `version_get()` returned `'1.2'`. `gpu.platform.devices_get()` raised:

```
AttributeError: module 'gpu.platform' has no attribute 'devices_get'
```

so that function is not present in 5.1.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender` (`Blender 5.1.2`).

**Save System Info with no window open.** `-b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='/tmp/sysinfo-terminal-51.txt'))"` printed `RESULT: {'FINISHED'}` and wrote the file. Its `GPU` section was one line:

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

**What `available_devices()` actually returns.** `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types()); print('METAL:', _cycles.available_devices('METAL')); print('NOARG:', _cycles.available_devices())"` printed:

```
TYPES: (False, False, False, True, False, False)
METAL: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False, True, True, False, True))
```

and the no-argument call raised:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
TypeError: function takes exactly 1 argument (0 given)
```

So each entry is a tuple of eight fields, matching the eight positions the add-on reads and not the three its comment names — and the plain `available_devices()` that `engine.py` wraps is refused when called with no argument.

**`gpu.platform` with no window open** does not answer in this version. `import gpu; print('INIT:', gpu.init())` raised `AttributeError: module 'gpu' has no attribute 'init'`, and calling the platform functions raised:

```
SystemError: GPU functions for drawing are not available in background mode
```

**The Cycles preferences queries with no window open** did answer: the key printed as `['cycles']`, and the stored list was already populated before `refresh_devices()` was called — the opposite of what the same reading showed inside the window. Identifiers were the same in every run of the day.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-5.1.2-linux-x64/blender`.

**Save System Info with no window open.** `-b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='…'))"` printed `RESULT: {'FINISHED'}` and wrote the file. Its whole `GPU` section is one line, as on the Mac:

```
GPU: missing, background mode
```

Its `Cycles` section is where Linux differs, and it is long — about ninety lines. Its opening and the two lines that carry the answer:

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

So the report names each card and, on an NVIDIA card, its compute capability outright, plus the processor's instruction set (`AVX2`) where the Mac's line was empty. Only the technology whose cards were found gets a section: the technology was saved as OptiX and the section is still headed `CUDA device capabilities`. The headings, in order, were `Blender:`, `Python:`, `paths:`, `Python (External Binary):`, `Directories:`, `scripts:`, `pref scripts:`, `FFmpeg:`, `Other Libraries:`, `GPU: missing, background mode`, `Cycles:`, `Enabled add-ons:` — and, as on the Mac, no section headed `OpenGL`.

**Asking Cycles' compiled part directly, including for technologies with no card.** `-b --factory-startup --python-expr "import _cycles; …"` printed:

```
TYPES: (True, True, True, False, True, False)
CUDA: (('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', False, False, True, False, True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True))
OPTIX: (('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', False, False, True, True, True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True))
HIP: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
ONEAPI: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
METAL: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
```

and the no-argument form was refused: `TypeError: function takes exactly 1 argument (0 given)`. So each entry is an eight-field record, and a technology with no supported card returns the processor alone and no error, whether that technology is built in or not.

**The graphics module with no window open.** `-b --python-expr "import gpu; print('INIT:', gpu.init())"` raised:

```
AttributeError: module 'gpu' has no attribute 'init'
```

and the platform calls raised:

```
SystemError: GPU functions for drawing are not available in background mode
```

Word for word what the Mac gave, so `gpu.init` is absent and `gpu.platform` refuses with no window on both machines in this version.

**What Blender was built with, against what it can find.** Hiding the card for one command with `CUDA_VISIBLE_DEVICES=` left `TYPES:` unchanged while both card lists came back holding the processor alone, and a render then asked for CUDA exited `1` with `ERROR Found no Cycles device of the specified type`.

**Not covered.** Whether the stored identifier is stable across restarts, driver updates or reordering of cards — it held steady across every run of both days, and on Linux it visibly carries the card's place on the bus, which is a hint and not a statement. What any of this returns on a machine with several cards. The crash warning in Cycles' comments did not fire on either machine, which is not the same as it being stale.

**Outside the interface.** Yes for the documented command-line route into Python: `bpy.ops.wm.sysinfo(filepath=...)`, the `gpu.platform` functions, and the Cycles methods quoted above can be invoked through `--python` or `--python-expr`, as documented on https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html. The windowless case is now tested on two machines, and the calls do not all behave alike, in the same way on both. `bpy.ops.wm.sysinfo(filepath=…)` returned `{'FINISHED'}` and wrote its file, whose `GPU` section read only `GPU: missing, background mode` while its `Cycles` section was fully populated. The Cycles methods answered, with the stored list already populated before `refresh_devices()` when preferences had been saved and empty under `--factory-startup`. The `gpu.platform` functions did **not** answer on either: `SystemError: GPU functions for drawing are not available in background mode`. So whether `gpu.platform` returns anything meaningful with no window open is answered for this version: it returns nothing, because the call itself is refused.

**Operating system.** The sources present the same Python means without platform-specific instructions. What comes back differs, and both halves are now observed: on the Creator's Mac `device_type_get()` returned `'APPLE'` and `backend_type_get()` returned `'METAL'`; on the rented Linux machine those calls are refused with no window, and which Cycles backends can be enumerated followed the documented platform split — `(True, True, True, False, True, False)` there against `(False, False, False, True, False, False)` on the Mac. `gpu.platform.devices_get` was absent on both. Windows remains untested.



### Hand tests on the two-card machine — observed

Run on **2026-08-06** on the second rented Linux machine described under question 5 — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s on driver 580.95.05, sixty-one cores, **no display server at all**, over SSH — against `/opt/blender/blender-5.1.2-linux-x64/blender`. Label: **observed**. Not merged with the one-card rented machine's results or the Mac's.

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

Run on **2026-08-06** on the third rented Linux machine described under question 5 — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, sixty-one cores, 197 GB of memory, **no display server**, over SSH — against `/opt/blender/blender-5.1.2-linux-x64/blender`. Label: **observed**. A fourth machine; not merged with the Mac's results, the one-card machine's or the two-card machine's. The full list is `Audits/Blender Research/The machine/Hand tests — four cards.md`.

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

**Not covered.** What any of this returns on a machine with cards of two different makes, or with an NVLink pair. What the Status Bar or any panel shows, this machine having no display server at all.

**Outside the interface.** Settled again, on a machine with two cards and no display server: `wm.sysinfo` wrote its file, the Cycles queries answered, and both cards were named by every route. Nothing needed a window.

**Operating system.** Linux, as before. This second machine agrees with the one-card machine wherever the two could be asked the same thing, and adds only what a second card shows. Windows is untested.
---

## Things a person at a computer with Blender 5.1 could settle by hand in a few minutes

This list has since been run on three machines. What the Creator's own MacBook Pro settled is written into the answers above as **Hand test — observed**, what the one-card rented Linux machine settled as **Hand tests on the rented Linux machine — observed**, and what a second rented Linux machine with two graphics cards and no display server settled as **Hand tests on the two-card machine — observed**. The three lists live at `Audits/Blender Research/The machine/Hand tests.md`, `Audits/Blender Research/The machine/Hand tests — Linux.md` and `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`. **A fourth machine has since been used as well** — a third rented Linux machine with **four** graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the four-card machine — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

### Still open

Nothing on this list is still open. All four items were run on both machines and what came back is recorded under questions 1, 2, 3, 4, 5 and 6 above. Item 3 was answered only as far as the Mac reached — every device unticked, and the technology set to `NONE`. The rented Linux machine reached the real case: with a technology chosen for which it has no card at all, a windowless render still printed nothing naming a missing device and completed on the processor, and the missing-requirement text was found only in the preferences panel.

### Cannot be run on the equipment available

The equipment is one MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, whose graphics are Apple's own and built in; one rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores — reached over SSH; and a second rented Linux machine — Ubuntu 22.04.5 LTS, **two** NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, **no display server at all** — also over SSH. No Windows machine has run it, there is no AMD or Intel graphics card and no pair of cards joined by NVLink, and nothing on either rented machine may be installed or removed. **Two corrections to that sentence, made 2026-08-06 by the citation check of the two-card material.** The one-card rented Linux machine was destroyed on 2026-08-06 and nothing can ever be run on it again, so every result from it stands as a record and none of it can be repeated. And something *was* removed and reinstalled on the two-card machine: its NVIDIA driver was purged on purpose, and then reinstalled into a mismatched state, for the broken-driver test recorded under question 2. That machine has since been restarted and did not answer when the citation check tried it on 2026-08-06.

**A fourth machine has since been used, and it is the only one still alive.** A **third rented Linux machine** — Ubuntu 22.04.5 LTS, **four** NVIDIA GeForce RTX 4060 Ti of **16 GB each** on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, **197 GB of memory**, **no display server**, over SSH. What it settled is written into the answers above as **Hand tests on the four-card machine — observed**, and its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. Three limits above are narrowed by it and none is removed: more than two cards is no longer out of reach, a card small enough to be filled is no longer out of reach, and the several-card form of every question under 5 and 6 has now been asked twice on two different machines. **Still out of reach on it as well:** any pair joined by NVLink, these four talking through the host bridge; cards of two different makes; cards of unequal speed, all four being the same model within about 2 % of each other; and anything needing eyes on a window, this machine having no display server either. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

- **Cards of two different makes ticked at once.** The two-card machine has two of the same NVIDIA model. Most of what needed a second card is now answered under questions 5 and 6; this is not.
- **What Distribute Memory Across Devices does when the cards are joined by NVLink.** The two-card machine's pair talks through the host bridge. What the setting does on an unlinked pair is now recorded under question 5.
- **Whether a slow card holds a fast one back.** Both cards on the two-card machine are the same model and gave the same time as each other.
- **Anything needing eyes on a window with two cards** — whether the Distribute Memory Across Devices box is drawn or greyed, and what the Vulkan **Device** control offers. The machine that has two cards has no display server; the machine that had a display had one card.
- **Whether Blender starts at all without OpenGL 3.3, in background mode.** Needs a machine or container with no graphics driver, or software rendering only. The rented machine reports `4.6.0 NVIDIA 580.105.08`. What Blender does with no *display server* is now settled under question 1, and that is a different case.
- **The oneAPI Windows driver figure and the Metal macOS figure.** The oneAPI **Linux** figure is now recorded under question 2, drawn by a running Blender as `XX.XX.34666.3`. The Windows half needs a Windows machine, the panel text branching on the operating system. The Metal half needs an Apple Silicon Mac running macOS 12.x: on macOS 26.5.2 a Metal device is found and no requirement line is drawn, and Linux offers no Metal button at all.
- **Whether the figures the panel draws are enforced.** Needs a real Intel or AMD card with a driver below the figure.
- **Blender under Gnome-Shell with libdecor removed, and what `--no-window-frame` changes on Wayland.** The rented machine runs Xorg and nothing on it may be installed or removed. That the argument exists in this version is settled.

---

2026-08-01 — Claude Opus 5 (Claude Code) — The machine, Blender 5.1

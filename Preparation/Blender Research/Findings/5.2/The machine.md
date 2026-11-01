# The machine — Blender 5.2

Sources consulted, all read on **2026-07-30** (the snapshot date given to me): Blender's manual for 5.2, Blender's Python API documentation for 5.2, the Blender 5.2 release notes, one image from the manual's `_images` folder, and Cycles' own files for 5.2.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One note on the source folder, for honesty: `Blender Sources/` also contains a handful of loose `.txt` files at its top level (`crashes.txt`, `cycles_device.txt`, `devices_api.txt`, `eevee_render.txt`, `eevee_results.txt`, `missing.txt`, `missing_filtered.txt`). I opened the first few lines of each to see what they were. They are plain search-result dumps — lines quoted out of the Blender documentation and Cycles files that sit alongside them, with file paths and line numbers. I did not use them as a source for any answer below; every finding here comes from my own reading of the 5.2 documentation and Cycles files.

---

## 1. What must a computer have on it before Blender will run at all?

**Label: documented**, except for the general minimum machine specification, which is **not in the sources** (see the record of looking at the end of this answer), and the inferred points marked below.

**The operating system.** Blender 5.2 runs on three:

> "Blender is available for download on Windows, macOS, and Linux."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html, read 2026-07-30

Each has its own condition.

On macOS, the machine must have an Apple silicon processor:

> "Blender 5.0 and later only supports Macs with Apple silicon."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html, read 2026-07-30

On Windows, the download must match the processor:

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html, read 2026-07-30

On Linux, the manual names X11 and Wayland as the supported windowing systems for the interface:

> "On Linux Blender supports both X11 and Wayland for official releases."
>
> "When Wayland is detected, it is the preferred system, otherwise X11 will be used."

**Label for the stronger conclusion that one must be present: inferred.** The quote says which systems are supported and which is preferred when detected. It does not state directly what happens when neither is present.

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux_windowing_environment.html, read 2026-07-30

**A graphics card and driver good enough for the interface.** Blender draws its own window on the graphics card, so this is a condition of starting, not only of rendering:

> "Always check that your graphics drivers are up to date and that OpenGL is properly supported."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html, read 2026-07-30

The floor is stated as a number in the troubleshooting pages:

> "This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

— Blender manual, https://docs.blender.org/manual/en/5.2/troubleshooting/gpu/windows/nvidia.html (the same sentence appears on every one of the twelve per-platform, per-vendor troubleshooting pages under `troubleshooting/gpu/`), read 2026-07-30

**Blender expects its bundled Python to be present and findable.** The manual lists this among the messages seen at startup:

> "found bundled python: {DIR}
>
> This message indicates that Blender was able to find the Python library for the Python interpreter embedded within Blender. If this folder is missing or unable to be found, it is likely that an error will occur, and this message will not appear."

— Blender manual, https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html, read 2026-07-30

Blender brings that Python with it rather than needing one on the machine:

> "Blender bundles a complete Python installation and does not interact with the system Python by default."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30

**Two conditions that are not hardware at all.** The manual's own checklist for "Blender fails to start" names them:

> "Make sure any antivirus software is not preventing Blender from starting.
>
> Ensure that you have appropriate user permissions on the computer."

— Blender manual, https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html, read 2026-07-30

**The broad statement of compatibility.** For machines set up for film and television work:

> "Blender follows the VFX reference platform, which means it is able to run on the same systems as other VFX software and exchange image, volume and scene files with them."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30

**What is not in the sources: the numbers.** The manual repeatedly points at a page that is not in the snapshot I was given:

> "Blender has a set of minimum and recommended requirements; so make sure these are met before trying to install Blender."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html, read 2026-07-30

and each of the three installation pages opens with "Check the Downloading Blender page to find the minimum requirements". That page lives on blender.org, not in the manual, and is not in `Blender Sources/`. So I have no memory figure, no processor figure, no minimum Windows version, no minimum macOS version for Blender itself, and no minimum Linux distribution.

*Record of looking, for that gap:* I searched the whole 5.2 manual for `system requirement`, `hardware requirement`, `minimum requirement`, `requirements`, `SSE`, `AVX`, `glibc`, `64-bit`, `Windows 10`, `Windows 11`, `macOS 1`, `Ubuntu`, `RAM`, `gigabyte`, `GB of`, and `processor`. The only hits that were about a minimum for running Blender were the "Downloading Blender" pointers above and the OpenGL 3.3 line. `Windows 10` appears only in the virtual-reality headset table; `macOS 1` appears only in the Cycles Metal requirement. I also read `getting_started/configuration/hardware.txt` in full, expecting a specification — it turns out to be about displays, mice, keyboards, tablets, 3D mice and VR headsets, with no computer specification at all. I ruled out the release notes by searching all 21 files for `device`, `gpu`, `driver`, `requirement`, `cuda`, `optix`, `hip`, `oneapi`, `metal`, `vulkan`, `opengl`, `macos`, `windows`, `linux`; nothing there states a minimum specification for 5.2.

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 5.2 started and ran normally, and **Help ‣ Save System Info** was used to write `system-info.txt`. The file's section headed `GPU:` read:

```
renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:
```

and was followed by sections headed `Implementation Dependent GPU Limits:` and `GLSL:`, then `Cycles:`. The whole file has since been read; there is **no section headed `OpenGL`** anywhere in it. This is a machine that meets the Apple silicon condition quoted above; it does not test the OpenGL floor.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — `/opt/blender/blender-5.2.0-linux-x64/blender` (`Blender 5.2.0 LTS`) was put to three tests bearing on this question. The machine is a container sharing somebody else's hardware, not a whole computer, and other work was running on it at the same time, so nothing here says how long anything took or how much memory was free.

**Blender starts and renders on Linux with no display reachable.** *(Wording corrected 2026-08-06 by the citation check of the two-card material. This originally said "with no display server at all", which the machine it was run on did not match — that machine had a real display server running and the run was merely cut off from it by removing `DISPLAY`. The two are different claims, and a machine with genuinely no display server is recorded separately at the end of this answer. Nothing observed here changes; only the description of the machine does.)* `DISPLAY` was removed from the environment and a background Cycles render completed at exit `0`, writing its PNG, on the graphics card:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
                            |                      Hardware Ray-Tracing: Off
                            | Denoising on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
00:09.945  render           | Saved: '/root/smoozy/The machine/work/nd-5.2-0001.png'
```

An EEVEE render did the same. So the X11-or-Wayland condition is about opening a window, not about running.

**Asked for a window with no display reachable, Blender refused and exited.** `env -u DISPLAY <blender> --python-expr "import bpy; print('WINDOW OPENED')"` never printed `WINDOW OPENED`. It printed, then exited with status `1`:

```
00:00.047  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.047  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4890, total unfreed memory 1.260777 MB
```

Note that 4.5 on the same machine printed the same information in a plainer shape and then hung rather than exiting.

**One line 5.2 prints on this machine that 4.5 does not**, but only when Cycles is asked for HIP:

```
00:00.468  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
```

Cycles reporting that AMD's library is not on the machine. Harmless here, but anything reading Blender's output has to expect it.

*(Corrected 2026-08-06 by the runner, after the check of the Linux hand tests could not reproduce this. The version split is right; "nearly every run" was not. Run again on the same machine with a plain Cycles render at factory settings, this line appeared **not at all**, in any of the four versions. It appears only when Cycles is actually asked for HIP — either the preferences are set to HIP, or the render is started with `--cycles-device HIP`. Then 5.0, 5.1 and 5.2 each print it exactly once and 4.5 prints nothing of the kind, failing instead with `Error: Found no Cycles device of the specified type`. So anything reading Blender's output has to expect this line when it asks for HIP, and not otherwise.)*

The OpenGL floor is still untested: with `gpu.init()` called first, this machine reports `4.6.0 NVIDIA 580.105.08`, above every figure the sources name.

**Not covered.** No numbers, as above. Nothing here tells me how much disk space an installation takes, whether a network is needed (beyond the note that Blender is designed not to require an internet connection), or what happens on a machine that meets some but not all of these.

**Outside the interface.** Most of the above still holds. The manual says a command-line render does not need a graphical display:

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30

The example names Linux. The sources do not say whether the OpenGL 3.3 floor still applies when Blender is started with `-b` and never draws a window. The release notes document a new way to start the graphics module in background mode, but do not say whether it otherwise remains unstarted:

> "Added `gpu.init()` to initialize gpu backend when running Blender in background mode `--background`."

— Blender 5.2 release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/, read 2026-07-30

That much is now observed on two machines. Started from outside its own window, 5.2 ran and rendered on the Creator's Mac and on the rented Linux machine, and on the Linux machine it did so with no display reachable — and, on a third machine that has no display server at all, in the block below. What does not hold outside the window is opening one, and what Blender says then is quoted in the Linux tests above. The OpenGL floor itself is still untested, both machines being far above it.

**Operating system.** No — the conditions differ by system, as set out above: Apple silicon only on macOS, a matching x64/arm64 build on Windows, and X11 or Wayland named on Linux. Two of the three are now tested: Linux runs and renders with neither X11 nor Wayland reachable, and refuses to open a window without them, naming both. Windows is untested. The graphics-driver advice is also system-specific: on macOS "graphics drivers are built into the operating system and the only way to get newer drivers is to upgrade macOS as a whole to the latest version" (https://docs.blender.org/manual/en/5.2/troubleshooting/gpu/apple/other.html, read 2026-07-30).

**Hand tests on a machine with no display server at all — observed.** On **2026-08-06**, on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-5.2.0-linux-x64/blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`. Label: **observed**. Nothing here is merged with the one-card rented machine's results or the Creator's Mac.

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
GPU  CYCLES exit=0  png=5067
GPU  BLENDER_EEVEE_NEXT exit=0  png=3478
GPU  BLENDER_EEVEE exit=0  png=3478
GPU  BLENDER_WORKBENCH exit=0  png=3669
CPU  CYCLES exit=0  png=5065
CPU  BLENDER_EEVEE_NEXT exit=0  png=3478
CPU  BLENDER_EEVEE exit=0  png=3478
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
00:00.059  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.059  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4890, total unfreed memory 1.260777 MB
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

---

## 2. What must a computer have on it before Blender will render using its graphics cards?

**The question is ambiguous** and I answer the reading I believe is meant. "Render using its graphics cards" I take to mean *Cycles rendering on the GPU*. The other reading is EEVEE, which only ever renders on the GPU; I cover that briefly at the end, because on that reading the answer is different.

**Label: documented** for the requirement lists; **observed** for the versions the Cycles add-on itself names on screen; **not in the sources** for what happens when a requirement is unmet at render time.

**Cycles GPU rendering, per manufacturer.** The manual gives one list per technology:

> "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher."
>
> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."
>
> "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."
>
> "Minimum driver versions:
>
> Windows: Radeon Software 24.9.1 or Radeon PRO Software 24.Q4
>
> Linux: Radeon Software 24.30 or ROCm HIP Runtime 6.3"
>
> "oneAPI is a computation library that is supported on Windows and Linux and requires an Intel® Arc™ graphics card with the Xe HPG architecture."
>
> "Minimum driver versions:
>
> Windows: Intel Graphics Driver XX.X.101.5518
>
> Linux: intel-level-zero-gpu package 1.3.27642, typically available through the intel-compute-runtime package XX.XX.34666.3"
>
> "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

— Blender manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30

**The driver, from the manufacturer.**

> "In case of problems, be sure to install the official graphics drivers from the GPU manufacturers website, or through the package manager on Linux. The graphics drivers provided by the computer manufacturer can sometimes be outdated or incomplete."

— same page, read 2026-07-30

**What does *not* have to be installed.** This is stated plainly, and it is the kind of thing that is easy to assume the other way round:

> "Normally users do not need to install the CUDA toolkit as Blender comes with precompiled kernels."

— same page, read 2026-07-30

**Requirement messages present in Cycles' own files.** The no-device branch contains its own copy of these numbers. Some differ from the manual's. Quoting the file:

```
            elif device_type == 'ONEAPI':
                import sys
                if sys.platform.startswith("win"):
                    ...
                    driver_version = "XX.X.101.8306"
                    col.label(
                        text=self._format_device_name(
                            rpt_("Requires Intel(R) Arc(TM) GPUs or newer Intel(R) Graphics")),
                        icon='BLANK1',
                        translate=False)
                    col.label(text=rpt_("with Windows driver version %s or newer") % driver_version,
                              icon='BLANK1', translate=False)
                elif sys.platform.startswith("linux"):
                    driver_version = "XX.XX.37435.3"
```

```
            elif device_type == 'METAL':
                mac_version = "12.2"
                col.label(text=rpt_("Requires Apple Silicon with macOS %s or newer") % mac_version,
                          icon='BLANK1', translate=False)
```

— Cycles' own files, `Blender Sources/cycles_5.2/properties.py`, lines 1904–1939 (inside `CyclesPreferences._draw_devices`), read 2026-07-30. **Label for the strings and their place in the file: observed.** Whether and when a running Blender shows them is not established by reading the file.

So on the same version, the manual says Intel Windows driver `XX.X.101.5518` and the add-on says `XX.X.101.8306`; the manual says Intel Linux `XX.XX.34666.3` and the add-on says `XX.XX.37435.3`; the manual says macOS 13.0 "to support all features" and the add-on says macOS 12.2. I report both without choosing between them. The add-on's AMD line is also worded more loosely than the manual's — `"Requires AMD GPU with RDNA architecture"` (same file, lines 1885 and 1896) against the manual's "RDNA1 architecture or newer".

**Three things must also be switched on.** Hardware alone is not enough:

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

— Blender manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30

And the starting position for those switches is **observed** in Cycles' own files. The compute device type defaults to entry `0`, which is `NONE`, on everything except Apple silicon, where it defaults to `5`, which is Metal:

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

— `Blender Sources/cycles_5.2/properties.py`, lines 1628–1635, read 2026-07-30

and the scene's own device setting starts on CPU:

```
    device: EnumProperty(
        name="Device",
        description="Device to use for rendering",
        items=enum_devices,
        default='CPU',
        update=update_render_passes,
    )
```

— `Blender Sources/cycles_5.2/properties.py`, lines 387–393 (in `CyclesRenderSettings`), read 2026-07-30

**A copy of Blender that was built with GPU rendering in it.** Not every copy is:

> "For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html, read 2026-07-30

**The other reading — EEVEE.** EEVEE has no requirement list of its own in these sources, but three things are stated:

> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient."
>
> "There is currently no support for multiple GPU systems."
>
> "Headless rendering is not supported on headless Windows systems."

— Blender manual, https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html, read 2026-07-30

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, Blender 5.2's **Edit ‣ Preferences ‣ System** page was opened and the **Cycles Render Devices** panel expanded. The row of technology buttons held exactly two, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — and two dropdown lists: **Kernel Optimization**, set to **Full**, offering **Off** and **Intersection Only**; and **MetalRT**, set to **Auto**, offering **Off** and **On**. Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed. In Blender's Python Console, `bpy.context.preferences.addons["cycles"].preferences.compute_device_type` returned `'METAL'`, matching the button shown as selected. No "Requires Apple Silicon with macOS … or newer" line appeared, because a Metal device was found, so this run records nothing about the macOS 12.2-against-13.0 disagreement above, and nothing about either oneAPI driver-number disagreement.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), a background Cycles render used the graphics card, ending `00:06.307  render           | Fra: 1 | Mem: 1607M | Sample 16/16 (1 kernels to optimize)`, so GPU rendering works in `--background` on this system. EEVEE rendered with no window too: `-b -E help` listed `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`, and a background EEVEE render exited `0`, ending `00:02.462  render           | Saved: '/tmp/eevee-52-0001.png'` — the manual's headless limitation is stated for Windows, and macOS is now tested. What Cycles does when a requirement is unmet was reached only as far as this machine allows: with every card switched off, and with the technology set to `NONE`, it rendered on the processor and said nothing. Full commands and output are under question 5.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, four things were put to 5.2.

**What this copy was built with.** `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed:

```
TYPES: (True, True, True, False, True, True)
```

Cycles' own file unpacks that as `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt`, so the official Linux x64 build carries CUDA, OptiX, HIP and oneAPI and not Metal.

**A graphics-card render works, with no window and with no display reachable.** With the technology saved as `CUDA` and the card ticked, a background render exited `0`, wrote its PNG and printed:

```
00:00.438  cycles           | Added device "NVIDIA GeForce RTX 4090" with id "CUDA_NVIDIA GeForce RTX 4090_0000:00:07".
```

ending, in the report `--debug-cycles` prints:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                            |                      Hardware Ray-Tracing: Off
                            | 
                            | Denoising on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
                            |                   Hardware Ray-Tracing: Off
```

With `OPTIX` saved, the same with the OptiX line. The `(1 kernels to optimize)` text this version carries on the Mac did **not** appear on Linux at all; that is a Metal marker, and on Linux the report line above is what names the device. Denoising ran on the processor in every graphics-card render here — which bears on the note above about the display card and the rendering card, though not on the requirement.

**What the panel says when the chosen technology has no compatible card — and this settles both oneAPI disagreements for Linux.** With a window open on the machine's own display, the technology set from a script and the System page photographed with `bpy.ops.screen.screenshot`, 5.2 drew, under **oneAPI**:

```
No compatible GPUs found for Cycles
Requires Intel® Arc™ GPUs or newer Intel® Graphics
  - intel-level-zero-gpu or intel-compute-runtime version
    XX.XX.37435.3 or newer
  - oneAPI Level-Zero Loader
```

and under **HIP**:

```
No compatible GPUs found for Cycles
Requires AMD GPU with RDNA architecture
and ROCm HIP Runtime 6.3 or newer
or AMD driver version 24.30 or newer
```

So the figure a running 5.2 draws for Intel on Linux is `XX.XX.37435.3`, the number in Cycles' own shipped file, not the manual's `XX.XX.34666.3`. And the wording of the first line is Cycles' own — `Requires Intel® Arc™ GPUs or newer Intel® Graphics` — not the manual's `Xe HPG architecture`. On the same machine, 5.0 and 5.1 draw `XX.XX.34666.3` and the older `Xe-HPG` wording, so the change is real and shows in what Blender itself puts on screen. The AMD figures likewise: 5.2 draws `6.3` and `24.30` where 5.0 and 5.1 draw `6.0` and `23.40`, and 4.5 draws only `23.40` with no ROCm line at all.

This records what the panel displays on a machine with no Intel or AMD card. It does not say what Blender would enforce against a real one, and the Windows figures cannot be read from Linux because the panel text branches on the operating system; a rented Windows machine would show them and none was used. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

The requirement line for CUDA and OptiX was never drawn, a compatible NVIDIA card being present. Its compute capability, as Blender's own system-information file reports it, is major `8`, minor `9`.

**EEVEE renders headless on Linux.** With `DISPLAY` removed, `-b <file> -E BLENDER_EEVEE -f 1` exited `0` and wrote its PNG:

```
00:01.008  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.373  render           | Saved: '/root/smoozy/The machine/work/eeveend-5.2-0001.png'
```


**Observed — what Blender does when the graphics driver itself is broken, 2026-08-06.** Every earlier answer here is about a technology having no card behind it. This is a different case and it had never been tested: the cards are present and the **driver** is gone or wrong. It was run on a second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, no display server, over SSH — which was broken on purpose for it and is not the machine the other rented-machine results here came from.

*What was done, to what.* Two states were made and this version was asked, in each, to render on the card through OptiX. First the driver packages were purged outright, so `nvidia-smi` and the graphics library were gone. Then the driver was reinstalled, which left the kernel module already running at `580.95.05` while the new library arrived as `580.105.08` — a mismatch, and the state an unasked overnight upgrade leaves behind until the machine is restarted.

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

*On what kind of computer.* The second rented Linux machine, described above. **This settles Linux on an NVIDIA machine only.** It was not run on the Creator's Mac, and Windows is untried everywhere.

*One thing this version does differently:* with the driver removed it prints `WARNING CUEW initialization failed: Error opening the library`, where Blender 4.5 prints nothing at all.

**Not covered.** The sources do not give a minimum amount of graphics memory. They do not say whether the display card and the rendering card may be the same card as a matter of requirement. They do not state a requirement list for the Vulkan drawing backend. What Cycles does at render time when the requirements are not met is now partly observed: with the chosen technology having no compatible card at all, it renders on the processor and says nothing (question 4); with that technology named on the command line, it refuses and exits `1`. What is still unknown is a card that is present but below a stated figure, which needs a card of that make.

**Outside the interface.** The hardware and driver requirements are properties of the machine. The documented `--cycles-device` option can choose the Cycles device from outside (see question 5). The preference and scene settings exist as Python properties, but the sources do not give a complete outside-interface recipe for setting all three switches. **Inferred:** on anything but Apple silicon, `default_device()` starts at `None`, so a working GPU setting appears to require something else to set it. Also, **observed**, Cycles is passed `bpy.app.background` at startup: `_cycles.init(path, user_path, bpy.app.background)` (`Blender Sources/cycles_5.2/engine.py`, line 52, read 2026-07-30). What that flag changes is inside the compiled part and is not visible in these files.

**Operating system.** No, this differs sharply by system. CUDA, OptiX, HIP and oneAPI are all stated as "supported on Windows and Linux" — none of them on macOS. Metal is macOS only, and Apple silicon only. Driver version numbers for HIP and oneAPI are given separately for Windows and Linux. EEVEE's headless limitation is Windows-specific. Two systems are now tested and both bear it out: on the Creator's Apple silicon Mac the technology row offered **None** and **Metal** and nothing else; on the rented Linux machine it offered five buttons — **None**, **CUDA**, **OptiX**, **HIP**, **oneAPI** — and no Metal, and drew the Linux driver figures quoted above. EEVEE rendered headless on both. Windows remains untested.

---

## 3. Does what a computer must have on it depend on how Blender was obtained and put there?

**Label: documented.** Yes, in several distinct ways.

**Whether GPU rendering is in the copy at all.** This is the sharpest case:

> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration. Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html, read 2026-07-30

**Whether administrator rights are needed.** On Windows the two official routes differ:

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."
>
> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender. No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html, read 2026-07-30

Registering Blender for all users is called out separately as needing more rights: "For All Users — Register Blender for all users, requires escalated privileges." (https://docs.blender.org/manual/en/5.2/editors/preferences/system.html, read 2026-07-30)

**What other software must already be there.** Each indirect route brings its own dependency:

> "Snap is a universal package manager designed to work across a range of distributions. Assuming snap is already installed, Blender can be installed through snap with:
>
> snap install blender --classic"

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html, read 2026-07-30

> "Download and install the Steam client for your operating system. Once it is installed, open the client and login to your Steam account, or create an account if you don't already have one."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/steam.html, read 2026-07-30

**What the extraction tool must be.** A named limitation:

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html, read 2026-07-30

**Whether the operating system will let it open.**

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html, read 2026-07-30

**Where it does *not* depend on the route.** The self-contained archive works anywhere on the disk:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30

**Inferred:** all three installation pages point at the same "Downloading Blender" page for minimum requirements, which suggests one specification rather than one per route. See question 1: that page is not in the sources, so this cannot be checked.

**Hand test on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, the copy under test was the official Linux x64 build from Blender's own download server, unpacked at `/opt/blender/blender-5.2.0-linux-x64/`. Asked what it was built with, `-b --factory-startup --python-expr "import _cycles; print('TYPES:', _cycles.get_device_types())"` printed `TYPES: (True, True, True, False, True, True)` — CUDA, OptiX, HIP and oneAPI present, Metal absent. So the route that matters most in this answer, a copy that carries no Cycles graphics-card rendering, is not what the official Linux download gives; it carries all four of the technologies its operating system supports. A distribution's own package is a different copy and is untested, because nothing may be installed on the rented machine.

**Not covered.** The sources do not say whether the graphics and driver requirements themselves differ by route (they read as the same everywhere). They do not say whether the Microsoft Store sandbox affects GPU rendering, whether a Steam or Snap copy carries Cycles GPU support, or what a copy built from source needs on the machine to build. None of those could be tested: two machines, one install route each.

**Outside the interface.** These are conditions on obtaining and installing Blender rather than on how a render is driven. The Windows installer still requires administrator rights in unattended use. The extractable archive is documented as suitable for automated deployment, but the manual does not say it is the required route. Blender is also stated to have no built-in update system: "Blender is designed to not require an internet connection, so it doesn't have a built-in update system. This means you will need to update Blender yourself" (https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html, read 2026-07-30). Snap, Steam and the Microsoft Store are each documented separately as updating Blender.

**Operating system.** No. The routes themselves are different per system: DMG on macOS; installer, zip or Microsoft Store on Windows; archive, distribution package or Snap on Linux; Steam on all three. The 7-zip point is Linux, the administrator-rights point is Windows, the Security and Privacy point is macOS.

---

## 4. Does Blender say anything about what is missing when it will not start, or when it will not render?

**Label: documented** for the startup and render messages the manual records; **observed** for message strings and exit handling present in Cycles' own files; **inferred** and **not in the sources** where marked.

**When it will not start.** The manual has a page of startup messages, and its opening advice is that you may have to go looking for them:

> "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window. To ensure you can see any error messages that appear at launch, open a command-line window and navigate to the location where the Blender executable is, and run blender from that location."

> "The Blender Console Window can display many different types of status and error messages. Some messages simply inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender non-responsive or shut down completely."

— Blender manual, https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html, read 2026-07-30

Two are named on that page: `found bundled python: {DIR}` and `Read prefs: {DIR}/userpref.blend`. Note that the Python one is diagnostic by its *absence* — "If this folder is missing or unable to be found, it is likely that an error will occur, and this message will not appear."

One named graphics message says what is missing:

> "Unsupported Graphics Driver Error
>
> This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

— Blender manual, https://docs.blender.org/manual/en/5.2/troubleshooting/gpu/windows/nvidia.html, read 2026-07-30

For a crash rather than a refusal, Blender writes a file:

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."

with the location given per system: `test.crash.txt` next to the loaded file on Windows (plus `blender.crash.txt` for crashes in the Blender module), the macOS Crash Reporter and `~/Library/Logs/DiagnosticReports/`, and `/tmp/blender.crash.txt` on Linux. — Blender manual, https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html, read 2026-07-30

**Messages prepared for the no-compatible-card case.** These are the exact strings present in the no-device branch. **Observed**, from Cycles' own files. Whether a running Blender displays them in every such case is not established by reading the file:

```
        if not found_device:
            col = box.column(align=True)
            col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)

            if device_type == 'CUDA':
                compute_capability = "5.0"
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

— `Blender Sources/cycles_5.2/properties.py`, lines 1864–1878 (inside `CyclesPreferences._draw_devices`), read 2026-07-30. The same block continues with the HIP, oneAPI and Metal wordings quoted in question 2, each split by operating system.

A second **observed** string is appended when a device's `is_optimized` value is false:

```
        for device in devices:
            name = self._format_device_name(device.name)
            if not device.is_optimized:
                name += rpt_(" (Unoptimized Performance)")
            box.prop(device, "use", text=name, translate=False)
```

— `Blender Sources/cycles_5.2/properties.py`, lines 1942–1946, read 2026-07-30

**When a render fails.** The manual lists four Cycles messages by their text and explains each: `Error: Out of memory` ("This usually means there is not enough memory to store the scene for use by the GPU"); `The NVIDIA OpenGL driver lost connection with the display driver`; `Unsupported GNU version`; and `CUDA Error: Kernel compilation failed` ("This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed"). — Blender manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30

**When the device override call returns false.** **Observed**, from Cycles' own files — the file then exits with status `1`:

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

— `Blender Sources/cycles_5.2/engine.py`, lines 38–41, read 2026-07-30

That the exit status is `1` is what the file says. **Inferred:** a refused device name is one reason the call may return false. The file does not state which inputs make it return false. Whether anything is printed before exit is decided inside `_cycles.set_device_override`, which is compiled and not in these files.

**Ways to ask for more to be said.** The logging options are documented, and one level is described in exactly these terms:

> "info: Information about devices, files, configuration, operations"

alongside `--log <match>` (with `--log "render,cycles"` given as an example), `--log-file <filepath>`, and `--debug-cycles` ("Enable debug messages from Cycles"). — Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30

The manual also names ready-made scripts shipped on Windows: "blender_debug_log.cmd is used in most cases. blender_debug_gpu.cmd and blender_debug_gpu_workaround.cmd log GPU-related errors." (https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html, read 2026-07-30)

One Python entry point is documented to raise rather than return quietly:

> "gpu.init()
>
> Initializes the GPU module for background use. If the initialization fails, a SystemError will be raised."

— Blender Python API, https://docs.blender.org/api/5.2/gpu.html, read 2026-07-30

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, every technology button in Blender 5.2's **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** row was clicked in turn. The row held **None** and **Metal**. Neither the "No compatible GPUs found for Cycles" line nor any "Requires…" line appeared under either. The two tick boxes were labelled exactly **Apple M1 Pro (GPU - 14 cores)** and **Apple M1 Pro**, with no " (Unoptimized Performance)" appended to either. A Metal device was present on this machine, so the no-compatible-card case was never reached and this run records nothing about what it displays, on the panel or anywhere else.

**Hand test from a terminal — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.2 in background mode printed a message for two failure cases and nothing for a third.

`-- --cycles-device OPTIX`: exit status `1`, no image file, and the literal line

```
00:01.557  reports          | ERROR Found no Cycles device of the specified type
```

`-- --cycles-device NOTADEVICE`: exit status `1`, no image file, and the literal line

```
00:00.257  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
```

So the `sys.exit(1)` this answer records is accompanied by text, and the exit status is `1` as the file shows.

A Cycles render asked for graphics cards with **every card switched off in the saved preferences** exited `0`, wrote the image, and printed nothing naming a missing or unusable device; the same held with the technology set to `NONE`. Full commands and output are under question 5.

**Crash files, observed in passing.** Several runs during the day ended in exit status `139` with a line of the form `Writing: /var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/blender.crash.txt`. The crash file's own `# Python backtrace` named a background thread in a third-party add-on installed on this machine, not device handling, and re-running the same command completed normally. Recorded here only because it shows the crash-file mechanism working on macOS and the name it used.

**Hand tests on the rented Linux machine — observed.** On **2026-08-06**, on the rented Linux machine, five things bearing on this question were put to 5.2.

**Devices Cycles will not accept.** `-- --cycles-device METAL`, `-- --cycles-device HIP` and `-- --cycles-device ONEAPI` all ended at exit `1`, wrote no image, and printed one line:

```
00:00.671  reports          | ERROR Found no Cycles device of the specified type
```

HIP and oneAPI are built into this copy and simply have no card; Blender says the same thing for that as for Metal, which is not built in at all. `-- --cycles-device NOTADEVICE` gave exit `1`, no image, and two lines:

```
00:00.224  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

Both messages are word for word the Mac's; the second line is one 4.5 does not print on the same machine.

**What Cycles prints when a render is asked for on the graphics card and no card is usable — settled here, and the answer is nothing.** With the technology saved as `ONEAPI` and no Intel card present (`SAVED: ONEAPI ACTIVE: False NUMGPU: 0`), a background render exited `0`, wrote its image and rendered on the processor. The whole output was searched for the phrase `No compatible GPUs found for Cycles` and it appeared **nought times**; no `Requires…` line appeared either. So the panel messages really are panel-only, as this answer reasoned. What they say is quoted under question 2.

**Where the crash file lands on Linux.** A crash forced with `-b <file> --python-expr "import ctypes; ctypes.string_at(0)"` gave exit `139` and:

```
Writing: /tmp/machine-test.crash.txt
```

With no blend-file loaded, `Writing: /tmp/blender.crash.txt`. So the manual is right about the folder and incomplete about the name: with a file loaded, the crash file is named after that file, and it still goes to `/tmp` rather than beside the blend-file.

**Exit statuses.** Three more are now on record beyond the `1` above: `0` for a render that fell back to the processor and wrote an image, `139` for a crash, and `1` for a windowed start with no display server.

**5.2 alone has a message about the drawing card being unavailable.** Started with a window, the Vulkan backend and a device number that does not exist:

```
00:00.627  ghost.context    | ERROR Requested Vulkan GPU '99' is unavailable or unsupported. Run with '--gpu-device help' to list available devices.
```

and used with the OpenGL backend, where it does not apply:

```
Warning: '--gpu-device' is only supported for the Vulkan backend. Ignoring the device override for the selected OpenGL backend and falling back to the stored GPU preference.
```

Neither message exists in 4.5, 5.0 or 5.1, which have no such argument. Both are about the card that draws, not the card that renders.

**Not covered.** The sources do not give the wording of any message shown when Blender refuses to start for want of a good enough graphics card — only the advice to look at the console and the one named "Unsupported Graphics Driver Error" — and no machine below the floor exists to reach it. They do not say which of these messages reach standard output as against standard error. What Cycles prints when no graphics card is usable is now answered.

**Outside the interface.** The messages quoted from the Cycles preferences are drawn on screen — they are label text in a panel, so a program driving Blender without a window will never see them. That is now observed rather than reasoned, on the rented Linux machine. The console messages and the crash files do survive, and the manual's own advice for seeing startup messages is to run from a command line. The `--log`, `--log-level`, `--log-file` and `--debug-cycles` options are command-line only by nature, and `--log-file` writes to a file, which is the form most useful to something reading the result later. The `sys.exit(1)` on a bad `--cycles-device` is a command-line behaviour by definition.

**Operating system.** Mostly yes, with named exceptions. The crash file location differs on all three systems, as quoted; two of the three are now observed — `blender.crash.txt` in the Mac's own temporary directory, and `/tmp/<name of the loaded file>.crash.txt` on Linux. The debug `.cmd` scripts are Windows only and were not run. `--debug-ghost` is marked "(Linux only)" and `-con, --start-console` "(Windows only)". The Cycles requirement messages branch on `sys.platform` for HIP and oneAPI, so a Linux machine and a Windows machine are told different things about the same missing card — and the Linux half of that is now quoted under question 2, while the Windows half stays unread. One line appears on Linux that the Mac never showed: `WARNING HIPEW initialization failed: Error opening HIP dynamic library`, at the start of very nearly every run.

---

## 5. How is it decided which of a computer's graphics cards a render uses, can more than one card be used for the same render, and how is that decided from outside Blender's own interface?

**Label: documented** for the mechanism and for multiple cards; **observed** for the defaults and the shape of the selection; one **inferred** point is marked where it occurs.

**How it is decided.** In two places, which must agree. First, one *technology* and a set of *cards* in the preferences; second, a per-scene choice of processor or graphics card:

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

— Blender manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30

The scene-side setting is described as:

> "GPU Compute:
>
> Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices. Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."

— Blender manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/index.html, read 2026-07-30

I also looked at the picture of that preferences panel — `_images/editors_preferences_section_system.webp`, shown on https://docs.blender.org/manual/en/5.2/editors/preferences/system.html, read 2026-07-30. **Label for what follows: documented**, since the picture is part of the manual. It shows "Cycles Render Devices" as a single row of mutually exclusive buttons — on the machine pictured, `None`, `CUDA` and `HIP`, with `None` selected. What is visible there is that the technology is a one-of-several choice, not a set. What cards would be listed underneath is not visible in that picture, because the selection is on `None`.

**The file holds one technology value at a time, and builds a list for that technology.** **Observed**, from Cycles' own files. The technology is a single-choice property whose list is built from what the machine has:

```
    def get_device_types(self, context):
        import _cycles
        has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()

        list = [('NONE', "None", n_("Do not use compute device"), 0)]
        if has_cuda:
            list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))
```

— `Blender Sources/cycles_5.2/properties.py`, lines 1637–1653, read 2026-07-30

and the list of cards offered is the cards of that one type, with the processor added on the end:

```
        # Sort entries into lists
        devices = []
        cpu_devices = []
        for device in device_list:
            entry = self.find_existing_device_entry(device)
            entry.is_optimized = device[7]
            if entry.type == compute_device_type:
                devices.append(entry)
            elif entry.type == 'CPU':
                cpu_devices.append(entry)
        # Extend all GPU devices with CPU.
        if len(devices) and compute_device_type != 'CPU':
            devices.extend(cpu_devices)
        return devices
```

— `Blender Sources/cycles_5.2/properties.py`, lines 1748–1761, read 2026-07-30

**What each card starts as.** **Observed** — when a card is met for the first time it is ticked, and the processor is not:

```
            if not entry:
                # Create new entry if no existing one was found
                entry = self.devices.add()
                entry.id = device[2]
                entry.name = device[0]
                entry.type = device[1]
                entry.use = entry.type != 'CPU'
```

— `Blender Sources/cycles_5.2/properties.py`, lines 1730–1736, read 2026-07-30

The list is not rebuilt on its own at startup, and the file says why:

```
    # Refresh device list. This does not happen automatically on Blender
    # startup due to unstable drivers that can cause crashes.
    def refresh_devices(self):
```

— `Blender Sources/cycles_5.2/properties.py`, lines 1763–1765, read 2026-07-30

**Can more than one card be used for the same render?** Yes, documented directly:

> "Can multiple GPUs be used for rendering?
>
> Yes, go to Preferences ‣ System ‣ Compute Device Panel, and configure it as you desire."

— Blender manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30

With one important consequence spelled out on the same page:

> "Would multiple GPUs increase available memory?
>
> Typically, no, each GPU can only access its own memory.
>
> The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."

and in the preferences page:

> "Distribute Memory Across Devices
>
> Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes. Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol.
>
> Currently only NVLink on NVIDIA GPUs is supported."

— Blender manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html, read 2026-07-30

The processor can join in too — "Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously" (quoted above).

One further **observed** rule in the file: if any enabled device lacks hardware ray-tracing support, `has_hardware_rt` is false for the group. What effect that has on a running render is not established here:

```
        # Any device without RT support will disable it for all.
        has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt
```

— `Blender Sources/cycles_5.2/properties.py`, lines 1976–1977, read 2026-07-30

For EEVEE the answer is the opposite: "There is currently no support for multiple GPU systems." (https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html, read 2026-07-30)

**How it is decided from outside Blender's own interface.** There is a command-line option, documented:

> "--cycles-device <device>
>
> Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU.
>
> Example:
>
> blender -b file.blend -f 20 -- --cycles-device OPTIX"

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30

Note the two dashes on their own before it — the same page says "Cycles add-on options must be specified following a double dash."

The add-on's own help text for that option says what it beats, which the manual does not. **Observed**:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

— `Blender Sources/cycles_5.2/engine.py`, lines 16–20, read 2026-07-30

**A limit on that option — label: inferred.** `--cycles-device` names a *technology*, not a card. The documented list of valid values contains only technology names and the `+CPU` suffix, and there is no syntax in it for naming one card among several. The per-card choice lives in a separate collection of settings, each with its own `use` switch:

```
class CyclesDeviceSettings(bpy.types.PropertyGroup):
    # Runtime properties
    __slots__ = ("is_optimized")

    # Properties saved in preferences
    id: StringProperty(name="ID", description="Unique identifier of the device")
    name: StringProperty(name="Name", description="Name of the device")
    use: BoolProperty(name="Use", description="Use device for rendering", default=True)
    type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
```

— `Blender Sources/cycles_5.2/properties.py`, lines 1614–1622, read 2026-07-30

The step I am taking: because the option's value set contains no card identifier, and because the card-by-card switches are stored separately in the preferences, I conclude that using `--cycles-device` alone chooses the technology but leaves the choice of *which* cards of that technology to whatever the preferences hold. A reader has to supply that joining step, so this is inferred, not documented.

**Inferred:** the card-by-card switches are properties on the Cycles add-on's preferences, and Blender's documented `--python`, `--python-expr` and `--python-text` options provide ways to run a script from outside. The sources do not provide the script that joins those pieces. The add-on reaches its own preferences as `context.preferences.addons[__package__].preferences` (`Blender Sources/cycles_5.2/ui.py`, lines 96–101, read 2026-07-30, **observed**). The literal key `"cycles"` is not stated anywhere read; the closest evidence is the API documentation giving Cycles' operator source as `addons_core/cycles/operators.py` (https://docs.blender.org/api/5.2/bpy.ops.cycles.html, read 2026-07-30).

**A different question that looks like this one.** Which card draws Blender's *interface* is a separate setting, and on Vulkan it can be pointed at a specific card:

> "Device Vulkan
>
> Specifies which GPU device to use for display drawing operations.
>
> This setting is useful for systems with multiple GPUs (e.g., integrated + discrete) where you want to force Blender to use a specific GPU for UI rendering."

— Blender manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html, read 2026-07-30

**Hand test — observed.** On **2026-08-01**, on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), the whole of **Edit ‣ Preferences ‣ System** was looked at, in all four versions. The page held, top to bottom: **Cycles Render Devices** (two buttons, `None` and `Metal`, with `Metal` selected; two tick boxes, `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked; `Kernel Optimization` showing `Full`; `MetalRT` showing `Auto`), **Network** (`Allow Online Access` ticked, `Time Out` 10, `Connection Limit` 5), **Memory & Limits** (`Undo Steps` 256, `Undo Memory Limit` 0, `Global Undo` ticked, `Console Scrollback Lines` 256, `Texture Time Out` 120, `Garbage Collection Rate` 60, `VBO Time Out` 120, `Garbage Collection Rate` 60, `Geometry Nodes Stack Limit` 100), **Video Sequencer** (`Memory Cache Limit` 16384, `Proxy Setup` showing `Automatic`), and **Sound**, collapsed.

**There was no Display Graphics section, and no graphics backend or device control anywhere on the page** — in any of the four versions. So on this machine the setting quoted above was not present to be read.

One thing to say plainly rather than let it read as a failed search: the test that produced this was mis-aimed. It asked what the **Device** list offers once the backend is set to **Vulkan**, and Vulkan is not something a Mac offers at all — the question assumed a control that does not exist on this operating system. What a Windows or Linux machine shows here is untested, because neither exists among the equipment available.

And on a laptop, the choice can be made before Blender is involved at all: "Which GPU to use for which application can be configured in your graphics driver or operating system settings." (https://docs.blender.org/manual/en/5.2/troubleshooting/gpu/windows/nvidia.html, read 2026-07-30)

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 5.2.

**The panel.** **Edit ‣ Preferences ‣ System ‣ Cycles Render Devices** was expanded and every button in its technology row clicked in turn. The row held exactly two buttons, **None** and **Metal**, with Metal selected on arrival; selecting None hid every other setting in the panel. With Metal selected the panel listed two tick boxes — **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, unticked — plus **Kernel Optimization** (**Full**; also **Off**, **Intersection Only**) and **MetalRT** (**Auto**; also **Off**, **On**). Unticking **Apple M1 Pro (GPU - 14 cores)** disabled the MetalRT control; nothing else changed.

**From Blender's Python Console** (Scripting workspace, window open). `list(bpy.context.preferences.addons.keys())` returned:

```
['io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'cycles', 'pose_library', 'bl_pkg']
```

so the add-on key is the literal string `cycles` on this machine — the string the sources would not give. With `p = bpy.context.preferences.addons["cycles"].preferences`:

- `p.compute_device_type` returned `'METAL'`.
- `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned, **before `refresh_devices()` was called**:

```
[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

- After `p.refresh_devices()` the same expression returned the same two entries.
- `p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, `p.has_multi_device()` returned `False`.
- `p.devices[0].use = False` was run — index 0 being the processor entry, unticked by default — and the panel then showed exactly what it shows by default. `p.devices[0].use = True` was run, and the **Apple M1 Pro** box in the panel became ticked.

So on this machine the "Persistent ID" was the string `CPU` for the processor and `METAL_Apple M1 Pro (GPU - 14 cores)` for the graphics card. One run records nothing about whether those identifiers survive a driver update or a card being moved. All of this was done from a Python Console inside Blender's own window; it records nothing about a background run.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender.app/Contents/MacOS/Blender`, which reports `Blender 5.2.0 LTS`. Scene: `/tmp/machine-test.blend`, Cycles, scene Device `GPU`, 16 samples. Output quoted as printed.

**The settings read with no window open.**

```
CYCLES KEYS: ['cycles']
BACKEND: METAL
BEFORE: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
AFTER: [('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

With `--factory-startup` added, `BEFORE:` was `[]` and `AFTER:` showed the card ticked and the processor unticked — so `--factory-startup` discards the saved per-card switches. The raw list, from `_cycles`:

```
TYPES: (False, False, False, True, False, False)
METAL: (('Apple M1 Pro', 'CPU', 'CPU', False, False, True, False, True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', False, True, True, False, True))
```

with `_cycles.available_devices()` raising `TypeError: function takes exactly 1 argument (0 given)`. The identifier this answer calls the "Persistent ID" was `CPU` and `METAL_Apple M1 Pro (GPU - 14 cores)`, unchanged across every run of the day.

**A background render with the saved preferences as found** exited `0`, wrote its PNG, and printed:

```
00:01.617  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
00:01.617  cycles           | Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
00:01.812  render           | Fra: 1 | Mem: 1457M | Sample 0/16 (Using optimized kernels)
00:06.307  render           | Fra: 1 | Mem: 1607M | Sample 16/16 (1 kernels to optimize)
00:06.308  render           | Fra: 1 | Mem: 1607M | Finished
```

**Every card switched off in the saved preferences, then rendered.** The setting command printed `SAVED STATE: METAL [('Apple M1 Pro', False), ('Apple M1 Pro (GPU - 14 cores)', False)]` — `bpy.ops.wm.save_userpref()` runs with no window — and the render exited `0`, wrote its PNG, and ended:

```
00:05.930  render           | Fra: 1 | Mem: 151M | Sample 16/16
```

with nothing naming the switched-off card. So the saved switches are honoured by a windowless render.

**The same state with `-- --cycles-device METAL`** exited `0` and ended:

```
00:05.484  render           | Fra: 1 | Mem: 1607M | Sample 16/16 (1 kernels to optimize)
```

**With the scene's own Device saved as `CPU`**, `-- --cycles-device METAL` still gave `00:05.700  render           | Fra: 1 | Mem: 1607M | Sample 16/16 (1 kernels to optimize)`. So the flag displaced both the preferences and the scene setting — the two things the add-on's help text claims, now tested rather than inferred.

**The technology set to `NONE`** gave a render on the processor with no message: `00:05.840  render           | Fra: 1 | Mem: 151M | Sample 16/16`.

**Values `--cycles-device` refuses.** `-- --cycles-device OPTIX`: exit `1`, no image, `00:01.557  reports          | ERROR Found no Cycles device of the specified type`. `-- --cycles-device NOTADEVICE`: exit `1`, no image, `00:00.257  cycles           | ERROR NOTADEVICE is not a valid Cycles device.` `-- --cycles-device METAL+CPU`: exit `0`, image written, ending `00:05.527  render           | Fra: 1 | Mem: 151M | Sample 16/16`.

**Preference changes made inside the render's own run** did not produce the graphics-card signature. Setting every `use` to `True` in the same process that then rendered printed `SET TO: [('Apple M1 Pro', True), ('Apple M1 Pro (GPU - 14 cores)', True)]` and ended `00:05.417  render           | Fra: 1 | Mem: 151M | Sample 16/16`. Setting them all to `False`, and setting `compute_device_type` to `'NONE'`, gave the same shape of result. The saved preferences were unchanged afterwards, and were restored to the state found.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-5.2.0-linux-x64/blender`. Its settings folder started empty, so this version began from factory settings rather than from anything a person had set. One thing about that folder, said plainly rather than left out. The first runs were meant to point Blender at a folder of this topic's own with `BLENDER_USER_RESOURCES`, and a mistake in the shell meant the variable was never actually set, so those runs used Blender's ordinary default folder instead. That folder was also empty when this work began, so "began from factory settings" holds either way. Every test that turns on saved preferences was then run again with the variable properly set, and every result came back identical. Both runs are described in `Audits/Blender Research/The machine/Hand tests — Linux.md`.

**What Blender finds, and how the one card is listed.** With an untouched settings folder:

```
CYCLES KEYS: ['cycles']
BACKEND: NONE
BEFORE: []
AFTER: [('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', True)]
```

The add-on key is `cycles` here too. The technology starts at `NONE` on this machine and at `METAL` on the Mac. Every graphics entry starts ticked and the processor unticked. And **one physical card is listed twice**, once under CUDA and once under OptiX, with two stored identifiers. Those identifiers visibly carry the card's place on the bus — `0000:00:07` — which is a hint about what a "Persistent ID" is made of, though nothing states it. With preferences saved, `BEFORE` came back already populated; `--factory-startup` emptied it again. `has_active_device()` returned `True`, `get_num_gpu_devices()` returned `1`, `has_multi_device()` returned `False`.

**What happens if `--cycles-device` names a technology with no card — answered.** `-- --cycles-device HIP` and `-- --cycles-device ONEAPI`, both built into this copy but with no card, exited `1`, wrote no image and printed `ERROR Found no Cycles device of the specified type` — the same as `METAL`, which is not built in at all.

**The saved switches are honoured, and the flag displaces them.** With the technology saved as `OPTIX` and every device switched off, a background render exited `0` and reported the processor. The same render with `-- --cycles-device OPTIX` appended reported `Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]`. With the devices back on and the technology set to `NONE`, the processor again, and no `Added device` line.

**The flag also displaces the scene's own setting.** The scene saved with Device `CPU`, rendered with `-- --cycles-device OPTIX`, reported the card.

**Whether the flag is written back into the preferences — no.** After every run using `--cycles-device`, the saved preferences read back exactly as they had been set.

**`+CPU` uses both, and Linux names both.** With `-- --cycles-device CUDA+CPU`:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                            |                      Hardware Ray-Tracing: Off
                            |                  AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (11 threads) [CPU]
                            |                      Hardware Ray-Tracing: Off
```

Eleven threads for the processor there and twelve when it works alone.

**A change made by a script inside the render's own run — and here the two machines disagree.** Three runs, each changing the preferences and then rendering in the same command, never saving them. Every device off: the processor. Every device on: the card and the processor both named. The technology set to `NONE`: `BACKEND NOW: NONE`, and the processor. All three exited `0` and wrote an image. On the rented Linux machine the change **took effect**. On the Creator's Mac the same three runs carried no graphics-card marker at all. Both stand; neither is chosen between here.

**5.2 has a command-line way of naming the card that draws, which no other version has.** Blender's own `--help` on this machine lists, in 5.2 alone:

```
--gpu-device <device>
	Select a specific GPU device, overriding the GPU device from user preferences for this
	run only. Accepted forms:

	* '<vendor-hex>/<device-hex>/<index-hex>' (matches the identifier used in user
	  preferences, with vendor PCI ID, device PCI ID and enumeration index all hex-encoded).
	* '<index>' (no slashes) picks the Nth supported device by enumeration order, decimal.
	* 'help' prints supported Vulkan devices and exits.

	Only used with the Vulkan backend. Other backends print a warning and ignore this option.
	If the requested Vulkan device is unavailable, Blender falls back to the saved GPU
	preference unless '--gpu-device-no-fallback' is also set.

--gpu-device-no-fallback 
	Fail instead of falling back when '--gpu-device' does not match a usable Vulkan device.
```

`--gpu-device help` printed a list and exited, **with no window and no display reachable** (this machine had a display server running; the same command on a machine with none at all is under question 6):

```
Blender GPU Device Listing (Vulkan):
Pass an Index or Device-ID to '--gpu-device'.
  Index  Device-ID    Name
  0      10de/2684/0  NVIDIA GeForce RTX 4090
  1      10de/2684/1  NVIDIA GeForce RTX 4090
  2      10005/0/2    llvmpipe (LLVM 15.0.7, 256 bits)
```

Used on a windowless render, where the backend is OpenGL, it said so and carried on:

```
Warning: '--gpu-device' is only supported for the Vulkan backend. Ignoring the device override for the selected OpenGL backend and falling back to the stored GPU preference.
```

and with a window, the Vulkan backend and a number that does not exist:

```
00:00.627  ghost.context    | ERROR Requested Vulkan GPU '99' is unavailable or unsupported. Run with '--gpu-device help' to list available devices.
```

In 4.5 the argument does not exist and Blender treats it as a file name (`unknown argument, loading as file: --gpu-device`). This names the card Blender **draws** with, not the card Cycles renders on, and it does not touch `--cycles-device`.

**The Display Graphics section on Linux.** Read with no window, `GPU_BACKEND NOW: OPENGL | CHOICES: ['OPENGL', 'METAL', 'VULKAN']` and `GPU_PREFERRED_DEVICE NOW: AUTO | CHOICES: ['AUTO']`. With a window, the System page carries a **Display Graphics** section, which the Mac has not got, showing `Backend: OpenGL`. With the backend saved as `VULKAN` and Blender restarted, a **Device** control appears reading `Auto`. This version does **not** show the "Current Vulkan backend limitations: • Low performance in VR" note that 4.5 and 5.0 show on the same machine. `gpu_preferred_device` still held only `['AUTO']` with Vulkan running, there being one card.

**Not covered.** How the work is divided between two cards, or whether cards of unequal speed are used unequally — needs a second card. What the "Persistent ID" is made of, or whether it survives a driver update or a card being moved between slots. Whether `--cycles-device` applies to one render or for the life of the process, though it is now observed not to be written back into the preferences.

**Outside the interface.** The technology choice is addressed directly: the documented `--cycles-device` example is a background render (`blender -b file.blend -f 20 -- --cycles-device OPTIX`), and the add-on's help text says it overrides both the preference and the scene setting. The card-by-card part is inferred: reaching those properties from outside would require a Python script, and the sources do not give a complete recipe. The hand tests supply the missing piece of that recipe on one machine, and with no window open: the key is the literal string `cycles`, the switches were read and written through `--python-expr` under `-b`, `bpy.ops.wm.save_userpref()` works there, and the saved file is honoured by a windowless render. `--cycles-device` displaced both the saved switches and the scene's own Device setting; `--factory-startup` discards the saved switches. A change made inside the render's own run did *not* produce the graphics-card signature, so a same-run change is not established as taking effect. What no test here could reach is a machine with more than one card.

**Operating system.** The documented values differ. `METAL` is only meaningful on macOS and only on Apple silicon; `CUDA`, `OPTIX`, `HIP` and `ONEAPI` are documented for Windows and Linux (question 2). The default technology is Metal on Apple silicon and None elsewhere — both halves now observed, `METAL` on the Mac and `NONE` on the rented Linux machine. "Distribute Memory Across Devices" is documented as depending on NVLink and needs two cards, so it could not be tested. EEVEE's lack of multiple-GPU support is stated without an operating-system qualification and likewise could not be tested. Two systems are now tested and every selection route behaved the same on both, with one exception, which is the same-run change recorded above. Linux additionally has a Display Graphics section, an Operating System Settings section and, in this version, the `--gpu-device` argument; macOS has none of the three. Windows remains untested.


### Hand tests on the two-card machine — observed
**Read this block against the *Not covered* paragraph just above it.** That paragraph was written before any two-card machine existed, so some of what it calls untested is answered here — how work is divided between two ticked cards, and the several-card form of the `--cycles-device` question above all. What it says about an NVLink pair, about two cards of different makes, about a slow card beside a fast one, and about anything needing a window still stands. *(Pointer added 2026-08-06 by the citation check of the two-card material.)*

Run on **2026-08-06** on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, run over SSH** — against `/opt/blender/blender-5.2.0-linux-x64/blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`. Label: **observed**.

**This is a different machine from the one-card rented Linux machine above, and nothing here is merged with that machine's results or with the Creator's Mac.** Its settings folder was `/root/smoozy/two cards/res-5.2`, empty when this work began, exported as `BLENDER_USER_RESOURCES` in the shell that ran Blender every time. **Nothing else was running on the machine**, which is why the timings are quoted at all. The full list is `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`.

**The two cards are not joined by NVLink.** `nvidia-smi topo -m` reports `PHB` between them — `Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)` — so this machine still cannot test an NVLink pair.

**Both cards are switched on at factory settings, and one card is listed twice.** With an untouched settings folder, `--factory-startup`, and a script that calls `refresh_devices()` and then prints every device as `(name, type, id, use)`:

```
PREFS PATH: /root/smoozy/two cards/res-5.2
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
| timestamp on `Finished` | `00:31.079`, `00:29.884`, `00:30.746` | `00:57.508`, `00:57.716`, `00:57.735` | `00:57.460`, `00:57.395`, `00:57.233` |
| the `Time:` summary line | `Time: 00:33.58 (Saving: 00:00.45)`, `Time: 00:33.62 (Saving: 00:00.33)` | `Time: 00:59.53 (Saving: 00:00.37)`, `Time: 00:59.78 (Saving: 00:00.42)` | not run in this form |

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

**What the memory readouts say with two cards.** Blender prints one figure, never two, and never names a card beside a memory number. On the timing scene its `Finished` line read `Mem: 8M` with two cards and `Mem: 1101M` with one — **smaller with two cards, not larger and not doubled**. What that field counts is not settled here and nothing is read into it.

**Frame order and output.** A four-frame animation was rendered with both cards and then with one. The order was ascending both ways:

```
00:04.690  render           | Saved: '/root/smoozy/two cards/work/ord_5.2/f_0001.png'
00:06.124  render           | Saved: '/root/smoozy/two cards/work/ord_5.2/f_0002.png'
00:07.456  render           | Saved: '/root/smoozy/two cards/work/ord_5.2/f_0003.png'
00:08.733  render           | Saved: '/root/smoozy/two cards/work/ord_5.2/f_0004.png'
```

The files were not byte-identical — but neither were two runs on the same cards, and Blender stamps a date into the PNG, so bytes cannot answer this alone. Comparing the pixels instead, of 921,600 values: **11 differed between two cards and one, 14 between two runs on two cards, and 10 between two runs on one card**, and the largest difference in every one was `0.003921568393707275` — one step in 255.

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

**Not covered.** Whether cards of two different makes can be ticked at once — both cards here are the same NVIDIA model. Whether a slow card holds a fast one back. What Distribute Memory Across Devices does when the cards **are** joined by NVLink. Whether that setting's box is drawn, greyed or hidden on an unlinked pair, and what the Vulkan **Device** control offers with two cards — both need a window, and this machine has no display server. Taking a card away mid-render was tried in 4.5 only and is recorded there.

**Outside the interface.** Everything above was driven from a terminal with no window and no display server. The sharp new point is that **the individual cards are still only reachable through the saved preferences or Python**, and that `--cycles-device` is all-or-nothing: it takes every card of its technology rather than the ones that were ticked.

**Operating system.** Linux only, as with the one-card machine, which this second machine agrees with on everything the first could reach. macOS was never asked any of the two-card questions, the Creator's Mac having one set of graphics. Windows is untested.
---

## 6. Can Blender be asked which graphics cards it can see and can use?

**Label: documented** for the reporting features; **observed** for the Cycles calls and their cautions; **inferred** where marked.

**Yes — a report Blender writes for you.** The manual describes it and, importantly, says the report has a Cycles section, not only a display section:

> "Save System Info
>
> This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems.
>
> You will be prompted to save a text file called system-info.txt."

and among its sections:

> "GPU
>
> Shows the GPU vendor, version and the capabilities of your hardware and driver."
>
> "Cycles
>
> The instruction sets and capabilities of each hardware render device available for use with Cycles."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/help.html, read 2026-07-30

The troubleshooting pages point at the same thing for the narrower question: "To find out which graphics card and driver Blender is using, use Help ‣ Save System Info inside Blender. The OpenGL section will have information about your graphics card, vendor and driver version." (https://docs.blender.org/manual/en/5.2/troubleshooting/gpu/windows/nvidia.html, read 2026-07-30)

The same report can be asked for without touching a menu:

> "bpy.ops.wm.sysinfo(*, filepath='')
>
> Generate system information, saved into a text file"

— Blender Python API, https://docs.blender.org/api/5.2/bpy.ops.wm.html, read 2026-07-30

**A direct list from the graphics-platform module.** The API has a function for it:

> "gpu.platform.devices_get()
>
> Get all available GPU devices.
>
> Returns:
>
> List of GPUDevice objects for each device."

with `backend_type_get()` ("Backend type ('OPENGL', 'VULKAN', 'METAL', 'NONE', 'UNKNOWN')"), `device_type_get()` ("Device type ('APPLE', 'NVIDIA', 'AMD', 'INTEL', 'SOFTWARE', 'QUALCOMM', 'UNKNOWN')"), `renderer_get()` ("Get GPU to be used for rendering. Returns: GPU name."), `vendor_get()` and `version_get()` ("Get GPU driver version."). — Blender Python API, https://docs.blender.org/api/5.2/gpu.platform.html, read 2026-07-30

Each item carries three things:

> "class gpu.types.GPUDevice
>
> Represents a GPU device.
>
> Variables:
>
> index – Device index.
>
> identifier – Device identifier.
>
> name – Device name."

— Blender Python API, https://docs.blender.org/api/5.2/gpu.types.html, read 2026-07-30

**Inferred:** the surrounding module wording and the OpenGL, Vulkan and Metal backend values indicate that this list concerns Blender's graphics platform rather than Cycles' rendering-device list. The quoted `devices_get()` description alone does not draw that boundary.

**Yes — a direct list, for Cycles.** **Observed**, from Cycles' own files. There is a plain function:

```
def available_devices():
    import _cycles
    return _cycles.available_devices()
```

— `Blender Sources/cycles_5.2/engine.py`, lines 152–154, read 2026-07-30

and the preferences object has a fuller set. The shape of each entry is stated in a comment:

```
    # Gets all devices types to display in the preferences for a compute device type.
    # This includes the CPU device.
    def get_devices_for_type(self, compute_device_type, device_list=None):
        # Layout of the device tuples: (Name, Type, Persistent ID)
```

— `Blender Sources/cycles_5.2/properties.py`, lines 1741–1744, read 2026-07-30

though the code alongside it reads further positions than those three — `device[3]` and `device[4]` for shared memory and hardware ray tracing (lines 1969–1972), `device[5]` for denoiser support (line 1822), `device[6]` for the OptiX denoiser (line 1838) and `device[7]` for whether the card is running optimised (line 1753). The comment names three; the code uses eight.

Alongside those are `get_num_gpu_devices()` (lines 1784–1795), `has_active_device()` — "return self.get_num_gpu_devices() > 0" (lines 1809–1810) — and `has_multi_device()` (lines 1797–1807).

**Two cautions, both stated in the files, both easy to miss.** First, asking is not free of risk:

```
    # Be careful when deciding when to call this function,
    # as Blender can crash with `_cycles.available_devices()` on some drivers.
    def get_device_list(self, compute_device_type):
```

— `Blender Sources/cycles_5.2/properties.py`, lines 1707–1709, read 2026-07-30

Second, the obvious-looking way to ask no longer returns anything:

```
    # Deprecated: use refresh_devices instead.
    def get_devices(self, compute_device_type=''):
        self.refresh_devices()
        return None
```

— `Blender Sources/cycles_5.2/properties.py`, lines 1774–1777, read 2026-07-30

**A possible fourth way, at a distance — inferred.** The logging level `info` is documented as "Information about devices, files, configuration, operations" (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). This suggests that `--log-level info` with `--log-file` may put device information into a file. The sources do not state what it prints.

### Hand tests — observed

All of the following was done on **2026-08-01** on the Creator's MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, in Blender 5.2.

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
Maximum Vertex Image Units:	64
Maximum Fragment Image Units:	64
Maximum Pipeline Image Units:	64
Maximum Image Units:	64
```


**`gpu.platform`, from Blender's Python Console** (Scripting workspace, window open). `backend_type_get()` returned `'METAL'`; `device_type_get()` returned `'APPLE'`; `renderer_get()` returned `'Metal API'`; `vendor_get()` returned `'Apple M1 Pro'`; `version_get()` returned `'1.2'`. `gpu.platform.devices_get()` returned:

```
[<GPUDevice index=0 identifier="METAL" name="Metal API">]
```

— one entry, whose `name` is `Metal API` and whose `identifier` is `METAL`. Alongside it, Cycles' own list on the same machine and in the same session named the card as `Apple M1 Pro (GPU - 14 cores)` with the identifier `METAL_Apple M1 Pro (GPU - 14 cores)`, and the processor as `Apple M1 Pro` with the identifier `CPU`. `gpu.init()` was not called in this run.

**Cycles' own functions, from the same console.** The add-on key was found to be the literal string `cycles`. With `p = bpy.context.preferences.addons["cycles"].preferences`, `[(d.name, d.type, d.id, d.use) for d in p.devices]` returned, **before `refresh_devices()` was called**:

```
[('Apple M1 Pro', 'CPU', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', 'METAL_Apple M1 Pro (GPU - 14 cores)', True)]
```

and returned the same two entries after `p.refresh_devices()`. `p.has_active_device()` returned `True`, `p.get_num_gpu_devices()` returned `1`, `p.has_multi_device()` returned `False`. `_cycles.available_devices()` itself was not called in this run, so the shape of the raw entries is still unrecorded.

### Hand tests from a terminal — observed

Run on **2026-08-01** on the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2), against `/Applications/Blender.app/Contents/MacOS/Blender` (`Blender 5.2.0 LTS`).

**Save System Info with no window open.** `-b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='/tmp/sysinfo-terminal-52.txt'))"` printed `RESULT: {'FINISHED'}` and wrote the file. Its `GPU` section was one line:

```
GPU: missing, background mode
```

Its `Cycles` section, in full:

```
Cycles:
========================================


CPU device capabilities: 

Metal device capabilities:
Number of devices: 1
		Device: Apple M1 Pro (GPU - 14 cores)
```

**`gpu.init()` and the graphics-platform list with no window open.** `-b --factory-startup --python-expr "import gpu; print('INIT:', gpu.init())"` printed:

```
INIT: None
```

Calling the platform functions **without** `gpu.init()` first raised:

```
SystemError: GPU functions for drawing requires the gpu module to be initialized. See gpu.init.
```

Calling them after `gpu.init()` printed:

```
BACKEND: METAL
DEVICETYPE: APPLE
RENDERER: Metal API
VENDOR: Apple M1 Pro
VERSION: 1.2
DEVICES: [<GPUDevice index=0 identifier="METAL" name="Metal API">]
```

So `gpu.init()` does what the release notes say, and after it the whole `gpu.platform` module answers with no window open. The same commands in 4.5, 5.0 and 5.1 raise `AttributeError: module 'gpu' has no attribute 'init'` and `SystemError: GPU functions for drawing are not available in background mode`, so this is a difference in what 5.2 makes reachable, not a difference in what the machine has.

**The two lists side by side, in the same conditions.** `gpu.platform.devices_get()` returned one entry named `Metal API` with identifier `METAL`; `_cycles.available_devices('METAL')` returned two entries, `Apple M1 Pro` with identifier `CPU` and `Apple M1 Pro (GPU - 14 cores)` with identifier `METAL_Apple M1 Pro (GPU - 14 cores)`. This is one machine with one graphics card, and it does not settle what either list does on a machine with several.

**The raw Cycles entries** are eight fields long, quoted in full under question 5, and `_cycles.available_devices()` with no argument is refused with `TypeError: function takes exactly 1 argument (0 given)`.

### Hand tests on the rented Linux machine — observed

Run on **2026-08-06** on the rented Linux machine, against `/opt/blender/blender-5.2.0-linux-x64/blender`.

**Save System Info with no window open.** `-b --python-expr "import bpy; print('RESULT:', bpy.ops.wm.sysinfo(filepath='…'))"` printed `RESULT: {'FINISHED'}` and wrote the file. Its whole `GPU` section is one line, as on the Mac:

```
GPU: missing, background mode
```

Its `Cycles` section is where Linux differs, and it is long — about ninety lines. Its opening and the two lines that carry the answer:

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

So on an NVIDIA card the report names the card's compute capability outright, and the processor's instruction set (`AVX2`) where the Mac's line was empty. Only the technology whose cards were found gets a section: the technology was saved as OptiX and the section is still headed `CUDA device capabilities`. The headings, in order, were `Blender:`, `Python:`, `paths:`, `Python (External Binary):`, `Directories:`, `scripts:`, `pref scripts:`, `FFmpeg:`, `Other Libraries:`, `GPU: missing, background mode`, `Cycles:`, `Enabled add-ons:`.

**Asking Cycles' compiled part directly, including for technologies with no card — which the Mac could not do.** `-b --factory-startup --python-expr "import _cycles; …"` printed:

```
TYPES: (True, True, True, False, True, True)
CUDA: (('NVIDIA GeForce RTX 4090', 'CUDA', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07', False, False, True, False, True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True))
OPTIX: (('NVIDIA GeForce RTX 4090', 'OPTIX', 'CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX', False, False, True, True, True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True))
HIP: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
ONEAPI: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
METAL: (('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', 'CPU', False, False, True, False, True),)
```

So a technology with no supported card returns **the processor alone, and no error**, whether that technology is built in (HIP, oneAPI) or not (Metal). Each entry is an eight-field record, as on the Mac.

**What `available_devices()` does with no argument — answered.** It is refused:

```
TypeError: function takes exactly 1 argument (0 given)
```

the same on both machines and in every version.

**Whether any of this works before `refresh_devices()` has been called — answered.** With preferences already saved, the stored list came back already populated before the refresh; with `--factory-startup`, it came back empty and the refresh filled it. Both on both machines.

**The graphics module with no window open, and what `devices_get()` names.** `gpu.init()` returned `None`; without it the platform calls raised `SystemError: GPU functions for drawing requires the gpu module to be initialized. See gpu.init.` After it, everything answered:

```
BACKEND: OPENGL
DEVICETYPE: NVIDIA
RENDERER: NVIDIA GeForce RTX 4090/PCIe/SSE2
VENDOR: NVIDIA Corporation
VERSION: 4.6.0 NVIDIA 580.105.08
DEVICES: [<GPUDevice index=0 identifier="OPENGL" name="NVIDIA GeForce RTX 4090/PCIe/SSE2">]
```

That one entry is named for the drawing backend (`identifier="OPENGL"`), where Cycles named the same card `CUDA_NVIDIA GeForce RTX 4090_0000:00:07` and `…_OptiX`. So the two lists are of different things about the same hardware, which is as far as one card can take the question.

**A fourth route this version has and no other does.** `--gpu-device help` printed a list of the cards the drawing side can use, from a terminal, with no window and no display reachable:

```
Blender GPU Device Listing (Vulkan):
Pass an Index or Device-ID to '--gpu-device'.
  Index  Device-ID    Name
  0      10de/2684/0  NVIDIA GeForce RTX 4090
  1      10de/2684/1  NVIDIA GeForce RTX 4090
  2      10005/0/2    llvmpipe (LLVM 15.0.7, 256 bits)
```

Note that it lists three entries — the one card twice and a software renderer — where `gpu.platform.devices_get()` returned one. No version's findings mention this argument.

**What Blender was built with, against what it can find.** Hiding the card for one command with `CUDA_VISIBLE_DEVICES=` left `TYPES:` unchanged while both card lists came back holding the processor alone.

**Not covered.** The sources do not give the layout or wording of `system-info.txt`, so a program reading it would be reading an undocumented format — though that format is now recorded above for both machines. Whether `gpu.platform.devices_get()` lists cards Cycles could render on or only cards the drawing backend enumerates is now partly answered: on one card the two routes gave different names for the same hardware and different counts against `--gpu-device help`, and a machine with several cards would settle it properly. Whether the stored identifier survives a driver update or a card being moved is untested on both machines.

**Outside the interface.** The sources document Python entry points for the system report and graphics-platform list, but do not state that the system report succeeds in background mode. In background mode, the release notes say `gpu.init()` was added to initialize the graphics module (https://developer.blender.org/docs/release_notes/5.2/python_api/, read 2026-07-30), and the API says it raises `SystemError` if initialization fails. The Cycles list is reached through Python in the supplied files, with the crash caution above. The hand tests now settle all of this on one machine. `bpy.ops.wm.sysinfo(filepath=…)` returned `{'FINISHED'}` and wrote its file with no window, though that file's `GPU` section read only `GPU: missing, background mode` while its `Cycles` section was fully populated. `gpu.init()` returned `None` with no window, and after it the whole `gpu.platform` module answered, `devices_get()` included; without it the calls raise `SystemError: GPU functions for drawing requires the gpu module to be initialized. See gpu.init.` The Cycles list answered with no window, and the stored `devices` collection was already populated before `refresh_devices()` was called, as it was inside the window. The logging route is still only inferred.

**Operating system.** The sources place no operating-system limit on the report or Python entry points, but do not directly say that every route works on all three systems. Two systems are now tested and every route worked on both. Results differ, as expected: on the Creator's Mac `backend_type_get()` returned `'METAL'` and `device_type_get()` returned `'APPLE'`; on the rented Linux machine, after `gpu.init()`, they returned `'OPENGL'` and `'NVIDIA'`. The Cycles list is limited to the technologies available on that system (question 2) — `(False, False, False, True, False, False)` on the Mac against `(True, True, True, False, True, True)` on Linux. The manual notes that macOS driver information changes with the operating system rather than independently. One route exists only on Linux and Windows: `--gpu-device help`, which lists the drawing side's cards from a terminal. Windows remains untested.



### Hand tests on the two-card machine — observed

Run on **2026-08-06** on the second rented Linux machine described under question 5 — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s on driver 580.95.05, sixty-one cores, **no display server at all**, over SSH — against `/opt/blender/blender-5.2.0-linux-x64/blender`. Label: **observed**. Not merged with the one-card rented machine's results or the Mac's.

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

**The system-information file lists the card twice.** `bpy.ops.wm.sysinfo(filepath=…)` returned `{'FINISHED'}` and wrote its file with no display server. Its `Cycles` section carries `CPU device capabilities: AVX2`, one heading `CUDA device capabilities:`, and then the card's name twice — once at line 83 of the file and once at line 172 — each followed by its own block of capabilities.

**And 5.2's own `--gpu-device help` lists both cards, with no window and no display server at all:**

```
Blender GPU Device Listing (Vulkan):
Pass an Index or Device-ID to '--gpu-device'.
  Index  Device-ID    Name
  0      10de/2684/0  NVIDIA GeForce RTX 4090
  1      10de/2684/1  NVIDIA GeForce RTX 4090
```

This is the several-card form of a thing only this version has, and it is the only route on this machine that gives each card a plain index.

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

**Not covered.** What any of this returns on a machine with cards of two different makes, or with an NVLink pair. What the Status Bar or any panel shows, this machine having no display server at all.

**Outside the interface.** Settled again, on a machine with two cards and no display server: `wm.sysinfo` wrote its file, the Cycles queries answered, and both cards were named by every route. Nothing needed a window.

**Operating system.** Linux, as before. This second machine agrees with the one-card machine wherever the two could be asked the same thing, and adds only what a second card shows. Windows is untested.
---

## Things a person could settle by hand in a few minutes

This list has since been run on three machines. What the Creator's own MacBook Pro settled is written into the answers above as **Hand test — observed**, what the one-card rented Linux machine settled as **Hand tests on the rented Linux machine — observed**, and what a second rented Linux machine with two graphics cards and no display server settled as **Hand tests on the two-card machine — observed**. The three lists live at `Audits/Blender Research/The machine/Hand tests.md`, `Audits/Blender Research/The machine/Hand tests — Linux.md` and `Audits/Blender Research/The machine/Hand tests — two cards and no screen.md`. **A fourth machine has since been used as well** — a third rented Linux machine with **four** graphics cards and no display server — and what it settled is written into the answers above as **Hand tests on the four-card machine — observed**. Its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

### Still open

Nothing on this list is still open. All four items were run on both machines and what came back is recorded under questions 1, 2, 3, 4, 5 and 6 above. Item 2 was answered only as far as the Mac reached — the technology set to `NONE`, and every device unticked. The rented Linux machine reached the real case: with a technology chosen for which it has no card at all, the render still completed on the processor without a word, and the missing-requirement text was found only in the preferences panel. Two more items are closed there as well: Blender does start and render on Linux with no display at all, and the Intel **Linux** figure a running 5.2 draws is `XX.XX.37435.3`.

### Cannot be run on the equipment available

The equipment is one MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, whose graphics are Apple's own and built in; one rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores — reached over SSH; and a second rented Linux machine — Ubuntu 22.04.5 LTS, **two** NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, **no display server at all** — also over SSH. No Windows machine has run it, there is no AMD or Intel graphics card and no pair of cards joined by NVLink, and nothing on either rented machine may be installed or removed. **Two corrections to that sentence, made 2026-08-06 by the citation check of the two-card material.** The one-card rented Linux machine was destroyed on 2026-08-06 and nothing can ever be run on it again, so every result from it stands as a record and none of it can be repeated. And something *was* removed and reinstalled on the two-card machine: its NVIDIA driver was purged on purpose, and then reinstalled into a mismatched state, for the broken-driver test recorded under question 2. That machine has since been restarted and did not answer when the citation check tried it on 2026-08-06.

**A fourth machine has since been used, and it is the only one still alive.** A **third rented Linux machine** — Ubuntu 22.04.5 LTS, **four** NVIDIA GeForce RTX 4060 Ti of **16 GB each** on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, **197 GB of memory**, **no display server**, over SSH. What it settled is written into the answers above as **Hand tests on the four-card machine — observed**, and its list is `Audits/Blender Research/The machine/Hand tests — four cards.md`. Three limits above are narrowed by it and none is removed: more than two cards is no longer out of reach, a card small enough to be filled is no longer out of reach, and the several-card form of every question under 5 and 6 has now been asked twice on two different machines. **Still out of reach on it as well:** any pair joined by NVLink, these four talking through the host bridge; cards of two different makes; cards of unequal speed, all four being the same model within about 2 % of each other; and anything needing eyes on a window, this machine having no display server either. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

- **Cards of two different makes ticked at once.** The two-card machine has two of the same NVIDIA model. Most of what needed a second card is now answered under questions 5 and 6; this is not.
- **What Distribute Memory Across Devices does when the cards are joined by NVLink.** The two-card machine's pair talks through the host bridge. What the setting does on an unlinked pair is now recorded under question 5.
- **Whether a slow card holds a fast one back.** Both cards on the two-card machine are the same model and gave the same time as each other.
- **Anything needing eyes on a window with two cards** — whether the Distribute Memory Across Devices box is drawn or greyed, and what the Vulkan **Device** control offers. The machine that has two cards has no display server; the machine that had a display had one card.
- **Whether a Persistent ID is stable.** Needs a driver update independent of the operating system, or a card that can be moved to a different slot. Neither machine allows it. What is now known is that on Linux the identifier visibly carries the card's place on the bus.
- **What Blender says when it will not start for want of a good enough card.** Needs a machine with a card below the graphics requirement, or a virtual machine without graphics passthrough. What Blender says when there is no display server is now settled under question 1, and that is a different case.
- **Which of the two Intel driver numbers for Windows is right.** Needs a Windows machine with an Intel Arc card; the panel text branches on the operating system, so a Linux machine can never draw the Windows lines. The Metal half of that item needs an Apple silicon Mac running macOS 12.x: on macOS 26.5.2 a Metal device is found and no requirement line is drawn, and Linux offers no Metal button at all.
- **Whether the figures the panel draws for Intel and AMD are enforced.** Needs a real Intel or AMD card with a driver below the figure.

---

2026-08-01 — Claude Opus 5 (Claude Code) — The machine, Blender 5.2

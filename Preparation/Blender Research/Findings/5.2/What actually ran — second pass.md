# What actually ran — Blender 5.2 — second pass

**Sources consulted, and when.**
Blender 5.2 manual (`https://docs.blender.org/manual/en/5.2/`), Blender 5.2 Python API documentation (`https://docs.blender.org/api/5.2/`), Blender 5.2 release notes (`https://developer.blender.org/docs/release_notes/5.2/`) and Cycles' own files as shipped with Blender 5.2 — all four read as snapshots taken **2026-07-30**. Blender's requirements page (`https://www.blender.org/download/requirements/`), read as a snapshot taken **2026-08-01**.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

A note on the requirements page before anything else: it never names 5.2. The closest it comes is "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." Because it does not name 5.2 in so many words, I have not used it as an authority for 5.2 anywhere below, and I say where that leaves a gap.

A note on Cycles' files: what is on disk here is the Cycles add-on's Python (`__init__.py`, `engine.py`, `properties.py`, `ui.py`, `operators.py`, and so on) plus `source/kernel` and `source/util`, which are the GPU/CPU kernel and maths headers. The host-side C++ that actually picks a device and prints about it is **not** in this folder. I checked: the only `printf` calls under `cycles_5.2/source` are debug-print helpers in `util/types_*.h`, `util/transform.h` and `util/projection.h`. So nothing here shows what a running render writes out.

---

## 1. When a render has finished, what can be learned about which parts of the computer it used?

**LABEL: documented** (for what the finished image can carry), and **not in the sources** (for the identity of the render device).

Two different things have to be separated here, and the sources treat them very differently.

**(a) What the finished image itself can carry.** Blender can write a fixed list of fields into the image file. The whole list is in the Metadata panel:

> "The Metadata panel includes options for writing metadata into render output."

and the fields, in full:

> "Date … Time … Render Time … Frame … Frame Range … Memory … Hostname … Camera … Lens … Scene … Marker … Filename … Strip Name … Note"

Of those, three say something about the machine and the work it did:

> "Render Time — Includes the render time."
> "Memory — Includes the peak memory usage."
> "Hostname — Includes the rendering machine's hostname."

Source: Blender manual, `https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html`. Date read 2026-07-30.

I also opened the picture of that panel, `_images/render_output_properties_metadata_panel.png`, referenced from that page as `[image: _images/render_output_properties_metadata_panel.png]`. It shows the checkboxes: Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Use Strip Me…, Note, Burn Into Image (Font Size, Text Color, Background, Include Labels). **Documented**, from the manual's own image: there is no device field, no processor field, no graphics-card field in that panel. In the default state shown, Render Time and Memory are ticked and Hostname is not.

The same three fields appear in the Python API with their defaults:

> "use_stamp_render_time — Include the render time in image metadata (default True)"
> "use_stamp_memory — Include the peak memory usage in image metadata (default True)"
> "use_stamp_hostname — Include the hostname of the machine that rendered the frame (default False)"

Source: Blender Python API, `https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html`. Date read 2026-07-30.

I counted the stamp switches in that page: there are sixteen (`use_stamp` plus fifteen `use_stamp_*`). None of them names a render device, a processor, a graphics card, or a driver.

**(b) The identity of the device that did the work.** The sources do not provide a way to learn this from a finished render. What they do provide is a way to learn what was *configured*, and one way to learn what was *available*.

Configured, and readable through Python at any time:

> "threads — Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) (in [1, 1024], default 1)"
> "threads_mode — Determine the amount of render threads used (default 'AUTO')"

Source: `https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html`. Date read 2026-07-30.

For Cycles, the choice is a scene setting plus a preference. **Observed**, in Cycles' own files:

> `enum_devices = (`
> `    ('CPU', "CPU", "Use CPU for rendering"),`
> `    ('GPU', "GPU Compute",`
> `        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),`
> `)`

File `cycles_5.2/properties.py`, lines 30–34.

> `    device: EnumProperty(`
> `        name="Device",`
> `        description="Device to use for rendering",`
> `        items=enum_devices,`

File `cycles_5.2/properties.py`, lines 387–390.

And the preference side, which holds the list of individual cards and whether each is ticked. **Observed**:

> `class CyclesDeviceSettings(bpy.types.PropertyGroup):`
> …
> `    id: StringProperty(name="ID", description="Unique identifier of the device")`
> `    name: StringProperty(name="Name", description="Name of the device")`
> `    use: BoolProperty(name="Use", description="Use device for rendering", default=True)`
> `    type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')`

File `cycles_5.2/properties.py`, lines 1614–1622.

> `    compute_device_type: EnumProperty(`
> `        name="Compute Device Type",`
> `        description="Device to use for computation (rendering with Cycles)",`

File `cycles_5.2/properties.py`, lines 1655–1657.

> `    def has_active_device(self):`
> `        return self.get_num_gpu_devices() > 0`

File `cycles_5.2/properties.py`, lines 1809–1810. Date read 2026-07-30 for all of these.

**Inferred**, and I write the step out: because `compute_device_type` and the `devices` collection are add-on preferences on `CyclesPreferences` (whose `bl_idname` is the Cycles package), and because `cycles_5.2/ui.py` reads them as `context.preferences.addons[__package__].preferences.compute_device_type` (file `cycles_5.2/ui.py`, lines 96–97) and `…preferences.has_active_device()` (lines 100–101), a script can read the same values the same way, at any time, including after a render. The step a reader has to supply is that a preference readable by the add-on's own drawing code is equally readable by any other script — the sources do not state that in words. What this gets you is still the *configuration*, not a record of what the finished render used.

Available, rather than used, and written to a file: the System Info text.

> "Save System Info — This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems."
> "You will be prompted to save a text file called system-info.txt."
> "GPU — Shows the GPU vendor, version and the capabilities of your hardware and driver."
> "Cycles — The instruction sets and capabilities of each hardware render device available for use with Cycles."

Source: Blender manual, `https://docs.blender.org/manual/en/5.2/getting_started/help.html`. Date read 2026-07-30. Note the wording: "available for use", not "used".

The matching Python call:

> "bpy.ops.wm.sysinfo(*, filepath='') — Generate system information, saved into a text file"

Source: `https://docs.blender.org/api/5.2/bpy.ops.wm.html`. Date read 2026-07-30.

One more thing that touches "which parts of the computer", from the log switches:

> "--log-level <level> — Set the logging verbosity level."
> "info: Information about devices, files, configuration, operations"

Source: Blender manual, `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`. Date read 2026-07-30. The manual says information-level logging covers devices. It does not say what those messages contain, or that any of them name the device a render used.

And a Cycles-only measurement that is itself device-dependent. **Documented**:

> "Render Time Cycles — An estimate for how long in milliseconds each pixel took to render. Note, this pass is not supported on GPU rendering."

Source: `https://docs.blender.org/manual/en/5.2/render/layers/passes.html`. Date read 2026-07-30. **Observed**, matching it in Cycles' files:

> `    pass_render_time: BoolProperty(`
> `        name="Render Time",`
> `        description="Reports time per pixel in milliseconds. Supported only on CPU render devices",`

File `cycles_5.2/properties.py`, lines 1533–1535, and

> `    if crl.pass_render_time:`
> `        # Only yield the pass if rendering on CPU`
> `        if scene.cycles.device == 'CPU':`
> `            yield (n_("Render Time"), "X", "VALUE")`

File `cycles_5.2/engine.py`, lines 223–226. **Inferred**: the presence or absence of a Render Time pass in a finished multi-layer file therefore tells you something about the device setting — the pass is only registered when the scene's Cycles device is CPU. The step: I am reading "the pass is only registered under this condition" as "if the pass is in the file, the condition held". A reader has to supply the assumption that nothing else adds that pass.

**Not covered.** Nothing here settles what the finished image, or any file Blender leaves behind, records about the actual processor, graphics card, driver, or core count used for that render. The metadata list gives you time, peak memory and hostname, and nothing more about the machine. `system-info.txt` is a snapshot of the machine, written when you ask for it, not a record attached to a render.

**Outside the interface.** Mostly yes, and in one place better. The metadata fields are render settings, written into the file, so they do not depend on a window being open. `--log-level info`, `--cycles-print-stats` and `--cycles-device` are command-line switches and only exist when Blender is started from a command line — the manual introduces them under "Cycles Render Options" and "Logging Options" in `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`. `system-info.txt` is documented as a menu item ("Save System Info") with a Python operator behind it that takes a `filepath`; the sources do not say whether that operator works with no interface running. The Cycles preference reads are Python, so they do not need a window, but the sources do not say so. **Settled by test on 2026-08-06, on Linux only — see below.** `bpy.ops.wm.sysinfo` does work with no window, returning `{'FINISHED'}` and writing its file, so the inferred step above holds; but its GPU section then reads `GPU: missing, background mode`, so the route works and the hardware part of what it produces does not. The `gpu.platform` calls do **not** work with no window — they raise `SystemError` in every version. And the line naming the processor that did the work is reachable with no window, on standard output. **Added by the tests of 2026-08-07, with the machine's graphics card repaired:** driving Blender from outside its window is no obstacle to naming the card. With no window and no display server, a run prints the card's model in its `Path tracing on:` line, `bpy.ops.wm.sysinfo` describes the card in full in its Cycles section, and the Cycles preferences list it before any render. The GPU section of `system-info.txt` still reads `GPU: missing, background mode`, so that one section is genuinely lost outside the window and nothing else is. **Added by the re-run of 2026-08-07, after the citation check:** nothing in these three tests needs a window, and all three were run on a machine that has no display server at all.

**Operating system.** The metadata fields are described without any operating-system qualification. The device *types* differ by system — "CUDA is supported on Windows and Linux", "HIP is supported on Windows and Linux", "oneAPI … is supported on Windows and Linux", "Metal for GPU acceleration is supported on Apple computers with Apple Silicon" (`https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html`, date read 2026-07-30) — so what there is to learn differs, even if the way to learn it does not. Cycles' own files also change their default by system. **Observed**: **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not. **Added by the re-run of 2026-08-07, after the citation check:** the three tests sent back by the citation check were run on that same Linux machine and nowhere else, so they are an NVIDIA-on-Linux answer; macOS and Windows are untouched by them.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.2`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**One thing about this machine bears on every device result here.** Its graphics card could not be reached while the tests ran — the card's kernel module (580.95.05) and the libraries that talk to it (580.173.02) no longer match. `nvidia-smi` answered `Failed to initialize NVML: Driver/library version mismatch`, and Blender's own view was `cycles | ERROR CUDA cuInit: Unknown error` with an empty device list. So every result below is from a machine that has a graphics card and cannot use it.

**Blender 5.2 prints which processor did the work, by name, and `--log-level info` on its own is enough to bring it out.** The scene was the factory cube at 160 by 120, 16 samples, Cycles. Run as `blender -b scene.blend --log "*" --log-level info -o out/li_ -F PNG -f 1`, standard output carried:

```
00:00.529  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: AMD EPYC 7B12 64-Core Processor (CPU) (24 threads) [CPU]
                            |                      Hardware Ray-Tracing: Off
                            | 
                            | Render Scheduler Summary
                            | 
                            | Mode: Headless
                            | Resolution: 160x120
```

That names the processor's model, the number of threads and the kind of device. Counted per switch and per stream, the line appears on **standard output** and never on the error stream:

```
  [--log cycles --log-level info] -> Path tracing line on stdout: 1, on stderr: 0
  [--log-level info]              -> Path tracing line on stdout: 1, on stderr: 0
  [--debug-cycles]                -> stdout: 1, stderr: 0
```

The same run also prints `cycles | Using AVX2 CPU kernels.` twice, near the start.

**A background render with no switches at all names no device.** Run as `blender -b scene.blend -o out/p_ -F PNG -f 1`, exit 0, frame written, error stream 0 bytes. The lines that carry the answer:

```
00:00.303  render           | Rendering single frame
00:00.303  render           | Rendering frame 1
00:00.304  render           | Start rendering: Scene, ViewLayer
00:00.304  render           | Engine: Cycles
00:00.311  render           | Fra: 1 | Mem: 0M | Loading render kernels (may take a few minutes the first time)
00:00.313  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.327  render           | Fra: 1 | Mem: 1M | Finished
00:00.440  render           | Saved: '/root/what-actually-ran/out/p_0001.png'
00:00.440  render           | Time: 00:00.13 (Saving: 00:00.11)
```

The words `Copying Transformations to device`, `Copying BVH to device` and `Updating Device | Writing constant memory` appear in the fuller output; they carry the word *device* without naming one.

**No field written into a finished image names a device.** With every `use_stamp_*` switched on and the note set to `SMOOZY NOTE`, a Cycles render to PNG carried exactly these fields, read straight out of the file's text chunks:

```
   File = /root/what-actually-ran/work/scene.blend
   Note = SMOOZY NOTE
   Date = 2026/08/06 22:59:30
   Marker = <none>
   Time = 00:00:00:01
   Frame = 1
   FrameRange = 1:3
   Camera = Camera
   Lens = 50.00
   Scene = Scene
   Strip = <none>
   RenderTime = 00:00.03
   Memory = 1M
   Hostname = ubuntu
   cycles.ViewLayer.samples = 16
   cycles.ViewLayer.total_time = 00:00.03
   cycles.ViewLayer.render_time = 00:00.05
   cycles.ViewLayer.synchronization_time = 00:00.01
```

The Memory field read `1M` — a plain figure, **not** the literal `ddM` a corrective-release note describes. The installed build is 5.0.1 or later in every case, so this says nothing about 5.0.0.

**The stamp switches at factory startup.** Run as `blender -b --factory-startup --python t_stamp.py`, all sixteen:

```
STAMP use_stamp False
STAMP use_stamp_camera True
STAMP use_stamp_date True
STAMP use_stamp_filename True
STAMP use_stamp_frame True
STAMP use_stamp_frame_range False
STAMP use_stamp_hostname False
STAMP use_stamp_labels True
STAMP use_stamp_lens False
STAMP use_stamp_marker False
STAMP use_stamp_memory False
STAMP use_stamp_note False
STAMP use_stamp_render_time True
STAMP use_stamp_scene True
STAMP use_stamp_sequencer_strip False
STAMP use_stamp_time True
```

**`use_stamp_memory` is False and `use_stamp_hostname` is False**, matching the manual's picture of the panel rather than the API page's stated defaults. The master switch `use_stamp` is off, so none of the rest is written unless somebody turns it on. The same run printed `THREADS 24 THREADS_MODE AUTO` — the machine's core count, not the documented default of 1.

**The settings read after a run do not say what ran.** A run whose scene was set to GPU Compute and which was overridden on the command line with `-- --cycles-device CPU` printed, from inside the same run:

```
AFTER_OVERRIDE compute_device_type= NONE  scene.cycles.device= GPU
```

while the same run's `Path tracing on:` line named the processor.

**`bpy.ops.wm.sysinfo` works with no window, but its GPU section is not filled in.** It returned `{'FINISHED'}`, exit 0, and wrote a 94-line file whose GPU and Cycles sections read:

```
GPU: missing, background mode

Cycles:
========================================


CPU device capabilities: AVX2

CUDA device capabilities:
Error initializing CUDA: Unknown error
```

**The `gpu` module answers nothing with no window.** Run as `blender -b --python-expr "import gpu; ..."`:

```
GPU_ERROR SystemError GPU functions for drawing requires the gpu module to be initialized. See gpu.init.
HAS_GPU_INIT True
```


> `        # Default to selecting the Metal compute device on Apple Silicon GPUs`
> `        # (drivers are tightly integrated with macOS so pose no stability risk)`
> `        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):`
> `            return 5`
> `        return 0`

File `cycles_5.2/properties.py`, lines 1631–1635.

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.2`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**The `Path tracing on:` line names the graphics card, its model and the route taken to it.** The same scene as before — the factory cube at 160 by 120, 16 samples, Cycles — with the Cycles preference set to OptiX, the card's OptiX device switched on and the scene set to GPU Compute. Run as `blender -b scene.blend --python setcard.py --log-level info -o out/l_ -F PNG -f 1`, **standard output** carried:

```
Path tracing on: NVIDIA GeForce RTX 4080 SUPER (OptiX) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07_OptiX]
                     Hardware Ray-Tracing: Off
```

and with the preference set to CUDA instead, the same line read:

```
Path tracing on: NVIDIA GeForce RTX 4080 SUPER (CUDA) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07]
                     Hardware Ray-Tracing: Off
```

So the line names the card's model, which of the two NVIDIA routes was taken, and an identifier holding the card's place on the bus. It is the same line, in the same place, brought out by the same switch as when it named the processor on 2026-08-06 — `--log-level info` on its own is still enough.

**Where in the run it arrives.** The line is part of a closing report: it is printed after the last sample and before the frame is written. In the 5.2 run the last sample line was at `00:02.270` and the report at `00:02.270`, with `Saved:` at `00:02.577`. So it answers *what did the work* once the work is done, not before it starts.

**A background render with no switches at all still names no card — but OptiX leaves its name behind.** Run with no log switches: exit 0, a frame written, error stream 0 bytes, and no line naming the card. What OptiX does put into the ordinary progress lines is its own name:

```
00:02.238  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building OptiX acceleration structure
```

Lines of that shape appear more than once in the OptiX run and **not once** in the CUDA run of the same scene. So at default settings the printed output can tell you OptiX was in use and cannot tell you CUDA was.

*Changed by the citation check, 2026-08-07.* This used to say "three times". A repeat on the same machine gave two in this version, and across the four versions the counts I read were four, four, three and two, so the number is not stable and has been taken out. The contrast the claim rests on held in every run either of us made.

**The memory figure jumps when the card does the work, and that needs no switch either.** The same frame, the same scene, the figure Blender prints in its progress lines and writes into the `Memory` stamp field:

| what did the work | the figure |
|---|---|
| the processor | `1M` |
| the card | `661M` |

The card's figure is the memory the card is holding, and it is hundreds of times the processor's for a scene of one cube. It names nothing, but it is a strong hint, and it is on disk as well as on screen.

**`bpy.ops.wm.sysinfo` now names the card, in its Cycles section only.** The same call, with no window: exit 0, `{'FINISHED'}`, and a file grown from 94 lines to 183. The GPU section is unchanged and still reads `GPU: missing, background mode`. The Cycles section, which on 2026-08-06 read `Error initializing CUDA: Unknown error`, now reads:

```
CPU device capabilities: AVX2

CUDA device capabilities:
	NVIDIA GeForce RTX 4080 SUPER
		CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK			1024
		CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT			80
		CU_DEVICE_ATTRIBUTE_CLOCK_RATE			2580000
		CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR			8
		CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR			9
		CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD			0
```

(Sixty more `CU_DEVICE_ATTRIBUTE_` lines sit between those, cut here for length.) The heading is `CUDA device capabilities:` in every version and there is **no OptiX heading** — the file describes the card once, by its CUDA identity, whichever backend a render would use. All ninety added lines are in this section; every other section is unchanged.

**The card can be listed before anything is rendered.** `preferences.addons['cycles'].preferences.devices`, read after `refresh_devices()` with `--factory-startup` and no scene, comes back identical in all four versions:

```
[('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False), ('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True)]
```

The card appears twice, once per backend, each switched on; the processor appears with its switch off. But `compute_device_type` still reads `NONE` at factory startup with the card sitting there working — so that setting says nothing at all about whether a card is present.

**`gpu.init()` makes the `gpu` module answer, with no window and no display server.** On 2026-08-06 this could not be tried, there being no working graphics library for it to reach. Run now as `blender -b --python-expr "import gpu; gpu.init(); ..."`, exit 0:

```
HAS_GPU_INIT True
GPU_INIT_OK
GPU_RENDERER NVIDIA GeForce RTX 4080 SUPER/PCIe/SSE2
GPU_DEVICE_TYPE NVIDIA
GPU_BACKEND OPENGL
GPU_VENDOR NVIDIA Corporation
```

So in 5.2 there is a second route to the card's name from a run with no window, and it names the vendor and the drawing backend as well. It is a route to the *machine's* card, not to what a render used: it answers the same whichever device Cycles was pointed at.

**The settings still do not say what ran, and can now be shown to mislead.** A run with the Cycles preference set to OptiX and the scene set to GPU Compute, overridden on the command line with `-- --cycles-device CPU`, printed from inside itself:

```
AFTER_OVERRIDE compute_device_type= OPTIX  scene.cycles.device= GPU
```

while the same run's `Path tracing on:` line named the processor. Both settings name the card's route; the processor did the work.

**A timing worth having, now that nothing else is running on this machine.** The same frame — 960 by 540, 1024 samples, adaptive sampling off — rendered twice, once with `-- --cycles-device OPTIX` and once with `-- --cycles-device CPU`:

| what did the work | wall clock | the `Time:` line Blender printed |
|---|---|---|
| the card, OptiX | 3.05 s | `Time: 00:01.63 (Saving: 00:00.23)` |
| the processor | 16.28 s | `Time: 00:15.48 (Saving: 00:00.22)` |

**This is one machine's timing, on one scene, on one day**, and it is offered as nothing more. What it does show is that the two are nowhere near each other: the card finished this frame about nine times sooner, which is far outside anything the spread between repeats could explain. The card's figure includes loading the render kernels, so it is if anything the slower of the two readings.

**Where I looked.** Searched the whole 5.2 manual and the whole 5.2 API text for: `stamp`, `metadata`, `hostname`, `peak memory`, `device used`, `which device`, `compute device`, `render device`, `rendered on`, `GPU used`, `sysinfo`, `system_info`, `System Info`, `statistics`, `render statistics`. Opened and read in full: the Metadata page and its panel picture; the Help menu page; the GPU Rendering page; the Cycles Render Settings index; the Cycles Performance page; the Preferences System page; the Command Line Arguments page; the Status Bar page. Read the `RenderSettings` API page's stamp block and thread block; read `bpy.app`'s member list and the `build_platform` / `build_options` / `version` entries. Read `cycles_5.2/properties.py` (device enums, `CyclesDeviceSettings`, `CyclesPreferences` in full), `cycles_5.2/engine.py` in full, `cycles_5.2/ui.py` device helpers. Ruled out `cycles_5.2/source` after grepping it for print and log calls and finding only maths debug helpers. Ruled out the requirements page because it does not name 5.2.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

**The machine.** The Creator's own Mac — a MacBook Pro with an **M1 Pro** chip, **macOS 26.5.2** (build 25F84), Apple's own graphics and no second card. Blender was `/Applications/Blender.app/Contents/MacOS/Blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)` — the same build hash as the Linux machine's 5.2, built at a different hour. Every run was pointed at a settings folder made before it was used (`BLENDER_USER_RESOURCES=/tmp/smoozy-war/settings/5.2`, and `/tmp/smoozy-war/settings2/5.2` where a run needed preferences that had been saved once) and never at the real one. Working files sat in `/tmp/smoozy-war/`; the prepared scene — the factory cube at 160 by 120, 16 samples, Cycles, denoising off — sat at `/Users/artist/Documents/Smoozy hand tests/What actually ran/scene.blend` and was built by Blender 4.5 so all four versions could open it. **Two other topics were running Blender on this Mac while these tests ran**, so nothing below is offered as a timing or a memory measurement. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**A background render with no switches names no device on macOS either.** `blender -b scene.blend -o out/p_ -F PNG -f 1`: exit 0, a frame written, **the error stream 0 bytes**, and no line naming a processor, a card, Metal, CUDA, OptiX, HIP or oneAPI. Line for line the same shape as the Linux run of 2026-08-06.

**`--log-level info` on its own still brings the device line out, and on macOS it names Apple's chip.** Same scene, `--log-level info`, on **standard output**:

```
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
                            |                      Hardware Ray-Tracing: Off
```

Counted per switch, the same as on Linux: `--log cycles --log-level info`, `--log-level info` and `--debug-cycles` each put the line on standard output once and on the error stream not at all.

**The line names the card when the card does the work.** With the Metal device switched on and the scene set to GPU Compute, the same line on standard output read:

```
Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                     Hardware Ray-Tracing: Off
```

So it names the chip, the number of graphics cores, the route (Metal), the word `display` — this machine's graphics drive the screen as well — and an identifier built from the two.

**One line in the same run must not be read as saying what ran.** At `--log-level info` this appears whether the work went to the processor or to the graphics:

```
cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
```

It was present, word for word, in a run whose `Path tracing on:` line named the processor. **`Added device` names what Cycles found; `Path tracing on:` names what did the work.**

**`--cycles-print-stats` still names no device.** Exit 0, the whole statistics block on **standard output**, the error stream 0 bytes, and searching the captured output for `Metal`, `CUDA`, `OptiX`, `HIP`, `oneAPI`, `GPU`, `CPU`, `Apple` and `M1` returned nothing.

**And with the graphics doing the work it refuses.** The same switch with `-- --cycles-device METAL`: exit 0, the block still on standard output, the error stream 0 bytes, and in place of the timings, in all four versions, the one sentence:

```
Profiling information not available (only works with CPU rendering)
```

— word for word what the mended Linux card produced. Nothing in it names a device.

**A render on the graphics writes no new field, but the `Memory` field jumps.** Every metadata switch on, the same one-cube scene, `-- --cycles-device METAL`: the field set is identical to the processor run's and **no field names the graphics, the backend or the technology** — every value searched for `metal`, `apple`, `m1`, `nvidia`, `geforce`, `cuda`, `optix`, `hip`, `oneapi` and `device`, zero matches, in all four versions. What changes is one number:

| what did the work | the `Memory` field in this version |
|---|---|
| the processor | `1M` |
| the graphics | `1409M` |

So the figure is a strong hint and names nothing — the same shape of hint the Linux card gave. **It is a memory reading taken on a shared machine**, so it is offered as an order of magnitude and not as a measurement.

**The settings still do not say what ran, and on this machine they mislead in the other direction.** A run with the scene set to GPU Compute and `-- --cycles-device CPU` on the command line printed from inside itself:

```
AFTER_OVERRIDE compute_device_type= METAL  scene.cycles.device= GPU
```

while the same run path-traced on the processor. Both settings name the graphics; the processor did the work.

**What a written image carries.** Every metadata switch on, Cycles, PNG and OpenEXR: **no field names a device, a card or a technology**, in any of the four versions. The `Hostname` field carried the machine's name, `Artists-Studio-MacBook.local`. The `Memory` field read `1M` for the processor run — nothing resembling `ddM` appeared.

**`bpy.ops.wm.sysinfo` with no window: the GPU section is empty and the Cycles section names the card.** Exit 0, `{'FINISHED'}`, a 96-line file. The GPU section reads `GPU: missing, background mode`, exactly as on Linux. The Cycles section, on this machine:

```
CPU device capabilities: 

Metal device capabilities:
Number of devices: 1
		Device: Apple M1 Pro (GPU - 14 cores)
```

Note that `CPU device capabilities:` is **blank** here where the Linux file carried `AVX2` — that line reports an instruction set this chip has no entry for.

**With a window open the GPU section is filled in.** The same operator, called from a script in a Blender started with a window, wrote a 120-line file. Comparing the two files line by line, the only differences are the temporary folder's name and this, where the windowless file had one line:

```
GPU:
========================================

renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:

Implementation Dependent GPU Limits:
========================================

Maximum Batch Vertices:	-2147483648
Maximum Batch Indices:	-2147483648
```

**In 5.2 `gpu.init()` opens a second route to the card's name with no window at all.** Run as `blender -b --python-expr "import gpu; gpu.init(); ..."`, exit 0:

```
GPU_INIT_RETURNED None
GPU_RENDERER Metal API
GPU_DEVICE_TYPE APPLE
GPU_BACKEND METAL
GPU_VENDOR Apple M1 Pro
```

Without the call first, the same run gives `GPU_ERROR SystemError GPU functions for drawing requires the gpu module to be initialized. See gpu.init.` This is a route to the *machine's* graphics, not to what a render used.

**The Metadata panel, seen on screen.** Blender started with a window on the factory scene, the Output Properties tab shown and its own search box set to `Metadata` so the panel opened, then Blender's own `screen.screenshot` operator. The panel showed `Metadata Input: Scene` and thirteen boxes: **Date, Time, Render Time, Frame, Camera, Scene and Filename ticked; Frame Range, Memory, Hostname, Lens, Marker and Strip Name unticked**, with `Note` and `Burn Into Image` as closed sub-panels, both unticked. **The Memory box is not ticked**, which matches what a running Blender reports and not the API page's stated default of True. Identical in all four versions. The picture is at `/tmp/smoozy-war/out/e27/panel_5.2.png`.

**The build reports itself as `BUILD_PLATFORM b'Darwin'`**, `BACKGROUND True`, `cycles: True`, `cycles_osl: True`, and every conditional image format compiled in. Against the Linux build of the same version, the differences are `coreaudio` True here and False there, `jack` and `pulseaudio` False here and True there.

**Threads read `THREADS 8 THREADS_MODE AUTO`** at factory startup — this machine's core count, not the documented default of 1.

---

### Tested by hand again after the citation check, 2026-08-07 — **LABEL: observed**

**Why these runs happened.** The citation check of this topic found four disagreements, recorded both sides of each, chose between none of them, and sent them back to the hand tests. Three of the four are tests. They were run on 2026-08-07 on the rented Linux machine, alone and last, because that machine was about to be taken away and nothing else could be asked of it afterwards. They are entries 42, 43 and 44 of `Audits/Blender Research/What actually ran/Hand tests.md`.

**The machine, named in full.** The rented Linux machine: Ubuntu 22.04.5 LTS, kernel `6.8.0-59-generic`, one NVIDIA GeForce RTX 4080 SUPER with 16376 MiB of graphics memory, driver **580.173.02** in both halves — `nvidia-smi` and `/proc/driver/nvidia/version` were read at the start of the sitting and agree — an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all. Nothing else was running on it: the load average was 0.19 and there was no Blender process. Everything was written inside `/root/again-what-actually-ran/`, and each of the four Blenders was given a settings folder of its own, made before it was used. The scene is the one-cube factory scene at 160 by 120, sixteen samples, denoising off, built by Blender 4.5 so that all four versions can open it. No test below reads what is written inside a finished image, so no picture and no file was compared.

**Two of the three tests answer this question. They are set out one after the other below.**

---

#### Test one — what the `Full path tracing report` block holds in this version, transcribed exactly

*What a negative result would look like, said before the run.* No `Hardware Ray-Tracing:` line in this version's block. That is a live outcome: Blender 5.0 was run identically alongside and its block has no such line, so the capture is plainly able to come back without one.

*What was done.* `/opt/blender/5.2/blender -b scene.blend --log-level info -o out/ -F PNG -f 1`, five times, both streams captured to separate files. The same run was then made three more times with the graphics card doing the work. This version was run as a control for a disagreement recorded in the 5.0 documents; the result is written here because it is this version's own answer as much as it is 5.0's control.

*What came back, word for word.* Exit 0 and a 0-byte error stream every time. The block, from the first of the five processor runs, uncut:

```
00:00.517  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: AMD EPYC 7B12 64-Core Processor (CPU) (24 threads) [CPU]
                            |                      Hardware Ray-Tracing: Off
                            | 
                            | Render Scheduler Summary
                            | 
                            | Mode: Headless
                            | Resolution: 160x120
```

**The `Hardware Ray-Tracing: Off` line is there, in all five runs**, and in all three of the runs with the graphics card doing the work, where the line above it reads `Path tracing on: NVIDIA GeForce RTX 4080 SUPER (OptiX) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07_OptiX]`. So it says `Off` even when an NVIDIA card with ray-tracing hardware is doing the work. The five runs are identical to one another once their timestamps are set aside, and identical to Blender 5.1's block in the same way. What the line means, and what would make it say anything other than `Off`, is not settled by any run here and is not claimed — the citation check sent that question back to research.

*The control did its job.* Blender 5.0, run identically on the same machine on the same day, has no such line in any of its eight runs. So the presence of the line here is a fact about the version.

*The disagreement this belongs to.* The disagreement sent back was in the 5.0 documents, where one run of 2026-08-06 recorded this line for 5.0 and every later run did not. Nothing in this version's record is disturbed: 5.2 has carried the line in every run anybody has made.

---

#### Test two — how many `Building OptiX acceleration structure` lines a run prints

*What a negative result would look like, said before the run.* The same number in all five runs of a version — which would have meant the two earlier counts were a miscount rather than a moving target, and the finding could then name the number with five repeats behind it. The CUDA runs beside them are the second way this test can fail: they show what the same count looks like when the answer is nought, so the counting is not something that only ever returns small positive numbers.

*What was done.* The shared scene rendered on the card with OptiX and **no log switches at all**, five times per version, counting the lines that carry `Building OptiX acceleration structure` on each stream. The card was pointed at by a script that sets the Cycles preference to OptiX and the scene to GPU, and prints the device list back so the run says rather than assumes what it used — it read `[('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', False), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False), ('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True)]` in every OptiX run. Five CUDA runs per version were made the same way, as the control.

*What came back.* The count is **not stable**. Five OptiX runs of the identical command in this version gave, in order:

```
2, 4, 2, 3, 3
```

and the five CUDA runs beside them gave `0, 0, 0, 0, 0`. Exit 0 and a frame written in all ten; the error stream was 0 bytes in all ten. **Both of the numbers the two earlier runs disagreed about — three and two — sit inside that spread**, and so does a third number neither of them reported.

*Why it moves, read straight off the lines.* The lines are ordinary progress lines, and which of the geometry steps gets a line of its own varies from run to run. Two runs of this version, side by side, with the timestamps and memory figures taken out:

```
run 1
  render | Fra: 1 | Updating Geometry BVH Light 3/3 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Scene BVH | Building OptiX acceleration structure

run 2
  render | Fra: 1 | Updating Geometry BVH backgroundlight 4/3 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Geometry BVH Light 3/3 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Geometry BVH Cube 2/1 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Scene BVH | Building OptiX acceleration structure
```

The `Updating Scene BVH` line is in every run of every version. The geometry lines above it are not. So the number is a fact about which progress steps happened to be reported, and never a fact about Blender or about OptiX.

*What this settles, and what must not be written.* Across the four versions the counts were 4.5 `3, 4, 4, 3, 4`; 5.0 `2, 4, 4, 2, 4`; 5.1 `3, 4, 3, 3, 4`; 5.2 `2, 4, 2, 3, 3`; and CUDA nought, twenty runs out of twenty. **The number must never be quoted as a fact about a version.** What holds, and held in every run anybody has made, is the half the claim rests on: lines of this shape appear in an OptiX run and never once in a CUDA run of the same scene. The right form of words is *more than once*, which is what the documents now say.


## 2. When a render has finished, what can be learned about which render engine produced it?

**LABEL: not in the sources** (for reading it off the finished render), with **documented** and **observed** parts around the edges.

**What the sources do say.** The engine is a scene setting, saved with the file and readable by name:

> "engine — Engine to use for rendering (default 'BLENDER_EEVEE')"

Source: `https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html`. Date read 2026-07-30. (The page lists the type as `Literal['BLENDER_EEVEE']`. I take that at face value and note the oddity rather than explaining it away — that is a single-value list in the documentation, not a claim that only one engine exists, since the same page also carries "has_multiple_engines — More than one rendering engine is available (default False, readonly)".)

The engine can also be forced from outside:

> "-E, --engine <engine> — Specify the render engine. Use -E help to list available engines."

Source: `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`. Date read 2026-07-30. And in the worked examples:

> "-E CYCLES — Use the "Cycles Render" engine. For a list of available render engines, run blender -E help."

Source: `https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html`. Date read 2026-07-30.

Every engine carries a name in its own definition. **Documented**, from the API:

> "bl_idname — (default "", never None)"
> "bl_label — (default "", never None)"

Source: `https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html`. Date read 2026-07-30. **Observed**, for Cycles specifically:

> `class CyclesRender(bpy.types.RenderEngine):`
> `    bl_idname = 'CYCLES'`
> `    bl_label = "Cycles"`

File `cycles_5.2/__init__.py`, lines 44–46. Date read 2026-07-30.

And Blender ships three:

> "Blender provides the following render engines: EEVEE … Cycles … Workbench"
> "More renderers from third-party developers are available as add-ons."

Source: `https://docs.blender.org/manual/en/5.2/render/render_engines.html`. Date read 2026-07-30.

**What the sources do not say.** Nothing in the metadata list is the engine. The sixteen stamp switches, and the panel picture, contain no engine field. So a finished image, as Blender writes it, does not carry the engine's name by any documented route.

There is one hook that could carry it, and the sources describe the hook without saying anyone uses it:

> "stamp_data_add_field(field, value) — Add engine-specific stamp data to the result"
> "field (str) – Field, Name of the stamp field to add (never None)"
> "value (str) – Value, Value of the stamp data (never None)"

Source: `https://docs.blender.org/api/5.2/bpy.types.RenderResult.html`. Date read 2026-07-30. **Not in the sources**: whether Cycles, EEVEE or Workbench call this, and with what. I searched Cycles' Python files for `stamp` and found nothing, and the host-side C++ is not in this folder.

**Not covered.** Whether an image file written by 5.2 carries any producer or software tag of its own, outside the stamp list. Whether the engine's name can be recovered from a multi-layer OpenEXR by its pass names or layer names. Whether third-party engines write anything of their own. What `-E help` actually prints.

**Outside the interface.** The engine setting is readable through Python either way, and `-E` only exists on a command line. The manual's own animation example — "blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a" (`https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html`) — is written for the no-window case, so the naming of the engine from outside is documented for exactly that case. Reading `RenderSettings.engine` back afterwards is a property read; the sources do not say it behaves differently with no window. **Settled by test on 2026-08-06, on Linux only — see below.** The engine's name reaches a finished image with no window involved, through the four `cycles.ViewLayer.*` stamp fields and, from 5.0 on, an OpenEXR `Software` attribute. `-E help` needs `-b` on a machine with no screen; in Blender 4.5 it never returns even then. **Added by the tests of 2026-08-07:** nothing about the engine changes when a graphics card does the work. Every route above behaves the same outside the window with the card in use as with the processor.

**Operating system.** Nothing in any of the above is qualified by operating system, with one exception that bears on which engine can run at all: "Headless rendering is not supported on headless Windows systems", under EEVEE's Limitations (`https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html`, date read 2026-07-30). **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.2`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**A finished image does carry a trace of the engine, in two ways, and one of them is new since 4.5.**

*Cycles writes four stamp fields of its own.* The same frame was rendered twice with every metadata switch on, once with `-E CYCLES` and once with `-E BLENDER_EEVEE`. The Cycles file carried four fields the EEVEE file did not:

```
   cycles.ViewLayer.samples = 16
   cycles.ViewLayer.total_time = 00:00.03
   cycles.ViewLayer.render_time = 00:00.05
   cycles.ViewLayer.synchronization_time = 00:00.01
```

The EEVEE render of the same frame carried only the documented stamp fields:

```
   File = /root/what-actually-ran/work/scene.blend
   Note = SMOOZY NOTE
   Date = 2026/08/06 22:59:57
   Marker = <none>
   Time = 00:00:00:01
   Frame = 1
   FrameRange = 1:3
   Camera = Camera
   Lens = 50.00
   Scene = Scene
   Strip = <none>
   RenderTime = 00:20.50
   Memory = 0M
   Hostname = ubuntu
```

So the hook the API describes for engine-specific stamp data is in use, and the field names begin with the engine's own name.

*An OpenEXR also carries a `Software` attribute.* The Cycles render to OpenEXR carried, among its attributes:

```
   Software (string) = Blender 5.2.0 LTS
   colorInteropID (string) = lin_rec709_scene
```

Neither appears in a PNG, and **neither appears in the equivalent file written by Blender 4.5** — that comparison was run on the same machine on the same day and is recorded in 4.5's own document. This closes the *Not covered* line above about a producer or software tag.

**The log names the engine at default settings.** The plain background render quoted under question 1 carries `render | Engine: Cycles` with no switch at all.

**`blender -b -E help` lists three engines.** Exit 0:

```
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

**The `-b` matters:** run without it on a machine with no screen, `-E help` printed GHOST display errors and never returned.

**An engine name Blender does not know is refused.** Run as `blender -b scene.blend -E NOT_AN_ENGINE -o out/be_ -F PNG -f 1`:

```
Error: engine not found 'NOT_AN_ENGINE'
```

**The engine list a running Blender reports really does hold one entry.** Run under `--factory-startup`:

```
ENGINE 'BLENDER_EEVEE'
ENGINE_ITEMS ['BLENDER_EEVEE']
HAS_MULTIPLE_ENGINES True
```

So the single-entry list is **not** an artefact of how the documentation was generated — a running Blender says the same thing, while the same Blender lists three engines to `-E help` and accepts `-E CYCLES`. Any reasoning above about why that list is short should be read against this.


### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.2`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**Which part of the computer did the work changes nothing about what names the engine.** The metadata test was run again with the scene on GPU Compute and OptiX really doing the work: a Cycles render with every `use_stamp_*` switched on, to PNG and to OpenEXR. **The field set is identical to the processor's** — the same four `cycles.ViewLayer.*` fields, and the same `Software` attribute in the OpenEXR naming Blender and its version. Nothing new names the engine, and no field names the card.

The one thing a card adds sits in the printed output, not in the file. `Building OptiX acceleration structure` (question 1) names a *technology*, not an engine — somebody reading only that line would know a graphics card was in use and still not know which engine had asked for it. Cycles is the only engine that can ask, on the evidence of `-E help`, but that is a step this run does not take.

`-E help`, the `render | Engine: Cycles` line and the engine list were settled on 2026-08-06 and are untouched by the card; they were not run again, so that a second result could not be laid over a first.

**Where I looked.** Searched the whole 5.2 manual and API for: `engine`, `stamp`, `metadata`, `EXIF`, `Software`, `producer`, `written by blender`, `created with blender`, `RenderEngine`. Opened: the Render Engines page, the Introduction to rendering page, the Metadata page and its picture, the Command Line Arguments and Rendering From The Command Line pages, `bpy.types.RenderSettings`, `bpy.types.RenderEngine` in full, `bpy.types.RenderResult` in full. Grepped `manual_5.2/render/output` for `engine` and found only a stereoscopy sentence, unrelated. Grepped Cycles' Python files for `stamp` — no match. Ruled out the release notes after grepping all twenty-one 5.2 files for `metadata`, `stamp`, `engine`: the only metadata mentions are glTF/USD and video-stereoscopy export, not render stamping.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**The log names the engine on macOS too.** A background render with no switches at all put this on standard output:

```
render           | Engine: Cycles
```

The line is in the same place in the same run in 5.0, 5.1 and 5.2, and Blender 4.5 on the same machine printed nothing of the kind.

**`-E help` lists three engines.** `blender -b -E help`, exit 0, standard output:

```
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

**But a running Blender's own engine list still holds one entry.** Read at factory startup with no window:

```
ENGINE 'BLENDER_EEVEE'   ENGINE_ITEMS ['BLENDER_EEVEE']   HAS_MULTIPLE_ENGINES True
```

So the short list is not an artefact of how the documentation was made, and it is not an artefact of the operating system either.

**Cycles writes four fields of its own into every image it renders, and EEVEE does not.** With every metadata switch on, the Cycles PNG carried `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time`; the EEVEE PNG of the same frame (engine `BLENDER_EEVEE` in this version) carried none of them. Counted in all four versions: four in the Cycles file, none in the EEVEE file.

**The OpenEXR carries `Software`.** From the Cycles OpenEXR written on this machine:

```
   Software (string) = Blender 5.2.0 LTS
   colorInteropID (string) = lin_rec709_scene
```

Blender 4.5 on the same machine wrote neither attribute. The PNGs carry no `Software` field in any version. Same as the Linux runs.

---

## 3. When a render has finished, what can be learned about which add-ons were in use while it ran?

**LABEL: not in the sources** for "in use while it ran", **documented** for "enabled now".

**The question is ambiguous** and I say so rather than choose quietly. "In use while it ran" can mean (i) which add-ons were enabled at the time, or (ii) which add-ons actually did something to that render. I answer (i), because that is the only reading the sources touch. Under reading (ii) my answer would be a flat "not in the sources": nothing I found records an add-on's contribution to a particular render.

**What is documented.** Enabled add-ons can be written to a file, on request:

> "Enabled Add-Ons — Lists add-ons currently in use along with their versions and paths."

That is a section of `system-info.txt`, described on the same page as the rest of the System Info sections:

> "You will be prompted to save a text file called system-info.txt."

Source: Blender manual, `https://docs.blender.org/manual/en/5.2/getting_started/help.html`. Date read 2026-07-30. Note the manual's own phrase is "currently in use", meaning at the moment the file is written — not during any particular render.

The operator behind it:

> "bpy.ops.wm.sysinfo(*, filepath='') — Generate system information, saved into a text file"

Source: `https://docs.blender.org/api/5.2/bpy.ops.wm.html`. Date read 2026-07-30.

Enabled add-ons are also a live list in the preferences:

> "addons — (default None, readonly) … Type: Addons[Addon]"

Source: `https://docs.blender.org/api/5.2/bpy.types.Preferences.html`. Date read 2026-07-30. And each entry:

> "class bpy.types.Addon(bpy_struct) — Python add-ons to be loaded automatically"
> "module — Module name (default "", never None)"
> "preferences — (readonly) … Type: AddonPreferences | None"

Source: `https://docs.blender.org/api/5.2/bpy.types.Addon.html`. Date read 2026-07-30.

**Observed**, that this collection is read in practice by name of package — Cycles reads its own preferences out of it:

> `def get_device_type(context):`
> `    return context.preferences.addons[__package__].preferences.compute_device_type`

File `cycles_5.2/ui.py`, lines 96–97. Date read 2026-07-30. And Cycles is itself an add-on, with a version-style header. **Observed**:

> `bl_info = {`
> `    "name": "Cycles Render Engine",`
> …
> `    "support": 'OFFICIAL',`
> `    "category": "Render"}`

File `cycles_5.2/__init__.py`, lines 7–16.

Add-ons can also be turned on from outside for a single run:

> "--addons <addon(s)> — Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons."

Source: `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`. Date read 2026-07-30. **Inferred**, one step: if a run was started with `--addons`, then that run had those add-ons on top of the defaults. The step the reader supplies is that the switch did what it says; the sources describe the switch, not its outcome.

Failures to load are reported, but to a place, not to a file:

> "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred."

Source: `https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html`, and the same sentence appears at `https://docs.blender.org/manual/en/5.2/advanced/extensions/addons.html`. Date read 2026-07-30.

**What is not in the sources.** Nothing records the enabled-add-on list *into a render*. The metadata list has no add-on field. The one place a person could put such a thing is the free-text note, and the manual says as much without ever mentioning add-ons:

> "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number."

Source: `https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html`. Date read 2026-07-30.

**Not covered.** Whether a blend-file records the add-ons that were enabled when it was saved. Whether an add-on that modified the scene leaves any trace in the render. Whether `system-info.txt` records add-on state at any moment other than the moment it is written. What "versions and paths" look like in that file (I have the manual's description of the section, not a sample of it).

**Outside the interface.** `--addons` is a command-line switch and exists only there. The preferences collection is Python and needs no window, though the sources do not say so. "Save System Info" is described as a menu item; its operator takes a `filepath`, which is what a script would need, but the sources do not state that it works with no interface running. The advice to "check the Console window" is about a console — the manual's Command Line page says "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from" (`https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html`, date read 2026-07-30) — so on a machine driven from outside, those messages land in the terminal. **Settled by test on 2026-08-06, on Linux only — see below.** `bpy.ops.wm.sysinfo` does work with no window in every version, and its Enabled add-ons section is filled in with versions and paths, so the inferred step above holds. **Added by the tests of 2026-08-07:** the add-on section of `system-info.txt` written from outside the window is unchanged by the card working, and no field a render writes names an add-on whichever device ran it.

**Operating system.** Nothing in the above is qualified by operating system. The one system-specific note nearby is that the Console Window is a separate thing on Windows: "When Blender is started on a Microsoft Windows operating system, the Console Window … The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console" (`https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html`, date read 2026-07-30). **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.2`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**Nothing about add-ons reaches a rendered image.** The fields written into a Cycles PNG and a Cycles OpenEXR with every stamp switch on are listed in full under question 1. None names an add-on. The only engine-shaped fields are the four `cycles.ViewLayer.*` fields recorded under question 2.

**`bpy.ops.wm.sysinfo` works with no window and its add-on section is filled in.** It returned `{'FINISHED'}` and wrote a 94-line file. The heading in the file is `Enabled add-ons:`, not `Enabled Add-Ons` as the manual writes it, and under it sat eight entries with versions and paths:

```
io_anim_bvh (version: (1, 0, 1), path: '.../scripts/addons_core/io_anim_bvh/__init__.py')
io_curve_svg (version: UNKNOWN, path: '.../scripts/addons_core/io_curve_svg/__init__.py')
io_mesh_uv_layout (version: (1, 2, 0), path: '.../scripts/addons_core/io_mesh_uv_layout/__init__.py')
io_scene_fbx (version: (5, 15, 0), path: '.../scripts/addons_core/io_scene_fbx/__init__.py')
io_scene_gltf2 (version: (5, 2, 39), path: '.../scripts/addons_core/io_scene_gltf2/__init__.py')
cycles (version: UNKNOWN, path: '.../scripts/addons_core/cycles/__init__.py')
pose_library (version: (2, 0), path: '.../scripts/addons_core/pose_library/__init__.py')
bl_pkg (version: (0, 0, 1), path: '.../scripts/addons_core/bl_pkg/__init__.py')
```

(The version numbers quoted are this version's; the paths are abbreviated here and are given in full in the hand-tests file.)

**The list that file holds is current, not stale.** In one session, without restarting, the file was written, an add-on switched off, and the file written again:

```
ON_BEFORE True
DISABLE {'FINISHED'}
ON_AFTER False
   io_anim_bvh in first file: 1  in second file: 0
```

So an add-on switched off part-way through a session is gone from the next report written in that session.

**The enabled list a script reads.** Run as `blender -b --python-expr "import bpy; print('MODULES', sorted(a.module for a in bpy.context.preferences.addons))"`:

```
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

The same command with `--addons node_wrangler` printed the identical list and said nothing at all about the named add-on. **What that does and does not show:** `node_wrangler` is not among the eight this Blender ships, so the run cannot tell "loaded but not listed" apart from "nothing to load". What it does establish is that naming an add-on that is not there produces no message and no failure.


### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.2`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**The card changes nothing here, and the negative result is now stronger for it.** Every field written into a PNG and into an OpenEXR by a render that really ran on the graphics card was read back (see question 1's run). **No field names an add-on**, exactly as when the processor did the work.

The `Enabled add-ons:` section of `system-info.txt` is unchanged too. The file grew from 94 lines to 183 with the card repaired, and **every added line is in the Cycles section**, describing the card's capabilities. The add-on list, its versions and its paths are identical.

So the 2026-08-06 answer stands, with the case that was missing from it filled in: a finished render leaves no trace of which add-ons were loaded, whichever part of the computer did the work.

**Where I looked.** Searched the whole 5.2 manual and API for: `add-on`, `addon`, `addons`, `extension`, `enabled add-on`, `add-ons currently`, `enabled extensions`, `addon_utils`, `bl_info`. Opened: the Preferences Add-ons page in full, the Extensions Add-ons page, the Help menu page, `bpy.types.Addon` in full, `bpy.types.Preferences`' `addons` entry, `bpy.ops.wm.sysinfo`. Checked the metadata field list and the panel picture again for anything add-on-shaped — nothing. Grepped the API folder for `addon_utils` — no match anywhere, so the module that lists add-ons in scripts is not in the API documentation at all. Grepped all twenty-one 5.2 release-note files for `add-on`/`addon` — the hits are three Loop Tools features moving into Blender proper, one Rigify bug fix, and one Brushstroke Tools bug fix; none of them about recording which add-ons ran.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**`bpy.ops.wm.sysinfo` fills its add-on section in with no window.** The same eight modules the Linux machine ships, at macOS paths under `/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core/` — `io_anim_bvh`, `io_curve_svg`, `io_mesh_uv_layout`, `io_scene_fbx`, `io_scene_gltf2`, `cycles`, `pose_library` and `bl_pkg`, each with its version in brackets and its full path. The file written with a window open carries the identical section.

**The list is current, not a record of the session's start.** In one session: write the file, switch `io_anim_bvh` off, write it again without restarting.

```
ON_BEFORE True
DISABLE {'FINISHED'}
ON_AFTER False
   io_anim_bvh in first file: 1  in second file: 0
```

**`--addons` changed nothing a script could read.** With and without `--addons node_wrangler`, the enabled list was identical, and identical to the Linux machine's:

```
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

Nothing was printed about the named add-on either way. The same limit applies as on Linux: `node_wrangler` is not among the eight this build ships, so this shows only that naming an absent add-on produces no message and no failure.

---

## 4. What does a render do when it is asked for something the computer or the installed Blender cannot provide, and what does it report?

**LABEL: mixed — documented in several named cases, not in the sources as a general rule.**

There is no single documented rule. What there is, is a set of named cases, and they do not all behave the same way. I give them as the sources give them.

**Case A — Cycles, GPU memory runs short: it keeps going, using system memory.** **Documented**:

> "Why does a scene that renders on the CPU not render on the GPU?"
> "There may be multiple causes, but the most common one is that there is not enough memory on your graphics card."
> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

Source: `https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html`. Date read 2026-07-30. Note that oneAPI is absent from that list of four.

**Case B — Cycles, memory runs short anyway: an error with a named text.** **Documented**:

> "Error: Out of memory"
> "This usually means there is not enough memory to store the scene for use by the GPU."

Same page and date. That page's "Error Messages" heading also names three more with their literal texts: "The NVIDIA OpenGL driver lost connection with the display driver", "Unsupported GNU version", "CUDA Error: Kernel compilation failed". Of the last:

> "This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed. In this case Blender may try to dynamically build a kernel for your graphics card and fail."

Same page and date. That is the clearest documented statement of "the installed Blender cannot provide it" — and it is reported as an error message naming CUDA.

**Case C — no compatible card at all: a message in the preferences, before anything runs.** **Observed**, in Cycles' own files:

> `        if not found_device:`
> `            col = box.column(align=True)`
> `            col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)`

File `cycles_5.2/properties.py`, lines 1864–1866, followed by per-backend advice lines quoted in the same block, for example:

> `                col.label(text=rpt_("Requires NVIDIA GPU with compute capability %s") % compute_capability,`

(lines 1875–1876, in the OptiX branch), and

> `            col.label(text=rpt_("Requires Apple Silicon with macOS %s or newer") % mac_version,`

(lines 1938–1939, in the Metal branch). Date read 2026-07-30. These are drawing calls in the preferences panel, so they are messages shown to a person looking at preferences, not something a render prints.

**Case D — the scene asks for GPU but no GPU is on: Cycles' interface treats it as CPU.** **Observed**:

> `def use_cpu(context):`
> `    cscene = context.scene.cycles`
> `    return (get_device_type(context) == 'NONE' or cscene.device == 'CPU' or not backend_has_active_gpu(context))`

(the blank line between the two statements in the file is omitted from the quote above and below)

File `cycles_5.2/ui.py`, lines 104–107. And:

> `def use_gpu(context):`
> `    cscene = context.scene.cycles`
> `    return (get_device_type(context) != 'NONE' and cscene.device == 'GPU' and backend_has_active_gpu(context))`

File `cycles_5.2/ui.py`, lines 110–113. Date read 2026-07-30.

**Inferred**, and this is exactly the line the brief warns about, so I write the step out plainly: these functions decide which settings the interface shows. From them I can say that *the interface* treats "device set to GPU, but no active GPU device" as the CPU case. Whether *the render* then actually runs on the CPU is a step beyond what the file says, because the render is done by the C++ that is not in this folder. Do not read this as "Cycles falls back to CPU". Read it as "Cycles' panels behave as if it will".

The manual's own words for the None case point the same way, and this one is **documented**:

> "None: When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."

Source: `https://docs.blender.org/manual/en/5.2/editors/preferences/system.html`. Date read 2026-07-30.

**Case E — a feature the chosen device cannot do: named, per feature.** **Documented**:

> "Path Guiding is not supported on any GPU."
> "Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

Source: `https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html`. Date read 2026-07-30. Also:

> "Custom cameras are not supported with GPU rendering unless using the OptiX backend."

Source: `https://docs.blender.org/manual/en/5.2/render/cycles/osl/camera.html`. Date read 2026-07-30. And:

> "GPU rendering using Radeon RX 6000 series HIP devices on Windows is not supported."

Source: `https://docs.blender.org/manual/en/5.2/render/cycles/object_settings/object_data.html`. Date read 2026-07-30. The sources say the feature is not supported. They do not say what happens if you ask for it anyway.

One case does say what happens, and it says the work moves rather than failing. **Documented**:

> "GPU: Use the GPU to process the denoise node if available, otherwise fallback to CPU."

Source: `https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html` (the same sentence appears for EEVEE at `https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/performance.html`). Date read 2026-07-30.

**Case F — the build does not have the feature: an error, with a fixed text.** **Observed**:

> `    def update_script_node(self, node):`
> `        if engine.with_osl():`
> `            from . import osl`
> `            osl.update_script_node(node, self.report)`
> `        else:`
> `            self.report({'ERROR'}, "OSL support disabled in this build")`

File `cycles_5.2/__init__.py`, lines 108–113, and the same message again at lines 115–120 for `update_custom_camera`. Date read 2026-07-30. That is a literal message text, tied to a build that lacks a feature.

The command line has the same shape for image formats. **Documented**:

> "-F, --render-format <format> — Set the render format. Valid options are: TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF."
> "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

Source: `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`. Date read 2026-07-30. What Blender does when you name one of those on a build that lacks it is **not in the sources**.

**Case G — a bad device name on the command line: it stops.** **Observed**, and this is the single most definite thing I found:

> `    if args.cycles_device:`
> `        import _cycles`
> `        if not _cycles.set_device_override(args.cycles_device):`
> `            sys.exit(1)`

File `cycles_5.2/engine.py`, lines 38–41. Date read 2026-07-30. **Inferred**, one step: if `--cycles-device` names something Cycles will not accept, the whole Blender process exits with status 1. The step: I am reading `set_device_override` returning a false value as "the device was not accepted" from the surrounding code's shape, since the function itself is in the C++ that is not here.

**Case H — nowhere to write the result: an error.** **Documented**:

> "When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."
> "Playblast operations also require an active output method. Both Render Playblast and Render Playblast on Keyframes will raise an error when render output is disabled in this panel."

Source: `https://docs.blender.org/manual/en/5.2/render/output/properties/output.html`. Date read 2026-07-30. And the same in the animation page:

> "If no valid output method is available, rendering operations will report errors."

Source: `https://docs.blender.org/manual/en/5.2/render/output/animation.html`. Date read 2026-07-30. This behaviour is new in 5.2, and the release note is explicit:

> "* Saving render output can now be disabled. (blender/blender@342f37566f)"

Source: `https://developer.blender.org/docs/release_notes/5.2/rendering/`. Date read 2026-07-30.

**Case I — the format cannot hold what you asked to write: it is silently dropped.** **Documented**:

> "Not all formats support metadata, and some may only support a limited subset of fields."
> "When using formats that do not support metadata, render information is not preserved in the saved file."

Source: `https://docs.blender.org/manual/en/5.2/files/media/image_formats.html`. Date read 2026-07-30. That page's table gives the answer per format: metadata is supported by JPEG, OpenEXR, PNG and WebP, and marked ✗ for AVIF, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw and TIFF. Nothing there says a warning is given.

**Case J — EEVEE, when the machine cannot take it.** **Documented**:

> "In practice, using too much GPU memory can make the GPU driver crash, freeze, or kill the application. So be careful of what you ask."
> "There is no standard way of estimating if the resources will fit into the GPU memory and/or if the GPU will render them successfully."
> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient."
> "There is currently no support for multiple GPU systems."
> "Headless rendering is not supported on headless Windows systems."

Source: `https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html`. Date read 2026-07-30. This is the plainest statement in the sources that a failure can be a crash rather than a message, and that EEVEE has no CPU to fall back to.

**And what "reporting" means, generally.** The API gives an engine two ways to say something went wrong:

> "report(type, message) — Report info, warning or error messages"
> "error_set(message) — Set error message displaying after the render is finished"

Source: `https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html`. Date read 2026-07-30. The second is worth noting for this question: an error can be held back and shown once the render is over.

Where such messages go, for a person: "The Info editor logs the executed operators as well as errors, warnings, and informational messages" (`https://docs.blender.org/manual/en/5.2/editors/info_editor.html`), and "Report Message — Informational messages or warnings, such as after saving a file. They disappear after a short time. Click them to show the full message in the Info Editor" (`https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html`). Date read 2026-07-30 for both.

And if it does not report at all but dies, something is left on disk:

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."

Source: `https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html`. Date read 2026-07-30.

**Not covered.** There is no general documented rule for "asked for the impossible". I could not find, anywhere in the sources: what Blender does when `-E` names an engine that is not installed; what it does when `-F` names a format the build lacks; what exit status a failed render leaves (the phrase "exit code", "exit status" and "return code" appear nowhere in the 5.2 manual or API); whether an unsupported feature is silently ignored, warned about, or refused; whether the GPU-to-system-memory move in Case A is announced anywhere.

**Outside the interface.** Poorly served, and this matters. Cases C, and the Info-editor and status-bar routes for reports, are all things drawn in a window and are simply absent when Blender is driven from outside. Case G (exit status 1) is the only one that is unambiguously about a run with no window. Cases A, B, E, F, H, I and the crash file do not depend on a window in what the sources say, but the sources also never say what any of them look like with no interface. The one bridge the sources build is the console: "Blender output is shown in the Console Window it is started from" and "If Blender exits unexpectedly, the messages may indicate the cause or error" (`https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html`, date read 2026-07-30). **Settled in part by test on 2026-08-06, on Linux only — see below.** Every failure message found reached standard output or the error stream with no window: the `--cycles-device` refusal, the unwritable-output error, and the `-F` format complaint. The preferences-panel messages could not be reached at all with no window, as expected, and no substitute for them appeared anywhere. **Added by the tests of 2026-08-07:** `--cycles-device` exists only outside the window, and with the card working it can now be seen to succeed silently for `OPTIX` and `CUDA` and to refuse `HIP` and `ONEAPI` with `Found no Cycles device of the specified type`. The out-of-memory case is still out of reach, because this machine's own memory gives out before the card's does.

**Operating system.** Several of these are system-specific by their own words. "Headless rendering is not supported on headless Windows systems" (EEVEE Limitations). "GPU rendering using Radeon RX 6000 series HIP devices on Windows is not supported." "The NVIDIA OpenGL driver lost connection with the display driver" is described as a Windows time-limit problem: "If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations." "Unsupported GNU version — On Linux, depending on your GCC version you might get this error." The crash file differs on all three systems: on Windows "a file is written based on the name of the currently loaded blend-file, so test.blend will create a file called test.crash.txt"; on macOS "Some .crash files can also be found in ~/Library/Logs/DiagnosticReports/"; on Linux "a file named blender.crash.txt is written to the /tmp directory" (all `https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html`, date read 2026-07-30). **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.2`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

Six different kinds of shortfall were tried, and they do not behave alike. **Windows was not tried at all** — no Windows machine has run it — so every Windows-specific line above stays untested rather than confirmed.

**A device demanded on the command line that Cycles will not give: exit 1 and no frame.** Five runs, each deleting the output first:

| `-- --cycles-device` | exit | frame written |
|---|---|---|
| `NOTADEVICE` | 1 | no |
| `OPTIX` | 1 | no |
| `CUDA` | 1 | no |
| `METAL` | 1 | no |
| `CPU` | 0 | yes |

A name Cycles does not know, on standard output through the log:

```
00:00.281  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

A real backend with no hardware behind it, also on standard output:

```
00:00.544  cycles           | ERROR CUDA cuInit: Unknown error
00:00.548  reports          | ERROR Found no Cycles device of the specified type
```

`METAL` gave only the second of those two, with no CUDA line. (In Blender 4.5 these same messages go to the **error** stream instead; that is recorded in 4.5's own document.)

**A scene asking for GPU Compute where no card can be used: it renders anyway, and it says what it used.** With `scene.cycles.device` set to `GPU` and the Cycles preference reading `NONE`, run as `blender -b scene.blend --python setgpu.py --log "*" --log-level info -o out/g_ -F PNG -f 1`:

```
exit=0
frame written
PREF_COMPUTE_DEVICE_TYPE NONE
SCENE_CYCLES_DEVICE GPU
                            | Path tracing on: AMD EPYC 7B12 64-Core Processor (CPU) (24 threads) [CPU]
00:00.684  render           | Time: 00:00.20 (Saving: 00:00.13)
```

The same file with the device set to CPU gave `exit=0`, a frame, the **identical** `Path tracing on:` line, and `Time: 00:00.19`. So the run is not silent about it. The times are not evidence and are not offered as any: the machine was shared with other work while these ran, the scene takes a fifth of a second, and the two runs differ by less than the spread between repeats.

**A render that cannot write its output: exit 1, and it says so.** With a plain file sitting where the output folder should be:

```
00:00.619  image            | ERROR Couldn't create directory for file .../notadir/z_0001.png: File exists
00:00.619  reports          | ERROR Render error (File exists) cannot save: '.../notadir/z_0001.png'
```

exit status 1.

**A format name that is not a format: it complains and renders anyway.** Run as `blender -b scene.blend -o out/f_ -F NOSUCHFORMAT -f 1`, exit **0**, a file `f_0001.png` written in the scene's own format, and this on the error stream:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
```

The wording covers a bad name and a missing build feature in one sentence, so the two cannot be told apart from it. This settles what the *Not covered* line above leaves open — with the limit that every conditional format is compiled into this build, so the second half of that sentence could not be provoked.

**A format that cannot hold metadata: nothing at all is said.** With every stamp switch on and `-F TIFF` and then `-F TGA`, both runs exited 0 and wrote their file. Counting lines in both streams matching any of `warn`, `error`, `metadata`, `not supported` or `preserved` gave 0 and 0. The metadata is simply gone, quietly.

**A render asked to stop part-way: a complete file, and exit 0.** — **This reading is wrong and is corrected further down this document.** *(Marked by the citation check, 2026-08-07.)* With adaptive sampling on, this scene converges and finishes by itself before six seconds, so the complete file and the exit 0 describe a render that ended normally, not one that was interrupted. The Mac tests of 2026-08-07 repeated it with `use_adaptive_sampling = False` and got status 143 and no file at all. Read that block, not this one. The original wording is kept below so the two runs can be told apart. A slow render (100000 samples at 1200 by 900) was started and sent a stop signal after six seconds. The exit status was **0**, and a valid PNG ending in `IEND` was left at the output path, carrying:

```
   RenderTime = 00:03.25
   cycles.ViewLayer.samples = 100000
   cycles.ViewLayer.total_time = 00:03.24
```

The stamp records what the scene asked for, not what was reached. **Nothing in the exit status, the file's size, its validity or its fields distinguishes it from a render that finished.** Note this is a render *asked to stop*, which is not the same as one that fails on its own; the unwritable-output case above ends non-zero.

**Two messages the sources describe cannot be provoked on this machine.** `bpy.app.build_options` reports `cycles_osl: True`, so no build here lacks Open Shading Language and "OSL support disabled in this build" cannot be reached. There is **no Embree entry in `build_options` at all**, so nothing can say whether the build has it and the Embree lines cannot be reached either. This build reports 44 fields, with `cycles: True`, `image_openexr: True`, `image_openjpeg: True`, `image_webp: True`, `codec_ffmpeg: True`, and `openmp: False`, `sdl: False`, `codec_avi: False`.


### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.2`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**`--cycles-device OPTIX` and `--cycles-device CUDA` now succeed, and say nothing at all.** The same command line that on 2026-08-06 gave exit 1 and no frame now gives exit 0, a frame written, and a 0-byte error stream:

| `--cycles-device` | exit | frame written | what was said |
|---|---|---|---|
| `OPTIX` | 0 | yes | nothing |
| `CUDA` | 0 | yes | nothing |
| `HIP` | 1 | no | `Found no Cycles device of the specified type` |
| `ONEAPI` | 1 | no | `Found no Cycles device of the specified type` |

The exit statuses and the frames are identical in all four versions. In **5.0, 5.1 and 5.2** `HIP` also brings out, ahead of the refusal, `cycles | WARNING HIPEW initialization failed: Error opening HIP dynamic library`; `ONEAPI` brings out the refusal alone. *(Changed by the citation check, 2026-08-07. This used to say the whole result was identical in all four versions. It is not: in 4.5 that warning is not printed at all in this run — the whole of its output is the banner, the blend-read line, one progress line and `Error: Found no Cycles device of the specified type`, with the error stream at 0 bytes. In 4.5 the warning appears only under `--debug-cycles`, and then on the error stream. The exit statuses and the frames are identical in all four, and were repeated in all four.)*

**This is the comparison 2026-08-06 could not make.** Then, every backend on this machine gave the same refusal, so the message could not be told apart from *the switch never works here*. Now the same machine on the same day gives exit 0 for two backends and the refusal for the other two — so `Found no Cycles device of the specified type` really does mean *there is no hardware for this backend*, and a success really is silent. A run that worked and a run that was refused differ in the exit status and in whether a frame exists, and in nothing else worded.

**What a graphics card running out of memory does is still not answered — and the reason is now measured rather than guessed at.** The card cannot be filled on this machine, because the machine's own memory gives out first. A grid of 72 million triangles rendered on the card without complaint: exit 0, a frame written, the card's memory peaking at 5,053 MiB of 16,376, and Cycles reporting `Mem: 3990M`. A grid of 128 million triangles never reached the card at all — the operating system killed Blender while it was still building the mesh, at `Started bpy.ops.mesh.primitive_grid_add(x_subdivisions=8000, y_subdivisions=8000, size=20)`:

```
Out of memory: Killed process 478499 (blender) total-vm:43362104kB, anon-rss:40221084kB, file-rss:1248kB, shmem-rss:0kB, UID:0 pgtables:83200kB oom_score_adj:0
```

That is the machine's 39 GB running out, not the card's 16 GB. Blender needs several times more of the machine's memory to build a scene than the card needs to hold it, so on a machine with this ratio the card cannot be pushed over by geometry. Nor by the frame: a 30000 by 30000 render with automatic tiling switched off (`scene.cycles.use_auto_tile = False`) **still tiled**, reporting `Rendered 0/225 Tiles`, so the render buffer never grew large enough either. Two more images were tried and are recorded in the list.

What the runs did find is the announcement of the facility, printed before any render begins, in all four versions:

```
Mapped host memory limit set to 38,013,009,920 bytes. (35.40G)
```

So Blender says up front how much of the machine's memory it is willing to lend the card. Whether it ever says anything when it starts using it is not settled here, and cannot be on this machine.

**Where I looked.** Searched the whole 5.2 manual and API for: `fall back`, `fallback`, `falls back`, `out of memory`, `insufficient memory`, `not enough memory`, `is not supported`, `are not supported`, `not available`, `an error is`, `error is reported`, `reports an error`, `will report`, `exit code`, `exit status`, `return code`, `missing`. Opened in full: the Cycles GPU Rendering page (including its whole Error Messages section), the EEVEE Limitations page, the Crashes page, the Output properties page, the Rendering Animations page, the Supported Image Formats page, the Preferences System page. Read `bpy.types.RenderEngine`'s reporting methods. Read `cycles_5.2/engine.py` and `cycles_5.2/__init__.py` in full, `cycles_5.2/operators.py` in full, and the whole `CyclesPreferences` block of `cycles_5.2/properties.py` including every `col.label` message. Grepped `cycles_5.2/ui.py` for report and label calls with warning-shaped words and found one unrelated hit ("No output node", line 1557). Ruled out `cycles_5.2/source` and `cycles_5.2/shader` — kernel and compiled-shader files, no host-side error handling.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**A device Cycles does not know, and backends with no hardware behind them.** Seven runs, the output file deleted before each:

| `--cycles-device` | exit | frame written |
|---|---|---|
| `NOTADEVICE` | 1 | no |
| `OPTIX` | 1 | no |
| `CUDA` | 1 | no |
| `HIP` | 1 | no |
| `ONEAPI` | 1 | no |
| `METAL` | 0 | yes |
| `CPU` | 0 | yes |

Identical in all four versions. **This is the mirror image of the Linux table**, where `METAL` was the one that failed and the NVIDIA routes worked once that machine's card was mended.

The wording, on **standard output**, for the unknown name:

```
cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

and for a real backend with no hardware behind it — `OPTIX`, `CUDA`, `HIP` and `ONEAPI` all alike:

```
reports          | ERROR Found no Cycles device of the specified type
```

**One line the Linux runs carried is missing here.** On Linux the same case printed a `cycles | ERROR CUDA cuInit: Unknown error` line above the report. On this machine there is no such line for any of the four absent backends — only the one sentence.

**A scene set to GPU Compute with no Cycles device switched on renders anyway.** The Cycles preference set to `NONE` and every device unticked, the scene still asking for GPU Compute:

```
SET_NONE ok
PREF_COMPUTE_DEVICE_TYPE NONE
DEVICES_IN_USE [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', False)]
SCENE_CYCLES_DEVICE GPU
```

Exit 0, a frame written, the error stream 0 bytes, in all four versions. In this version the device line says plainly what happened — `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]` — while in 4.5 on the same machine nothing at any log level does.

**An unknown engine name is refused.** `-E NOT_AN_ENGINE`: no frame, exit 1, and on the **error** stream:

```
Error: engine not found 'NOT_AN_ENGINE'
```

Blender 4.5 on this machine gave the same message and exited **134**, with `libc++abi: terminating` after it.

**A render that cannot write its output.** A plain file put where the output folder should be. Exit **1**, and two lines on standard output, from two different categories:

```
image            | ERROR Couldn't create directory for file /tmp/smoozy-war/out/e20/fail/notadir/z_0001.png: File exists
reports          | ERROR Render error (File exists) cannot save: '/tmp/smoozy-war/out/e20/fail/notadir/z_0001.png'
```

Blender 4.5 on this machine gave one line and different wording — `(Not a directory)` here against `(Invalid argument)` in the same test on Linux. The words in the brackets are the operating system's, not Blender's.

**An invalid `-F` name complains and renders anyway.** Exit 0, a file written in the scene's own format, and this on the **error** stream in all four versions:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
```

**A format that cannot hold metadata says nothing.** TIFF and TGA, every metadata switch on: exit 0, the file written, and no line in either stream matching `warn`, `error`, `metadata`, `not supported` or `preserved` — zero, in all four versions and both formats.

**"No compatible GPUs found for Cycles" could not be reached, and the panel shows why.** Blender started with a window, Preferences ▸ System shown and photographed by Blender's own screenshot operator. The Cycles Render Devices panel offers two buttons, `None` and `Metal`, with `Metal` chosen; under them a list holding `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked, then `Kernel Optimization: Full` and `MetalRT: Auto`. Setting the preference to `None` leaves the two buttons and **nothing else at all** — no list, no message. There is no CUDA, OptiX, HIP or oneAPI button on this machine, so there is no way to put the panel into the state that message belongs to. Pictures at `/tmp/smoozy-war/out/e28/prefs_5.2.png` and `/tmp/smoozy-war/out/e28/prefs_none_5.2.png`.

---

## 5. Where can each of the things above be learned from — what a render prints while it works, what it leaves on disk, or what can be asked afterwards?

**LABEL: mixed — documented for what exists in each place, not in the sources for what a render prints.**

**What a render prints while it works.** The sources establish that there is printed output and where it goes, but never quote a single line of it.

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors."
> "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."

Source: `https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html`. Date read 2026-07-30.

What can be turned on, in that stream:

> "--log <match> — Enable logging categories, taking a single comma separated argument."
> "--log "render,cycles": both render and cycles messages."
> "--log-level <level> … info: Information about devices, files, configuration, operations"
> "--log-file <filepath> — Set a file to output the log to."
> "--log-list-categories — List all available logging categories for --log, and exit."
> "--log-show-memory — Show memory usage for each log message."
> "-q, --quiet — Suppress status printing (warnings & errors are still printed)."

Source: `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`. Date read 2026-07-30. `--log-file` matters here: the printed stream can be turned into a file on disk, so "prints while it works" and "leaves on disk" are not separate places if you ask for a log file.

Cycles adds one of its own:

> "--cycles-print-stats — Log statistics about render memory and time usage."

Same page and date. **Observed**, the same switch in Cycles' files, with its own wording:

> `    parser.add_argument("--cycles-print-stats",`
> `                        help="Print rendering statistics to stderr",`
> `                        action='store_true')`

File `cycles_5.2/engine.py`, lines 13–15. Date read 2026-07-30. Note the difference: the manual says "Log statistics about render memory and time usage", Cycles' own file says "Print rendering statistics to stderr". The second is more specific and names the stream.

And there is a documented description of the running progress text, from the handler that receives it:

> "bpy.app.handlers.render_stats — on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

Source: `https://docs.blender.org/api/5.2/bpy.app.handlers.html`. Date read 2026-07-30. That is the closest the sources come to saying what a render prints: current frame, current sample, render time, saving time. No device, no engine, no add-ons.

**What it leaves on disk.** Four things, all documented.

1. The image, with the metadata fields of question 1, subject to the format supporting metadata (question 4, case I). Blender can read those back itself: "Displays metadata about the selected Render Result" (`https://docs.blender.org/manual/en/5.2/editors/image/navigating.html`), and for a file on disk, the Video Sequencer's Metadata tab — "Lists information that has been encoded in the currently visible movie or image file … This also works for images produced by Blender". The same page names the outside route: "The metadata can't be edited from Blender. Instead, you can use an external program such as exiftool." (`https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html`). Date read 2026-07-30 for all three.

2. A cached multi-layer OpenEXR, if asked for: "Saving – Cache Result — Saves the rendered view layers and their passes to a multi-layer OpenEXR image. … The image is stored in the Render Cache folder as specified in the File Paths Preferences." (`https://docs.blender.org/manual/en/5.2/render/output/properties/output.html`, date read 2026-07-30).

3. `system-info.txt`, on request — the GPU, Cycles device and Enabled Add-Ons sections quoted under questions 1 and 3.

4. A crash file, if it dies — quoted under question 4, with its three per-system locations.

5. A log file, if `--log-file` was given.

**What can be asked afterwards.** Everything in the running Blender's own data: `scene.render.engine`, `scene.render.threads` and `threads_mode`, the sixteen stamp switches, `bpy.context.preferences.addons`, and the Cycles device preferences. Plus the build's own facts:

> "bpy.app.build_platform — The platform this blender instance was built for"
> "bpy.app.version — The Blender version as a tuple of 3 numbers (major, minor, micro). eg. (4, 3, 1)"
> "bpy.app.version_string — The Blender version formatted as a string"
> "bpy.app.build_options — Constant value bpy.app.build_options(bullet=True, … cycles=True, cycles_osl=True, … image_openexr=True, … )"

Source: `https://docs.blender.org/api/5.2/bpy.app.html`. Date read 2026-07-30. `build_options` is the direct answer to "what can this installed Blender provide" — the documented sample value lists `cycles`, `cycles_osl`, `image_openexr` and thirty-odd others as true or false.

And from the command line, before or instead of a render: "-v, --version — Print Blender version and exit", "-E help to list available engines", "-c help to list all other commands", "--log-list-categories … and exit" (same arguments page, date read 2026-07-30).

**Mapping this back to questions 1–4, plainly:**

- Which parts of the computer (Q1): *on disk* — render time, peak memory and hostname in the image; *asked afterwards* — the configured thread count and the configured Cycles device; *on disk on request* — the available devices, in `system-info.txt`. **Not in the sources**: any route, in any of the three places, that names the device a finished render actually used.
- Which engine (Q2): *asked afterwards* only, from `scene.render.engine`. **Not in the sources**: any printed or on-disk record.
- Which add-ons (Q3): *asked afterwards* from `preferences.addons`; *on disk on request* in `system-info.txt`. **Not in the sources**: anything tied to the render.
- Failure (Q4): *printed* to the console; *asked afterwards* through the Info editor while the session lives; *on disk* only via `--log-file` or the crash file.

**Not covered.** The exact text of anything a render prints. Whether the Info editor's contents can be read by a script. Whether `system-info.txt` can be produced without an interface. What the Render Cache folder's file is named.

**Outside the interface.** This is where the three places come apart. The printed stream and `--log-file` are the command-line case by definition and are the strongest option there. The on-disk items are all files, so they survive with no window — except the two reading routes for image metadata (Image editor overlay, Video Sequencer Metadata tab) which are both panels; the sources name `exiftool` as the outside-Blender way to read the same fields. The Info editor and the status bar exist only with a window. **Settled by test on 2026-08-06, on Linux only — see below.** With no window, everything a render prints goes to standard output and the error stream stays empty, apart from the `-F` complaint and, in 4.5 only, the device and `--cycles-device` messages. `--cycles-print-stats` goes to standard output, not the error stream. There is no route from Blender's Python to a written file's metadata in any version, with or without a window. **Added by the tests of 2026-08-07:** the printed route is the one that names the card, and it works with no window. `--cycles-print-stats` replaces its profiling sections with `Profiling information not available (only works with CPU rendering)` when the card did the work. The on-disk route names no card at all. **Added by the re-run of 2026-08-07, after the citation check:** nothing in these three tests needs a window, and all three were run on a machine that has no display server at all.

**Operating system.** The three places are described without operating-system qualification, except the crash file's location (three different answers, quoted in question 4) and the Windows console being a separate window that is toggled off by default (`https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html`, date read 2026-07-30). **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not. **Added by the re-run of 2026-08-07, after the citation check:** the three tests sent back by the citation check were run on that same Linux machine and nowhere else, so they are an NVIDIA-on-Linux answer; macOS and Windows are untouched by them.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.2`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

These runs close the biggest hole this answer names — the exact shape of what a render prints.

**Which stream everything lands on.** With the two streams captured to separate files, a plain background render put **everything on standard output and left the error stream empty** — 0 bytes. So did a run with `--cycles-print-stats`, and so did the `Path tracing on:` line under every switch that produces it. The exceptions found are the `-F` format complaint (question 4) and, in Blender 4.5 only, the device and `--cycles-device` messages; those go to the error stream.

**What `--cycles-print-stats` prints, and where.** Run as `blender -b scene.blend -o out/s_ -F PNG -f 1 -- --cycles-print-stats`, exit 0. **The statistics went to standard output, and the error stream was 0 bytes** — the opposite of the "to stderr" that Cycles' own file says, and the two descriptions the answer above says are unreconciled are settled in favour of neither: it is standard output. They begin with a per-stage timing block (`Scene`, `Geometry`, `Light`, `Object`, `Image`, `Background`, `Bake`, `Camera`, `Film`, `Integrator`, `OSL`, `Particles`, `SVM`, `Tables`, `Procedurals`) and continue into:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 386B (386)
        Cube                             289B (289)
        backgroundlight                  50B (50)
        Light                            47B (47)
Image statistics:
  Total image memory: 0B (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.34s), Self 13.10% (0.04s)
    Ray setup                       : Total 29.76% (0.10s), Self 29.76% (0.10s)
    Intersect Closest               : Total 23.81% (0.08s), Self 23.81% (0.08s)
    Shade Surface                   : Total 22.32% (0.07s), Self 0.00% (0.00s)
...
Shader statistics:
  Material                        : 0.07s (Relative cost: 1.00)
Object statistics:
  Cube                            : 0.07s (Relative cost: 1.00)
```

(The kernel block is thirty lines long and is kept whole in the hand-tests file; the four quoted here are the largest and the shape is the same throughout.) **No statistic names a device.** Searching the whole captured output for `device`, `CUDA`, `OptiX`, `Metal`, `HIP`, `oneAPI`, `GPU`, `CPU`, `EPYC`, `NVIDIA` and `GeForce` matched only the ordinary render lines that carry the word *device* without naming one, and the `device_update` timing line.

**`--log-list-categories` exists in this version and lists 183 categories.** Exit 0. The list opens `addon`, `anim`, `anim.action`, `anim.data`, `anim.driver`, and the categories bearing on this topic are `cycles`, `render`, `hydra.render`, `compositor.gpu`, `image.gpu` and a family of `gpu.*` entries. There is an `addon` category. The device line lives under `cycles` at `info` level — see question 1. (In Blender 4.5 the switch does not exist at all; that is recorded in 4.5's own document.)

**`--log-level` takes a name in this version.** `--log-level info` was accepted and the render exited 0. (Blender 4.5 takes a number and refuses the name; recorded in 4.5's document.)

**A written render's metadata cannot be read back from Blender's own Python.** Run as `blender -b --python-expr` loading one of the stamped PNGs:

```
LOADED (160, 120)
HAS_metadata_attr False
stamp-ish attrs: []
```

The image loads and reports its size, so Blender read the file; there is no route from Python to the fields inside it. This closes the *not in the sources* recorded above.

**What `system-info.txt` holds when written with no window** is under questions 1 and 3 — the operator works, but the GPU section reads `GPU: missing, background mode`.


### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.2`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**What a render prints while it works** gained four things, all of them new because the card works.

- The `Path tracing on:` line names the card, on **standard output** as before (question 1).
- `Building OptiX acceleration structure` appears in the ordinary progress lines, with no switch at all — OptiX only, never CUDA.
- The memory figure in the progress lines reads `661M` where the processor's run read `1M`.
- `Mapped host memory limit set to 38,013,009,920 bytes. (35.40G)` is printed before the render starts, under a log switch. *(Qualified by the citation check, 2026-08-07: the line and the figure reproduced exactly in all four versions, but it needs `--debug-cycles` in 4.5 and `--log-level info` in 5.0, 5.1 and 5.2. A run with no switches never prints it.)*

**`--cycles-print-stats` prints something different when the card did the work, and the difference is itself a sign.** The statistics still go to **standard output** and the error stream is still empty (0 bytes) in every version, exactly as on 2026-08-06. The Mesh and Image sections are unchanged. But where the processor's run printed a Kernel statistics tree, a Shader statistics line and an Object statistics line, the card's run prints one sentence in their place:

```
Image statistics:
  Total image memory: 0B (0)
Profiling information not available (only works with CPU rendering)
```

So this switch answers *which part of the computer did the work* by what it refuses to print, and it names no device while doing it.

**What it leaves on disk** gained one thing and lost none. No field in a PNG or an OpenEXR names the card (question 1). The `Memory` stamp field carries the card's figure rather than the processor's, which is a hint and not a name. And the Render Time pass — see question 6 — turns out to record the scene's setting rather than what ran, so it is not a route to this at all.

**What can be asked afterwards** gained two routes and kept one trap. `bpy.ops.wm.sysinfo` now describes the card in full in its Cycles section, and the Cycles preferences list it by name before any render (question 1). In 5.2, `gpu.init()` followed by `gpu.platform.renderer_get()` is a second route, and returns `NVIDIA GeForce RTX 4080 SUPER/PCIe/SSE2` with no window at all. The trap is unchanged: the settings read back after a run say what was asked for, not what happened.

**Where I looked.** Searched the whole 5.2 manual and API for: `console`, `terminal`, `stdout`, `stderr`, `log`, `log-file`, `print`, `Loading render kernels`, `compiling kernels`, `Synchronizing`, `Updating Scene`, `Waiting for render`, `Saved:`, `render statistics`, `statistics`, `EXIF`, `exiftool`. The five progress-text guesses returned nothing at all, which is what makes "the sources never quote what a render prints" a real answer rather than an assumption. Opened in full: the Command Line index, the Command Line Arguments page, Rendering From The Command Line, the Windows launch page, the Info Editor page, the Status Bar page, the Image editor Navigating page, the Video Sequencer Preview Sidebar page, the Output properties page, the Crashes page. Read `bpy.app.handlers` in full for the render handlers, and `bpy.app`'s build and version entries.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**Which stream everything lands on.** In this version, every one of these went to **standard output** and left the error stream at 0 bytes: the ordinary progress lines, the `Engine:` line, the `Path tracing on:` device line at `--log-level info`, the whole `--cycles-print-stats` block, the `Found no Cycles device` refusal, and the two-line failure when a render cannot save. The only things that reached the **error** stream were the `-F` complaint and the `engine not found` refusal.

**`--log-list-categories` works, and the list is shorter on macOS with different graphics categories.** Exit 0 and **149 categories** on this machine, against 183 for the same version on Linux. The graphics ones here:

```
compositor.gpu
cycles
gpu.debug.metal
gpu.framebuffer
gpu.metal
gpu.mipmap
gpu.platform
gpu.shader
gpu.shader.deps
gpu.texture
hydra.render
render
```

The Linux list carried `gpu.debug`, `gpu.opengl` and `gpu.vulkan` where this one carries `gpu.debug.metal`. **So the category list is a fact about the operating system as well as the version**, and a plan that hard-codes a category name from one machine will not find it on the other. `addon`, `cycles` and `render` are in both.

**`--log-level` takes a name.** `--log-level info` accepted, exit 0. In 4.5 on the same machine it is refused with `Error: not a number '--log-level info'.` on the error stream, and the render then goes ahead anyway, exit 0.

**A written render's fields cannot be read back from Blender's Python.** Identical in all four versions:

```
LOADED (160, 120)
HAS_metadata_attr False
stamp-ish attrs: []
```

**And there is no Metadata panel in the Image editor to read them from either.** The Image editor's sidebar was opened on a stamped PNG loaded from disk. Its tabs are `Tool`, `Image`, `View`, `Scopes` and `Mask` — there is no Metadata tab. Asking the running Blender for every sidebar panel the Image editor defines gives, for the `Image` tab, exactly `Image`, `Render Slots` and `UDIM Tiles`, and **no panel anywhere in the Image editor carries a label containing "metadata"** — the only two panels in the whole of Blender that do are the Asset Browser's and Output Properties' own. Identical in all four versions. A picture of the open sidebar is at `/tmp/smoozy-war/out/e29/img_5.2.png`. The sidebar tab could not be switched from a script (`Region.active_panel_category` is read-only), so the panel list was read instead of photographed.

**The render handlers fire in one order and the frame is already on disk at `render_write`.**

```
HANDLER render_init      args=['Scene', 'NoneType']
HANDLER render_pre       args=['Scene', 'NoneType']
HANDLER render_post      args=['Scene', 'NoneType']
HANDLER render_write     args=['Scene', 'NoneType'] | file exists now: True
HANDLER render_complete  args=['Scene', 'NoneType']
```

`render_cancel` did not fire; all six handlers exist; every handler is handed two arguments, not one. `render_stats` fired **44** times with no window. First and last:

```
HANDLER render_stats     args=('Mem: 0M | Synchronizing object | Cube', None)
HANDLER render_stats     args=('Time: 00:00.08 (Saving: 00:00.05)', None)
```

**With a window open, only 5.2 keeps handing the same strings over.** The same render, driven by `bpy.ops.render.render(write_still=True)` from a timer inside a Blender started with a window:

| version | `render_stats` calls, no window | `render_stats` calls, window open |
|---|---|---|
| 4.5 | 42 | **1** |
| 5.0 | 43 | **1** |
| 5.1 | 43 | **1** |
| 5.2 | 44 | **44** |

The one string the older three hand over with a window open is the closing time line. **The control matters and was run**: the same operator-driven render with no window gave 42, 43, 43 and 44, so the difference is the window and not the way the render was started. In 5.2 the windowed strings are the same forty-four, in the same order, as the windowless ones.

**Each frame of a background animation is announced before it is rendered.**

```
render           | Rendering frame 1
render           | Saved: '/tmp/smoozy-war/out/e19/an_0001.png'
render           | Rendering frame 2
render           | Saved: '/tmp/smoozy-war/out/e19/an_0002.png'
render           | Rendering frame 3
render           | Saved: '/tmp/smoozy-war/out/e19/an_0003.png'
```

**A render asked to stop leaves nothing behind, and this corrects the set-up used on Linux.** The Linux run of 2026-08-06 set 100000 samples and killed the run after six seconds, and read a complete image and exit 0 as the answer. Run again here, the same set-up **never reached six seconds**: with adaptive sampling on, all four versions converged at `Sample 960/100000` and finished by themselves at about five seconds, exit 0, a complete valid image, and `cycles.ViewLayer.samples = 100000` inside it. That is a render that finished, not one that was stopped, so it says nothing about interruption. Repeated with `use_adaptive_sampling = False` so the render really was still going at six seconds:

```
still rendering at six seconds: yes
exit after SIGTERM=143
no file at output path
```

Identical in all four versions. **So a render stopped part-way through writes no file and ends with status 143**, and the earlier reading — a complete file and exit 0 — describes a render that converged, not one that was interrupted.

---

### Tested by hand again after the citation check, 2026-08-07 — **LABEL: observed**

**Why these runs happened.** The citation check of this topic found four disagreements, recorded both sides of each, chose between none of them, and sent them back to the hand tests. Three of the four are tests. They were run on 2026-08-07 on the rented Linux machine, alone and last, because that machine was about to be taken away and nothing else could be asked of it afterwards. They are entries 42, 43 and 44 of `Audits/Blender Research/What actually ran/Hand tests.md`.

**The machine, named in full.** The rented Linux machine: Ubuntu 22.04.5 LTS, kernel `6.8.0-59-generic`, one NVIDIA GeForce RTX 4080 SUPER with 16376 MiB of graphics memory, driver **580.173.02** in both halves — `nvidia-smi` and `/proc/driver/nvidia/version` were read at the start of the sitting and agree — an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all. Nothing else was running on it: the load average was 0.19 and there was no Blender process. Everything was written inside `/root/again-what-actually-ran/`, and each of the four Blenders was given a settings folder of its own, made before it was used. The scene is the one-cube factory scene at 160 by 120, sixteen samples, denoising off, built by Blender 4.5 so that all four versions can open it. No test below reads what is written inside a finished image, so no picture and no file was compared.

**The third of the three tests answers this question.** The other two are written out under question 1: what the `Full path tracing report` block holds in this version, and how many `Building OptiX acceleration structure` lines a run prints. Both bear on this question too, because both are about what a render prints while it works: the first is reached only by a log switch, the second needs no switch at all.

---

#### Test three — whether this version's log-category list carries `gpu.opengl`

*What a negative result would look like, said before the run.* `gpu.opengl` absent from this version's list. That is a live outcome: Blender 5.0 was run identically alongside and its list has no such entry.

*What was done.* `blender -b --log-list-categories`, three times per version, both streams captured, and the output searched for a line reading exactly `gpu.opengl`.

*What came back.* **This version's list carries `gpu.opengl`.** Exit 0, three runs, identical each time; 183 lines, of which the first is the version banner, so **182 categories**. Every category whose name contains `gpu`, in full:

```
compositor.gpu
gpu.debug
gpu.framebuffer
gpu.mipmap
gpu.opengl
gpu.platform
gpu.shader
gpu.shader.deps
gpu.texture
gpu.vulkan
image.gpu
```

*Where the disagreement was.* It was in the 5.0 documents, over whether the Linux 5.0 list carries `gpu.opengl`. These runs are that question's control: on the same machine on the same day, 5.0 has no such entry and 5.1 and 5.2 both have one, so the entry arrives at 5.1. Blender 4.5 has no such switch at all.


## 6. At what point can each of the things above be learned — before a render starts, while it runs, or only once it has finished?

**LABEL: mixed — documented for the handler timings, inferred for the rest.**

**The documented timings.** Blender names seven moments around a render, and this list is the backbone of the answer:

> "bpy.app.handlers.render_init — on initialization of a render job. Accepts one argument: the scene data-block being rendered"
> "bpy.app.handlers.render_pre — on render (before). Accepts one argument: the scene data-block being rendered"
> "bpy.app.handlers.render_stats — on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."
> "bpy.app.handlers.render_post — on render (after). Accepts one argument: the scene data-block being rendered"
> "bpy.app.handlers.render_write — on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"
> "bpy.app.handlers.render_complete — on completion of render job. Accepts one argument: the scene data-block being rendered"
> "bpy.app.handlers.render_cancel — on canceling a render job. Accepts one argument: the scene data-block being rendered"

Source: `https://docs.blender.org/api/5.2/bpy.app.handlers.html`. Date read 2026-07-30.

**Inferred**, one step, and I write it out: since each of those hands over the scene data-block, and since the things askable afterwards in question 5 (`scene.render.engine`, `scene.render.threads`, the stamp switches, and — through `bpy.context.preferences` — the add-on list and Cycles' device preferences) are all properties of that scene or of the preferences, they can be read at every one of those seven moments as well as before and after. The step a reader supplies: that a handler receiving the scene can reach the rest of the data through it and through `bpy.context`. The sources describe the handlers' arguments; they do not spell that out.

**One thing is documented as deliberately late:**

> "error_set(message) — Set error message displaying after the render is finished"

Source: `https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html`. Date read 2026-07-30. So at least one class of failure message is, by design, only visible once the render is over.

**Taking the earlier questions one at a time:**

- **Q1, configured device and thread count** — before, during and after. They are settings, and settings exist from the moment the file is open. **Inferred** as above.
- **Q1, available devices** — before, at any time. The device list is queried on demand. **Observed**: `def get_device_list(self, compute_device_type): … device_list = _cycles.available_devices(compute_device_type)` (`cycles_5.2/properties.py`, lines 1709–1711), with a warning that the timing is not free: `# Be careful when deciding when to call this function, as Blender can crash with _cycles.available_devices() on some drivers.` (lines 1707–1708). And **observed**, that it is not done automatically at startup: `# Refresh device list. This does not happen automatically on Blender startup due to unstable drivers that can cause crashes.` (lines 1763–1764). Date read 2026-07-30. That is a documented reason to expect the list to be empty until something asks for it.
- **Q1, render time, peak memory, hostname** — only once the frame is done, because they are written into the image as it is saved. **Inferred**, one step: the Metadata panel is described as "options for writing metadata into render output", and Render Time is by nature not final until the render is. The step: I am reading "the render time is written into the output" as "it cannot be known before the render ends". Peak memory is the same shape. Hostname is not — the machine's name is knowable at any time; only its *presence in the file* waits for the save.
- **Q1, `system-info.txt`** — whenever asked. It is an operator, not a stage of a render.
- **Q2, the engine** — before, during and after, from the scene setting; and before the run even starts, from `-E` on the command line.
- **Q3, add-ons** — before, during and after, from the preferences; before the run starts, from `--addons`; and at any moment, into a file, from `bpy.ops.wm.sysinfo`.
- **Q4, failures** — spread across all three. Before: "No compatible GPUs found for Cycles" is drawn in the preferences, with no render involved (`cycles_5.2/properties.py`, line 1866, **observed**). Before, and fatally: the `sys.exit(1)` on a rejected `--cycles-device` happens in `_parse_command_line()`, which is called from `init()` (`cycles_5.2/engine.py`, lines 44–53, **observed**) — that is add-on registration time, long before any frame. During: out-of-memory and driver errors, by their nature. After: anything set with `error_set`. And whenever it happens: the crash file.

**Not covered.** Whether the metadata written into a frame can be inspected before that frame is written. Whether `render_stats` fires at a fixed interval or per sample or per frame — the sources say only "on printing render statistics". Whether `render_complete` fires per frame or per job in an animation (the wording is "render job", but the sources do not settle it against `render_write`, which is explicitly per frame). Whether any of the seven handlers fire when there is no window.

**Outside the interface.** The handlers are Python and the sources do not qualify them by interface. Two of the "before" routes — `-E`, `--addons`, `--cycles-device`, `-v`, `-E help`, `--log-list-categories` — exist only from a command line, which is the outside case. The "before" route that is *only* inside is the preferences message in case C of question 4: it is a drawn label, so on a machine with no window nothing shows it, and **not in the sources** is whether the same condition is announced any other way. **Settled by test on 2026-08-06, on Linux only — see below.** All six render handlers fire with no window, in the order init, pre, post, write, complete, with the frame already on disk when `render_write` fires; `render_cancel` does not fire on a finished render. `bpy.app.is_job_running` works with no window and does not crash. So the inferred step above holds. **Added by the tests of 2026-08-07:** which parts of the computer are available can now be learned before a render starts, from outside the window; which part did the work can only be learned once the render has finished, from a closing report.

**Operating system.** Nothing about the timings is qualified by operating system in any of the sources I read. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.2`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**The handlers do fire with no window, and this is the order they fire in.** A script was attached to `render_init`, `render_pre`, `render_post`, `render_write`, `render_complete`, `render_cancel` and `render_stats`, and the output file was deleted first so that checking for it means something:

```
HANDLER render_init      t=0.000 args=['Scene', 'NoneType']
HANDLER render_pre       t=0.001 args=['Scene', 'NoneType']
HANDLER render_post      t=0.144 args=['Scene', 'NoneType']
HANDLER render_write     t=0.144 args=['Scene', 'NoneType'] | file exists now: True
HANDLER render_complete  t=0.144 args=['Scene', 'NoneType']
```

Three things this settles. **`render_post` fires before `render_write`**, which this answer records as unknown. **The frame is already on disk when `render_write` fires** — `file exists now: True` — so the metadata reaches disk at or before that moment, which is the step marked inferred above. And **`render_cancel` did not fire**, which is what a finished render should give. All six handlers exist; none reported itself missing.

**Every handler is handed two arguments, not the one the documentation describes.** The scene-level ones get the scene and a second argument of type `NoneType`; `render_stats` gets a string and a second argument of `None`.

**`render_stats` fired 44 times for one frame**, so it is not once per frame. Its first and last strings:

```
HANDLER render_stats     args=('Mem: 0M | Synchronizing object | Cube', None)
HANDLER render_stats     args=('Time: 00:00.14 (Saving: 00:00.10)', None)
```

**What it is handed does not match either description in the sources.** There is no frame number and no peak-memory figure in the string, and no sample count; the frame number lives in the log line's own prefix (`render | Fra: 1 | ...`), which the handler never sees. Blender 4.5 hands over a longer string that does carry the frame, the running memory and the peak — that comparison was run on the same machine on the same day and is in 4.5's document.

**Each frame of a background animation is announced before it is rendered.** Run as `blender -b scene.blend -o out/an_ -F PNG -s 1 -e 3 -a`:

```
00:00.289  render           | Rendering frame 1
00:00.457  render           | Saved: '/root/what-actually-ran/out/hd/an_0001.png'
00:00.457  render           | Rendering frame 2
00:00.484  render           | Saved: '/root/what-actually-ran/out/hd/an_0002.png'
00:00.485  render           | Rendering frame 3
00:00.507  render           | Saved: '/root/what-actually-ran/out/hd/an_0003.png'
```

**Asking whether a render is under way works with no window, and does not crash.** Run as `blender -b --python-expr "import bpy; print('IS_JOB_RUNNING', bpy.app.is_job_running('RENDER'))"`, exit 0:

```
IS_JOB_RUNNING False
```

The installed build is 5.0.1 or later, which is the release a corrective note says mended this, so it says nothing about 5.0.0.

**A value read after a render need not be the one in force during it.** A run overridden with `-- --cycles-device CPU`, whose scene said GPU Compute, printed from inside itself `AFTER_OVERRIDE compute_device_type= NONE  scene.cycles.device= GPU` while its `Path tracing on:` line named the processor. So the settings and what ran can disagree, and only the printed line tells them apart.

**The Render Time pass follows the setting, not the hardware.** With the pass switched on and the output set to multi-layer OpenEXR, two runs on a machine where **both** did their work on the processor:

```
  device=CPU layers: ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R', 'ViewLayer.Render Time.X']
  device=GPU layers: ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R']
```

So a file carrying the layer tells you the scene said CPU, and tells you nothing about what actually ran. Whether a working graphics card changes that is untested. (The property does not exist at all in Blender 4.5; recorded in 4.5's document.)


### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.2/blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.2`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**Before a render starts** — which parts of the computer are *available* is now answerable, and it was not on 2026-08-06. `refresh_devices()` and the Cycles preferences name the card with no render and no window; `bpy.ops.wm.sysinfo` describes it in full. What is still not answerable beforehand is which of them will do the work: `compute_device_type` reads `NONE` at factory startup with the card sitting there working, and the command line can override whatever the scene says.

**While it runs** — two signals arrive early. `Mapped host memory limit set to 38,013,009,920 bytes. (35.40G)` comes before the scene is even synchronized, and the memory figure in the progress lines is already the card's by the first sample line (`Mem: 661M | Sample 0/16`). The OptiX name arrives in the progress lines during the geometry step.

**Only once it has finished** — the `Path tracing on:` line, which is the only one that names the card, is a closing report. It is printed after the last sample and before the frame is written: in the 5.2 run, last sample at `00:02.270`, report at `00:02.270`, `Saved:` at `00:02.577`. So a run that is stopped part-way never prints it.

**The `Render Time` pass records the scene's setting, not what ran — settled.** On 2026-08-06 the pass appeared with the scene set to CPU and not with it set to GPU, on a machine where the processor did the work either way; that showed it followed the setting rather than the hardware, but could not show which setting. With the card working, six runs of the same frame to multi-layer OpenEXR settle it. `ran on` below is read from each run's own `Path tracing on:` line:

| the scene's device | `--cycles-device` | what actually ran | `ViewLayer.Render Time.X` in the file |
|---|---|---|---|
| CPU | none | the processor | present |
| CPU | `CPU` | the processor | present |
| CPU | `OPTIX` | **the card** | **present** |
| GPU | none | the card | absent |
| GPU | `CPU` | **the processor** | **absent** |
| GPU | `OPTIX` | the card | absent |

Identical in 5.0, 5.1 and 5.2. The two bold rows are the ones that decide it: the pass is written when the *scene* says CPU even though the card did the work, and withheld when the scene says GPU even though the processor did the work. **So this pass can never tell anybody, afterwards, which part of the computer did the work.** It reports an intention.

**Where I looked.** Read `bpy.app.handlers` in full, and `bpy.types.RenderEngine`'s `update_stats`, `update_progress`, `update_memory_stats`, `report` and `error_set` entries. Re-read `cycles_5.2/engine.py` for when `_parse_command_line` runs relative to `init`. Re-read the `CyclesPreferences` device-list functions in `cycles_5.2/properties.py` for their own comments about when they may safely be called. Searched the manual and API for `before rendering`, `during rendering`, `after rendering`, `render job`, `progress`. Found the Status Bar's "Running Task — Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate." (`https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html`, date read 2026-07-30) — an interface-only during-the-render route, included for completeness.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**Before a render starts.** With no window and `--factory-startup`, this machine's Cycles preference already reads `METAL` and the card is already listed and ticked:

```
COMPUTE_DEVICE_TYPE METAL
DEVICES [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]
```

**This differs from the Linux machine, where the same read gave `NONE`.** The list only fills in after `refresh_devices()` is called: read without that call in the same session, `p.devices` came back empty while `compute_device_type` still said `METAL`.

**And that difference decides what a render actually uses.** With a settings folder that has never had preferences saved, a scene set to GPU Compute path-traced on the **processor**, in the very same run in which Cycles printed that it had added the Metal device. With a settings folder in which preferences had been saved once, after `refresh_devices()`, the identical scene and command path-traced on the **card**. So on this machine, whether a GPU render reaches the graphics before it starts depends on the state of the settings folder, and nothing in the scene file says which it will be.

**The sixteen metadata switches at factory startup are the same as on Linux**, `use_stamp_memory` and `use_stamp_hostname` both False and the master `use_stamp` False, so none of them is written unless somebody turns the master on. Threads read `THREADS 8 THREADS_MODE AUTO`.

**The Render Time pass follows the scene's setting and not what did the work, shown here in the harder direction.** The Linux machine could only show it with the card unusable. On this machine the card works, so both halves are real:

```
  device=CPU layers: ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R', 'ViewLayer.Render Time.X']
                     Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
  device=GPU layers: ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R']
                     Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
```

The GPU run genuinely ran on the card and the layer is still absent. `pass_render_time` exists in 5.0, 5.1 and 5.2 and does not exist in 4.5.

**While it runs.** `render_stats` hands a string over dozens of times per frame with no window — 44 times for this scene in this version. **With a window open that stream of strings survives only in 5.2**; 4.5, 5.0 and 5.1 hand over one string, the closing time line. So a plan that watches `render_stats` to follow a render's progress works with no window in every version, and with a window only from 5.2.

**Only once it has finished.** The device report arrives after the last sample and before the frame is saved; `render_write` fires with the file already on disk; and the fields inside the image can be read by anything except Blender's own Python.

**`bpy.app.is_job_running` works with no window**, exit 0, no crash:

```
IS_JOB_RUNNING False
```

The installed 5.0 is 5.0.1 on this machine too, which is the release the corrective note says mended it, so this says nothing about 5.0.0.

---

## Things only a running Blender can settle

The list that stood here has been run. What was found is written into the answers above as observed findings, and the whole procedure — every command and every raw result — is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**Run and settled**, across three sittings: the rented Linux machine on 2026-08-06 with its graphics card unreachable, the same machine on 2026-08-07 with the card mended, and the Creator's Mac on 2026-08-07. Whether anything in a render names the device that did the work, on the processor and on a card; whether a written image carries a device or an engine field; what `--cycles-print-stats` prints and on which stream; what a scene set to GPU Compute does when no card can be used; what a rejected `--cycles-device` does; whether `system-info.txt` can be written with no window and what its sections hold, with and without a window; which log categories exist, on both operating systems; whether the frame counter appears during a background animation; whether an engine writes its own stamp fields; whether the Memory metadata box starts ticked, in the panel as well as in the settings; whether the Render Time pass follows the setting or the hardware; whether the `gpu` module answers with no window, and what `gpu.init()` does; whether `--addons` changes the list a script reads; whether the printed statistics differ with a window open; what the Cycles preferences show when the graphics work; whether the Image editor lists a saved file's fields; and what a render asked to stop leaves behind.

**Still open, and why.**

1. **Whether a graphics card running out of memory announces the move into system memory.** Tried on the mended Linux card and not answerable there — the machine's own memory gives out before the card's does. The Mac is no help either: its graphics memory is the machine's memory.

2. **What Blender 5.0.0 did**, as against 5.0.1. Both installed 5.0 builds are 5.0.1 on both machines, so the peak-memory field and the background-mode crash the corrective notes describe cannot be reached.

3. **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

4. **Whether the two build-dependent messages** — "OSL support disabled in this build" and the Embree lines — ever reach anybody. Every build on both machines reports `cycles_osl: True`, and `build_options` has no Embree entry at all, so neither can be provoked or even checked here.

---

2026-08-06 — Opus 5 (Claude Code) — What actually ran, Blender 5.2

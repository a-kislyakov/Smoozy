# What actually ran — Blender 5.2

**Sources consulted, and dates.** Blender's manual for 5.2, Blender's Python API documentation for 5.2, the 5.2 release notes, and Cycles' own files as shipped with 5.2 — all read on **2026-07-30** (the date the copies were taken). Blender's requirements page, read on **2026-08-01**. Pictures from the manual are named where I used them.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

A note on the requirements page: it never names 5.2 in so many words. The nearest it comes is a sentence about "Blender 5.0 and later". I have flagged that where I use it, and I have leaned on the manual instead wherever the manual says the same thing.

A note on the whole topic: Blender's documentation describes settings and features very well, and describes what a running render *prints, writes or reports* hardly at all. Several answers below are therefore "not in the sources", and the record of where I looked is given each time.

---

## 1. When a render has finished, what can be learned about which parts of the computer it used?

**Answer.** Almost nothing is recorded *by* the finished render. The manual sets out the metadata Blender can write into a rendered image as a list of fields, and none of the fields on that list names a processor, a graphics card or a rendering device. The only hardware-adjacent things on the list are the machine's hostname and the peak memory used. The manual does not say in so many words that this list is everything the metadata can hold, so the step from "not on the list" to "not written at all" is one I supplied — **inferred**.

The list of metadata fields, quoted:

> "Date … Includes the current date and time.
> Time … Includes the current scene time and render frame at HH:MM:SS.FF.
> Render Time … Includes the render time.
> Frame … Includes the frame number.
> Frame Range … Includes the start and end frame numbers.
> Memory … Includes the peak memory usage.
> Hostname … Includes the rendering machine's hostname.
> Camera … Includes the name of the active camera.
> Lens … Includes the name of the active camera's lens value.
> Scene … Includes the name of the active scene.
> Marker … Includes the name of the last marker.
> Filename … Includes the filename of the blend-file.
> Strip Name … Includes the name of the foreground sequence strip.
> Note … Includes a custom note."

— Blender manual, Metadata, <https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html>, read 2026-07-30.

I also opened the picture the manual shows on that page, `_images/render_output_properties_metadata_panel.png` (Metadata panel). The checkboxes visible in it are Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Use Strip Me…, plus Note, Burn Into Image, Font Size, Text Color, Background, Include Labels. No device, processor or graphics-card field is visible. **documented** (the picture is part of the manual).

There is one escape hatch, and it is the recommended one for exactly this purpose. The Note field is free text, and the manual says so:

> "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number."

— same page and date.

And a render engine can add its own stamp fields:

> "stamp_data_add_field(field, value) — Add engine-specific stamp data to the result"

— Python API, RenderResult, <https://docs.blender.org/api/5.2/bpy.types.RenderResult.html>, read 2026-07-30.

What *can* be learned separately from the render, by asking the running Blender, is what devices Cycles keeps settings for. Cycles' own preferences hold this:

> `devices: CollectionProperty(type=CyclesDeviceSettings)`

and each entry is:

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

— Cycles' own files, `cycles_5.2/properties.py`, lines 1614–1622 and 1662, read 2026-07-30. **observed**.

Alongside these, `CyclesPreferences` carries `compute_device_type` ("Device to use for computation (rendering with Cycles)", line 1655–1660) and the helpers `get_num_gpu_devices()`, `has_multi_device()` and `has_active_device()` (lines 1784–1810). **observed** that all of these exist under those names; what any of them holds or returns on a given machine is not visible in the file. The scene-level choice is a two-way switch only:

> ```
> enum_devices = (
>     ('CPU', "CPU", "Use CPU for rendering"),
>     ('GPU', "GPU Compute",
>         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
> )
> ```

— `cycles_5.2/properties.py`, lines 30–34, read 2026-07-30. **observed**. So the scene records "CPU" or "GPU Compute", and *which* GPU is a preference of the machine, not a property of the scene or of the render.

There is also a whole-machine report that Blender can write out on demand, which does name the hardware:

> "Save System Info — This extracts system information … You will be prompted to save a text file called system-info.txt. It contains the following sections: … GPU — Shows the GPU vendor, version and the capabilities of your hardware and driver. … Cycles — The instruction sets and capabilities of each hardware render device available for use with Cycles."

— Blender manual, Help, <https://docs.blender.org/manual/en/5.2/getting_started/help.html>, read 2026-07-30. **documented**.

That report describes the *machine*, not the render. It is a separate action, not something a finished render leaves behind.

**Label.** documented (metadata field list, system-info sections), observed (the Cycles files), inferred (that the quoted field list is everything the metadata can hold).

**Not covered.** This does not settle whether the *actual* device a finished render ran on can be recovered at all after the fact — only that the documented metadata does not carry it and the documented settings describe intent and availability rather than outcome. It also does not settle what a third-party render engine might add through `stamp_data_add_field`.

**Outside the interface.** The metadata field list is a property of the output, not of the interface, so it holds when Blender is driven from outside. The Cycles preference values are ordinary properties and readable from a script. Two things differ. First, the device can be forced from the command line, overriding both preference and scene: "--cycles-device <device> — Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. Append +CPU to a GPU device to render on both CPU and GPU." (manual, Command Line Arguments, <https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html>, read 2026-07-30) — so outside the interface the preference may not be the thing that ran. Second, "Save System Info" is described as a menu item; the equivalent operator exists (`bpy.ops.wm.sysinfo(*, filepath='')` — "Generate system information, saved into a text file", <https://docs.blender.org/api/5.2/bpy.ops.wm.html>, read 2026-07-30), but the sources do not say whether it runs in background mode. **Settled by test on 2026-08-06, on Linux only — see below.** `bpy.ops.wm.sysinfo` does work with no window, returning `{'FINISHED'}` and writing its file, so the inferred step above holds; but its GPU section then reads `GPU: missing, background mode`, so the route works and the hardware part of what it produces does not. The `gpu.platform` calls do **not** work with no window — they raise `SystemError` in every version. And the line naming the processor that did the work is reachable with no window, on standard output. **Added by the tests of 2026-08-07, with the machine's graphics card repaired:** driving Blender from outside its window is no obstacle to naming the card. With no window and no display server, a run prints the card's model in its `Path tracing on:` line, `bpy.ops.wm.sysinfo` describes the card in full in its Cycles section, and the Cycles preferences list it before any render. The GPU section of `system-info.txt` still reads `GPU: missing, background mode`, so that one section is genuinely lost outside the window and nothing else is. **Added by the re-run of 2026-08-07, after the citation check:** nothing in these three tests needs a window, and all three were run on a machine that has no display server at all.

**Operating system.** The metadata fields are not qualified by operating system anywhere I read. Which devices can appear does depend on the operating system: "CUDA is supported on Windows and Linux", "OptiX is supported on Windows and Linux", "HIP is supported on Windows and Linux", "oneAPI … is supported on Windows and Linux", "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." (manual, GPU Rendering, <https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html>, read 2026-07-30). Cycles' own default reflects this: "Default to selecting the Metal compute device on Apple Silicon GPUs" (`cycles_5.2/properties.py`, lines 1628–1635) — **observed**. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not. **Added by the re-run of 2026-08-07, after the citation check:** the three tests sent back by the citation check were run on that same Linux machine and nowhere else, so they are an NVIDIA-on-Linux answer; macOS and Windows are untouched by them.

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

**Where I looked.** Searched the whole 5.2 manual and the whole 5.2 API documentation for: `stamp`, `metadata`, `device used`, `which device`, `rendered with`, `produced by`, `provenance`, `Software tag`, `EXIF`, `XMP`, `system info`, `compute device`. Opened the Metadata page and its panel picture, the image-formats Metadata section, the Help page, the Preferences ▸ System page, the Cycles Render Settings page, the GPU Rendering page. Read `cycles_5.2/properties.py`, `cycles_5.2/ui.py`, `cycles_5.2/engine.py`, `cycles_5.2/__init__.py` in full or by search for `device`. Ruled out `imbuf` (searched `api_5.2/imbuf.txt` and `api_5.2/imbuf.types.txt` for `metadata` — no hits), and ruled out `bpy.types.Image` and `bpy.types.Scene` for the same reason.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

**The machine.** The Creator's own Mac — a MacBook Pro with an **M1 Pro** chip, **macOS 26.5.2** (build 25F84), Apple's own graphics and no second card. Blender was `/Applications/Blender.app/Contents/MacOS/Blender`, which reported itself as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)` — the same build hash as the Linux machine's 5.2, built at a different hour. Every run was pointed at a settings folder made before it was used (`BLENDER_USER_RESOURCES=/tmp/smoozy-war/settings/5.2`, and `/tmp/smoozy-war/settings2/5.2` where a run needed preferences that had been saved once) and never at the real one. Working files sat in `/tmp/smoozy-war/`; the prepared scene — the factory cube at 160 by 120, 16 samples, Cycles, denoising off — sat at `/Users/artist/Documents/Smoozy hand tests/What actually ran/scene.blend` and was built by Blender 4.5 so all four versions could open it. **Two other topics were running Blender on this Mac while these tests ran**, so nothing below is offered as a timing or a memory measurement. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**A background render with no switches names no device on macOS either.** `blender -b scene.blend -o out/p_ -F PNG -f 1`: exit 0, a frame written, **the error stream 0 bytes**, and no line naming a processor, a card, Metal, CUDA, OptiX, HIP or oneAPI. Line for line the same shape as the Linux run of 2026-08-06.

**`--log-level info` on its own still brings the device line out, and on macOS it names Apple's chip.** Same scene, `--log-level info`, on **standard output**:

```
00:00.529  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
                            |                      Hardware Ray-Tracing: Off
                            | 
                            | Render Scheduler Summary
                            | 
                            | Mode: Headless
                            | Resolution: 160x120
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
00:00.685  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
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

while the same run's device line read `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]`. Both settings name the graphics; the processor did the work.

**What a written image carries.** Every metadata switch on, Cycles, PNG and OpenEXR: **no field names a device, a card or a technology**, in any of the four versions. The `Hostname` field carried the machine's name, `Artists-Studio-MacBook.local`. The `Memory` field read `1M` in 5.2 (and `0.75M` in 4.5) for the processor run — nothing resembling `ddM` appeared.

**`bpy.ops.wm.sysinfo` with no window: the GPU section is empty and the Cycles section names the card.** Exit 0, `{'FINISHED'}`, a 96-line file. The GPU section reads `GPU: missing, background mode`, exactly as on Linux. The Cycles section, on this machine:

```
Cycles:
========================================


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

**The build reports itself as `BUILD_PLATFORM b'Darwin'`**, `BACKGROUND True`, `cycles: True`, `cycles_osl: True`, and every conditional image format compiled in. Against the Linux build of the same version, the differences are `coreaudio` True here and False there, `jack` and `pulseaudio` False here and True there, and `xr_openxr` True in this build.

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

**Answer.** Nothing from the written-out image by itself, on the documented evidence: "engine" is not one of the metadata fields quoted in answer 1, and it is not visible in the Metadata panel picture. What can be read is the scene's engine setting, from the file or the running Blender:

> "engine — Engine to use for rendering (default 'BLENDER_EEVEE')"

— Python API, RenderSettings, <https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html>, read 2026-07-30. **documented**.

The same page carries:

> "has_multiple_engines — More than one rendering engine is available (default False, readonly)"

— same page and date.

Engine identifiers appear in example code in the API documentation:

> `return context.scene.render.engine == 'CYCLES'`

— Python API, NodeTree, <https://docs.blender.org/api/5.2/bpy.types.NodeTree.html>, read 2026-07-30. **documented** that an example in the API compares the engine setting against this word. The page does not say that the word is Cycles' identifier; that is stated in Cycles' own file instead:

> ```
> class CyclesRender(bpy.types.RenderEngine):
>     bl_idname = 'CYCLES'
>     bl_label = "Cycles"
> ```

— `cycles_5.2/__init__.py`, lines 44–46, read 2026-07-30. **observed** — this is where the identifier is set out.

The manual names the engines that ship with Blender:

> "Blender provides the following render engines: EEVEE / Cycles / Workbench … More renderers from third-party developers are available as add-ons."

— Blender manual, Render Engines, <https://docs.blender.org/manual/en/5.2/render/render_engines.html>, read 2026-07-30. **documented**.

The engine can also be set from outside for a given run, so the file's stored engine and the engine that ran need not agree:

> "-E, --engine <engine> — Specify the render engine. Use -E help to list available engines."

— manual, Command Line Arguments, address as above, read 2026-07-30. **documented**.

If the engine is to appear in the finished image at all, it has to be put there deliberately, by the Note field or by `stamp_data_add_field` (both quoted in answer 1).

**Label.** documented for the engine setting and the command-line override; **not in the sources** for the engine being recorded in the render output itself.

**Not covered.** This does not settle whether some image format writes a producer or software tag of its own accord that names Blender or the engine — the manual's format pages do not say. It also does not settle what a third-party engine add-on writes.

**Outside the interface.** The engine property is ordinary data and readable from a script with no interface. `-E`/`--engine` only exists when Blender is driven from outside, and it overrides the file, which is the one real difference: a job launched from outside can render with an engine the file does not name. Note also the ordering trap the manual gives for command-line arguments generally: "Arguments are executed in the order they are given!" (manual, Rendering From The Command Line, <https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html>, read 2026-07-30). **Settled by test on 2026-08-06, on Linux only — see below.** The engine's name reaches a finished image with no window involved, through the four `cycles.ViewLayer.*` stamp fields and, from 5.0 on, an OpenEXR `Software` attribute. `-E help` needs `-b` on a machine with no screen; in Blender 4.5 it never returns even then. **Added by the tests of 2026-08-07:** nothing about the engine changes when a graphics card does the work. Every route above behaves the same outside the window with the card in use as with the processor.

**Operating system.** Nothing in what I read makes the engine setting or `-E` operating-system dependent. One engine-side limitation is stated for one system: "Headless rendering is not supported on headless Windows systems." — manual, EEVEE Limitations, <https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html>, read 2026-07-30. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

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

**Where I looked.** Searched the whole manual and API for `BLENDER_EEVEE`, `BLENDER_WORKBENCH`, `'CYCLES'`, `engine`, `has_multiple_engines`, `produced by`, `Software tag`. Read the Render Engines page, the Metadata page and its picture, the RenderSettings API page, the RenderResult API page, the Command Line Arguments and Rendering From The Command Line pages, `cycles_5.2/__init__.py`. Ruled out the image-format pages (searched `files/media/image_formats.txt` for `metadata` — it discusses support in general terms and lists no fields).

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**The log names the engine on macOS too.** A background render with no switches at all put this on standard output:

```
00:00.837  render           | Engine: Cycles
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

So the short list is not an artefact of how the documentation was made, and it is not an artefact of the operating system either: the same Blender that lists three engines to `-E help` offers one through this property, on both machines.

**Cycles writes four fields of its own into every image it renders, and EEVEE does not.** With every metadata switch on, the Cycles PNG carried `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time`; the EEVEE PNG of the same frame carried none of them. Counted in all four versions: four in the Cycles file, none in the EEVEE file.

**The OpenEXR carries `Software`.** From the Cycles OpenEXR written on this machine:

```
   Software (string) = Blender 5.2.0 LTS
   colorInteropID (string) = lin_rec709_scene
```

Blender 4.5 on the same machine wrote neither attribute. The PNGs carry no `Software` field in any version. Same as the Linux runs.

---

## 3. When a render has finished, what can be learned about which add-ons were in use while it ran?

**Answer.** Nothing from the render itself; add-ons are not on the metadata field list of answer 1, and no add-on field is visible in the Metadata panel picture. What can be learned is what add-ons the *running Blender* had enabled, from two places.

First, the system-information report, which is explicit about it:

> "Enabled Add-Ons — Lists add-ons currently in use along with their versions and paths."

— Blender manual, Help, <https://docs.blender.org/manual/en/5.2/getting_started/help.html>, read 2026-07-30. **documented**. That report is written as `system-info.txt` and is produced on demand, not by rendering.

Second, the enabled add-ons are ordinary data on the preferences:

> "addons — (default None, readonly) — Type: Addons[Addon]"

— Python API, Preferences, <https://docs.blender.org/api/5.2/bpy.types.Preferences.html>, read 2026-07-30, and

> "class bpy.types.Addon(bpy_struct) — Python add-ons to be loaded automatically … module — Module name (default "", never None)"

— Python API, Addon, <https://docs.blender.org/api/5.2/bpy.types.Addon.html>, read 2026-07-30. **documented**.

Cycles itself reaches into that collection by name, which shows the shape of the lookup in practice:

> `return context.preferences.addons[__package__].preferences.compute_device_type`

— `cycles_5.2/ui.py`, lines 96–97, read 2026-07-30. **observed**.

Add-ons are a preference of the installation, not a property of the blend-file: "The Add-ons section lets you manage secondary scripts, called "Add-ons" that extend Blender's functionality." — manual, Add-ons, <https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html>, read 2026-07-30.

**Label.** documented for what is *available* to be asked; **not in the sources** for the finished render carrying anything about add-ons.

**Not covered.** "In use while it ran" could mean "enabled" or "actually did something during the render". The sources only reach "enabled" (and the system-info wording "currently in use" is itself the enabled list, judging by the surrounding text). Nothing I read distinguishes an add-on that was enabled from one that actually took part in a render. This was an ambiguity in the question; I answered the "enabled" reading, and the other reading — which add-ons actually contributed — is not in the sources at all.

**Outside the interface.** The add-on list is readable from a script and does not need an interface. One thing exists only outside: "--addons <addon(s)> — Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons." — manual, Command Line Arguments, address as above, read 2026-07-30. So a job launched from outside can be running with add-ons that no saved preference names, which is exactly the case where a record of what was enabled matters. The manual also notes, for the interface case only, "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred." (Add-ons page, same date). **Settled by test on 2026-08-06, on Linux only — see below.** `bpy.ops.wm.sysinfo` does work with no window in every version, and its Enabled add-ons section is filled in with versions and paths, so the inferred step above holds. **Added by the tests of 2026-08-07:** the add-on section of `system-info.txt` written from outside the window is unchanged by the card working, and no field a render writes names an add-on whichever device ran it.

**Operating system.** Nothing in what I read makes the add-on list or `--addons` operating-system dependent. Paths shown in the report will naturally differ by system, but the sources do not say so in those words. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

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

**Where I looked.** Searched the whole manual and API for `addon`, `add-on`, `Enabled Add-Ons`, `sysinfo`, `extensions`. Read the Add-ons preferences page, the Extensions preferences page listing, the Help page, `bpy.types.Addon`, `bpy.types.Addons`, `bpy.types.Preferences`, `bpy.ops.extensions`. Searched the API for `addon_utils` (no hits — it is not documented). Searched the 5.2 release notes for `addon`/`add-on` (only unrelated items: Loop Tools features becoming built in, a Rigify bug fix, a Brushstroke Tools bug fix). Ruled out the metadata pages as above.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**`bpy.ops.wm.sysinfo` fills its add-on section in with no window.** The eight modules, with versions and paths inside the application bundle:

```
io_anim_bvh (version: (1, 0, 1), path: '/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core/io_anim_bvh/__init__.py')
io_curve_svg (version: UNKNOWN, path: '/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core/io_curve_svg/__init__.py')
io_mesh_uv_layout (version: (1, 2, 0), path: '/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core/io_mesh_uv_layout/__init__.py')
io_scene_fbx (version: (5, 15, 0), path: '/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core/io_scene_fbx/__init__.py')
io_scene_gltf2 (version: (5, 2, 39), path: '/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core/io_scene_gltf2/__init__.py')
cycles (version: UNKNOWN, path: '/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core/cycles/__init__.py')
pose_library (version: (2, 0), path: '/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core/pose_library/__init__.py')
bl_pkg (version: (0, 0, 1), path: '/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core/bl_pkg/__init__.py')
```

The same eight modules as the Linux machine ships, at macOS paths. The file with a window open carries the identical section.

**The list is current, not a record of the session's start.** In one session: write the file, switch `io_anim_bvh` off, write it again without restarting.

```
ON_BEFORE True
DISABLE {'FINISHED'}
ON_AFTER False
   io_anim_bvh in first file: 1  in second file: 0
```

**`--addons` changed nothing a script could read.** With and without `--addons node_wrangler`, the enabled list was identical and identical to the Linux machine's:

```
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

Nothing was printed about the named add-on either way. The same limit applies as on Linux: `node_wrangler` is not among the eight this build ships, so this shows only that naming an absent add-on produces no message and no failure.

---

## 4. What does a render do when it is asked for something the computer or the installed Blender cannot provide, and what does it report?

**Answer.** There is no single documented rule. The sources give a handful of specific, named cases, and are silent on the general one.

**Case 1 — the graphics card runs out of memory.** Blender moves the work into system memory rather than stopping:

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

— Blender manual, GPU Rendering, <https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html>, read 2026-07-30. **documented**. Note this names four of the five backends; oneAPI is not in that sentence.

**Case 2 — it still does not fit.** There is a named error:

> "Error: Out of memory — This usually means there is not enough memory to store the scene for use by the GPU."

— same page and date. **documented**. The page's other named errors are "The NVIDIA OpenGL driver lost connection with the display driver", "Unsupported GNU version", and "CUDA Error: Kernel compilation failed — This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed." All are given as messages a user sees; the page does not say where they are printed.

**Case 3 — no usable graphics card at all.** Cycles' own preferences panel says so plainly, with the reason:

> ```
> col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)
> ```

followed, per backend, by lines such as `"Requires NVIDIA GPU with compute capability %s"`, `"and NVIDIA driver version %s or newer"`, `"Requires AMD GPU with RDNA architecture"` — `cycles_5.2/properties.py`, lines 1858–1900, read 2026-07-30. **observed**. That is a preferences panel, not a render.

What the render then does is a step beyond what the file says, so: Cycles' Python side treats the situation as CPU —

> ```
> def use_cpu(context):
>     cscene = context.scene.cycles
>
>     return (get_device_type(context) == 'NONE' or cscene.device == 'CPU' or not backend_has_active_gpu(context))
> ```

— `cycles_5.2/ui.py`, lines 104–107, read 2026-07-30. **observed** that this function exists and returns that. But `use_cpu` and its twin `use_gpu` are only ever called from panel `poll` and `draw` methods (I checked every call site in `ui.py`, `properties.py` and `operators.py`: lines 137, 146, 365, 947, 2533, all inside the interface code). So concluding that the *render* falls back to the CPU when a GPU is selected but none is usable is **inferred**, and the step is: the add-on's own Python considers "GPU selected but no active device" to be the CPU case for the purpose of which settings apply. The actual device choice is made in code that is not in this folder, and the sources do not state the outcome.

**Case 4 — a device is demanded from the command line and cannot be set.** Cycles exits:

> ```
>     if args.cycles_device:
>         import _cycles
>         if not _cycles.set_device_override(args.cycles_device):
>             sys.exit(1)
> ```

— `cycles_5.2/engine.py`, lines 38–41, read 2026-07-30. **observed** that this is the code. That a bad or unavailable `--cycles-device` value therefore ends the process with status 1 is **inferred**; the step is that `set_device_override` returning false is the only path to that `sys.exit(1)`, and what makes it return false is not visible in these files.

**Case 5 — a feature the installed Blender may not carry.** Cycles' own file holds a branch that reports a message and does nothing else:

> ```
>     def update_script_node(self, node):
>         if engine.with_osl():
>             from . import osl
>             osl.update_script_node(node, self.report)
>         else:
>             self.report({'ERROR'}, "OSL support disabled in this build")
> ```

— `cycles_5.2/__init__.py`, lines 108–113, read 2026-07-30. **observed** that the file holds this branch and this message, and that the same message and pattern appear again for `update_custom_camera` (lines 115–120). Two steps beyond that are mine and are **inferred**: that the branch is taken exactly when the installed Blender was built without the feature, and that nothing else then happens. What `engine.with_osl()` returns is decided in code that is not in this folder, and the file does not say what a render makes of the report. Whether a build has a feature is itself readable: `bpy.app.build_options` is documented as a constant listing `cycles=True, cycles_osl=True, …` — Python API, Application Data, <https://docs.blender.org/api/5.2/bpy.app.html>, read 2026-07-30.

**Case 6 — the engine cannot do what the scene asks.** The manual states limitations rather than behaviour, but a few say what happens: "Path Guiding is not supported on any GPU."; "OSL is not supported with GPU rendering unless using the OptiX backend." (manual, GPU Rendering and Cycles OSL, <https://docs.blender.org/manual/en/5.2/render/cycles/osl/index.html>, read 2026-07-30). For EEVEE: "This mode is not supported and falls back to Displacement and Bump." (manual, EEVEE Material Settings, <https://docs.blender.org/manual/en/5.2/render/eevee/material_settings.html>, read 2026-07-30) — that one names the outcome. Also for EEVEE: "There is no standard way of estimating if the resources will fit into the GPU memory and/or if the GPU will render them successfully." and "Headless rendering is not supported on headless Windows systems." (manual, EEVEE Limitations, address as above). The 5.2 release notes give one more: "Support is currently limited to Cycles. When using EEVEE, a warning is shown on the node and the output is zero." (release notes, Cycles, <https://developer.blender.org/docs/release_notes/5.2/cycles/>, read 2026-07-30).

**How a render reports at all.** The machinery is documented, though not the messages:

> "report(type, message) — Report info, warning or error messages"
> "error_set(message) — Set error message displaying after the render is finished"
> "update_stats(stats, info) — Update and signal to redraw render status text"

— Python API, RenderEngine, <https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html>, read 2026-07-30. **documented**.

**The general case is not in the sources.** I found no statement anywhere of what a render does or reports when the request cannot be met in general — no exit codes for render failure, no list of render error messages, no statement of whether errors go to the console, to a file, or only to a window. The only exit code documented at all is for scripts: "--python-exit-code <code> — Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables." (manual, Command Line Arguments, address as above). **not in the sources** for the general case.

**Label.** documented (cases 1, 2, 6, and the reporting machinery); observed (cases 3, 4, 5 as code); inferred (the render-time consequences in cases 3 and 4, and in case 5 what makes the branch run and what follows it — steps written out above); not in the sources (the general rule, and exit codes for render failure).

**Not covered.** None of this settles what a failing render leaves behind for a program that is watching from outside — whether the process exit status distinguishes success from failure, whether errors reach standard output or standard error, or whether a partly written file is left on disk. It also does not settle the oneAPI case for memory overflow, since that backend is absent from the sentence that names the others.

**Outside the interface.** This is where the sources are thinnest and it matters most. The out-of-memory behaviour is a property of the renderer, not the interface, so it should hold either way, but the sources do not say so. The "No compatible GPUs found for Cycles" text and the EEVEE node warning are interface elements and have no counterpart outside it. `report()` and `error_set()` are described in terms of display ("displaying after the render is finished"), and the sources do not say where those messages go with no window. The `sys.exit(1)` for `--cycles-device` is by nature an outside-the-interface behaviour. One case is explicitly worse outside: "Headless rendering is not supported on headless Windows systems." (EEVEE Limitations). One 5.2 change is relevant: "Added `gpu.init()` to initialize gpu backend when running Blender in background mode `--background`." — release notes, Python API, <https://developer.blender.org/docs/release_notes/5.2/python_api/>, read 2026-07-30. **Settled in part by test on 2026-08-06, on Linux only — see below.** Every failure message found reached standard output or the error stream with no window: the `--cycles-device` refusal, the unwritable-output error, and the `-F` format complaint. The preferences-panel messages could not be reached at all with no window, as expected, and no substitute for them appeared anywhere. **Added by the tests of 2026-08-07:** `--cycles-device` exists only outside the window, and with the card working it can now be seen to succeed silently for `OPTIX` and `CUDA` and to refuse `HIP` and `ONEAPI` with `Found no Cycles device of the specified type`. The out-of-memory case is still out of reach, because this machine's own memory gives out before the card's does.

**Operating system.** Several of the named cases are single-system. "The NVIDIA OpenGL driver lost connection with the display driver … Windows has a limit on the time the GPU can do render computations." and "Unsupported GNU version — On Linux, depending on your GCC version you might get this error." (manual, GPU Rendering). The EEVEE headless limitation is Windows-only. The backend-to-system mapping quoted in answer 1 applies here too. Crash files land in different places per system: "Windows — On a crash, a file is written based on the name of the currently loaded blend-file, so test.blend will create a file called test.crash.txt."; "Linux — After a crash, a file named blender.crash.txt is written to the /tmp directory."; on macOS "the macOS Crash Reporter shows a window with backtrace" and "Some .crash files can also be found in ~/Library/Logs/DiagnosticReports/" — manual, Crashes, <https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html>, read 2026-07-30. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

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

**Where I looked.** Searched the whole manual for `fall back`, `fallback`, `falls back`, `out of memory`, `out-of-memory`, `unsupported`, `is not supported`, `are not supported`, `Error:`, `exit code`, `exit status`, `error code`. Searched the whole API for `report`, `error_set`, `background mode`. Read the GPU Rendering page in full, the EEVEE Limitations pages, the Crashes page, the Command Line Arguments page in full, the RenderEngine API page. Read `cycles_5.2/engine.py` in full, `cycles_5.2/__init__.py` in full, and the relevant stretches of `cycles_5.2/properties.py` and `cycles_5.2/ui.py`. Searched the 5.2 release notes for `device`, `log`, `command line`. Nothing in any of these gives a general failure rule.

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

Identical in all four versions. **This is the mirror image of the Linux table**, where `METAL` was the one that failed and the NVIDIA routes worked once the card was mended. The wording, on **standard output**, for the unknown name:

```
00:00.290  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

and for a real backend with no hardware behind it — `OPTIX`, `CUDA`, `HIP` and `ONEAPI` all alike:

```
00:00.434  reports          | ERROR Found no Cycles device of the specified type
```

**One line the Linux runs carried is missing here.** On Linux the same case printed a `cycles | ERROR CUDA cuInit: Unknown error` line above the report. On this machine there is no such line for any of the four absent backends — only the one sentence.

**A scene set to GPU Compute with no Cycles device switched on renders anyway.** The Cycles preference set to `NONE` and every device unticked, the scene still asking for GPU Compute:

```
SET_NONE ok
PREF_COMPUTE_DEVICE_TYPE NONE
DEVICES_IN_USE [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', False)]
SCENE_CYCLES_DEVICE GPU
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
```

Exit 0, a frame written, the error stream 0 bytes, in all four versions. In 5.0, 5.1 and 5.2 the device line says plainly what happened; in 4.5 nothing at any log level does, and even `--debug-cycles` in 4.5 only puts the same line on the error stream.

**An unknown engine name is refused.** `-E NOT_AN_ENGINE`: no frame, and on the **error** stream:

```
Error: engine not found 'NOT_AN_ENGINE'
```

Exit 1 in 5.0, 5.1 and 5.2. **Blender 4.5 on this machine exited 134** and put `libc++abi: terminating` on the error stream after the same message — see question 4's note on `-E help` below.

**`-E help` on macOS prints its listing and then stops badly in 4.5.** In 5.0, 5.1 and 5.2 it exits 0 cleanly. In 4.5 it printed the full three-engine listing on standard output and then exited **134** with `libc++abi: terminating` on the error stream, three times out of three. That is a different failure from the Linux machine, where 4.5 printed nothing at all and had to be killed.

**A render that cannot write its output.** A plain file put where the output folder should be. Exit **1** in all four versions, and on standard output:

```
00:00.459  image            | ERROR Couldn't create directory for file /tmp/smoozy-war/out/e20/fail/notadir/z_0001.png: File exists
00:00.460  reports          | ERROR Render error (File exists) cannot save: '/tmp/smoozy-war/out/e20/fail/notadir/z_0001.png'
```

Blender 4.5 on this machine gave one line and **different wording from the same test on Linux** — `Error: Render error (Not a directory) cannot save: ...` here against `(Invalid argument)` there. The words in the brackets are the operating system's, not Blender's.

**An invalid `-F` name complains and renders anyway.** Exit 0, a file written in the scene's own format, and this on the **error** stream in all four versions:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
```

**A format that cannot hold metadata says nothing.** TIFF and TGA, every metadata switch on: exit 0, the file written, and no line in either stream matching `warn`, `error`, `metadata`, `not supported` or `preserved` — zero, in all four versions and both formats.

**"No compatible GPUs found for Cycles" could not be reached, and the panel shows why.** Blender started with a window, Preferences ▸ System shown and screenshotted by Blender's own operator. The Cycles Render Devices panel offers two buttons, `None` and `Metal`, with `Metal` chosen; under them a list holding `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked, then `Kernel Optimization: Full` and `MetalRT: Auto`. Setting the preference to `None` leaves the two buttons and **nothing else at all** — no list, no message. There is no CUDA, OptiX, HIP or oneAPI button on this machine, so there is no way to put the panel into the state that message belongs to. Pictures at `/tmp/smoozy-war/out/e28/prefs_5.2.png` and `/tmp/smoozy-war/out/e28/prefs_none_5.2.png`.

---

## 5. Where can each of the things above be learned from — what a render prints while it works, what it leaves on disk, or what can be asked afterwards?

Taking the three places in turn, for each of the four things above.

### What a render prints while it works

**Almost entirely not in the sources.** I could find no page in the 5.2 manual or the 5.2 API documentation that states what Blender prints to a console during a render — no sample lines, no description of the progress text. The nearest things are switches that cause printing:

> "--cycles-print-stats — Log statistics about render memory and time usage."

— manual, Command Line Arguments, <https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html>, read 2026-07-30; and in Cycles' own file the same switch is described as:

> ```
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
> ```

— `cycles_5.2/engine.py`, lines 13–15, read 2026-07-30. **observed** — and note this file, unlike the manual, names the destination: standard error.

> "--log-level <level> — Set the logging verbosity level. fatal: Fatal errors only / error: Errors only / warning: Warnings / info: Information about devices, files, configuration, operations / debug: Verbose messages for developers / trace: Very verbose code execution tracing"

— manual, Command Line Arguments, same date. **documented**. The word "devices" in the `info` line is the only place in the whole manual where logging is tied to hardware. What it actually prints is not shown.

> "--log <match> — Enable logging categories, taking a single comma separated argument. … --log "render,cycles": both render and cycles messages." and "--log-list-categories — List all available logging categories for --log, and exit." and "--debug-cycles — Enable debug messages from Cycles."

— same page and date. **documented**.

There is one picture of Blender printing to a console: `_images/advanced_command-line_launch_windows_console.png`, shown on the manual's Windows launching page (<https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html>, read 2026-07-30). What is visible in it is startup and shutdown text only — lines reading "Read new prefs:", "found bundled python:", "read blend:" and "Blender quit". Nothing about rendering, devices, engines or add-ons is visible. I take from it only that Blender prints to the console at all. **documented**, with the caveat that the paths visible in the picture come from an older build than the one this document is about, so I take no wording from it.

The engine side of printing is documented as machinery without content: `update_stats(stats, info)`, `update_progress(progress)`, `update_memory_stats(*, memory_used=0.0, memory_peak=0.0)` and `report(type, message)` — Python API, RenderEngine, <https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html>, read 2026-07-30.

One handler tells us the shape of the progress text, and it is the single most concrete statement in the sources about what a running render says:

> "bpy.app.handlers.render_stats — on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

— Python API, Application Handlers, <https://docs.blender.org/api/5.2/bpy.app.handlers.html>, read 2026-07-30. **documented**. Frame, sample, render time, saving time — and no device, no engine, no add-ons.

So: of the four things, none is documented as printed. What is documented as printed is memory and time — and those say how much of the computer was used, not which parts of it. Device, engine and add-ons are **not in the sources** as things a render prints.

### What it leaves on disk

- **The rendered image, with metadata.** The field list is quoted in answer 1. It carries hostname, memory, render time and a free-text Note; it carries no device, engine or add-on. Subject to: "Only some image formats support metadata" (manual, Metadata page) and "Not all formats support metadata, and some may only support a limited subset of fields. When using formats that do not support metadata, render information is not preserved in the saved file." (manual, Image Formats, <https://docs.blender.org/manual/en/5.2/files/media/image_formats.html>, read 2026-07-30). **documented**.
- **A log file, if asked for.** "--log-file <filepath> — Set a file to output the log to." (Command Line Arguments). **documented**.
- **A system-information file, if asked for.** `system-info.txt`, with its GPU, Cycles and Enabled Add-Ons sections (quoted in answers 1 and 3). This is the one artefact on disk that names devices, hardware and add-ons — and it is not produced by rendering. **documented**.
- **Crash files, if it crashes.** Names and locations per system, quoted in answer 4. **documented**.
- **A GPU profile file, if asked for.** "--profile-gpu — Enable CPU & GPU performance profiling for GPU debug groups (Outputs a profile.json file in the Trace Event Format to the current directory)" (Command Line Arguments). **documented**.

### What can be asked afterwards

- **Of the running Blender, by script:** the scene's engine (`scene.render.engine`), the Cycles scene device (`scene.cycles.device`), the Cycles preference device type and device list, and `preferences.addons` — all quoted in answers 1–3. **documented** / **observed**.
- **Of the running Blender, about itself:** `bpy.app.version_string`, `bpy.app.version`, `bpy.app.build_*`, `bpy.app.build_options`, `bpy.app.background`, `bpy.app.binary_path`, `bpy.app.tempdir` — Python API, Application Data, <https://docs.blender.org/api/5.2/bpy.app.html>, read 2026-07-30. The API documentation carries a worked example of exactly this kind of report, registered as a command: "Report basic system information." printing `bpy.app.version_string`, `platform.platform()`, `platform.machine()`, `os.cpu_count()` and a dump of `bpy.app`, registered with `bpy.utils.register_cli_command("sysinfo", sysinfo_command)` — Python API, Utilities, <https://docs.blender.org/api/5.2/bpy.utils.html>, read 2026-07-30. **documented**. That example is the sources' own answer to "how do I find out what ran", and it names the processor count and the platform.
- **Of a finished image file, by hand:** the Image editor's sidebar has a "Metadata — Lists image metadata." panel (manual, Image editor sidebar, <https://docs.blender.org/manual/en/5.2/editors/image/sidebar.html>, read 2026-07-30), and the Video Sequencer's Metadata tab "Lists information that has been encoded in the currently visible movie or image file … This also works for images produced by Blender" (manual, VSE preview sidebar, <https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html>, read 2026-07-30). That page also points outside Blender for the job: "The metadata can't be edited from Blender. Instead, you can use an external program such as exiftool." **documented**.
- **Of the session, by hand:** "The Info editor logs the executed operators as well as errors, warnings, and informational messages." (manual, Info Editor, <https://docs.blender.org/manual/en/5.2/editors/info_editor.html>, read 2026-07-30). **documented**.

**Label.** documented and observed as marked above; **not in the sources** for what a render prints about device, engine or add-ons.

**Not covered.** No source I read says whether the metadata written into a file can be read back through Python. I searched the API for `metadata` in `bpy.types.Image`, `bpy.types.Scene`, `imbuf` and `imbuf.types` and found nothing, so reading a finished render's metadata from a script is not in the sources.

**Outside the interface.** This is the whole point of the split, so it is worth being exact. Of the three places: printing and files on disk survive outside the interface — indeed `--cycles-print-stats`, `--log`, `--log-file` and `--profile-gpu` only exist outside it, since they are command-line arguments. Of the "ask afterwards" list, everything reached through Python survives; the Image editor sidebar, the VSE Metadata tab and the Info editor do not, since they are editors. `bpy.app.background` is documented as "True when blender is running without a user interface (started with -b)", so a script can tell which case it is in. **Settled by test on 2026-08-06, on Linux only — see below.** With no window, everything a render prints goes to standard output and the error stream stays empty, apart from the `-F` complaint and, in 4.5 only, the device and `--cycles-device` messages. `--cycles-print-stats` goes to standard output, not the error stream. There is no route from Blender's Python to a written file's metadata in any version, with or without a window. **Added by the tests of 2026-08-07:** the printed route is the one that names the card, and it works with no window. `--cycles-print-stats` replaces its profiling sections with `Profiling information not available (only works with CPU rendering)` when the card did the work. The on-disk route names no card at all. **Added by the re-run of 2026-08-07, after the citation check:** nothing in these three tests needs a window, and all three were run on a machine that has no display server at all.

**Operating system.** The console itself behaves differently on Windows: "When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." (manual, Windows launching page, address as above). Crash file locations differ per system (quoted in answer 4). `-con, --start-console` is marked "(Windows only)" and `--debug-ghost` "(Linux only)" (Command Line Arguments). Nothing else in this answer is qualified by system in the sources. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not. **Added by the re-run of 2026-08-07, after the citation check:** the three tests sent back by the citation check were run on that same Linux machine and nowhere else, so they are an NVIDIA-on-Linux answer; macOS and Windows are untouched by them.

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

**Where I looked.** Searched the whole manual for `Fra:`, `Mem:`, `Remaining`, `stdout`, `stderr`, `standard output`, `render log`, `render report`, `render time`, `progress bar`, `console`, `terminal`. Searched the whole API for `render_` handlers, `update_stats`, `update_progress`, `sysinfo`. Read the Command Line Arguments page in full, both command-line launching pages, the Info Editor page, the Image editor sidebar page, the VSE preview sidebar page, the Image Formats metadata section, the Crashes page. Opened the Windows console picture. Read `cycles_5.2/engine.py` in full.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**Which stream everything lands on.** In 5.2, every one of these went to **standard output** and left the error stream at 0 bytes: the ordinary progress lines, the `Engine:` line, the `Path tracing on:` device line at `--log-level info`, the whole `--cycles-print-stats` block, the `Found no Cycles device` refusal, and the two-line failure when a render cannot save. The only things that reached the **error** stream in 5.2 were the `-F` complaint and the `engine not found` refusal.

**`--log-list-categories` works, and the list is shorter on macOS with different graphics categories.** Exit 0 and **149 categories** on this machine (5.0 gave 135, 5.1 gave 139), against 183 for the same 5.2 on Linux. The graphics ones here:

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
hydra.scene_index
render
```

The Linux list carried `gpu.debug`, `gpu.opengl` and `gpu.vulkan` where this one carries `gpu.debug.metal` and `gpu.metal`. **So the category list is a fact about the operating system as well as the version**, and a plan that hard-codes a category name from one machine will not find it on the other. `addon`, `cycles` and `render` are in both.

**`--log-level` takes a name.** `--log-level info` accepted, exit 0, in 5.0, 5.1 and 5.2. In 4.5 on the same machine it is refused with `Error: not a number '--log-level info'.` on the error stream, and the render then goes ahead anyway, exit 0.

**A written render's fields cannot be read back from Blender's Python.** Identical in all four versions:

```
LOADED (160, 120)
HAS_metadata_attr False
stamp-ish attrs: []
```

**And there is no Metadata panel in the Image editor to read them from either.** The Image editor's sidebar was opened on a stamped PNG loaded from disk. Its tabs are `Tool`, `Image`, `View`, `Scopes` and `Mask` — there is no Metadata tab. Asking the running Blender for every sidebar panel the Image editor defines gives, for the `Image` tab, exactly `Image`, `Render Slots` and `UDIM Tiles`, and **no panel anywhere in the Image editor carries a label containing "metadata"** — the only two panels in the whole of Blender that do are the Asset Browser's and Output Properties' own. Identical in all four versions. A picture of the open sidebar is at `/tmp/smoozy-war/out/e29/img_5.2.png`. The sidebar tab could not be switched from a script (`Region.active_panel_category` is read-only), so the panel list was read instead of photographed.

**The render handlers fire in one order and the frame is already on disk at `render_write`.**

```
HANDLER render_init      t=0.000 args=['Scene', 'NoneType']
HANDLER render_pre       t=0.000 args=['Scene', 'NoneType']
HANDLER render_post      t=0.105 args=['Scene', 'NoneType']
HANDLER render_write     t=0.106 args=['Scene', 'NoneType'] | file exists now: True
HANDLER render_complete  t=0.106 args=['Scene', 'NoneType']
```

`render_cancel` did not fire; all six handlers exist. `render_stats` fired 44 times, first and last:

```
HANDLER render_stats     args=('Mem: 0M | Synchronizing object | Cube', None)
HANDLER render_stats     args=('Time: 00:00.10 (Saving: 00:00.08)', None)
```

Every handler is handed two arguments, not one.

**With a window open, 5.2 keeps handing the same strings over — and the older versions do not.** The same render, driven by `bpy.ops.render.render(write_still=True)` from a timer inside a Blender started with a window:

| version | `render_stats` calls, no window | `render_stats` calls, window open |
|---|---|---|
| 4.5 | 42 | **1** |
| 5.0 | 43 | **1** |
| 5.1 | 43 | **1** |
| 5.2 | 44 | **44** |

The one string 4.5, 5.0 and 5.1 hand over with a window open is the closing line, `('Time: 00:00.06 (Saving: 00:00.04)', None)`. **The control matters and was run**: the same operator-driven render with no window gave 42, 43, 43 and 44, so the difference is the window and not the way the render was started. In 5.2 the windowed strings are the same forty-four, in the same order, as the windowless ones.

**Each frame of a background animation is announced before it is rendered.**

```
00:00.697  render           | Rendering frame 1
00:00.817  render           | Saved: '/tmp/smoozy-war/out/e19/an_0001.png'
00:00.817  render           | Rendering frame 2
00:00.842  render           | Saved: '/tmp/smoozy-war/out/e19/an_0002.png'
00:00.842  render           | Rendering frame 3
00:00.862  render           | Saved: '/tmp/smoozy-war/out/e19/an_0003.png'
```

**A render asked to stop leaves nothing behind, and this corrects the set-up used on Linux.** The Linux run of 2026-08-06 set 100000 samples and killed the run after six seconds, and read a complete image and exit 0 as the answer. Run again here, the same set-up **never reached six seconds**: with adaptive sampling on, all four versions converged at `Sample 960/100000` and finished by themselves at about five seconds, exit 0, a complete valid image, `RenderTime = 00:05.09` and `cycles.ViewLayer.samples = 100000` inside it. That is a render that finished, not one that was stopped, so it says nothing about interruption. Repeated with `use_adaptive_sampling = False` so the render really was still going at six seconds:

```
still rendering at six seconds: yes
exit after SIGTERM=143
no file at output path
```

Identical in all four versions. **So a render stopped part-way through writes no file and ends with status 143**, and the earlier reading — a complete file and exit 0 — describes a render that converged, not one that was interrupted.

**A finished render's own image still carries the fields.** The converged run's PNG opened cleanly (`first 8 bytes: b'\x89PNG\r\n\x1a\n'`, `ends with IEND: True`) and carried `RenderTime`, `Frame`, `Camera`, `Scene`, `File` and the four `cycles.` fields.

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

**Answer, thing by thing.**

**Which parts of the computer (answer 1).** Before the render. The device settings and the device list are preferences and scene properties, readable at any time — nothing in the sources ties reading them to a render being under way or over. The command-line override is applied at start-up, before any render: Cycles parses its arguments inside `init()` —

> ```
> def init():
>     import bpy
>     import _cycles
>     import os.path
>
>     path = os.path.dirname(__file__)
>     user_path = os.path.dirname(os.path.abspath(bpy.utils.user_resource('CONFIG', path='')))
>
>     _cycles.init(path, user_path, bpy.app.background)
>     _parse_command_line()
> ```

— `cycles_5.2/engine.py`, lines 44–53, read 2026-07-30, and `init()` is called from `register()` in `cycles_5.2/__init__.py` (line 151). **observed**: the command-line parsing sits inside `init()`, and `init()` is called from `register()`. That this puts the device override before any render is a step past the files and is **inferred**; the step is that an add-on's `register()` runs when the add-on is loaded, and the files do not say when that happens relative to a render. Afterwards, the only thing the render itself hands back about the machine is hostname and peak memory, through the metadata (**documented**, answer 1).

**Which render engine (answer 2).** Before the render, from `scene.render.engine`. **documented**. Nothing in the sources makes it learnable *from* the finished render.

**Which add-ons (answer 3).** Before the render, from `preferences.addons`, or at any time from a written-out `system-info.txt`. **documented**. Nothing in the sources makes it learnable from the finished render.

**Failure and what it reports (answer 4).** Mixed, and mostly while it runs. The one timing statement I found is:

> "error_set(message) — Set error message displaying after the render is finished"

— Python API, RenderEngine, <https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html>, read 2026-07-30. **documented**: some errors are held back and shown once the render is over. The `--cycles-device` failure happens before any render at all (it is inside `init()`, quoted above) — **inferred** from the placement of `_parse_command_line()` in `init()`; the step is that a `sys.exit(1)` reached during registration ends the process before rendering begins.

**The general framework of "when" is documented, and it is the render handlers.** These are the sources' own answer to this question:

> "bpy.app.handlers.render_init — on initialization of a render job."
> "bpy.app.handlers.render_pre — on render (before)."
> "bpy.app.handlers.render_stats — on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."
> "bpy.app.handlers.render_post — on render (after)."
> "bpy.app.handlers.render_write — on writing a render frame (directly after the frame is written)."
> "bpy.app.handlers.render_complete — on completion of render job."
> "bpy.app.handlers.render_cancel — on canceling a render job."

— Python API, Application Handlers, <https://docs.blender.org/api/5.2/bpy.app.handlers.html>, read 2026-07-30. Each of the scene-level ones "Accepts one argument: the scene data-block being rendered". **documented**.

So the answer in shape: everything about device, engine and add-ons is knowable **before** the render, by asking the settings; **during** the render, the only thing documented as coming out is progress statistics (frame, sample, render time, saving time) plus whatever `--cycles-print-stats` or the log switches emit; and **after** the render, the finished frame carries only the metadata fields of answer 1, while a script can still ask the same settings it could have asked before. Nothing in the sources lets you recover, from a finished render alone, what device, engine or add-ons produced it.

**Label.** documented (the handlers, `error_set`, the metadata fields); observed (the placement of the Cycles command-line parsing); inferred (that this placement puts the device override before any render, and that a bad `--cycles-device` therefore ends things before rendering — steps written out).

**Not covered.** The handlers say *when* they fire, not what data is available at each point beyond the scene. Whether the settings read after a render are guaranteed to be the ones that were in force during it — for instance, whether a command-line device override is visible in the preference afterwards — is not stated anywhere I read. This is the gap that matters most for the question, and the sources do not close it.

**Outside the interface.** Handlers are Python and work with no interface, and the manual shows the pattern for a scripted background animation render: "To render an animation in background mode, allowing drivers and other scripts to run: blender --background --enable-autoexec my_movie.blend --render-anim" — manual, Security, <https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html>, read 2026-07-30. `render_stats` fires "on printing render statistics", which ties it to the printing that happens outside the interface too. The one caution is that the interface-side displays (`error_set` "displaying after the render is finished", `update_stats` "signal to redraw render status text") are described in terms of drawing, and the sources do not say what becomes of them with no window. **Settled by test on 2026-08-06, on Linux only — see below.** All six render handlers fire with no window, in the order init, pre, post, write, complete, with the frame already on disk when `render_write` fires; `render_cancel` does not fire on a finished render. `bpy.app.is_job_running` works with no window and does not crash. So the inferred step above holds. **Added by the tests of 2026-08-07:** which parts of the computer are available can now be learned before a render starts, from outside the window; which part did the work can only be learned once the render has finished, from a closing report.

**Operating system.** Nothing in this answer is qualified by operating system anywhere I read. The requirements page (<https://www.blender.org/download/requirements/>, read 2026-08-01) does not speak to timing at all; it never names 5.2 in so many words, and the only sentence that could reach it is "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." I record that rather than rely on it. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

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

**Where I looked.** Searched the whole API for `handlers`, `render_init`, `render_stats`, `render_complete`, `render_write`, `error_set`. Read the Application Handlers page, the RenderEngine page, `cycles_5.2/engine.py` and `cycles_5.2/__init__.py` in full. Searched the manual for `render job`, `render progress`, `progress bar`. Searched the 5.2 release notes for `handler` and `render`.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**Before a render starts.** With no window and `--factory-startup`, this machine's Cycles preference already reads `METAL` and the card is already listed and ticked:

```
COMPUTE_DEVICE_TYPE METAL
DEVICES [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]
```

**This differs from the Linux machine, where the same read gave `NONE`.** The list only fills in after `refresh_devices()` is called: read without that call in the same session, `p.devices` came back empty while `compute_device_type` still said `METAL`.

**And that difference decides what a render actually uses.** With a settings folder that has never had preferences saved, a scene set to GPU Compute path-traced on the **processor** — `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]` — in the very same run that printed `cycles | Added device "Apple M1 Pro (GPU - 14 cores)"`. With a settings folder in which preferences had been saved once, after `refresh_devices()`, the identical scene and command path-traced on the **card**. So on this machine, whether a GPU render reaches the graphics before it starts depends on the state of the settings folder, and nothing in the scene file says which it will be. In 4.5 no log level reports it at all.

**The sixteen metadata switches at factory startup are the same as on Linux**, `use_stamp_memory` and `use_stamp_hostname` both False and the master `use_stamp` False, so none of them is written unless somebody turns the master on. Threads read `THREADS 8 THREADS_MODE AUTO`.

**The Render Time pass follows the scene's setting and not what did the work, shown here in the harder direction.** The Linux machine could only show it with the card unusable. On this machine the card works, so both halves are real:

```
  device=CPU layers: ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R', 'ViewLayer.Render Time.X']
                     Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
  device=GPU layers: ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R']
                     Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
```

The GPU run genuinely ran on the card and the layer is still absent. `pass_render_time` exists in 5.0, 5.1 and 5.2 and does not exist in 4.5.

**While it runs.** The frame in hand is knowable from the `Rendering frame N` line, and `render_stats` hands a string over dozens of times per frame with no window. **With a window open that stream of strings survives only in 5.2** — 4.5, 5.0 and 5.1 hand over one string, the closing time line. So a plan that watches `render_stats` to follow a render's progress works with no window in every version, and with a window only from 5.2.

**Only once it has finished.** The `Path tracing on:` report arrives after the last sample and before the frame is saved; `render_write` fires with the file already on disk; and the fields inside the image can be read by anything except Blender's own Python.

**`bpy.app.is_job_running` works with no window**, exit 0, no crash:

```
IS_JOB_RUNNING False
```

The installed 5.0 is 5.0.1 on this machine too, which is the release the corrective note says mended it, so this says nothing about 5.0.0.

---

## Things only a running Blender can settle

The list that stood here has been run. What was found is written into the answers above as observed findings, and the whole procedure — every command and every raw result — is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**Run and settled**, across three sittings: the rented Linux machine on 2026-08-06 with its graphics card unreachable, the same machine on 2026-08-07 with the card mended, and the Creator's Mac on 2026-08-07. Whether anything in a render names the device that did the work, on the processor and on a card; whether a written image carries a device or an engine field; what `--cycles-print-stats` prints and on which stream; what a scene set to GPU Compute does when no card can be used; what a rejected `--cycles-device` does; whether `system-info.txt` can be written with no window and what its sections hold, with and without a window; which log categories exist, on both operating systems; whether the frame counter appears during a background animation; whether an engine writes its own stamp fields; whether the Memory metadata box starts ticked, in the panel as well as in the settings; whether the Render Time pass follows the setting or the hardware; whether the peak-memory field is broken; whether the `gpu` module answers with no window, and what `gpu.init()` does; whether `--addons` changes the list a script reads; whether the printed statistics differ with a window open; what the Cycles preferences show when the graphics work; whether the Image editor lists a saved file's fields; and what a render asked to stop leaves behind.

**Still open, and why.**

1. **Whether a graphics card running out of memory announces the move into system memory.** Tried on the mended Linux card and not answerable there — the machine's own memory gives out before the card's does. The Mac is no help either: its graphics memory is the machine's memory.

2. **What Blender 5.0.0 did**, as against 5.0.1. Both installed 5.0 builds are 5.0.1 on both machines, so the peak-memory field and the background-mode crash the corrective notes describe cannot be reached.

3. **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

4. **Whether the two build-dependent messages** — "OSL support disabled in this build" and the Embree lines — ever reach anybody. Every build on both machines reports `cycles_osl: True`, and `build_options` has no Embree entry at all, so neither can be provoked or even checked here.

---

2026-08-06 — Opus 5 (Claude Code) — What actually ran, Blender 5.2

# What actually ran — Blender 4.5

**Sources consulted, and when.**
Blender 4.5 manual (including its `_images` screenshots), Blender 4.5 Python API documentation, Blender 4.5 release notes, and Cycles' own add-on files as shipped with Blender 4.5 — all read on **2026-07-30**. Blender's requirements page, read on **2026-08-01**.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.** One qualification, so the line is exactly true: when I created the folder I was told to save into, the shell listed the file names already in it. I opened none of them and read nothing but their names.

A note that applies throughout. Several of these questions ask what can be learned about a render *that has finished*. Blender's documentation describes settings and features far more often than it describes what a finished render leaves behind. Where I say "not in the sources", the record of where I looked is given with the answer.

---

## 1. When a render has finished, what can be learned about which parts of the computer it used?

**Short answer:** two things, and only if they were switched on before the render: the machine's host name, and the peak memory used. Nothing in the sources says that any record of *which processor or which graphics card did the work* is kept once the render is over.

### What is documented

Blender can stamp information into the rendered image. The manual gives the complete list of what may be included, and two entries touch the computer:

> "Memory
>
> Includes the peak memory usage."

> "Hostname
>
> Includes the rendering machine's hostname."

*Source:* manual, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html — read 2026-07-30.
**LABEL: documented**

The rest of the list on that page is Date, Time, Render Time, Frame, Frame Range, Camera, Lens, Scene, Marker, Filename, Strip Name and Note. There is no entry for a processor, a graphics card, a device, or a number of threads. The screenshot on that same page (`_images/render_output_properties_metadata_panel.png`) shows the panel itself, with the check boxes Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Use Strip Me…, Note and Burn Into Image — and nothing else.
**LABEL: documented** — that this list holds these entries and no processor, graphics-card, device or thread entry among them, which is what the page and the picture show. That nothing anywhere keeps a record of the device is a wider claim than the list carries, and it is answered further down under *What is not in the sources*.

The same set appears in the API as the `use_stamp_*` properties of `RenderSettings`, again with no device entry — for example:

> "use_stamp_memory … Include the peak memory usage in image metadata"

> "use_stamp_hostname"

*Source:* API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.
**LABEL: documented**

Whether that stamp survives at all depends on the file format. The manual's format table has a "Metadata" column, ticked only for PNG, JPEG, OpenEXR and WebP; BMP, Iris, JPEG 2000, Targa, Cineon & DPX, Radiance HDR and TIFF are marked with a cross.
*Source:* manual, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30.
**LABEL: documented**

There is a free-text field that a person could fill with anything, including a device name, and the manual explicitly suggests it for exactly this kind of use:

> "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number."

*Source:* manual, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html — read 2026-07-30.
**LABEL: documented**

### What can be asked of Blender rather than read off the image

The Cycles add-on's own files show what settings exist and what they hold. Note that these are *settings*, read at any time — they say what Cycles was told to use, not what it did.

Which kind of device Cycles was set to use for a scene:

```
    device: EnumProperty(
        name="Device",
        description="Device to use for rendering",
        items=enum_devices,
        default='CPU',
    )
```
and the two choices:
```
enum_devices = (
    ('CPU', "CPU", "Use CPU for rendering"),
    ('GPU', "GPU Compute",
        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
)
```
*Source:* Cycles' own files, `cycles_4.5/properties.py`, lines 378–383 (`CyclesRenderSettings.device`) and lines 30–34 (`enum_devices`) — read 2026-07-30.
**LABEL: observed**

Which technology and which individual cards are ticked, in the preferences:

```
class CyclesDeviceSettings(bpy.types.PropertyGroup):
    id: StringProperty(name="ID")
    name: StringProperty(name="Name")
    use: BoolProperty(name="Use", default=True)
    type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
```
```
enum_device_type = (
    ('CPU', "CPU", "CPU", 0),
    ('CUDA', "CUDA", "CUDA", 1),
    ('OPTIX', "OptiX", "OptiX", 3),
    ('HIP', "HIP", "HIP", 4),
    ('METAL', "Metal", "Metal", 5),
    ('ONEAPI', "oneAPI", "oneAPI", 6)
)
```
and, on `CyclesPreferences`, `compute_device_type` ("Device to use for computation (rendering with Cycles)"), a `devices` collection of the above, and helper methods `get_device_list`, `get_devices_for_type`, `get_num_gpu_devices`, `has_multi_device` and `has_active_device`.
*Source:* Cycles' own files, `cycles_4.5/properties.py`, lines 1548–1552, 176–183, 1555–1740 — read 2026-07-30.
**LABEL: observed**

The number of processor cores Blender was allowed to use is a scene setting:

> "threads
>
> Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems)"

*Source:* API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.
**LABEL: documented**

The GPU module can be asked what graphics hardware is in play:

> "gpu.platform.renderer_get()
>
> Get GPU to be used for rendering.
>
> Returns: GPU name."

with companions `backend_type_get()` ('OPENGL', 'VULKAN', 'METAL', 'NONE', 'UNKNOWN'), `device_type_get()` ('APPLE', 'NVIDIA', 'AMD', 'INTEL', 'SOFTWARE', 'QUALCOMM', 'UNKNOWN'), `vendor_get()` and `version_get()`.
*Source:* API, https://docs.blender.org/api/4.5/gpu.platform.html — read 2026-07-30.
**LABEL: documented.** Careful: the sources do not say that this is the same device Cycles renders on. `gpu.platform` is documented as "access to GPU Platform definitions" for the GPU module, and Cycles' devices are configured separately in the Cycles preferences. Treating `renderer_get()` as "the card that rendered your Cycles image" would be an unsupported step.

### The one file Blender writes that names the hardware

> "Save System Info
>
> This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems.
>
> You will be prompted to save a text file called system-info.txt."

and among its sections:

> "GPU
>
> Shows the GPU vendor, version and the capabilities of your hardware and driver."

> "Cycles
>
> The instruction sets and capabilities of each hardware render device available for use with Cycles."

*Source:* manual, https://docs.blender.org/manual/en/4.5/getting_started/help.html — read 2026-07-30.
**LABEL: documented**

It is available as an operator, so it need not be reached through the menu:

> "bpy.ops.wm.sysinfo(*, filepath='')
>
> Generate system information, saved into a text file"

*Source:* API, https://docs.blender.org/api/4.5/bpy.ops.wm.html — read 2026-07-30.
**LABEL: documented**

Note what this is and is not. **LABEL: inferred** — the quoted words describe a file written when you ask for it, listing hardware "available for use with Cycles", so it describes the machine at the moment of asking rather than what a particular finished render used. The step is the plain meaning of "available"; no source says whether the file holds anything about a render already done.

### Statistics Cycles can be asked to print

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.
**LABEL: documented**

Cycles' own file gives the same switch a slightly fuller description, including where the output goes:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```
*Source:* Cycles' own files, `cycles_4.5/engine.py`, lines 13–15 — read 2026-07-30.
**LABEL: observed** (this is the text of the option as the file defines it; what the statistics actually contain is not stated in any file I was given)

### What is not in the sources

No source I was given says that a finished render records which processor or which graphics card produced it, nor that any such value is written into an image, a log, or the blend-file. The path-template feature, which is the one place Blender substitutes machine-known values into an output file name, lists its complete set of variables — `blend_name`, `fps`, `resolution_x`, `resolution_y` — with the note "The set of available variables will expand over time". No device or engine variable exists.
*Source:* manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30.
**LABEL: not in the sources** (for a per-render record of the device used)

**Where I looked.** Searched the whole 4.5 manual, the whole 4.5 API documentation, all 4.5 release notes and all Cycles files for: `metadata`, `stamp`, `device`, `compute_device_type`, `get_devices`, `has_active_device`, `sysinfo`, `System Info`, `system-info`, `GPU`, `hardware`, `renderer`, `threads`, `which device`, `device used`, `rendered on`, `provenance`, `audit`, `render log`, `manifest`, `sidecar`. Read in full: the Metadata page, the image-formats page, `RenderSettings` and `RenderResult` in the API, the Cycles preferences and render-settings code in `properties.py`, `engine.py`, the Help menu page, the System preferences page, the path-templates section. Opened the pictures `_images/render_output_properties_metadata_panel.png` (settles the panel's contents) and `_images/editors_preferences_section_system.png` (ruled out — the Cycles Render Devices panel is collapsed in it, so it shows no device names).

**Not covered.** Whether Cycles or EEVEE write anything of their own into an image's metadata beyond the documented stamp list; what `--cycles-print-stats` actually prints; whether the render statistics name a device; whether an OpenEXR header carries any Blender-written attribute not listed on the Metadata page.

**Outside the interface.** The stamp settings, the Cycles device settings, `RenderSettings.threads` and the `gpu.platform` calls are all data and Python, with nothing in their documentation tying them to a window. `--cycles-print-stats` is a command-line switch and is only reachable from outside the interface. `Help ‣ Save System Info` is a menu item, but the same thing is an operator (`bpy.ops.wm.sysinfo`) that takes a file path, so it does not need the menu — **inferred**, the step being that an operator with a `filepath` argument can be called from a script; the sources do not say it works with no interface open. One documented difference: the render-statistics text differs in background mode (see question 5). **Settled by test on 2026-08-06, on Linux only — see below.** `bpy.ops.wm.sysinfo` does work with no window, returning `{'FINISHED'}` and writing its file, so the inferred step above holds; but its GPU section then reads `GPU: missing, background mode`, so the route works and the hardware part of what it produces does not. The `gpu.platform` calls do **not** work with no window — they raise `SystemError` in every version. And the line naming the processor that did the work is reachable with no window, on standard output. **Added by the tests of 2026-08-07, with the machine's graphics card repaired:** driving Blender from outside its window is no obstacle to naming the card. With no window and no display server, a run prints the card's model in its `Path tracing on:` line, `bpy.ops.wm.sysinfo` describes the card in full in its Cycles section, and the Cycles preferences list it before any render. The GPU section of `system-info.txt` still reads `GPU: missing, background mode`, so that one section is genuinely lost outside the window and nothing else is. **Added by the re-run of 2026-08-07, after the citation check:** nothing in these three tests needs a window, and all three were run on a machine that has no display server at all.

**Operating system.** Which devices can appear at all is operating-system dependent, and this is documented: CUDA, OptiX, HIP and oneAPI are each described as "supported on Windows and Linux", while "Metal is supported on Apple computers with Apple Silicon" (manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30). The metadata stamp itself is described with no operating-system qualification. The sources do not say the stamp behaves differently on any system. **Settled in part by a test on 2026-08-06 — see below: on Linux, Blender 4.5 does print the processor it used, so the answer is no longer open on that system. macOS is untested.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not. **Added by the re-run of 2026-08-07, after the citation check:** the three tests sent back by the citation check were run on that same Linux machine and nowhere else, so they are an NVIDIA-on-Linux answer; macOS and Windows are untouched by them.

### Tested by hand, 2026-08-06 — **LABEL: observed**

All of the following were run on **the rented Linux machine**: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server. The Blender was `/opt/blender/4.5/blender`, which reported itself as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`. It was pointed at a settings folder made for the purpose, `BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/4.5`, never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The full procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**One thing about this machine bears on every device result here.** Its graphics card could not be reached while the tests ran — the card's kernel module (580.95.05) and the libraries that talk to it (580.173.02) no longer match. `nvidia-smi` answered `Failed to initialize NVML: Driver/library version mismatch`, and Blender's own view was `cycles | ERROR CUDA cuInit: Unknown error` with an empty device list. So every result below is from a machine that has a graphics card and cannot use it.

**Blender 4.5 does print which processor did the work — but only under `--debug-cycles`, and only on the error stream.** The scene was the factory cube at 160 by 120, 16 samples, Cycles. Run as `blender -b scene.blend --debug-cycles -o out/dc_ -F PNG -f 1`, with the two streams captured to separate files, the error stream carried:

```
I0806 23:07:49.259145 326351 device_impl.cpp:47] Using AVX2 CPU kernels.
I0806 23:07:49.259738 326351 device_impl.cpp:47] Using AVX2 CPU kernels.
Path tracing on: AMD EPYC 7B12 64-Core Processor (CPU) (24 threads) [CPU]
```

That names the processor's model, the number of threads and the kind of device. **No log switch reaches it in 4.5.** Run as `blender -b scene.blend --log "*" --log-level -1 -o out/y_ -F PNG -f 1` — everything 4.5's log system will emit — it printed 382 lines and not one matched `CUDA`, `OptiX`, `Metal`, `HIP`, `oneAPI`, `GeForce`, `NVIDIA`, `EPYC`, `Path tracing on` or `Engine:`.

**A background render with no switches at all names no device.** Run as `blender -b scene.blend -o out/p_ -F PNG -f 1`, exit 0, frame written, error stream 0 bytes, and the whole of standard output at the two ends (the middle is forty more lines of the same shape):

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/what-actually-ran/work/scene.blend"
Fra:1 Mem:8.42M (Peak 9.12M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Light
Fra:1 Mem:8.24M (Peak 9.12M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Loading render kernels (may take a few minutes the first time)
Fra:1 Mem:9.31M (Peak 9.31M) | Time:00:00.01 | Remaining:00:00.04 | Mem:0.75M, Peak:0.75M | Scene, ViewLayer | Sample 1/16
Fra:1 Mem:9.60M (Peak 9.60M) | Time:00:00.02 | Mem:0.75M, Peak:0.75M | Scene, ViewLayer | Finished
Saved: '/root/what-actually-ran/out/p_0001.png'
Time: 00:00.13 (Saving: 00:00.10)


Blender quit
```

**No field written into a finished image names a device.** With every `use_stamp_*` switched on and the note set to `SMOOZY NOTE`, a Cycles render to PNG carried exactly these fields, read straight out of the file's text chunks:

```
   File = /root/what-actually-ran/work/scene.blend
   Note = SMOOZY NOTE
   Date = 2026/08/06 22:59:14
   Marker = <none>
   Time = 00:00:00:01
   Frame = 1
   FrameRange = 1:3
   Camera = Camera
   Lens = 50.00
   Scene = Scene
   Strip = <none>
   RenderTime = 00:00.02
   Memory = 0.75M
   Hostname = ubuntu
   cycles.ViewLayer.samples = 16
   cycles.ViewLayer.total_time = 00:00.02
   cycles.ViewLayer.render_time = 00:00.01
   cycles.ViewLayer.synchronization_time = 00:00.00
```

The same render to OpenEXR carried the same fields as header attributes plus the format's own (`channels`, `compression`, `dataWindow`, `displayWindow`, `lineOrder`, `pixelAspectRatio`, `screenWindowCenter`, `screenWindowWidth`, `xDensity`). No field in either names a processor, a card or a technology. The Memory field read `0.75M`.

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

The master switch `use_stamp` is off, so none of the rest is written unless somebody turns it on. The same run printed `THREADS 24 THREADS_MODE AUTO` — the machine's core count, not the documented default of 1.

**The settings read after a run do not say what ran.** A run whose scene was set to GPU Compute and which was overridden on the command line with `-- --cycles-device CPU` printed, from inside the same run:

```
AFTER_OVERRIDE compute_device_type= NONE  scene.cycles.device= GPU
```

**`bpy.ops.wm.sysinfo` works with no window, but its GPU section is not filled in.** Run as `blender -b --python-expr "import bpy; print(bpy.ops.wm.sysinfo(filepath='.../si_4.5.txt'))"`, it returned `{'FINISHED'}`, exit 0, and wrote a 94-line file whose GPU and Cycles sections read:

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
GPU_ERROR SystemError GPU functions for drawing are not available in background mode
HAS_GPU_INIT False
```

---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/4.5/blender`, which reported itself as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/4.5`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**The `Path tracing on:` line names the graphics card, its model and the route taken to it.** The same scene as before — the factory cube at 160 by 120, 16 samples, Cycles — with the Cycles preference set to OptiX, the card's OptiX device switched on and the scene set to GPU Compute. Run as `blender -b scene.blend --python setcard.py --debug-cycles -o out/l_ -F PNG -f 1`, the **error** stream carried:

```
Path tracing on: NVIDIA GeForce RTX 4080 SUPER (OptiX) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07_OptiX]
```

and with the preference set to CUDA instead, the same line read:

```
Path tracing on: NVIDIA GeForce RTX 4080 SUPER (CUDA) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07]
```

So the line names the card's model, which of the two NVIDIA routes was taken, and an identifier holding the card's place on the bus. It is the same line, in the same place, brought out by the same switch as when it named the processor on 2026-08-06 — no log switch reaches it in 4.5, so `--debug-cycles` remains the only route.

**Where in the run it arrives.** The line is part of a closing report: it is printed after the last sample and before the frame is written. In the 5.2 run the last sample line was at `00:02.270` and the report at `00:02.270`, with `Saved:` at `00:02.577`. So it answers *what did the work* once the work is done, not before it starts.

**A background render with no switches at all still names no card — but OptiX leaves its name behind.** Run with no log switches: exit 0, a frame written, error stream 0 bytes, and no line naming the card. What OptiX does put into the ordinary progress lines is its own name:

```
Fra:1 Mem:8.33M (Peak 35.41M) | Time:00:00.31 | Mem:0.01M, Peak:0.01M | Scene, ViewLayer | Updating Scene BVH | Building OptiX acceleration structure
```

Lines of that shape appear four times in the OptiX run and **not once** in the CUDA run of the same scene. So at default settings the printed output can tell you OptiX was in use and cannot tell you CUDA was.

**The memory figure jumps when the card does the work, and that needs no switch either.** The same frame, the same scene, the figure Blender prints in its progress lines and writes into the `Memory` stamp field:

| what did the work | the figure |
|---|---|
| the processor | `0.75M` |
| the card | `638.26M` |

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

**The `gpu` module still answers nothing, and the card being repaired makes no difference to it.** The same call as on 2026-08-06, run again with the card working:

```
HAS_GPU_INIT False
GPU_ERROR SystemError GPU functions for drawing are not available in background mode
```

So the refusal is about the missing window, not about the missing card — which the 2026-08-06 run could not tell apart. `gpu.init` does not exist in this version; it exists in 5.2 and does work there.

**The settings still do not say what ran, and can now be shown to mislead.** A run with the Cycles preference set to OptiX and the scene set to GPU Compute, overridden on the command line with `-- --cycles-device CPU`, printed from inside itself:

```
AFTER_OVERRIDE compute_device_type= OPTIX  scene.cycles.device= GPU
```

while nothing the run printed at that log level named any device — 4.5 keeps that line behind `--debug-cycles`. Both settings name the card's route; the command line had sent the work to the processor.

**A timing worth having, now that nothing else is running on this machine.** The same frame — 960 by 540, 1024 samples, adaptive sampling off — rendered twice, once with `-- --cycles-device OPTIX` and once with `-- --cycles-device CPU`:

| what did the work | wall clock | the `Time:` line Blender printed |
|---|---|---|
| the card, OptiX | 5.29 s | `Time: 00:04.71 (Saving: 00:00.19)` |
| the processor | 27.73 s | `Time: 00:27.15 (Saving: 00:00.18)` |

**This is one machine's timing, on one scene, on one day**, and it is offered as nothing more. What it does show is that the two are nowhere near each other: the card finished this frame about six times sooner, which is far outside anything the spread between repeats could explain. The card's figure includes loading the render kernels, so it is if anything the slower of the two readings.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

**The machine.** The Creator's own Mac — a MacBook Pro with an **M1 Pro** chip, **macOS 26.5.2** (build 25F84), Apple's own graphics and no second card. Blender was `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, which reported itself as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)` — the same build hash as the Linux machine's 4.5, built at a different hour. Every run was pointed at a settings folder made before it was used (`BLENDER_USER_RESOURCES=/tmp/smoozy-war/settings/4.5`, and `/tmp/smoozy-war/settings2/4.5` where a run needed preferences that had been saved once) and never at the real one. Working files sat in `/tmp/smoozy-war/`; the prepared scene — the factory cube at 160 by 120, 16 samples, Cycles, denoising off — sat at `/Users/artist/Documents/Smoozy hand tests/What actually ran/scene.blend` and was built by Blender 4.5 so all four versions could open it. **Two other topics were running Blender on this Mac while these tests ran**, so nothing below is offered as a timing or a memory measurement. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**A background render with no switches names no device on macOS either.** `blender -b scene.blend -o out/p_ -F PNG -f 1`: exit 0, a frame written, **the error stream 0 bytes**, and no line naming a processor, a card, Metal, CUDA, OptiX, HIP or oneAPI. Line for line the same shape as the Linux run of 2026-08-06.

**No log switch reaches the device line in 4.5 on this machine either.** Run with `--log "*" --log-level -1` — everything this version's log system will emit — 4.5 printed 430 lines and **not one** matched `CUDA`, `OptiX`, `Metal`, `HIP`, `oneAPI`, `Apple`, `M1`, `Path tracing on` or `Engine:`. Only `--debug-cycles` brings the line out, and it lands on the **error** stream:

```
I0807 02:32:12.685928 22322362 session.cpp:173] 
Full path tracing report

Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]

Render Scheduler Summary

Mode: Headless
Resolution: 160x120
```

**The same line names the card when the card does the work**, still on the error stream and still only under `--debug-cycles`:

```
I0807 02:37:42.588058 22337322 device.mm:105] Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
I0807 02:37:42.588124 22337322 device.mm:108] Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
```

So the line names the chip, the number of graphics cores, the route (Metal), the word `display` — this machine's graphics drive the screen as well — and an identifier built from the two.

**One of those lines must not be read as saying what ran.** The `Added device` line reports what Cycles found, not what did the work; in 5.0, 5.1 and 5.2 on the same machine it appears word for word in runs whose `Path tracing on:` line names the processor.

**`--cycles-print-stats` still names no device.** Exit 0, the whole statistics block on **standard output**, the error stream 0 bytes, and searching the captured output for `Metal`, `CUDA`, `OptiX`, `HIP`, `oneAPI`, `GPU`, `CPU`, `Apple` and `M1` returned nothing.

**And with the graphics doing the work it refuses.** The same switch with `-- --cycles-device METAL`: exit 0, the block still on standard output, the error stream 0 bytes, and in place of the timings, in all four versions, the one sentence:

```
Profiling information not available (only works with CPU rendering)
```

— word for word what the mended Linux card produced. Nothing in it names a device.

**A render on the graphics writes no new field, but the `Memory` field jumps.** Every metadata switch on, the same one-cube scene, `-- --cycles-device METAL`: the field set is identical to the processor run's and **no field names the graphics, the backend or the technology** — every value searched for `metal`, `apple`, `m1`, `nvidia`, `geforce`, `cuda`, `optix`, `hip`, `oneapi` and `device`, zero matches, in all four versions. What changes is one number:

| what did the work | the `Memory` field in this version |
|---|---|
| the processor | `0.75M` |
| the graphics | `1360.81M` |

So the figure is a strong hint and names nothing — the same shape of hint the Linux card gave. **It is a memory reading taken on a shared machine**, so it is offered as an order of magnitude and not as a measurement.

**The settings still do not say what ran, and on this machine they mislead in the other direction.** A run with the scene set to GPU Compute and `-- --cycles-device CPU` on the command line printed from inside itself:

```
AFTER_OVERRIDE compute_device_type= METAL  scene.cycles.device= GPU
```

while the same run path-traced on the processor. Both settings name the graphics; the processor did the work.

**What a written image carries.** Every metadata switch on, Cycles, PNG and OpenEXR: **no field names a device, a card or a technology**, in any of the four versions. The `Hostname` field carried the machine's name, `Artists-Studio-MacBook.local`. The `Memory` field read `0.75M` for the processor run — nothing resembling `ddM` appeared.

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

**The `gpu` module still answers nothing with no window in this version**, on a machine whose graphics work:

```
GPU_ERROR SystemError GPU functions for drawing are not available in background mode
HAS_GPU_INIT False
```

There is no `gpu.init` to call here — 5.2 alone offers one, and there it answers.

**The Metadata panel, seen on screen.** Blender started with a window on the factory scene, the Output Properties tab shown and its own search box set to `Metadata` so the panel opened, then Blender's own `screen.screenshot` operator. The panel showed `Metadata Input: Scene` and thirteen boxes: **Date, Time, Render Time, Frame, Camera, Scene and Filename ticked; Frame Range, Memory, Hostname, Lens, Marker and Strip Name unticked**, with `Note` and `Burn Into Image` as closed sub-panels, both unticked. **The Memory box is not ticked**, which matches what a running Blender reports and not the API page's stated default of True. Identical in all four versions. The picture is at `/tmp/smoozy-war/out/e27/panel_4.5.png`.

**The build reports itself as `BUILD_PLATFORM b'Darwin'`**, `BACKGROUND True`, `cycles: True`, `cycles_osl: True`, and every conditional image format compiled in. Against the Linux build of the same version, the differences are `coreaudio` True here and False there, `jack` and `pulseaudio` False here and True there.

**Threads read `THREADS 8 THREADS_MODE AUTO`** at factory startup — this machine's core count, not the documented default of 1.

---

### Tested by hand again after the citation check, 2026-08-07 — **LABEL: observed**

**Why these runs happened.** The citation check of this topic found four disagreements, recorded both sides of each, chose between none of them, and sent them back to the hand tests. Three of the four are tests. They were run on 2026-08-07 on the rented Linux machine, alone and last, because that machine was about to be taken away and nothing else could be asked of it afterwards. They are entries 42, 43 and 44 of `Audits/Blender Research/What actually ran/Hand tests.md`.

**The machine, named in full.** The rented Linux machine: Ubuntu 22.04.5 LTS, kernel `6.8.0-59-generic`, one NVIDIA GeForce RTX 4080 SUPER with 16376 MiB of graphics memory, driver **580.173.02** in both halves — `nvidia-smi` and `/proc/driver/nvidia/version` were read at the start of the sitting and agree — an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all. Nothing else was running on it: the load average was 0.19 and there was no Blender process. Everything was written inside `/root/again-what-actually-ran/`, and each of the four Blenders was given a settings folder of its own, made before it was used. The scene is the one-cube factory scene at 160 by 120, sixteen samples, denoising off, built by Blender 4.5 so that all four versions can open it. No test below reads what is written inside a finished image, so no picture and no file was compared.

**Two of the three tests answer this question. They are set out one after the other below.**

---

#### Test one — what the `Full path tracing report` block holds in this version, transcribed exactly

*What a negative result would look like, said before the run.* A `Hardware Ray-Tracing:` line inside this version's block. Blender 5.1 and 5.2 were run identically alongside as controls, precisely so that this test can come out the other way: if those two show the line and this one does not, the absence is a fact about the version and not a fault in the capture.

*What was done.* `/opt/blender/4.5/blender -b scene.blend --debug-cycles -o out/ -F PNG -f 1`, five times, both streams captured to separate files. `--debug-cycles` is the only switch that reaches this line in 4.5 (see the tests of 2026-08-06 above), and the line lands on the **error** stream. The same run was then made three more times with the graphics card doing the work.

*What came back, word for word.* Exit 0 every time. The block, from the error stream of the first of the five processor runs, uncut from the line above it:

```
I0807 05:43:52.574143 558427 session.cpp:173] 
Full path tracing report

Path tracing on: AMD EPYC 7B12 64-Core Processor (CPU) (24 threads) [CPU]

Render Scheduler Summary

Mode: Headless
Resolution: 160x120
```

**There is no `Hardware Ray-Tracing:` line in Blender 4.5, in any of the eight runs**, on either stream, on the processor or on the card. Two smaller things this version does differently, and both are exact: its heading reads `Full path tracing report` with **no colon**, where 5.0, 5.1 and 5.2 all write `Full path tracing report:`; and it carries no `cycles           |` prefix, because in 4.5 this is Cycles printing for itself rather than Blender's log. With the card doing the work the same block opens `Path tracing on: NVIDIA GeForce RTX 4080 SUPER (OptiX) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07_OptiX]`, still with no `Hardware Ray-Tracing:` line.

*The controls did their job.* Blender 5.1 and 5.2, run identically on the same machine on the same day, **do** carry `Hardware Ray-Tracing: Off` in every one of their eight runs each. So the test could have come out the other way, and did not.

*The disagreement this settles for 4.5, and the one it does not.* Nothing in this document ever claimed a `Hardware Ray-Tracing:` line for 4.5, so nothing here is disturbed. The disagreement sent back was in the 5.0 documents, and it is not resolved: the 2026-08-06 transcript there carries the line, and every later run of the same test on the same machine does not.

---

#### Test two — how many `Building OptiX acceleration structure` lines a run prints

*What a negative result would look like, said before the run.* The same number in all five runs of a version — which would have meant the two earlier counts were a miscount rather than a moving target, and the finding could then name the number with five repeats behind it. The CUDA runs beside them are the second way this test can fail: they show what the same count looks like when the answer is nought, so the counting is not something that only ever returns small positive numbers.

*What was done.* The shared scene rendered on the card with OptiX and **no log switches at all**, five times per version, counting the lines that carry `Building OptiX acceleration structure` on each stream. The card was pointed at by a script that sets the Cycles preference to OptiX and the scene to GPU, and prints the device list back so the run says rather than assumes what it used — it read `[('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', False), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False), ('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True)]` in every OptiX run. Five CUDA runs per version were made the same way, as the control.

*What came back.* The count is **not stable**. Five OptiX runs of the identical command in this version gave, in order:

```
3, 4, 4, 3, 4
```

and the five CUDA runs beside them gave `0, 0, 0, 0, 0`. Exit 0 and a frame written in all ten; the error stream was 0 bytes in all ten.

*Why it moves, read straight off the lines.* The lines are ordinary progress lines, and which of the geometry steps gets a line of its own varies from run to run. Two runs of this version, side by side, with the timestamps and memory figures taken out:

```
run 1
  Updating Geometry BVH Cube 2/1 | Building OptiX acceleration structure
  Updating Geometry BVH Light 3/2 | Building OptiX acceleration structure
  Updating Scene BVH | Building OptiX acceleration structure

run 2
  Updating Geometry BVH light 4/3 | Building OptiX acceleration structure
  Updating Geometry BVH Light 3/3 | Building OptiX acceleration structure
  Updating Geometry BVH Cube 2/1 | Building OptiX acceleration structure
  Updating Scene BVH | Building OptiX acceleration structure
```

The `Updating Scene BVH` line is in every run of every version. The geometry lines above it are not. So the number is a fact about which progress steps happened to be reported, and never a fact about Blender or about OptiX.

*What this settles, and what must not be written.* Across the four versions the counts were 4.5 `3, 4, 4, 3, 4`; 5.0 `2, 4, 4, 2, 4`; 5.1 `3, 4, 3, 3, 4`; 5.2 `2, 4, 2, 3, 3`; and CUDA nought, twenty runs out of twenty. **The number must never be quoted as a fact about a version.** What holds, and held in every run anybody has made, is the half the claim rests on: lines of this shape appear in an OptiX run and never once in a CUDA run of the same scene. The right form of words is *more than once*, which is what the documents now say.


## 2. When a render has finished, what can be learned about which render engine produced it?

**Short answer:** no source I was given says anything about the engine is recorded automatically. Blender's documented stamp list has no engine field. An engine *may* add its own stamp fields, and Blender provides the call for it, but no source says that Cycles or EEVEE use it. What you can learn afterwards is what the *file is currently set to*, which is not the same thing.

### The stamp list has no engine entry

The Include list on the Metadata page — Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Note — contains no engine or renderer entry, and the panel screenshot agrees.
*Source:* manual, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html and its picture `_images/render_output_properties_metadata_panel.png` — read 2026-07-30.
**LABEL: documented** — that this list holds these entries and no engine entry among them. That nothing anywhere records the engine is a wider claim than the list carries, and the rest of this answer treats it as such.

### But an engine can add its own

> "stamp_data_add_field(field, value)
>
> Add engine-specific stamp data to the result
>
> Parameters:
> field (string, (never None)) – Field, Name of the stamp field to add
> value (string, (never None)) – Value, Value of the stamp data"

*Source:* API, https://docs.blender.org/api/4.5/bpy.types.RenderResult.html — read 2026-07-30.
**LABEL: documented** — that the mechanism exists. Whether any shipped engine uses it, and whether it would write the engine's own name, is **not in the sources**.

### What can be asked afterwards

The engine is a scene setting, readable at any time:

> "engine
>
> Engine to use for rendering"

*Source:* API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

(The same page states the type as "enum in ['BLENDER_EEVEE_NEXT'], default 'BLENDER_EEVEE_NEXT'". Cycles registers itself as an engine with `bl_idname = 'CYCLES'` — Cycles' own files, `cycles_4.5/__init__.py`, line 43, read 2026-07-30. **LABEL for the `'CYCLES'` identifier: observed.** So the API page's one choice and Cycles' own file do not agree about what may be chosen. Why they do not is **not in the sources** — nothing I was given says how that page's list of choices is arrived at.)

There is also a read-only context value:

> "bpy.context.engine
>
> Type: string, default "", (readonly, never None)"

*Source:* API, https://docs.blender.org/api/4.5/bpy.context.html — read 2026-07-30.
**LABEL: documented**

And the engine can be chosen or listed from the command line:

> "-E, --engine <engine>
>
> Specify the render engine. Use -E help to list available engines."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

> "-E CYCLES
>
> Use the "Cycles Render" engine. For a list of available render engines, run blender -E help."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30.
**LABEL: documented**

All of these tell you what the file or the session is set to *now*.
**LABEL: inferred** that none of them is attached to a finished image — the step being that a scene setting and a context value describe the state they are read in, which the quoted descriptions do not say in so many words.

### One indirect trace

Cycles' own file lists the passes it registers, and several names are Cycles' own: `"Noisy Image"`, `"Noisy Shadow Catcher"`, `"Denoising Normal"`, `"Denoising Albedo"`, `"Denoising Depth"`, `"Debug Sample Count"`, `"Guiding Color"`, `"Guiding Probability"`, `"Guiding Average Roughness"`.
*Source:* Cycles' own files, `cycles_4.5/engine.py`, `list_render_passes`, lines 177–262 — read 2026-07-30.
**LABEL: observed** — that Cycles registers passes under these names.

If a multi-layer OpenEXR from a finished render contains a layer named, say, "Noisy Image", that is a sign Cycles produced it.
**LABEL: inferred.** The step: the file shows Cycles registering that pass name; I am assuming a registered pass name reaches the written file under the same name, and that no other engine registers the same name. Neither assumption is stated anywhere I was given. It is also conditional — those passes only appear when the matching options are on, so their absence proves nothing.

**Where I looked.** Searched the whole manual, API and release notes for `engine`, `renderer`, `stamp`, `metadata`, `stamp_data_add_field`, `arbitrary values`, `CYCLES`, `software`, `producer`, `creator`, `written by blender`, `EXR header`, `exr attribute`, `header attribute`. Read the Metadata page, the image-formats page, `RenderResult`, `RenderEngine` and `RenderSettings` in the API, `bpy.context`, the command-line arguments and command-line render pages, and Cycles' `__init__.py`, `engine.py` and `ui.py`. Ruled out: the compositing Cryptomatte page (its only metadata mention is "Cryptomatte sidecars (metadata files) are not supported"); the path-templates section (its variable list has no engine).

**Not covered.** Whether Blender writes any format-native header field (an OpenEXR `software` attribute, a PNG text chunk) naming itself or the engine; whether the Note field is populated by default with anything; what a `.blend` saved after a render retains about the render.

**Outside the interface.** `scene.render.engine`, `bpy.context.engine`, `RenderResult.stamp_data_add_field` and `-E` are all data, Python or command-line, none of them tied to a window. `-E help` and `-E CYCLES` only exist outside the interface. The sources do not say any of this differs when Blender is driven from outside. **Settled by test on 2026-08-06, on Linux only — see below.** The engine's name reaches a finished image with no window involved, through the four `cycles.ViewLayer.*` stamp fields and, from 5.0 on, an OpenEXR `Software` attribute. `-E help` needs `-b` on a machine with no screen; in Blender 4.5 it never returns even then. **Added by the tests of 2026-08-07:** nothing about the engine changes when a graphics card does the work. Every route above behaves the same outside the window with the card in use as with the processor.

**Operating system.** Nothing in any of the above is qualified by operating system in the sources. Which engines are present could in principle vary with the build (see `bpy.app.build_options` under question 4, where `cycles=True` appears as a build option), but the sources do not tie that to a particular operating system. **A test on 2026-08-06 settled the Linux side — see below. macOS is untested.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder exactly as recorded under question 1: **the rented Linux machine**, `/opt/blender/4.5/blender` reporting `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, its own settings folder. **Nothing was run on macOS.**

**A finished image does carry a trace of the engine, and it is Cycles' own doing.** The same frame was rendered twice with every metadata switch on, once with `-E CYCLES` and once with `-E BLENDER_EEVEE_NEXT`. The Cycles file carried four fields the EEVEE file did not:

```
   cycles.ViewLayer.samples = 16
   cycles.ViewLayer.total_time = 00:00.02
   cycles.ViewLayer.render_time = 00:00.01
   cycles.ViewLayer.synchronization_time = 00:00.00
```

The EEVEE render of the same frame carried only the documented stamp fields and none of those four:

```
   File = /root/what-actually-ran/work/scene.blend
   Note = SMOOZY NOTE
   Date = 2026/08/06 22:59:28
   Marker = <none>
   Time = 00:00:00:01
   Frame = 1
   FrameRange = 1:3
   Camera = Camera
   Lens = 50.00
   Scene = Scene
   Strip = <none>
   RenderTime = 00:13.60
   Memory = 0.00M
   Hostname = ubuntu
```

Note the `4.5` detail: the EEVEE identifier in this version is `BLENDER_EEVEE_NEXT`, and using `BLENDER_EEVEE` gets an engine-not-found error instead of a render.

**Nothing 4.5 prints during a render names the engine.** The plain background render quoted under question 1 has no engine line, and neither does the run at `--log "*" --log-level -1`, which was searched for `Engine:` among other terms and matched nothing.

**No `Software` attribute is written into an OpenEXR by this version.** The Cycles OpenEXR from the run above was read attribute by attribute and carries none. (The same test in 5.0, 5.1 and 5.2 does find one; that is recorded in those versions' documents and is not a claim about this one.)

**`blender -E help` never returns in this version.** Run as `timeout -s KILL 25 /opt/blender/4.5/blender -b -E help </dev/null >o45.txt 2>e45.txt`, both files came back **empty** and the shell reported:

```
exit=137
Killed                  timeout -s KILL 25 /opt/blender/4.5/blender -b -E help
```

A status of 137 is the kill. Tried three times: twice with `-b` and once without. Without `-b`, on a machine with no screen, it first printed

```
GHOST: failed to initialize display for back-end(s): ['WAYLAND', 'X11']
  'WAYLAND': unable to connect to display!
  'X11': unable to open a display!
GHOST: unable to initialize, exiting!
```

and then hung the same way.

**An engine name Blender does not know is refused, and then this version hangs.** Run as `blender -b scene.blend -E NOT_AN_ENGINE -o out/be_ -F PNG -f 1`, the **error** stream carries:

```
Error: engine not found 'NOT_AN_ENGINE'
```

*Added by the citation check, 2026-08-07.* A repeat on the same machine gave that message, on the error stream, with standard output empty — and then the process **never returned**. It sat for nine minutes with the message already written and had to be killed. So no exit status can be recorded for this case on Linux in 4.5. It is the same hang this version shows for `-E help`, above, and it belongs in both places. Blender 5.0, 5.1 and 5.2 on the same machine print the same message and exit 1 straight away.

**The engine list a running Blender reports really does hold one entry.** Run under `--factory-startup`:

```
ENGINE 'BLENDER_EEVEE_NEXT'
ENGINE_ITEMS ['BLENDER_EEVEE_NEXT']
HAS_MULTIPLE_ENGINES True
```

So the single-entry list is not an artefact of how the documentation was generated — a running Blender says the same. `has_multiple_engines` exists in this version and reads True.

---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/4.5/blender`, which reported itself as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/4.5`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**Which part of the computer did the work changes nothing about what names the engine.** The metadata test was run again with the scene on GPU Compute and OptiX really doing the work: a Cycles render with every `use_stamp_*` switched on, to PNG and to OpenEXR. **The field set is identical to the processor's** — the same four `cycles.ViewLayer.*` fields (and, as before, no `Software` attribute in the OpenEXR — that is a 5.0-and-later thing, not a device thing). Nothing new names the engine, and no field names the card.

The one thing a card adds sits in the printed output, not in the file. `Building OptiX acceleration structure` (question 1) names a *technology*, not an engine — somebody reading only that line would know a graphics card was in use and still not know which engine had asked for it. Cycles is the only engine that can ask, on the evidence of `-E help`, but that is a step this run does not take.

`-E help`, the `render | Engine: Cycles` line and the engine list were settled on 2026-08-06 and are untouched by the card; they were not run again, so that a second result could not be laid over a first.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**Nothing in what 4.5 prints names the engine.** A background render with no switches, and the same render with every log switch this version has, carried no `Engine:` line anywhere in either stream. The same scene in 5.0, 5.1 and 5.2 on this same machine printed `render | Engine: Cycles`. So the difference is the version, not the operating system.

**`-E help` lists three engines and then stops badly.** Standard output:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

and then exit status **134** with `libc++abi: terminating` on the error stream, three times out of three. **This is not what the same command did on Linux**, where 4.5 printed nothing at all and had to be killed after twenty-five seconds. So the listing does exist in 4.5 and the switch is unusable in a different way on each operating system.

**But a running Blender's own engine list still holds one entry.** Read at factory startup with no window:

```
ENGINE 'BLENDER_EEVEE_NEXT'   ENGINE_ITEMS ['BLENDER_EEVEE_NEXT']   HAS_MULTIPLE_ENGINES True
```

So the short list is not an artefact of how the documentation was made, and it is not an artefact of the operating system either.

**Cycles writes four fields of its own into every image it renders, and EEVEE does not.** With every metadata switch on, the Cycles PNG carried `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time`; the EEVEE PNG of the same frame (engine `BLENDER_EEVEE_NEXT` in this version) carried none of them. Counted in all four versions: four in the Cycles file, none in the EEVEE file.

**The OpenEXR carries no `Software` attribute in this version.** Read on this machine, 4.5's Cycles OpenEXR has neither `Software` nor `colorInteropID`, while 5.0, 5.1 and 5.2 on the same machine wrote both — `Software (string) = Blender 5.0.1`, `Blender 5.1.2` and `Blender 5.2.0 LTS`. Same as the Linux runs.

---

## 3. When a render has finished, what can be learned about which add-ons were in use while it ran?

**Short answer:** no source I was given says anything about add-ons is recorded into the render. Blender can, on request, write a file that lists the add-ons currently enabled — but you have to ask for it, and it describes the session you ask in, not a render that has finished.

### What Blender will tell you, when asked

The system-info file has a section for exactly this:

> "Enabled Add-Ons
>
> Lists add-ons currently in use along with their versions and paths."

*Source:* manual, https://docs.blender.org/manual/en/4.5/getting_started/help.html — read 2026-07-30.
**LABEL: documented**

It is produced by an operator, so a script can trigger it:

> "bpy.ops.wm.sysinfo(*, filepath='')
>
> Generate system information, saved into a text file"

*Source:* API, https://docs.blender.org/api/4.5/bpy.ops.wm.html — read 2026-07-30.
**LABEL: documented**

The enabled add-ons are also a plain collection in the preferences:

> "addons
>
> Type: Addons bpy_prop_collection of Addon, (readonly)"

*Source:* API, https://docs.blender.org/api/4.5/bpy.types.Preferences.html — read 2026-07-30.

and each entry carries its module name:

> "class bpy.types.Addon(bpy_struct)
>
> Python add-ons to be loaded automatically
>
> module
>
> Module name"

*Source:* API, https://docs.blender.org/api/4.5/bpy.types.Addon.html — read 2026-07-30.
**LABEL: documented**

There is also a command that lists packages without any interface:

> "blender --command extension list [-h] [-s]
>
> List packages from all enabled repositories."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html — read 2026-07-30.
**LABEL: documented**

And add-ons can be switched on for a particular run:

> "--addons <addon(s)>
>
> Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.
**LABEL: documented**

The manual's production-deployment page shows a startup script enabling an add-on through a module called `addon_utils`:

> "def register():
> import addon_utils
> addon_utils.enable("my-addon")"

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30.
**LABEL: documented** that the module and that call exist. What else the module offers is **not in the sources**: searching the whole 4.5 API documentation for `addon_utils` returns nothing at all, and the only mention of it anywhere I was given is this one manual example.

### What is not recorded

The Metadata page's Include list has no add-on entry, and the panel picture shows none. Nothing in the manual, the API documentation, the release notes or Cycles' files says that a render writes a list of enabled add-ons anywhere — not into the image, not into a log, not into a file beside it. The only place a person could put such a list on their own is the free-text Note field, which the manual describes as taking "any information you like".

**LABEL: not in the sources** (for anything automatic)

Note also that the enabled-add-on list lives in the preferences, and preferences are stored separately from the scene:

> "./config/userpref.blend
>
> User preferences."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30.
**LABEL: documented** that this is where preferences live. That the blend-file being rendered therefore carries no record of them is **inferred**; the step is that a value stored in `userpref.blend` is not stored in the scene file, which the sources do not spell out.

**Where I looked.** Searched the whole manual, API, release notes and Cycles files for `addon`, `add-on`, `addons`, `addon_utils`, `extension`, `Enabled Add-Ons`, `sysinfo`, `System Info`, `manifest`, `metadata`, `stamp`. Read the Help menu page, the extensions command-line page, `bpy.types.Addon`, `bpy.types.Addons`, `bpy.types.Preferences`, `bpy.ops.wm.sysinfo`, the deploying-in-production page, the directory-layout page, and the Metadata page. Checked the Metadata page and the `RenderSettings` stamp properties specifically for the words `add-on`, `addon` and `extension` — none appear in either.

**Not covered.** What the system-info file's add-on section looks like in detail (the manual describes it, but no sample is in the sources); whether an add-on that misbehaves during a render leaves any trace naming itself; whether extensions record themselves in a blend-file that uses them.

**Outside the interface.** `bpy.ops.wm.sysinfo`, `preferences.addons`, `--addons` and `blender --command extension list` are all reachable without the interface — the last two exist only there, and `--command` is documented as implying background mode ("This implies --background mode.", command-line arguments page). The `Help ‣ Save System Info` menu route is interface-only, but the operator route is not. The Add-ons and Extensions preference panels are interface-only. **A test on 2026-08-06 settled the operator's behaviour with no window — see below.** **Added by the tests of 2026-08-07:** the add-on section of `system-info.txt` written from outside the window is unchanged by the card working, and no field a render writes names an add-on whichever device ran it.

**Operating system.** Nothing here is qualified by operating system in the sources. **The test below was run on Linux only; macOS is untested.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder exactly as recorded under question 1: **the rented Linux machine**, `/opt/blender/4.5/blender` reporting `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, its own settings folder. **Nothing was run on macOS.**

**Nothing about add-ons reaches a rendered image.** The fields written into a Cycles PNG and a Cycles OpenEXR with every stamp switch on are listed in full under question 1. None names an add-on, and none is an add-on field. The only Cycles-shaped fields are the four `cycles.ViewLayer.*` timing and sample fields recorded under question 2.

**`bpy.ops.wm.sysinfo` works with no window and its add-on section is filled in.** Run as `blender -b --python-expr "import bpy; print(bpy.ops.wm.sysinfo(filepath='.../si_4.5.txt'))"`, it returned `{'FINISHED'}` and wrote a 94-line file. The heading in the file is `Enabled add-ons:`, not `Enabled Add-Ons` as the manual writes it, and under it sat eight entries with versions and paths — the same eight in every version tested, differing only in their paths.

**The list that file holds is current, not stale.** In one session, without restarting, the file was written, an add-on switched off, and the file written again:

```
ON_BEFORE True
DISABLE {'FINISHED'}
ON_AFTER False
   io_anim_bvh in first file: 1  in second file: 0
```

So an add-on switched off part-way through a session is gone from the next report written in that session. This settles the open point recorded above under *Not covered* in the equivalent question elsewhere — the report describes the moment it is written.

**The enabled list a script reads.** Run as `blender -b --python-expr "import bpy; print('MODULES', sorted(a.module for a in bpy.context.preferences.addons))"`:

```
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

The same command with `--addons node_wrangler` printed the identical list and said nothing at all about the named add-on. **What that does and does not show:** `node_wrangler` is not among the eight this Blender ships, so the run cannot tell "loaded but not listed" apart from "nothing to load". What it does establish is that naming an add-on that is not there produces no message and no failure.

---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/4.5/blender`, which reported itself as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/4.5`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**The card changes nothing here, and the negative result is now stronger for it.** Every field written into a PNG and into an OpenEXR by a render that really ran on the graphics card was read back (see question 1's run). **No field names an add-on**, exactly as when the processor did the work.

The `Enabled add-ons:` section of `system-info.txt` is unchanged too. The file grew from 94 lines to 183 with the card repaired, and **every added line is in the Cycles section**, describing the card's capabilities. The add-on list, its versions and its paths are identical.

So the 2026-08-06 answer stands, with the case that was missing from it filled in: a finished render leaves no trace of which add-ons were loaded, whichever part of the computer did the work.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**`bpy.ops.wm.sysinfo` fills its add-on section in with no window.** The same eight modules the Linux machine ships, at macOS paths under `/Applications/Blender 4.5.12.app/Contents/Resources/4.5/scripts/addons_core/` — `io_anim_bvh`, `io_curve_svg`, `io_mesh_uv_layout`, `io_scene_fbx`, `io_scene_gltf2`, `cycles`, `pose_library` and `bl_pkg`, each with its version in brackets and its full path. The file written with a window open carries the identical section.

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

This is the question the sources cover least evenly. Some specific failures are documented with their exact wording; the general behaviour is not.

### Documented: it quietly makes do, in one named case

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

*Source:* manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html — read 2026-07-30.
**LABEL: documented.** Note what it does *not* say: it does not say anything is reported when this happens.

### Documented: it uses the processor when no device is configured

> "None:
>
> When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."

*Source:* manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html — read 2026-07-30.
**LABEL: documented**

### Documented: named errors, quoted exactly

> "Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU."

> "CUDA Error: Kernel compilation failed
>
> This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed. In this case Blender may try to dynamically build a kernel for your graphics card and fail."

> "The NVIDIA OpenGL driver lost connection with the display driver
>
> If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations."

> "Unsupported GNU version
>
> On Linux, depending on your GCC version you might get this error."

*Source:* all four from the manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html — read 2026-07-30.
**LABEL: documented**

> "Unsupported Graphics Driver Error
>
> This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender."

*Source:* manual, https://docs.blender.org/manual/en/4.5/troubleshooting/gpu/common/other.html — read 2026-07-30.
**LABEL: documented**

### Documented: where errors go

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html — read 2026-07-30.

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

> "Compilation errors will be shown in the system console." (about Open Shading Language scripts)

*Source:* manual, https://docs.blender.org/manual/en/4.5/render/cycles/osl/index.html — read 2026-07-30.
**LABEL: documented**

An engine also has two documented ways of saying something went wrong:

> "report(type, message)
>
> Report info, warning or error messages"

> "error_set(message)
>
> Set error message displaying after the render is finished"

*Source:* API, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html — read 2026-07-30.
**LABEL: documented**

### Observed in Cycles' own files

Cycles' files carry a line that ends the program when a device named on the command line is not accepted:

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```
*Source:* Cycles' own files, `cycles_4.5/engine.py`, lines 38–41 — read 2026-07-30.
**LABEL: observed** — that is what the file says. That a running Blender therefore stops with status 1 when `--cycles-device` names something unusable is **inferred**; the step is that `sys.exit(1)` at that point ends the process with that status, which the file does not state and I did not run.

The valid names are given in the same file's help text: "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'. Additionally, you can append '+CPU' to any GPU type for hybrid rendering." (Cycles' own files, `cycles_4.5/engine.py`, lines 16–20 — read 2026-07-30).

Cycles' files carry an error message about a build that lacks a feature:

```
            self.report({'ERROR'}, "OSL support disabled in this build")
```
*Source:* Cycles' own files, `cycles_4.5/__init__.py`, lines 112 and 119 — read 2026-07-30.
**LABEL: observed** — that this text sits in the file at those two places. That somebody asking for that feature in a build without it is shown this message is **inferred**; the file does not say when the line runs or what it produces, and I did not run it. Only a running Blender settles it.

Cycles' files also carry two lines of text about a missing library:

```
                sub.label(text="Cycles built without Embree support")
                sub.label(text="CPU raytracing performance will be poor")
```
*Source:* Cycles' own files, `cycles_4.5/ui.py`, lines 902–903 — read 2026-07-30.
**LABEL: observed** — that these two texts sit in the file at those places. That they are shown to anybody, and that a render goes ahead regardless, is **inferred**; the file does not say when these lines run or what they produce, and I did not run it. Only a running Blender settles it.

### Documented: what the installed Blender contains

Whether a build can provide a thing at all is readable:

> "bpy.app.build_options
>
> Constant value bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, … cycles=True, cycles_osl=True, freestyle=True, … image_openexr=True, … openvdb=True, alembic=True, usd=True, … experimental_features=False)"

*Source:* API, https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30.
**LABEL: documented.** (The values quoted are those of the build the documentation was generated from; the point is that the option names exist and can be read.)

The same idea appears for output formats on the command line, where the second list is explicitly conditional:

> "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.
**LABEL: documented.** What happens if you ask for one that is not there is **not in the sources**.

### Documented: when it does not survive at all

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."

with per-system detail: on Windows "a file is written based on the name of the currently loaded blend-file, so test.blend will create a file called test.crash.txt"; on macOS "Some .crash files can also be found in ~/Library/Logs/DiagnosticReports/"; on Linux "a file named blender.crash.txt is written to the /tmp directory."
*Source:* manual, https://docs.blender.org/manual/en/4.5/troubleshooting/crash.html — read 2026-07-30.
**LABEL: documented**

There is also a switch for making failure loud:

> "--debug-exit-on-error
>
> Immediately exit when internal errors are detected."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.
**LABEL: documented**

### What is not in the sources

Nothing I was given says:
- what exit status `blender` returns when a render fails, other than the Python-specific `--python-exit-code <code>` ("Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables");
- whether a partial or blank image file is still written when a render fails part-way;
- whether Cycles falls back from a chosen graphics card to the processor when that card turns out to be unusable at render time (the documented automatic fallback is only from graphics memory to system memory, which is a different thing);
- what wording any of these failures uses in a background run, as opposed to the interface;
- what happens when `-E` names an engine that is not installed, or `-F` names a format the build lacks.

**LABEL for the above: not in the sources**

**Where I looked.** Searched the whole manual, API, release notes and Cycles files for: `Error:`, `error`, `fail`, `fallback`, `fall back`, `falls back`, `not supported`, `unsupported`, `cannot render`, `will not render`, `render will fail`, `unable to`, `built without`, `not compiled`, `disabled in this build`, `exit code`, `exit-code`, `status code`, `return code`, `out of memory`, `crash`. Read in full: the Cycles GPU Rendering page (including its whole Error Messages section), the common GPU troubleshooting page, the crash page, the OSL page, the command-line arguments page, `RenderEngine` in the API, `bpy.app` in the API, and Cycles' `engine.py`, `__init__.py`, `operators.py`, `osl.py` and the relevant part of `ui.py`. Checked the EEVEE limitations page (it lists feature limits, not failure behaviour) and the 4.5 release notes for Cycles, EEVEE, core and bugfixes.

**Not covered.** Everything in the "not in the sources" list above. Also: the sources give error *texts* for a handful of situations but never a general rule for what a render does on encountering something it cannot do.

**Outside the interface.** Mixed, and the difference matters here. The error texts quoted from the GPU Rendering and troubleshooting pages are described without saying where they appear. `RenderEngine.report` and `error_set` are described in interface terms ("Report info, warning or error messages"; "Set error message displaying after the render is finished"), and the Info editor that logs such messages is an editor — "The Info editor logs the executed operators as well as errors, warnings, and informational messages" (manual, https://docs.blender.org/manual/en/4.5/editors/info_editor.html, read 2026-07-30). What is definitely available outside the interface is the console: errors are printed there, and `-q` keeps printing them. The `--cycles-device` rejection is a command-line-only path. Whether an error raised through `report` also reaches the console in a background run is **not in the sources**. **Settled in part by test on 2026-08-06, on Linux only — see below.** Every failure message found reached standard output or the error stream with no window: the `--cycles-device` refusal, the unwritable-output error, and the `-F` format complaint. The preferences-panel messages could not be reached at all with no window, as expected, and no substitute for them appeared anywhere. **Added by the tests of 2026-08-07:** `--cycles-device` exists only outside the window, and with the card working it can now be seen to succeed silently for `OPTIX` and `CUDA` and to refuse `HIP` and `ONEAPI` with `Found no Cycles device of the specified type`. The out-of-memory case is still out of reach, because this machine's own memory gives out before the card's does.

**Operating system.** Several of the documented failures are explicitly tied to one system: "The NVIDIA OpenGL driver lost connection with the display driver" is described as a Windows time-limit problem, "Unsupported GNU version" is "On Linux", and the crash-file locations differ on Windows, macOS and Linux (all from the pages cited above). Which devices exist per system is likewise documented (question 1). The rest is unqualified. **The tests below were run on Linux only. macOS is untested, and Windows was not tried at all — no Windows machine has run it, so every Windows-specific line above stays untested rather than confirmed.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder exactly as recorded under question 1: **the rented Linux machine**, `/opt/blender/4.5/blender` reporting `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, its own settings folder, and a graphics card that could not be reached. **Nothing was run on macOS.** Six different kinds of shortfall were tried, and they do not behave alike.

**A device demanded on the command line that Cycles will not give: exit 1 and no frame.** Five runs, each deleting the output first:

| `-- --cycles-device` | exit | frame written |
|---|---|---|
| `NOTADEVICE` | 1 | no |
| `OPTIX` | 1 | no |
| `CUDA` | 1 | no |
| `METAL` | 1 | no |
| `CPU` | 0 | yes |

A name Cycles does not know, on the **error** stream:

```
Error: NOTADEVICE is not a valid Cycles device.
```

A real backend with no hardware behind it — the first line on standard output, the second on the error stream:

```
Error: Found no Cycles device of the specified type
CUDA cuInit: Unknown error
```

`METAL` gave only the first of those two, with no CUDA line.

**A scene asking for GPU Compute where no card can be used: it renders anyway, and says nothing.** With `scene.cycles.device` set to `GPU` and the Cycles preference reading `NONE`, run as `blender -b scene.blend --python setgpu.py --log "*" --log-level -1 -o out/g_ -F PNG -f 1`:

```
exit=0
frame written
PREF_COMPUTE_DEVICE_TYPE NONE
SCENE_CYCLES_DEVICE GPU
Time: 00:00.25 (Saving: 00:00.21)
```

The same file with the device set to CPU gave `exit=0`, a frame, and `Time: 00:00.23`. **Nothing printed in either run names a device**, at any log level. So for this version the record is *finished, frame written, nothing reported* — which is not the same as a fallback and is not written here as one. The times are not evidence: the machine was shared with other work, the scene takes a fifth of a second, and the two runs differ by less than the spread between repeats.

**A render that cannot write its output: exit 1, and it says so.** With a plain file sitting where the output folder should be:

```
Error: Render error (Invalid argument) cannot save: '/root/what-actually-ran/out/fail/notadir/z_0001.png'
```

exit status 1.

**A format name that is not a format: it complains and renders anyway.** Run as `blender -b scene.blend -o out/f_ -F NOSUCHFORMAT -f 1`, exit **0**, a file `f_0001.png` written in the scene's own format, and this on the error stream:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
```

The wording covers a bad name and a missing build feature in one sentence, so the two cannot be told apart from it.

**A format that cannot hold metadata: nothing at all is said.** With every stamp switch on and `-F TIFF` and then `-F TGA`, both runs exited 0 and wrote their file. Counting lines in both streams matching any of `warn`, `error`, `metadata`, `not supported` or `preserved` gave 0 and 0. The metadata is simply gone, quietly.

**A render asked to stop part-way: a complete file, and exit 0.** — **This reading is wrong and is corrected further down this document.** *(Marked by the citation check, 2026-08-07.)* With adaptive sampling on, this scene converges and finishes by itself before six seconds, so the complete file and the exit 0 describe a render that ended normally, not one that was interrupted. The Mac tests of 2026-08-07 repeated it with `use_adaptive_sampling = False` and got status 143 and no file at all. Read that block, not this one. The original wording is kept below so the two runs can be told apart. A slow render (100000 samples at 1200 by 900) was started and sent a stop signal after six seconds:

```
exit after SIGTERM=0  file at output path: -rw-r--r-- 1 root root 622839 .../k_0001.png
```

The file was a valid PNG ending in `IEND`. **Nothing in the exit status, the file's size, its validity or its fields distinguishes it from a render that finished.**

**Two messages the sources describe cannot be provoked on this machine.** `bpy.app.build_options` reports `cycles_osl: True`, so no build here lacks Open Shading Language and "OSL support disabled in this build" cannot be reached. There is **no Embree entry in `build_options` at all** in this version, so nothing can say whether the build has it and "Cycles built without Embree support" cannot be reached either. Every conditional image format is compiled in — `image_openexr`, `image_openjpeg`, `image_webp`, `image_cineon`, `image_tiff`, `image_hdr` and `codec_ffmpeg` all True — so the "not compiled in this release" half of the format message above could not be provoked. This version reports 45 build fields, one more than 5.0, 5.1 and 5.2; the extra one is `collada: True`.

---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/4.5/blender`, which reported itself as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/4.5`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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

The wording, on the **error** stream, for the unknown name:

```
Error: NOTADEVICE is not a valid Cycles device.
```

and for a real backend with no hardware behind it — `OPTIX`, `CUDA`, `HIP` and `ONEAPI` all alike — on **standard output**:

```
Error: Found no Cycles device of the specified type
```

**One line the Linux runs carried is missing here.** On Linux the same case put `CUDA cuInit: Unknown error` on the error stream beside that sentence. On this machine there is no such line for any of the four absent backends — only the one sentence, and the error stream is empty.

**A scene set to GPU Compute with no Cycles device switched on renders anyway.** The Cycles preference set to `NONE` and every device unticked, the scene still asking for GPU Compute:

```
SET_NONE ok
PREF_COMPUTE_DEVICE_TYPE NONE
DEVICES_IN_USE [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', False)]
SCENE_CYCLES_DEVICE GPU
```

Exit 0, a frame written, the error stream 0 bytes, in all four versions. **In 4.5 nothing printed says what happened** at any log level; only `--debug-cycles` brings the line out, and then only on the error stream, where it reads `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]`. So the honest record for 4.5 is *finished, frame written, nothing reported* — which is not the same as a fallback and must not be written up as one.

**An unknown engine name is refused, and 4.5 then stops badly.** `-E NOT_AN_ENGINE`: no frame, and on the **error** stream:

```
Error: engine not found 'NOT_AN_ENGINE'
```

Exit status **134** here, against 1 in 5.0, 5.1 and 5.2 on the same machine. The same 134 and the same `libc++abi: terminating` follow `-E help` in this version — see question 2.

**A render that cannot write its output.** A plain file put where the output folder should be. Exit **1**, and one line on standard output:

```
Error: Render error (Not a directory) cannot save: '/tmp/smoozy-war/out/e20/fail/notadir/z_0001.png'
```

**The words in the brackets differ from the same test on Linux**, which gave `(Invalid argument)`. They are the operating system's words, not Blender's.

**An invalid `-F` name complains and renders anyway.** Exit 0, a file written in the scene's own format, and this on the **error** stream in all four versions:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
```

**A format that cannot hold metadata says nothing.** TIFF and TGA, every metadata switch on: exit 0, the file written, and no line in either stream matching `warn`, `error`, `metadata`, `not supported` or `preserved` — zero, in all four versions and both formats.

**"No compatible GPUs found for Cycles" could not be reached, and the panel shows why.** Blender started with a window, Preferences ▸ System shown and photographed by Blender's own screenshot operator. The Cycles Render Devices panel offers two buttons, `None` and `Metal`, with `Metal` chosen; under them a list holding `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked, then `Kernel Optimization: Full` and `MetalRT: Auto`. Setting the preference to `None` leaves the two buttons and **nothing else at all** — no list, no message. There is no CUDA, OptiX, HIP or oneAPI button on this machine, so there is no way to put the panel into the state that message belongs to. Pictures at `/tmp/smoozy-war/out/e28/prefs_4.5.png` and `/tmp/smoozy-war/out/e28/prefs_none_5.2.png`.

---

## 5. Where can each of the things above be learned from — what a render prints while it works, what it leaves on disk, or what can be asked afterwards?

Taking the three places in turn.

### What a render prints while it works

The console is documented as the place:

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors."
> "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html — read 2026-07-30.
**LABEL: documented**

That there is a running status print during a render follows from the switch that turns it off:

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.
**LABEL: documented** that such printing exists and can be suppressed. What it contains is **not in the sources** — no page in the 4.5 manual or API documentation shows a sample of a render's console output.

Cycles adds an optional stream of its own, on standard error:

> "--cycles-print-stats — Log statistics about render memory and time usage." (manual, command-line arguments page)
> `help="Print rendering statistics to stderr"` (Cycles' own files, `cycles_4.5/engine.py`, line 14 — read 2026-07-30)

**LABEL: documented / observed** respectively.

Blender can be told to print more, and where:

> "--debug-cycles — Enable debug messages from Cycles."
> "--log <match> — Enable logging categories, taking a single comma separated argument."
> "--log-level <level> — Set the logging verbosity level (higher for more details) defaults to 1, use -1 to log all levels."
> "--log-file <filepath> — Set a file to output the log to."
> "--debug-all — Enable all debug messages."

*Source:* manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.
**LABEL: documented.** Note `--log-file` moves the printed stream onto disk, which blurs the first two columns of this question. The category names to pass to `--log` are not listed anywhere in the sources beyond the examples `"wm.*"` and `"*undo*"`; there is no documented category for the renderer or for devices.

Python can be handed the same statistics as they are printed:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

*Source:* API, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30.
**LABEL: documented**

An engine feeds the running display through:

> "update_stats(stats, info) — Update and signal to redraw render status text"
> "update_progress(progress) — Update progress percentage of render"
> "update_memory_stats(*, memory_used=0.0, memory_peak=0.0) — Update memory usage statistics … Current memory usage in megabytes … Peak memory usage in megabytes"

*Source:* API, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html — read 2026-07-30.
**LABEL: documented**

In the interface, there is also a progress bar: "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate." (manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html, read 2026-07-30). **LABEL: documented**, and interface-only by nature.

### What it leaves on disk

- **The rendered image's metadata** — the stamp fields of question 1, but only in PNG, JPEG, OpenEXR and WebP (image-formats page).
- **Burned-in text** — "Burn into Image: Add metadata as text to the render", with font size, colours and "Include Labels" (Metadata page). This puts the same values into the pixels, where any format keeps them.
- **A log file**, if `--log-file <filepath>` was given.
- **`profile.json`** — "--profile-gpu … (Outputs a profile.json file in the Trace Event Format to the current directory)" (command-line arguments page). **LABEL: documented.** It is described as "CPU & GPU performance profiling for GPU debug groups"; nothing says it covers a Cycles render.
- **`system-info.txt`**, if `Help ‣ Save System Info` or `bpy.ops.wm.sysinfo` was used — carries the GPU, Cycles-device and enabled-add-on sections quoted earlier.
- **A crash file**, if Blender crashed — locations per system as quoted in question 4.

**LABEL: documented** for each, from the pages already cited.

### What can be asked afterwards

Inside Blender, through Python: `scene.render.engine` and `bpy.context.engine`; the Cycles settings and preferences shown in question 1; `RenderSettings.threads` and the `use_stamp_*` flags; `preferences.addons`; `bpy.app.build_options`, `bpy.app.version_string`, `bpy.app.build_date`, `bpy.app.build_hash`, `bpy.app.build_platform` and the rest of the `build_*` family; `bpy.app.background` ("True when blender is running without a user interface (started with -b)"); the `gpu.platform` calls. All are read at the moment you ask, and describe the session you ask in.

From an image file that was written: Blender can read its metadata back.

> "Metadata
>
> Lists image metadata." (Image editor sidebar)

*Source:* manual, https://docs.blender.org/manual/en/4.5/editors/image/sidebar.html — read 2026-07-30.

> "Lists information that has been encoded in the currently visible movie or image file … This also works for images produced by Blender; see Render Output for the metadata that can be included in this case."
> "The metadata can't be edited from Blender. Instead, you can use an external program such as exiftool."

*Source:* manual, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html — read 2026-07-30.
**LABEL: documented**

For movie files there is a Python call:

> "metadata()
>
> Retrieve metadata of the movie file
>
> Returns: Dict-like object containing the metadata"

*Source:* API, https://docs.blender.org/api/4.5/bpy.types.MovieStrip.html (and the identical entry on https://docs.blender.org/api/4.5/bpy.types.MovieClip.html) — read 2026-07-30.
**LABEL: documented**

No equivalent `metadata()` call is documented on `bpy.types.Image`; searching that page for `metadata` returns nothing.
**LABEL: not in the sources** (for reading a still image's metadata from Python)

### Mapping the earlier questions onto the three places

- **Which parts of the computer (Q1):** on disk, only host name and peak memory, and only via the stamp. Asked afterwards, the *settings* and the machine description (system-info, `gpu.platform`). Printed while it works, whatever `--cycles-print-stats` and the ordinary status printing contain — **not in the sources**.
- **Which engine (Q2):** on disk, nothing documented. Asked afterwards, the current setting. Printed, **not in the sources**.
- **Which add-ons (Q3):** on disk, only if `system-info.txt` was written on purpose. Asked afterwards, `preferences.addons` or `blender --command extension list`. Printed, **not in the sources**.
- **Failures (Q4):** printed to the console (errors survive `-q`); on disk in a crash file, or a log file if `--log-file` was given; asked afterwards, only what an engine chose to keep via `error_set`.

**Where I looked.** Searched the whole manual, API and release notes for `console`, `terminal`, `stderr`, `print`, `progress`, `status`, `log`, `log-file`, `quiet`, `debug-cycles`, `render_stats`, `update_stats`, `Fra:`, `Mem:`, `Peak`, `Remaining`, `Render Time`, `Statistics`, `metadata`, `exiftool`, `profile.json`. Read the command-line index and arguments pages, the command-line launch pages for all three systems, the status-bar page, the Info editor page, the Image editor sidebar page, the Video Sequencer preview sidebar page, `bpy.app.handlers`, `bpy.app`, `RenderEngine`, `MovieStrip`, `MovieClip` and `Image` in the API. Opened `_images/advanced_command-line_launch_windows_console.png` looking for a sample of console output — ruled out: it shows a Blender 2.77 console at startup ("Read new prefs…", "found bundled python…", "read blend…", "Blender quit"), not a render, and not this version.

**Not covered.** The actual shape and contents of what a render prints — no sample exists anywhere in the sources. Whether the stamp is written for video output as well as stills (some `use_stamp_*` descriptions say "image/video metadata" and others say "image metadata", which hints at a difference but does not settle it). Whether `--log` has any renderer-related category.

**Outside the interface.** The console, `--cycles-print-stats`, `--log-file`, `--profile-gpu`, image metadata on disk and the crash file are all available with no interface. The progress bar, the Info editor, the Image editor's Metadata panel and the Video Sequencer's Metadata tab are interface-only. One documented difference in the printed stream itself: `render_stats` is described as carrying "render/saving time plus **in background mode** frame/used [peak] memory" — so the statistics text is documented to differ between a background run and an ordinary one. On Windows the console is present but hidden: "The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." (manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, read 2026-07-30), and `-con, --start-console` is documented as "(Windows only)". **Settled by test on 2026-08-06, on Linux only — see below.** With no window, everything a render prints goes to standard output and the error stream stays empty, apart from the `-F` complaint and, in 4.5 only, the device and `--cycles-device` messages. `--cycles-print-stats` goes to standard output, not the error stream. There is no route from Blender's Python to a written file's metadata in any version, with or without a window. **Added by the tests of 2026-08-07:** the printed route is the one that names the card, and it works with no window. `--cycles-print-stats` replaces its profiling sections with `Profiling information not available (only works with CPU rendering)` when the card did the work. The on-disk route names no card at all. **Added by the re-run of 2026-08-07, after the citation check:** nothing in these three tests needs a window, and all three were run on a machine that has no display server at all.

**Operating system.** Crash-file locations differ per system (question 4). `-con/--start-console` is Windows only; `--debug-ghost` is "(Linux only)". The console itself exists on all three, with the Windows wrinkle above. Image metadata support is described per format, not per system. **The tests below were run on Linux only; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not. **Added by the re-run of 2026-08-07, after the citation check:** the three tests sent back by the citation check were run on that same Linux machine and nowhere else, so they are an NVIDIA-on-Linux answer; macOS and Windows are untouched by them.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder exactly as recorded under question 1: **the rented Linux machine**, `/opt/blender/4.5/blender` reporting `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, its own settings folder. **Nothing was run on macOS.** These runs close the biggest hole this answer names — that no sample of a render's printed output exists in the sources.

**Which stream everything lands on.** With the two streams captured to separate files, a plain background render put **everything on standard output and left the error stream empty** — 0 bytes. So did a run with `--cycles-print-stats`. The exceptions found in this version are all errors and one debug channel, and they go to the **error** stream: the `--cycles-device` refusal (question 4), the `-F` format complaint (question 4), the `--log-level` complaint below, and the `--debug-cycles` device line (question 1).

**What `--cycles-print-stats` prints, and where.** Run as `blender -b scene.blend -o out/s_ -F PNG -f 1 -- --cycles-print-stats`, exit 0. **The statistics went to standard output, and the error stream was 0 bytes** — the opposite of the "to stderr" that Cycles' own file says. They begin with a per-stage timing block:

```
Update statistics:
Scene:
    Total time: 0.000132s
        device_update                            0.000132s
Geometry:
    Total time: 0.000000s
Light:
    Total time: 0.000000s
Object:
    Total time: 0.000000s
Image:
    Total time: 0.000000s
Background:
    Total time: 0.000000s
Bake:
    Total time: 0.000000s
Camera:
    Total time: 0.000000s
Film:
    Total time: 0.000000s
Integrator:
    Total time: 0.000000s
OSL:
    Total time: 0.000000s
Particles:
    Total time: 0.000000s
SVM:
    Total time: 0.000000s
Tables:
    Total time: 0.000000s
Procedurals:
    Total time: 0.000000s
```

and continue into a `Render statistics:` block of mesh memory, image memory, per-kernel times and per-shader times. **No statistic names a device.**

**`--log-list-categories` does not exist in this version.** Run as `blender -b --log-list-categories`, exit 1, the error stream carrying:

```
unknown argument, loading as file: --log-list-categories
```

and standard output:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Error: Cannot read file "/root/--log-list-categories": No such file or directory
```

**`--log-level` takes a number in this version, not a name.** Run with `--log-level info`, the error stream carried:

```
Error: not a number '--log-level info'.
```

and **the render went ahead regardless, exiting 0**. With `--log-level -1` it printed 382 lines, exit 0; with `--log-level 2`, 86 lines, exit 0.

**A written render's metadata cannot be read back from Blender's own Python.** Run as `blender -b --python-expr` loading one of the stamped PNGs:

```
LOADED (160, 120)
HAS_metadata_attr False
stamp-ish attrs: []
```

The image loads and reports its size, so Blender read the file; there is no route from Python to the fields inside it. This closes the *not in the sources* recorded above.

**What `system-info.txt` holds when written with no window** is recorded under questions 1 and 3 — the operator works, but the GPU section reads `GPU: missing, background mode`.

---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/4.5/blender`, which reported itself as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/4.5`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**What a render prints while it works** gained four things, all of them new because the card works.

- The `Path tracing on:` line names the card, on the **error** stream as before (question 1).
- `Building OptiX acceleration structure` appears in the ordinary progress lines, with no switch at all — OptiX only, never CUDA.
- The memory figure in the progress lines reads `638.26M` where the processor's run read `0.75M`.
- `Mapped host memory limit set to 38,013,009,920 bytes. (35.40G)` is printed before the render starts, under a log switch. *(Qualified by the citation check, 2026-08-07: the line and the figure reproduced exactly in all four versions, but it needs `--debug-cycles` in 4.5 and `--log-level info` in 5.0, 5.1 and 5.2. A run with no switches never prints it.)*

**`--cycles-print-stats` prints something different when the card did the work, and the difference is itself a sign.** The statistics still go to **standard output** and the error stream is still empty (0 bytes) in every version, exactly as on 2026-08-06. The Mesh and Image sections are unchanged. But where the processor's run printed a Kernel statistics tree, a Shader statistics line and an Object statistics line, the card's run prints one sentence in their place:

```
Image statistics:
  Total image memory: 0B (0)
Profiling information not available (only works with CPU rendering)
```

So this switch answers *which part of the computer did the work* by what it refuses to print, and it names no device while doing it.

**What it leaves on disk** gained one thing and lost none. No field in a PNG or an OpenEXR names the card (question 1). The `Memory` stamp field carries the card's figure rather than the processor's, which is a hint and not a name. And the Render Time pass — see question 6 — turns out to record the scene's setting rather than what ran, so it is not a route to this at all.

**What can be asked afterwards** gained two routes and kept one trap. `bpy.ops.wm.sysinfo` now describes the card in full in its Cycles section, and the Cycles preferences list it by name before any render (question 1). The trap is unchanged: the settings read back after a run say what was asked for, not what happened.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**Which stream everything lands on, and 4.5 is the odd one out.** The ordinary progress lines, the `--cycles-print-stats` block, the `Found no Cycles device` refusal and the failure when a render cannot save all went to **standard output** with the error stream at 0 bytes. What reached the **error** stream in 4.5 were: the `Path tracing on:` device line under `--debug-cycles`, the `NOTADEVICE is not a valid Cycles device` refusal, the `-F` complaint, the `engine not found` refusal and the `--log-level` complaint. In 5.0, 5.1 and 5.2 on the same machine the first two of those are on standard output instead.

**`--log-list-categories` does not exist in this version.** Exit 1, and on the error stream:

```
unknown argument, loading as file: --log-list-categories
```

with, on standard output:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Error: Cannot read file "/private/tmp/smoozy-war/--log-list-categories": No such file or directory
```

**`--log-level` takes a number and refuses a name.** `--log-level info` put `Error: not a number '--log-level info'.` on the error stream and then **rendered anyway, exiting 0**. `--log-level -1` gave 430 printed lines and `--log-level 2` gave 86, both exit 0.

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

`render_cancel` did not fire; all six handlers exist; every handler is handed two arguments, not one. `render_stats` fired **42** times with no window. First and last:

```
HANDLER render_stats     args=('Fra:1 Mem:8.39M (Peak 9.10M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Cube', None)
HANDLER render_stats     args=('Time: 00:00.07 (Saving: 00:00.05)', None)
```

**With a window open, only 5.2 keeps handing the same strings over.** The same render, driven by `bpy.ops.render.render(write_still=True)` from a timer inside a Blender started with a window:

| version | `render_stats` calls, no window | `render_stats` calls, window open |
|---|---|---|
| 4.5 | 42 | **1** |
| 5.0 | 43 | **1** |
| 5.1 | 43 | **1** |
| 5.2 | 44 | **44** |

The one string this version hands over with a window open is the closing line, `('Time: 00:00.06 (Saving: 00:00.04)', None)` in 4.5. **The control matters and was run**: the same operator-driven render with no window gave 42, 43, 43 and 44, so the difference is the window and not the way the render was started. So a plan that watches `render_stats` to follow a render's progress works with no window in this version and does not work with one.

**Each frame of a background animation is knowable, but there is no line of its own.** 4.5 prints no `Rendering frame` line; the frame number sits in the prefix of every progress line instead:

```
Fra:1 Mem:8.39M (Peak 9.10M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Light
Saved: '/tmp/smoozy-war/out/e19/an_0001.png'
Fra:2 Mem:8.39M (Peak 9.33M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Light
Saved: '/tmp/smoozy-war/out/e19/an_0002.png'
Fra:3 Mem:8.39M (Peak 9.33M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Light
Saved: '/tmp/smoozy-war/out/e19/an_0003.png'
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

*What a negative result would look like, said before the run.* The switch accepted here. Blender 5.0, 5.1 and 5.2 were run identically as controls, and all three do accept it, so the refusal below is a fact about this version.

*What was done.* `blender -b --log-list-categories`, three times per version, both streams captured, and the output searched for a line reading exactly `gpu.opengl`.

*What came back, word for word.* **The switch does not exist in Blender 4.5.** Exit 1, three times out of three, identical each time. The error stream, in full — 57 bytes:

```
unknown argument, loading as file: --log-list-categories
```

and standard output, in full:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Error: Cannot read file "/root/--log-list-categories": No such file or directory

Blender quit
```

So there is no category list in this version to carry `gpu.opengl` or anything else. For the record of what the controls gave on the same machine on the same day: Blender 5.0 printed 170 lines, of which the first is its version banner, so **169 categories, and no `gpu.opengl` among them**; 5.1 printed 174 lines, 173 categories, `gpu.opengl` present; 5.2 printed 183 lines, 182 categories, `gpu.opengl` present.


## 6. At what point can each of the things above be learned — before a render starts, while it runs, or only once it has finished?

The sources let this be answered largely from the render handlers, which mark the moments Blender itself recognises.

> "bpy.app.handlers.render_init — on initialization of a render job. Accepts one argument: the scene datablock being rendered"
> "bpy.app.handlers.render_pre — on render (before)"
> "bpy.app.handlers.render_post — on render (after)"
> "bpy.app.handlers.render_stats — on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."
> "bpy.app.handlers.render_write — on writing a render frame (directly after the frame is written)"
> "bpy.app.handlers.render_complete — on completion of render job. Accepts one argument: the scene datablock being rendered"
> "bpy.app.handlers.render_cancel — on canceling a render job. Accepts one argument: the scene datablock being rendered"

*Source:* API, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30.
**LABEL: documented**

### Before it starts

Everything that is a *setting* or a *description of the machine* is available before anything is rendered, because none of it depends on a render having happened: the engine (`scene.render.engine`), the Cycles device and preferences, `RenderSettings.threads`, the enabled add-ons, `bpy.app.build_options` and the `build_*` values, `gpu.platform`, and `system-info.txt` on request. `render_init` marks the moment a job begins.
**LABEL: inferred** for the general claim. The step: each of these is documented as a property or an operator with no stated dependence on a render, so it can be read at any time — the sources do not say "before a render" in so many words for any of them. The `bpy.app` module carries one supporting line: "This module contains application values that remain unchanged during runtime." (API, https://docs.blender.org/api/4.5/bpy.app.html, read 2026-07-30) — **documented** for `bpy.app` specifically.

One caution that is stated outright, about asking the Cycles preferences for the device list:

```
    # Be careful when deciding when to call this function,
    # as Blender can crash with `_cycles.available_devices()` on some drivers.
    def get_device_list(self, compute_device_type):
```
and, on the refresh method:
```
    # Refresh device list. This does not happen automatically on Blender
    # startup due to unstable drivers that can cause crashes.
```
*Source:* Cycles' own files, `cycles_4.5/properties.py`, lines 1637–1639 and 1693–1695 — read 2026-07-30.
**LABEL: observed.** So the device list is not necessarily populated the instant Blender starts; something has to ask for it.

### While it runs

The console status printing, the progress bar, `render_stats`, `--cycles-print-stats`, and an engine's `update_stats` / `update_progress` / `update_memory_stats` all belong here. `render_pre` and `render_post` bracket each frame.
**LABEL: documented**, from the quotes in question 5.

### Only once it has finished

Two things are documented as arriving at the end and not before:

> "error_set(message)
>
> Set error message displaying after the render is finished"

*Source:* API, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html — read 2026-07-30.

and the stamp fields whose values cannot exist earlier — "Render Time: Includes the render time" and "Memory: Includes the peak memory usage" — which reach disk when the frame is written, at or about the moment `render_write` is documented to fire ("directly after the frame is written").
**LABEL: documented** for the quotes. Two steps beyond them, both **inferred**. First, that a render time and a peak-memory figure are only knowable at the end — the step being the plain meaning of "render time" and "peak" rather than anything the sources state. Second, that those values reach disk at the moment `render_write` fires — the step being that a handler said to run directly after the frame is written runs at the same point the frame's stamp is written, which no source says. The ordering of `render_write` against `render_post` is likewise not in the sources (see *Not covered* below).

### Per earlier question

- **Q1 (parts of the computer):** device *settings* — any time, before included. Peak memory and host name in the image — only once the frame is written. Which device actually did the work — **not in the sources at any point**.
- **Q2 (engine):** the setting, any time. Anything attached to the finished image — **not in the sources**.
- **Q3 (add-ons):** any time, from the preferences or `system-info.txt`. Nothing that only becomes knowable afterwards.
- **Q4 (failures):** as they happen, in the console; `error_set` explicitly after the render finishes; a crash file only after a crash.

**Where I looked.** Searched the whole manual and API for `render_init`, `render_pre`, `render_post`, `render_stats`, `render_write`, `render_complete`, `render_cancel`, `handlers`, `before`, `after`, `during`, `while rendering`, `at render time`. Read `bpy.app.handlers` and `bpy.app` in full, `RenderEngine` in full, the status-bar page, and the device-list section of Cycles' `properties.py`.

**Not covered.** Whether a value read *before* a render is still true *during* it — nothing in the sources addresses a device or add-on changing mid-job. Whether `render_stats` fires once per frame or more often. The ordering of `render_write` against `render_post`.

**Outside the interface.** The handlers are Python and carry no interface requirement; the handlers page's own warning about altering data mentions the viewport, which implies they run in an interface session too, but nothing says they do not run in a background one — and `render_stats` is documented to describe background mode specifically, which is direct evidence that these fire outside the interface. The progress bar is interface-only. **LABEL for the handlers running in background: inferred**, the step being that `render_stats`' own description names background mode. **Settled by test on 2026-08-06, on Linux only — see below.** All six render handlers fire with no window, in the order init, pre, post, write, complete, with the frame already on disk when `render_write` fires; `render_cancel` does not fire on a finished render. `bpy.app.is_job_running` works with no window and does not crash. So the inferred step above holds. **Added by the tests of 2026-08-07:** which parts of the computer are available can now be learned before a render starts, from outside the window; which part did the work can only be learned once the render has finished, from a closing report.

**Operating system.** Nothing about the timing of any of this is qualified by operating system in the sources. **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder exactly as recorded under question 1: **the rented Linux machine**, `/opt/blender/4.5/blender` reporting `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, its own settings folder. **Nothing was run on macOS.**

**The handlers do fire with no window, and this is the order they fire in.** A script was attached to `render_init`, `render_pre`, `render_post`, `render_write`, `render_complete`, `render_cancel` and `render_stats`, and the output file was deleted first so that checking for it means something. Run as `blender -b scene.blend --python handlers.py -o out/h_ -F PNG -f 1`:

```
HANDLER render_init      t=0.000 args=['Scene', 'NoneType']
HANDLER render_pre       t=0.001 args=['Scene', 'NoneType']
HANDLER render_post      t=0.143 args=['Scene', 'NoneType']
HANDLER render_write     t=0.143 args=['Scene', 'NoneType'] | file exists now: True
HANDLER render_complete  t=0.143 args=['Scene', 'NoneType']
```

Three things this settles, each shown by the run rather than argued. **`render_post` fires before `render_write`**, which the *Not covered* line above records as unknown. **The frame is already on disk when `render_write` fires** — `file exists now: True` — so the stamp reaches disk at or before that moment, which is the step marked inferred further up this answer. And **`render_cancel` did not fire**, which is what a finished render should give. All six handlers exist in this version; none reported itself missing.

**Every handler is handed two arguments, not the one the documentation describes.** The scene-level ones get the scene and a second argument of type `NoneType`; `render_stats` gets a string and a second argument of `None`.

**`render_stats` fired 42 times for one frame** — so it is not once per frame. Its first and last strings:

```
HANDLER render_stats     args=('Fra:1 Mem:8.42M (Peak 9.12M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Cube', None)
HANDLER render_stats     args=('Time: 00:00.14 (Saving: 00:00.10)', None)
```

In this version the string carries the frame number, the running memory and the peak inside itself. (The same test in 5.0, 5.1 and 5.2 gives a shorter string without them; that is recorded in those documents.)

**Each frame of a background animation is knowable while it runs, by reading the progress lines.** Run as `blender -b scene.blend -o out/an_ -F PNG -s 1 -e 3 -a`, this version has **no line of its own announcing a frame**; the number is in the prefix of every progress line instead:

```
Fra:1 Mem:8.42M (Peak 9.12M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Light
Saved: '/root/what-actually-ran/out/hd/an_0001.png'
Fra:2 Mem:8.42M (Peak 9.60M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Light
Saved: '/root/what-actually-ran/out/hd/an_0002.png'
Fra:3 Mem:8.42M (Peak 9.60M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Light
Saved: '/root/what-actually-ran/out/hd/an_0003.png'
```

**Asking whether a render is under way works with no window.** Run as `blender -b --python-expr "import bpy; print('IS_JOB_RUNNING', bpy.app.is_job_running('RENDER'))"`, exit 0, no crash:

```
IS_JOB_RUNNING False
```

**A value read after a render need not be the one that was in force during it.** A run overridden with `-- --cycles-device CPU`, whose scene said GPU Compute, printed from inside itself:

```
AFTER_OVERRIDE compute_device_type= NONE  scene.cycles.device= GPU
```

**The Render Time pass does not exist in this version.** Run as `blender -b --python-expr` checking for it:

```
HAS_pass_render_time False
```

(It exists in 5.0, 5.1 and 5.2; that is recorded in those documents and is not a claim about this one.)

---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/4.5/blender`, which reported itself as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/4.5`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**Before a render starts** — which parts of the computer are *available* is now answerable, and it was not on 2026-08-06. `refresh_devices()` and the Cycles preferences name the card with no render and no window; `bpy.ops.wm.sysinfo` describes it in full. What is still not answerable beforehand is which of them will do the work: `compute_device_type` reads `NONE` at factory startup with the card sitting there working, and the command line can override whatever the scene says.

**While it runs** — two signals arrive early. `Mapped host memory limit set to 38,013,009,920 bytes. (35.40G)` comes before the scene is even synchronized, and the memory figure in the progress lines is already the card's by the first sample line (`Mem: 638.26M | Sample 0/16`). The OptiX name arrives in the progress lines during the geometry step.

**Only once it has finished** — the `Path tracing on:` line, which is the only one that names the card, is a closing report. It is printed after the last sample and before the frame is written: in the 5.2 run, last sample at `00:02.270`, report at `00:02.270`, `Saved:` at `00:02.577`. So a run that is stopped part-way never prints it.

**One thing 4.5 cannot do at any point.** The `Render Time` pass does not exist in this version (`pass_render_time` is absent), so the whole of the paragraph above about it is a 5.0-and-later matter. In 4.5 there is nothing on disk that even purports to record this.

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

**The Render Time pass does not exist in this version.** `HAS_pass_render_time False`, and neither the CPU run nor the GPU run carried such a layer. In 5.0, 5.1 and 5.2 on this machine the property exists and the layer appears when the scene says CPU and not when it says GPU — including in a GPU run that genuinely ran on the card. So where the pass exists it reports an intention, not what ran.

**While it runs.** `render_stats` hands a string over dozens of times per frame with no window — 42 times for this scene in this version. **With a window open that stream of strings survives only in 5.2**; 4.5, 5.0 and 5.1 hand over one string, the closing time line. So a plan that watches `render_stats` to follow a render's progress works with no window in every version, and with a window only from 5.2.

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

2026-08-06 — Opus 5 (Claude Code) — What actually ran, Blender 4.5

# What actually ran — Blender 5.1 — second pass

**Sources consulted, and when.**
Blender's manual for 5.1, Blender's Python API documentation for 5.1, Blender's release notes for 5.1, and Cycles' own files as shipped with 5.1 — all four read on **2026-07-30** (the date the copies in `Blender Sources/` were taken). Blender's requirements page, read on **2026-08-01**.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

A note on the requirements page before anything else: it never says "5.1". The nearest it comes is a note that reads "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." I have not treated that page as speaking for 5.1 anywhere in what follows, except to say once, under question 4, that it does not name 5.1.

A note on labels: several of these questions have more than one true answer with different footings. Where that happens I have given each part its own label and a headline label for the question as a whole.

---

## 1. When a render has finished, what can be learned about which parts of the computer it used?

**Headline label: not in the sources**, for the plain reading — that the finished render tells you which processor or which graphics card did the work. Nothing in the sources says a finished render records that. What the sources *do* give are four narrower things, each labelled below.

### 1a. The image itself can carry the machine's name, the peak memory and the render time — and nothing that names the processor or the graphics card. — **documented**

The Metadata panel's full list of what can be written into a render is:

> "Date … Time … Render Time … Frame … Frame Range … Memory … Hostname … Camera … Lens … Scene … Marker … Filename … Strip Name … Note"

with these three being the ones that touch the machine:

> "Render Time
>
> Includes the render time."

> "Memory
>
> Includes the peak memory usage."

> "Hostname
>
> Includes the rendering machine's hostname."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html — read 2026-07-30.

The same list appears in the Python API, one property per item, with the same wording, for example:

> "use_stamp_hostname
>
> Include the hostname of the machine that rendered the frame (default False)"

*Corrected by the citation check, 2026-08-07: this was quoted as "(default True)". The page says `(default False)`, and the running Blender agrees — `use_stamp_hostname` reads False at factory startup in every version tested.*

> "use_stamp_memory
>
> Include the peak memory usage in image metadata (default True)"

Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

I also opened the picture the manual puts at the top of that page, `_images/render_output_properties_metadata_panel.png` (the "Metadata panel" screenshot on https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html, read 2026-07-30). It shows the panel with these tick-boxes and no others: Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Use Strip Me…, plus Note and Burn Into Image sections. **documented** — there is no tick-box for a processor, a graphics card, a device, or a render engine.

There is one escape hatch, and the manual points at it itself:

> "Hint
>
> It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number."

Same page and date. So anything about the machine can be *put* into a render by whoever set the render up — but it is not there of its own accord.

Only some file formats carry any of it:

> "Not all formats support metadata, and some may only support a limited subset of fields."
>
> "When using formats that do not support metadata, render information is not preserved in the saved file."

Source: the manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html — read 2026-07-30. The table at the top of that same page has a "Metadata" column; the formats with a tick in it are JPEG, OpenEXR, PNG and WebP. AVIF, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw and TIFF have a cross.

### 1b. There is one trace of the device inside a Cycles file — an extra pass that only appears when the scene was set to CPU. — **observed**

Cycles decides which passes go into the render result in Python, and one of them is gated on the device:

```
    # Debug passes.
    if crl.pass_debug_sample_count:
        yield ("Debug Sample Count", "X", 'VALUE')
    if crl.pass_render_time:
        # Only yield the pass if rendering on CPU
        if scene.cycles.device == 'CPU':
            yield ("Render Time", "X", "VALUE")
```

Source: `Blender Sources/cycles_5.1/engine.py`, lines 219–225, inside `list_render_passes` — read 2026-07-30.

The setting behind it says the same thing in words:

```
    pass_render_time: BoolProperty(
        name="Render Time",
        description="Reports time per pixel in milliseconds. Supported only on CPU render devices",
        default=False,
        update=update_render_passes,
    )
```

Source: `Blender Sources/cycles_5.1/properties.py`, lines 1488–1493 — read 2026-07-30.

What this gets you, and it is narrow: if the Render Time pass was switched on, then a saved multi-layer file containing a layer called "Render Time" tells you the *scene's* Cycles device setting was `CPU`. **inferred**, and here is the step — the quoted code only decides which passes are registered; that a registered pass ends up as a named layer in a saved multi-layer file is something the sources do not state, and I am supplying it. Worse, it reports the scene *setting*, not what actually ran; see 1c.

### 1c. What you can read back afterwards is the settings, and the settings can be overridden without changing. — **documented** and **observed**

Cycles' device settings are ordinary properties and can be read at any time. The scene-level one has two values only:

```
enum_devices = (
    ('CPU', "CPU", "Use CPU for rendering"),
    ('GPU', "GPU Compute",
        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
)
```

Source: `Blender Sources/cycles_5.1/properties.py`, lines 30–34 — read 2026-07-30. **observed**

The preference side holds the type and a list of the actual devices, each with a name, an identifier and a tick:

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

and

```
    compute_device_type: EnumProperty(
        name="Compute Device Type",
        description="Device to use for computation (rendering with Cycles)",
        default=CyclesPreferences.default_device(),
        items=CyclesPreferences.get_device_types,
    )

    devices: CollectionProperty(type=CyclesDeviceSettings)
```

Source: `Blender Sources/cycles_5.1/properties.py`, lines 1558–1566 and 1599–1606 — read 2026-07-30. **observed**

The device types are:

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

Source: `Blender Sources/cycles_5.1/properties.py`, lines 172–179 — read 2026-07-30. **observed**

Now the catch. A command-line switch can override both of these without altering either:

> "--cycles-device <device>
>
> Set the device used for rendering.
>
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. **documented**

Cycles' own help text for the same switch is blunter:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

Source: `Blender Sources/cycles_5.1/engine.py`, lines 16–20 — read 2026-07-30. **observed**

So reading the settings back after a render tells you what was configured, not necessarily what ran. **inferred**, and the step is: the switch is described as overriding the preference and the scene setting, and nothing in the sources says it writes its value back into them, so a later read of those settings can disagree with what ran.

### 1d. If the engine was EEVEE, a graphics card did the work. — **documented**, with an **inferred** step

> "CPU Rendering
>
> Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient. CPU power is still needed to handle high scene complexity as the geometry must be prepared by the CPU before rendering each frame."

> "Multiple GPU Support
>
> There is currently no support for multiple GPU systems."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html — read 2026-07-30.

The inferred step: knowing the render was made by EEVEE, you may conclude a graphics card did the rendering and that only one did. The source states the engine's nature; it does not state that you can read this backwards from a finished render.

### 1e. What the sources offer instead is a snapshot of the machine, taken when you ask for it — not when the render ran. — **documented**

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

Source: the manual, https://docs.blender.org/manual/en/5.1/getting_started/help.html — read 2026-07-30.

It is scriptable: `bpy.ops.wm.sysinfo(*, filepath='')`, described as "Generate system information, saved into a text file". Source: the API documentation, https://docs.blender.org/api/5.1/bpy.ops.wm.html — read 2026-07-30.

Cycles also exposes a system-information call of its own:

```
def system_info():
    import _cycles
    return _cycles.system_info()
```

Source: `Blender Sources/cycles_5.1/engine.py`, lines 172–174 — read 2026-07-30. **observed**. What it returns is not stated anywhere in the sources.

And the graphics side can be asked directly:

> "gpu.platform.renderer_get()
>
> Get GPU to be used for rendering.
>
> Returns:
>
> GPU name."

alongside `backend_type_get()`, `device_type_get()` ("‘APPLE’, ‘NVIDIA’, ‘AMD’, ‘INTEL’, ‘SOFTWARE’, ‘QUALCOMM’, ‘UNKNOWN’"), `vendor_get()` and `version_get()` ("Get GPU driver version"). Source: the API documentation, https://docs.blender.org/api/5.1/gpu.platform.html — read 2026-07-30.

Read the wording carefully. This is the `gpu` module, which is Blender's drawing layer. It says "GPU to be used for rendering", but the sources nowhere connect this module to Cycles' choice of compute device, and Cycles' device list comes from an entirely separate place (1c). Treating `gpu.platform.renderer_get()` as naming the card that rendered a Cycles frame would be my invention, not the source's claim.

**Not covered.** Nothing here settles which processor or graphics card actually performed a finished render. The sources never state that any such record exists — not in the image, not in a log, not in a property. Nor do they say what happens when several devices are used at once (the `+CPU` hybrid case), or whether anything distinguishes a frame rendered by two devices from one rendered by one. Nothing says how memory or render-time figures are measured.

**Outside the interface.** Everything in 1a, 1b and 1c holds when Blender is driven from outside its own window; metadata settings, device settings and passes are file and preference data, and `--cycles-device` exists only for that case. Two differences matter. First, the extra render statistics are described as background-mode-specific: the `render_stats` handler passes "the render stats (render/saving time plus in background mode frame/used [peak] memory)" (the API documentation, https://docs.blender.org/api/5.1/bpy.app.handlers.html, read 2026-07-30) — the memory figures are called out as background only. Second, the `gpu` module of 1e may not be there at all: a comment in the API's own render-engine example reads "# Lazily import GPU module, so that the render engine works in background mode where the GPU module can't be imported by default." (the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html, read 2026-07-30). Save System Info is described as a menu item; whether `bpy.ops.wm.sysinfo` works with no interface, the sources do not say. **Settled by test on 2026-08-06, on Linux only — see below.** `bpy.ops.wm.sysinfo` does work with no window, returning `{'FINISHED'}` and writing its file, so the inferred step above holds; but its GPU section then reads `GPU: missing, background mode`, so the route works and the hardware part of what it produces does not. The `gpu.platform` calls do **not** work with no window — they raise `SystemError` in every version. And the line naming the processor that did the work is reachable with no window, on standard output. **Added by the tests of 2026-08-07, with the machine's graphics card repaired:** driving Blender from outside its window is no obstacle to naming the card. With no window and no display server, a run prints the card's model in its `Path tracing on:` line, `bpy.ops.wm.sysinfo` describes the card in full in its Cycles section, and the Cycles preferences list it before any render. The GPU section of `system-info.txt` still reads `GPU: missing, background mode`, so that one section is genuinely lost outside the window and nothing else is. **Added by the re-run of 2026-08-07, after the citation check:** nothing in these three tests needs a window, and all three were run on a machine that has no display server at all.

**Operating system.** The Cycles device types are not all available everywhere — the manual states CUDA "is supported on Windows and Linux", OptiX "is supported on Windows and Linux", HIP "is supported on Windows and Linux", oneAPI "is supported on Windows and Linux", and "Metal for GPU acceleration is supported on Apple computers with Apple Silicon" (the manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html, read 2026-07-30). Cycles' own default reflects this: "# Default to selecting the Metal compute device on Apple Silicon GPUs" (`Blender Sources/cycles_5.1/properties.py`, lines 1573–1579, read 2026-07-30). The metadata fields themselves are not described as differing by operating system. For EEVEE there is one hard operating-system line: "Headless rendering is not supported on headless Windows systems." (the manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html, read 2026-07-30). **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not. **Added by the re-run of 2026-08-07, after the citation check:** the three tests sent back by the citation check were run on that same Linux machine and nowhere else, so they are an NVIDIA-on-Linux answer; macOS and Windows are untouched by them.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.1`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**One thing about this machine bears on every device result here.** Its graphics card could not be reached while the tests ran — the card's kernel module (580.95.05) and the libraries that talk to it (580.173.02) no longer match. `nvidia-smi` answered `Failed to initialize NVML: Driver/library version mismatch`, and Blender's own view was `cycles | ERROR CUDA cuInit: Unknown error` with an empty device list. So every result below is from a machine that has a graphics card and cannot use it.

**Blender 5.1 prints which processor did the work, by name, and `--log-level info` on its own is enough to bring it out.** The scene was the factory cube at 160 by 120, 16 samples, Cycles. Run as `blender -b scene.blend --log "*" --log-level info -o out/li_ -F PNG -f 1`, standard output carried:

```
00:00.600  cycles           | Full path tracing report:
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
   RenderTime = 00:00.02
   Memory = 1M
   Hostname = ubuntu
   cycles.ViewLayer.samples = 16
   cycles.ViewLayer.total_time = 00:00.02
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
GPU_ERROR SystemError GPU functions for drawing are not available in background mode
HAS_GPU_INIT False
```


### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.1`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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
00:02.170  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building OptiX acceleration structure
```

Lines of that shape appear three times in the OptiX run and **not once** in the CUDA run of the same scene. So at default settings the printed output can tell you OptiX was in use and cannot tell you CUDA was.

**The memory figure jumps when the card does the work, and that needs no switch either.** The same frame, the same scene, the figure Blender prints in its progress lines and writes into the `Memory` stamp field:

| what did the work | the figure |
|---|---|
| the processor | `1M` |
| the card | `650M` |

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

while the same run's `Path tracing on:` line named the processor. Both settings name the card's route; the processor did the work.

**A timing worth having, now that nothing else is running on this machine.** The same frame — 960 by 540, 1024 samples, adaptive sampling off — rendered twice, once with `-- --cycles-device OPTIX` and once with `-- --cycles-device CPU`:

| what did the work | wall clock | the `Time:` line Blender printed |
|---|---|---|
| the card, OptiX | 2.80 s | `Time: 00:02.09 (Saving: 00:00.21)` |
| the processor | 22.54 s | `Time: 00:21.66 (Saving: 00:00.32)` |

**This is one machine's timing, on one scene, on one day**, and it is offered as nothing more. What it does show is that the two are nowhere near each other: the card finished this frame about ten times sooner, which is far outside anything the spread between repeats could explain. The card's figure includes loading the render kernels, so it is if anything the slower of the two readings.

**Where I looked.** Searched the whole of the 5.1 manual, the whole of the 5.1 API documentation and the whole of the 5.1 release notes for: `metadata`, `stamp`, `system info`, `sysinfo`, `device used`, `which device`, `rendered on`, `GPU used`, `hardware used`, `records the`, `written into the image`, `embedded in the`, `render log`, `render report`, `provenance`, `render farm`, `render node`, `job number`, `print-stats`, `print_stats`, `exit code`, `render statistics`. Opened in full: the Metadata page, the Image Formats page, the Help menu page, the System preferences page, the GPU Rendering page, the EEVEE Limitations page, the Cycles Performance page, the Command Line Arguments page, the Rendering From The Command Line page, the File Paths (path templates) page, `bpy.types.RenderSettings`, `bpy.types.RenderResult`, `bpy.app`, `bpy.app.handlers`, `gpu.platform`, `bpy.utils`. Opened the Metadata panel picture. Searched all of Cycles' Python files for `device`, `stamp`, `system_info`. Ruled out: path templates (their full list of variables is quoted on the File Paths page and contains nothing about hardware — `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, `fps`, `resolution_x`, `resolution_y`, `scene_name`, `camera_name`, `node_name`); the Cycles source folder shipped here, which holds only kernel and utility headers (`Blender Sources/cycles_5.1/source/kernel` and `.../util`) and no session or device code that could speak to what a finished render records.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

**The machine.** The Creator's own Mac — a MacBook Pro with an **M1 Pro** chip, **macOS 26.5.2** (build 25F84), Apple's own graphics and no second card. Blender was `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)` — the same build hash as the Linux machine's 5.1, built at a different hour. Every run was pointed at a settings folder made before it was used (`BLENDER_USER_RESOURCES=/tmp/smoozy-war/settings/5.1`, and `/tmp/smoozy-war/settings2/5.1` where a run needed preferences that had been saved once) and never at the real one. Working files sat in `/tmp/smoozy-war/`; the prepared scene — the factory cube at 160 by 120, 16 samples, Cycles, denoising off — sat at `/Users/artist/Documents/Smoozy hand tests/What actually ran/scene.blend` and was built by Blender 4.5 so all four versions could open it. **Two other topics were running Blender on this Mac while these tests ran**, so nothing below is offered as a timing or a memory measurement. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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
| the graphics | `1385M` |

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

**The `gpu` module still answers nothing with no window in this version**, on a machine whose graphics work:

```
GPU_ERROR SystemError GPU functions for drawing are not available in background mode
HAS_GPU_INIT False
```

There is no `gpu.init` to call here — 5.2 alone offers one, and there it answers.

**The Metadata panel, seen on screen.** Blender started with a window on the factory scene, the Output Properties tab shown and its own search box set to `Metadata` so the panel opened, then Blender's own `screen.screenshot` operator. The panel showed `Metadata Input: Scene` and thirteen boxes: **Date, Time, Render Time, Frame, Camera, Scene and Filename ticked; Frame Range, Memory, Hostname, Lens, Marker and Strip Name unticked**, with `Note` and `Burn Into Image` as closed sub-panels, both unticked. **The Memory box is not ticked**, which matches what a running Blender reports and not the API page's stated default of True. Identical in all four versions. The picture is at `/tmp/smoozy-war/out/e27/panel_5.1.png`.

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

*What was done.* `/opt/blender/5.1/blender -b scene.blend --log-level info -o out/ -F PNG -f 1`, five times, both streams captured to separate files. The same run was then made three more times with the graphics card doing the work. This version was run as a control for a disagreement recorded in the 5.0 documents; the result is written here because it is this version's own answer as much as it is 5.0's control.

*What came back, word for word.* Exit 0 and a 0-byte error stream every time. The block, from the first of the five processor runs, uncut:

```
00:00.379  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: AMD EPYC 7B12 64-Core Processor (CPU) (24 threads) [CPU]
                            |                      Hardware Ray-Tracing: Off
                            | 
                            | Render Scheduler Summary
                            | 
                            | Mode: Headless
                            | Resolution: 160x120
```

**The `Hardware Ray-Tracing: Off` line is there, in all five runs**, and in all three of the runs with the graphics card doing the work, where the line above it reads `Path tracing on: NVIDIA GeForce RTX 4080 SUPER (OptiX) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07_OptiX]`. So it says `Off` even when an NVIDIA card with ray-tracing hardware is doing the work. The five runs are identical to one another once their timestamps are set aside, and identical to Blender 5.2's block in the same way. What the line means, and what would make it say anything other than `Off`, is not settled by any run here and is not claimed — the citation check sent that question back to research.

*The control did its job.* Blender 5.0, run identically on the same machine on the same day, has no such line in any of its eight runs. So the presence of the line here is a fact about the version.

*The disagreement this belongs to.* The disagreement sent back was in the 5.0 documents, where one run of 2026-08-06 recorded this line for 5.0 and every later run did not. Nothing in this version's record is disturbed: 5.1 has carried the line in every run anybody has made.

---

#### Test two — how many `Building OptiX acceleration structure` lines a run prints

*What a negative result would look like, said before the run.* The same number in all five runs of a version — which would have meant the two earlier counts were a miscount rather than a moving target, and the finding could then name the number with five repeats behind it. The CUDA runs beside them are the second way this test can fail: they show what the same count looks like when the answer is nought, so the counting is not something that only ever returns small positive numbers.

*What was done.* The shared scene rendered on the card with OptiX and **no log switches at all**, five times per version, counting the lines that carry `Building OptiX acceleration structure` on each stream. The card was pointed at by a script that sets the Cycles preference to OptiX and the scene to GPU, and prints the device list back so the run says rather than assumes what it used — it read `[('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', False), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False), ('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True)]` in every OptiX run. Five CUDA runs per version were made the same way, as the control.

*What came back.* The count is **not stable**. Five OptiX runs of the identical command in this version gave, in order:

```
3, 4, 3, 3, 4
```

and the five CUDA runs beside them gave `0, 0, 0, 0, 0`. Exit 0 and a frame written in all ten; the error stream was 0 bytes in all ten.

*Why it moves, read straight off the lines.* The lines are ordinary progress lines, and which of the geometry steps gets a line of its own varies from run to run. Two runs of this version, side by side, with the timestamps and memory figures taken out:

```
run 1
  render | Fra: 1 | Updating Geometry BVH light 4/3 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Geometry BVH Light 3/3 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Scene BVH | Building OptiX acceleration structure

run 2
  render | Fra: 1 | Updating Geometry BVH light 4/3 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Geometry BVH Light 3/3 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Geometry BVH Cube 2/1 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Scene BVH | Building OptiX acceleration structure
```

The `Updating Scene BVH` line is in every run of every version. The geometry lines above it are not. So the number is a fact about which progress steps happened to be reported, and never a fact about Blender or about OptiX.

*What this settles, and what must not be written.* Across the four versions the counts were 4.5 `3, 4, 4, 3, 4`; 5.0 `2, 4, 4, 2, 4`; 5.1 `3, 4, 3, 3, 4`; 5.2 `2, 4, 2, 3, 3`; and CUDA nought, twenty runs out of twenty. **The number must never be quoted as a fact about a version.** What holds, and held in every run anybody has made, is the half the claim rests on: lines of this shape appear in an OptiX run and never once in a CUDA run of the same scene. The right form of words is *more than once*, which is what the documents now say.


## 2. When a render has finished, what can be learned about which render engine produced it?

**Headline label: not in the sources** for the image itself, **documented** for the file that made it.

### 2a. The engine is not among the things written into a render. — **documented** (as an absence, from an exhaustive list)

The Metadata page's list of what can be included, and the panel picture, are quoted in full under 1a. Neither contains an engine field. The Python API's full set of `use_stamp_*` properties (also under 1a) contains no engine entry either. The engine can only reach a render through the free-text Note field, by someone deliberately putting it there.

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html and the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — both read 2026-07-30.

### 2b. The scene that produced the render holds the engine, and it can be read. — **documented**

> "engine
>
> Engine to use for rendering (default 'BLENDER_EEVEE')"

and

> "has_multiple_engines
>
> More than one rendering engine is available (default False, readonly)"

Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

The identifier for EEVEE is given there as `BLENDER_EEVEE`. Cycles' identifier comes from its own files:

```
class CyclesRender(bpy.types.RenderEngine):
    bl_idname = 'CYCLES'
    bl_label = "Cycles"
```

Source: `Blender Sources/cycles_5.1/__init__.py`, lines 42–44 — read 2026-07-30. **observed**. Its add-on record names it too:

```
bl_info = {
    "name": "Cycles Render Engine",
```

Source: `Blender Sources/cycles_5.1/__init__.py`, lines 7–8 — read 2026-07-30. **observed**

The manual's own example uses the same identifier on the command line: "-E CYCLES … Use the 'Cycles Render' engine." (the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30).

Blender ships three:

> "Blender includes three render engines with different strengths:
>
> EEVEE is a physically based realtime renderer.
>
> Cycles is a physically based path tracer.
>
> Workbench is designed for layout, modeling and previews.
>
> More renderers from third-party developers are available as add-ons."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/introduction.html — read 2026-07-30. The identifier for Workbench is **not in the sources**.

### 2c. The engine can be forced from the command line, so the file's engine and the render's engine can differ. — **documented**

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

**inferred**, step written out: if the engine can be specified on the command line, then the engine recorded in the file is not proof of the engine that produced a given render. The sources do not say whether `-E` writes its value back into the scene.

### 2d. An engine may add its own fields to a render result — but nothing in the sources says Cycles does. — **documented** for the mechanism, **not in the sources** for its use

> "stamp_data_add_field(field, value)
>
> Add engine-specific stamp data to the result
>
> Parameters:
>
> field (str) – Field, Name of the stamp field to add (never None)
>
> value (str) – Value, Value of the stamp data (never None)"

Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderResult.html — read 2026-07-30.

I searched every file in `Blender Sources/cycles_5.1` for the word `stamp` and found no occurrence at all. **observed**. That rules out Cycles' Python side using it; Cycles' compiled side is not in the sources, so it cannot rule out the whole engine.

### 2e. The pass names in a multi-layer file are engine-specific. — **observed**, and then **inferred**

Cycles' pass list includes names not shared with other engines — "Noisy Image", "Noisy Shadow Catcher", "Denoising Albedo", "Denoising Normal", "Denoising Depth", "Debug Sample Count", "Shadow Catcher", "Volume Majorant". Source: `Blender Sources/cycles_5.1/engine.py`, `list_render_passes`, lines ~180–260 — read 2026-07-30.

The inferred step, and it is a real step: finding a layer called "Noisy Image" in a saved multi-layer file would point at Cycles. The sources do not say these names are unique to Cycles, do not say what EEVEE or Workbench name their passes, and do not say that registered passes become layer names in a saved file. I would not lean on this.

**Not covered.** Nothing settles which engine produced a given image file from the file alone. The sources give no engine identifier for Workbench, say nothing about what a third-party engine add-on records, and say nothing about whether anything an engine writes with `stamp_data_add_field` survives into a saved file or under which formats.

**Outside the interface.** All of 2b, 2c and 2d hold when Blender is driven from outside its window; `-E` and `-E help` exist only for that case. There is one property that behaves differently and it is worth naming because it is easy to trip on: "use_single_layer — Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line." (the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html, read 2026-07-30). Nothing similar is said of `engine`. **Settled by test on 2026-08-06, on Linux only — see below.** The engine's name reaches a finished image with no window involved, through the four `cycles.ViewLayer.*` stamp fields and, from 5.0 on, an OpenEXR `Software` attribute. `-E help` needs `-b` on a machine with no screen; in Blender 4.5 it never returns even then. **Added by the tests of 2026-08-07:** nothing about the engine changes when a graphics card does the work. Every route above behaves the same outside the window with the card in use as with the processor.

**Operating system.** Nothing in the sources makes the engine identifier, `-E`, or the metadata list depend on the operating system. EEVEE carries the one operating-system limit already quoted: "Headless rendering is not supported on headless Windows systems." (the manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html, read 2026-07-30). Whether Cycles is present at all is a build matter rather than an operating-system one — `bpy.app.build_options` is documented with `cycles=True` among its entries (the API documentation, https://docs.blender.org/api/5.1/bpy.app.html, read 2026-07-30). **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.1`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**A finished image does carry a trace of the engine, in two ways, and one of them is new since 4.5.**

*Cycles writes four stamp fields of its own.* The same frame was rendered twice with every metadata switch on, once with `-E CYCLES` and once with `-E BLENDER_EEVEE`. The Cycles file carried four fields the EEVEE file did not:

```
   cycles.ViewLayer.samples = 16
   cycles.ViewLayer.total_time = 00:00.02
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
   RenderTime = 00:27.60
   Memory = 0M
   Hostname = ubuntu
```

So the hook the API describes for engine-specific stamp data is in use, and the field names begin with the engine's own name.

*An OpenEXR also carries a `Software` attribute.* The Cycles render to OpenEXR carried, among its attributes:

```
   Software (string) = Blender 5.1.2
   colorInteropID (string) = lin_rec709_scene
```

Neither appears in a PNG, and **neither appears in the equivalent file written by Blender 4.5** — that comparison was run on the same machine on the same day and is recorded in 4.5's own document. This closes the *Not covered* line above about a producer or software tag.

**The log names the engine at default settings.** The plain background render quoted under question 1 carries `render | Engine: Cycles` with no switch at all.

**`blender -b -E help` lists three engines.** Exit 0:

```
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
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

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.1`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**Which part of the computer did the work changes nothing about what names the engine.** The metadata test was run again with the scene on GPU Compute and OptiX really doing the work: a Cycles render with every `use_stamp_*` switched on, to PNG and to OpenEXR. **The field set is identical to the processor's** — the same four `cycles.ViewLayer.*` fields, and the same `Software` attribute in the OpenEXR naming Blender and its version. Nothing new names the engine, and no field names the card.

The one thing a card adds sits in the printed output, not in the file. `Building OptiX acceleration structure` (question 1) names a *technology*, not an engine — somebody reading only that line would know a graphics card was in use and still not know which engine had asked for it. Cycles is the only engine that can ask, on the evidence of `-E help`, but that is a step this run does not take.

`-E help`, the `render | Engine: Cycles` line and the engine list were settled on 2026-08-06 and are untouched by the card; they were not run again, so that a second result could not be laid over a first.

**Where I looked.** Searched the whole 5.1 manual, API documentation and release notes for `engine`, `render engine`, `CYCLES`, `BLENDER_EEVEE`, `BLENDER_WORKBENCH`, `stamp`, `metadata`. Opened: the Render Introduction page, the Metadata page, the Command Line Arguments page, the Rendering From The Command Line page, `bpy.types.RenderSettings`, `bpy.types.RenderResult`, `bpy.types.RenderEngine`, the Image Formats page, the File Paths page. Searched all Cycles Python files for `stamp` (no hits) and read `__init__.py` and `engine.py` in full. Ruled out: path templates as a way to put the engine in the filename — the complete variable list on the File Paths page (quoted under 1) has no engine variable.

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
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
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
   Software (string) = Blender 5.1.2
   colorInteropID (string) = lin_rec709_scene
```

Blender 4.5 on the same machine wrote neither attribute. The PNGs carry no `Software` field in any version. Same as the Linux runs.

---

## 3. When a render has finished, what can be learned about which add-ons were in use while it ran?

**Headline label: not in the sources** for the finished render, **documented** for the running Blender.

### 3a. Nothing about add-ons goes into a render. — **documented** (as an absence, from an exhaustive list)

The complete Metadata list and its panel picture are under 1a. There is no add-on field, no extension field, no script field. The only way an add-on's name reaches an image is if something writes it into the Note field.

### 3b. The enabled add-ons are a property of the running Blender and can be read out. — **documented**

> "class bpy.types.Addon(bpy_struct)
>
> Python add-ons to be loaded automatically
>
> module
>
> Module name (default "", never None)"

and

> "class bpy.types.Addons(bpy_prop_collection)
>
> Collection of add-ons"

reached through `Preferences.addons`. Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.Addon.html and https://docs.blender.org/api/5.1/bpy.types.Addons.html — read 2026-07-30.

The most complete written-down list is the system information file:

> "Enabled Add-Ons
>
> Lists add-ons currently in use along with their versions and paths."

Source: the manual, https://docs.blender.org/manual/en/5.1/getting_started/help.html — read 2026-07-30. Note the phrase: "currently in use". It is a snapshot of the moment you ask, not a record of a past render.

The manual also names the module that enumerates add-on locations:

> "import addon_utils
> print(addon_utils.paths())"

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html — read 2026-07-30. And in a deployment context: "import addon_utils / addon_utils.enable("my-addon")" (the manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html, read 2026-07-30).

Installed extensions can be listed without a script at all:

> "Subcommand: list
>
> usage:
>
> blender --command extension list [-h] [-s]
>
> List packages from all enabled repositories."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html — read 2026-07-30. That lists packages from repositories, which is not the same as the set that was enabled during a render.

### 3c. Add-ons can be added for one run from the command line. — **documented**

> "--addons <addon(s)>
>
> Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

**inferred**, step written out: because add-ons can be turned on for a single run, a list of enabled add-ons read at some later time need not match the list that was enabled during a particular render. The sources do not say `--addons` changes anything saved.

Two further things can move the set of add-ons in play, both **documented**: "Filter Add-ons — Determines which add-ons are enabled in the active workspace. When unchecked, the global add-ons will be used. When checked, you can enable individual add-ons in the list below." (the manual, https://docs.blender.org/manual/en/5.1/interface/window_system/workspaces.html, read 2026-07-30); and the whole Add-ons preferences section, which is about enabling and disabling them at will (the manual, https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html, read 2026-07-30).

### 3d. Cycles is itself an add-on. — **observed**

```
bl_info = {
    "name": "Cycles Render Engine",
    "author": "",
    "blender": (2, 80, 0),
    "description": "Cycles renderer integration",
    "warning": "",
    "doc_url": "https://docs.blender.org/manual/en/latest/render/cycles/",
    "tracker_url": "",
    "support": 'OFFICIAL',
    "category": "Render"}
```

Source: `Blender Sources/cycles_5.1/__init__.py`, lines 7–16 — read 2026-07-30. Its preferences are an add-on preferences class: `class CyclesPreferences(bpy.types.AddonPreferences): bl_idname = __package__` (`Blender Sources/cycles_5.1/properties.py`, lines 1569–1570, read 2026-07-30).

**inferred**, step written out: if a Cycles render happened, the Cycles add-on was in use, since the engine is delivered as one. That is the only add-on the sources let you conclude was in use from a finished render, and only if you already know the engine — which question 2 shows you often cannot.

**Not covered.** Nothing settles which add-ons were enabled during a past render. The sources say nothing about whether a blend-file records the add-ons that touched it, nothing about warnings when a file is opened without an add-on it once used, and nothing about add-ons being written into image metadata or logs. The system information file lists add-ons "currently in use", not the set at any earlier moment.

**Outside the interface.** `--addons` and `--command extension list` exist only for driving Blender from outside. The Python collections in 3b are readable in either case. The one part that is described purely as a menu action is Save System Info, which the manual presents under Help; its operator takes a filepath and so is scriptable, but the sources do not say whether it runs with no interface. Workspace add-on filtering (3c) is an interface concept and the sources do not say what it means when there is no interface. **Settled by test on 2026-08-06, on Linux only — see below.** `bpy.ops.wm.sysinfo` does work with no window in every version, and its Enabled add-ons section is filled in with versions and paths, so the inferred step above holds. **Added by the tests of 2026-08-07:** the add-on section of `system-info.txt` written from outside the window is unchanged by the card working, and no field a render writes names an add-on whichever device ran it.

**Operating system.** Nothing in the sources makes add-on listing, `--addons`, or the extension subcommands depend on the operating system. The one operating-system note near this ground is about where add-ons live, and it is about configuration rather than the operating system as such: "The destination of the add-on depends on your Blender configuration. When installing an add-on the source and destination paths are printed in the console." (the manual, https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html, read 2026-07-30). **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.1`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.1`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**The card changes nothing here, and the negative result is now stronger for it.** Every field written into a PNG and into an OpenEXR by a render that really ran on the graphics card was read back (see question 1's run). **No field names an add-on**, exactly as when the processor did the work.

The `Enabled add-ons:` section of `system-info.txt` is unchanged too. The file grew from 94 lines to 183 with the card repaired, and **every added line is in the Cycles section**, describing the card's capabilities. The add-on list, its versions and its paths are identical.

So the 2026-08-06 answer stands, with the case that was missing from it filled in: a finished render leaves no trace of which add-ons were loaded, whichever part of the computer did the work.

**Where I looked.** Searched the whole 5.1 manual, API documentation and release notes for `add-on`, `addon`, `addon_utils`, `enabled add-ons`, `addons currently`, `list of add-ons`, `which add-ons`, `--addons`, `extension`, `metadata`, `stamp`. Opened: the Add-ons preferences page, the Extensions Command Line Arguments page, the Extensions index and Getting Started pages, the Help menu page, the Add-on tutorial page, the Deploying Blender page, the Workspaces page, `bpy.types.Addon`, `bpy.types.Addons`, `bpy.types.Preferences`, `bpy.types.WorkSpace`. Ruled out: the Metadata page and `bpy.types.RenderSettings`, whose complete field lists have no add-on entry; the File Paths page, whose complete variable list has no add-on entry.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**`bpy.ops.wm.sysinfo` fills its add-on section in with no window.** The same eight modules the Linux machine ships, at macOS paths under `/Applications/Blender 5.1.2.app/Contents/Resources/5.1/scripts/addons_core/` — `io_anim_bvh`, `io_curve_svg`, `io_mesh_uv_layout`, `io_scene_fbx`, `io_scene_gltf2`, `cycles`, `pose_library` and `bl_pkg`, each with its version in brackets and its full path. The file written with a window open carries the identical section.

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

**Headline label: mixed.** The sources describe a handful of specific cases well and are silent on the general one. I read the question as being about capability the machine or the build lacks — a missing graphics card, a format not compiled in, a feature the engine cannot do. It could also be read as being about a scene asking for something absent from the file, such as a missing texture; the sources I was given say nothing useful about that reading and I have not pursued it.

### 4a. A device asked for on the command line that cannot be set: Blender stops there and then. — **observed**

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

Source: `Blender Sources/cycles_5.1/engine.py`, lines 38–41 — read 2026-07-30.

That is a plain exit with status 1. What, if anything, is printed alongside it is not in the sources — `set_device_override` is compiled and not among the files given.

### 4b. A command-line argument Blender does not know: it exits. — **documented**

> "Arguments must be separated by white space, eg:
>
> blender -ba test.blend
>
> …will exit since -ba is an unknown argument."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. The status it exits with is not stated.

### 4c. No suitable graphics card for Cycles: it says so, and says what would be needed. — **observed**

```
        if not found_device:
            col = box.column(align=True)
            col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)
```

followed by per-backend requirement lines, for example:

```
            if device_type == 'CUDA':
                compute_capability = "5.0"
                col.label(text=rpt_("Requires NVIDIA GPU with compute capability %s") % compute_capability,
                          icon='BLANK1', translate=False)
```

and

```
            elif device_type == 'METAL':
                mac_version = "12.2"
                col.label(text=rpt_("Requires Apple Silicon with macOS %s or newer") % mac_version,
                          icon='BLANK1', translate=False)
```

Source: `Blender Sources/cycles_5.1/properties.py`, lines 1806–1870 (inside `_draw_devices`) — read 2026-07-30.

This is drawn in the preferences panel. It is a message on screen, so it is of no use when Blender is driven from outside its own window.

### 4d. A scene set to GPU on a machine with no usable GPU: Cycles' own code treats that as CPU. — **observed**, then **inferred**

```
def use_cpu(context):
    cscene = context.scene.cycles

    return (get_device_type(context) == 'NONE' or cscene.device == 'CPU' or not backend_has_active_gpu(context))
```

Source: `Blender Sources/cycles_5.1/ui.py`, lines 104–107 — read 2026-07-30, where `backend_has_active_gpu` calls `has_active_device()`, which is `return self.get_num_gpu_devices() > 0` (`Blender Sources/cycles_5.1/properties.py`, lines 1753–1754, read 2026-07-30).

The manual says the same for the preference itself:

> "None:
>
> When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."

Source: the manual, https://docs.blender.org/manual/en/5.1/editors/preferences/system.html — read 2026-07-30. **documented**

**inferred**, step written out: the quoted function is a helper that decides what the interface offers, not the renderer's own decision. That Cycles *renders* on the CPU when the scene says GPU and no GPU is enabled is a step past what the file says. No source I was given states that a render falls back, or says whether anything is reported when it does.

### 4e. A graphics card that runs out of memory: some backends quietly borrow system memory; otherwise an error. — **documented**

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

and

> "Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html — read 2026-07-30. The same page carries "The NVIDIA OpenGL driver lost connection with the display driver", "Unsupported GNU version" and "CUDA Error: Kernel compilation failed" as named error messages, the last with the cause "This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed."

Note that oneAPI is absent from the list of backends that borrow system memory. The sources do not say what oneAPI does instead.

### 4f. EEVEE and memory: the sources say plainly that it can go badly and cannot be predicted. — **documented**

> "In practice, using too much GPU memory can make the GPU driver crash, freeze, or kill the application. So be careful of what you ask."
>
> "There is no standard way of estimating if the resources will fit into the GPU memory and/or if the GPU will render them successfully."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html — read 2026-07-30. The same page carries "Headless rendering is not supported on headless Windows systems." and "There is no plan to support CPU (software) rendering".

### 4g. A feature the build does not carry: an error report, where the sources show one. — **observed**

```
    def update_script_node(self, node):
        if engine.with_osl():
            from . import osl
            osl.update_script_node(node, self.report)
        else:
            self.report({'ERROR'}, "OSL support disabled in this build")
```

Source: `Blender Sources/cycles_5.1/__init__.py`, lines 106–111, and the same message again at line 118 for `update_custom_camera` — read 2026-07-30.

Other Cycles messages of the same kind, all **observed** from `Blender Sources/cycles_5.1/osl.py` — read 2026-07-30:

```
        report({'ERROR'}, "OSL script compilation failed, see console for errors")
```
(line 314), and

```
        report({'ERROR'}, "Custom Camera shader compilation failed, see console for errors")
```
(line 366). Both point at the console for the detail.

The reporting mechanism is documented: "report(type, message) — Report info, warning or error messages" and, importantly for a finished render, "error_set(message) — Set error message displaying after the render is finished". Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

### 4h. A file format the build does not carry: the manual warns, but does not say what happens. — **documented** for the warning, **not in the sources** for the behaviour

> "-F, --render-format <format>
>
> Set the render format.
> Valid options are:
> TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF.
>
> Formats that can be compiled into Blender, not available on all systems:
> OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

What can be checked in advance is documented: `bpy.app.build_options` is published as a constant whose value includes `cycles=True`, `cycles_osl=True`, `image_openexr=True`, `image_openjpeg=True`, `image_webp=True`, `codec_ffmpeg=True`, `compositor_cpu=True`, `openmp=False` and many more. Source: the API documentation, https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30. Those particular values describe the build the documentation was generated from, not necessarily any other build.

### 4i. A Python failure can be made to set the exit status. — **documented**

> "--python-exit-code <code>
>
> Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. This is the only exit status the sources put a number on besides 4a's `sys.exit(1)`.

### 4j. If it goes far enough wrong, a file is written. — **documented**

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."

with the details differing by operating system, quoted under "Operating system" below. Source: the manual, https://docs.blender.org/manual/en/5.1/troubleshooting/crash.html — read 2026-07-30.

**Not covered.** The sources do not say, for the general case, whether a render that cannot get what it asked for stops, degrades, or carries on. They give no exit status for a failed render, no catalogue of render error messages, and no statement about what is printed when the Cycles device override fails, when a format is not compiled in, or when a GPU is asked for and none is usable. They say nothing about what happens when Blender is driven from outside its window and the failure would normally be shown in a panel (4c) or in a post-render message (4g).

**Outside the interface.** This splits sharply. 4a, 4b, 4h and 4i are about the command line and hold only there. 4c is a preferences panel and is of no use with no interface. 4g's `report` and `error_set` are described in interface terms ("displaying after the render is finished") and the sources do not say where those messages go when there is no window; the two OSL messages that say "see console for errors" are the only pointer that anything reaches the console. 4e, 4f and 4j are engine and system behaviour and nothing marks them as interface-only. One general note applies: "-q, --quiet — Suppress status printing (warnings & errors are still printed)." (the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30) — which is the sources saying, in passing, that warnings and errors are printed at all. **Settled in part by test on 2026-08-06, on Linux only — see below.** Every failure message found reached standard output or the error stream with no window: the `--cycles-device` refusal, the unwritable-output error, and the `-F` format complaint. The preferences-panel messages could not be reached at all with no window, as expected, and no substitute for them appeared anywhere. **Added by the tests of 2026-08-07:** `--cycles-device` exists only outside the window, and with the card working it can now be seen to succeed silently for `OPTIX` and `CUDA` and to refuse `HIP` and `ONEAPI` with `Found no Cycles device of the specified type`. The out-of-memory case is still out of reach, because this machine's own memory gives out before the card's does.

**Operating system.** Several of these are operating-system specific and the sources say so outright. Cycles' backend requirement messages differ by system — the HIP branch reads `if sys.platform[:3] == "win"` with "and AMD Adrenalin driver 24.9.1 or newer" against `elif sys.platform.startswith("linux")` with "and ROCm HIP Runtime 6.0 or newer", and the oneAPI branch splits the same way (`Blender Sources/cycles_5.1/properties.py`, lines 1826–1870, read 2026-07-30). "Unsupported GNU version — On Linux, depending on your GCC version you might get this error." (the manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html, read 2026-07-30). EEVEE: "Headless rendering is not supported on headless Windows systems." Crash files differ completely: on Windows "a file is written based on the name of the currently loaded blend-file, so test.blend will create a file called test.crash.txt"; on macOS "the macOS Crash Reporter shows a window with backtrace after some time" and files may appear in `~/Library/Logs/DiagnosticReports/`; on Linux "a file named blender.crash.txt is written to the /tmp directory" (the manual, https://docs.blender.org/manual/en/5.1/troubleshooting/crash.html, read 2026-07-30). Blender's requirements page does not name 5.1, so I have taken nothing from it here. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.1`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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
00:00.635  render           | Time: 00:00.14 (Saving: 00:00.13)
```

The same file with the device set to CPU gave `exit=0`, a frame, the **identical** `Path tracing on:` line, and `Time: 00:00.23`. So the run is not silent about it. The times are not evidence and are not offered as any: the machine was shared with other work while these ran, the scene takes a fifth of a second, and the two runs differ by less than the spread between repeats.

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

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.1`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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

**Where I looked.** Searched the whole 5.1 manual, API documentation and release notes for `fallback`, `fall back`, `falls back`, `revert to CPU`, `use the CPU instead`, `not supported`, `unsupported`, `error`, `warning`, `exit code`, `exit-code`, `return code`, `exit status`, `out of memory`, `headless`, `background mode`, `requires a GPU`, `without a GPU`. Searched every Cycles Python file for `fall back`, `fallback`, `not supported`, `unsupported`, `error`, `warning`, `report(`, `sys.exit`. Opened in full: the GPU Rendering page, the EEVEE Limitations page, the Crashes page, the Startup Messages page, the Command Line Arguments page, the System preferences page, the Cycles Performance page, `bpy.types.RenderEngine`, `bpy.app`, and Cycles' `__init__.py`, `engine.py`, `operators.py` and `osl.py`. Ruled out: the 5.1 release notes, which I searched for `device`, `metadata`, `stamp`, `log`, `command line`, `add-on` and `addon` — the only entries touching this ground are a Cycles note that "For AMD GPUs, hardware ray-tracing is now enabled by default through HIP RT" (https://developer.blender.org/docs/release_notes/5.1/cycles/) and a Core note that "The Python user-site-directory is no longer loaded by default … Use the `--python-use-user-env` command line argument to restore access to user modules." (https://developer.blender.org/docs/release_notes/5.1/core/), both read 2026-07-30. Also ruled out: `Blender Sources/cycles_5.1/source`, which contains only kernel and utility headers and no session, device or error-handling code.

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

**"No compatible GPUs found for Cycles" could not be reached, and the panel shows why.** Blender started with a window, Preferences ▸ System shown and photographed by Blender's own screenshot operator. The Cycles Render Devices panel offers two buttons, `None` and `Metal`, with `Metal` chosen; under them a list holding `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked, then `Kernel Optimization: Full` and `MetalRT: Auto`. Setting the preference to `None` leaves the two buttons and **nothing else at all** — no list, no message. There is no CUDA, OptiX, HIP or oneAPI button on this machine, so there is no way to put the panel into the state that message belongs to. Pictures at `/tmp/smoozy-war/out/e28/prefs_5.1.png` and `/tmp/smoozy-war/out/e28/prefs_none_5.2.png`.

---

## 5. Where can each of the things above be learned from — what a render prints while it works, what it leaves on disk, or what can be asked afterwards?

**Headline label: mixed** — **documented** for the channels that exist, **not in the sources** for the exact text of anything printed.

### 5a. What a render prints while it works

That it prints at all is documented, twice over:

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html — read 2026-07-30. **documented**

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. **documented**

What can be turned on:

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

(the manual, same page and date) — and in Cycles' own words, with the destination named:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

Source: `Blender Sources/cycles_5.1/engine.py`, lines 13–15 — read 2026-07-30. **observed**. This is the only place in anything I was given that names which stream Blender writes to.

The logging switches, all **documented** from the manual's Command Line Arguments page, read 2026-07-30:

> "--log <match>
>
> Enable logging categories, taking a single comma separated argument.
>
> --log "*": everything
> --log "event": every category starting with event.
> --log "render,cycles": both render and cycles messages."

> "--log-level <level>
>
> Set the logging verbosity level.
>
> fatal: Fatal errors only
> error: Errors only
> warning: Warnings
> info: Information about devices, files, configuration, operations
> debug: Verbose messages for developers
> trace: Very verbose code execution tracing"

That `info` line is the one place in the sources where logging and devices are joined up.

Also: `--log-show-memory` ("Show memory usage for each log message"), `--log-list-categories` ("List all available logging categories for --log, and exit"), `--debug-cycles` ("Enable debug messages from Cycles"), `-d, --debug` ("Turn debugging on"), `--debug-all` ("Enable all debug messages"). And for video: "When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d." (the manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30).

**The exact lines Blender prints during a render are not in the sources.** I searched the whole manual, API documentation and release notes for `Fra:`, `Saved:`, `Blender quit`, `Time: 00` and found nothing. The manual's page on console messages documents exactly two lines, both from startup: "found bundled python: {DIR}" and "Read prefs: {DIR}/userpref.blend" (the manual, https://docs.blender.org/manual/en/5.1/troubleshooting/startup.html, read 2026-07-30). I opened the one console screenshot the manual carries, `_images/advanced_command-line_launch_windows_console.png` on https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html (read 2026-07-30). **documented** — it shows a Windows command prompt in which `blender` prints, in order, "Read new prefs: …\userpref.blend", "found bundled python: …", "read blend: C:\tmp\Vincent.blend", a blank line, and "Blender quit". It shows no rendering. The paths visible in it contain `Blender\2.77`, so it is a picture of a much older release and I would not lean on it for 5.1.

One printed thing can be caught in Python rather than read off a screen:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

Source: the API documentation, https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30. **documented**. This is a string handed to your own code at the moment statistics are printed. The sources do not give its format.

### 5b. What a render leaves on disk

- **The image, with metadata.** Everything under 1a. Only JPEG, OpenEXR, PNG and WebP carry any. **documented**
- **Passes as layers.** The Cycles pass list of 1b/2e. **observed**
- **A log file, if asked for.** "--log-file <filepath> — Set a file to output the log to." **documented**, the manual's Command Line Arguments page, read 2026-07-30.
- **A GPU profile, if asked for.** "--profile-gpu — Enable CPU & GPU performance profiling for GPU debug groups (Outputs a profile.json file in the Trace Event Format to the current directory)". Same page and date. **documented**
- **A system information file, if asked for.** `system-info.txt` — 1e and 3b. **documented**
- **Crash files, if it crashes.** 4j, and per operating system. **documented**
- **Placeholder frames, if asked for.** "Image Sequence – Overwrite Image Multi-Layer EXR / Create empty placeholder frames while rendering." and the "Primitive Render Farm" hint that follows it (the manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30). **documented** — these say a frame is being worked on, and nothing about the machine.
- **Shader sources and traces**, only under debug switches (`--debug-gpu-shader-source` writes "Files … in the current working directory inside a directory named "Shaders""). Same Command Line Arguments page and date. **documented**

### 5c. What can be asked afterwards

All from the API documentation, read 2026-07-30, except where noted:

- The engine: `RenderSettings.engine`, `has_multiple_engines` (https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html). **documented**
- The enabled add-ons: `Preferences.addons`, a "Collection of add-ons", each with a `module` (https://docs.blender.org/api/5.1/bpy.types.Addons.html, https://docs.blender.org/api/5.1/bpy.types.Addon.html). **documented**
- The build: `bpy.app.version`, `version_string`, `build_hash`, `build_date`, `build_platform`, `build_system`, `build_type`, `build_options`, `binary_path`, `background`, `factory_startup`, `portable`, `tempdir` (https://docs.blender.org/api/5.1/bpy.app.html). **documented**
- The graphics layer: `gpu.platform.renderer_get()`, `vendor_get()`, `version_get()`, `device_type_get()`, `backend_type_get()` (https://docs.blender.org/api/5.1/gpu.platform.html), with the two cautions in 1e. **documented**
- Cycles' devices: `compute_device_type`, `devices` with `id`, `name`, `type`, `use`, and `has_active_device()` / `get_num_gpu_devices()` / `has_multi_device()` (`Blender Sources/cycles_5.1/properties.py`, read 2026-07-30). **observed**
- Whether a render is under way: "bpy.app.is_job_running(job_type) — Check whether a job of the given type is running", with `RENDER: Regular rendering.` among the job types (https://docs.blender.org/api/5.1/bpy.app.html and the Wm Job Type Items list at https://docs.blender.org/api/5.1/bpy_types_enum_items/wm_job_type_items.html). **documented**
- Without any script at all: `blender -v` ("Print Blender version and exit"), `blender -E help` ("Use -E help to list available engines"), `blender --command extension list`, `blender --log-list-categories`, and `blender --command sysinfo` if you register such a command — the API documentation gives a worked example that "Report basic system information", printing `bpy.app.version_string`, `platform.platform()`, `platform.machine()` and `os.cpu_count()` (https://docs.blender.org/api/5.1/bpy.utils.html). **documented**

**Not covered.** The exact text, format and stream of everything Blender prints during a render, apart from the single line that `--cycles-print-stats` goes "to stderr". Whether the log switches capture Cycles' statistics. Whether metadata is written for video output as well as stills, beyond the several `use_stamp_*` descriptions that say "image/video metadata". What the log file contains. What `_cycles.system_info()` returns.

**Outside the interface.** This is where the channels part company. Everything in 5a and 5b is either only available from outside the window (the command-line switches) or unaffected by it (the written image). Everything in 5c works either way except the `gpu` module, which "can't be imported by default" in background mode (the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html, read 2026-07-30), and Save System Info, whose behaviour with no interface the sources do not state. The `render_stats` handler is explicitly richer outside the window: the memory figures are described as background-mode only. The manual's own advice for checking on work done with no interface is: "If the results can be displayed as text then print them or write them to a file." (the API documentation, https://docs.blender.org/api/5.1/info_tips_and_tricks.html, read 2026-07-30). **Settled by test on 2026-08-06, on Linux only — see below.** With no window, everything a render prints goes to standard output and the error stream stays empty, apart from the `-F` complaint and, in 4.5 only, the device and `--cycles-device` messages. `--cycles-print-stats` goes to standard output, not the error stream. There is no route from Blender's Python to a written file's metadata in any version, with or without a window. **Added by the tests of 2026-08-07:** the printed route is the one that names the card, and it works with no window. `--cycles-print-stats` replaces its profiling sections with `Profiling information not available (only works with CPU rendering)` when the card did the work. The on-disk route names no card at all. **Added by the re-run of 2026-08-07, after the citation check:** nothing in these three tests needs a window, and all three were run on a machine that has no display server at all.

**Operating system.** The console itself differs: "-con, --start-console — Start with the console window open (ignored if -b is set), (Windows only)." and on Windows "the Console Window … will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." (the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html, read 2026-07-30). Where temporary files land differs: "TEMP: Store temporary files here (MS-Windows)." against "TMPDIR: Store temporary files here (UNIX Systems)." (the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30). Crash files differ entirely, as quoted under 4. The logging switches, `--log-file`, metadata and the Python routes are not described as differing. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not. **Added by the re-run of 2026-08-07, after the citation check:** the three tests sent back by the citation check were run on that same Linux machine and nowhere else, so they are an NVIDIA-on-Linux answer; macOS and Windows are untouched by them.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.1`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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

**`--log-list-categories` exists in this version and lists 174 categories.** Exit 0. The list opens `addon`, `anim`, `anim.action`, `anim.data`, `anim.driver`, and the categories bearing on this topic are `cycles`, `render`, `hydra.render`, `compositor.gpu`, `image.gpu` and a family of `gpu.*` entries. There is an `addon` category. The device line lives under `cycles` at `info` level — see question 1. (In Blender 4.5 the switch does not exist at all; that is recorded in 4.5's own document.)

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

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.1`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**What a render prints while it works** gained four things, all of them new because the card works.

- The `Path tracing on:` line names the card, on **standard output** as before (question 1).
- `Building OptiX acceleration structure` appears in the ordinary progress lines, with no switch at all — OptiX only, never CUDA.
- The memory figure in the progress lines reads `650M` where the processor's run read `1M`.
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

**Where I looked.** Searched the whole 5.1 manual, API documentation and release notes for `Fra:`, `Saved:`, `Blender quit`, `Time: 00`, `console`, `stdout`, `stderr`, `log`, `print-stats`, `render statistics`, `metadata`. Opened: the Using Blender From The Command Line index, the three Launching pages, the Command Line Arguments page, the Rendering From The Command Line page, the Startup Messages page, the Crashes page, the Output properties page, the Metadata page, the Image Formats page, `bpy.app`, `bpy.app.handlers`, `bpy.utils`, `bpy.types.RenderEngine`, `gpu.platform`, `info_tips_and_tricks`. Opened the Windows console screenshot. Searched Cycles' Python files for `print`, `stderr`, `stats`.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**Which stream everything lands on.** In this version, every one of these went to **standard output** and left the error stream at 0 bytes: the ordinary progress lines, the `Engine:` line, the `Path tracing on:` device line at `--log-level info`, the whole `--cycles-print-stats` block, the `Found no Cycles device` refusal, and the two-line failure when a render cannot save. The only things that reached the **error** stream were the `-F` complaint and the `engine not found` refusal.

**`--log-list-categories` works, and the list is shorter on macOS with different graphics categories.** Exit 0 and **139 categories** on this machine, against 174 for the same version on Linux. The graphics ones here:

```
compositor.gpu
cycles
gpu.debug.metal
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

`render_cancel` did not fire; all six handlers exist; every handler is handed two arguments, not one. `render_stats` fired **43** times with no window. First and last:

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

The one string this version hands over with a window open is the closing line, `('Time: 00:00.06 (Saving: 00:00.04)', None)` in 4.5. **The control matters and was run**: the same operator-driven render with no window gave 42, 43, 43 and 44, so the difference is the window and not the way the render was started. So a plan that watches `render_stats` to follow a render's progress works with no window in this version and does not work with one.

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

*What came back.* **This version's list carries `gpu.opengl`.** Exit 0, three runs, identical each time; 174 lines, of which the first is the version banner, so **173 categories**. Every category whose name contains `gpu`, in full:

```
compositor.gpu
gpu.debug
gpu.opengl
gpu.platform
gpu.shader
gpu.shader.deps
gpu.texture
gpu.vulkan
image.gpu
```

*Where the disagreement was.* It was in the 5.0 documents, over whether the Linux 5.0 list carries `gpu.opengl`. These runs are that question's control: on the same machine on the same day, 5.0 has no such entry and 5.1 has one, so the entry arrives at this version. Blender 5.2 also has it; Blender 4.5 has no such switch at all.


## 6. At what point can each of the things above be learned — before a render starts, while it runs, or only once it has finished?

**Headline label: documented**, from the descriptions of the handlers and command-line switches; with the timing of two things — metadata and log lines — **inferred** from what those descriptions say.

### 6a. Before a render starts

All of these need no render to have happened. **documented**, sources as cited in questions 1–5, read 2026-07-30 (Cycles files **observed**).

- Which engines the build has: `blender -E help`.
- Which engine the file will use: `RenderSettings.engine`.
- What the build can do at all: `bpy.app.build_options`, `bpy.app.version`, `blender -v`.
- Which add-ons are enabled: `Preferences.addons`; which extensions are installed: `blender --command extension list`.
- Which Cycles devices exist and which are ticked: the preference `devices` collection and `compute_device_type`; `has_active_device()`.
- What the machine is: Save System Info, `gpu.platform.*`.
- A handler that fires at the very start: "bpy.app.handlers.render_init — on initialization of a render job. Accepts one argument: the scene data-block being rendered", and "render_pre — on render (before)". (https://docs.blender.org/api/5.1/bpy.app.handlers.html)

One caution that belongs here: Cycles does not look for devices when Blender starts.

```
    # Refresh device list. This does not happen automatically on Blender
    # startup due to unstable drivers that can cause crashes.
    def refresh_devices(self):
```

Source: `Blender Sources/cycles_5.1/properties.py`, lines 1707–1709 — read 2026-07-30. **observed**. **inferred**, step written out: reading the `devices` collection without first triggering a refresh may therefore show a stale or empty list; the comment says the list is not refreshed automatically, and I am supplying the consequence.

### 6b. While it runs

**documented**, all from https://docs.blender.org/api/5.1/bpy.app.handlers.html and https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html, read 2026-07-30:

- "render_stats — on printing render statistics" — this is the moment the render/saving time, and in background mode the frame and used/peak memory, become available as a string.
- "render_write — on writing a render frame (directly after the frame is written)" — the point at which a file, with whatever metadata it carries, exists on disk.
- "render_post — on render (after)" and "render_cancel".
- From the engine side: `update_stats(stats, info)` "Update and signal to redraw render status text", `update_progress(progress)` "Percentage of render that's done", `update_memory_stats(*, memory_used=0.0, memory_peak=0.0)` "Update memory usage statistics", `report(type, message)`.
- `bpy.app.is_job_running('RENDER')`.
- Everything printed to the console or to `--log-file` arrives while the render runs. **inferred**, step written out: the switches are described as printing or logging, and the manual describes the console as showing "messages about Blender's operations, status, and internal errors" as they happen; the sources do not state the timing of individual lines.

### 6c. Only once it has finished

**documented**:

- "render_complete — on completion of render job."
- "error_set(message) — Set error message displaying after the render is finished." This is by its own words a finished-render thing.
- The metadata fields that describe the run rather than the scene — Render Time, Memory (peak), Hostname — can only be complete when the frame is done. **inferred**, step written out: the manual says Render Time "Includes the render time" and Memory "Includes the peak memory usage"; neither figure can be final before the frame ends, and the frame is written once. The sources do not say when the values are gathered.
- Whatever the finished file contains: its metadata (1a), its passes (1b, 2e).
- Cycles' `--cycles-print-stats` output, described as "statistics about render memory and time usage" — the sources do not say whether it prints during or at the end. **not in the sources** for its timing.

### 6d. Which question falls where

- **Question 1, the machine's parts.** The device *settings* — any time, before included. The hostname, peak memory and render time in the image — after, and only if written. Which processor or graphics card actually did the work — never, from these sources.
- **Question 2, the engine.** The scene's engine setting — any time, before included. The engine that actually produced a given image — the sources give no after-the-fact route at all, only the settings and, at a stretch, the pass names.
- **Question 3, the add-ons.** The enabled list — any time, but always as of the moment you ask. For a past render — never, from these sources.

**Not covered.** The sources give no ordering between handlers beyond their names, no statement of when metadata values are captured, no timing for `--cycles-print-stats`, and nothing on how any of this behaves across a multi-frame animation as opposed to a single frame.

**Outside the interface.** The handlers, `is_job_running` and the engine methods are Python and are not described as interface-only, but `update_stats` says "signal to redraw render status text" and `error_set` says "displaying after the render is finished" — both are worded as if a window exists, and the sources do not say what they do without one. `render_stats` is the opposite case: it is documented as carrying *more* in background mode. The before-and-after command-line routes (`-E help`, `--command extension list`, `-v`) are outside-the-window routes by definition, and each of them exits rather than rendering, so they cannot be used during a render of the same Blender. **Settled by test on 2026-08-06, on Linux only — see below.** All six render handlers fire with no window, in the order init, pre, post, write, complete, with the frame already on disk when `render_write` fires; `render_cancel` does not fire on a finished render. `bpy.app.is_job_running` works with no window and does not crash. So the inferred step above holds. **Added by the tests of 2026-08-07:** which parts of the computer are available can now be learned before a render starts, from outside the window; which part did the work can only be learned once the render has finished, from a closing report.

**Operating system.** Nothing in the sources makes the timing of any of this depend on the operating system. The one timing-adjacent operating-system fact is that on Windows the console is hidden by default when Blender starts with a window (the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html, read 2026-07-30), so anything printed during a render is not seen unless the console is opened. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.1`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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

**`render_stats` fired 43 times for one frame**, so it is not once per frame. Its first and last strings:

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

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.1/blender`, which reported itself as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.1`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**Before a render starts** — which parts of the computer are *available* is now answerable, and it was not on 2026-08-06. `refresh_devices()` and the Cycles preferences name the card with no render and no window; `bpy.ops.wm.sysinfo` describes it in full. What is still not answerable beforehand is which of them will do the work: `compute_device_type` reads `NONE` at factory startup with the card sitting there working, and the command line can override whatever the scene says.

**While it runs** — two signals arrive early. `Mapped host memory limit set to 38,013,009,920 bytes. (35.40G)` comes before the scene is even synchronized, and the memory figure in the progress lines is already the card's by the first sample line (`Mem: 650M | Sample 0/16`). The OptiX name arrives in the progress lines during the geometry step.

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

**Where I looked.** Read `bpy.app.handlers` in full and listed every handler it defines. Read `bpy.types.RenderEngine` in full for its reporting and progress methods. Read `bpy.app` in full. Searched the manual, API documentation and release notes for `before rendering`, `during render`, `after the render`, `on completion`, `render job`. Opened the Command Line Arguments page and the Metadata page again for the timing wording. Read `refresh_devices` and its comment in Cycles' `properties.py`.

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

**While it runs.** `render_stats` hands a string over dozens of times per frame with no window — 43 times for this scene in this version. **With a window open that stream of strings survives only in 5.2**; 4.5, 5.0 and 5.1 hand over one string, the closing time line. So a plan that watches `render_stats` to follow a render's progress works with no window in every version, and with a window only from 5.2.

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

2026-08-06 — Opus 5 (Claude Code) — What actually ran, Blender 5.1

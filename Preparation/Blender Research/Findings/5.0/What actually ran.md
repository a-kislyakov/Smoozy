# What actually ran — Blender 5.0

**Sources consulted, and dates read.**
Blender 5.0 manual (`Blender Sources/manual_5.0`, including its `_images` folder), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and the Cycles add-on's own files (`Blender Sources/cycles_5.0`) — all read 2026-07-30. Blender's requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`) — read 2026-08-01.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 1. When a render has finished, what can be learned about which parts of the computer it used?

**Short answer.** Three separate things exist, and none of them is a plain record of "this render ran on this device".

**(a) What the image itself can carry.** The manual's Metadata panel lists exactly what can be written into render output. The hardware-flavoured entries are:

> "Memory
>
> Includes the peak memory usage."

> "Hostname
>
> Includes the rendering machine's hostname."

> "Render Time
>
> Includes the render time."

Manual, *Metadata*, https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html — read 2026-07-30.

The Python side gives the same list with switch names and defaults, e.g.:

> "use_stamp_hostname
>
> Include the hostname of the machine that rendered the frame
>
> Type:
>
> boolean, default False"

> "use_stamp_memory
>
> Include the peak memory usage in image metadata
>
> Type:
>
> boolean, default True"

API, *RenderSettings*, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

The complete set of stamp switches on `RenderSettings` is: `use_stamp`, `use_stamp_camera`, `use_stamp_date`, `use_stamp_filename`, `use_stamp_frame`, `use_stamp_frame_range`, `use_stamp_hostname`, `use_stamp_labels`, `use_stamp_lens`, `use_stamp_marker`, `use_stamp_memory`, `use_stamp_note`, `use_stamp_render_time`, `use_stamp_scene`, `use_stamp_sequencer_strip`, `use_stamp_time`. There is no switch for a render device, a processor, a graphics card, a backend or a driver.

The manual's own picture of that panel agrees. Image `_images/render_output_properties_metadata_panel.png`, shown on the Metadata page above, displays the checkbox grid: Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, "Use Strip Me…", then Note, Burn Into Image (Font Size, Text Color, Background, Include Labels). No device entry is visible. (In that picture Memory is unticked, while the API says `use_stamp_memory` defaults to True — the picture shows one file's state, not necessarily the factory default.)

One caution about Memory in this exact version. The 5.0.1 corrective-release list contains:

> "* Metadata output for "Peak Memory" displays "ddM" in Blender 5.0. (blender/blender#150524)"

Release notes, *Corrective Releases*, https://developer.blender.org/docs/release_notes/5.0/corrective_releases/ — read 2026-07-30.

**(b) What the log can carry.** The release notes give a worked example of 5.0's log output:

> "00:00.359  gpu.platform     | Using GPU "Apple M3 Max Metal API 1.2"
> 00:00.359  gpu.platform     | Using Backend "Metal"
> 00:01.041  cycles           | Added device "Apple M3 Max (GPU - 40 cores)" with id "METAL_Apple M3 Max (GPU - 40 cores)"."

and, further down the same block:

> "00:26.799  render           | Mem: 2983M | Sample 1/16 
> 00:26.799  render           | Mem: 2983M | Sample 16/16
> 00:28.631  render           | Saved: '/tmp/0001.png'
> 00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)"

Release notes, *Core*, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30. The same page says `--log-level info` "enables all info level log messages, with a moderate amount of output. This may be useful to enable in production for troubleshooting issues."

What the words themselves say is that a GPU and a backend are in use under the `gpu.platform` category, and which device Cycles *added*. Reading `gpu.platform` as the display side of Blender rather than as the render device is **inferred** — the step is that the category name is taken to mean the window system, which nothing quoted says. No line in this example, after the render block begins, names a device.

*Changed by the citation check, 2026-08-07.* This paragraph used to say that the GPU and Cycles-device lines "carry timestamps under two seconds, ahead of any render line in the same example". They do not. The device lines are at `00:00.359`, `00:00.359` and `00:01.041`; the render block begins at `00:00.244`, which is earlier than the last of them. The two halves are separated in the release notes by that page's own elision (`...`), which says they are not one continuous run. The quoted block carries no ordering between them, so the claim has been removed rather than corrected.

**(c) What Cycles can be asked to print.** Cycles' own argument parser:

> `parser.add_argument("--cycles-print-stats",`
> `                    help="Print rendering statistics to stderr",`
> `                    action='store_true')`

Cycles file `cycles_5.0/engine.py`, in `_configure_argument_parser()`, lines 13–15 — read 2026-07-30. **observed.** The manual describes the same flag as "Log statistics about render memory and time usage." (Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.)

**(d) What the settings say, as opposed to what ran.** After the fact you can read back the settings the render was asked to use: the Cycles scene device —

> `enum_devices = (`
> `    ('CPU', "CPU", "Use CPU for rendering"),`
> `    ('GPU', "GPU Compute",`
> `        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),`
> `)`

Cycles file `cycles_5.0/properties.py`, lines 30–34 — read 2026-07-30. **observed.**

— the per-device preference entries —

> `    id: StringProperty(name="ID", description="Unique identifier of the device")`
> `    name: StringProperty(name="Name", description="Name of the device")`
> `    use: BoolProperty(name="Use", description="Use device for rendering", default=True)`
> `    type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')`

Cycles file `cycles_5.0/properties.py`, class `CyclesDeviceSettings`, lines 1563–1566 — read 2026-07-30. **observed.** The device types are `CPU`, `CUDA`, `OPTIX`, `HIP`, `METAL`, `ONEAPI` (`enum_device_type`, same file, lines 172–179).

— and the CPU thread count:

> "threads
>
> Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems)"

API, *RenderSettings*, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

**(e) A machine-wide report, not a per-render one.** "Save System Info" writes a file whose Cycles section holds "The instruction sets and capabilities of each hardware render device available for use with Cycles." Manual, *Help System*, https://docs.blender.org/manual/en/5.0/getting_started/help.html — read 2026-07-30. That is what is *available*, not what was *used*, and it is written when you ask for it, not by a render.

**LABEL:** documented for (a), (b) and (e), and for the manual and API quotations in (c) and (d); observed for the Cycles file quotations in (c) and (d); inferred where marked in (b). For the specific claim "a finished render records which device did the work" — **not in the sources**.

**Where I looked.** Searched the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` for: `stamp`, `metadata`, `hostname`, `peak memory`, `out of memory`, `render time`, `statistics`, `render stats`, `device used`, `render device`, `which device`, `Using device`, `Rendering on`, `exif`, `provenance`, `sidecar`, `manifest`, `render log`. Read in full: the Metadata manual page and its picture, the Output manual page, the Command Line Arguments page, the Cycles GPU Rendering page, the Cycles Render Settings and Performance pages, the Preferences System page, the Help System page, `bpy.types.RenderSettings`, `bpy.types.RenderResult`, `bpy.app`, `bpy.app.handlers`, `cycles_5.0/engine.py`, and the device sections of `cycles_5.0/properties.py`. Ruled out: `bpy.types.Image` and `bpy.types.RenderLayer` (no metadata property of any kind — searched both for `metadata`), and the image-format table (it lists which formats carry metadata at all, not what the metadata contains).

**Not covered.** I did not establish what a *finished* render prints or stores about the device actually used when the render ran, as distinct from the device it was configured to use. The log example in the release notes is one illustrative excerpt with an elision (`...`) in the middle; it is not a specification of every line 5.0 prints, so lines naming the render device may exist and simply not appear there. I also did not establish which log categories exist. No source quoted anywhere in this document names a flag that lists them, so the flag written here as `--log-list-categories` carries no citation and nothing downstream should lean on it until one is found.

**Outside the interface.** Everything above works when Blender is driven from outside its own window, and some of it only exists there. `--cycles-print-stats` and `--log-level` are command-line flags. The release-notes example is background render output. `bpy.app.handlers.render_stats` explicitly names background mode: its argument is "the render stats (render/saving time plus in background mode frame/used [peak] memory)" (API, *Application Handlers*, https://docs.blender.org/api/5.0/bpy.app.handlers.html — read 2026-07-30) — **inferred**, the step being that a description that adds fields "in background mode" means the stats string differs between the two, though the sources do not say what the non-background string contains. The image metadata itself does not depend on how Blender was started; the sources do not say otherwise. "Save System Info" is described only as a menu item under Help. **Settled by test on 2026-08-06, on Linux only — see below.** `bpy.ops.wm.sysinfo` does work with no window, returning `{'FINISHED'}` and writing its file, so the inferred step above holds; but its GPU section then reads `GPU: missing, background mode`, so the route works and the hardware part of what it produces does not. The `gpu.platform` calls do **not** work with no window — they raise `SystemError` in every version. And the line naming the processor that did the work is reachable with no window, on standard output. **Added by the tests of 2026-08-07, with the machine's graphics card repaired:** driving Blender from outside its window is no obstacle to naming the card. With no window and no display server, a run prints the card's model in its `Path tracing on:` line, `bpy.ops.wm.sysinfo` describes the card in full in its Cycles section, and the Cycles preferences list it before any render. The GPU section of `system-info.txt` still reads `GPU: missing, background mode`, so that one section is genuinely lost outside the window and nothing else is. **Added by the re-run of 2026-08-07, after the citation check:** nothing in these three tests needs a window, and all three were run on a machine that has no display server at all.

**Operating system.** The metadata fields are described without any operating-system condition. The Cycles device *types* differ by system: the manual states CUDA, OptiX, HIP and oneAPI are "supported on Windows and Linux", and "Metal is supported on Apple computers with Apple Silicon" (Manual, *GPU Rendering*, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30). So what a device name can say differs by system, but that a device name exists does not. The requirements page names this version once: "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." (https://www.blender.org/download/requirements/ — read 2026-08-01.) **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not. **Added by the re-run of 2026-08-07, after the citation check:** the three tests sent back by the citation check were run on that same Linux machine and nowhere else, so they are an NVIDIA-on-Linux answer; macOS and Windows are untouched by them.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.0`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**One thing about this machine bears on every device result here.** Its graphics card could not be reached while the tests ran — the card's kernel module (580.95.05) and the libraries that talk to it (580.173.02) no longer match. `nvidia-smi` answered `Failed to initialize NVML: Driver/library version mismatch`, and Blender's own view was `cycles | ERROR CUDA cuInit: Unknown error` with an empty device list. So every result below is from a machine that has a graphics card and cannot use it.

**Blender 5.0 prints which processor did the work, by name, and `--log-level info` on its own is enough to bring it out.** The scene was the factory cube at 160 by 120, 16 samples, Cycles. Run as `blender -b scene.blend --log "*" --log-level info -o out/li_ -F PNG -f 1`, standard output carried:

```
00:00.527  cycles           | Full path tracing report:
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

*Note added by the citation check, 2026-08-07.* The block above was run again on the same machine, with the same build (`Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`), the same scene and the same switch. The repeat produced the block **without** the `Hardware Ray-Tracing: Off` line:

```
00:00.309  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: AMD EPYC 7B12 64-Core Processor (CPU) (24 threads) [CPU]
                            | 
                            | Render Scheduler Summary
                            | 
                            | Mode: Headless
                            | Resolution: 160x120
```

Blender 5.1 and 5.2 on the same machine do print that line, and their documents' blocks match their runs exactly. The two 5.0 runs are recorded side by side and neither has been chosen; the question is sent back to the hand tests. Everything the paragraph claims — the model, the thread count, the kind of device, and the stream — reproduced.

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
   RenderTime = 00:00.06
   Memory = 1M
   Hostname = ubuntu
   cycles.ViewLayer.samples = 16
   cycles.ViewLayer.total_time = 00:00.06
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


---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.0`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**The `Path tracing on:` line names the graphics card, its model and the route taken to it.** The same scene as before — the factory cube at 160 by 120, 16 samples, Cycles — with the Cycles preference set to OptiX, the card's OptiX device switched on and the scene set to GPU Compute. Run as `blender -b scene.blend --python setcard.py --log-level info -o out/l_ -F PNG -f 1`, **standard output** carried:

```
Path tracing on: NVIDIA GeForce RTX 4080 SUPER (OptiX) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07_OptiX]
```

and with the preference set to CUDA instead, the same line read:

```
Path tracing on: NVIDIA GeForce RTX 4080 SUPER (CUDA) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07]
```

So the line names the card's model, which of the two NVIDIA routes was taken, and an identifier holding the card's place on the bus. It is the same line, in the same place, brought out by the same switch as when it named the processor on 2026-08-06 — `--log-level info` on its own is still enough.

**Where in the run it arrives.** The line is part of a closing report: it is printed after the last sample and before the frame is written. In the 5.2 run the last sample line was at `00:02.270` and the report at `00:02.270`, with `Saved:` at `00:02.577`. So it answers *what did the work* once the work is done, not before it starts.

**A background render with no switches at all still names no card — but OptiX leaves its name behind.** Run with no log switches: exit 0, a frame written, error stream 0 bytes, and no line naming the card. What OptiX does put into the ordinary progress lines is its own name:

```
00:02.012  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building OptiX acceleration structure
```

Lines of that shape appear more than once in the OptiX run and **not once** in the CUDA run of the same scene. So at default settings the printed output can tell you OptiX was in use and cannot tell you CUDA was.

*Changed by the citation check, 2026-08-07.* This used to say "three times". A repeat on the same machine gave four. Across the four versions the counts I read were four, four, three and two against the four, three, three and three recorded here, so the number is not stable and has been taken out. The contrast the claim rests on — present under OptiX, never under CUDA — held in every run either of us made.

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
| the card, OptiX | 2.08 s | `Time: 00:01.51 (Saving: 00:00.15)` |
| the processor | 24.27 s | `Time: 00:23.40 (Saving: 00:00.17)` |

**This is one machine's timing, on one scene, on one day**, and it is offered as nothing more. What it does show is that the two are nowhere near each other: the card finished this frame about fifteen times sooner, which is far outside anything the spread between repeats could explain. The card's figure includes loading the render kernels, so it is if anything the slower of the two readings.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

**The machine.** The Creator's own Mac — a MacBook Pro with an **M1 Pro** chip, **macOS 26.5.2** (build 25F84), Apple's own graphics and no second card. Blender was `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)` — the same build hash as the Linux machine's 5.0, built at a different hour. Every run was pointed at a settings folder made before it was used (`BLENDER_USER_RESOURCES=/tmp/smoozy-war/settings/5.0`, and `/tmp/smoozy-war/settings2/5.0` where a run needed preferences that had been saved once) and never at the real one. Working files sat in `/tmp/smoozy-war/`; the prepared scene — the factory cube at 160 by 120, 16 samples, Cycles, denoising off — sat at `/Users/artist/Documents/Smoozy hand tests/What actually ran/scene.blend` and was built by Blender 4.5 so all four versions could open it. **Two other topics were running Blender on this Mac while these tests ran**, so nothing below is offered as a timing or a memory measurement. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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

**The Metadata panel, seen on screen.** Blender started with a window on the factory scene, the Output Properties tab shown and its own search box set to `Metadata` so the panel opened, then Blender's own `screen.screenshot` operator. The panel showed `Metadata Input: Scene` and thirteen boxes: **Date, Time, Render Time, Frame, Camera, Scene and Filename ticked; Frame Range, Memory, Hostname, Lens, Marker and Strip Name unticked**, with `Note` and `Burn Into Image` as closed sub-panels, both unticked. **The Memory box is not ticked**, which matches what a running Blender reports and not the API page's stated default of True. Identical in all four versions. The picture is at `/tmp/smoozy-war/out/e27/panel_5.0.png`.

**The build reports itself as `BUILD_PLATFORM b'Darwin'`**, `BACKGROUND True`, `cycles: True`, `cycles_osl: True`, and every conditional image format compiled in. Against the Linux build of the same version, the differences are `coreaudio` True here and False there, `jack` and `pulseaudio` False here and True there.

**Threads read `THREADS 8 THREADS_MODE AUTO`** at factory startup — this machine's core count, not the documented default of 1.

---

### Tested by hand again after the citation check, 2026-08-07 — **LABEL: observed**

**Why these runs happened.** The citation check of this topic found four disagreements, recorded both sides of each, chose between none of them, and sent them back to the hand tests. Three of the four are tests. They were run on 2026-08-07 on the rented Linux machine, alone and last, because that machine was about to be taken away and nothing else could be asked of it afterwards. They are entries 42, 43 and 44 of `Audits/Blender Research/What actually ran/Hand tests.md`.

**The machine, named in full.** The rented Linux machine: Ubuntu 22.04.5 LTS, kernel `6.8.0-59-generic`, one NVIDIA GeForce RTX 4080 SUPER with 16376 MiB of graphics memory, driver **580.173.02** in both halves — `nvidia-smi` and `/proc/driver/nvidia/version` were read at the start of the sitting and agree — an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all. Nothing else was running on it: the load average was 0.19 and there was no Blender process. Everything was written inside `/root/again-what-actually-ran/`, and each of the four Blenders was given a settings folder of its own, made before it was used. The scene is the one-cube factory scene at 160 by 120, sixteen samples, denoising off, built by Blender 4.5 so that all four versions can open it. No test below reads what is written inside a finished image, so no picture and no file was compared.

**Two of the three tests answer this question. They are set out one after the other below.**

---

#### Test one — what the `Full path tracing report` block holds in this version, transcribed exactly

*What a negative result would look like, said before the run.* A `Hardware Ray-Tracing:` line inside this version's block — which is exactly what the 2026-08-06 transcript above records. Blender 5.1 and 5.2 were run identically alongside as controls, precisely so that this test can come out the other way: if those two show the line and this one does not, the absence is a fact about the version and not a fault in the capture.

*What was done.* `/opt/blender/5.0/blender -b scene.blend --log-level info -o out/ -F PNG -f 1`, five times, both streams captured to separate files, on `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)` — the same build named in the 2026-08-06 record above. The same run was then made three more times with the graphics card doing the work.

*What came back, word for word.* Exit 0 and a 0-byte error stream every time. The block, from the first of the five processor runs, uncut:

```
00:00.309  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: AMD EPYC 7B12 64-Core Processor (CPU) (24 threads) [CPU]
                            | 
                            | Render Scheduler Summary
                            | 
                            | Mode: Headless
                            | Resolution: 160x120
```

**There is no `Hardware Ray-Tracing:` line.** The five runs are identical to one another once their timestamps are set aside — the same eight lines, byte for byte. Searching the whole of the captured output of all five runs, both streams, for `Hardware Ray-Tracing` and for `ray-trac` in any case gives nought matches. The three runs with the card doing the work carry the card's name in that line — `Path tracing on: NVIDIA GeForce RTX 4080 SUPER (OptiX) [CUDA_NVIDIA GeForce RTX 4080 SUPER_0000:00:07_OptiX]` — and still no `Hardware Ray-Tracing:` line.

*The controls did their job.* Blender 5.1 and 5.2, run identically on the same machine on the same day, **do** carry `Hardware Ray-Tracing: Off` in every one of their eight runs each, on the processor and on the card alike. So the test could have come out the other way, and did not.

*Two runs of the same test on the same machine came out differently, and neither has been chosen.* The 2026-08-06 transcript above stands exactly as it was written, and it carries a `Hardware Ray-Tracing: Off` line. The citation check's repeat of 2026-08-07 did not, and these five further runs of 2026-08-07 did not either. That is one run against ten, on the same machine, with the same build, the same scene and the same switch. **This finding does not choose between them.** What it can say is what the ten runs have in common with the one: the block exists, it is on standard output, it is brought out by `--log-level info`, and its `Path tracing on:` line names the processor's model, its thread count and the kind of device. Anything downstream that needs the `Hardware Ray-Tracing:` line must treat its presence in 5.0 as unsettled, and must not read its absence from a 5.0 run as an error.

---

#### Test two — how many `Building OptiX acceleration structure` lines a run prints

*What a negative result would look like, said before the run.* The same number in all five runs of a version — which would have meant the two earlier counts were a miscount rather than a moving target, and the finding could then name the number with five repeats behind it. The CUDA runs beside them are the second way this test can fail: they show what the same count looks like when the answer is nought, so the counting is not something that only ever returns small positive numbers.

*What was done.* The shared scene rendered on the card with OptiX and **no log switches at all**, five times per version, counting the lines that carry `Building OptiX acceleration structure` on each stream. The card was pointed at by a script that sets the Cycles preference to OptiX and the scene to GPU, and prints the device list back so the run says rather than assumes what it used — it read `[('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', False), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False), ('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True)]` in every OptiX run. Five CUDA runs per version were made the same way, as the control.

*What came back.* The count is **not stable**. Five OptiX runs of the identical command in this version gave, in order:

```
2, 4, 4, 2, 4
```

and the five CUDA runs beside them gave `0, 0, 0, 0, 0`. Exit 0 and a frame written in all ten; the error stream was 0 bytes in all ten. **Both of the numbers the two earlier runs disagreed about — three and four — sit inside that spread**, and so does a third number neither of them reported.

*Why it moves, read straight off the lines.* The lines are ordinary progress lines, and which of the geometry steps gets a line of its own varies from run to run. Two runs of this version, side by side, with the timestamps and memory figures taken out:

```
run 1
  render | Fra: 1 | Updating Geometry BVH Cube 2/1 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Scene BVH | Building OptiX acceleration structure

run 2
  render | Fra: 1 | Updating Geometry BVH light 4/3 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Geometry BVH Light 3/3 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Geometry BVH Cube 2/3 | Building OptiX acceleration structure
  render | Fra: 1 | Updating Scene BVH | Building OptiX acceleration structure
```

The `Updating Scene BVH` line is in every run of every version. The geometry lines above it are not. So the number is a fact about which progress steps happened to be reported, and never a fact about Blender or about OptiX.

*What this settles, and what must not be written.* Across the four versions the counts were 4.5 `3, 4, 4, 3, 4`; 5.0 `2, 4, 4, 2, 4`; 5.1 `3, 4, 3, 3, 4`; 5.2 `2, 4, 2, 3, 3`; and CUDA nought, twenty runs out of twenty. **The number must never be quoted as a fact about a version.** What holds, and held in every run anybody has made, is the half the claim rests on: lines of this shape appear in an OptiX run and never once in a CUDA run of the same scene. The right form of words is *more than once*, which is what the documents now say.


## 2. When a render has finished, what can be learned about which render engine produced it?

**(a) The log names it.** The release-notes example of 5.0 log output contains, in the render block:

> "00:00.244  render           | Rendering single frame (frame 1)
> 00:00.245  render           | Start rendering: Scene, ViewLayer
> 00:00.245  render           | Engine: Cycles"

Release notes, *Core*, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30. **documented.**

**(b) No stamp field names it.** The stamp list quoted in answer 1 has no engine field. The nearest thing is a hook for the engine to add one itself:

> "stamp_data_add_field(field, value)
>
> Add engine-specific stamp data to the result"

API, *RenderResult*, https://docs.blender.org/api/5.0/bpy.types.RenderResult.html — read 2026-07-30. The sources do not say whether Cycles, EEVEE or Workbench call it, or with what.

**(c) The setting can be read back.** The scene keeps the engine:

> "engine
>
> Engine to use for rendering
>
> Type:
>
> enum in ['BLENDER_EEVEE'], default 'BLENDER_EEVEE'"

API, *RenderSettings*, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30. The list showing only `BLENDER_EEVEE` is a property of how the documentation was generated, not a statement that only one engine exists — Cycles is an add-on and registers its own identifier, and the manual's own example uses `-E CYCLES` and a doc example elsewhere reads `return context.scene.render.engine == 'CYCLES'` (API, *NodeTree*, https://docs.blender.org/api/5.0/bpy.types.NodeTree.html — read 2026-07-30). That reading is **inferred**; the step is that a documented usable value (`CYCLES`) is absent from the documented enum list, so the list must be incomplete rather than exhaustive. **Do not read this paragraph on its own: the 2026-08-06 test below found a running Blender reporting the same one-entry list, so the list is not an artefact of how the documentation was generated and this inference does not hold as stated.** *(Pointer added by the citation check, 2026-08-07, because the inference and the test that refutes it sat eight paragraphs apart with nothing joining them.)*

**(d) It can be set and enumerated from outside.**

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

**LABEL:** documented for (a), (b) and (d); inferred for the enum reading in (c), with the step written out above.

**Where I looked.** Searched all four source sets for `Engine:`, `engine`, `bl_idname`, `BLENDER_EEVEE`, `BLENDER_WORKBENCH`, `CYCLES`, `stamp`, `metadata`, `Software`. Read `bpy.types.RenderSettings`, `bpy.types.RenderResult`, `bpy.types.RenderEngine`, the Command Line Arguments and Rendering From The Command Line manual pages, the Metadata manual page and its picture, and `release_notes_5.0/core.md`. Ruled out the image-format page (`manual_5.0/files/media/image_formats.txt`) — it says which formats support metadata but never lists an engine or software tag; searched it for `Software` and found only unrelated prose.

**Not covered.** Whether any engine shipped with Blender actually writes an engine-specific stamp field, and what an image file written by 5.0 contains beyond the listed stamp fields (for example an OpenEXR or PNG software tag). The sources say nothing either way.

**Outside the interface.** The `Engine: Cycles` line is from a log example of a background render, so this holds outside the interface — indeed the log is the main place it appears. `-E` and `-E help` exist only outside the interface. The scene setting is readable both ways. **Settled by test on 2026-08-06, on Linux only — see below.** The engine's name reaches a finished image with no window involved, through the four `cycles.ViewLayer.*` stamp fields and, from 5.0 on, an OpenEXR `Software` attribute. `-E help` needs `-b` on a machine with no screen; in Blender 4.5 it never returns even then. **Added by the tests of 2026-08-07:** nothing about the engine changes when a graphics card does the work. Every route above behaves the same outside the window with the card in use as with the processor.

**Operating system.** Nothing in any of the quoted material is conditioned on an operating system. Which engines are *available* depends on what is installed and compiled in (`bpy.app.build_options` lists `cycles=True` in the documented example build), not on the operating system as such — but note the whole engine list is what `-E help` reports on that machine, and the sources do not say it is the same everywhere. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.0`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**A finished image does carry a trace of the engine, in two ways, and one of them is new since 4.5.**

*Cycles writes four stamp fields of its own.* The same frame was rendered twice with every metadata switch on, once with `-E CYCLES` and once with `-E BLENDER_EEVEE`. The Cycles file carried four fields the EEVEE file did not:

```
   cycles.ViewLayer.samples = 16
   cycles.ViewLayer.total_time = 00:00.06
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
   RenderTime = 00:25.81
   Memory = 0M
   Hostname = ubuntu
```

So the hook the API describes for engine-specific stamp data is in use, and the field names begin with the engine's own name.

*An OpenEXR also carries a `Software` attribute.* The Cycles render to OpenEXR carried, among its attributes:

```
   Software (string) = Blender 5.0.1
   colorInteropID (string) = lin_rec709_scene
```

Neither appears in a PNG, and **neither appears in the equivalent file written by Blender 4.5** — that comparison was run on the same machine on the same day and is recorded in 4.5's own document. This closes the *Not covered* line above about a producer or software tag.

**The log names the engine at default settings.** The plain background render quoted under question 1 carries `render | Engine: Cycles` with no switch at all.

**`blender -b -E help` lists three engines.** Exit 0:

```
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
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


---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.0`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**Which part of the computer did the work changes nothing about what names the engine.** The metadata test was run again with the scene on GPU Compute and OptiX really doing the work: a Cycles render with every `use_stamp_*` switched on, to PNG and to OpenEXR. **The field set is identical to the processor's** — the same four `cycles.ViewLayer.*` fields, and the same `Software` attribute in the OpenEXR naming Blender and its version. Nothing new names the engine, and no field names the card.

The one thing a card adds sits in the printed output, not in the file. `Building OptiX acceleration structure` (question 1) names a *technology*, not an engine — somebody reading only that line would know a graphics card was in use and still not know which engine had asked for it. Cycles is the only engine that can ask, on the evidence of `-E help`, but that is a step this run does not take.

`-E help`, the `render | Engine: Cycles` line and the engine list were settled on 2026-08-06 and are untouched by the card; they were not run again, so that a second result could not be laid over a first.

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
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
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
   Software (string) = Blender 5.0.1
   colorInteropID (string) = lin_rec709_scene
```

Blender 4.5 on the same machine wrote neither attribute. The PNGs carry no `Software` field in any version. Same as the Linux runs.

---

## 3. When a render has finished, what can be learned about which add-ons were in use while it ran?

**LABEL: not in the sources**, for anything a render itself records. Nothing in the render output, the render log example, the stamp fields, or any render-related API names the add-ons that were loaded.

What the sources *do* offer is three ways to find out separately from the render.

**(a) A saved report.** "Save System Info" writes `system-info.txt`, and one of its sections is:

> "Enabled Add-Ons
>
> Lists add-ons currently in use along with their versions and paths."

Manual, *Help System*, https://docs.blender.org/manual/en/5.0/getting_started/help.html — read 2026-07-30. The same page says "You will be prompted to save a text file called system-info.txt." The matching operator is:

> "bpy.ops.wm.sysinfo(*, filepath='')
>
> Generate system information, saved into a text file"

API, *bpy.ops.wm*, https://docs.blender.org/api/5.0/bpy.ops.wm.html — read 2026-07-30.

**(b) Asking Blender directly.** The preferences hold the collection:

> "addons
>
> Type:
>
> Addons bpy_prop_collection of Addon, (readonly)"

API, *Preferences*, https://docs.blender.org/api/5.0/bpy.types.Preferences.html — read 2026-07-30. Each entry is:

> "class bpy.types.Addon(bpy_struct)
>
> Python add-ons to be loaded automatically
>
> module
>
> Module name"

API, *Addon*, https://docs.blender.org/api/5.0/bpy.types.Addon.html — read 2026-07-30. Cycles itself reaches into this collection by its own package name, e.g. `context.preferences.addons[__package__].preferences` (Cycles file `cycles_5.0/ui.py`, function `get_device_type`, lines 96–97 — read 2026-07-30, **observed**).

**(c) Controlling the set from outside, so you know it by construction.**

> "--addons <addon(s)>
>
> Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons."

Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. Alongside it, `--factory-startup` "Skip reading the startup.blend in the users home directory."

**LABEL:** **not in the sources** for anything a render itself records, as stated at the top of this answer. For the three routes beside the render: documented for (a) and (c), and for the API quotations in (b); observed for the Cycles file quotation in (b).

**Where I looked.** Searched `manual_5.0`, `api_5.0`, `release_notes_5.0`, `cycles_5.0` for: `add-on`, `addon`, `addons`, `addon_utils`, `extension`, `Enabled Add-Ons`, `enabled add-on`, `loaded add-on`, `stamp`, `metadata`. Read the Preferences Add-ons manual page, the Help System page, the Extensions section listing (`manual_5.0/advanced/extensions/`), `bpy.types.Addon`, `bpy.types.Addons`, `bpy.types.Preferences`, `bpy.types.PreferencesView`, and the Metadata manual page and picture. `addon_utils` appears nowhere in the API documentation — it is mentioned only in two manual pages (`advanced/scripting/addon_tutorial.txt`, `advanced/deploying_blender.txt`), neither about renders. Ruled out the render-side API: `bpy.types.RenderResult`, `bpy.types.RenderEngine`, `bpy.app.handlers` — searched all three for `addon` and found nothing.

**Not covered.** Whether a blend-file, a render log at any verbosity, or an output image records the add-on set. I could not list the log categories, so I cannot say whether an add-on or extension category exists that would print them — and the flag named for that in answer 1 carries no citation.

**Outside the interface.** `--addons` and `--factory-startup` exist only outside the interface, and are the sharpest tool here: they fix the add-on set for the run rather than reporting it. `bpy.ops.wm.sysinfo` takes a `filepath` argument, so **inferred** it can be called from a script with no window — the step being that a documented file-path parameter on an operator implies it need not go through a file-chooser; the sources do not confirm this. Reading `preferences.addons` needs no window. **Settled by test on 2026-08-06, on Linux only — see below.** `bpy.ops.wm.sysinfo` does work with no window in every version, and its Enabled add-ons section is filled in with versions and paths, so the inferred step above holds. **Added by the tests of 2026-08-07:** the add-on section of `system-info.txt` written from outside the window is unchanged by the card working, and no field a render writes names an add-on whichever device ran it.

**Operating system.** Nothing quoted is conditioned on an operating system. Where `system-info.txt` is saved depends on where you point it; the manual gives no per-system path. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.0`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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


---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.0`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

**The card changes nothing here, and the negative result is now stronger for it.** Every field written into a PNG and into an OpenEXR by a render that really ran on the graphics card was read back (see question 1's run). **No field names an add-on**, exactly as when the processor did the work.

The `Enabled add-ons:` section of `system-info.txt` is unchanged too. The file grew from 94 lines to 183 with the card repaired, and **every added line is in the Cycles section**, describing the card's capabilities. The add-on list, its versions and its paths are identical.

So the 2026-08-06 answer stands, with the case that was missing from it filled in: a finished render leaves no trace of which add-ons were loaded, whichever part of the computer did the work.

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**`bpy.ops.wm.sysinfo` fills its add-on section in with no window.** The same eight modules the Linux machine ships, at macOS paths under `/Applications/Blender 5.0.1.app/Contents/Resources/5.0/scripts/addons_core/` — `io_anim_bvh`, `io_curve_svg`, `io_mesh_uv_layout`, `io_scene_fbx`, `io_scene_gltf2`, `cycles`, `pose_library` and `bl_pkg`, each with its version in brackets and its full path. The file written with a window open carries the identical section.

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

The sources cover several distinct cases and answer them very unevenly.

**(a) Not enough graphics memory — it keeps going on system memory.**

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
> try to use system memory. This has a performance impact, but will usually still result in a faster render
> than using CPU rendering."

Manual, *GPU Rendering*, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30. **documented.** Note oneAPI is absent from that list of four.

**(b) Out of memory outright — an error message.** The same page's Error Messages section:

> "Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU."

and

> "CUDA Error: Kernel compilation failed
>
> This error may happen if you have a new NVIDIA graphics card that is not yet supported by
> the Blender version and CUDA toolkit you have installed.
> In this case Blender may try to dynamically build a kernel for your graphics card and fail."

Same page and date. **documented.** A matching report type exists in the API: `ERROR_OUT_OF_MEMORY: Out of Memory.` (API, *Wm Report Items*, https://docs.blender.org/api/5.0/bpy_types_enum_items/wm_report_items.html — read 2026-07-30.)

**(c) How an engine reports at all.** Two routes:

> "report(type, message)
>
> Report info, warning or error messages"

> "error_set(message)
>
> Set error message displaying after the render is finished"

API, *RenderEngine*, https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — read 2026-07-30. **documented.** Where those go: the Info editor "logs the executed operators as well as errors, warnings, and informational messages" (Manual, *Info Editor*, https://docs.blender.org/manual/en/5.0/editors/info_editor.html — read 2026-07-30), and 5.0 routed reports into the log system: "Most of Blender now uses the same logging system, including operators reports and background render progress." (Release notes, *Core*, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.)

**(d) No usable graphics device configured at all.** Cycles' preferences panel puts up a plain label:

> `            col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='INFO', translate=False)`

Cycles file `cycles_5.0/properties.py`, in `CyclesPreferences._draw_devices`, line 1810 — read 2026-07-30. **observed.** Followed by requirement lines per backend, e.g. `"Requires NVIDIA GPU with compute capability %s"` with `compute_capability = "5.0"` (lines 1813–1815), `"Requires Apple Silicon with macOS %s or newer"` with `mac_version = "12.2"` (lines 1868–1870). This is a panel label; it is drawn in the preferences window, not printed by a render.

The manual states the plain fallback:

> "None:
>
> When set to None or when the only option is None:
> the CPU will be used as the computing device for Cycles."

Manual, *Preferences — System*, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30. **documented.**

Cycles' own helper carries the same logic one step further:

> `def use_cpu(context):`
> `    cscene = context.scene.cycles`
> ``
> `    return (get_device_type(context) == 'NONE' or cscene.device == 'CPU' or not backend_has_active_gpu(context))`

Cycles file `cycles_5.0/ui.py`, lines 104–108 — read 2026-07-30. **observed** as text. Reading it as "a scene set to GPU Compute with no enabled GPU device renders on the CPU" is **inferred** — the step is that this function only ever decides which panels and options the interface shows, and the sources nowhere say the renderer takes the same branch.

**(e) A feature the chosen device cannot do.** The manual is explicit for several:

> "Path Guiding is not supported on any GPU.
>
> Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

Manual, *GPU Rendering*, same address and date. And the Render Time debug pass:

> "Render Time Cycles
>
> An estimate for how long in milliseconds each pixel took to render.
> Note, this pass is not supported on GPU rendering."

Manual, *Passes*, https://docs.blender.org/manual/en/5.0/render/layers/passes.html — read 2026-07-30. Cycles' own code shows what it does about it — it simply does not offer the pass:

> `    if crl.pass_render_time:`
> `        # Only yield the pass if rendering on CPU`
> `        if scene.cycles.device == 'CPU':`
> `            yield ("Render Time", "X", "VALUE")`

Cycles file `cycles_5.0/engine.py`, in `list_render_passes()`, lines 222–225 — read 2026-07-30. **observed.** So for this one case the answer to "what does it report" is: nothing — the pass is silently absent from the result. That the absence is silent is **inferred**; the step is that the code has no `report` or warning on that branch, and I did not search Cycles' C++ sources.

**(f) EEVEE, where the shortfall is structural.**

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

and

> "Multiple GPU Support
>
> There is currently no support for
> multiple GPU systems."

Manual, *EEVEE Limitations*, https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html — read 2026-07-30. **documented.** The manual does not say what is reported when you try.

**(g) A file format the build does not have.** The command-line format list distinguishes the two:

> "-F, --render-format <format>
>
> Set the render format.
> Valid options are:
> TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF.
>
> Formats that can be compiled into Blender, not available on all systems:
> OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. **documented.** What the build actually has can be read: `bpy.app.build_options`, whose documented example value includes `image_openexr=True`, `image_webp=True`, `cycles=True`, `cycles_osl=True` (API, *bpy.app*, https://docs.blender.org/api/5.0/bpy.app.html — read 2026-07-30). The sources do not say what happens if you pass a format the build lacks.

**(h) A bad device name on the command line — it stops.**

> `    if args.cycles_device:`
> `        import _cycles`
> `        if not _cycles.set_device_override(args.cycles_device):`
> `            sys.exit(1)`

Cycles file `cycles_5.0/engine.py`, in `_parse_command_line()`, lines 38–41 — read 2026-07-30. **observed.** Reading this as "an unusable `--cycles-device` value ends the run with exit status 1" is **inferred** — the step is that `sys.exit(1)` is a Python call whose effect I am asserting from general behaviour, not from anything the sources state, and I cannot see what `set_device_override` returns for which inputs.

**(i) When it does not survive at all.** "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory." — with per-system names: `test.crash.txt` and `blender.crash.txt` on Windows, macOS Crash Reporter plus `~/Library/Logs/DiagnosticReports/`, and `blender.crash.txt` in `/tmp` on Linux. Manual, *Crashes*, https://docs.blender.org/manual/en/5.0/troubleshooting/crash.html — read 2026-07-30. **documented** for the sentence in quotation marks. The per-system file names after it are given in summary rather than in the page's own words, so they are not quoted here.

**LABEL:** documented for (a), (b), (c), (e), (f), (g), (i); observed for the quoted Cycles lines in (d), (e) and (h); inferred where I said so, with each step written out.

**Where I looked.** Searched all four source sets for: `out of memory`, `not enough memory`, `insufficient memory`, `fall back`, `fallback`, `falls back`, `not supported`, `unsupported`, `is not available`, `no compatible`, `error`, `Error`, `warn`, `missing`, `crash`, `headless`, `background mode`. Read in full: the Cycles GPU Rendering page (including its whole Error Messages section), the EEVEE Limitations page, the Crashes page, the common Graphics Hardware troubleshooting page, the Command Line Arguments page, `bpy.types.RenderEngine`, `bpy_types_enum_items/wm_report_items`, the Info Editor page, and `cycles_5.0/engine.py`, `cycles_5.0/ui.py`, `cycles_5.0/operators.py`, `cycles_5.0/osl.py`, plus the device-drawing and preference sections of `cycles_5.0/properties.py`. Ruled out: the Freestyle, compositing and geometry-node areas of the manual — searched for the same error terms and found only unrelated hits.

**Not covered.** This is the weakest area. For most of the shortfalls above, the sources describe *that* something is unsupported and never state what message appears, on which stream, or with what exit status. I found no documented list of render error strings, no documented exit codes for a failed render (the only exit-code setting documented anywhere is `--python-exit-code`, for Python exceptions in scripts run from the command line), and nothing that says whether a frame that fails still writes a file. I also did not open Cycles' C++ sources under `cycles_5.0/source/`, so error text that lives there is outside what I checked.

**Outside the interface.** Mixed, and it matters. (a), (b), (e), (g), (h) and (i) are not tied to the window. (c) is: `error_set` says the message displays "after the render is finished", and `report` messages land in the Info editor — but 5.0 routes operator reports through the log system, so **inferred** they also reach the log when there is no window; the step is that "including operators reports" in the Core release note is a statement about the logging system, not specifically about background mode. (d)'s "No compatible GPUs found for Cycles" is a preferences panel label and cannot appear at all with no window. (f)'s headless limitation only bites outside the interface. **Settled in part by test on 2026-08-06, on Linux only — see below.** Every failure message found reached standard output or the error stream with no window: the `--cycles-device` refusal, the unwritable-output error, and the `-F` format complaint. The preferences-panel messages could not be reached at all with no window, as expected, and no substitute for them appeared anywhere. **Added by the tests of 2026-08-07:** `--cycles-device` exists only outside the window, and with the card working it can now be seen to succeed silently for `OPTIX` and `CUDA` and to refuse `HIP` and `ONEAPI` with `Found no Cycles device of the specified type`. The out-of-memory case is still out of reach, because this machine's own memory gives out before the card's does.

**Operating system.** Several of these are per-system. (a) names CUDA, OptiX, HIP and Metal but not oneAPI. (f) names Windows specifically for the headless limitation. (i) gives three different crash-file behaviours for Windows, macOS and Linux. The Cycles requirement labels in (d) branch on `sys.platform` for HIP and oneAPI (Cycles file `cycles_5.0/properties.py`, lines 1825 and 1850). The requirements page names this version once: "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." (https://www.blender.org/download/requirements/ — read 2026-08-01.) Note that this differs from the Cycles panel's `mac_version = "12.2"` and from the manual's Metal note, "macOS 13.0 or newer is required to support all features." **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.0`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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
00:00.560  render           | Time: 00:00.17 (Saving: 00:00.13)
```

The same file with the device set to CPU gave `exit=0`, a frame, the **identical** `Path tracing on:` line, and `Time: 00:00.22`. So the run is not silent about it. The times are not evidence and are not offered as any: the machine was shared with other work while these ran, the scene takes a fifth of a second, and the two runs differ by less than the spread between repeats.

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


---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.0`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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

**"No compatible GPUs found for Cycles" could not be reached, and the panel shows why.** Blender started with a window, Preferences ▸ System shown and photographed by Blender's own screenshot operator. The Cycles Render Devices panel offers two buttons, `None` and `Metal`, with `Metal` chosen; under them a list holding `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked, then `Kernel Optimization: Full` and `MetalRT: Auto`. Setting the preference to `None` leaves the two buttons and **nothing else at all** — no list, no message. There is no CUDA, OptiX, HIP or oneAPI button on this machine, so there is no way to put the panel into the state that message belongs to. Pictures at `/tmp/smoozy-war/out/e28/prefs_5.0.png` and `/tmp/smoozy-war/out/e28/prefs_none_5.2.png`.

---

## 5. Where can each of the things above be learned from — what a render prints while it works, what it leaves on disk, or what can be asked afterwards?

Taking the three places in turn.

**What a render prints while it works.** The log, and it is the richest channel. 5.0's format is shown in the release-notes example quoted in answers 1 and 2: a timestamp, a category, a pipe, then the message — with `render`, `cycles` and `gpu.platform` among the categories. Release notes, *Core*, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30. What is on it can be widened and narrowed:

> "--log <match>
>
> Enable logging categories, taking a single comma separated argument.
>
> --log "*": everything
> […]
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

> "--log-show-memory
>
> Show memory usage for each log message."

Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. Note `info` is described as "Information about devices, files, configuration, operations" — devices named first.

The stats can also be captured in Blender rather than read off a stream:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

API, *Application Handlers*, https://docs.blender.org/api/5.0/bpy.app.handlers.html — read 2026-07-30. Its siblings are `render_init`, `render_pre`, `render_post`, `render_write` ("on writing a render frame (directly after the frame is written)"), `render_complete` and `render_cancel`, same page and date.

Cycles adds `--cycles-print-stats`, which its own file says goes "to stderr" (`cycles_5.0/engine.py`, line 14 — read 2026-07-30, **observed**), while the manual calls it "Log statistics about render memory and time usage." Those two descriptions differ; the sources do not reconcile them.

**What it leaves on disk.** Four kinds of thing.

1. *Metadata inside the image.* The stamp fields of answer 1, plus Burn into Image which "Add[s] metadata as text to the render" (Manual, *Metadata*, https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html — read 2026-07-30). This channel is narrow: the manual's format table marks Metadata supported only for JPEG, OpenEXR, PNG and WebP, and unsupported for BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw and TIFF (Manual, *Supported Graphics Formats*, https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30). The metadata page carries its own warning: "Only some image formats support metadata".

2. *A log file.* `--log-file <filepath>` — "Set a file to output the log to." Manual, *Command Line Arguments*, same address and date.

3. *Files that mark progress rather than content.* "Create empty placeholder frames while rendering", and the render-farm hint beside it: "Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory." Manual, *Output*, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30. Also Cache Result, which "Saves the rendered view layers and their passes to a multi-layer OpenEXR image" in "the Render Cache folder as specified in the File Paths Preferences" (same page).

4. *Reports about the machine, written on request or on failure.* `system-info.txt` (answer 3), the crash files (answer 4(i)), and `--profile-gpu`, which "Outputs a profile.json file in the Trace Event Format to the current directory" (Manual, *Command Line Arguments*, same address and date).

**What can be asked afterwards.** Inside Blender: the Image editor's Show Metadata overlay "Displays metadata about the selected Render Result" (Manual, *Navigating*, https://docs.blender.org/manual/en/5.0/editors/image/navigating.html, and *Overlays*, https://docs.blender.org/manual/en/5.0/editors/image/overlays.html — read 2026-07-30); the Image editor sidebar's Metadata panel "Lists image metadata" (Manual, *Sidebar*, https://docs.blender.org/manual/en/5.0/editors/image/sidebar.html — read 2026-07-30); and the Video Sequencer's Metadata tab, which "Lists information that has been encoded in the currently visible movie or image file […] This also works for images produced by Blender" (Manual, *Sidebar*, https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/sidebar.html — read 2026-07-30). Through Python: `scene.render.*` including `engine`, `threads` and every `use_stamp_*`; `preferences.addons`; `bpy.app.version`, `bpy.app.build_hash`, `bpy.app.build_platform`, `bpy.app.build_options`, `bpy.app.background`.

The one thing Blender explicitly cannot do afterwards is *write* metadata: "The metadata can't be edited from Blender. Instead, you can use an external program such as exiftool." Same VSE sidebar page and date.

**LABEL:** documented throughout; observed for the two Cycles file quotations.

**Where I looked.** This answer records no searches of its own. It gathers what answers 1 to 4 found and adds the log flags, the output settings and the metadata viewers, all read on the pages cited above: Command Line Arguments, Output, Metadata, Supported Graphics Formats, the Image editor's Navigating, Overlays and Sidebar pages, the Video Sequencer sidebar page, and `bpy.app.handlers`. The two shortfalls named below — that nothing shows the device actually used, and that nothing shows the add-ons — rest on the searches recorded under answers 1 and 3. Nothing at all was searched for the default log level, or for which stream the render lines go to; those two are unsearched rather than absent.

**Not covered.** Which of the three channels carries which fact is only partly settled. In particular I could not establish that the device actually used appears in any of them (see answer 1), nor that add-ons appear in any of them (answer 3). I also did not establish the default log level when no flag is given, nor whether the render lines in the example appear on standard output or standard error.

**Outside the interface.** This split is largely a split about the interface. The log, the log file, `--cycles-print-stats`, `--profile-gpu` and the handlers all work with no window and several exist only there. The three metadata *viewers* — Image editor overlay, Image editor sidebar, VSE sidebar — require the interface; the metadata in the file does not, and can be read by an outside tool such as the exiftool the manual names. The 5.0 release notes warn precisely this audience: "The format of [logging output has changed], including background render progress. Add-ons or render farms parsing this output may need to be updated." (Release notes, *Python API*, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30.) **Settled by test on 2026-08-06, on Linux only — see below.** With no window, everything a render prints goes to standard output and the error stream stays empty, apart from the `-F` complaint and, in 4.5 only, the device and `--cycles-device` messages. `--cycles-print-stats` goes to standard output, not the error stream. There is no route from Blender's Python to a written file's metadata in any version, with or without a window. **Added by the tests of 2026-08-07:** the printed route is the one that names the card, and it works with no window. `--cycles-print-stats` replaces its profiling sections with `Profiling information not available (only works with CPU rendering)` when the card did the work. The on-disk route names no card at all. **Added by the re-run of 2026-08-07, after the citation check:** nothing in these three tests needs a window, and all three were run on a machine that has no display server at all.

**Operating system.** Nothing in this answer is stated as system-dependent, except the crash-file locations already noted in answer 4 and the temporary-directory environment variables the manual gives per system: "TEMP: Store temporary files here (MS-Windows)." and "TMPDIR: Store temporary files here (UNIX Systems)." (Manual, *Command Line Arguments*, same address and date.) **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not. **Added by the re-run of 2026-08-07, after the citation check:** the three tests sent back by the citation check were run on that same Linux machine and nowhere else, so they are an NVIDIA-on-Linux answer; macOS and Windows are untouched by them.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.0`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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

**`--log-list-categories` exists in this version and lists 169 categories.** Exit 0. *(Corrected by the citation check, 2026-08-07: the run prints 170 lines, the first of which is the version banner, so there are 169 categories. Everything else in this paragraph reproduced.)* The list opens `addon`, `anim`, `anim.action`, `anim.data`, `anim.driver`, and the categories bearing on this topic are `cycles`, `render`, `hydra.render`, `compositor.gpu`, `image.gpu` and a family of `gpu.*` entries. There is an `addon` category. The device line lives under `cycles` at `info` level — see question 1. (In Blender 4.5 the switch does not exist at all; that is recorded in 4.5's own document.)

**`--log-level` takes a name in this version.** `--log-level info` was accepted and the render exited 0. (Blender 4.5 takes a number and refuses the name; recorded in 4.5's document.)

**A written render's metadata cannot be read back from Blender's own Python.** Run as `blender -b --python-expr` loading one of the stamped PNGs:

```
LOADED (160, 120)
HAS_metadata_attr False
stamp-ish attrs: []
```

The image loads and reports its size, so Blender read the file; there is no route from Python to the fields inside it. This closes the *not in the sources* recorded above.

**What `system-info.txt` holds when written with no window** is under questions 1 and 3 — the operator works, but the GPU section reads `GPU: missing, background mode`.


---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.0`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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

---

### Tested by hand on the Mac, 2026-08-07 — **LABEL: observed**

The machine and the method are described under question 1.

**Which stream everything lands on.** In this version, every one of these went to **standard output** and left the error stream at 0 bytes: the ordinary progress lines, the `Engine:` line, the `Path tracing on:` device line at `--log-level info`, the whole `--cycles-print-stats` block, the `Found no Cycles device` refusal, and the two-line failure when a render cannot save. The only things that reached the **error** stream were the `-F` complaint and the `engine not found` refusal.

**`--log-list-categories` works, and the list is shorter on macOS with different graphics categories.** Exit 0 and **135 categories** on this machine, against 170 for the same version on Linux. The graphics ones here:

```
compositor.gpu
cycles
gpu.debug.metal
gpu.platform
gpu.shader
gpu.texture
hydra.render
render
```

The Linux list carried `gpu.debug` and `gpu.vulkan` where this one carries `gpu.debug.metal`. **So the category list is a fact about the operating system as well as the version**, and a plan that hard-codes a category name from one machine will not find it on the other. `addon`, `cycles` and `render` are in both.

*Changed by the citation check, 2026-08-07.* `gpu.opengl` was named here as part of the Linux 5.0 list. A repeat of `--log-list-categories` on the Linux machine with this version has no such entry; the whole of its graphics family is `compositor.gpu`, `cycles`, `gpu.debug`, `gpu.platform`, `gpu.shader`, `gpu.texture`, `gpu.vulkan`, `hydra.render`, `hydra.scene`, `image.gpu` and `render`. `gpu.opengl` appears from 5.1 on. The two runs disagree and the question is sent back to the hand tests; the name has been taken out of this sentence meanwhile, and the point the sentence makes is untouched.

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

*What a negative result would look like, said before the run.* `gpu.opengl` present in this version's list — which is what the Mac section of this document originally said the Linux list carried. Blender 5.1 and 5.2 were run identically as controls: if the same search finds the entry in those two lists, then the search works and its silence here means something.

*What was done.* `blender -b --log-list-categories`, three times per version, both streams captured, and the output searched for a line reading exactly `gpu.opengl`.

*What came back.* **This version's list has no `gpu.opengl` entry.** Exit 0, three runs, and all three files are identical to the byte. The list is 170 lines, of which the first is the version banner, so **169 categories**. Every category whose name contains `gpu`, in full:

```
compositor.gpu
gpu.debug
gpu.platform
gpu.shader
gpu.texture
gpu.vulkan
image.gpu
```

A case-insensitive search of the whole list for `opengl` gives nought matches.

*The controls did their job.* The same search on the same machine on the same day finds `gpu.opengl` in Blender 5.1 (174 lines, 173 categories) and in Blender 5.2 (183 lines, 182 categories). Blender 4.5 has no such switch at all. So the entry arrives at 5.1, and the search that found it there is the one that found nothing here.

*Two runs of the same test on the same machine came out differently, and neither has been chosen.* The Mac section further down this document said the Linux list carried `gpu.debug`, `gpu.opengl` and `gpu.vulkan`; the citation check's repeat found no `gpu.opengl`; these three further runs found none either. That is one run against four. **This finding does not choose between them.** The sentence in the Mac section was already changed by the citation check to drop the name, and it stays as it was left; what these runs add is that four separate readings of the 5.0 list on this machine agree with each other, and the point the Mac sentence was making — that the category list is a fact about the operating system as well as about the version — is untouched by any of this.


## 6. At what point can each of the things above be learned — before a render starts, while it runs, or only once it has finished?

**Before it starts.** Everything that is a setting or a property of the installation, rather than a result: the engine (`scene.render.engine`, or fixed with `-E`), the Cycles device and thread count, the enabled add-ons (`preferences.addons`, or fixed with `--addons`), the machine's device list and capabilities (`system-info.txt`, whose Cycles section covers "each hardware render device available for use with Cycles"), and what the build can do (`bpy.app.build_options`). **documented.**

*Changed by the citation check, 2026-08-07.* This paragraph used to add that the release-notes log example shows the device lines "emitted at startup, before rendering", read off the printed timestamps. It does not: the device lines are at `00:00.359` and `00:01.041` and the render block begins at `00:00.244`, and the two halves are separated by the page's own elision. The reading has been removed, and with it the words "with the ordering read off the printed timestamps" from the label. When in the run the device is named was later settled by test — see the 2026-08-07 block below.

The device override is also decided at start: `--cycles-device` is described as "Set the device to use for Cycles, overriding user preferences and the scene setting." (Cycles file `cycles_5.0/engine.py`, lines 16–19 — read 2026-07-30, **observed**), and it is parsed inside `init()`, which runs when the add-on starts (same file, lines 44–53).

**While it runs.** Progress and running memory. The log example shows `Mem: 2983M | Sample 1/16` and `Sample 16/16` lines arriving before the save line. `bpy.app.handlers.render_pre`, `render_post` and `render_stats` fire during the job. Placeholder frames are created "while rendering" (Manual, *Output*, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30). Inside the interface the status bar "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate." (Manual, *Status Bar*, https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html — read 2026-07-30.) An engine drives that through `update_progress(progress)`, `update_stats(stats, info)` and `update_memory_stats(memory_used, memory_peak)` (API, *RenderEngine*, https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — read 2026-07-30).

For this version there is a specific caveat about what is visible during an animation with no window. The 5.0.1 corrective list contains:

> "* Show currently rendered frame in background mode. (blender/blender#150174)"

Release notes, *Corrective Releases*, https://developer.blender.org/docs/release_notes/5.0/corrective_releases/ — read 2026-07-30. **documented** as a change. What it implies about 5.0.0 is **inferred** — the step being that a fix titled "Show currently rendered frame in background mode" means it was not shown before the fix, which the release note does not state outright.

**Only once it has finished.** Everything that measures the whole job. Render Time and Memory (peak) as stamp fields are by nature end-of-job values, and metadata reaches disk when the frame is written — `render_write` fires "directly after the frame is written". The log's closing line is `Time: 00:28.38 (Saving: 00:00.15)`, after `Saved: '/tmp/0001.png'`. `render_complete` fires "on completion of render job". And an engine's `error_set(message)` is described as setting an "error message displaying after the render is finished".

**LABEL:** documented, except the Cycles file quotations about `--cycles-device`, which are observed, and one inference flagged in place — what the 5.0.1 fix implies about 5.0.0. The ordering read off the log timestamps stays documented, since the timestamps are printed in the quoted example.

**Where I looked.** Searched all four source sets for: `progress`, `while rendering`, `render_pre`, `render_post`, `render_complete`, `render_write`, `render_stats`, `placeholder`, `background mode`, `currently rendered frame`, `status bar`, `Saved:`, `Time:`, `Mem:`. Read `bpy.app.handlers` in full, `bpy.types.RenderEngine` in full, the Status Bar manual page, the Output manual page, the Command Line Arguments page, `release_notes_5.0/core.md`, `release_notes_5.0/python_api.md`, and `release_notes_5.0/corrective_releases.md` in full.

**Not covered.** The sources give one example of log output and a list of handlers; they do not give a timeline. I could not establish how often the progress lines are emitted, whether they are emitted at all at the default log level, or whether anything about the device is emitted at any point during the render rather than at startup. Nor did I establish whether metadata is written for a frame that fails partway.

**Outside the interface.** The status-bar progress bar and its hover time estimate exist only inside the interface. The log timeline, the handlers, and the metadata-on-write behaviour do not depend on a window — and the `render_stats` handler description names background mode explicitly. The 5.0.1 fix quoted above is specifically about background mode, so what is visible during a run differs between the two in this version. **Settled by test on 2026-08-06, on Linux only — see below.** All six render handlers fire with no window, in the order init, pre, post, write, complete, with the frame already on disk when `render_write` fires; `render_cancel` does not fire on a finished render. `bpy.app.is_job_running` works with no window and does not crash. So the inferred step above holds. **Added by the tests of 2026-08-07:** which parts of the computer are available can now be learned before a render starts, from outside the window; which part did the work can only be learned once the render has finished, from a closing report.

**Operating system.** Nothing in this answer is stated as system-dependent. The one nearby caution is EEVEE's "Headless rendering is not supported on headless Windows systems", which decides whether there is a run to observe at all rather than when things can be observed. **The tests recorded below were run on Linux only, on 2026-08-06; macOS is untested and Windows was not tried.** **The tests of 2026-08-07 were run on the same Linux machine, with its graphics card working and its driver at 580.173.02; macOS is still untested and Windows was still not tried.** So everything the card added above is an NVIDIA-on-Linux answer: CUDA and OptiX are what was seen, and Metal, HIP and oneAPI were not.

### Tested by hand, 2026-08-06 — **LABEL: observed**

Machine, Blender and settings folder as recorded under question 1 of this document: **the rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server — running `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings/5.0`) and never at the real one. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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


---

### Tested by hand with the graphics card working, 2026-08-07 — **LABEL: observed**

The machine of the 2026-08-06 tests, repaired. **The rented Linux machine** — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, no display server at all — now carrying NVIDIA driver **580.173.02** in both halves, the kernel module and the libraries agreeing at last. `nvidia-smi` answers, and Blender finds the card. Blender was `/opt/blender/5.0/blender`, which reported itself as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, pointed at a settings folder made for the purpose before it was used (`BLENDER_USER_RESOURCES=/root/what-actually-ran/settings-card/5.0`) and never at the real one. Nothing else was running on the machine. **Nothing was run on macOS**, so every result below covers Linux only. The whole procedure is in `Audits/Blender Research/What actually ran/Hand tests.md`.

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

2026-08-06 — Opus 5 (Claude Code) — What actually ran, Blender 5.0

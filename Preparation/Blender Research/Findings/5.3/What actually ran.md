# What actually ran — Blender 5.3

- **Sources consulted:** the Blender 5.3 manual, the Blender 5.3 Python API documentation and the Blender 5.3 release notes, all as copies taken on 2026-10-11; Cycles' own files for 5.3, as copies taken on 2026-10-16. I worked through them on 2026-10-17. As instructed, every "date read" below is the snapshot date: 2026-10-11 for the manual, API and release notes, 2026-10-16 for Cycles' own files.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Notes that belong beside that line, so nothing is hidden:
- The tool I run in placed some text into my context on its own when the session started, before I did anything. That text was the short signpost in the project's `CLAUDE.md` (it says only "read Start Here.md", which I did not do), a list of one-line titles of the owner's memory notes, and a git status listing file names and commit titles. I opened none of these files. None of that text describes how Blender behaves, and none of it is used below.
- Inside `Blender Sources/` I read the folder's own note `About this folder.md`. I did not open the `requirements/` folder, because the brief does not list it as a source answers may come from.
- Before writing, I ran one check that only counted whether a file with this file's name already existed in the output folder. It printed `0` and showed no other names.
- To look at one `.webp` picture from the manual, I converted a copy to `.png` inside the allowed scratch folder.
- I started no helpers and ran nothing in the background. I did not run Blender. Nothing below is labelled "observed" because of running anything; every "observed" finding comes from reading Cycles' own files.

Quotations from the manual are from the Blender Manual by the Blender Documentation Team, licensed CC-BY-SA 4.0. Quotations from Cycles' own files are from files licensed Apache-2.0.

How Cycles' own files are cited: by file name and line numbers inside the Cycles folder for 5.3, for example "Cycles `engine.py`, lines 13–15".

---

## 1. When a render has finished, what can be learned about which parts of the computer it used?

**Short answer.** Very little is recorded about this as a matter of course. Blender can write the machine's hostname and the peak memory into the saved image, if the file type carries metadata and those options are ticked. Nothing in Blender's own list of metadata names the CPU, the GPU, the compute backend or the thread count. What can be read afterwards is what was *asked for* — the scene's Cycles device, the Cycles device preferences, the thread setting — and Cycles' own files show three reasons those settings may not match what ran. Command-line options exist that print statistics and log "information about devices", but the sources do not say what those lines contain. Blender's "Save System Info" lists the Cycles devices *available*, not the ones a render used.

**Label:** documented (1.1, 1.3, 1.4 part, 1.5, 1.6 part, 1.7 part, 1.8), observed (1.4 part, 1.7 part), inferred (1.2, and the steps written out in 1.3, 1.4, 1.6, 1.7, 1.8).

**1.1 Hostname and peak memory can be written into the image — documented.**
- Manual, Output Properties ‣ Metadata: "Memory — Includes the peak memory usage." and "Hostname — Includes the rendering machine’s hostname." Address: https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html — read 2026-10-11.
- API, `RenderSettings.use_stamp_hostname`: "Include the hostname of the machine that rendered the frame (default False)". `RenderSettings.use_stamp_memory`: "Include the peak memory usage in image metadata (default True)". Address: https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11.
- Only some file types carry it. Manual: "Only some image formats support metadata" (metadata page, above). The table on the image-formats page marks Metadata ✓ for JPEG, OpenEXR, PNG and WebP, and ✗ for AVIF, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw and TIFF. The same page: "When using formats that do not support metadata, render information is not preserved in the saved file." Address: https://docs.blender.org/manual/en/5.3/files/media/image_formats.html — read 2026-10-11.
- The manual does not say whether "peak memory" means system memory, GPU memory, or both.

**1.2 Blender's own metadata has no field for the device, backend or threads — inferred.**
- The manual's "Include" list on the metadata page names: Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Note. The panel picture on that page (`_images/render_output_properties_metadata_panel.png`, documented) shows the same switches plus "Use Strip Me…" and "Burn Into Image". The API page for `RenderSettings` lists every `use_stamp_*` switch: camera, date, filename, frame, frame_range, hostname, labels, lens, marker, memory, note, render_time, scene, sequencer_strip, time.
- The step: the three lists agree and none of them names a CPU, GPU, backend, device or thread count. So Blender's own metadata does not record them.
- The edge of this: render engines can add their own fields. API, `RenderResult.stamp_data_add_field(field, value)`: "Add engine-specific stamp data to the result". Address: https://docs.blender.org/api/5.3/bpy.types.RenderResult.html — read 2026-10-11. I searched every Python file in Cycles' own files for "stamp" and found no use of it. Whether the compiled part of Cycles adds fields is not in the sources.

**1.3 The Note field can carry anything a script puts there — documented; using it for the device is inferred.**
- Manual, metadata page: "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number." Read 2026-10-11.
- API, `RenderSettings.stamp_note_text`: "Custom text to appear in the stamp note".
- The step: a script that set this text to a device name before rendering would get that text into the file. That records what the script wrote, not what Cycles did.

**1.4 What was asked for can be read afterwards; what ran can differ — observed and documented; step inferred.**
- The scene setting. Cycles `properties.py`, lines 32–36: `('CPU', "CPU", "Use CPU for rendering"),` and `('GPU', "GPU Compute", "Use GPU compute device for rendering, configured in the system tab in the user preferences"),`. Lines 430–434: `device: EnumProperty(` … `description="Device to use for rendering",` … `default='CPU',`. Read 2026-10-16.
- The preferences. Cycles `properties.py`, lines 1710–1717: `compute_device_type: EnumProperty(` … `name="Compute Device Type",` … `devices: CollectionProperty(type=CyclesDeviceSettings)`; lines 1674–1677 give each device `id`, `name`, `use` ("Use device for rendering") and `type`. Read 2026-10-16.
- Thread setting. API, `RenderSettings.threads`: "Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems)"; `threads_mode`: "Auto-Detect – Automatically determine the number of threads, based on CPUs." https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11.
- Difference one, the command line can override both. Cycles `engine.py`, lines 16–19: `"--cycles-device"`, `help="Set the device to use for Cycles, overriding user preferences and the scene setting."`. Read 2026-10-16.
- Difference two, a ticked device can be left out. Cycles `properties.py`, lines 2123–2124: `# Devices that do not meet the driver requirement are not used;` `# skip them.` Read 2026-10-16.
- Difference three, the final choice is made outside these files. Cycles `engine.py`, line 78: `engine.session = _cycles.create(engine.as_pointer(), prefs, data, screen, region, v3d, rv3d, preview_osl)`. Read 2026-10-16. The preferences are handed to the compiled part of Cycles, which is not in the sources.
- The step (inferred): reading the scene and the preferences afterwards tells you what was requested. It does not prove which device rendered, because of these three gaps.

**1.5 Command-line options that print or log about memory, time and devices — documented; their content is not.**
- Manual, Command Line Arguments, Cycles Render Options: "--cycles-print-stats — Log statistics about render memory and time usage." Cycles `engine.py`, lines 13–14, gives the same option the text `help="Print rendering statistics to stderr"` (observed, read 2026-10-16).
- Same manual page, Logging Options: "--log-level <level>" … "info: Information about devices, files, configuration, operations"; "--log "render,cycles": both render and cycles messages."; "--log-show-memory — Show memory usage for each log message."; Debug Options: "--debug-cycles — Enable debug messages from Cycles." Address: https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.
- The sources do not say what these lines contain. Nothing says they name the CPU or GPU that rendered.

**1.6 "Save System Info" lists the devices available — documented; that it omits the ones used is inferred.**
- Manual, Help menu: "Cycles — The instruction sets and capabilities of each hardware render device available for use with Cycles." and "GPU — Shows the GPU vendor, version and the capabilities of your hardware and driver." and "You will be prompted to save a text file called system-info.txt." Address: https://docs.blender.org/manual/en/5.3/getting_started/help.html — read 2026-10-11.
- API, `bpy.ops.wm.sysinfo(*, filepath='')`: "Generate system information, saved into a text file". Address: https://docs.blender.org/api/5.3/bpy.ops.wm.html — read 2026-10-11.

**1.7 The Cycles "Render Time" pass exists only when the scene device is CPU — observed and documented; step inferred.**
- Cycles `properties.py`, line 1583: `description="Reports time per pixel in milliseconds. Supported only on CPU render devices",`. Cycles `engine.py`, lines 229–232: `if crl.pass_render_time:` `# Only yield the pass if rendering on CPU` `if scene.cycles.device == 'CPU':` `yield (n_("Render Time"), "X", "VALUE")`. Read 2026-10-16.
- Manual, Passes: "An estimate for how long in milliseconds each pixel took to render. Note, this pass is not supported on GPU rendering." https://docs.blender.org/manual/en/5.3/render/layers/passes.html — read 2026-10-11.
- The step (inferred): a saved file with a "Render Time" pass was made while the *scene setting* said CPU. The test reads the scene setting, not a command-line override, so this does not settle what ran if the command-line override was used.

**1.8 EEVEE renders on the GPU only — documented; which GPU is inferred.**
- Manual, EEVEE limitations: "Being a rasterization engine, EEVEE only uses the power of the GPU to render." and "There is currently no support for multiple GPU systems." Address: https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html — read 2026-10-11.
- API, `gpu.platform.renderer_get()`: "Get GPU to be used for rendering."; also `device_type_get()`, `vendor_get()`, `version_get()` ("Get GPU driver version."). https://docs.blender.org/api/5.3/gpu.platform.html — read 2026-10-11.
- The step: these describe the GPU Blender's own graphics layer runs on. That EEVEE renders on that same GPU is my inference; no source says so in words.

**Not covered.** Whether any printed or logged line names the device that actually rendered. Whether "peak memory" includes GPU memory. Whether Cycles adds its own metadata fields. How work is split when GPU and CPU render together. Whether Workbench uses the GPU (I found no statement).

**Where I looked.** Manual: searched all pages for "metadata", "peak memory", "statistics", "render stat", "device used", "devices used", "which device", "which gpu", "used for rendering", "rendering machine", "render node", "hostname", "--log", "log-level", "debug-cycles", "print-stats", "system info". Read the metadata, image-formats, command-line arguments, command-line render, Help menu, GPU rendering, Cycles render settings, Cycles performance, preferences System, passes, status bar, image editor, EEVEE limitations pages. API: `RenderSettings`, `RenderResult`, `RenderEngine`, `bpy.app.handlers`, `bpy.ops.wm`, `gpu.platform`, `gpu`, `PreferencesSystem`, `Image`, `RenderSlot`, `imbuf`; searched all API pages for "stamp" and "metadata". Release notes: searched all subjects for "device", "gpu", "cpu", "metadata", "stamp", "log", "stats"; nothing bears on this question. Cycles' own files: read `engine.py` and `__init__.py` whole; read the device parts of `properties.py` and `ui.py`; searched every Cycles file for "stamp", "hostname", "metadata", "print", "report".

**Outside the interface.** The metadata switches are scene settings and nothing says they behave differently in a command-line render; that they apply there is inferred. The printing and logging options in 1.5 exist only on the command line. "Save System Info" is a menu item, but the same action is an operator that takes a file path (1.6), so a script can call it; the sources do not say whether it works, or fills every section, when Blender runs without its window. The preferences are used when running from the command line — manual, Scripting & Security: "In this case, the Preferences are still used but you may want to override them" (https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html, read 2026-10-11). For 1.8, without a window the GPU module must be started first — API: "gpu.init() — Initializes the GPU module for background use. If the initialization fails, a SystemError will be raised." (https://docs.blender.org/api/5.3/gpu.html, read 2026-10-11). **Settled by the hand tests of 2026-10-17, on the Creator's Mac only — see below.** With no window: the line naming the device that path-traced is printed at `--log-level info` on standard output (entry 2); the metadata switches apply and Cycles adds four fields of its own (entry 9); "Save System Info" works through `bpy.ops.wm.sysinfo`, names the graphics in its Cycles section and leaves its GPU section as `GPU: missing, background mode`, which only a window fills in (entries 22, 33); `gpu.init()` succeeds and the `gpu.platform` calls then name the machine's graphics (entry 28). Nothing in the image, the statistics or a render's own window names the device (entries 3, 9, 35); the settings read afterwards say what was asked for, not what ran (entries 16, 18, 19).

**Operating system.** The sources tie GPU backends to systems. Manual, GPU Rendering: "CUDA is supported on Windows and Linux", "OptiX is supported on Windows and Linux", "HIP is supported on Windows and Linux", oneAPI "is supported on Windows and Linux", and "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." (https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11). Cycles `properties.py`, lines 1686–1689, observed: `# Default to selecting the Metal compute device on Apple Silicon GPUs` … `return 5` (5 is Metal in the list at lines 174–181), so the preference starts on Metal on Apple Silicon Macs. Nothing says the hostname or memory metadata differs by system. **The hand tests of 2026-10-17 were run on the Creator's Mac only** (macOS 26.6.2, Apple M1 Pro, Metal) — so everything below is a macOS-on-Apple-silicon answer, with Metal the only graphics route seen; Linux, Windows and NVIDIA, AMD or Intel graphics were not tried in 5.3, there being no such machine for this sitting. The Cycles preference read `METAL` at factory startup here, as Cycles' own file says it should on Apple silicon (entry 18).

### Tested by hand on the Mac, 2026-10-17 — **LABEL: observed**

**The machine.** The Creator's own Mac — a MacBook Pro with an **Apple M1 Pro** chip, its own 14-core graphics reached through **Metal**, 16 GB of memory, **macOS 26.6.2** (build 25G83). Blender was `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, which reported itself as `Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)`. Every run was pointed at a settings folder made for it before it was used — `BLENDER_USER_RESOURCES` set to a folder inside the work folder, which Blender reported back as the one in use — and never at the Creator's own; Blender's temporary folder was pointed at the work folder too. **The scene** is the factory cube, camera and light with these changes: Cycles, 16 samples, denoising off, 160 by 120 pixels, frames 1 to 3, PNG, and its own output path set to a folder beside the file. It was built by Blender 5.2.2 so that 5.3 opens it as an older file. **Two limits.** The Creator's own Blender was open throughout and other work was running on the Mac, so nothing below is offered as a timing or as a memory measurement. And **nothing was run on Linux or Windows**, so every result below covers macOS on Apple's own graphics only. Each result names its entry in `Audits/Blender Research/What actually ran/Hand tests — 5.3.md`, which holds every command and script whole.

**A background render with no switches names no device** (entry 1). Run as `blender -b scene.blend -o out/p_ -F PNG -f 1`, with the two streams written to separate files: exit 0, a frame written, the error stream **0 bytes**, and no line naming a processor, the graphics, Metal, CUDA, OptiX, HIP or oneAPI. The whole of standard output, cut only in the middle, where thirty-four more `Fra: 1 | Mem: 1M | Updating …` lines of the same shape sit:

```
00:00.418  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/work/scene.blend"
00:00.423  render           | Rendering single frame
00:00.423  render           | Rendering frame 1
00:00.424  render           | Start rendering: Scene, ViewLayer
00:00.424  render           | Engine: Cycles
00:00.458  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.458  render           | Fra: 1 | Mem: 0M | Synchronizing object | Light
00:00.460  render           | Fra: 1 | Mem: 0M | Initializing
00:00.460  render           | Fra: 1 | Mem: 0M | Waiting for render to start
...
00:00.462  render           | Fra: 1 | Mem: 1M | Updating Objects | Copying Transformations to device
...
00:00.466  render           | Fra: 1 | Mem: 1M | Updating Device | Writing constant memory
00:00.466  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.469  render           | Fra: 1 | Remaining: 00:00.04 | Mem: 2M | Sample 1/16
00:00.482  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.482  render           | Fra: 1 | Mem: 2M | Finished
00:00.556  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/out/e1/p_0001.png'
00:00.556  render           | Time: 00:00.13 (Saving: 00:00.07)

Blender quit
```

The lines `Copying Transformations to device`, `Copying BVH to device`, `Copying Mesh to device` and `Updating Device | Writing constant memory` carry the word *device* without naming one. The version banner arrives part-way down because the two streams were captured to files and two writers reach the same file in their own order; it is on standard output like everything else.

**`--log-level info` on its own makes Blender name the device that path-traced, on standard output** (entry 2). Eight runs of the same scene, each with the two streams in separate files; every one exited 0 and wrote its frame, and every error stream was 0 bytes. Counting the lines that read `Path tracing on`:

| how it was run | standard output | error stream |
|---|---|---|
| `--log "*" --log-level info` | 1 | 0 |
| `--log "cycles" --log-level info` | 1 | 0 |
| `--log-level info` | 1 | 0 |
| `--debug-cycles` | 1 | 0 |
| `--log-level info`, and `-- --cycles-device METAL` | 1 | 0 |
| `--log-level info`, and `-- --cycles-device METAL+CPU` | 1 | 0 |
| `--log "render,cycles" --log-level info -b scene.blend -f 1 -- --cycles-print-stats` | 1 | 0 |
| the same, ending `-- --cycles-print-stats --cycles-device CPU` | 1 | 0 |

The line sits in a closing report. On the processor, from the `--log-level info` run, uncut:

```
00:00.352  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
                            |                    Hardware Ray-Tracing: Off
                            | 
                            | Render Scheduler Summary
                            | 
                            | Mode: Headless
                            | Resolution: 160x120
```

On the graphics, with `-- --cycles-device METAL`:

```
00:01.014  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                            |                    Hardware Ray-Tracing: Off
                            | 
                            | Render Scheduler Summary
                            | 
                            | Mode: Headless
                            | Resolution: 160x120
```

So the line names the chip, the number of graphics cores, the route (Metal), the word `display`, and an identifier built from them. In this version the `Hardware Ray-Tracing: Off` line is indented by twenty spaces after the bar, in every run; it reads `Off` on the processor and on the graphics alike.

**Asked for both at once, it names both.** With `-- --cycles-device METAL+CPU`, exit 0 and a frame written, the report read:

```
00:00.718  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                            |                    Hardware Ray-Tracing: Off
                            |                  Apple M1 Pro (CPU) (7 threads) [CPU]
                            |                    Hardware Ray-Tracing: Off
                            | 
                            | Render Scheduler Summary
                            | 
                            | Mode: Headless
                            | Resolution: 160x120
```

and, before the render began, three times over: `00:00.398  cycles           | CPU render threads reduced from 8 to 7, to dedicate to GPU.` Nothing printed says how the work was shared between the two. No earlier version's findings carry a run of this kind, so this stands beside silence there, not beside a different answer.

**One line must not be read as saying what ran.** `00:00.335  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".` appeared in the processor-only run as well as in the graphics runs. `Added device` names what Cycles found; `Path tracing on:` names what did the work.

**The graphics run also says, in its ordinary progress lines, that Metal programs were still being prepared**: `Fra: 1 | Mem: 1369M | Sample 0/16 (8 kernels to optimize)`, then `Sample 1/16 (4 kernels to optimize)` and `Sample 16/16 (4 kernels to optimize)`. The memory figure on those lines was `1369M`–`1370M` against `2M` for the processor run of the same scene; it is a memory reading on a shared machine and is offered only as an order of size.

**`--cycles-print-stats` names no device, and when the graphics did the work it says so by refusing** (entry 3). Run as `blender -b scene.blend -o out/s_ -F PNG -f 1 -- --cycles-print-stats`: exit 0, error stream 0 bytes, and a statistics block on standard output whose mesh, image, kernel, shader and object sections name nothing but the scene's own objects — searching the whole output for `metal`, `cuda`, `optix`, `oneapi`, `gpu`, `cpu`, `apple`, `m1` and `nvidia` matched nothing. The same with `--cycles-device METAL` added after the `--`: exit 0, error stream 0 bytes, and in place of the kernel, shader and object sections one sentence:

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
Profiling information not available (only works with CPU rendering)
```

That sentence was the only line in the graphics run's output matching any of the device words above.

**Nothing printed by an EEVEE or a Workbench render names the graphics chip that drew it** (entry 14). The same scene rendered with `-E BLENDER_EEVEE` and with `-E BLENDER_WORKBENCH`, each with `--log "*" --log-level info`, exit 0, error stream 0 bytes. EEVEE printed 23 lines and Workbench 18, and a search of both streams for `metal`, `apple`, `m1`, `gpu`, `graphic`, `renderer`, `vendor` and `device` matched **nothing** in either. The EEVEE run's whole standard output:

```
00:00.019  color_management | Using Blender config: "/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio"
00:00.127  translation      | Locale en_US used for translation
00:00.357  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/work/scene.blend"
00:00.362  blend            | Blender file read in 0m0.00s
00:00.362  blend            |  * Loading libraries: 0m0.00s
00:00.362  blend            |  * Applying overrides: 0m0.00s
00:00.362  blend            |  * Resyncing overrides: 0m0.00s (0 root overrides), including recursive resyncs: 0m0.00s)
00:00.362  render           | Rendering single frame
00:00.362  render           | Rendering frame 1
00:00.362  render           | Start rendering: Scene, ViewLayer
00:00.362  render           | Engine: EEVEE
00:00.362  ghost.system     | Create background system
00:00.362  ghost.system     | Create Cocoa system
00:00.499  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.789  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.956  render           | Fra: 1 | Rendering 50 / 64 samples
00:01.049  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.069  eevee            | Rendered 64 samples in 0.593607 seconds
00:01.137  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/out/e14/ee_0001.png'
00:01.137  render           | Time: 00:00.77 (Saving: 00:00.06)

Blender quit
```

The Workbench run's is the same shape, with `Engine: Workbench`, no sample lines and no `eevee` line. A Cycles render on the processor run the same way, as the control, printed its `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]` line and `Added device "Apple M1 Pro (GPU - 14 cores)" …` — so the search does find device names where they are printed.

**At the most detailed log level, EEVEE and Workbench name the drawing backend and still not the chip.** The same two renders with `--log-level debug` (505 and 443 lines, exit 0, error stream 0 bytes) printed lines in the `gpu.metal` category, such as `00:00.458  gpu.metal        | PSO CACHE: Stored new variant in PSO cache for shader 'WOWorld_world_world' Hash: '553244174571339776'` (twelve in the EEVEE run) and `00:00.448  gpu.metal        | PSO CACHE: Stored new variant in PSO cache for shader 'workbench_prepass_mesh_opaque_studio_material_no_clip' Hash: '1111703722838387789'` (five in the Workbench run), along with dozens of `gpu.shader | Compiling Shader "…"` lines. A Cycles processor render at the same level printed no `gpu.metal` or `gpu.shader` line at all. No line in any of these runs named the chip, its vendor or its renderer. This debug-level run was added after the information-level runs named nothing; no earlier version's findings carry either.

**The settings read during a run do not say what ran** (entry 16) — this is the gap 1.4 reasons about. One run set the scene to GPU Compute from a script, was overridden on the command line with `-- --cycles-device CPU`, and printed both settings back from inside itself, with `--log-level info` on: exit 0, a frame written, and

```
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
AFTER_OVERRIDE compute_device_type= METAL  scene.cycles.device= GPU
```

Both settings name the graphics; the processor did the work. On this Mac the Cycles preference reads `METAL` without being set, and a scene set to GPU Compute reached the graphics only from a settings folder with preferences saved into it — the processor otherwise, in a run where Cycles had found the graphics (entry 18, under question 6).

**The Render Time pass follows the scene's setting and pays no attention to what did the work** (entry 19) — so the step in 1.7, and any reading of the pass as a record of the device, does not hold. Six renders of the same frame to multi-layer OpenEXR, each with the pass switched on, a script pointing Cycles' preference at Metal, and `--log-level info`; the scene's device set to CPU or to GPU Compute, and each run with no override, with `-- --cycles-device CPU` and with `-- --cycles-device METAL`. All exited 0. "What did the work" below is each run's own `Path tracing on:` line; whether the pass was written was read from every part of the file's header:

| the scene's device | `--cycles-device` | what did the work, by its own report | `ViewLayer.Render Time.X` in the file |
|---|---|---|---|
| CPU | none | `Apple M1 Pro (CPU) (8 threads) [CPU]` | present |
| CPU | `CPU` | `Apple M1 Pro (CPU) (8 threads) [CPU]` | present |
| CPU | `METAL` | **`Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]`** | **present** |
| GPU | none | `Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]` | absent |
| GPU | `CPU` | **`Apple M1 Pro (CPU) (8 threads) [CPU]`** | **absent** |
| GPU | `METAL` | `Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]` | absent |

The two bold rows decide it: the pass was written when the scene said CPU and the graphics did the work, and withheld when the scene said GPU Compute and the processor did the work. A file that carries the pass was made from a scene set to CPU; it says nothing about what rendered it.

**How the pass is stored in this version.** When the pass is written, Blender writes the multi-layer OpenEXR as a file of two parts — read straight from the file: `multi-part: True`, `part 0: name='ViewLayer.Combined' channels=['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R']`, `part 1: name='ViewLayer.Render Time' channels=['ViewLayer.Render Time.X']`. The files without the pass are single-part, holding only the Combined channels. A reader that looks only at the first part of such a file sees no Render Time pass even where there is one; the first reading of these six files was made that way and is set aside for that reason.

**"Save System Info" works with no window; its Cycles section names the graphics and its GPU section is empty** (entry 22) — this settles what 1.6 and the *Outside the interface* line could not say. `blender -b --python-expr "import bpy; print(bpy.ops.wm.sysinfo(filepath='…/si_5.3.txt'))"`: exit 0, `{'FINISHED'}`, and a 96-line file. Its headings in order are `Blender:`, `Python:`, `Python (External Binary):`, `Directories:`, `FFmpeg:`, `Other Libraries:`, then the GPU line, `Cycles:` and `Enabled add-ons:`. The `Blender:` section carries `platform: 'macOS-26.6.2-arm64-arm-64bit-Mach-O'`. The GPU and Cycles parts, whole:

```
GPU: missing, background mode

Cycles:
=========================================


CPU device capabilities: 

Metal device capabilities:
Number of devices: 1
		Device: Apple M1 Pro (GPU - 14 cores)
```

`CPU device capabilities:` is blank on this chip. The `Directories:` section names the settings folder the run was pointed at. This describes the machine at the moment the file is written, not any render.

**Written with a window open, the same file fills its GPU section in, and differs in nothing else** (entry 33). From the same settings folder, the file was written once with no window and once by the same operator called from a script in a Blender started with its window. `{'FINISHED'}` both times; 96 lines without the window, 120 with it. `diff` of the two found only the temporary folder's name and this — where the windowless file has `GPU: missing, background mode`, the windowed file has:

```
GPU:
=========================================

renderer:	'Metal API'
vendor:		'Apple M1 Pro'
version:	'1.2'
device type:	'APPLE'
backend type:	'METAL'
extensions:

Implementation Dependent GPU Limits:
=========================================

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

So the one thing lost by writing the file from outside the window is its GPU section; the Cycles and add-on sections are the same either way.

**A render shown in Blender's own window names no device beside it** (entry 35). Blender 5.3 was started with its window on the shared scene; a script pointed Cycles at Metal and the scene at GPU Compute, made the render 1600 by 900 so its window would be wide enough to read, and started the render the way the interface does (`bpy.ops.render.render('INVOKE_DEFAULT')`, which returned `{'RUNNING_MODAL'}`, with the display preference reading `WINDOW`). Two seconds after the render completed, the render's own window was redrawn and photographed with Blender's screenshot command. The picture: an Image editor showing `Render Result`, `Slot 1`, `ViewLayer`, the finished cube, and above it the one line of text:

```
Frame:1 | Time:00:00.81 | Mem:1419M, Peak: 1419M
```

Frame, time and memory — nothing naming the graphics, Metal, the processor or a device. A first attempt at the scene's own 160 by 120 opened a window too narrow for the line, which was cut off at `Frame:1 | Time:00:00.25 | M`. No earlier version's findings carry this run.

**The `gpu` module answers with no window only after `gpu.init()`, and then names the machine's graphics** (entry 28) — the step 1.8 and its *Outside the interface* line rest on. With no window and no `gpu.init()`: `GPU_ERROR SystemError GPU functions for drawing requires the gpu module to be initialized. See gpu.init.` and `HAS_GPU_INIT True`. With `gpu.init()` called first, exit 0:

```
GPU_INIT_RETURNED None
GPU_RENDERER Metal API
GPU_DEVICE_TYPE APPLE
GPU_BACKEND METAL
GPU_VENDOR Apple M1 Pro
```

This names the machine's graphics and the drawing route, whatever a render used: it is asked of the program, not of a render.

**The metadata switches, the engine and the thread count at factory startup** (entry 12). Read by a script run as `blender -b --factory-startup --python t_stamp.py`, exit 0:

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
ENGINE 'BLENDER_EEVEE'
ENGINE_ITEMS ['BLENDER_EEVEE']
HAS_MULTIPLE_ENGINES True
THREADS 8 THREADS_MODE AUTO
```

**`use_stamp_memory` is False** at factory startup — not the "default True" the API page gives (1.1) — and **`use_stamp_hostname` is False**. The master switch `use_stamp` is False too. The thread count is this machine's eight cores.

**The Metadata panel itself, seen on screen, has Memory and Hostname unticked** (entry 30). Blender 5.3 was started with its window on a factory-startup scene file, and a script set the Properties editor to Output Properties with its own search box set to `Metadata`, forced the window to redraw, and took Blender's own screenshot. The panel showed `Metadata Input: Scene`, and under `Include`: **Date, Time, Render Time, Frame, Camera, Scene and Filename ticked; Frame Range, Memory, Hostname, Lens, Marker and Strip Name unticked**; then `Note` and `Burn Into Image` as closed sub-panels, both unticked. That agrees with the settings read through Python and with the manual's picture (1.2), and not with the API page's "default True" for Memory. No box appears that is not among the sixteen `use_stamp_*` settings, except `Metadata Input`, a drop-down with no such name. Two earlier screenshots from the same script, without the forced redraw, showed the Properties editor still on the Object tab while Blender reported it set to Output — the window had not been redrawn — and are set aside for that reason.

**No field written into a finished image names a device — on the processor or on the graphics — but the `Memory` field jumps** (entry 9). Every `use_stamp_*` switch was turned on and the note set to `SMOOZY NOTE` by a script run on the command line, and the same frame was written six ways: Cycles to PNG and to OpenEXR on the processor; the same two with `-- --cycles-device METAL` (each run's own device line read `Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]`); and EEVEE and Workbench to PNG. All exited 0. The fields were read straight out of the files — a PNG's text chunks and an OpenEXR's header attributes — by a small reader that decodes the text ones and nothing else. The Cycles PNG on the processor, every field:

```
   File = /private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/work/scene.blend
   Note = SMOOZY NOTE
   Date = 2026/10/17 23:23:33
   Marker = <none>
   Time = 00:00:00:01
   Frame = 1
   FrameRange = 1:3
   Camera = Camera
   Lens = 50.00
   Scene = Scene
   Strip = <none>
   RenderTime = 00:00.02
   Memory = 2M
   Hostname = Artists-Studio-MacBook.local
   cycles.ViewLayer.samples = 16
   cycles.ViewLayer.total_time = 00:00.01
   cycles.ViewLayer.render_time = 00:00.01
   cycles.ViewLayer.synchronization_time = 00:00.00
```

The Cycles PNG written on the graphics carries the same field names, with these values changed: `RenderTime = 00:00.36`, **`Memory = 1370M`**, `cycles.ViewLayer.total_time = 00:00.34`, `cycles.ViewLayer.render_time = 00:00.14`, `cycles.ViewLayer.synchronization_time = 00:00.20`. Searching every field of all six files for `metal`, `apple`, `m1`, `nvidia`, `geforce`, `cuda`, `optix`, `oneapi`, `device`, `addon` and `add-on` gave **0** matches. So the `Memory` figure is a hint that the graphics did the work and names nothing; on this Mac the graphics share the machine's memory, and the figure is a reading taken on a shared machine, offered only as an order of size. `Hostname` carried the machine's name.

---

## 2. When a render has finished, what can be learned about which render engine produced it?

**Short answer.** Blender's own image metadata has no field for the render engine. What can be read afterwards is the scene's engine setting, which is what was asked for; a command-line option can replace it for one run. Render engines are able to add their own metadata fields, but the sources do not say whether Cycles, EEVEE or Workbench does.

**Label:** documented (2.2, 2.3, 2.5), observed (2.4), inferred (2.1, and the steps written out in 2.3, 2.4, 2.6).

**2.1 No engine field in Blender's own metadata — inferred.** Same lists and same step as 1.2: the manual's "Include" list, the panel picture and the API's full set of `use_stamp_*` switches name no engine.

**2.2 Engines may add fields of their own — documented; whether they do is not in the sources.** API, `RenderResult.stamp_data_add_field`: "Add engine-specific stamp data to the result". https://docs.blender.org/api/5.3/bpy.types.RenderResult.html — read 2026-10-11. No Python file in Cycles' own files uses it (searched for "stamp").

**2.3 The scene's engine setting, and the command-line override — documented; step inferred.**
- API, `RenderSettings.engine`: "Engine to use for rendering (default 'BLENDER_EEVEE')". https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11. (The API page lists only `'BLENDER_EEVEE'` as a value; the page does not explain why.)
- Manual, Command Line Arguments: "-E, --engine <engine> — Specify the render engine. Use -E help to list available engines." https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.
- Manual, Rendering From The Command Line: "-E CYCLES — Use the “Cycles Render” engine." https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html — read 2026-10-11.
- Order matters. Same arguments page: "Arguments are executed in the order they are given." and "…will not render to /tmp because loading the blend-file overwrites the render output that was set."
- The step (inferred): the engine named in a saved file is what was asked for. If `-E` was used, the run may have used a different engine from the one in the file. By the manual's own rule for the output path, an `-E` given *before* the file would be overwritten when the file loads; the manual states that rule for the output path, not for the engine.

**2.4 The name Cycles goes by — observed.** Cycles `__init__.py`, lines 44–46: `class CyclesRender(bpy.types.RenderEngine):` `bl_idname = 'CYCLES'` `bl_label = "Cycles"`. Read 2026-10-16. So the engine setting reads `CYCLES` when Cycles was chosen.

**2.5 Other render engines come from add-ons — documented.** Manual, Render Engines: "More renderers from third-party developers are available as add-ons." https://docs.blender.org/manual/en/5.3/render/render_engines.html — read 2026-10-11. API, `RenderSettings.has_multiple_engines`: "More than one rendering engine is available".

**2.6 Asking during or on completion of the render — documented hooks; what they reveal is inferred.** API, `bpy.app.handlers`: `render_init` "on initialization of a render job. Accepts one argument: the scene data-block being rendered"; `render_pre`, `render_post`, `render_write`, `render_complete`, `render_cancel` take the same argument. https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11. The step: a script attached to one of these can read that scene's engine setting at that moment, which would include any `-E` change already applied. The sources do not say this in words.

**Not covered.** Whether Cycles, EEVEE or Workbench write any field that names them. Whether render-pass names in a multi-layer file identify the engine. What happens to the engine setting when `-E` comes before the file.

**Where I looked.** Manual: searched all pages for "engine-specific", "engine specific", "which engine", "rendered with", "rendered by", "engine used", "render engine" near "missing", "not found", "unknown", "not available", "add-on"; read the render-engines, introduction, metadata, command-line pages. API: `RenderSettings`, `RenderResult`, `RenderEngine`, `bpy.app.handlers`; searched all API pages for "stamp" and "metadata". Release notes: searched all subjects for "engine", "metadata", "stamp"; the only 5.3 render-engine note is about `bl_write_viewport_depth` (viewport drawing), which does not bear on this. Cycles' own files: `__init__.py`, `engine.py`; searched all Python files for "stamp".

**Outside the interface.** The engine setting and `-E` both work outside the window. The handlers can be attached from a script given on the command line; that they run in a command-line render is not stated but nothing excludes it (inferred). Seeing metadata in the Image Editor is inside the window; I found no Python function that reads metadata back out of a saved image (searched the API for "metadata" and "stamp"). **Settled by the hand tests of 2026-10-17, on the Creator's Mac only — see below.** With no window: the printed output names the engine at default settings (entry 1); Cycles writes its four `cycles.ViewLayer.*` fields and an OpenEXR carries `Software` (entry 9); the handlers do run in a command-line render (entry 7, under question 5); `-E` counts only after the file on the line, and a file whose engine is not installed renders with EEVEE after an `ERROR` line (entry 13); and 5.3's own `OpenEXR` module reads a written OpenEXR's fields back from Python (entry 11, under question 5). The Image editor has no Metadata panel at all (entry 31, under question 5).

**Operating system.** The sources do not say this differs by operating system. **The hand tests of 2026-10-17 were run on the Creator's Mac only**; Linux and Windows were not tried in 5.3.

### Tested by hand on the Mac, 2026-10-17 — **LABEL: observed**

The machine, the Blender and the method are described under question 1. Nothing was run on Linux or Windows.

**The printed output names the engine at default settings** (entry 1). The background render with no switches described under question 1 printed, on standard output, before any work began:

```
00:00.424  render           | Start rendering: Scene, ViewLayer
00:00.424  render           | Engine: Cycles
```

**A running Blender's own engine list holds one entry** (entry 12). At factory startup, read through the engine setting's own list of allowed values: `ENGINE 'BLENDER_EEVEE'`, `ENGINE_ITEMS ['BLENDER_EEVEE']`, `HAS_MULTIPLE_ENGINES True`. So the single value on the API page (2.3) is what a running Blender reports too — while the same Blender accepts `-E CYCLES` and `-E BLENDER_WORKBENCH` (entry 9) and lists three engines to `-E help` (entry 13, below).

**`-E help` lists three engines** (entry 13). `blender -b -E help`, exit 0, on standard output:

```
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

**Where `-E` sits on the line decides whether it counts** (entry 13). The shared scene is set to Cycles. `blender -b -E BLENDER_EEVEE scene.blend -o out/pre_ -F PNG -f 1` — the engine given *before* the file — exited 0 and printed `00:00.326  render           | Engine: Cycles`. `blender -b scene.blend -E BLENDER_EEVEE -o out/post_ -F PNG -f 1` — the engine given *after* the file — exited 0 and printed `00:00.333  render           | Engine: EEVEE`. So an engine named before the file is replaced by the file's own when the file loads, the same rule the manual states for the output path (2.3). No earlier version's findings carry this run.

**A file whose engine is not installed renders with EEVEE, and says so once** (entry 13). A file was made whose scene's engine is one no Blender has: a script defined a render engine called `SMOOZY_TEST_ENGINE` for the length of one session, set the scene to it and saved the file under a new name; nothing was installed and no preference saved. Opened and rendered by a fresh Blender — `blender -b foreign.blend --python-expr "…print the scene's engine…" -o out/fo_ -F PNG -f 1` — it exited **0**, wrote a frame, left the error stream at 0 bytes, and printed on standard output:

```
00:00.330  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/work/foreign.blend"
00:00.334  reports          | ERROR Engine 'SMOOZY_TEST_ENGINE' not available for scene 'Scene' (an add-on may need to be installed or enabled)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.334  render           | Rendering single frame
00:00.335  render           | Rendering frame 1
00:00.335  render           | Start rendering: Scene, ViewLayer
00:00.335  render           | Engine: EEVEE
00:00.467  render           | Fra: 1 | Rendering 1 / 64 samples
00:00.662  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.817  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.907  render           | Fra: 1 | Rendering 64 / 64 samples
00:00.994  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/out/e13/fo_0001.png'
00:00.994  render           | Time: 00:00.65 (Saving: 00:00.06)
ENGINE_AFTER_LOAD 'BLENDER_EEVEE'

Blender quit
```

The script's own line, `ENGINE_AFTER_LOAD 'BLENDER_EEVEE'`, comes from a script given before `-f` on the command line, so it ran once the file had loaded and before the render; in the captured file it arrived last. So once loaded, the scene's engine reads `BLENDER_EEVEE`: the file no longer says which engine it asked for, and a script reading the engine afterwards sees EEVEE. The `ERROR` line and the `Engine: EEVEE` line are the only record. No earlier version's findings carry this run.

**A finished image does carry a trace of the engine, in two ways** (entry 9). With every metadata switch on, the same frame was written by Cycles, EEVEE and Workbench (`-E CYCLES`, `-E BLENDER_EEVEE`, `-E BLENDER_WORKBENCH`), each exiting 0, and the fields read straight out of the files.

*Cycles writes four fields of its own; EEVEE and Workbench write none.* The Cycles PNG carried, after the fields the Metadata panel lists:

```
   cycles.ViewLayer.samples = 16
   cycles.ViewLayer.total_time = 00:00.01
   cycles.ViewLayer.render_time = 00:00.01
   cycles.ViewLayer.synchronization_time = 00:00.00
```

The EEVEE PNG of the same frame, every field:

```
   File = /private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/work/scene.blend
   Note = SMOOZY NOTE
   Date = 2026/10/17 23:23:38
   Marker = <none>
   Time = 00:00:00:01
   Frame = 1
   FrameRange = 1:3
   Camera = Camera
   Lens = 50.00
   Scene = Scene
   Strip = <none>
   RenderTime = 00:00.72
   Memory = 0M
   Hostname = Artists-Studio-MacBook.local
```

and the Workbench PNG carried exactly the same fourteen field names, with `RenderTime = 00:00.12` and `Memory = 0M`. So a file with `cycles.` fields was made by Cycles; a file without them could be EEVEE or Workbench, and its fields do not say which.

*An OpenEXR names the program.* The Cycles OpenEXR, every attribute the reader found:

```
   Camera (string) = Camera
   Date (string) = 2026/10/17 23:23:33
   File (string) = /private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/work/scene.blend
   Frame (string) = 1
   FrameRange (string) = 1:3
   Hostname (string) = Artists-Studio-MacBook.local
   Lens (string) = 50.00
   Marker (string) = <none>
   Memory (string) = 2M
   Note (string) = SMOOZY NOTE
   RenderTime (string) = 00:00.01
   Scene (string) = Scene
   Software (string) = Blender 5.3.0 Beta
   Strip (string) = <none>
   Time (string) = 00:00:00:01
   channels (chlist) = <73 bytes>
   colorInteropID (string) = lin_rec709_scene
   compression (compression) = <1 bytes>
   cycles.ViewLayer.render_time (string) = 00:00.01
   cycles.ViewLayer.samples (string) = 16
   cycles.ViewLayer.synchronization_time (string) = 00:00.00
   cycles.ViewLayer.total_time (string) = 00:00.01
   dataWindow (box2i) = <16 bytes>
   displayWindow (box2i) = <16 bytes>
   lineOrder (lineOrder) = <1 bytes>
   name (string) = 
   pixelAspectRatio (float) = <4 bytes>
   screenWindowCenter (v2f) = <8 bytes>
   screenWindowWidth (float) = <4 bytes>
   type (string) = scanlineimage
   xDensity (float) = <4 bytes>
```

`Software (string) = Blender 5.3.0 Beta` names the program and its version, not the engine. No PNG carried a `Software` field. The OpenEXR written on the graphics carried the identical set of attribute names.

*Channel names in a multi-layer file do not tell the engines apart, with the passes the scene has.* The scene written to multi-layer OpenEXR by Cycles and by EEVEE, with no metadata switches changed, and the channel names listed from each file's header:

```
=== cml_0001.exr ===
   channels: ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R']
=== eml_0001.exr ===
   channels: ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R']
```

Identical. This scene has only the Combined pass switched on; a pass only one engine offers was not tried here (the Render Time pass, which is Cycles' own, is under question 1's entry 19). No earlier version's findings carry the Workbench file or the channel names.

---

## 3. When a render has finished, what can be learned about which add-ons were in use while it ran?

**Short answer.** The rendered image does not record add-ons. A running Blender can be asked which add-ons are enabled, through the preferences, and "Save System Info" writes a list of add-ons "currently in use" with versions and paths. Both describe the moment they are asked, not the render. Add-ons can be switched on and off at any time, and the command line can enable extra ones for one run.

**Label:** documented (3.2, 3.3, 3.4, 3.5, 3.6), observed (3.2), inferred (3.1, and the steps in 3.2, 3.4, 3.5).

**3.1 No add-on field in the image's metadata — inferred.** Same lists and step as 1.2; none of them names add-ons.

**3.2 The preferences hold the add-on list — documented and observed; that it means "enabled" is inferred.**
- API, `Preferences.addons`: "(default None, readonly)" of type `Addons[Addon]`. `Addon`: "Python add-ons to be loaded automatically"; `Addon.module`: "Module name". https://docs.blender.org/api/5.3/bpy.types.Addon.html and https://docs.blender.org/api/5.3/bpy.types.Preferences.html — read 2026-10-11.
- Cycles reads its own entry there. Cycles `ui.py`, line 100: `return context.preferences.addons[__package__].preferences.compute_device_type`. Read 2026-10-16 (observed).
- Extension add-ons carry their repository in the name. Manual, Add-ons (extensions): "now instead of kitsu the module name would be bl_ext.{repository_module_name}.kitsu instead." https://docs.blender.org/manual/en/5.3/advanced/extensions/addons.html — read 2026-10-11.
- The step: "to be loaded automatically" plus Cycles finding itself in this list point to this being the list of enabled add-ons. The API does not say "enabled" in words. The API gives no version field for an add-on.

**3.3 "Save System Info" lists add-ons in use, with versions and paths — documented.** Manual, Help menu: "Enabled Add-Ons — Lists add-ons currently in use along with their versions and paths." https://docs.blender.org/manual/en/5.3/getting_started/help.html — read 2026-10-11. The same action as an operator: `bpy.ops.wm.sysinfo(*, filepath='')` — "Generate system information, saved into a text file". https://docs.blender.org/api/5.3/bpy.ops.wm.html — read 2026-10-11.

**3.4 The command line can add add-ons for a run — documented.** Manual, Command Line Arguments, Python Options: "--addons <addon(s)> — Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons." https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11. The step (inferred): a list read from the saved preferences after the run would not show add-ons enabled only this way, unless something saved them; the sources do not say whether `--addons` is saved.

**3.5 Add-ons can be turned on or off while Blender runs — documented.** API, `bpy.ops.preferences.addon_enable`: "Turn on this add-on"; `addon_disable`: "Turn off this add-on". https://docs.blender.org/api/5.3/bpy.ops.preferences.html — read 2026-10-11. Manual: "To enable or disable an add-on check or uncheck the box to the right of the add-ons. The add-on functionality should be immediately available." https://docs.blender.org/manual/en/5.3/editors/preferences/addons.html — read 2026-10-11. The step (inferred): a list taken before or after a render can differ from what was enabled during it.

**3.6 Add-on problems are printed to the console — documented.** Manual, Add-ons preferences: "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred." Same page. And: "Refresh Local — … Any issues are reported as warnings."

**Not covered.** Whether `--factory-startup` changes which add-ons load (the manual says only "Skip reading the startup.blend in the users home directory."). Whether any add-on list is written to disk by a render. Whether `--addons` is remembered.

**Where I looked.** Manual: searched all pages for "enabled add-on", "add-ons currently", "add-ons in use", "loaded add-on", "default add-on", "--addons", "factory-startup", "background mode", "system info"; read the Add-ons preferences page, the extensions Add-ons page, the extension command-line page, the Scripting & Security page, the Help menu page. API: `Preferences`, `Addons`, `Addon`, `AddonPreferences` (by name), `bpy.ops.preferences`, `bpy.ops.wm`, `bpy.app`; searched for "addon_utils" (not present). Release notes: searched all subjects for "add-on", "addon", "extension"; the hits are about assets and Loop Tools features, not about listing add-ons. Cycles' own files: `__init__.py` (its `bl_info`), `ui.py`, `properties.py`.

**Outside the interface.** The preferences list and the operators can be used from a script without the window (the API exposes them; that they work there is inferred). `--addons` is command-line only. The manual says the preferences are still used on the command line (quoted under question 1). "Save System Info" from a script without the window is not described. **Settled by the hand tests of 2026-10-17, on the Creator's Mac only — see below.** With no window, the add-on list can be read by a script and `bpy.ops.wm.sysinfo` writes its add-on section with versions and paths; both describe the moment they are read (entries 22, 23). The add-on list read during a run does not include an add-on loaded by `--addons`, and `--factory-startup` does not change it (entry 24). With a window open the add-on section is the same (entry 33, under question 1).

**Operating system.** Not stated to differ. **The hand tests of 2026-10-17 were run on the Creator's Mac only**; the add-on paths they show are macOS paths inside the application, and Linux and Windows were not tried in 5.3. The manual notes a Windows-specific cause of add-on failure — "an add-on that just fails when enabled with an error – e.g: ... is not a valid Win32 application." (https://docs.blender.org/manual/en/5.3/troubleshooting/python.html, read 2026-10-11) — which concerns failing to enable, not what can be learned.

### Tested by hand on the Mac, 2026-10-17 — **LABEL: observed**

The machine, the Blender and the method are described under question 1. Nothing was run on Linux or Windows.

**Nothing about add-ons reaches a rendered image** (entry 9). Every field written into six files with every metadata switch on — Cycles PNG and OpenEXR on the processor and on the graphics, EEVEE PNG, Workbench PNG — is listed under questions 1 and 2. None names an add-on: a search of every field for `addon` and `add-on` gave 0 matches. The only engine-shaped fields are Cycles' four `cycles.ViewLayer.*` fields.

**"Save System Info" works with no window and lists the enabled add-ons with versions and paths** (entry 22). `bpy.ops.wm.sysinfo(filepath=…)` from a Blender with no window returned `{'FINISHED'}` and wrote a 96-line file whose last section, headed `Enabled add-ons:` (not "Enabled Add-Ons" as the manual writes it), read:

```
io_anim_bvh (version: (1, 0, 1), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/io_anim_bvh/__init__.py')
io_curve_svg (version: UNKNOWN, path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/io_curve_svg/__init__.py')
io_mesh_uv_layout (version: (1, 2, 0), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/io_mesh_uv_layout/__init__.py')
io_scene_fbx (version: (5, 15, 0), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/io_scene_fbx/__init__.py')
io_scene_gltf2 (version: (5, 3, 40), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/io_scene_gltf2/__init__.py')
cycles (version: UNKNOWN, path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/cycles/__init__.py')
pose_library (version: (2, 0), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/pose_library/__init__.py')
bl_pkg (version: (0, 0, 1), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/bl_pkg/__init__.py')
```

So the "Save System Info" route reaches outside the window, and what it lists is what is on at the moment the file is written.

**The add-on list it writes is current, not a record of the session's start** (entry 23) — the step 3.5 infers. In one Blender session with no window: the file written, `io_anim_bvh` switched off with `bpy.ops.preferences.addon_disable(module='io_anim_bvh')`, the file written again, nothing saved:

```
ON_BEFORE True
DISABLE {'FINISHED'}
ON_AFTER False
   io_anim_bvh in first file: 1  in second file: 0
```

So a list taken after a render can differ from what was on during it, and the file says only what is on when it is written.

**The list a script reads leaves out add-ons switched on by `--addons`, and `--factory-startup` changes nothing in it** (entry 24) — this settles 3.4's open step and the *not covered* lines about `--factory-startup` and whether `--addons` is remembered. This Blender ships thirteen add-ons in its own folder (`bl_pkg`, `cycles`, `hydra_storm`, `io_anim_bvh`, `io_curve_svg`, `io_mesh_uv_layout`, `io_scene_fbx`, `io_scene_gltf2`, `node_wrangler`, `pose_library`, `rigify`, `ui_translate`, `viewport_vr_preview`), and has eight of them on. Each command below printed the add-on list from inside a Blender with no window:

| how it was run | what the list read |
|---|---|
| `blender -b --python-expr "…"` | `['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']` |
| `blender --factory-startup -b --python-expr "…"` | the same eight |
| `blender -b --addons node_wrangler --python-expr "…"` | the same eight, and nothing printed about `node_wrangler` |
| `blender -b --addons space_view3d_math_vis --python-expr "…"` | the same eight, after `Add-on not loaded: "space_view3d_math_vis", cause: No module named 'space_view3d_math_vis'` |
| `blender -b --addons hydra_storm --python-expr "…"`, against a settings folder of its own | the same eight |
| then `blender -b --python-expr "…"` against that folder, no `--addons` | the same eight |

So an add-on that does not exist is named in a message; one that ships and is off is named in nothing. To see whether `--addons` did anything at all, the run also asked Python whether the add-on's code had been loaded: with `--addons hydra_storm`, `LISTED False IN_SYS_MODULES True` — the same whether `--addons` stood before `-b`, after it, or after a scene file — and without `--addons`, `LISTED False IN_SYS_MODULES False`. So `--addons` loads the add-on's code for that run, the add-on list read during the same run does not include it, and the next run without the flag does not have it either; the settings folder held nothing afterwards. A record of the add-ons in use, read from that list during or after a render started with `--addons`, misses the ones `--addons` brought in. No earlier version's findings carry the `--factory-startup`, `hydra_storm` or loaded-code runs.

---

## 4. What does a render do when it is asked for something the computer or the installed Blender cannot provide, and what does it report?

**How I read the question.** It was broad. I took it to mean requests that the hardware, its drivers, or this build of Blender cannot meet: a GPU or GPU feature that is absent or too old, not enough memory, a feature this build lacks, or an engine that is not installed. I left out missing project files (textures, linked files), which a reader might also count; those are about the project, not the computer.

**Short answer.** The sources describe several cases, mostly by what Blender does rather than what it says. A Cycles scene set to GPU with no GPU backend chosen uses the CPU. GPU devices with too-old drivers are not used, and Cycles' own logic then treats the render as a CPU render; the sources show no message for this. A full GPU memory spills into system memory on four backends; otherwise there is an "Out of memory" error. A bad `--cycles-device` value can make Blender exit with code 1 before anything renders. Features a device cannot do are dropped or simply not available. For an engine that is not installed, the sources are silent.

**Label:** documented (4.1 part, 4.4, 4.5, 4.7 part, 4.8 part, 4.9, 4.10, 4.11), observed (4.1 part, 4.2, 4.3, 4.6 part, 4.7 part, 4.8 part), inferred (steps written out in 4.2, 4.3, 4.6, 4.7), not in the sources (4.12).

**4.1 Scene asks for GPU, no GPU backend chosen: the CPU is used — documented and observed.** Manual, Preferences ‣ System ‣ Cycles Render Devices: "None: When set to None or when the only option is None: the CPU will be used as the computing device for Cycles." https://docs.blender.org/manual/en/5.3/editors/preferences/system.html — read 2026-10-11. What it reports: not in the sources. Inside the window, Cycles draws the scene's Device setting as inactive — Cycles `ui.py`, lines 2549–2550: `col.active = show_device_active(context)` `col.prop(cscene, "device")`, where `show_device_active` (lines 145–149) is false when the scene says GPU and no GPU is in use (observed, read 2026-10-16).

**4.2 GPU drivers too old, or no usable GPU: devices are skipped and Cycles' own test says CPU — observed; that the render then runs on CPU is inferred.**
- Cycles `properties.py`, lines 2123–2124: `# Devices that do not meet the driver requirement are not used;` `# skip them.` Lines 1849–1851 (counting GPU devices): `# Skip devices that do not meet the driver requirement.` `if not device[8]:` `continue`. Lines 1875–1876: `def has_active_device(self):` `return self.get_num_gpu_devices() > 0`. Read 2026-10-16.
- Cycles `ui.py`, lines 107–110: `def use_cpu(context):` … `return (get_device_type(context) == 'NONE' or cscene.device == 'CPU' or not backend_has_active_gpu(context))`. Read 2026-10-16.
- The step: Cycles' own interface code counts a render as CPU when no GPU device is both ticked and meeting its driver requirement. The device choice for the render itself is made in the compiled part (see 1.4), which is not in the sources, so "the render runs on the CPU" is my inference from Cycles' interface code.
- What it reports. Only interface text appears in the sources. In the preferences, a device below the driver requirement gets " (Disabled)" added to its name (`properties.py`, line 2056) and a line such as "Requires NVIDIA driver version %s or newer" (lines 2061 and 2063). When no device of the chosen type exists, the preferences show "No compatible GPUs found for Cycles" (line 1969) followed by requirement lines such as "Requires Apple Silicon with macOS %s or newer" (line 2040). Nothing in the sources says anything is printed during a render.
- A note on the numbers: Cycles `properties.py`, line 1958, sets `optix_minimum_driver_version = "580"` for that text, while the manual's GPU Rendering page says OptiX needs "a driver version of at least 575". The other backends' driver numbers also differ between the two sources. Which number the actual check uses is decided in the compiled part and is not in the sources.

**4.3 `--cycles-device` that Cycles cannot set: Blender exits with code 1, before rendering — observed; step inferred.**
- Cycles `engine.py`, lines 38–41: `if args.cycles_device:` `import _cycles` `if not _cycles.set_device_override(args.cycles_device):` `sys.exit(1)`. Read 2026-10-16.
- When this runs: `engine.py` lines 44–53, `init()` calls `_parse_command_line()`; `__init__.py` line 146, `register()` calls `engine.init()`. So it runs when the Cycles add-on is registered. Lines 28–29: `if "--" not in argv:` `return` — the option is read only after a double dash, as the manual also says: "Cycles add-on options must be specified following a double dash."
- The step (inferred): registration happens as Blender starts, so a refused value stops Blender before any file renders, with exit code 1.
- Not in the sources: which values make the override fail (only the allowed names are listed: "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'." — `engine.py` line 18), whether asking for a backend the machine lacks counts as failing, and what is printed.

**4.4 GPU memory runs out — documented.** Manual, GPU Rendering: "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering." and, under Error Messages, "Error: Out of memory — This usually means there is not enough memory to store the scene for use by the GPU." https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html — read 2026-10-11. oneAPI is not in that list; the sources do not say what oneAPI does. For EEVEE: "In practice, using too much GPU memory can make the GPU driver crash, freeze, or kill the application." https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html — read 2026-10-11. Manual, Crashes: "The most common causes of Blender crashes: Running out of memory. Issues with graphics hardware or drivers." and "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash." https://docs.blender.org/manual/en/5.3/troubleshooting/crash.html — read 2026-10-11.

**4.5 No ready GPU program for the card — documented.** Manual, GPU Rendering: "CUDA Error: Kernel compilation failed — This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed. In this case Blender may try to dynamically build a kernel for your graphics card and fail." Same page as 4.4.

**4.6 Optimized GPU programs not ready yet (Metal) — observed setting description; behavior during render inferred.** Cycles `properties.py`, lines 1751–1753: "Optimized kernels are requested at the start of a render. If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set is available in the cache. This can result in additional CPU usage for a brief time (tens of seconds)". Read 2026-10-16. Lines 2145–2152 show this setting only for Metal on Apple Silicon. The interface adds " (Unoptimized Performance)" to a device name when it is not optimized (line 2048).

**4.7 Features a device cannot do.**
- Render Time pass on GPU — observed: the pass is not created unless the scene device is CPU (`engine.py`, lines 229–232, quoted in 1.7). The step (inferred): the pass is silently missing; that code has no message.
- Path guiding on GPU — documented and observed. Manual, Sampling: "Path guiding is only available when rendering on a CPU." (https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html, read 2026-10-11). Cycles `source/kernel/features.h`, lines 163–168: `#ifndef __KERNEL_GPU__` `#  if defined(WITH_PATH_GUIDING)` `#    define __PATH_GUIDING__` … `#endif /* !__KERNEL_GPU__ */` (read 2026-10-16) — the GPU programs are built without it. What a GPU render reports when path guiding is switched on: not in the sources.
- OSL on a GPU other than OptiX — documented as unsupported: "OSL is not supported with GPU rendering unless using the OptiX backend." (https://docs.blender.org/manual/en/5.3/render/cycles/osl/index.html, read 2026-10-11). What happens if it is switched on anyway: not in the sources.
- Denoise node on GPU — documented fallback: "GPU: Use the GPU to process the denoise node if available, otherwise fallback to CPU." (https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html, read 2026-10-11).

**4.8 Features this build of Blender lacks.**
- Documented that builds differ: API, `bpy.app.build_options` lists compiled features, for example `cycles=True, cycles_osl=True, … image_openexr=True` in the documented build (https://docs.blender.org/api/5.3/bpy.app.html, read 2026-10-11). Manual, Format Options: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." What a render does when asked for a missing format: not in the sources.
- Observed for OSL: Cycles `__init__.py`, lines 103–108: `def update_script_node(self, node):` `if engine.with_osl():` … `else:` `self.report({'ERROR'}, "OSL support disabled in this build")` (and the same at lines 110–115 for custom cameras). Read 2026-10-16. This is reported when a script node or custom camera is compiled, not during the render itself.

**4.9 EEVEE with no screen on Windows — documented.** Manual, EEVEE limitations: "Headless rendering is not supported on headless Windows systems." Same page as 4.4. What it reports: not in the sources.

**4.10 How a render engine reports, in general — documented.** API, `RenderEngine.report(type, message)`: "Report info, warning or error messages"; `RenderEngine.error_set(message)`: "Set error message displaying after the render is finished". https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-10-11. Cycles' Python files do not call `error_set` (searched); whether the compiled part does is not in the sources.

**4.11 Where reports go — documented.** Manual, Command Line Arguments: "-q, --quiet — Suppress status printing (warnings & errors are still printed)." Manual, Using Blender From The Command Line: "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender’s operations, status, and internal errors." (https://docs.blender.org/manual/en/5.3/advanced/command_line/index.html, read 2026-10-11). Inside the window, the status bar shows "Informational messages or warnings" that "disappear after a short time. Click them to show the full message in the Info Editor." (https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html, read 2026-10-11).

**4.12 Engine not installed — not in the sources.** Neither what happens when `-E` names an engine that is not present, nor what happens when a file's engine comes from an add-on that is not installed. I searched every manual, API and release-notes page for "engine" near "missing", "not found", "unknown", "not available", "unavailable", "unregistered", "not registered", "invalid engine", "no engine". The only related statement is that `-E help` lists the engines available.

**Not covered.** The exact text, if any, printed for 4.1, 4.2, 4.7 and 4.12. Which `--cycles-device` values fail. What a missing image format does. oneAPI's behaviour when GPU memory is full. Whether EEVEE can render on Linux or macOS with no screen.

**Where I looked.** Manual: searched all pages for "fall back", "fallback", "falls back", "not supported", "unsupported", "compiled", "this build", "driver requirement", "driver version", "exit code", "out of memory", "headless", "background mode", "magenta", "missing"; read the GPU Rendering, Preferences System, Cycles render settings, performance, sampling, OSL, EEVEE limitations, crash, startup, Python troubleshooting, GPU troubleshooting (index and Linux NVIDIA) pages, and the picture `_images/editors_preferences_section_system.webp` (it shows the Cycles Render Devices row with only "None", "CUDA", "HIP" on the machine pictured — documented, and consistent with Cycles `properties.py` lines 1692–1708 offering only the backends found). API: `RenderEngine`, `RenderResult`, `bpy.app`, `bpy.ops.render`, `gpu`, `gpu.platform`, `PreferencesSystem`. Release notes: all subjects searched; the only hit near this question is EEVEE's Vulkan note that the option for hardware shadows "will be shown as inactive if the current backend/GPU doesn't support hardware ray queries" (https://developer.blender.org/docs/release_notes/5.3/eevee/, read 2026-10-11), which is a viewport setting. Cycles' own files: `engine.py`, `__init__.py` whole; device and denoiser parts of `properties.py`; `ui.py` device helpers; `source/kernel/features.h`; searched all files for "fallback", "report(", "error_set", "print(".

**Outside the interface.** 4.1, 4.2, 4.4 and 4.7 are about the render, so they apply outside the window too; the inactive setting and the preference texts in 4.1–4.2 and 4.6 are seen only inside the window. 4.3 exists only on the command line. 4.11's console messages are the outside-the-window channel; the status bar and Info Editor are inside only. 4.10's reports reach the window; whether `report` messages are also printed without a window is not stated, except that "warnings & errors are still printed" under `--quiet`. **Settled in part by the hand tests of 2026-10-17, on the Creator's Mac only — see below.** With no window, every shortfall tried reached the printed output or the exit status: a refused `--cycles-device` (exit 1, no frame — entry 15), an unknown engine on the command line (exit 1 — entry 13), a file's engine not installed (an `ERROR` line, then EEVEE, exit 0 — entry 13), a render that cannot save (exit 1 — entry 25), an unknown image format (a complaint, then the scene's own format — entry 26), an unknown argument (read as a file name — entry 26), and a render stopped part-way (no file, status 143 — entry 27). Three shortfalls say nothing at all with no window: a format that cannot hold metadata (entry 10), a GPU scene with the preference set to None (entry 17, which only the device line at `--log-level info` reveals), and path guiding or Open Shading Language asked of the graphics (entries 20, 21). The preference panel's texts were reached only with a window, and "No compatible GPUs found for Cycles" not at all on this Mac (entry 32).

**Operating system.** Yes, it differs. GPU backends by system are quoted under question 1. The HIP and oneAPI requirement texts in Cycles' preferences differ for Windows and Linux (`properties.py`, lines 1985–2037 and 2065–2077). The Metal kernel note (4.6) applies only to Apple Silicon Macs. EEVEE's headless limit (4.9) is stated for Windows only. The crash file differs: manual, Crashes — Windows "test.blend will create a file called test.crash.txt"; macOS "the macOS Crash Reporter shows a window with backtrace"; Linux "a file named blender.crash.txt is written to the /tmp directory." **The hand tests of 2026-10-17 were run on the Creator's Mac only**, so they settle macOS on Apple silicon with Metal. On this Mac `METAL` and `CPU` are the backends `--cycles-device` accepts and `OPTIX`, `CUDA`, `HIP` and `ONEAPI` are refused (entry 15); the Windows-only and Linux-only cases above — headless EEVEE on Windows, the display-driver timeout, the GCC message, the crash-file names — and every NVIDIA, AMD and Intel graphics case, including a too-old driver and a full graphics memory, were not tried in 5.3: there was no Linux machine, no Windows machine and no such card for this sitting. Where a crash report lands on macOS was not tried either, since it needs Blender made to crash and writes outside the work folder.

### Tested by hand on the Mac, 2026-10-17 — **LABEL: observed**

The machine, the Blender and the method are described under question 1. Nothing was run on Linux or Windows, so every Windows- and Linux-specific line above stays untested here.

**A format that cannot hold metadata drops it without a word** (entry 10). With every metadata switch on, the scene written as `-F TIFF` and then as `-F TGA`: both exited 0 and wrote their file (`TIFF_0001.tif`, `TGA_0001.tga`), and counting the lines in both streams that match any of `warn`, `error`, `metadata`, `not supported` or `preserved` gave **0** for each.

**An engine named on the command line that does not exist is refused; an engine a file asks for that is not installed is replaced** (entry 13) — this settles what 4.12 found not in the sources. `blender -b scene.blend -E NOT_AN_ENGINE -o out/be_ -F PNG -f 1`: **exit 1**, no frame written, and on the error stream an empty line and then `Error: engine not found 'NOT_AN_ENGINE'`. But a file saved with an engine this Blender does not have — `SMOOZY_TEST_ENGINE`, defined for one session by a script and never installed — rendered: **exit 0**, a frame written, the error stream 0 bytes, and on standard output `00:00.334  reports          | ERROR Engine 'SMOOZY_TEST_ENGINE' not available for scene 'Scene' (an add-on may need to be installed or enabled)` followed by `00:00.335  render           | Engine: EEVEE`. The whole output is under question 2. So in the second case the run reports an error and finishes anyway, on a different engine, and its exit status says it succeeded.

**A device Cycles does not know, or a backend this Mac has no hardware for, stops the run with exit 1 and no frame** (entry 15) — this settles which values 4.3 could not name. Eight runs of `blender -b scene.blend -o out/d_<name>_ -F PNG -f 1 -- --cycles-device <name>`, each writing to its own output name:

| `--cycles-device` | exit | frame written | error stream |
|---|---|---|---|
| `NOTADEVICE` | 1 | no | 0 bytes |
| `BANANA` | 1 | no | 0 bytes |
| `OPTIX` | 1 | no | 0 bytes |
| `CUDA` | 1 | no | 0 bytes |
| `HIP` | 1 | no | 0 bytes |
| `ONEAPI` | 1 | no | 0 bytes |
| `METAL` | 0 | yes | 0 bytes |
| `CPU` | 0 | yes | 0 bytes |

A name Cycles does not know — the whole of standard output:

```
00:00.384  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

(`BANANA` gave the same, with its own name.) A real backend with no hardware behind it — the whole of standard output for `OPTIX`, and `CUDA`, `HIP` and `ONEAPI` printed the same `ERROR` line:

```
00:00.329  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/work/scene.blend"
00:00.335  render           | Rendering single frame
00:00.335  render           | Rendering frame 1
00:00.336  render           | Start rendering: Scene, ViewLayer
00:00.336  render           | Engine: Cycles
00:00.347  render           | Fra: 1 | Mem: 0M | Initializing
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.347  reports          | ERROR Found no Cycles device of the specified type

Blender quit
```

Every refusal is on standard output, and none of them mentions a CUDA driver or any library. The two successes say nothing about the device at all. The `METAL` run's ordinary progress lines did carry a note about Metal programs: `Fra: 1 | Mem: 1369M | Sample 0/16 (Using optimized kernels)`, then `Sample 1/16 (4 kernels to optimize)` and `Sample 16/16 (4 kernels to optimize)`, and the frame was written — so a render on the graphics goes ahead while those programs are still being prepared, and its progress lines say how many are left. How long that takes, and what a render prints when none of them is ready yet, was not tested (see *Cannot be run*).

**A scene set to GPU Compute with the Cycles preference set to None renders on the processor, and says so** (entry 17) — the case 4.1 describes. A script refreshed the device list, set the preference to `NONE`, unticked every device and set the scene to GPU Compute; the run had `--log "*" --log-level info`. Exit 0, a frame written, the error stream 0 bytes:

```
SET_NONE ok
PREF_COMPUTE_DEVICE_TYPE NONE
DEVICES_IN_USE [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', False)]
SCENE_CYCLES_DEVICE GPU
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
00:00.425  render           | Time: 00:00.08 (Saving: 00:00.06)
```

The same file run beside it with the scene set to CPU printed the identical `Path tracing on:` line and `Time: 00:00.08 (Saving: 00:00.06)`. A search of the GPU-asking run's output for `warn`, `fall` and `substitut` found nothing. So nothing warns; the only sign is the device line, which needs a log switch.

**Path guiding switched on with the graphics doing the work: the render finishes, on the graphics, and nothing is said** (entry 20) — the case 4.7 left open. The running Blender was first asked for its Cycles settings about guiding; the on-off one is `use_guiding`, which it describes as `Guiding | Use path guiding for sampling paths. Path guiding incrementally learns the light distribution of the scene and guides path into directions with high direct and indirect light contributions`, off by default. Four renders with `--log "*" --log-level info`: on the graphics (a script pointing Cycles at Metal and the scene at GPU Compute) with guiding on and off, and on the processor with guiding on and off. All four exited 0 and wrote a frame. Each run's own report:

```
== gon
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
GUIDING True
== goff
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
GUIDING False
== con
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
GUIDING True
== coff
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
GUIDING False
```

No line in any of the four mentions guiding. Compared line by line with timestamps and figures taken out, **the two graphics runs are identical** — switching guiding on changed nothing printed. The two processor runs are not: with guiding off the samples were reported as one step, `cycles | Rendered 15 samples in N (N per sample), occupancy: 1`; with it on, as steps of four — `Rendered 4 samples …`, `Sample 5/16`, `Rendered 4 samples …`, `Sample 9/16`, `Rendered 4 samples …`, `Sample 13/16`, `Rendered 3 samples …`. So on the processor the setting changes how the render proceeds, and on the graphics a render asked for it carries on as though it were off, without a word. No earlier version's findings carry this run.

**Open Shading Language switched on with the graphics doing the work: the render finishes, on the graphics, and nothing is said** (entry 21) — the case 4.7 left open. The running Blender describes the setting as `shading_system`: `Open Shading Language | Use Open Shading Language`, off by default. With it switched on, two renders with `--log "*" --log-level info`, one on the graphics (Cycles pointed at Metal, the scene at GPU Compute) and one on the processor. Both exited 0 and wrote a frame, and their reports read:

```
== gon
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
OSL True
== con
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
OSL True
```

Neither printed a line containing `osl` or `shading language`. Set beside the matching runs with it off (entry 20's, same scene and set-up), timestamps and figures taken out: **on the graphics the two outputs are identical**, both carrying `cycles | Shader manager updated 8 shaders in …`. On the processor they differ — with it on the run printed `00:00.414  cycles           | Using shader search path: /Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/cycles/shader` and no `Shader manager updated` line. So the setting is taken up on the processor and, on the graphics, the render goes ahead as though it were off, with nothing said. No earlier version's findings carry this run.

**What this build carries, as it reports itself** (entry 29) — the list 4.8 points at. Read with `--factory-startup` and no window: `VERSION_STRING 5.3.0 Beta`, `BUILD_HASH b'5d8151335b6b'`, `BUILD_PLATFORM b'Darwin'`, `BUILD_DATE b'2026-10-16'`, `BACKGROUND True`, and forty-six build options, among them `'cycles': True`, `'cycles_osl': True`, `'image_cineon': True`, `'image_openexr': True`, `'image_openjpeg': True`, `'image_tiff': True`, `'image_webp': True`, `'codec_ffmpeg': True`, `'io_spz': True`, `'opentimelineio': True`, `'openmp': False`, `'sdl': False`, `'codec_avi': False`, `'jack': False`, `'pulseaudio': False`, `'coreaudio': True`, `'xr_openxr': True`. No option names Embree. So on this build Open Shading Language is present and every image format the manual calls optional is compiled in: "OSL support disabled in this build" and the "not compiled in" half of the `-F` complaint cannot be provoked here.

**"No compatible GPUs found for Cycles" cannot be made to appear on this Mac, and the preferences panel shows why** (entry 32) — the texts 4.2 quotes from Cycles' own file. Blender 5.3 was started with its window; a script printed the Cycles preference, opened Preferences on the System section, photographed that window three times two seconds apart, set the Cycles preference to `NONE` for the session, photographed it three more times, set it back and quit; each photograph was taken after forcing the Preferences window to redraw. The script printed `COMPUTE_DEVICE_TYPE METAL` and `DEVICES [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`. **As the preference stood**, the Cycles Render Devices panel showed two buttons, `None` and `Metal`, with `Metal` chosen; beneath them a list holding `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked; then `Kernel Optimization: Full` and `MetalRT: Auto`. **Set to `None`**, the panel showed the two buttons with `None` chosen and **nothing else at all** — no list, no message, the Network panel straight beneath. There is no CUDA, OptiX, HIP or oneAPI button on this Mac, so no backend here can be chosen and then find nothing, and the words never appear. The photographs lag a step behind the window: the first was blank grey, and the first taken after `None` was set still showed `Metal` chosen; the reading above is from the later ones. A tooltip for `Undo Memory Size` was showing over the lower part of the panel in several pictures; this sitting sent no clicks and no pointer movement to the window.

**A render that cannot write its output fails, with exit 1, and says why** (entry 25). With a plain file sitting where the output folder should be, `blender -b scene.blend -o …/fail/notadir/z_ -F PNG -f 1` exited **1**, left the error stream at 0 bytes, and printed on standard output:

```
00:00.414  image            | ERROR Couldn't create directory for file /private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/out/e25/fail/notadir/z_0001.png: File exists
00:00.414  reports          | ERROR Render error (File exists) cannot save: '/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/out/e25/fail/notadir/z_0001.png'
```

**An image format Blender does not know is complained about and ignored** (entry 26). `blender -b scene.blend -o out/f_ -F NOSUCHFORMAT -f 1`: exit **0**, a file `f_0001.png` written in the scene's own format, and on the error stream an empty line and then `Error: Format from '-F / --render-format' not known or not compiled in this release.` The one sentence covers a wrong name and a format missing from the build alike; every image format the manual calls optional is in this build (entry 29), so the second half of it could not be provoked.

**An argument Blender does not know is read as a file name** (entry 26) — not the immediate exit the manual's `-ba` example describes. Run with a fixed wait of eight seconds and then stopped if still going, in the same command:

- `blender -b scene.blend --not-a-real-option -o out/u_ -F PNG -f 1` exited **1** with nothing rendered. On the error stream: `unknown argument, loading as file: --not-a-real-option`. On standard output: `Read blend: "…/work/scene.blend"`, then `00:00.353  reports          | ERROR Cannot read file "/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/out/e26/--not-a-real-option": No such file or directory`, then the banner and `Blender quit`. The `-o`, `-F` and `-f` after it were never acted on.
- `blender -ba scene.blend` — the manual's own example — printed `unknown argument, loading as file: -ba` on the error stream, `ERROR Cannot read file "…/out/e26/-ba": No such file or directory` and then `Read blend: "…/work/scene.blend"` on standard output, and **was still running eight seconds later**; it was stopped then, ending with status 143. What it was doing in those seconds — whether it had opened a window — was not looked at.

No earlier version's findings carry the unknown-argument runs.

**A render stopped part-way leaves nothing at the output path and ends with status 143** (entry 27). The scene made slow — 100000 samples at 1200 by 900 — started in the background, left for six seconds, and sent the ordinary stop signal, all in one command, twice:

- **Adaptive sampling switched off**, so the render was really still going: `still rendering at six seconds: yes`, `exit after SIGTERM=143`, and `ls` found **no file** at the output path. The last lines printed were ordinary progress lines — `Fra: 1 | Mem: 1M | Sample 0/100000`, then `Fra: 1 | Remaining: 01:10:02.40 | Mem: 18M | Sample 1/100000` — and nothing announced the stop. The error stream was 0 bytes.
- **Adaptive sampling left on**, which is how an earlier set-up for this test was made: the render **finished by itself before the six seconds were up** — `still rendering at six seconds: no`, exit 0 — having stopped at `Sample 960/100000` and printed `Finished`. It left a complete PNG (first bytes `b'\x89PNG\r\n\x1a\n'`, ending in `IEND`, 607431 bytes) whose fields read `RenderTime = 00:04.26` and `cycles.ViewLayer.samples = 100000`. So that run was not stopped, and its image records the sample count the scene asked for, not the 960 it reached.

So a render stopped mid-way can be told from a finished one by the missing file and the status; a finished image cannot be told, from its fields, from one that reached fewer samples than it asked for. This covers one frame stopped by one kind of signal on this Mac.

---

## 5. Where can each of the things above be learned from — what a render prints while it works, what it leaves on disk, or what can be asked afterwards?

**Short answer.** On disk, the only standing record is the image's metadata (hostname, peak memory, render time, a free note), and only for file types that carry metadata, or as text burned into the picture. Printed output exists for statistics, logging and errors, but the sources do not say what it contains about devices, engine or add-ons. Asked afterwards, Blender can report its settings and preferences, the engine setting, the enabled add-ons and the available devices, but these describe the moment of asking.

**Label:** documented and observed as cited in questions 1–4; the arrangement below is mine.

**Parts of the computer (question 1).**
- Printed: `--cycles-print-stats` ("Log statistics about render memory and time usage"; to stderr per Cycles `engine.py` line 14); `--log` with `--log-level info` ("Information about devices, files, configuration, operations"); `--debug-cycles`. Contents not described (1.5). The `render_stats` handler shows what the printed progress line holds — API: "the render progress as a string containing current frame, current sample, render time and saving time." (https://docs.blender.org/api/5.3/bpy.app.handlers.html, read 2026-10-11) — no device named.
- On disk: hostname and peak memory in metadata (1.1); "Burn into Image — Add metadata as text to the render." (metadata page) for file types without metadata; the Render Time pass, CPU only (1.7); a log file if asked for — "--log-file <filepath> — Set a file to output the log to." (arguments page); `system-info.txt` if saved (1.6).
- Asked afterwards: scene device, Cycles preferences, threads (1.4); available devices via "Save System Info" (1.6); the display GPU via `gpu.platform` (1.8).

**Render engine (question 2).**
- Printed: not described in the sources.
- On disk: nothing in Blender's own metadata (2.1); engine-specific fields possible but not described (2.2); the engine setting inside a saved `.blend` (2.3).
- Asked afterwards or during: the scene's engine setting (2.3); handlers during the render (2.6); `-E help` for engines available (2.3).

**Add-ons (question 3).**
- Printed: add-on errors in the console (3.6).
- On disk: nothing from the render; `system-info.txt` if saved, with versions and paths (3.3).
- Asked afterwards: `Preferences.addons` (3.2).

**What happens when something cannot be provided (question 4).**
- Printed: errors and warnings go to the console and survive `--quiet` (4.11); named error texts "Error: Out of memory" and "CUDA Error: Kernel compilation failed" (4.4, 4.5); an exit code of 1 for a refused `--cycles-device` (4.3).
- On disk: a crash file after a crash (4.4).
- Asked afterwards: the Cycles device list with driver status (4.2), inside the preferences; `bpy.app.build_options` for what this build includes (4.8).

**Reading metadata back.** Inside the window: "Show Metadata — Displays metadata about the selected Render Result." (https://docs.blender.org/manual/en/5.3/editors/image/navigating.html, read 2026-10-11) and the Image Editor sidebar's "Metadata — Lists image metadata." (https://docs.blender.org/manual/en/5.3/editors/image/sidebar.html, read 2026-10-11). Outside the window: I found no Blender Python function that reads metadata from a saved image (searched the API for "metadata", "stamp", "header" in `Image`, `imbuf` and all pages). The 5.3 release notes say a Python module for OpenEXR now ships — "`OpenEXR` - see OpenEXR Python module documentation." (https://developer.blender.org/docs/release_notes/5.3/python_api/, read 2026-10-11) — but what it can read is described only on an outside page I may not open.

**Not covered.** The content of printed lines, in every case above.

**Where I looked.** As recorded under questions 1–4, plus the API pages `Image`, `RenderSlot`, `SpaceImageOverlay`, `imbuf`, `imbuf.types`.

**Outside the interface.** Everything under "Printed" and "On disk" is available outside the window. Under "Asked afterwards", the settings, preferences and add-on list can be read by a script; the Image Editor's metadata view and the preference panel texts are inside the window only. **Settled by the hand tests of 2026-10-17, on the Creator's Mac only — see below.** With no window: everything a plain render prints goes to standard output, the error stream staying empty (entry 1); `--cycles-print-stats` goes to standard output too, not to the error stream its help text names (entry 3); `--log-list-categories` and `--log-level` with a name both work (entries 4, 5); a written OpenEXR's fields can be read back through the `OpenEXR` module that ships with 5.3, a PNG's cannot, and no loaded image exposes its fields (entry 11). Inside the window there is no Metadata panel in the Image editor at all (entry 31); the only route the sources name to a saved image's fields there was not found.

**Operating system.** Where the console is differs: manual, startup troubleshooting, "If you launch Blender from an icon within your graphical desktop environment, you may not see the console window." (https://docs.blender.org/manual/en/5.3/troubleshooting/startup.html, read 2026-10-11), and "-con, --start-console — Start with the console window open (ignored if -b is set), (Windows only)." The crash file location differs (question 4). Otherwise the sources do not say. **The hand tests of 2026-10-17 were run on the Creator's Mac only.** The log-category list they record is this Mac's — 153 categories, with `gpu.metal` and without `gpu.opengl` (entry 4); a list from Linux or Windows was not taken in 5.3.

### Tested by hand on the Mac, 2026-10-17 — **LABEL: observed**

The machine, the Blender and the method are described under question 1. Nothing was run on Linux or Windows.

**A plain background render prints everything on standard output** (entry 1). With the two streams captured to separate files, the render with no switches left the error stream at **0 bytes**; every line, the `Engine:` line and the `Saved:` line included, was on standard output. The printed lines carry the frame, the running memory, the stage of the work, the sample count and the closing `Time:` line; none names a device. The full output is quoted under question 1.

**The device line is reached by a log switch, and lands on standard output** (entry 2). `--log-level info` alone, `--log "cycles" --log-level info`, `--log "*" --log-level info` and `--debug-cycles` each put one `Path tracing on:` line on standard output and none on the error stream, on the processor, on the graphics and on both. The whole report is quoted under question 1. `Added device …` is printed at the same level whichever device does the work, so it names what was found and not what ran.

**`--cycles-print-stats` prints to standard output, not to the error stream Cycles' own help text names** (entry 3). Three runs with the two streams in separate files — one frame on the processor, one frame with `--cycles-device METAL`, and frames 1 to 3 on the processor — each exited 0 with an error stream of **0 bytes**, and every statistics line was on standard output. Each frame prints two blocks: an `Update statistics:` block of per-stage times (`Scene:` with its `device_update` line, then `Geometry:`, `Light:`, `Object:`, `Image:`, `Background:`, `Bake:`, `Camera:`, `Film:`, `Integrator:`, `OSL:`, `Particles:` and more, each with a `Total time:`), and a `Render statistics:` block. The processor run's `Render statistics:` block, whole:

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
  Total render time               : Total 100.00% (0.11s), Self 16.07% (0.02s)
    Intersect Closest               : Total 30.36% (0.03s), Self 30.36% (0.03s)
    Ray setup                       : Total 25.00% (0.03s), Self 25.00% (0.03s)
    Shade Surface                   : Total 18.75% (0.02s), Self 0.00% (0.00s)
      Indirect Light                  : Total 7.14% (0.01s), Self 7.14% (0.01s)
      Direct Light                    : Total 4.46% (0.01s), Self 4.46% (0.01s)
      Setup                           : Total 3.57% (0.00s), Self 3.57% (0.00s)
      Shader Evaluation               : Total 3.57% (0.00s), Self 3.57% (0.00s)
      Render Passes                   : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Ambient Occlusion               : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Shade Light                     : Total 8.04% (0.01s), Self 0.00% (0.00s)
      Setup                           : Total 8.04% (0.01s), Self 8.04% (0.01s)
      Shader Evaluation               : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Intersect Shadow                : Total 1.79% (0.00s), Self 1.79% (0.00s)
    Intersect Subsurface            : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Intersect Volume Stack          : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Intersect Blocked Light         : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Shade Volume                    : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Setup                           : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Integrate                       : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Direct Light                    : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Indirect Light                  : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Shade Shadow                    : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Setup                           : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Surface                         : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Volume                          : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Blocked Light                   : Total 0.00% (0.00s), Self 0.00% (0.00s)
Shader statistics:
  Material                        : 0.02s (Relative cost: 1.00)
Object statistics:
  Cube                            : 0.02s (Relative cost: 1.00)
```

The graphics run's block is under question 1.

**`--log-list-categories` exists and lists 153 categories on this Mac** (entry 4). Run three times as `blender -b --log-list-categories`: exit 0 each time, the error stream 0 bytes, the three outputs identical to the byte, 154 lines of which the first is the version banner. The list opens `addon`, `anim`, `anim.action`, `anim.data`, `anim.driver` and ends `video.read`, `video.write`, `wm.xr`. Every category with `gpu` in its name, as printed:

```
compositor.gpu
gpu.framebuffer
gpu.metal
gpu.mipmap
gpu.platform
gpu.shader
gpu.shader.deps
gpu.texture
image.gpu
```

`addon`, `cycles`, `render`, `hydra.render` and `hydra.scene_index` are in the list. **There is no `gpu.opengl`** (a search for a line reading exactly that found nothing) and no `gpu.debug.metal`. The device line lives under `cycles` at `info` level — see question 1.

**A saved image's fields cannot be read through Blender's own image data — but in 5.3 an OpenEXR's can be read through the OpenEXR module that ships with it** (entry 11). First, the stamped Cycles PNG loaded as an image inside Blender with no window:

```
LOADED (160, 120)
HAS_metadata_attr False
stamp-ish attrs: []
```

The image loads and reports its size; nothing on it is shaped like a metadata attribute, and no attribute name contains *meta* or *stamp*.

Second, the module the 5.3 release notes name. `import OpenEXR` inside Blender 5.3 with no window printed `OPENEXR_IMPORTED /Applications/Blender 5.3b.app/Contents/Resources/5.3/python/lib/python3.13/site-packages/OpenEXR.cpython-313-darwin.so`, `OPENEXR_VERSION 3.4.10`, a list of names including `File`, `InputFile`, `Header` and `isOpenExrFile`, and `OPENEXR_DOC Read and write EXR high-dynamic range image files`. Nothing was guessed about how to use it: the module was asked to describe its own `File` class, and it answered with its own example — `f = OpenEXR.File("image.exr")` — and, for `File.header`, "The header metadata for the given part if specified, or for the first part if not", and for opening a file, `File(filename: str, separate_channels: bool = False, header_only: bool = False)`, where `header_only` means "If True, read only the header metadata, not the image pixel data." Run exactly so on the Cycles OpenEXR written with every metadata switch on — `f = OpenEXR.File(path, header_only=True)`, then every entry of `f.header()` printed — Blender exited 0 and printed:

```
HEADER Camera = 'Camera'
HEADER Date = '2026/10/17 23:23:33'
HEADER File = '/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/work/scene.blend'
HEADER Frame = '1'
HEADER FrameRange = '1:3'
HEADER Hostname = 'Artists-Studio-MacBook.local'
HEADER Lens = '50.00'
HEADER Marker = '<none>'
HEADER Memory = '2M'
HEADER Note = 'SMOOZY NOTE'
HEADER RenderTime = '00:00.01'
HEADER Scene = 'Scene'
HEADER Software = 'Blender 5.3.0 Beta'
HEADER Strip = '<none>'
HEADER Time = '00:00:00:01'
HEADER channels = [Channel("A", xSampling=1, ySampling=1), Channel("B", xSampling=1, ySampling=1), Channel("G", xSampling=1, ySampling=1), Channel("R", xSampling=1, ySampling=1)]
HEADER colorInteropID = 'lin_rec709_scene'
HEADER compression = <Compression.ZIP_COMPRESSION: 3>
HEADER cycles.ViewLayer.render_time = '00:00.01'
HEADER cycles.ViewLayer.samples = '16'
HEADER cycles.ViewLayer.synchronization_time = '00:00.00'
HEADER cycles.ViewLayer.total_time = '00:00.01'
HEADER dataWindow = (array([0, 0], dtype=int32), array([159, 119], dtype=int32))
HEADER displayWindow = (array([0, 0], dtype=int32), array([159, 119], dtype=int32))
HEADER lineOrder = <LineOrder.INCREASING_Y: 0>
HEADER name = ''
HEADER pixelAspectRatio = 1.0
HEADER screenWindowCenter = array([0., 0.], dtype=float32)
HEADER screenWindowWidth = 1.0
HEADER type = <Storage.scanlineimage: 0>
HEADER xDensity = 72.0
```

So, from Python inside Blender 5.3 with no window, every field of a written OpenEXR can be read — the stamp fields, Cycles' four fields and `Software` among them — and they match what reading the file's bytes directly gave (question 2). **This reaches OpenEXR files only**; the module's own description is of "EXR high-dynamic range image files", and nothing here reads a PNG's fields. No earlier version's findings record this module.

**Which stream the failures land on** (entries 13, 15, 25, 26). On standard output: the `--cycles-device` refusals, the two lines of a render that cannot save, and the `ERROR Engine '…' not available` line. On the error stream: `Error: engine not found 'NOT_AN_ENGINE'`, the `-F` complaint, and `unknown argument, loading as file: …`. All are quoted under question 4.

**There is no Metadata panel in the Image editor to read a saved image's fields from** (entry 31) — the route this answer names from the manual ("Metadata — Lists image metadata.") was not found in the running program. Blender 5.3 was started with its window; a script loaded the stamped Cycles PNG from entry 9, turned the 3D view into an Image editor showing it, opened the sidebar, asked the running program which sidebar panels the Image editor defines, forced a redraw and took Blender's own screenshot. It printed:

```
SIDEBAR TABS ['Image', 'Mask', 'Scopes', 'Tool', 'View']
PANELS IN Image TAB ['Image', 'Render Slots', 'UDIM Tiles']
ANY PANEL LABELLED METADATA ['ASSETBROWSER_PT_metadata', 'RENDER_PT_stamp']
```

— so in the whole program the only panels whose label contains *metadata* belong to the Asset Browser and to Output Properties (entry 30). The picture shows the Image editor with `cyc_0001.png` loaded and the sidebar open on its `Tool` tab, with `Tool`, `Image`, `View` and `Scopes` tabs down its edge and no Metadata tab. The sidebar tab could not be switched from the script, so the panel list stands in for photographing each tab. **The picture also shows text drawn into the image itself**, because the switches turned on in entry 9 include `use_stamp` (the panel's *Burn Into Image*): it reads, partly overlapping, `File /priv…`, `Date 2026/10/17 23:23:3…`, `RenderTime 00:00.02`, `Peak Memory 2M`, `Hostname Artists-Studio…`, `SMOOZY NOTE`, `Marker <none…` and `Strip <none>`, `Time`, `Scene` — the same fields as the file's text fields, and nothing naming a device or an engine.

**A format that cannot hold metadata says nothing about dropping it** (entry 10) — TIFF and TGA with every switch on, exit 0, both files written, nothing in either stream; under question 4.

**What a finished image holds on disk** (entry 9). With every metadata switch on: the fields the Metadata panel lists, four `cycles.ViewLayer.*` fields in any Cycles image, and in an OpenEXR a `Software` attribute naming Blender and its version, a `colorInteropID` attribute, and `name` and `type` attributes — all quoted under questions 1 and 2. Nothing in either format names a device, the graphics or an add-on.

**Asked afterwards, with no window, the `gpu` module names the machine's graphics once `gpu.init()` is called** (entry 28): `GPU_RENDERER Metal API`, `GPU_DEVICE_TYPE APPLE`, `GPU_BACKEND METAL`, `GPU_VENDOR Apple M1 Pro`. Quoted under question 1.

**For EEVEE and Workbench, nothing printed names the graphics chip** (entry 14) — at information level not even the backend; at debug level `gpu.metal` lines name Metal and nothing more. Quoted under question 1.

**`--log-level` takes a name** (entry 5). `blender -b scene.blend --log "*" --log-level info -o out/y_ -F PNG -f 1`: exit 0, a frame written, 143 lines on standard output and the error stream 0 bytes — nothing refused.

**Each frame of a background animation is announced on a line of its own** (entry 6). Frames 1 to 3, run as `blender -b scene.blend -o out/an_ -F PNG -s 1 -e 3 -a`: exit 0, three frames written, error stream 0 bytes. The lines that carry it, by line number in standard output:

```
3:00:00.321  render           | Rendering frame 1
6:00:00.325  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
50:00:00.401  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/out/e6/an_0001.png'
52:00:00.401  render           | Rendering frame 2
55:00:00.402  render           | Fra: 2 | Mem: 0M | Synchronizing object | Cube
97:00:00.419  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/out/e6/an_0002.png'
99:00:00.419  render           | Rendering frame 3
103:00:00.419  render           | Fra: 3 | Mem: 0M | Synchronizing object | Cube
144:00:00.436  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/out/e6/an_0003.png'
```

The frame number is also in the `Fra:` field of every progress line.

**The render handlers fire with no window, in one order, with the frame on disk at `render_write` — and `render_stats` is handed the progress strings, not frame or memory figures** (entry 7). A script attached a recorder to `render_init`, `render_pre`, `render_post`, `render_write`, `render_complete`, `render_cancel` and `render_stats`; the one at `render_write` also asked whether the output file existed yet, the output folder being new and empty. Exit 0, error stream 0 bytes:

```
HANDLER render_init      t=0.000 args=['Scene', 'NoneType']
HANDLER render_pre       t=0.000 args=['Scene', 'NoneType']
HANDLER render_post      t=0.091 args=['Scene', 'NoneType']
HANDLER render_write     t=0.091 args=['Scene', 'NoneType'] | file exists now: True
HANDLER render_complete  t=0.091 args=['Scene', 'NoneType']
```

All six handlers exist; `render_cancel` did not fire. Each is handed two things — the scene and a second value that is empty — where the documentation describes one. `render_stats` fired **44** times; its first and last:

```
HANDLER render_stats     args=('Mem: 0M | Synchronizing object | Cube', None)
HANDLER render_stats     args=('Time: 00:00.09 (Saving: 00:00.06)', None)
```

The strings carry no frame number and no peak memory; the frame number sits in the printed line's own `Fra:` prefix, which the handler never sees.

**A statistics handler written to take one value works too.** The 5.3 audit's command, run as written with `-o` added: `blender -b scene.blend --python-expr "import bpy; bpy.app.handlers.render_stats.append(lambda s: print('STATS:', s))" -o out/lam_ -F PNG -f 1`. Exit 0, a frame written, error stream 0 bytes, no error or traceback anywhere, and 44 lines printed, the first three and the last:

```
STATS: Mem: 0M | Synchronizing object | Cube
STATS: Mem: 0M | Synchronizing object | Light
STATS: Mem: 0M | Initializing
STATS: Time: 00:00.08 (Saving: 00:00.06)
```

So a handler taking one value is handed the string alone, and one taking any number is handed the string and an empty second value. No earlier version's findings carry a one-value handler.

**With a window open, the statistics handler is handed the same 44 strings** (entry 34). A script recorded every handler call while the render operator, `bpy.ops.render.render(write_still=True)`, rendered the shared scene — once from a timer in a Blender started with its window, and once called directly in a Blender with no window. Both handed `render_stats` **44** strings, the same ones in the same order: set side by side with the times taken out, the only difference was one figure inside a string, `Remaining: 00:00.06` against `Remaining: 00:00.01`. With the window, the first and last of what was recorded:

```
render_init t=4.002
render_pre t=4.002
render_stats t=4.025 ('Mem: 1M | Synchronizing object | Cube', None)
render_stats t=4.025 ('Mem: 1M | Synchronizing object | Light', None)
...
render_stats t=4.055 ('Mem: 2M | Finished', None)
render_stats t=4.138 ('Time: 00:00.13 (Saving: 00:00.08)', None)
render_post t=4.138
render_write t=4.138
render_complete t=4.138
```

So a script that follows a render's progress through this handler gets the whole stream in this version whether or not a window is open.

---

## 6. At what point can each of the things above be learned — before a render starts, while it runs, or only once it has finished?

**Short answer.** Before: what is available and what is asked for — devices, engines, add-ons, build features. During: printed statistics, log lines and errors, and anything a handler script reads. After: the metadata in the saved file, and the settings, which still show only what was asked for. One thing is fixed at the very start: a refused `--cycles-device` stops Blender before any render.

**Label:** documented and observed as cited; timing steps inferred where marked.

**Before the render starts.**
- Available Cycles devices: from the Cycles preferences, which query the system when asked. Cycles `properties.py`, lines 1819–1820: `# Refresh device list. This does not happen automatically on Blender` `# startup due to unstable drivers that can cause crashes.` and lines 1762–1763: `# Be careful when deciding when to call this function,` `# as Blender can crash with `_cycles.available_devices()` on some drivers.` (observed, read 2026-10-16). The step (inferred): the stored device list may be out of date until something asks for the devices again.
- Available engines: `-E help` (documented, 2.3).
- Enabled add-ons: `Preferences.addons` and "Save System Info" (3.2, 3.3).
- Build features: `bpy.app.build_options` (4.8).
- The `--cycles-device` check: at add-on registration (4.3, observed); stops Blender with exit code 1 if refused.
- The `render_init` and `render_pre` handlers run at the start: "on initialization of a render job", "on render (before)" (https://docs.blender.org/api/5.3/bpy.app.handlers.html, read 2026-10-11).

**While it runs.**
- Printed: progress statistics ("on printing render statistics … current frame, current sample, render time and saving time" — `render_stats` handler, same API page); log lines; errors and warnings.
- When `--cycles-print-stats` prints — during or at the end — is not in the sources.
- Optimized-kernel requests happen "at the start of a render" (4.6, observed).
- Out-of-memory spill to system memory happens during the render (4.4; timing inferred from "if the GPU memory is full").

**Once it has finished.**
- Metadata in the saved file (1.1). The `render_write` handler runs "on writing a render frame (directly after the frame is written)" (same API page).
- `render_post`, `render_complete`, `render_cancel` handlers: "on render (after)", "on completion of render job", "on canceling a render job".
- `RenderEngine.error_set`: "Set error message displaying after the render is finished" (4.10).
- A crash file after a crash (4.4).
- Settings and preferences can be read at any point, but they show what was asked for (1.4, 2.3).

**Not covered.** When log lines about devices appear, if they appear at all. Whether `--cycles-print-stats` prints per frame or once.

**Where I looked.** As recorded under questions 1–5.

**Outside the interface.** All the "before" items except the preference panels can be reached by a script or the command line. Handlers are set by scripts and are not tied to the window. Seeing metadata after the fact is inside the window only (question 5). **Settled by the hand tests of 2026-10-17, on the Creator's Mac only — see below.** With no window the render handlers fire in order with the frame on disk at `render_write` (entry 7), `render_stats` hands over the same 44 strings with a window as without (entry 34), `bpy.app.is_job_running` answers (entry 8), each frame of an animation is announced as it starts (entry 6), and the device line arrives only once a frame's sampling is done (entry 2). Seeing a written OpenEXR's fields afterwards is not inside the window only in 5.3: its `OpenEXR` module reads them (entry 11).

**Operating system.** The sources do not say timing differs by operating system. **The hand tests of 2026-10-17 were run on the Creator's Mac only**; Linux and Windows were not tried in 5.3.

### Tested by hand on the Mac, 2026-10-17 — **LABEL: observed**

The machine, the Blender and the method are described under question 1. Nothing was run on Linux or Windows.

**The engine is printed before the render starts** (entry 1). In the background render with no switches, `render | Engine: Cycles` came at `00:00.424`, before the first `Synchronizing object` line and before `Sample 0/16` at `00:00.466`. No line at default settings names a device at any point in the run.

**The line that names the device arrives only once the sampling is done** (entry 2). In the `--log-level info` processor run, by line number in the captured output: `Sample 16/16` at line 105, `Full path tracing report:` at 108, `Path tracing on:` at 110, `Saved:` at 140. In the graphics run: `Sample 16/16` at 107 (`00:01.014`), the report at 110 (`00:01.014`), `Saved:` at 142 (`00:01.105`). So it is a closing report, printed after the last sample and before the frame is written. What a render stopped before its last sample prints was not tested here.

**`--cycles-print-stats` prints once per frame, not once per run** (entry 3). Frames 1 to 3 of the shared scene, run as `blender -b scene.blend -o out/a_ -F PNG -s 1 -e 3 -a -- --cycles-print-stats`: exit 0, three frames written, and three of each block. Where they fall in the captured standard output, by line number — first `grep -n -E "Render statistics:|Rendering frame|Saved:|Finished" a_out.txt`, the path shortened here to `…`:

```
a_out.txt:3:00:00.437  render           | Rendering frame 1
a_out.txt:82:00:00.645  render           | Fra: 1 | Mem: 2M | Finished
a_out.txt:83:00:00.718  render           | Saved: '…/out/e3/a_0001.png'
a_out.txt:85:Render statistics:
a_out.txt:127:00:00.718  render           | Rendering frame 2
a_out.txt:204:00:00.750  render           | Fra: 2 | Mem: 2M | Finished
a_out.txt:205:00:00.752  render           | Saved: '…/out/e3/a_0002.png'
a_out.txt:207:Render statistics:
a_out.txt:249:00:00.752  render           | Rendering frame 3
a_out.txt:326:00:00.776  render           | Fra: 3 | Mem: 2M | Finished
a_out.txt:327:00:00.779  render           | Saved: '…/out/e3/a_0003.png'
a_out.txt:329:Render statistics:
```

and then `grep -n "Update statistics:" a_out.txt`:

```
15:Update statistics:
138:Update statistics:
260:Update statistics:
```

So each frame's update times come early in that frame and its render statistics come at its end, before the next frame is announced. The statistics lines carry no timestamp of their own, and the version banner shows that two writers can reach the same file out of order, so where exactly a block falls against the `Saved:` line beside it is not established — only that there is one of each per frame. No earlier version's findings carry a run of this kind.

**The frame in hand is known while the run is going** (entry 6). In a background animation each frame is announced by `render | Rendering frame N` before its first progress line and before its `Saved:` line — frame 1 at line 3, its `Saved:` at line 50, frame 2 announced at line 52. The lines are quoted under question 5.

**A refused `--cycles-device` stops the run at two different points, depending on why it is refused** (entry 15). A name Cycles does not know (`NOTADEVICE`, `BANANA`) is refused as the very first printed line, before the file is read: no `Read blend:` line appears at all. A real backend with no hardware behind it (`OPTIX`, `CUDA`, `HIP`, `ONEAPI`) is refused only after the file is read and the render has begun — after `Engine: Cycles` and `Fra: 1 | Mem: 0M | Initializing` — and before the first sample. Both end with exit 1 and no frame. The outputs are quoted under question 4.

**Before a render starts, the Cycles preference already reads Metal on this Mac, but the device list is empty until something asks for it** (entry 18) — this is the stored list 6 says "may be out of date". At factory startup with no window: `NO_REFRESH COMPUTE_DEVICE_TYPE METAL DEVICES []`; then, after `refresh_devices()` in the same session, `AFTER_REFRESH COMPUTE_DEVICE_TYPE METAL DEVICES [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`.

**And whether a scene set to GPU Compute reaches the graphics depends on the settings folder, which the scene file does not say** (entry 18). One settings folder had never had preferences saved into it. Into a second, made for the purpose, preferences were saved once — by a script that first asked Blender which folder it was using and saved only when the answer was that folder; it printed `CONFIG_IN_USE …/settings2/5.3/config` and `SAVED METAL [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`, and the folder then held `config/userpref.blend`. The same scene, set to GPU Compute by a script on the command line, rendered once against each folder with `--log "*" --log-level info`; both exited 0 and wrote a frame. Against the never-saved folder:

```
00:00.332  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
```

Against the folder with preferences saved:

```
00:00.340  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
```

Same scene, same command, same Blender: the processor in one, the graphics in the other, and in the first Cycles printed that it had found the graphics. Whether the saved file or the `refresh_devices()` call before saving makes the difference is not separated by these two runs. `-- --cycles-device METAL` reached the graphics from the never-saved folder (entry 2).

**Nothing on disk afterwards tells which part did the work — the Render Time pass included** (entry 19). The pass is written whenever the scene says CPU and withheld whenever it says GPU Compute, whatever actually rendered; the table is under question 1.

**A setting read after an override need not be the one in force** (entry 16). With the scene set to GPU Compute and the command line overriding to the processor, the run read back `compute_device_type= METAL  scene.cycles.device= GPU` while its own device line named the processor. Only the printed line told them apart; quoted under question 1.

**Before anything is rendered, the settings say nothing will be stamped** (entry 12). At factory startup the master metadata switch `use_stamp` is False and `use_stamp_memory` and `use_stamp_hostname` are both False; the full list is under question 1.

**The add-ons can be read before, during or after, but they describe the moment they are read** (entries 22 to 24). `bpy.ops.wm.sysinfo` lists what is on at the moment it writes — an add-on switched off mid-session is gone from the next file — and the list a script reads during a run started with `--addons` does not include what that flag loaded. Quoted under question 3.

**Asking whether a render is under way works with no window** (entry 8). `blender -b --python-expr "import bpy; print('IS_JOB_RUNNING', bpy.app.is_job_running('RENDER'))"` (with the call wrapped to print any error): Blender exited 0 and printed `IS_JOB_RUNNING False`.

**Once an OpenEXR is written, its fields can be read back from Blender 5.3's own Python** (entry 11), through the `OpenEXR` module that ships with it — quoted under question 5. A PNG's fields cannot be read that way, and no image loaded into Blender exposes its fields.

**While it runs, the progress strings reach a script with or without a window** (entry 34): 44 `render_stats` strings in both cases for the same render, quoted under question 5.

**The handlers mark the stages, and the frame is on disk when `render_write` fires** (entry 7). With no window: `render_init` and `render_pre` at the start, `render_post` when the render is done, `render_write` with the file already on disk (`file exists now: True`), then `render_complete`; `render_stats` 44 times in between. The output is quoted under question 5.

---

## Things a person at a computer could settle in a few minutes

The ten items that stood here were run on 2026-10-17 on the Creator's Mac, and what they found is written into the answers above as observed findings: item 1 as entries 2 and 3, item 2 as entry 4, item 3 as entry 17, item 4 as entry 15, item 5 as entries 9 and 31, item 6 as entry 22, item 7 as entry 13, item 8 as entries 1 and 7, item 9 as entry 20, and item 10 as entry 24 — the entry numbers of `Audits/Blender Research/What actually ran/Hand tests — 5.3.md`, which holds every command and raw output.

**Still open, and why.**

1. **Anything on Linux or Windows** — item 1 on an NVIDIA, AMD or Intel graphics card, item 6 written to a Windows path, and every Linux- or Windows-only case under question 4. There was no Linux machine, no Windows machine and no such card for this sitting. A Windows answer would have to name its edition; see the equipment section of `Preparation/Blender Research/Method.md`.
2. **What a render does when the graphics memory runs out, and whether it says so** (4.4), on any backend. On this Mac the graphics share the machine's 16 GB, so filling one fills the other.
3. **A graphics device below its driver requirement** (4.2), and the " (Disabled)" and "Requires … driver" texts. Needs a card whose driver is too old.
4. **"No compatible GPUs found for Cycles"** (4.2). Cannot be reached on this Mac: Metal is the only backend offered and it finds the graphics (entry 32).
5. **Metal programs not yet prepared** (4.6). The progress lines' `(N kernels to optimize)` was seen (entries 2, 15); what a render prints with none of them ready was not, since that needs Blender's store of compiled programs emptied outside the work folder.
6. **"OSL support disabled in this build", and an image format "not compiled in this release"** (4.8). This build carries both (entry 29), so neither can be provoked.
7. **Where a crash report lands on macOS.** Needs Blender made to crash, which writes outside the work folder.
8. **Whether Workbench renders on the graphics.** Entry 14 shows only that nothing printed names the chip; the documentation question went to research.

---

2026-10-17 — Claude Opus 5.5 (Claude Code) — What actually ran, Blender 5.3

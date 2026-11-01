Sources consulted:
- Blender 5.3 Manual (snapshot 2026-10-11)
- Blender 5.3 Python API Documentation (snapshot 2026-10-11)
- Blender 5.3 Release Notes (snapshot 2026-10-11)
- Cycles 5.3 source files (snapshot 2026-10-16)
- Question sheet: /Users/artist/Desktop/Work/Smoozy v2/Preparation/Blender Research/Questions/What actually ran.md (read on 2026-10-17)

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

# What actually ran — Second Pass

## 1. When a render has finished, what can be learned about which parts of the computer it used?

**LABEL**: documented (for memory and hostname metadata); inferred (for CPU deduction via the Render Time pass); not in the sources (for post-render GPU/CPU device identification in standard image metadata or RenderResult API)

**Not covered**: Does not cover external operating system process profiling tools, third-party render engines, or custom add-ons that may implement their own execution logging.

### Memory usage (RAM)

1. **Peak memory in image metadata and render stamp**:
   Blender can record the peak memory usage reached during rendering into the image metadata, and optionally burn it directly into the rendered image pixels.
   - Exact words:
     ```
     Memory

     Includes the peak memory usage.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html`
     Date read: 2026-10-11
   - Exact words:
     ```
     use_stamp_memory

     Include the peak memory usage in image metadata (default True)

     Type:

     bool
     ```
     Source: Python API Documentation, `https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html`
     Date read: 2026-10-11
   - Exact words:
     ```
     Burn into Image

     Add metadata as text to the render.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html`
     Date read: 2026-10-11

2. **Render engine memory statistics API**:
   Render engines provide memory updates through the API:
   - Exact words:
     ```
     update_memory_stats(*, memory_used=0.0, memory_peak=0.0)

     Update memory usage statistics

     Parameters:

     memory_used (float) – Current memory usage in megabytes (in [0, inf], optional)

     memory_peak (float) – Peak memory usage in megabytes (in [0, inf], optional)
     ```
     Source: Python API Documentation, `https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html`
     Date read: 2026-10-11

3. **Status Bar memory readout**:
   In the user interface, Blender displays memory usage in the status bar:
   - Exact words:
     ```
     System Memory

     Shows an estimate of Blender’s RAM consumption. On a single-instance single-machine scenario,
     this estimate provides a measurement against the hardware limit of the machine.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html`
     Date read: 2026-10-11

4. **Command-line render memory statistics**:
   When rendering with Cycles from the command line, the `--cycles-print-stats` flag outputs render memory statistics to stderr:
   - Exact words:
     ```
     --cycles-print-stats

     Log statistics about render memory and time usage.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html`
     Date read: 2026-10-11
   - Exact lines:
     ```python
         parser.add_argument("--cycles-print-stats",
                             help="Print rendering statistics to stderr",
                             action='store_true')
     ```
     Source: Cycles source file `Blender Sources/cycles_5.3/engine.py`, lines 13-15
     Date read: 2026-10-16

### Machine identity (Hostname)

1. **Host machine name**:
   Blender can record the network hostname of the computer that performed the render into image metadata and stamp it onto the frame.
   - Exact words:
     ```
     Hostname

     Includes the rendering machine’s hostname.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html`
     Date read: 2026-10-11
   - Exact words:
     ```
     use_stamp_hostname

     Include the hostname of the machine that rendered the frame (default False)

     Type:

     bool
     ```
     Source: Python API Documentation, `https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html`
     Date read: 2026-10-11

2. **Custom farm identifier / note**:
   Arbitrary machine or node identification can be stored via the Note field:
   - Exact words:
     ```
     It can be useful to use the Note field if you are setting up a render farm.
     Since you can script any information you like into it,
     such as an identifier for the render node or the job number.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html`
     Date read: 2026-10-11

### Execution device (CPU vs. GPU / Specific Compute Devices)

1. **Standard image metadata does not record compute devices**:
   The metadata options documented for the Metadata panel (`Date`, `Time`, `Render Time`, `Frame`, `Frame Range`, `Memory`, `Hostname`, `Camera`, `Lens`, `Scene`, `Marker`, `Filename`, `Strip Name`, `Note`) contain no field for recording whether the render was executed on CPU, CUDA, OptiX, HIP, oneAPI, or Metal, nor do they record GPU model names.
   - Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html`
     Date read: 2026-10-11

2. **Post-render API data-blocks do not expose execution device**:
   In the Python API, `bpy.types.RenderResult` contains only layer collections, pixel resolution, view collections, a file loading function, and a method to add engine-specific stamp fields:
   - Exact words:
     ```
     class bpy.types.RenderResult(bpy_struct)

     Result of rendering, including all layers and passes

     layers
     resolution_x
     resolution_y
     views
     load_from_file(filepath)
     stamp_data_add_field(field, value)
     ```
     Source: Python API Documentation, `https://docs.blender.org/api/5.3/bpy.types.RenderResult.html`
     Date read: 2026-10-11
   There is no property on `RenderResult` or `Image` recording which device actually executed the render. Pre-render settings (`scene.cycles.device` and user preferences) record what was *configured* prior to render, but no post-render execution audit log of the active device is saved on the data-block.

3. **Inferring CPU execution via the Render Time pass in Cycles**:
   Cycles provides a debug pass named `Render Time`. The manual states:
   - Exact words:
     ```
     Render Time Cycles

     An estimate for how long in milliseconds each pixel took to render.
     Note, this pass is not supported on GPU rendering.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/layers/passes.html`
     Date read: 2026-10-11
   In Cycles' own source code, the pass is only yielded when rendering on CPU:
   - Exact lines:
     ```python
         if crl.pass_render_time:
             # Only yield the pass if rendering on CPU
             if scene.cycles.device == 'CPU':
                 yield (n_("Render Time"), "X", "VALUE")
     ```
     Source: Cycles source file `Blender Sources/cycles_5.3/engine.py`, lines 229-232
     Date read: 2026-10-16
   - **Inference step**: If a completed render output contains a populated `Render Time` pass, it follows that the scene setting was configured for CPU (`scene.cycles.device == 'CPU'`). The source code checks the scene data-block property rather than the active execution device, so it does not establish that the render actually ran on CPU if overridden by `--cycles-device`. *(Corrected by citation check: narrowed claim from proving CPU execution to proving scene setting, as the check in `engine.py` inspects `scene.cycles.device` and Hand Test 19 observed the pass written under GPU override).*

4. **Inferring CPU execution via Path Guiding**:
   Path guiding in Cycles is only functional on CPU:
   - Exact words:
     ```
     Path guiding is only available when rendering on a CPU.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html`
     Date read: 2026-10-11
   - Exact words:
     ```
     Path Guiding is not supported on any GPU.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html`
     Date read: 2026-10-11

5. **Python application handler `render_stats`**:
   The handler called upon printing render statistics does not report compute devices:
   - Exact words:
     ```
     bpy.app.handlers.render_stats

     on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time.
     ```
     Source: Python API Documentation, `https://docs.blender.org/api/5.3/bpy.app.handlers.html`
     Date read: 2026-10-11

### Record of where I looked

- **Searches run**:
  - `grep -rn "use_stamp_"` in `Blender Sources/api_5.3`: located all stamp attributes on `RenderSettings`.
  - `grep -rn "peak memory"` in `Blender Sources/manual_5.3`: found references in `metadata.txt` and `usd.txt`.
  - `grep -rin "render time"` in `Blender Sources/manual_5.3`: inspected all occurrences across manual; identified `render/layers/passes.txt` mentioning `Render Time Cycles`.
  - `grep -rn "RenderResult"` in `Blender Sources/api_5.3`: checked all properties and methods on `RenderResult`.
  - `grep -rn "stamp_data_add_field"` in `Blender Sources`: verified where engine stamp fields are defined and used.
  - `grep -rn "render_stats"` and `grep -rn "render_complete"` in `Blender Sources/api_5.3`: verified handler signatures.
  - `grep -rn "print_stats"` and `grep -rn "stats"` in `Blender Sources/cycles_5.3`: located `--cycles-print-stats` argument parsing.
- **Pages opened**:
  - `manual_5.3/render/output/properties/metadata.txt`: reviewed complete list of metadata include fields.
  - `manual_5.3/render/layers/passes.txt`: reviewed debug passes including `Render Time`.
  - `manual_5.3/editors/image/sidebar.txt`, `image_settings.txt`, `introduction.txt`, `overlays.txt`, `navigating.txt`: inspected Image Editor sidebar, metadata display overlay, and Render Result display options.
  - `manual_5.3/interface/window_system/status_bar.txt`: checked status bar resource and memory readouts.
  - `manual_5.3/files/media/image_formats.txt`: checked image format metadata limitations.
  - `manual_5.3/advanced/command_line/arguments.txt`: checked command line flags for logging and stats.
  - `api_5.3/bpy.types.RenderSettings.txt`: verified RNA properties for stamp fields.
  - `api_5.3/bpy.types.RenderResult.txt`: examined full class structure of `RenderResult`.
  - `api_5.3/bpy.types.RenderEngine.txt`: checked engine update methods (`update_stats`, `update_memory_stats`).
  - `api_5.3/bpy.types.Image.txt` and `bpy.types.RenderSlot.txt`: checked image data-block properties.
  - `cycles_5.3/engine.py`: checked argument parser, pass registrations, and device queries.
- **Ruled out**:
  - Ruled out standard metadata saving device/GPU model: `metadata.txt` and `bpy.types.RenderSettings` enumerate all metadata fields; no device or GPU field exists.
  - Ruled out post-render data-block queries for execution hardware: examined `RenderResult` and `Image`; ruled out because neither RNA structure exposes execution hardware properties.
  - Ruled out `bpy.app.handlers.render_stats` reporting hardware: ruled out because its string argument is documented strictly as frame, sample, render time, and saving time.

### Outside the interface
Holds outside the interface. Peak memory usage and machine hostname are written into image metadata and burned into image files when configured, whether rendered via GUI or command line (`blender -b`). Command line execution with `--cycles-print-stats` outputs render memory and timing statistics to stderr. Python handlers (`render_stats`, `render_complete`) function in background mode. Status Bar and Image Editor overlays are interface-only. **Tested by hand on 2026-10-17, on the Creator's Mac only — see below.** With no window, the peak memory, hostname and Cycles' own fields are written into the image (entry 9); `--cycles-print-stats` writes to standard output, not standard error (entry 3); the device that path-traced is named by `Path tracing on:` at `--log-level info` and by nothing in the image (entries 2, 9); "Save System Info" works and names the graphics in its Cycles section, with its GPU section empty until a window is open (entries 22, 33). The render handlers do fire with no window: in a background render `render_init`, `render_pre`, `render_post`, `render_write` (with the frame already on disk) and `render_complete` fired in that order, and `render_stats` 44 times, each handed two values, the progress string and an empty second one (entry 7). A finished render shown in Blender's own window carries frame, time and memory and no device (entry 35).

### Operating system
Holds across all supported operating systems (Windows, Linux, macOS). Hostname reporting relies on standard OS hostname resolution. **The hand tests of 2026-10-17 were run on the Creator's Mac only** (macOS 26.6.2, Apple M1 Pro, Metal); what they found is a macOS answer, and Windows and Linux were not tried in 5.3, there being no such machine for this sitting.

### Tested by hand on the Mac, 2026-10-17 — **LABEL: observed**

**The machine.** The Creator's own Mac — a MacBook Pro with an **Apple M1 Pro** chip, its own 14-core graphics reached through **Metal**, 16 GB of memory, **macOS 26.6.2** (build 25G83). Blender was `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, which reported itself as `Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)`. Every run was pointed at a settings folder made for it before it was used — `BLENDER_USER_RESOURCES` set to a folder inside the work folder, which Blender reported back as the one in use — and never at the Creator's own; Blender's temporary folder was pointed at the work folder too. **The scene** is the factory cube, camera and light with these changes: Cycles, 16 samples, denoising off, 160 by 120 pixels, frames 1 to 3, PNG, and its own output path set to a folder beside the file. It was built by Blender 5.2.2 so that 5.3 opens it as an older file. **Two limits.** The Creator's own Blender was open throughout and other work was running on the Mac, so nothing below is offered as a timing or as a memory measurement. And **nothing was run on Linux or Windows**, so every result below covers macOS on Apple's own graphics only. Each result names its entry in `Audits/Blender Research/What actually ran/Hand tests — 5.3.md`, which holds every command and script whole.

**A background render with no switches names no device** (entry 1). Run as `blender -b scene.blend -o out/p_ -F PNG -f 1`, with the two streams written to separate files: exit 0, a frame written, the error stream **0 bytes**, and no line naming a processor, the graphics, Metal, CUDA, OptiX, HIP or oneAPI. The lines that carry the answer, from standard output:

```
00:00.424  render           | Start rendering: Scene, ViewLayer
00:00.424  render           | Engine: Cycles
00:00.458  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
00:00.462  render           | Fra: 1 | Mem: 1M | Updating Objects | Copying Transformations to device
00:00.466  render           | Fra: 1 | Mem: 1M | Updating Device | Writing constant memory
00:00.466  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.482  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.482  render           | Fra: 1 | Mem: 2M | Finished
00:00.556  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/What actually ran/out/e1/p_0001.png'
00:00.556  render           | Time: 00:00.13 (Saving: 00:00.07)
```

The lines with *device* in them carry the word without naming one.

**`--log-level info` on its own makes Blender name the device that path-traced, on standard output** (entry 2). The same scene run eight ways — `--log "*" --log-level info`; `--log "cycles" --log-level info`; `--log-level info`; `--debug-cycles`; `--log-level info` with `-- --cycles-device METAL`; `--log-level info` with `-- --cycles-device METAL+CPU`; and `--log "render,cycles" --log-level info -b scene.blend -f 1 -- --cycles-print-stats`, alone and with `--cycles-device CPU` added after the `--`. Every run exited 0 and wrote its frame; every error stream was 0 bytes; every run put exactly one `Path tracing on:` line on standard output and none on the error stream. On the processor:

```
00:00.352  cycles           | Full path tracing report:
                            | 
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
                            |                    Hardware Ray-Tracing: Off
```

On the graphics (`-- --cycles-device METAL`):

```
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                            |                    Hardware Ray-Tracing: Off
```

Asked for both (`-- --cycles-device METAL+CPU`), the report named both, the processor with one thread fewer:

```
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                            |                    Hardware Ray-Tracing: Off
                            |                  Apple M1 Pro (CPU) (7 threads) [CPU]
                            |                    Hardware Ray-Tracing: Off
```

after printing `CPU render threads reduced from 8 to 7, to dedicate to GPU.` three times before the render began. The report is printed after the last sample line and before `Saved:`. `Added device "Apple M1 Pro (GPU - 14 cores)" …` appeared in the processor-only run too, so it names what Cycles found, not what did the work. The graphics run's ordinary progress lines read `Sample 0/16 (8 kernels to optimize)`, then `(4 kernels to optimize)`, with `Mem: 1369M`–`1370M` against `2M` on the processor — a memory reading on a shared machine, offered only as an order of size.

**`--cycles-print-stats` goes to standard output, and names no device** (entry 3). Run as `blender -b scene.blend -o out/s_ -F PNG -f 1 -- --cycles-print-stats` with the two streams in separate files: exit 0, the error stream **0 bytes**, the whole statistics block on standard output — not on the error stream named in the `engine.py` help text quoted above. Its sections are `Mesh statistics`, `Image statistics`, `Kernel statistics`, `Shader statistics` and `Object statistics`, and the whole output matched none of `metal`, `cuda`, `optix`, `oneapi`, `gpu`, `cpu`, `apple`, `m1` or `nvidia`. With `--cycles-device METAL` added after the `--`, still exit 0 and a 0-byte error stream, the kernel, shader and object sections are replaced by one sentence:

```
Image statistics:
  Total image memory: 0B (0)
Profiling information not available (only works with CPU rendering)
```

Run over frames 1 to 3, it printed three `Render statistics:` blocks, one after each frame.

**No field written into a finished image names a device; the `Memory` field jumps when the graphics do the work** (entry 9). Every `use_stamp_*` switch turned on and the note set to `SMOOZY NOTE`, Cycles writing the same frame to PNG and to OpenEXR, once on the processor and once with `-- --cycles-device METAL` (that run's own report naming `Apple M1 Pro (GPU - 14 cores) (Metal)`). All exited 0. Read straight out of the files, the processor PNG carried `File`, `Note`, `Date`, `Marker`, `Time`, `Frame`, `FrameRange`, `Camera`, `Lens`, `Scene`, `Strip`, `RenderTime`, `Memory = 2M`, `Hostname = Artists-Studio-MacBook.local` and four Cycles fields — `cycles.ViewLayer.samples = 16`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time`, `cycles.ViewLayer.synchronization_time`. The graphics PNG carried the same names, with `Memory = 1370M`. The OpenEXR also carried `Software (string) = Blender 5.3.0 Beta`. Searching every field of these files for `metal`, `apple`, `m1`, `nvidia`, `geforce`, `cuda`, `optix`, `oneapi` and `device` gave 0 matches. The memory figure is a reading on a shared machine whose graphics use the machine's own memory, offered only as an order of size.

**At factory startup the Memory and Hostname switches are off** (entry 12). Read by a script run as `blender -b --factory-startup --python t_stamp.py`, exit 0, among the sixteen switches: `STAMP use_stamp False`, `STAMP use_stamp_hostname False`, `STAMP use_stamp_memory False`. So the "(default True)" quoted above for `use_stamp_memory` is not what a running 5.3 starts with, and with the master switch `use_stamp` off nothing is written into the picture itself unless it is turned on.

**The Render Time pass follows the scene's setting, not what did the work** (entry 19) — so the inference step written above, that a populated Render Time pass means a processor render, does not hold. Six renders to multi-layer OpenEXR with the pass on, the scene set to CPU or GPU Compute, each with no override, with `-- --cycles-device CPU` and with `-- --cycles-device METAL`; what did the work read from each run's own `Path tracing on:` line, the pass read from every part of the file:

| scene | override | what did the work | Render Time pass |
|---|---|---|---|
| CPU | none | processor | present |
| CPU | `CPU` | processor | present |
| CPU | `METAL` | **graphics (Metal)** | **present** |
| GPU | none | graphics (Metal) | absent |
| GPU | `CPU` | **processor** | **absent** |
| GPU | `METAL` | graphics (Metal) | absent |

All exited 0. With the pass present the file is two-part — `part 0: name='ViewLayer.Combined'`, `part 1: name='ViewLayer.Render Time' channels=['ViewLayer.Render Time.X']` — so it is seen only by reading past the first part.

**"Save System Info" works with no window, names the graphics in its Cycles section, and leaves its GPU section empty** (entry 22). `blender -b --python-expr "import bpy; print(bpy.ops.wm.sysinfo(filepath='…/si_5.3.txt'))"`: exit 0, `{'FINISHED'}`, a 96-line file holding `platform: 'macOS-26.6.2-arm64-arm-64bit-Mach-O'`, `GPU: missing, background mode`, and under `Cycles:` the lines `Metal device capabilities:`, `Number of devices: 1`, `Device: Apple M1 Pro (GPU - 14 cores)`. It describes the machine when written, not a render. Written instead from a Blender started with its window, by the same operator from a script (entry 33), the file was 120 lines and differed only in the temporary folder's name and its GPU section, which then read `renderer: 'Metal API'`, `vendor: 'Apple M1 Pro'`, `version: '1.2'`, `device type: 'APPLE'`, `backend type: 'METAL'` and a list of limits.

**The finished render, shown in Blender's own window, carries frame, time and memory and no device** (entry 35) — the hand test listed at the end of this document. A render started from a script in a Blender with its window open, on the graphics (Metal, scene at GPU Compute, 1600 by 900), photographed in its own window two seconds after it completed by Blender's screenshot command: the header showed `Render Result`, `Slot 1`, `ViewLayer`, and above the picture the line `Frame:1 | Time:00:00.81 | Mem:1419M, Peak: 1419M`. Nothing on screen named the graphics or the device.

**Nothing printed by an EEVEE or a Workbench render names the graphics chip** (entry 14). Rendered with `-E BLENDER_EEVEE` and `-E BLENDER_WORKBENCH` and `--log "*" --log-level info`: exit 0, error stream 0 bytes, and no line in either matching `metal`, `apple`, `m1`, `gpu`, `graphic`, `renderer`, `vendor` or `device` — the device-related lines are `ghost.system | Create background system` and `ghost.system | Create Cocoa system`. At `--log-level debug` both printed `gpu.metal | PSO CACHE: Stored new variant in PSO cache for shader '…'` lines, naming Metal as the drawing route, and still nothing naming the chip. A Cycles render on the processor at the same levels printed its `Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]` line and no `gpu.metal` line.

---

## 4. What does a render do when it is asked for something the computer or the installed Blender cannot provide, and what does it report?

**LABEL**: documented (for manual and API descriptions of memory errors, crash logs, CLI exit on unknown arguments, and driver errors); observed (for Cycles source code handling of CLI device override failures, disabled devices in UI, and unsupported denoiser/OSL checks); inferred (as noted below)

**Not covered**: Does not cover unhandled low-level operating system kernel panics, third-party plugin crashes, or abrupt hardware failure.

### Command-line argument and engine requests

1. **Unknown CLI argument**:
   When passed an unknown argument from the command line, Blender exits immediately:
   - Exact words:
     ```
     blender -ba test.blend

     …will exit since -ba is an unknown argument.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html`
     Date read: 2026-10-11

2. **Render engine selection (`-E`)**:
   When asked for a render engine via `-E <engine>`, the manual specifies checking available engines with `-E help`:
   - Exact words:
     ```
     -E, --engine <engine>

     Specify the render engine.
     Use -E help to list available engines.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html`
     Date read: 2026-10-11
   - Exact words:
     ```
     For a list of available render engines, run blender -E help.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html`
     Date read: 2026-10-11

3. **Cycles command-line device override failure (`--cycles-device`)**:
   When `--cycles-device <device>` is specified on the command line and the device override cannot be set (e.g. invalid device name or unavailable backend), Cycles terminates the Blender process with exit status 1.
   - Exact lines:
     ```python
         if args.cycles_device:
             import _cycles
             if not _cycles.set_device_override(args.cycles_device):
                 sys.exit(1)
     ```
     Source: Cycles source file `Blender Sources/cycles_5.3/engine.py`, lines 38-41
     Date read: 2026-10-16

### Unsupported hardware or missing/incompatible drivers

1. **Cycles Preferences device detection failure**:
   When no compatible GPU is detected for a backend in Cycles Preferences, Cycles displays an informational status label:
   - Exact lines:
     ```python
             if not found_device:
                 col = box.column(align=True)
                 col.label(text=rpt_("No compatible GPUs found for Cycles"), icon='STATUS_INFO', translate=False)
     ```
     Source: Cycles source file `Blender Sources/cycles_5.3/properties.py`, lines 1967-1969
     Date read: 2026-10-16
   Depending on the backend, specific requirements are reported in the UI:
   - CUDA / OptiX: `Requires NVIDIA GPU with compute capability 5.0 and NVIDIA driver version %s or newer` (`properties.py`, lines 1973-1981).
   - HIP on Windows: `Requires AMD GPU with RDNA architecture and AMD Adrenalin driver 24.9.1 or newer or AMD Radeon Pro 24.Q4 driver or newer` (`properties.py`, lines 1986-1993).
   - HIP on Linux: `Requires AMD GPU with RDNA architecture and ROCm HIP Runtime 6.3 or newer or AMD driver version 24.30 or newer` (`properties.py`, lines 1995-2003).
   - oneAPI on Windows: `Requires Intel(R) Arc(TM) GPUs or newer Intel(R) Graphics with Windows driver version XX.X.101.8306 or newer` (`properties.py`, lines 2016-2021).
   - oneAPI on Linux: `Requires Intel(R) Arc(TM) GPUs or newer Intel(R) Graphics ... intel-level-zero-gpu or intel-compute-runtime version XX.XX.37435.3 or newer ... oneAPI Level-Zero Loader` (`properties.py`, lines 2024-2037).
   - Metal on macOS: `Requires Apple Silicon with macOS 13.0 or newer` (`properties.py`, lines 2039-2041).

2. **Incompatible driver installed**:
   If a GPU device is present but does not meet driver requirements, Cycles disables the device checkbox in Preferences, marks it `(Disabled)`, and reports driver upgrade details:
   - Exact lines:
     ```python
                 if not device.meets_driver_requirement:
                     import sys
                     row.active = False
                     name += rpt_(" (Disabled)")
                     row.prop(device, "use", text=name, translate=False)
     ```
     Source: Cycles source file `Blender Sources/cycles_5.3/properties.py`, lines 2053-2057
     Date read: 2026-10-16
   Details displayed: `Requires NVIDIA driver version ... or newer`, `Requires AMD Adrenalin driver ...`, `Requires Windows driver version ...`, or `Driver upgrade required` (`properties.py`, lines 2060-2084).

3. **Deactivation of the GPU Device selector in Scene properties**:
   If no usable GPU backend is configured or enabled in preferences, the Scene render device dropdown in Cycles is deactivated (greyed out) in the interface:
   - Exact lines:
     ```python
             col = layout.column()
             col.active = show_device_active(context)
             col.prop(cscene, "device")
     ```
     Source: Cycles source file `Blender Sources/cycles_5.3/ui.py`, lines 2548-2550
     Date read: 2026-10-16
   In `ui.py`, lines 145-149, `show_device_active(context)` returns `use_gpu(context)` when `cscene.device == 'GPU'`. In line 116, `use_gpu(context)` requires `get_device_type(context) != 'NONE'` and `backend_has_active_gpu(context)`.

4. **Dynamic CUDA kernel compilation failure**:
   If an NVIDIA GPU is not yet supported by precompiled kernels, dynamic compilation may fail and report:
   - Exact words:
     ```
     CUDA Error: Kernel compilation failed

     This error may happen if you have a new NVIDIA graphics card that is not yet supported by
     the Blender version and CUDA toolkit you have installed.
     In this case Blender may try to dynamically build a kernel for your graphics card and fail.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html`
     Date read: 2026-10-11

5. **Incompatible GCC version on Linux**:
   - Exact words:
     ```
     Unsupported GNU version

     On Linux, depending on your GCC version you might get this error.
     ...
     #error -- unsupported GNU version! gcc x.x and up are not supported!
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html`
     Date read: 2026-10-11

6. **OpenGL driver requirement failure**:
   - Exact words:
     ```
     Unsupported Graphics Driver Error

     This means your graphics card and driver do not have the minimum required OpenGL 3.3 version needed by Blender.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/troubleshooting/gpu/windows/other.html`
     Date read: 2026-10-11

### Memory exhaustion (GPU VRAM and System RAM)

1. **GPU VRAM exhaustion and system memory fallback**:
   When GPU memory is exhausted during rendering, Cycles automatically attempts to fall back to system memory on supported architectures:
   - Exact words:
     ```
     With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
     try to use system memory. This has a performance impact, but will usually still result in a faster render
     than using CPU rendering.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html`
     Date read: 2026-10-11
   If allocation still fails, Cycles reports:
   - Exact words:
     ```
     Error: Out of memory

     This usually means there is not enough memory to store the scene for use by the GPU.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html`
     Date read: 2026-10-11

2. **EEVEE GPU memory exhaustion**:
   EEVEE relies entirely on the GPU driver for memory management. Excessive memory demands can cause the driver to crash, freeze, or terminate Blender:
   - Exact words:
     ```
     In practice, using too much GPU memory can make the GPU driver crash, freeze, or kill the application.
     So be careful of what you ask.
     There is no standard way of estimating if the resources will fit into the GPU memory and/or
     if the GPU will render them successfully.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html`
     Date read: 2026-10-11

3. **System RAM exhaustion and crash logs**:
   When system memory (RAM) is exhausted, Blender crashes and writes a crash log to disk:
   - Exact words:
     ```
     The most common causes of Blender crashes:

     Running out of memory.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/troubleshooting/crash.html`
     Date read: 2026-10-11
   - Exact words:
     ```
     Crash Log

     When Blender crashes, it writes out a text file which contains information that may help identify the cause of the
     crash. Usually, this file is written in the Temporary Directory directory.
     ```
     ```
     Windows
     On a crash, a file is written based on the name of the currently loaded blend-file, so test.blend will create a
     file called test.crash.txt.
     ...
     macOS
     After a crash, the macOS Crash Reporter shows a window with backtrace after some time, or when Blender is opened
     again.
     ...
     Linux
     After a crash, a file named blender.crash.txt is written to the /tmp directory.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/troubleshooting/crash.html`
     Date read: 2026-10-11

### Display driver timeout (TDR)

When a GPU is shared between OS display and rendering on Windows and a render computation exceeds the driver timeout threshold:
- Exact words:
  ```
  The NVIDIA OpenGL driver lost connection with the display driver

  If a GPU is used for both display and rendering,
  Windows has a limit on the time the GPU can do render computations.
  If you have a particularly heavy scene, Cycles can take up too much GPU time.
  ```
  Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html`
  Date read: 2026-10-11

### Unsupported feature combinations

1. **Path Guiding on GPU**:
   - Exact words:
     ```
     Path Guiding is not supported on any GPU.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html`
     Date read: 2026-10-11
   - Exact words:
     ```
     Path guiding is only available when rendering on a CPU.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html`
     Date read: 2026-10-11

2. **CPU rendering in EEVEE**:
   EEVEE does not provide software/CPU rendering:
   - Exact words:
     ```
     Being a rasterization engine, EEVEE only uses the power of
     the GPU to render.
     There is no plan to support CPU (software) rendering
     as it would be very inefficient.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html`
     Date read: 2026-10-11

3. **Multiple GPUs in EEVEE**:
   - Exact words:
     ```
     There is currently no support for
     multiple GPU systems.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html`
     Date read: 2026-10-11

4. **Headless rendering in EEVEE on Windows**:
   - Exact words:
     ```
     Headless rendering is not supported on headless Windows systems.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html`
     Date read: 2026-10-11

5. **Render Time pass on GPU**:
   - Exact words:
     ```
     Render Time Cycles

     An estimate for how long in milliseconds each pixel took to render.
     Note, this pass is not supported on GPU rendering.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/layers/passes.html`
     Date read: 2026-10-11
   - Exact lines:
     ```python
             # Render Time pass - disabled for GPU devices
             scene = context.scene
             cscene = scene.cycles
             row = col.row()
             row.enabled = (cscene.device == 'CPU')
             row.prop(cycles_view_layer, "pass_render_time", text="Render Time")
     ```
     Source: Cycles source file `Blender Sources/cycles_5.3/ui.py`, lines 1118-1123 *(Corrected by citation check: corrected line numbers and replaced misquoted `crl` with `cycles_view_layer` and `text="Render Time"` as in `ui.py`)*
     Date read: 2026-10-16

6. **Open Shading Language (OSL) on non-OptiX GPU**:
   - Exact words:
     ```
     Open Shading Language is only supported for OptiX, with some limitations listed in the documentation.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html`
     Date read: 2026-10-11
   In `cycles_5.3/ui.py`, lines 2553-2556, the `shading_system` (OSL) UI control is only shown when rendering on CPU or OptiX with supported OSL version.

7. **DLSS viewport denoising on unsupported hardware**:
   If DLSS denoising is enabled on hardware without DLSS support, Cycles displays an info label in the sampling panel:
   - Exact lines:
     ```python
                 col.label_multiline(
                     text=rpt_("Requires NVIDIA GPU with compute capability %s and NVIDIA driver version %s or newer.") %
                     ("7.5", "590"), icon='STATUS_INFO', translate=False)
     ```
     Source: Cycles source file `Blender Sources/cycles_5.3/ui.py`, lines 284-286
     Date read: 2026-10-16

8. **Blender build compiled without viewport denoiser**:
   If compiled without denoiser libraries, Cycles displays:
   - Exact lines:
     ```python
         else:
             items = [('AUTO', "None", n_("Blender was compiled without a viewport denoiser"), 0)]
     ```
     Source: Cycles source file `Blender Sources/cycles_5.3/properties.py`, lines 311-313
     Date read: 2026-10-16

9. **Formats not compiled into Blender**:
   If an output format is requested that was not compiled into the Blender build:
   - Exact words:
     ```
     Formats that can be compiled into Blender, not available on all systems:
     OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP.
     ```
     Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html`
     Date read: 2026-10-11

10. **EEVEE-only nodes in Cycles**:
    If an EEVEE-only shader node (such as Shader to RGB) is used in Cycles:
    - Exact words:
      ```
      EEVEE only Nodes

      These nodes are only available if EEVEE is the active render engine. These nodes will not work in Cycles.
      ```
      Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/eevee/limitations/nodes_support.html`
      Date read: 2026-10-11

11. **Shadow Caustics hardware restrictions in Cycles**:
    - Exact words:
      ```
      Shadow Caustics are not supported when using HIP without hardware ray-tracing.
      ```
      Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html`
      Date read: 2026-10-11
    - Exact words:
      ```
      GPU rendering using Radeon RX 6000 series HIP devices on Windows is not supported.
      ```
      Source: Blender Manual, `https://docs.blender.org/manual/en/5.3/render/cycles/object_settings/object_data.html`
      Date read: 2026-10-11

### Render engine error reporting mechanisms (Python API)

When an error occurs during execution, the render engine interface provides dedicated methods to report errors and set the terminal error message:
- Exact words:
  ```
  report(type, message)

  Report info, warning or error messages

  Parameters:

  type (set[Literal[Wm Report Items]]) – Type

  message (str) – Report Message, (never None)

  error_set(message)

  Set error message displaying after the render is finished

  Parameters:

  message (str) – Report Message, (never None)
  ```
  Source: Python API Documentation, `https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html`
  Date read: 2026-10-11
The report types supported by `Wm Report Items` are:
- Exact words:
  ```
  DEBUG:
  Debug.
  INFO:
  Info.
  OPERATOR:
  Operator.
  PROPERTY:
  Property.
  WARNING:
  Warning.
  ERROR:
  Error.
  ERROR_INVALID_INPUT:
  Invalid Input.
  ERROR_INVALID_CONTEXT:
  Invalid Context.
  ERROR_OUT_OF_MEMORY:
  Out of Memory.
  ```
  Source: Python API Documentation, `https://docs.blender.org/api/5.3/bpy_types_enum_items/wm_report_items.html`
  Date read: 2026-10-11

### Record of where I looked

- **Searches run**:
  - `grep -in "exit"` in `Blender Sources/manual_5.3/advanced/command_line/arguments.txt`: identified CLI exit on unknown arguments.
  - `grep -in "cycles-device"` and `grep -in "set_device_override"` in `Blender Sources/cycles_5.3/engine.py`: found `sys.exit(1)` on invalid device override.
  - `grep -in "No compatible GPUs"` and `grep -in "meets_driver_requirement"` in `Blender Sources/cycles_5.3/properties.py`: located GPU detection labels and driver upgrade warnings.
  - `grep -in "error"` in `Blender Sources/manual_5.3/render/cycles/gpu_rendering.txt`: found `Error: Out of memory`, `CUDA Error: Kernel compilation failed`, `Unsupported GNU version`, and TDR driver disconnect.
  - `grep -in "not supported"` in `Blender Sources/manual_5.3/render/cycles`: checked feature limitations (Path Guiding, OSL, caustics).
  - `grep -rn "limitations"` in `Blender Sources/manual_5.3/render/eevee`: inspected EEVEE memory management, CPU rendering limitations, and node compatibility.
  - `grep -rn "crash"` in `Blender Sources/manual_5.3/troubleshooting`: checked crash reasons (memory exhaustion) and crash file locations across OSes.
  - `grep -rn "report"`, `grep -rn "error_set"`, and `grep -rn "Wm Report Items"` in `Blender Sources/api_5.3`: checked RenderEngine error handling and report enum constants.
- **Pages opened**:
  - `manual_5.3/render/cycles/gpu_rendering.txt`: examined all error messages, memory limits, and hardware requirements.
  - `manual_5.3/render/cycles/render_settings/sampling.txt`: checked CPU exclusivity of Path Guiding.
  - `manual_5.3/render/cycles/render_settings/performance.txt`: checked tile size and memory reduction settings.
  - `manual_5.3/render/cycles/object_settings/object_data.txt`: checked caustics GPU limitations.
  - `manual_5.3/render/eevee/limitations/limitations.txt`: checked EEVEE GPU memory driver crash risk, lack of CPU rendering, and lack of multi-GPU support.
  - `manual_5.3/render/eevee/limitations/nodes_support.txt`: checked EEVEE-only nodes in Cycles.
  - `manual_5.3/troubleshooting/crash.txt`: checked crash log writing per OS for memory exhaustion.
  - `manual_5.3/troubleshooting/gpu/windows/other.txt`: checked OpenGL minimum version errors.
  - `manual_5.3/advanced/command_line/arguments.txt`: checked command-line parsing rules and exit behaviors.
  - `manual_5.3/advanced/command_line/render.txt`: checked render engine command-line selection.
  - `api_5.3/bpy.types.RenderEngine.txt`: checked `report()`, `error_set()`, `end_result()`, and memory update functions.
  - `api_5.3/bpy_types_enum_items/wm_report_items.txt`: checked all report severity items including `ERROR_OUT_OF_MEMORY`.
  - `cycles_5.3/engine.py`: verified command line device override exit behavior (`sys.exit(1)`).
  - `cycles_5.3/properties.py`: verified device list checks, disabled driver flags, and fallback enums.
  - `cycles_5.3/ui.py`: checked UI greying-out and warning labels for devices, OSL, denoisers, and passes.
- **Ruled out**:
  - Ruled out automatic mid-render fallback from GPU to CPU on device failure: Neither Cycles nor EEVEE documentation mentions automatic runtime fallback from GPU to CPU when a GPU errors during a render. EEVEE has no CPU renderer, and Cycles GPU memory exhaustion falls back to host system RAM via driver page migration, not to the CPU render engine.
  - Ruled out silent continuation on unrecognized CLI arguments: `arguments.txt` explicitly rules this out by stating that Blender exits when an unknown argument is encountered.

### Outside the interface
Holds outside the interface. CLI errors and device override failures cause Blender to terminate execution with error messages or exit status 1 (`sys.exit(1)`). Memory exhaustion crashes write crash log text files to disk in the OS temporary directory without an interactive UI dialog. GPU VRAM overflow triggers driver system memory fallback or prints `Error: Out of memory` to stderr/stdout. GUI-only elements (such as greyed-out property rows in Preferences or post-render popup notifications via `error_set`) do not appear in background CLI rendering. **Tested by hand on 2026-10-17, on the Creator's Mac only — see below.** With no window: a refused `--cycles-device` ends with exit 1 and a message on standard output (entry 15); an unknown engine given with `-E` ends with exit 1 (entry 13); a file whose engine is not installed prints an `ERROR` line and renders with EEVEE, exit 0 (entry 13); an unknown argument is read as a file name rather than ending Blender at once (entry 26); a GPU scene with no usable backend, and path guiding or Open Shading Language asked of the graphics, finish without a word (entries 17, 20, 21). The preference panel's texts were reached only with a window open (entry 32).

### Operating system
Differs by operating system:
- Headless rendering with EEVEE is unsupported on headless Windows systems.
- Windows enforces display driver timeout (TDR) when display and render share a GPU, leading to `The NVIDIA OpenGL driver lost connection with the display driver`.
- Crash log paths differ: Windows writes `<blend_name>.crash.txt` to `%TEMP%`; Linux writes `/tmp/blender.crash.txt`; macOS uses Apple Crash Reporter and writes to `~/Library/Logs/DiagnosticReports/`.
- GPU backend support is OS-dependent: Metal requires Apple Silicon on macOS 13.0+; CUDA and OptiX require Windows or Linux; HIP requires specific Windows Adrenalin/Pro drivers or Linux ROCm/AMD drivers; oneAPI requires specific Windows or Linux compute packages.

**The hand tests of 2026-10-17 were run on the Creator's Mac only**, so they settle macOS on Apple silicon with Metal: there `METAL` and `CPU` are accepted by `--cycles-device` and `OPTIX`, `CUDA`, `HIP` and `ONEAPI` are refused (entry 15). The Windows- and Linux-specific cases listed above, the crash-log locations, and every NVIDIA, AMD and Intel graphics case — memory running out, a kernel failing to build, a driver below its requirement — were not tried in 5.3: there was no Windows or Linux machine and no such card for this sitting, and making Blender crash would write outside the work folder.

### Tested by hand on the Mac, 2026-10-17 — **LABEL: observed**

The machine, the Blender and the method are described under question 1's hand-test section above. Nothing was run on Linux or Windows, so every Windows- and Linux-specific line above stays untested here.

**A format that cannot hold metadata drops it without a word** (entry 10). With every metadata switch on, the scene written as `-F TIFF` and then as `-F TGA`: both exited 0 and wrote their file, and counting the lines in both streams that match any of `warn`, `error`, `metadata`, `not supported` or `preserved` gave **0** for each.

**`-E` with an engine that does not exist is refused; a file asking for an engine that is not installed renders with EEVEE** (entry 13). `blender -b -E help` exited 0 and listed `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`. `blender -b scene.blend -E NOT_AN_ENGINE -o out/be_ -F PNG -f 1`: **exit 1**, no frame, and on the error stream `Error: engine not found 'NOT_AN_ENGINE'`. A file saved with an engine no Blender has — `SMOOZY_TEST_ENGINE`, defined for one session by a script and never installed — rendered in a fresh Blender with **exit 0**, a frame written, the error stream 0 bytes, and on standard output:

```
00:00.334  reports          | ERROR Engine 'SMOOZY_TEST_ENGINE' not available for scene 'Scene' (an add-on may need to be installed or enabled)
00:00.335  render           | Start rendering: Scene, ViewLayer
00:00.335  render           | Engine: EEVEE
```

and the scene's engine, read by a script once the file had loaded, was `'BLENDER_EEVEE'`. Also: `-E BLENDER_EEVEE` written *before* the file name gave `Engine: Cycles` (the file's own), and written *after* it gave `Engine: EEVEE`.

**`--cycles-device` with a name Cycles does not know, or with a backend this Mac has no hardware for, ends with exit 1 and no frame** (entry 15). Eight runs of `blender -b scene.blend -o out/d_<name>_ -F PNG -f 1 -- --cycles-device <name>`: `NOTADEVICE`, `BANANA`, `OPTIX`, `CUDA`, `HIP` and `ONEAPI` each exited **1** with no frame; `METAL` and `CPU` each exited **0** with a frame. Every error stream was 0 bytes. The unknown names printed, as the first line, before the file was read: `00:00.384  cycles           | ERROR NOTADEVICE is not a valid Cycles device.` and then `Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB`. The four absent backends each read the file, began the render (`Engine: Cycles`, `Fra: 1 | Mem: 0M | Initializing`) and then printed `00:00.347  reports          | ERROR Found no Cycles device of the specified type`. Both kinds ended with exit status 1, and what was printed was on standard output.

**"No compatible GPUs found for Cycles" cannot be made to appear on this Mac** (entry 32) — the label quoted above under *Cycles Preferences device detection failure*. Preferences opened in a Blender with its window, System section, photographed by Blender's own screenshot command after a forced redraw: the Cycles Render Devices panel offered two buttons, `None` and `Metal`, with `Metal` chosen, `Apple M1 Pro (GPU - 14 cores)` ticked, `Apple M1 Pro` unticked, `Kernel Optimization: Full`, `MetalRT: Auto`. With the preference set to `None` for the session, the panel showed the two buttons and nothing else — no list and no message. No other backend is offered on this Mac, so the label's state cannot be reached here.

**A scene asking for GPU Compute with no usable GPU backend renders on the processor, without a warning** (entry 17). A script refreshed the device list, set the Cycles preference to `NONE`, unticked every device and set the scene to GPU Compute — the state the code quoted above under *Deactivation of the GPU Device selector* describes — and the run had `--log "*" --log-level info`. Exit 0, a frame written, the error stream 0 bytes, and:

```
PREF_COMPUTE_DEVICE_TYPE NONE
DEVICES_IN_USE [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', False)]
SCENE_CYCLES_DEVICE GPU
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
```

No line matched `warn`, `fall` or `substitut`. The same file with the scene set to CPU printed the identical device line.

**Path guiding asked for on the graphics: the render finishes on the graphics and says nothing** (entry 20) — the case the two quotes under *Path Guiding on GPU* above describe. `use_guiding` (its own description: "Use path guiding for sampling paths…") switched on, Cycles pointed at Metal, the scene at GPU Compute, `--log "*" --log-level info`: exit 0, a frame, the report `Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]`, and no line mentioning guiding. Its output was identical, timestamps and figures aside, to the same run with guiding off. On the processor the same switch did change what was printed — the samples reported in steps of four (`Rendered 4 samples …`, `Sample 5/16`, `Sample 9/16`, `Sample 13/16`) where with it off they were one step of fifteen — again with no line naming guiding.

**Open Shading Language asked for on the graphics: the render finishes on the graphics and says nothing** (entry 21) — the case the quote under *Open Shading Language (OSL) on non-OptiX GPU* above describes. `shading_system` (its own description: "Use Open Shading Language") switched on, Cycles pointed at Metal, the scene at GPU Compute, `--log "*" --log-level info`: exit 0, a frame, the report `Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]`, no line mentioning it, and an output identical, timestamps and figures aside, to the same run with it off. On the processor the same switch printed `cycles | Using shader search path: /Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/cycles/shader` in place of `Shader manager updated 8 shaders in …`.

**An unknown command-line argument is read as a file name rather than ending Blender at once** (entry 26) — set against the *Unknown CLI argument* finding above. Each run was given a fixed eight seconds and then stopped if still going, in the same command. `blender -b scene.blend --not-a-real-option -o out/u_ -F PNG -f 1`: **exit 1**, nothing rendered, `unknown argument, loading as file: --not-a-real-option` on the error stream and `ERROR Cannot read file "…/out/e26/--not-a-real-option": No such file or directory` on standard output. `blender -ba scene.blend`, the manual's own example: `unknown argument, loading as file: -ba`, `ERROR Cannot read file "…/out/e26/-ba": No such file or directory`, then `Read blend: "…/work/scene.blend"` — and still running at eight seconds, when it was stopped (status 143). So in the first case the run ended with exit 1 and in the second it carried on.

**A render that cannot write its output fails with exit 1 and says why** (entry 25). With a plain file where the output folder should be: exit 1, error stream 0 bytes, and on standard output `ERROR Couldn't create directory for file …/notadir/z_0001.png: File exists` and `ERROR Render error (File exists) cannot save: '…/notadir/z_0001.png'`. And `-F NOSUCHFORMAT` gave exit 0, a PNG in the scene's own format, and on the error stream `Error: Format from '-F / --render-format' not known or not compiled in this release.` — the case the quote under *Formats not compiled into Blender* above warns about, which on this build could only be provoked as a wrong name.

**A render stopped part-way leaves no file and ends with status 143** (entry 27). A slow render (100000 samples, 1200 by 900, adaptive sampling off) started in the background and sent the ordinary stop signal after six seconds: still rendering then, exit status **143**, no file at the output path, error stream 0 bytes, and the last printed line an ordinary `Sample 1/100000` progress line. With adaptive sampling on, the same render finished by itself at `Sample 960/100000` before the six seconds, exit 0, a complete PNG whose fields still said `cycles.ViewLayer.samples = 100000`.

---

## Things a person could settle by hand

The five items that stood here were run on 2026-10-17 on the Creator's Mac, and what they found is written into the two answers above as observed findings: item 1 as entry 35, item 2 as entries 9 and 11 (the second written into question 5 of the first 5.3 document), item 3 as entry 13, item 4 as entry 15, and item 5 as entry 19 — the entry numbers of `Audits/Blender Research/What actually ran/Hand tests — 5.3.md`, which holds every command and raw output. Item 1 was run with Apple's own graphics doing the work, through Metal.

**Still open, and why.** Each of the five on Linux or Windows, and item 1 on an NVIDIA, AMD or Intel card: there was no Linux machine, no Windows machine and no such card for this sitting. A Windows answer would have to name its edition; see the equipment section of `Preparation/Blender Research/Method.md`. The memory, driver, crash-log and TDR cases under question 4 above stay untested for the same reasons.


2026-10-17 — Gemini 3.8 Flash High — What actually ran, Blender 5.3

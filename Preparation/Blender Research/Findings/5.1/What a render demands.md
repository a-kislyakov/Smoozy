# What a render demands — Blender 5.1

**Sources consulted, and when.**
Blender's manual for 5.1 (`Blender Sources/manual_5.1`, including its `_images` folder), Blender's Python API documentation for 5.1 (`Blender Sources/api_5.1`), the Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`) — all four read on **2026-07-30** (the snapshot date given to me). Blender's requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`) — read on **2026-08-01**.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

One disclosure that the line above does not cover, because it is not about opening anything. Before I read my brief, this session's harness had already placed two things in front of me without my asking: the text of the project's `CLAUDE.md`, and a short list of the operator's saved preferences. I did not open either, and I opened nothing they pointed to. `CLAUDE.md` turned out to be a signpost that tells Blender researchers to stop reading it, which I did. The preferences list contained one line naming what this project is for. I have not used it anywhere below, and no answer here would read differently if I had never seen it — but you should know it reached me, and discount accordingly if that matters to you.

---

## A note on the question sheet before the answers

The sheet's own preamble sets the scope: "the demand a render makes — how hard it works the computer and how much of it the render takes up". Question 1 asks "what settings, if any, change how much a render demands of the computer".

**That question is ambiguous, and I want to flag it rather than quietly pick.** Read at its widest, nearly every render setting changes demand: a scene with more samples takes longer, a bigger image takes more memory, an extra light bounce costs time. Read narrowly, it means the settings whose stated *purpose* is to govern resource use — cores, memory, which processor does the work.

**I answer the narrow reading as the main answer**, because the manual itself draws exactly that line: it gives Cycles, EEVEE and Workbench each a panel literally called "Performance", and describes the Cycles one as holding "Properties that affect render speeds or memory consumption." That is a boundary the source drew, not one I invented. **I then list the wider set separately**, marked as such, so nothing is hidden. Where the wider reading was meant, the second list is the answer to questions 1 and 2.

---

## 1. What settings, if any, change how much a render demands of the computer it runs on?

**LABEL: mixed — documented** for the list itself, with the Cycles panel contents also confirmed **observed** in Cycles' own files, and two things marked **inferred** where they are reached below: what the preferences picture in 1c does not hold, and the claim that the narrow list is complete.

### 1a. The settings the manual itself files under "Performance"

**Cycles.** The manual's Cycles Performance page opens:

> "Properties that affect render speeds or memory consumption.
> There are several presets available to help choose between different trade offs:
>
> Default:
>
> Balances memory saving and faster rendering settings.
>
> Faster Render:
>
> Uses settings to render faster at the cost of higher memory consumption.
>
> Lower Memory:
>
> Uses settings to decrease memory considered at the cost of slower renders."

The panel holds, in the manual's own words:

> "Threads
>
> Threads Mode
>
> Method to determine the maximum number of CPU cores to use while rendering.
>
> Auto-Detect:
>
> Automatically chooses the amount of threads to match the number of logical processors on your computer.
>
> Fixed:
>
> Manually choose the maximum number threads to use for rendering.
> This can be useful for example, if you want to use your computer while rendering,
> you can set the property to a thread count lower the amount of logical processors on your computer.
>
> Threads
>
> The maximum number of CPU cores to use simultaneously while rendering.
>
> Memory
>
> Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage.
> Tiles are cached to disk while rendering to save memory.
>
> [...]
>
> Acceleration Structure
>
> Use Spatial Splits
>
> Spatial splits improve the rendering performance in scenes with a mix of large and small polygons.
> The downsides are longer BVH build times and slightly increased memory usage.
>
> Use Curves BVH
>
> Use a special type of BVH for rendering curves.
> The bounding boxes are not axis aligned allowing a spatially closer fit to the curve geometry.
> Disabling this option will reduce memory, at the cost of increasing curve render time.
>
> BVH Time Steps
>
> Split BVH primitives by this number of time steps to speed up render time at the expense of memory.
>
> Use Compact BVH
>
> Use a more compact BVH structure, which can reduce RAM usage but render slower.
>
> Final Render
>
> Persistent Data
>
> Keep render data in memory after rendering for faster re-renders and animation renders
> at the cost of extra memory usage while performing other tasks in Blender.
>
> When using multiple View Layers,
> only data from a single view layer is preserved to keep memory usage within bounds;
> however, objects shared between view layers are preserved."

The same panel also carries a Viewport Pixel Size, a Compositor Device (CPU or GPU) and Precision, and Denoise Node device and quality settings. Of those, Pixel Size is viewport-only ("Option to control the resolution for viewport rendering"), and I set it aside as not bearing on a render.

Source: Blender manual — Cycles Render Settings ‣ Performance, `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html`. Read 2026-07-30.

**The same list, observed in Cycles' own files.** Cycles defines a "Performance" preset, and the list of values it stores is the sharpest statement of what Cycles itself counts as a performance setting:

```
class AddPresetPerformance(AddPresetBase, Operator):
    '''Add an Performance Preset'''
    bl_idname = "render.cycles_performance_preset_add"
    ...
    preset_values = [
        "render.threads_mode",
        "render.use_persistent_data",
        "cycles.debug_use_spatial_splits",
        "cycles.debug_use_compact_bvh",
        "cycles.debug_use_hair_bvh",
        "cycles.debug_bvh_time_steps",
        "cycles.tile_size",
    ]
```

File: `Blender Sources/cycles_5.1/presets.py`, class `AddPresetPerformance` (near the end of the file, before the `classes` tuple). Read 2026-07-30. **Label: observed.**

Note what this shows: two of the seven (`threads_mode`, `use_persistent_data`) live on Blender's own render settings, and five live on Cycles' settings. That split matters for question 4.

Their kinds, defaults and ranges, quoted from Cycles' own file `Blender Sources/cycles_5.1/properties.py`, class `CyclesRenderSettings` (**observed**):

> `debug_use_spatial_splits: BoolProperty(name="Use Spatial Splits", description="Use BVH spatial splits: longer builder time, faster render", default=False,)`
>
> `debug_use_hair_bvh: BoolProperty(name="Use Curves BVH", description="Use special type BVH optimized for curves (uses more ram but renders faster)", default=True,)`
>
> `debug_use_compact_bvh: BoolProperty(name="Use Compact BVH", description="Use compact BVH structure (uses less ram but renders slower)", default=False,)`
>
> `debug_bvh_time_steps: IntProperty(name="BVH Time Steps", description="Split BVH primitives by this number of time steps to speed up render time in cost of memory", default=0, min=0, max=16,)`
>
> `tile_size: IntProperty(name="Tile Size", default=2048, description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory", min=8, max=8192,)`
>
> `use_auto_tile: BoolProperty(name="Auto Tile", description="Deprecated, tiling is always enabled", default=True,)`

That last one is worth pointing at: the file says in so many words that tiling is no longer optional.

The two that live on Blender's own render settings, from the API documentation (`https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html`, read 2026-07-30):

> "threads
>
> Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) (in [1, 1024], default 1)"

> "threads_mode
>
> Determine the amount of render threads used (default 'AUTO')
>
> AUTO Auto-Detect – Automatically determine the number of threads, based on CPUs.
>
> FIXED Fixed – Manually determine the number of threads."

> "use_persistent_data
>
> Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage (default False)"

**EEVEE.** Its Performance panel is much shorter:

> "High Quality Normals
>
> Uses higher precision normals and tangents which can improve
> visual quality for dense meshes with high frequency textures at the cost of memory.
>
> Memory
>
> Shadow Pool
>
> A bigger pool size allows for more shadows in the scene
> but might not fit into GPU memory and decreases performance.
> Increasing the size might fix the Shadow buffer full error.
>
> [...]
>
> Light Probes Volume Pool
>
> A bigger pool size allows for more irradiance grids in the scene
> but might not fit into GPU memory and decreases performance."

Source: `https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/performance.html`. Read 2026-07-30. There is **no threads setting and no tile size setting on EEVEE's Performance panel.**

**Workbench.** Its Performance page contains one item only, in full:

> "High Quality Normals
>
> Uses higher precision normals and tangents which can improve
> visual quality for dense meshes with high frequency textures at the cost of memory."

Source: `https://docs.blender.org/manual/en/5.1/render/workbench/performance.html`. Read 2026-07-30.

### 1b. Which processor does the work — a setting of its own

> "Device
>
> Specifies the compute device used for rendering in Cycles.
> The choice of device can significantly affect render performance, GPU Compute is often faster .
>
> CPU:
>
> Render using the system's central processing unit. This mode is compatible with all Cycles features.
> The number of CPU threads used can be changed in the
> Performance Properties.
>
> GPU Compute:
>
> Render using one or more graphics processing units (GPUs) through the compute backend
> and devices selected in the Cycles Render Devices."

Source: Blender manual — Cycles Render Settings, `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html`. Read 2026-07-30.

Observed in Cycles' own file `Blender Sources/cycles_5.1/properties.py`:

> `enum_devices = (('CPU', "CPU", "Use CPU for rendering"), ('GPU', "GPU Compute", "Use GPU compute device for rendering, configured in the system tab in the user preferences"),)`

and, in class `CyclesRenderSettings`:

> `device: EnumProperty(name="Device", description="Device to use for rendering", items=enum_devices, default='CPU', update=update_render_passes,)`

**Label: observed.** So the scene-level choice is only two-valued — CPU or GPU Compute — and defaults to CPU. *Which* GPUs are used is not a scene setting; it is a preference (see question 3).

### 1c. Settings that sit in the Preferences, not in a scene

The manual's System preferences page lists, among others:

> "Cycles Render Devices
>
> Changes the Computing Device the Cycles render engine uses to render images."

> "Distribute Memory Across Devices
>
> Allocates resources across multiple GPUs rather than duplicating data,
> effectively freeing up space for larger scenes."

> "Embree on GPU
>
> Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance."

> "HIP RT
>
> Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above."

> "MetalRT
>
> MetalRT for ray tracing uses less memory for scenes which use curves extensively,
> and can give better performance in specific cases."

Source: `https://docs.blender.org/manual/en/5.1/editors/preferences/system.html`. Read 2026-07-30. I also opened the picture on that page, `_images/editors_preferences_section_system.webp` (**documented**, from the manual). It shows a "Cycles Render Devices" panel with the choices None / CUDA / HIP as buttons, a "Display Graphics" panel with a "Backend" dropdown reading OpenGL, and a "Memory & Limits" panel containing Undo Steps, Undo Memory Limit, Global Undo, Console Scrollback Lines, Texture Time Out and Garbage Collection Rate. What the picture settles, **documented**: those six items are what the "Memory & Limits" section holds. That none of the six caps a render's memory is **inferred, and the step is mine** — the picture gives me their names only, and the reader has to supply what each of them does. What the picture does *not* settle: which device backends appear on some other machine, since the buttons shown reflect that one computer's hardware.

The same page also carries two shader-compilation settings that trade memory for speed:

> "Shader Compilation Method [...] Thread: Uses multiple threads within a single process to compile shaders concurrently. This method is more memory-efficient but may be slower on some systems. Subprocess: Uses multiple separate subprocesses to compile shaders in parallel. This can be faster, especially on systems with high core counts, but consumes significantly more RAM."

> "Threads / Subprocesses
>
> The number of shader compilation threads or subprocesses to use.
> The maximum value is limited by the number of logical CPU cores on the system.
>
> Increasing the number can reduce shader compilation time at the cost of higher memory usage.
> A value of 0 lets Blender automatically choose a suitable number based on the system configuration."

The manual adds for both: "This option is not available on macOS and requires using OpenGL backend. Changing this setting requires restarting Blender to take effect."

### 1d. Settings that only exist as command-line switches

These are not properties of a file at all. From the manual's Command Line Arguments page (`https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`, read 2026-07-30):

> "-t, --threads <threads>
>
> Use amount of <threads> for rendering and other operations
> [1-1024], 0 to use the systems processor count."

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU."

> "--qos <level>
>
> Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).
>
> default: Uses the default behavior of the OS.
> high: Always makes use of performance cores.
> eco: Schedules Blender threads exclusively to efficiency cores."

> "--gpu-backend
>
> Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."

> "--debug-depsgraph-no-threads
>
> Switch dependency graph to a single threaded evaluation."

`--cycles-device` is confirmed **observed** in Cycles' own file `Blender Sources/cycles_5.1/engine.py`, function `_configure_argument_parser`:

> `parser.add_argument("--cycles-device", help="Set the device to use for Cycles, overriding user preferences and the scene setting." "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'." "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.", default=None)`

Note that Cycles' own help text says something the manual does not: that this switch is **"overriding user preferences and the scene setting."**

### 1e. The wider set — settings that change demand as a side effect of changing quality

Answering the wide reading of the question. All quotes are from the 5.1 manual, read 2026-07-30.

Scene-wide, all engines — **Simplify** (`https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/simplify.html`):

> "Max Subdivision — Maximum number of subdivision by the Subdivision Surface modifiers."
> "Child Particles — Show only a subset of all child hairs and particles."
> "Texture Limit — Automatically scales textures down so that they are no larger than the values chosen. This can help reduce computer memory resources when rendering large scenes with huge textures."
> "Volume Resolution — Resolution percentage of volume objects in the viewport. This mostly affects memory usage rather than computation times."
> "Camera Cull — Automatically culls objects based on the camera frustum defined by the Margin."
> "Distance Cull — Automatically culls objects based on their distance from the active camera."

Output size (`https://docs.blender.org/manual/en/5.1/render/output/properties/format.html`):

> "Resolution X, Y — The number of pixels horizontally and vertically in the image."
> "Percentage — Slider to reduce or increase the size of the rendered image relative to the Resolution values."
> "Render Region — Renders just a portion of the view instead of the entire frame."

Cycles sampling (`https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html`):

> "Render (Max) Samples — The number of paths to trace per pixel in the final render. A higher number results in a cleaner image at the cost of a longer render time."
> "Time Limit — Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit."
> "Noise Threshold — The threshold to decide whether to continue sampling a pixel. Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times."
> "Use GPU [denoising] — Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled."
> "Light Threshold — Probabilistically terminates light samples when the light contribution is below the threshold. This avoids wasting time on lights that contribute little to the image."
> "Scrambling Distance — A technique that reduces the randomness between pixels in an attempt to improve GPU rendering performance, at the cost of potential artifacts."
> "Sample Subset — Only render a subset of the samples. [...] A typical use case is to distribute the rendering of a single frame over multiple machines."

Cycles volumes (`https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/volumes.html`): "Render time can be reduced by increasing the step size, at the cost of […]" and "Increase to reduce render time, at the cost of losing detail."

Cycles subdivision (`https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/subdivision.html`): "while higher values use less memory."

EEVEE sampling (`https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html`): "TAA is sample based so the more samples the more aliasing is reduced at the cost of performance"; "Steps — Number of shadow map sample per shadow ray. Higher step count results in softer shadows but have a higher cost"; "Volumetric Shadows — […] This is a very computationally expensive option"; "Light Threshold — […] Lower values improve performance."

EEVEE volumes (`https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/volumes.html`): "To achieve this, EEVEE uses several 3D textures which have a high video memory usage."

EEVEE ray tracing (`https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/raytracing.html`): "Lower options will be faster and use less memory but will produce blurrier results."

Object-level, Cycles (`https://docs.blender.org/manual/en/5.1/render/cycles/object_settings/object_data.html`): "Controls accuracy of deformation motion blur, more steps uses more memory."

**Not covered.** I have not established that any list above is complete for the wide reading — I cannot, because the wide reading has no boundary the sources draw. That the narrow list (1a–1d) is complete for 5.1 is **inferred, and the step is mine**: the manual's three Performance pages and Cycles' own preset definition name the same settings, and I have taken that agreement as meaning nothing was left out. No source says either of them is a complete list. I have not checked settings belonging to third-party render engines, which the manual says exist ("More renderers from third-party developers are available as add-ons", `https://docs.blender.org/manual/en/5.1/render/introduction.html`). I have not covered Freestyle, the compositor, or the video sequencer, except where they appear on a Performance panel. Nothing here tells you *how much* any setting changes demand by, in any unit.

**Outside the interface.** Every setting in 1a, 1b and 1e is a property saved in the blend-file, so it applies whatever starts the render; the sources do not say any of them behaves differently in background mode. Two carry a real caveat, both documented. First, EEVEE: "Headless rendering is not supported on headless Windows systems" (`https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html`), so on a Windows machine with no display, EEVEE's settings are moot because EEVEE cannot render there at all. Second, the Preferences settings of 1c: when Blender is used as a Python module, "The Python module behaves as if `--factory-startup` was passed as a command line argument", and the user's preferences are ignored unless explicitly loaded (`https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html`). The switches of 1d exist *only* outside the interface. Whether `-t` on the command line overrides `threads_mode`/`threads` stored in the file is **not stated anywhere I searched** — see the hand-test list.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

**Operating system.** Several items are explicitly not universal, and the sources say so. `--qos` is "(Windows only)". `-con, --start-console` is "(Windows only)" and `--no-window-frame` "(Linux only)" — neither bears on render demand, but they show the manual does mark platform limits when they exist. Shader Compilation Method and its thread count are "not available on macOS and requires using OpenGL backend". MetalRT is Apple-only, HIP RT is AMD-only, Embree on GPU is "Only supported with oneAPI rendering devices", and Distribute Memory Across Devices is "Currently only NVLink on NVIDIA GPUs is supported". The GPU backends themselves are split by platform: "CUDA is supported on Windows and Linux", "OptiX is supported on Windows and Linux", "HIP is supported on Windows and Linux", "oneAPI […] supported on Windows and Linux", "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features" (`https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`). Threads, Tile Size, Persistent Data and the BVH options are not marked platform-specific anywhere I looked; the sources do not say they behave identically on every system either.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.1/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 3 — whether `-t` changes how much of the processor a Cycles render actually uses.** Done to a file built from a script and saved: Cycles, Device CPU, 320 by 240, 256 samples, seed 7, denoising off, adaptive sampling off, Threads Mode Auto-Detect; and to a copy of it with Threads Mode Fixed and Threads 1. Six runs of `/usr/bin/time -v blender -b --factory-startup <file> -f 1 -o <folder>/x_ -F OPEN_EXR -x 1`, varying only `-t`. What came back:

```
auto-detect, no -t : Percent of CPU 1600%, wall 0:02.15, peak 302964 kb
-t 1               : Percent of CPU 99%,   wall 0:33.82, peak 294632 kb
-t 2               : Percent of CPU 186%,  wall 0:18.19, peak 292904 kb
-t 0               : Percent of CPU 1217%, wall 0:02.83, peak 302496 kb
Fixed 1 in the file: Percent of CPU 90%,   wall 0:37.49, peak 301580 kb
Fixed 1 with -t 8  : Percent of CPU 633%,  wall 0:05.42, peak 293560 kb
```

**Observed.** The argument reaches the renderer: one thread takes about fifteen times as long as all of them, two threads about half of that, and `-t 0` matches the unrestricted run. The file's own Fixed 1 also reaches it. Note what the processor percentage does and does not show here: with the file asking for one thread, this version reported 90% while still taking the single-thread wall-clock time.

**Hand test 15 — which of the settings named in one version's findings exist in the others.** Done by walking `bl_rna.properties` of `scene.render`, `scene.cycles`, `scene.eevee`, `scene.display` and the Cycles preferences in all four researched versions and comparing the five lists. This version's dump holds 274 properties. Every name that is not in all four:

```
Absent here and present in 5.2 only:
  scene.render.use_texture_cache, scene.render.use_auto_generate_texture_cache,
  scene.cycles.texture_resolution, scene.cycles.texture_resolution_render,
  scene.cycles.debug_use_texture_cache_eviction, scene.cycles.debug_texture_cache_preserve_unused,
  scene.render.anisotropic_filter, scene.render.save_output, scene.cycles.use_pixel_jitter
Present here and in 5.2, absent from 4.5 and 5.0:
  scene.eevee.direct_light_intensity, scene.eevee.indirect_light_intensity
Present in 4.5 only, gone by here:
  scene.cycles.feature_set; scene.eevee.gtao_distance, gtao_quality, use_gtao; the eleven bake settings
Present here and in 4.5 and 5.0, gone by 5.2: scene.eevee.fast_gi_thickness_far
Values that differ: scene.render.compositor_device reads 'CPU' here and 'GPU' in 5.2;
  the Cycles preference use_hiprt reads True here and in 5.2, and False in 4.5 and 5.0.
```

**Observed.** Every other name is present in all four. *Corrected by the citation check, 2026-08-07:* the count of twenty-eight is right but this list held only twenty-seven. The missing name is **`scene.eevee.fast_gi_thickness_far`, present in 4.5, 5.0 and 5.1 and absent from 5.2** — a pattern this list had no heading for, which is how it fell out. The check re-ran the sweep across all four Blenders on the rented Linux machine and got the same twenty-eight names with that one added. Also noted: the property totals quoted here cannot be reproduced from the procedure as written, because it does not say whether `rna_type` is counted; the set difference, which is what this test rests on, does reproduce. Where a setting is named in one version's findings and not another's, that is now settled one way or the other by this rather than left as a silence.

**Hand test 25 — what the Cycles Performance presets write.** Done by finding the preset files under Blender's own `presets/cycles/performance/` folder, recording every property of `scene.render` and `scene.cycles`, applying each preset with `bpy.ops.script.execute_preset(filepath=<the file>, menu_idname="CYCLES_PT_performance_presets")` — the operator's arguments read off the running Blender, not guessed — and listing what changed. What came back:

```
PRESET-FILES ['.../presets/cycles/performance/Lower_Memory.py', '.../Faster_Render.py', '.../Default.py']
OPERATOR-ARGS ['filepath', 'menu_idname', 'rna_type']
---- applying Default.py        APPLIED-OK   NOTHING CHANGED
---- applying Faster_Render.py  APPLIED-OK
     CHANGED cycles.debug_bvh_time_steps      0 -> 2
     CHANGED cycles.debug_use_spatial_splits  False -> True
     CHANGED render.use_persistent_data       False -> True
---- applying Lower_Memory.py   APPLIED-OK
     CHANGED cycles.debug_bvh_time_steps      2 -> 0
     CHANGED cycles.debug_use_compact_bvh     False -> True
     CHANGED cycles.debug_use_spatial_splits  True -> False
     CHANGED cycles.tile_size                 2048 -> 512
     CHANGED render.use_persistent_data       True -> False
```

**Observed.** Nothing outside the Performance settings changed. The scene started at its defaults, so a preset writing a value the scene already held shows as no change; what this settles is that no property beyond those five moved.

**Hand test 26 — what `--cycles-print-stats` prints.** Done by `blender -b --factory-startup scene.blend -o <folder>/g_ -F OPEN_EXR -x 1 -f 1 -- --cycles-print-stats`. Statistics were printed: a breakdown by kind of work (`Intersect Closest`, `Intersect Shadow`, `Shade Light`, `Shade Shadow`, `Shade Volume` and so on, each with a total and a self time), then shader statistics, then object statistics. **No line names the device that rendered.** A search of the whole output for the words device, CUDA, OptiX and GPU returned only ordinary progress lines.

**Observed.** Anything that would read the device off this output cannot settle which device rendered.

**Hand test 7 — what Blender does when the file asks for the graphics card and none can be reached.** Done to a file saved with Cycles as the engine and Device set to **GPU Compute**, on this machine whose card was unreachable. `blender -b --factory-startup gpu.blend -o <folder>/g_ -F OPEN_EXR -x 1 -f 1 -- --cycles-print-stats`, with the Cycles backend preference at its factory value of None. **Exit code 0, and `g_0001.exr` written, 309998 bytes.** A search of the whole output for device, CUDA, OptiX, GPU, error, warning, fall and CPU returned only ordinary progress lines — nothing named a missing card. *Flagged by the citation check, 2026-08-07:* **this half of the test could not have failed in the direction that matters.** "Blender did not complain" and "Blender did not use the card" are different things, and searching printed output cannot tell them apart — hand test 26 establishes that Blender never names the device that rendered, which is exactly why looking for it cannot work. What settles the point is which picture came out, and the citation check ran that on the repaired card: a file saved with Device GPU Compute, in a settings folder naming no backend, matched the processor render exactly (`IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`) and did not match the card render. So the conclusion holds, but it rests on hand test 28 and not on this. Sent to the hand tests to be rebuilt as a picture comparison. Repeated with `CUDA` saved in the preferences, the run again exited 0 and wrote the image, and printed one line about it:

```
00:00.423  cycles           | ERROR CUDA cuInit: Unknown error
```

**Observed.** A render started from outside, asking for the graphics card on a machine that has none it can use, produces the image and — unless the preference names a backend — says nothing at all.

**Hand test 19 — whether a Cycles Time Limit produces a usable image.** Done to a 640 by 480 Cycles scene at 64 samples. Rendered once with Time Limit 0, then again with `--python-expr "import bpy; bpy.context.scene.cycles.time_limit=0.3"`. What came back: no limit `Time: 00:01.52`; limit `Time: 00:00.41`. Both files 4923758 bytes. Compared by pixels, the two are not the same picture: `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.39908283948898315`.

**Observed.** A time limit writes a complete file of the usual size, holding a picture cut short rather than a blank or a missing one.

**Hand test 24 — whether Persistent Data changes the time or the memory of an animation.** Done by rendering the whole ten frames with `-a`, once with `use_persistent_data` off and once on, under EEVEE and under Cycles, timed by the operating system. One run of each, on a shared machine:

```
EEVEE, ten frames, Persistent Data off : 78.12 s wall, 391% cpu, 2202156 kb peak
EEVEE, ten frames, Persistent Data on  : 79.73 s wall, 385% cpu, 2199588 kb peak
Cycles, ten frames, Persistent Data off: 21.59 s wall, 1708% cpu, 352396 kb peak
Cycles, ten frames, Persistent Data on : 21.26 s wall, 1758% cpu, 332896 kb peak
```

**Observed.** On a scene this small and a machine this busy, one run of each cannot separate the time differences from noise. What can be said is that peak memory did not go up when Persistent Data was switched on; under Cycles it came out lower.

**Hand test 16, cost side — what tile size and the acceleration-structure options cost.** Same 640 by 480 scene, one run each, shared machine:

```
base      2.39 s wall, 1428% cpu, 316832 kb peak
tile 64   3.63 s wall, 1098% cpu, 331536 kb peak
tile 8192 2.89 s wall, 1261% cpu, 315944 kb peak
tiling off 2.49 s wall, 1479% cpu, 317052 kb peak
compact BVH 2.69 s wall, 1440% cpu, 319128 kb peak
spatial splits 3.33 s wall, 1143% cpu, 315544 kb peak
```

**Observed.** Recorded as measured; a single run each on a busy machine is not enough to separate small differences from noise.

**Hand test 31 — the texture cache.** Done to a copy of the 640 by 480 Cycles scene with three 1024 by 1024 PNG images written next to it and wired into three materials, 52 kB of image files in all. First a render with nothing changed: `2.31 s wall, 349792 kb peak-rss`. Then the two settings were asked for by name:

```
HAS-use_texture_cache False HAS-use_auto_generate_texture_cache False
```

**Observed.** Neither setting exists in this version, so the rest of the test cannot be carried out here. Hand test 15 above establishes the same by a different route.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.1/` or `/root/render-demands/prefs-card-cuda/5.1/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 27, cost side — what the card costs and what the processor costs.** One file rendered three times, changing only which device did the work: once with Device CPU and the Cycles backend preference at None, once with Device GPU Compute and the backend CUDA, once with Device GPU Compute and the backend OptiX. In the two card runs only the card was ticked and the processor was left unticked. The file: Cycles, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, OpenEXR at full float with no compression. Timed by the operating system:

```
processor   : Cycles reported Time: 00:01.63, 1577% of a processor, peak memory 320544 kb
card, CUDA  : Cycles reported Time: 00:00.71, 82% of a processor, peak memory 456760 kb
card, OptiX : Cycles reported Time: 00:00.80, 81% of a processor, peak memory 640548 kb
```

**Observed.** Choosing the card rather than the processor changes what the render demands in three ways at once, all in the same direction in this version. It takes less time. It stops asking for the whole processor — about fifteen hundred per cent of one core drops to about one hundred per cent, which is to say the render stops being a processor job. And it asks for **more** ordinary memory, not less, because the card's own working memory is held in addition. This is one small scene on one busy machine; the shape is what is being recorded, not the seconds.

**Hand test 28, memory side — what the saved backend preference takes off the card even when the processor does the work.** Through every run of hand test 28 this Blender was watched for how much card memory it held, by asking the driver which processes held work on the card and how much each held — a reading about this process alone, so the other topic's Blender could not confuse it. On a heavier version of the same file, 1024 samples:

```
backend preference None, render on the processor : never held any card memory at all
backend preference CUDA, render on the processor : held card memory, peak 248 MiB
render on the card                               : held card memory, peak 1652 MiB
```

**Observed.** A saved backend preference costs card memory on its own. With the preference naming CUDA, this Blender took about 248 MB of the card for the whole length of a render that ran entirely on the processor; with the preference at None it took none. That is a cost the preference exacts even in a run where it makes no difference at all to which device renders.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.1/`.

**Asked which compute backends it has, this Blender offers two and refuses the rest:**

```
TYPE NONE  DEVICES []
TYPE METAL DEVICES [('Apple M1 Pro (GPU - 14 cores)', 'METAL', True), ('Apple M1 Pro', 'CPU', False)]
TYPE CUDA  COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "CUDA" not found in ('NONE', 'METAL')
```

and `OPTIX`, `HIP` and `ONEAPI` were refused in exactly the same words. So on this operating system the backend setting has only two values it will take.

**One limit runs through everything below, and it is why not one number here is a time or a memory figure.** The Mac was shared with other topics' work while these ran — another topic's Blender was found running on it partway through — so **nothing was timed and nothing measured memory**. The rented Linux machine carries this round's timings, and it is the kind of computer a Smoozy render actually happens on. Where a test's only possible evidence was a time or a memory figure, it was not run here at all, and that is said under the question it would have answered. Every picture comparison below compares the pixels and never the saved file, because Blender writes the time of day into every image it saves.

The full list of tests, with the procedure for each, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 1 — what engines this build offers.** `blender -b -E help`, with no file:

```
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
EXIT=0
```

The command returned cleanly. Without `-b` the same three names were printed and the run also exited 0 — this machine has a screen, where the Linux machine has no display server and the same command fails there before reaching the list.

**Observed.** This build offers three engines and names EEVEE `BLENDER_EEVEE`, the same name the rented Linux machine got from it.

**Hand test 15 — which settings exist on this operating system.** The same sweep as on Linux: every property of `scene.render`, `scene.cycles`, `scene.eevee`, `scene.display` and the Cycles preferences, read out of a running Blender and compared across all four researched versions on this machine. This version's dump holds **273** properties.

**Observed.** The list of names that are not in all four came out **exactly the same on macOS as on Linux** — the same twenty-eight names, on the same versions. So none of the differences between the versions' findings is an operating-system difference. Two values differ between versions here as well: `scene.render.compositor_device` reads `CPU` in 4.5, 5.0 and 5.1 and `GPU` in 5.2, and the Cycles preference `use_hiprt` reads `False` in 4.5 and 5.0 and `True` in 5.1 and 5.2 — on a Mac, where HIP is not one of the backends this build will accept at all.

**Hand test 7 — a file that asks for the graphics card, with the backend preference naming nothing.** A file saved with Cycles as the engine and Device **GPU Compute**, run against a settings folder whose saved backend is `NONE`: `blender -b gpu.blend -o <folder>/g_ -F OPEN_EXR -x 1 -f 1 -- --cycles-print-stats`. **Exit code 0, and the image was written.** A search of the whole output for the words device, CUDA, OptiX, GPU, Metal, error, warning, fall and CPU returned only ordinary progress lines — nothing named a device, and nothing said the card had not been used.

**Observed.** Hand test 28 below settles what actually happened: that run rendered **on the processor**, and never touched Apple's graphics at all. So a file asking for the card, in a settings folder naming no backend, quietly does not get the card — and says nothing about it either way. That is the same result the Linux machine gave when its card was unreachable, now got on a machine whose graphics work perfectly, which shows the result belongs to the preference and not to a broken driver.

**Hand test 25 — what the Cycles Performance presets write.** Done exactly as on Linux, with the operator's arguments read off the running Blender rather than guessed:

```
OPERATOR-ARGS ['rna_type', 'filepath', 'menu_idname']
---- applying Default.py        APPLIED-OK   NOTHING CHANGED
---- applying Faster_Render.py  APPLIED-OK
     CHANGED cycles.debug_bvh_time_steps      0 -> 2
     CHANGED cycles.debug_use_spatial_splits  False -> True
     CHANGED render.use_persistent_data       False -> True
---- applying Lower_Memory.py   APPLIED-OK
     CHANGED cycles.debug_bvh_time_steps      2 -> 0
     CHANGED cycles.debug_use_compact_bvh     False -> True
     CHANGED cycles.debug_use_spatial_splits  True -> False
     CHANGED cycles.tile_size                 2048 -> 512
     CHANGED render.use_persistent_data       True -> False
```

**Observed.** The same five properties, and nothing outside the Performance settings. The two machines agree.

**Hand test 26 — what `--cycles-print-stats` prints.** Statistics were printed: a breakdown by kind of work, then shader statistics, then object statistics. **No line names the device that rendered.** A search of the whole output for device, CUDA, OptiX, GPU and Metal returned only ordinary progress lines of the form `Updating Scene BVH | Copying BVH to device`.

**Observed.** The same as on Linux. Anything that would read the device off this output cannot settle which device rendered — and on this machine that holds for a card render too: hand test 28's Metal runs printed no word about Metal anywhere.

**Hand test 21 — Render Single Layer from the command line.** A file with two view layers, `ViewLayer` and `Second`, saved once with the box clear and once with it ticked, each rendered from the command line. Both view layers were named in Blender's own progress lines in **both** runs.

**Observed.** The setting is not honoured from outside, exactly as on Linux and exactly as the manual's note says.

**Hand test 12 — Resolution Percentage in a command-line render.** The file is 160 by 120 at 37 per cent. The image that was written:

```
IMAGE r_0001.png SIZE (59, 44)
```

**Observed.** The percentage reaches a render started from outside. The Linux machine wrote the same size.

**Hand test 19 — what a Cycles Time Limit produces.** The 640 by 480, 64-sample scene rendered once with no limit and once with `time_limit` set to 0.3 seconds. Both files came out **4923797 bytes**, and the two pictures are not the same picture:

```
no limit vs a 0.3-second limit : IDENTICAL-PIXELS False MAX-ABS-DIFF 3.168557643890381 NUM-DIFFERING-CHANNEL-VALUES 921600 of 1228800
```

**Observed.** A time limit writes a complete file of the usual size holding a picture cut short, not a blank one and not a missing one. How long each render took was **not measured**, because the machine was not idle; the Linux run carries the times.

**Hand test 31 — the texture cache.** A copy of the same scene with three 1024 by 1024 PNG images written beside it and wired into the three materials.

```
HAS-use_texture_cache False HAS-use_auto_generate_texture_cache False
```

**Observed.** Neither setting exists in this version on macOS either, so the rest of the test cannot be carried out here. The same file rendered twice with nothing changed gave `IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`, and no folder appeared beside the images.

**Three tests were not run on this machine at all, and the reason is the same for all three.** Hand test 3 (whether `-t` changes how much of the processor a Cycles render actually uses), hand test 23 (whether the thread count changes anything under EEVEE or Workbench) and hand test 24 (whether Persistent Data changes the time or the memory of an animation) have no evidence except a time or a memory figure. The Mac was shared with other topics' work throughout, so a figure taken here would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. **What they leave open on macOS is left open and is not guessed at.**

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.1/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — which picture comes out when the file asks for the graphics card and the settings folder names no backend. This is hand test 7 rebuilt, because hand test 7 could not have failed.** That test searched Blender's printed output for complaints about a missing card and found none; "Blender did not complain" and "Blender did not use the card" are different things, and hand test 26 in this same document is why no search of output can tell them apart. **What changed is the evidence: the question is now settled by which picture came out.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render.

Five renders, none of them with `--factory-startup`, so the saved preferences were read every time: `blender -b <file> -o <folder>/<tag>_ -F OPEN_EXR -x 1 -f 1 --`. (a) the file saying CPU, no argument — the **processor reference**; (b) the same again, unchanged; (c) the file saying CPU with `-- --cycles-device CUDA` — the **card reference**; (d) the same again, unchanged; (e) **the test itself**, the file saying GPU Compute with the backend preference naming nothing.

The two controls first, and the size of the gap this has to judge against:

```
processor run 1 vs run 2 : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0                  DIFFERING 0 of 1228800
card run 1 vs run 2      : IDENTICAL-PIXELS False MAX-ABS-DIFF 3.337860107421875e-06    DIFFERING 467847 of 1228800
processor vs card        : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.061817049980163574     DIFFERING 600257 of 1228800
```

and then the test:

```
(e) file GPU Compute, backend preference NONE, no argument
    against the processor reference : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0              DIFFERING 0 of 1228800
    against the card reference      : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.061817049980163574  DIFFERING 600257 of 1228800
```

**Observed.** A file asking for GPU Compute, in a settings folder naming no backend, renders on the **processor** — not approximately but to the very same numbers — and misses the card by the whole device gap, which is some twenty thousand times the difference between two runs on the same device. The test could have come out the other way and did not. A second reading says the same without any reference to the pictures: what each run held on the card, this Blender alone.

```
              Cycles reported   % of a processor   card memory held by this run
processor        Time: 00:01.64      1577%          none at all
card (CUDA)      Time: 00:01.27      65%            1568 MiB
file GPU, no backend named   Time: 00:02.48   1163%   none at all
```

The wall-clock times were 0:02.27, 0:02.07 and 0:03.11; they are a busy machine's times and carry nothing on their own. **The card memory carries it: the run never touched the card.** The five renders were made twice, ten minutes apart, and both passes gave pixel-identical pictures (`IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`), so this is not one moment's accident. **Still owed on the Creator's Mac**, where the question is sharper because the graphics there work perfectly.

**Hand test 34 — which Cycles backends this build accepts, which of them have a device behind them, and what a render does when the argument names one that does not.** The citation check sent this back because the two machines had been asked different questions — the Mac was asked which values the setting will take, the Linux machine which backends have a card behind them. Both are asked here, of this machine, in the same sitting, and kept apart. **The device-listing half is rebuilt**: each value is set in a **fresh Blender** rather than one after another, and the list is read with `get_devices_for_type`, the call that takes the backend as its argument, with the plain `p.devices` collection printed beside it. This version printed:

```
TYPE NONE ACCEPTED
TYPE NONE DEVICES-FOR-TYPE []
TYPE NONE P-DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE CUDA ACCEPTED
TYPE CUDA DEVICES-FOR-TYPE [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE CUDA P-DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX ACCEPTED
TYPE OPTIX DEVICES-FOR-TYPE [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX P-DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE HIP ACCEPTED
TYPE HIP DEVICES-FOR-TYPE []
TYPE HIP P-DEVICES [('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE ONEAPI ACCEPTED
TYPE ONEAPI DEVICES-FOR-TYPE []
TYPE ONEAPI P-DEVICES [('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE METAL COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')
```

**The other three versions printed exactly the same lines.**

**Observed, and the two questions come apart.** This machine **takes five of the six names and refuses one**, and the refusal names the five it will take. Only **two** of the five have anything behind them. And the `NONE` line shows why the old reading could not settle it: with the backend set to a value under which nothing can render, the plain `p.devices` collection holds the card twice over while `get_devices_for_type` holds nothing — so `p.devices` is not a list of what the chosen backend offers.

**A third half was added, because a device list is still Blender talking about itself.** Each backend was given to a real render of the file that says CPU, with the preference naming nothing, and settled by which picture came out against the two references above:

```
--cycles-device CPU    exit 0, image written : matched the PROCESSOR reference exactly, MAX-ABS-DIFF 0.0
--cycles-device CUDA   exit 0, image written : matched the CARD reference, MAX-ABS-DIFF 2.86102294921875e-06; against the processor 0.06181681156158447
--cycles-device OPTIX  exit 0, image written : matched NEITHER — against the processor 0.061817049980163574, against the CUDA card 0.05315208435058594
--cycles-device HIP    exit 1, no image written
--cycles-device ONEAPI exit 1, no image written
--cycles-device METAL  exit 1, no image written
```

OptiX being a third picture is not a fault in the test: hand test 27 already found the two backends on the same card disagreeing with each other. What matters is that it is not the processor's picture. What the three refused runs printed, word for word:

```
00:00.383  reports          | ERROR Found no Cycles device of the specified type
```

and `HIP` printed one line more, before it:

```
00:00.385  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
00:00.388  reports          | ERROR Found no Cycles device of the specified type
```

Card memory, this Blender alone: `CPU`, `HIP`, `ONEAPI` and `METAL` held **none at all**; `CUDA` held 268 MiB and `OPTIX` 1646 MiB. (The card was sampled five times a second and these renders last about two seconds, so a peak can be missed — the reading that carries weight is nothing against something.)

**Observed.** Naming a backend this build accepts but has no hardware for does **not** quietly fall back to the processor: the render stops, writes no image, and says `Found no Cycles device of the specified type`. So a program that writes `HIP` or `ONEAPI` into this setting on this machine gets no error when it writes it, and finds out at render time. **Still owed on the Creator's Mac** for the device-listing half and the render half.

**Hand test 35 — the property sweep written out exactly, so its totals can be checked.** The citation check sent hand test 15 back over its numbers, not its finding: three sittings had reported three different totals and the procedure as written did not say whether each block's own `rna_type` entry is counted. The sweep was run again with that stated — `blender -b --factory-startup <file> --python <script>`, walking `bl_rna.properties` of five blocks named exactly (`scene.render`, `scene.cycles`, `scene.eevee`, `scene.display`, and the Cycles add-on's preferences), counting each block twice, once including `rna_type` and once excluding it. This version: `scene.render` 98 / 97, `scene.cycles` 106 / 105, `scene.eevee` 55 / 54, `scene.display` 10 / 9, the Cycles preferences 9 / 8 — **278 counting `rna_type`, 273 not counting it**.

**Observed.** The three totals on the record are now told apart. Counting `rna_type` gives 290, 276, 278 and 286 across the four versions, which is the citation check's figure for 4.5 exactly. Not counting it gives 285, 271, 273 and 281, which is the Mac sitting's figures exactly. The first Linux sitting's 286, 272, 274 and 282 are **neither** — one more than the not-counting figure in every version — so **they cannot be reproduced from any stated rule and should be read as decoration**. This version's earlier figure was 274. **The finding itself reproduced exactly**: the set of property names not present in all four versions came back as the same twenty-eight, on the same versions, `scene.eevee.fast_gi_thickness_far` among them — present in 4.5, 5.0 and 5.1 and absent from 5.2 — so the count and the written-out list now agree.



## 2. For each of them, what does it change about what the render produces?

**LABEL: mixed — documented for some, "not in the sources" for the most important ones.** I split them.

### 2a. Settings the sources say nothing about, output-wise

For **Threads Mode, Threads, Tile Size, Use Spatial Splits, Use Curves BVH, Use Compact BVH, BVH Time Steps and Persistent Data**, every description I found is phrased purely in terms of time and memory. Not one of them says whether the finished image is the same or different. Read them again and notice what is missing: "Split BVH primitives by this number of time steps to speed up render time at the expense of memory" — time and memory, no word about the picture. "Use compact BVH structure (uses less ram but renders slower)" — same. "Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage while performing other tasks in Blender" — same.

**The sources do not state that these leave the image unchanged, and I will not say that they do.** It is the natural assumption and it may well be right, but nothing I read carries it, and assuming it is exactly the kind of step that turns a finding into a guess. **Label for that part: not in the sources.**

Where I looked, and for what: I read all of `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html` and the `CyclesRenderSettings` definitions of all five Cycles properties in `properties.py`, plus the `RenderSettings.threads`, `threads_mode` and `use_persistent_data` entries in the API documentation. I then searched the whole 5.1 manual and the whole 5.1 API documentation for the words *deterministic*, *identical*, *reproducible*, *same result*, *bit-for-bit*, and searched the render section for *render time*, *memory usage*, *memory consumption*, *more memory*, *less memory*, *reduce memory*, *uses more ram*, *less ram*, *faster*, *slower*, *performance*. Nothing tied any of these eight settings to a change in output. I ruled out the sampling, film and light-paths pages as the wrong place to look, since none of these eight settings appear on them.

One near-miss worth recording, because it is the closest the sources come. Cycles' own interface file warns that tiling interacts with path guiding:

> `if render_area > tile_area and render_size_x <= 8192 and render_size_y <= 8192:`
> `    layout.label(text="May work poorly with render tiling", icon='INFO')`

File: `Blender Sources/cycles_5.1/ui.py`, in `CYCLES_RENDER_PT_sampling_path_guiding.draw`, guarded by `if cscene.use_guiding:`. The lines just above compute the effective tile size and the render area:

> `# Calculation matches TileManager::compute_render_tile_size and`
> `# Session::get_effective_tile_size`
> `if cscene.tile_size < 128:`
> `    tile_size = cscene.tile_size`
> `else:`
> `    tile_size = (cscene.tile_size + 128 - 1) & ~(128 - 1)`
> `    tile_size = min(tile_size, 8192)`
> `tile_area = tile_size ** 2`
> `render_scale = scene.render.resolution_percentage / 100.0`
> `render_size_x = int(scene.render.resolution_x * render_scale)`
> `render_size_y = int(scene.render.resolution_y * render_scale)`
> `render_area = render_size_x * render_size_y`

**Label: observed** that this warning exists and under exactly these conditions. **Inferred, and the step is mine:** since a warning that path guiding "may work poorly" is a warning about quality, Tile Size can affect the output of a render that has path guiding switched on. The step a reader has to supply is that "work poorly" means a worse image rather than a slower one; the file does not say which.

One more thing sits in those lines, and it needs care. **Observed:** the interface file computes a tile size by rounding the set value up to a multiple of 128 above 128 and capping it at 8192, and a comment there says this calculation "matches" two named parts of the renderer. **Inferred, and the step is mine:** that the render therefore uses a rounded number rather than the one that was set. The step a reader has to supply is that the interface's own copy of the calculation really is what the renderer does — the only thing saying so is that comment, and what a file draws is not what a render does.

### 2b. Device — the sources do say the output can differ

> "Note
>
> GPU rendering may have limitations compared to CPU rendering, depending on the backend."

(`https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html`)

> "Limitations
>
> Path Guiding is not supported on any GPU.
>
> Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

(`https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`)

> "Note
>
> Path guiding is only available when rendering on a CPU."

(`https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html`)

And on CPU mode: "This mode is compatible with all Cycles features."

**Label: documented** for what the quotes say: path guiding is unavailable on every GPU, and Open Shading Language is available only on OptiX, while CPU "is compatible with all Cycles features". **Inferred, and the step is mine:** that a scene relying on one of those features therefore comes out different on a GPU. The step a reader supplies is that a feature being unavailable changes the picture rather than stopping the render.

Whether a scene using none of those features renders to an identical image on CPU and GPU is **not answered here**, and I did not record a search aimed at that question — so treat it as an open question rather than an established silence.

### 2c. The wider set — where the source states the output effect plainly

- **Texture Limit**: "Automatically scales textures down so that they are no larger than the values chosen." The rendered textures are lower resolution.
- **Max Subdivision**: caps "Maximum number of subdivision by the Subdivision Surface modifiers" — geometry is coarser.
- **Child Particles**: "Show only a subset of all child hairs and particles" — hair and particles are missing.
- **Camera Cull / Distance Cull**: "Automatically culls objects" — objects are absent.
- **Resolution X, Y and Percentage**: the image has a different pixel size.
- **Render Region**: "Renders just a portion of the view instead of the entire frame."
- **Max Samples / Noise Threshold**: noise. "A higher number results in a cleaner image at the cost of a longer render time"; "lower values meaning better quality but longer render times".
- **Time Limit**: "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet" — a noisier image than asked for.
- **Scrambling Distance**: "at the cost of potential artifacts". And: "Not compatible with the Blue-Noise sampling pattern."
- **Denoise Use GPU**: nothing said about output differing between CPU and GPU denoising, only that GPU "is significantly faster […] but requires additional GPU memory".
- **Sample Subset**: "it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets – otherwise, the subsets will have incompatible noise and combining them will give a worse result."
- **EEVEE Shadow Pool**: "Increasing the size might fix the Shadow buffer full error." And, from the EEVEE limitations page, "Active light probe volumes must fit inside the Light Probes Volume Memory Pool." **Inferred, step stated:** since a pool that is too small produces an error and probe volumes must fit inside it, a pool set too small costs you shadows or probe lighting in the image. The sources describe the failure but do not describe what the picture then looks like.
- **EEVEE / Workbench High Quality Normals**: "can improve visual quality for dense meshes with high frequency textures at the cost of memory."
- **Compositor Precision**: "Auto: Use full precision for final renders, half precision otherwise" — so for a final render, Auto and Full agree.
- **Denoise Node quality (Final Quality)**: "High: Produces the highest quality output at the cost of long processing times. […] Fast: Produces an output quickly at a noticeable cost of quality."

Every quote in this list except the last two was cited with its address earlier in this document. No address is recorded here for the Compositor Precision quote or the Denoise Node quality quote.

**Not covered.** For the eight settings in 2a — the heart of the "Performance" panel — this answer establishes that the sources are silent, not that the image is unchanged. Anyone who needs to know that must test it (see the hand-test list). Nothing here quantifies any quality change. I have not covered third-party engines. I have also **not answered this question for the Preferences settings of 1c** — Distribute Memory Across Devices, Embree on GPU, HIP RT, MetalRT, Shader Compilation Method and its thread count — **nor for the command-line switches of 1d** other than `--cycles-device`, which is dealt with below. What those change about what a render produces is not covered here at all.

**Outside the interface.** These are all file-stored properties and the sources give no reason to expect a different effect on output when a render is started from outside. One documented exception bears on this question directly: the Preferences-level device choice is ignored when Blender runs as a Python module unless deliberately loaded, so a scene set to "GPU Compute" may fall back differently there — the sources do not say what happens then. `--cycles-device` is described by Cycles' own file as "overriding user preferences and the scene setting". **Inferred, and the step is mine:** if the switch behaves as that help text describes, a render driven from the command line can produce the GPU-limited output described in 2b even when the file says CPU. The step a reader supplies is that a help text describes what the render actually does; nothing I read confirms it, and there is a hand test for it.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Operating system.** The output effects quoted above are not marked platform-specific. But because 2b's answer depends on which GPU backend is in use, and backends are platform-split (OptiX on Windows and Linux only; Metal on Apple Silicon only; and so on, per `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`), *which* limitations apply to a GPU render does depend on the operating system. OSL is the clearest case: "Open Shading Language is only supported for OptiX", and OptiX is not available on macOS.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.1/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 16 — whether thread count, tile size, Use Tiling, the three acceleration-structure options, BVH Time Steps, Persistent Data or High Quality Normals change the picture.** Done to a scene built from a script and saved: Cycles, Device CPU, 640 by 480 at 100 per cent, 64 samples, Seed 7, Use Animated Seed off, Denoise off, Adaptive Sampling off, Max Bounces 4, stamping off, saved as OpenEXR at full float with no compression. Twelve renders of frame 1, each changing exactly one thing — nothing; `-t 1`; `-t 8`; Tile Size 64; Tile Size 8192; Use Tiling off; Use Spatial Splits on; Use Compact BVH on; Use Curves BVH off; BVH Time Steps 4; Persistent Data on; High Quality Normals on — then each compared against the base **by pixels**, by loading both images and subtracting them channel by channel. What came back, for every one of the eleven pairs:

```
IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
```

Two control renders were made to show the comparison can tell a difference at all — without them no result from it would mean anything:

```
seed 7 → 8: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.30967605113983154 NUM-DIFFERING-CHANNEL-VALUES 862779 of 1228800
64 samples → 63: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.04032999277114868 NUM-DIFFERING-CHANNEL-VALUES 862791 of 1228800
```

**Observed.** None of those eleven changes altered a single channel value, on this scene, on this machine, in this version.

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench as well as Cycles.** Done to a scene with a Subdivision Surface modifier at render level 4. Six renders: each of the three engines with Simplify off, and each with `use_simplify` on and `simplify_subdivision_render` set to 0, compared by pixels:

```
CYCLES            : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.12282812595367432 NUM-DIFFERING-CHANNEL-VALUES 28221 of 76800
BLENDER_EEVEE     : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.105712890625      NUM-DIFFERING-CHANNEL-VALUES 3564 of 76800
BLENDER_WORKBENCH : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.0909423828125     NUM-DIFFERING-CHANNEL-VALUES 3397 of 76800
```

**Observed.** All three engines changed.

**Hand test 18 — whether the Viewport-labelled settings touch a render started from outside.** Done to the same 640 by 480 scene: one render with nothing changed, one with Cycles Viewport Samples 1, viewport Dicing Rate 100, viewport Texture Limit 128, Simplify's viewport Max Subdivision 0, Preview Pixel Size 8 and EEVEE's Viewport Samples 1, every Render counterpart left alone. What came back:

```
VIEWPORT-SET 1 100.0 128 0 8
IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
```

**Observed.** None of the six reached the render. The comparison is the same one whose controls are quoted above.

**Hand test 12 — whether Resolution Percentage reaches a render started from the command line.** Done to a file saved at 160 by 120 with Resolution Percentage 37. `blender -b --factory-startup many.blend -o <folder>/r_ -F PNG -x 1 -f 1`, then the written file loaded and its size printed: `IMAGE r_0001.png SIZE (59, 44)`.

**Observed.** The percentage is applied.

**Hand test 19 — what a Time Limit does to the picture.** Same scene rendered with Time Limit 0 (`Time: 00:01.52`) and with Time Limit 0.3 seconds (`Time: 00:00.41`). Both files 4923758 bytes; compared by pixels, `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.39908283948898315`.

**Observed.** A time limit changes the picture as well as the cost.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.1/` or `/root/render-demands/prefs-card-cuda/5.1/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 27 — whether the same file renders to the same picture on the processor and on the card.** One file, three renders, changing only the device: Device CPU with the backend preference at None; Device GPU Compute with the backend CUDA; Device GPU Compute with the backend OptiX. The file was built to leave nothing else free to move: Cycles, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, no path guiding, no Open Shading Language, four bounces, OpenEXR at full float with no compression. The three images were then compared **by pixels** — both loaded and subtracted channel by channel — and never by comparing the saved files. What came back:

```
processor vs card (CUDA)  : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.06814593076705933  DIFFERING 704950 of 1228800  MEAN-ABS-DIFF 0.0001558240328449756
processor vs card (OptiX) : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.06115788221359253  DIFFERING 704821 of 1228800  MEAN-ABS-DIFF 0.00016435282304883003
card CUDA vs card OptiX   : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.05027270317077637  DIFFERING 674189 of 1228800  MEAN-ABS-DIFF 0.0001329765800619498
```

The control, which is what makes those numbers mean anything: each of the three renders was run a second time, unchanged, and compared with its first run.

```
processor, run 1 vs run 2  : IDENTICAL-PIXELS True   MAX-ABS-DIFF 0.0
card CUDA, run 1 vs run 2  : IDENTICAL-PIXELS False  MAX-ABS-DIFF 7.152557373046875e-07  DIFFERING 615742 of 1228800
card OptiX, run 1 vs run 2 : IDENTICAL-PIXELS False  MAX-ABS-DIFF 7.152557373046875e-07  DIFFERING 614929 of 1228800
```

**Observed.** **The device changes the picture.** Which device did the work is not only a matter of what the render costs; the image that comes out is a different image. Two things say how much of a difference it is. It is small in size — the largest single channel value moves by about 0.0681 on a nought-to-one scale, and the average move is about a ten-thousandth — but it is spread over more than half of the image. And it is far above the noise of the machine: the processor repeats its own render exactly, and the card repeats its own render to within about a millionth, which is about the finest step a number of this kind can take, while the difference between processor and card is some tens of thousands of times larger than that. The two card backends do not agree with each other either. So a frame rendered on the processor and a frame rendered on the card are not interchangeable, and nor are frames from two different backends on the same card.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.1/`.

**Asked which compute backends it has, this Blender offers two and refuses the rest:**

```
TYPE NONE  DEVICES []
TYPE METAL DEVICES [('Apple M1 Pro (GPU - 14 cores)', 'METAL', True), ('Apple M1 Pro', 'CPU', False)]
TYPE CUDA  COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "CUDA" not found in ('NONE', 'METAL')
```

and `OPTIX`, `HIP` and `ONEAPI` were refused in exactly the same words. So on this operating system the backend setting has only two values it will take.

**One limit runs through everything below, and it is why not one number here is a time or a memory figure.** The Mac was shared with other topics' work while these ran — another topic's Blender was found running on it partway through — so **nothing was timed and nothing measured memory**. The rented Linux machine carries this round's timings, and it is the kind of computer a Smoozy render actually happens on. Where a test's only possible evidence was a time or a memory figure, it was not run here at all, and that is said under the question it would have answered. Every picture comparison below compares the pixels and never the saved file, because Blender writes the time of day into every image it saves.

The full list of tests, with the procedure for each, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 16 — whether the cost settings change the picture.** One 640 by 480 Cycles scene at 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, written to OpenEXR at full float with no compression and stamping off. Twelve renders, each changing exactly one thing, each compared with the base **by pixels**:

```
base vs t1 : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
base vs t8 : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
base vs tile64 : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
base vs tile8192 : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
base vs notile : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
base vs spatial : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
base vs compact : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
base vs nohair : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
base vs bvhsteps : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
base vs persist : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
base vs hqn : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
```

and the two controls, which are what say the comparison can tell a difference at all:

```
base vs seed8 (seed 7 -> 8):    IDENTICAL-PIXELS False MAX-ABS-DIFF 0.6618781089782715 NUM-DIFFERING-CHANNEL-VALUES 714819 of 1228800
base vs s63 (64 samples -> 63): IDENTICAL-PIXELS False MAX-ABS-DIFF 0.30008816719055176 NUM-DIFFERING-CHANNEL-VALUES 921587 of 1228800
```

**Observed.** Thread count, tile size, Use Tiling, the three acceleration-structure options, BVH Time Steps, Persistent Data and High Quality Normals all left the picture untouched, on a machine with a different processor from the one the same eleven were shown clean on. The rented Linux machine gave the same eleven identical results. **The two machines agree.**

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench.** A scene with a Subdivision Surface modifier at render level 4, rendered under each of the three engines with Simplify off, then again with Simplify on and Max Subdivision (render) at 0, each pair compared by pixels:

```
CYCLES              simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.35849255323410034 NUM-DIFFERING-CHANNEL-VALUES 37720 of 76800
BLENDER_EEVEE       simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.380859375 NUM-DIFFERING-CHANNEL-VALUES 10319 of 76800
BLENDER_WORKBENCH   simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.10693359375 NUM-DIFFERING-CHANNEL-VALUES 8580 of 76800
```

**Observed.** All three engines changed. Simplify's Render column is not Cycles-only in effect. The Linux machine gave the same three.

**Hand test 18 — whether the Viewport-labelled settings touch a render started from outside.** Six of them pushed to extremes — Cycles Viewport Samples 1, viewport Dicing Rate 100, viewport Texture Limit 128, Simplify's viewport Max Subdivision 0, Preview Pixel Size 8 and EEVEE's Viewport Samples 1 — with every Render counterpart left alone:

```
VIEWPORT-SET 1 100.0 128 0 8 1
base vs the six viewport settings : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
```

**One thing was added here that the Linux run did not do**, because the scene it used has no subdivision in it, so Simplify's viewport Max Subdivision had nothing to bite on and a "no change" from it would have meant nothing. Repeated on the scene that *does* carry a Subdivision Surface modifier, with Simplify switched **on** in both renders and only the **viewport** Max Subdivision moved from 6 to 0, the render Max Subdivision left at 6:

```
SIMPLIFY True viewport-max-subdiv 6 render-max-subdiv 6
SIMPLIFY True viewport-max-subdiv 0 render-max-subdiv 6
viewport max subdivision 6 vs 0, render max subdivision left at 6 : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 76800
```

**Observed.** No Viewport-labelled setting reached the render. The added run is the one that carries weight: hand test 17 shows that moving the **Render** Max Subdivision on this very file does change the picture, so the comparison was able to see a difference of exactly this kind and did not see one when only the Viewport number moved.

**Hand test 27 — whether the processor and Apple's own graphics render the same picture.** The same file rendered twice, changing only the device: once with Device CPU and the backend preference at `NONE`, once with Device GPU Compute and the backend `METAL`, with only the graphics device ticked and the processor left unticked. Compared by pixels:

```
processor vs card (Metal) : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.04524683952331543 NUM-DIFFERING-CHANNEL-VALUES 638882 of 1228800 MEAN 7.729685701936735e-07
```

**And the control, without which none of that means anything** — each of the two renders repeated, unchanged, and compared with its own first run:

```
processor run 1 vs run 2   : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0
card Metal run 1 vs run 2  : IDENTICAL-PIXELS False MAX-ABS-DIFF 2.86102294921875e-06 NUM-DIFFERING-CHANNEL-VALUES 246637 of 1228800
```

**Observed.** **The device changes the picture on this machine too.** More than half of the channel values differ, the largest by about three to five hundredths on a nought-to-one scale. The processor repeats its own render exactly; Apple's graphics do not, but only by about a millionth, which is some tens of thousands of times smaller than the gap between the two devices. The rented Linux machine found the same thing with an NVIDIA card through CUDA and through OptiX. **Two different makes of graphics processor, two operating systems, the same answer: choosing the device changes the image, not only what the image costs.** What it costs was not measured here; the Linux machine carries that.

**Hand test 20 — a start-up script setting Samples.** `--python-expr "import bpy; bpy.context.scene.cycles.samples=7"` against the 64-sample file:

```
SAMPLES-SET 7
the 64-sample base vs the 7-sample render : IDENTICAL-PIXELS False MAX-ABS-DIFF 1.968618392944336 NUM-DIFFERING-CHANNEL-VALUES 921600 of 1228800
```

**Observed.** The assignment reaches the render and changes the picture. The render time was not recorded here.

**Hand test 31 — the texture cache and the picture.**

```
HAS-use_texture_cache False HAS-use_auto_generate_texture_cache False
```

**Observed.** Neither setting exists in this version on macOS either, so the rest of the test cannot be carried out here. The same file rendered twice with nothing changed gave `IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`, and no folder appeared beside the images.

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.1/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — what the device does to the picture, and what a file asking for a card it has not been given produces.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render.

Two renders of it, changing only the device, plus a repeat of each: the file saying CPU with no argument, and the same file with `-- --cycles-device CUDA`. Compared on the pixels, never on the saved files:

```
processor run 1 vs run 2 : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0                  DIFFERING 0 of 1228800
card run 1 vs run 2      : IDENTICAL-PIXELS False MAX-ABS-DIFF 3.337860107421875e-06    DIFFERING 467847 of 1228800
processor vs card        : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.061817049980163574     DIFFERING 600257 of 1228800
```

**Observed.** The device changes the picture, on a second scene and in a second sitting: about half the channel values differ, the largest by some hundredths on a nought-to-one scale, where the processor repeats its own render exactly and the card repeats its own to about three millionths. This is hand test 27's result standing up on a scene built for a different question.

**And the one that answers what the file's own Device produces.** The same scene saved with Device **GPU Compute**, rendered with the backend preference naming nothing:

```
against the processor reference : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0              DIFFERING 0 of 1228800
against the card reference      : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.061817049980163574  DIFFERING 600257 of 1228800
```

**Observed.** It produces the **processor's** picture, to the very last number. So a file whose Device says GPU Compute does not, on its own, produce the graphics card's output; what it produces depends on whether a backend has been named somewhere else, and this is settled here by which picture came out rather than by what Blender printed, which names no device at all.

**One more thing about the picture, from hand test 34.** The same file rendered with `-- --cycles-device OPTIX` matched **neither** reference — against the processor 0.061817049980163574, against the CUDA card 0.05315208435058594. Three devices, three pictures. **Still owed on the Creator's Mac.**



## 3. Which of them can be settled at the moment a render is started from outside Blender, and which only by changing the file?

**LABEL: mixed — documented** for which switches exist and what they say, **inferred** for the two claims that rest on reading rather than on a quote (that a script argument can set anything, in 3a; that the settings in 3b have no switch at all), and **not in the sources** for the gap in 3d.

There are three homes, not two, and separating them is the whole answer.

### 3a. Settable on the command line at the moment the render starts

The complete list of render-relevant switches, from `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html` (read 2026-07-30). I read the entire page.

| Switch | What the manual says |
|---|---|
| `-t, --threads <threads>` | "Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count." |
| `--cycles-device <device>` | "Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL." Must follow a double dash: "Cycles add-on options must be specified following a double dash." |
| `--cycles-print-stats` | "Log statistics about render memory and time usage." |
| `-E, --engine <engine>` | "Specify the render engine. Use -E help to list available engines." |
| `-S, --scene <name>` | "Set the active scene <name> for rendering." |
| `--qos <level>` | "Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only)." |
| `--gpu-backend` | "Force to use a specific GPU backend. Valid options: vulkan, metal, opengl." |
| `--debug-depsgraph-no-threads` | "Switch dependency graph to a single threaded evaluation." |
| `-b, --background` | "Run in background (often used for UI-less rendering)." |

The manual's worked example (`https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html`) shows two of them together:

> "blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a
>
> -E CYCLES
>
> Use the "Cycles Render" engine. For a list of available render engines, run blender -E help.
>
> [...]
>
> -t 2
>
> Use only two threads."

And the ordering rule, which matters if you are building these commands automatically:

> "Arguments are executed in the order they are given. eg:
>
> blender --background test.blend --render-frame 1 --render-output "/tmp"
>
> …will not render to /tmp because --render-frame 1 renders before the output path is set.
>
> blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

That second line is the one to hold on to: **loading the blend-file overwrites settings applied before it.**

**A fourth route exists and covers everything else.** Blender will run arbitrary Python given on the command line:

> "-P, --python <filepath>
>
> Run the given Python script file."

> "--python-expr <expression>
>
> Run the given expression as a Python script.
>
> The expression may be a complete multi-line script; you are limited only by the platform's maximum argument length."

Every setting in question 1 that Blender exposes to Python as a property — all of 1a, 1b, 1e, and the Preferences of 1c — can therefore be set at launch by a script argument, placed after the file so the file load does not overwrite it. **Label for that claim: inferred.** The step: the manual documents that these switches run arbitrary Python; the API documentation documents these settings as writable properties; putting the two together gives you the result. The manual nowhere works this example for a performance setting, so I am joining two documented facts, not quoting one.

Also documented, and relevant if a file's own scripts matter: "-y, --enable-autoexec — Enable automatic Python script execution" and "-Y, --disable-autoexec — […] (default)" (`https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html`, read 2026-07-30). That page adds: "In this case the Preferences are still used but you may want to override them". Note that this concerns scripts *inside the blend-file*; `-P` and `--python-expr` are not described as needing it.

### 3b. Settled only in the blend-file (or by a script at launch)

Everything else in question 1's list has no dedicated switch. Specifically, there is **no command-line switch** for: Tile Size, Use Spatial Splits, Use Curves BVH, Use Compact BVH, BVH Time Steps, Persistent Data, Threads Mode (as opposed to the thread count), Simplify and its Texture Limit, Resolution X/Y, Percentage, Render Region, Samples, Noise Threshold, Time Limit, Sample Subset, EEVEE's Shadow Pool or Light Probes Volume Pool, or High Quality Normals.

**Label: inferred, and the step is mine.** No source says these settings have no switch; what I have is an exhaustive read that found none, and the step a reader must supply is that the page I read lists every switch there is. Where I looked: I read the entire Command Line Arguments page for 5.1 top to bottom — all sections: Render, Cycles Render, Format, Animation Playback, Window, Python, Network, Logging, Debug, GPU, Misc, Other, Argument Parsing, Environment Variables. I additionally searched the whole 5.1 manual for `CYCLES_` to find any environment variable that might set them, and found only one, unrelated: "CYCLES_CUDA_EXTRA_CFLAGS" on the GPU Rendering page, for choosing a compiler. I searched Cycles' own `.py` files for `environ` and `getenv` and found no environment-variable handling at all. The only command line handling Cycles' own files contain is the two switches quoted above, in `Blender Sources/cycles_5.1/engine.py` — **observed**.

### 3c. Settled in the Preferences, which are a different file again

The 1c settings — Cycles Render Devices, Distribute Memory Across Devices, HIP RT, MetalRT, Embree on GPU, Shader Compilation Method and its thread count — are not in the blend-file at all. The manual places the preferences in their own file:

> "./config/userpref.blend
>
> User preferences."

(`https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html`)

and states the separation:

> "Note
>
> This only resets the preferences and will not affect settings stored in the startup file.
> This includes app templates, area locations, and any Blender properties not part of the preferences."

(`https://docs.blender.org/manual/en/5.1/editors/preferences/introduction.html`)

Both read 2026-07-30. So sending a blend-file to another machine does not carry the choice of which GPUs to use with it. `--cycles-device` is the documented way to settle that at launch instead; Cycles' own help text for it says it overrides "user preferences and the scene setting" — **observed**, `Blender Sources/cycles_5.1/engine.py`.

### 3d. The gap I could not close

**Does `-t 2` override a file whose Threads Mode is "Fixed" with Threads set to 16? Or the other way round?** The manual describes `-t` as "Use amount of <threads> for rendering and other operations" and describes `threads`/`threads_mode` as scene properties, and **never relates the two**. Unlike `--cycles-device`, whose own help text says "overriding user preferences and the scene setting", nothing states what `-t` does when the file disagrees with it. Combined with the documented rule that "loading the blend-file overwrites" earlier arguments, this is genuinely uncertain. **Label for this sub-question: not in the sources.** Where I looked: the whole Command Line Arguments page; the Rendering From The Command Line page; `bpy.types.RenderSettings` in the API documentation; and a search of the entire 5.1 API documentation for "threads" outside threading topics, which returned only `PreferencesSystem`, `RenderEngine` and one line in the Blender-as-a-module page. There is a hand-test for this at the end.

One documented remark that touches it, from `https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html` (read 2026-07-30):

> "Command Line Arguments Unsupported
>
> Functionality controlled by command line arguments (shown by calling blender --help) isn't accessible.
>
> Typically this isn't such a limitation although there are some command line arguments that don't have
> equivalents in Blender's Python API (--threads and --log for example)."

**Inferred, step stated:** the API documentation treats `--threads` as *not* having a Python equivalent, even though `RenderSettings.threads` exists. The step a reader must supply is that these are therefore two different things — a process-wide thread setting and a per-scene one — rather than the same setting under two names. The source does not say that outright; it says only that no equivalent exists. This is the strongest hint I found that `-t` is not simply writing the scene property, and it is still only a hint.

**Not covered.** How the three homes resolve when they conflict, except for `--cycles-device`. Whether `-E` overrides the scene's engine permanently or only for that run. Whether `--gpu-backend` affects a Cycles render at all: nothing I read connects the switch to Cycles, and I did not record a search for it, so this is an open question rather than a settled silence.

**Outside the interface.** This question is entirely about being outside the interface, so the answer stands as given. The one thing to add: "-b, --background — Run in background (often used for UI-less rendering)", and the same page notes the audio device is disabled in background mode by default. Also relevant if you are automating: "--disable-depsgraph-on-file-load — Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)", with the manual's note that "this is a temporary option, in the future depsgraph will never be automatically generated on file load in background mode."

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Operating system.** `--qos` is "(Windows only)" and so is available on no other system. Everything else in 3a is listed without a platform note. The EEVEE limitation applies here with force: "Headless rendering is not supported on headless Windows systems" — so on Windows with no display attached, no argument settles anything for EEVEE, because the render will not run. The sources do not say the same of Cycles.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.1/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 2 — whether `-t` changes the thread settings stored in the scene, and whether where it sits matters.** Done to a file saved with Threads Mode Fixed and Threads 1. Five runs, each reading back with `--python-expr "import bpy; r=bpy.context.scene.render; print('THREADS', r.threads_mode, r.threads)"`:

```
(a) no -t                                    THREADS FIXED 1
(b) -t 8 after the file name                 THREADS FIXED 8
(c) -t 8 before the file name                THREADS FIXED 8
(d) -t 0 after the file name                 THREADS FIXED 1
(e) --factory-startup, no file (Auto-Detect) THREADS AUTO 24
```

**Observed.** `-t` writes the scene's own Threads value whichever side of the file name it sits on. `-t 0` leaves the stored value alone. Under Auto-Detect the property reports the machine's logical processor count — twenty-four here — rather than a placeholder.

**Hand test 4 — which wins when `-t` and a start-up script both set the thread count.** Done to the 320 by 240 Cycles scene. Three runs, with `<set 5>` standing for `import bpy; r=bpy.context.scene.render; r.threads_mode="FIXED"; r.threads=5; print("SET-BY-SCRIPT", r.threads_mode, r.threads)`:

```
(a) blender -b --factory-startup heavy.blend -t 3 --python-expr "<set 5>" --python-expr "<print>" -f 1 ...
    SET-BY-SCRIPT FIXED 3
    FINAL FIXED 3
    wall 0:13.60
(b) the same with the script before -t 3
    SET-BY-SCRIPT FIXED 3
    FINAL FIXED 3
    wall 0:12.89
(c) the script alone, no -t
    SET-BY-SCRIPT FIXED 5
    FINAL FIXED 5
    wall 0:07.86
```

**Observed.** The script assigned 5 and printed back 3. Order in the command line made no difference. With no `-t` the same script got 5.

**Hand test 5 — whether `-E` overrides the file's engine, whether its place matters, and whether the file on disk changes.** Done to a file saved with EEVEE as its engine:

```
(a) no -E                                   ENGINE BLENDER_EEVEE
(b) -E CYCLES after the file name           ENGINE CYCLES
(c) -E CYCLES before the file name          ENGINE BLENDER_EEVEE
(d) -E CYCLES, then save a copy             BEFORE-SAVE CYCLES ; the copy reads COPY-ENGINE CYCLES
(e) the original file, read again           ENGINE BLENDER_EEVEE
(f) -E BOGUS_ENGINE                         Error: engine not found 'BOGUS_ENGINE'
```

**Observed.** The argument takes effect only after the file name. It does not change the file on disk, but a session that saves afterwards writes the overridden engine into the new file. An unknown name is refused by name.

**Hand test 1 — what engines the build offers.** Done by running the documented listing:

```
$ blender -b -E help
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
EXIT=0
```

Without `-b`, on this machine, all four versions fail before reaching the list: `Failed to initialize display for back-end(s): ['WAYLAND', 'X11'] ... ERROR Unable to initialize GHOST, exiting!`

**Observed.**

**Hand test 6 — whether `--cycles-device` changes the device stored in the scene.** Done to a file saved with Cycles and Device CPU. Read back before a render and after one:

```
(a) no override                                          DEVICE CPU
(b) -- --cycles-device OPTIX, read before any render     DEVICE CPU
(c) -f 1 first, then read, with --cycles-device OPTIX
00:00.460  cycles           | ERROR CUDA cuInit: Unknown error
DEVICE CPU
(d) -f 1 first, then read, with --cycles-device CPU      DEVICE CPU
(e) -- --cycles-device NOSUCHDEVICE
00:00.408  cycles           | ERROR NOSUCHDEVICE is not a valid Cycles device.
```

**Observed.** The stored value never moved, at either moment. The machine's card was unreachable, so what the override does to a render that can use a card is not settled here.

**Hand test 8 — whether a render started from outside reads the saved preferences.** Done in a settings folder of its own. The Cycles compute backend was set to `CUDA` and `bpy.ops.wm.save_userpref()` called, producing `config/userpref.blend` of 176837 bytes. Then read back:

```
plain background run (no --factory-startup)   READ-PREF CUDA
the same with --factory-startup               READ-PREF NONE
```

**Observed.** A background run does read the saved preferences; `--factory-startup` stops it.

**Hand test 20 — whether a start-up script can set a setting that has no argument of its own.** Done to the 640 by 480 scene, whose Samples is 64: `blender -b --factory-startup med.blend --python-expr "import bpy; bpy.context.scene.cycles.samples=7; print('SAMPLES-SET', bpy.context.scene.cycles.samples)" -o <folder>/s7_ -F OPEN_EXR -x 1 -f 1`. What came back: `SAMPLES-SET 7`, `Time: 00:00.30` against 00:01.52 for the unchanged render, and the picture differs from the 64-sample one — `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.6514395475387573`.

**Observed.** The assignment reaches the render.

**Hand test 21 — whether "Render Single Layer" is honoured from the command line.** Done to a file with two view layers, `ViewLayer` and `Second`, saved once with the setting off and once on, each rendered with `-f 1`. In both runs Blender named both view layers in its own progress lines (`Scene, Second` and `Scene, ViewLayer`).

**Observed.** The setting made no difference to a render started from the command line.

**One thing learned by getting a command wrong, recorded because it bears on every command in this answer.** An earlier run gave the arguments as `-f 1 -o <folder>/r_ -F OPEN_EXR -x 1`, with the render before the output path, and the image went to `/tmp/0001.exr` instead. `-o`, `-F` and `-x` have to come before `-f`.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.1/` or `/root/render-demands/prefs-card-cuda/5.1/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 28 — what `--cycles-device` does to a render that really can use the card.** This is the test the earlier sitting could not run at all. Six timed runs, none of them using `--factory-startup`, so the settings folder's saved preferences were read every time. Two files, both Cycles at 640 by 480 with 1024 samples: one saved with Device **CPU**, one saved with Device **GPU Compute**. Two settings folders: one with the Cycles backend preference saved as **None**, one with it saved as **CUDA**. Each run was `blender -b <file> -o <folder>/x_ -F OPEN_EXR -x 1 -f 1 -- --cycles-print-stats`, with the device argument added after `--cycles-print-stats` where one was given. Alongside the timing, this Blender alone was watched for whether it held work on the card.

```
file CPU,  preference None, no argument            : Time 00:24.66, wall 26.25 s, 2076% of a processor — never held any card memory at all
file CPU,  preference None, --cycles-device OPTIX  : Time 00:04.14, wall 5.82 s, 92% of a processor — held card memory, peak 1652 MiB
file CPU,  preference None, --cycles-device CUDA   : Time 00:03.62, wall 4.45 s, 93% of a processor — held card memory, peak 1638 MiB
file CPU,  preference CUDA, no argument            : Time 00:23.75, wall 24.60 s, 2166% of a processor — held card memory, peak 248 MiB
file GPU,  preference CUDA, no argument            : Time 00:03.60, wall 4.42 s, 95% of a processor — held card memory, peak 1638 MiB
file GPU,  preference CUDA, --cycles-device CPU    : Time 00:24.73, wall 25.58 s, 2088% of a processor — held card memory, peak 248 MiB
```

Two more runs were added, because this is the exact pair the 4.5 audit asked for — the file already asking for the card, with the preferences naming no backend at all:

```
file GPU,  preference None, no argument            : Time 00:26.88, wall 27.54 s, 2248% of a processor — never held any card memory at all
file GPU,  preference None, --cycles-device OPTIX  : Time 00:03.44, wall 4.38 s, 86% of a processor — held card memory, peak 1652 MiB
```

**Observed.** **The argument wins, in both directions.** A file that says CPU, with preferences that name no backend at all, renders on the card the moment `--cycles-device` names one — about seven times faster, the processor left almost idle, and the card holding well over a gigabyte. And a file that says GPU Compute, with CUDA saved in the preferences, is dragged back onto the processor by `--cycles-device CPU`. So neither the file's own Device setting nor the saved backend preference can stop the argument. What the two of them do decide is what happens when no argument is given: the file's Device says whether the card is used at all, and the preference says which backend. A file saying GPU Compute with the preference at None renders on the processor and never touches the card — which is the earlier sitting's result, confirmed here with a working card rather than an unreachable one — and a file saying CPU renders on the processor however the preference is set.

**A correction to hand test 26, which was run with no reachable card.** That test found that nothing Blender printed named the device that rendered. With a working card that holds for CUDA — a card render through CUDA printed no word about the card anywhere — but not quite for OptiX, whose renders name it in their ordinary progress lines while the acceleration structure is being built:

```
00:03.017  render           | Fra: 1 | Mem: 3M | Updating Scene BVH | Building OptiX acceleration structure
```

Those lines say OptiX built the structure. They are not a statement of which device rendered, and no line anywhere says that. Anything reading the device off Blender's output still cannot settle it; the timing, and what the process holds on the card, remain the evidence.

**Hand test 6, run again now the card works — whether `--cycles-device` changes the device stored in the scene.** The earlier sitting could only show this on a machine where the named device was absent, which left the obvious objection open. Repeated here against a file saved with Cycles and Device CPU, with the card present and working:

```
(a) no argument                                            : DEVICE CPU
(b) -- --cycles-device OPTIX, read before any render       : DEVICE CPU
(c) -- --cycles-device OPTIX, read after a render          : DEVICE CPU   (and the render ran on the card)
(d) -- --cycles-device CPU, read after a render            : DEVICE CPU
(e) -- --cycles-device NOSUCHDEVICE                        : 00:00.316  cycles           | ERROR NOSUCHDEVICE is not a valid Cycles device.
(f) preference CUDA, -- --cycles-device OPTIX, after render: DEVICE CPU   and the preference still reads CUDA
```

**Observed.** The stored value never moves, before a render or after one, whether or not the named device exists and whether or not it is used. The argument changes what does the work for that one run and writes nothing down — not into the scene, not into the saved preferences. An unknown device name is still refused by name. One thing changed from the earlier sitting: with no reachable card, asking for OptiX also produced `CUDA cuInit: Unknown error`; with the card working, that line is gone and the render simply runs on the card.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.1/`.

**Asked which compute backends it has, this Blender offers two and refuses the rest:**

```
TYPE NONE  DEVICES []
TYPE METAL DEVICES [('Apple M1 Pro (GPU - 14 cores)', 'METAL', True), ('Apple M1 Pro', 'CPU', False)]
TYPE CUDA  COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "CUDA" not found in ('NONE', 'METAL')
```

and `OPTIX`, `HIP` and `ONEAPI` were refused in exactly the same words. So on this operating system the backend setting has only two values it will take.

**One limit runs through everything below, and it is why not one number here is a time or a memory figure.** The Mac was shared with other topics' work while these ran — another topic's Blender was found running on it partway through — so **nothing was timed and nothing measured memory**. The rented Linux machine carries this round's timings, and it is the kind of computer a Smoozy render actually happens on. Where a test's only possible evidence was a time or a memory figure, it was not run here at all, and that is said under the question it would have answered. Every picture comparison below compares the pixels and never the saved file, because Blender writes the time of day into every image it saves.

The full list of tests, with the procedure for each, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 2 — whether `-t` writes the thread setting stored in the scene, and whether its place in the command line matters.** A file saved with Threads Mode Fixed and Threads 1, read back with `--python-expr "import bpy; r=bpy.context.scene.render; print('THREADS', r.threads_mode, r.threads)"`:

```
(a) no -t                                   THREADS FIXED 1
(b) -t 8 after the file name                THREADS FIXED 8
(c) -t 8 before the file name               THREADS FIXED 8
(d) -t 0 after the file name                THREADS FIXED 1
(e) factory startup, no file, Auto-Detect   THREADS AUTO 8
```

**Observed.** `-t` writes the scene's own Threads value whichever side of the file name it sits on; `-t 0` leaves the stored value alone; and under Auto-Detect the property reports this machine's logical processor count, which is 8. The rented Linux machine gave the same five lines, with 24 in the last one, which is its own processor count. **The two machines agree.**

**Hand test 4 — which wins when `-t` and a start-up script both set the thread count.** A script that assigns 5 and then prints back what it just assigned:

```
(a) -t 3 first, then the script    SET-BY-SCRIPT FIXED 3    FINAL FIXED 3
(b) the script first, then -t 3    SET-BY-SCRIPT FIXED 3    FINAL FIXED 3
(c) the script alone, no -t        SET-BY-SCRIPT FIXED 5    FINAL FIXED 5
```

**Observed.** The line worth reading is `SET-BY-SCRIPT FIXED 3`: the script set 5 and what came back was 3. With no `-t` the same script got 5. Where the argument sits made no difference. The same on the Linux machine.

**Hand test 5 — whether `-E` overrides the file's engine, and whether the file on disk changes.** A file saved with EEVEE as its engine:

```
(a) no -E                                              ENGINE BLENDER_EEVEE
(b) -E CYCLES after the file name                      ENGINE CYCLES
(c) -E CYCLES before the file name                     ENGINE BLENDER_EEVEE
(d) -E CYCLES, then save a copy, then read the copy    BEFORE-SAVE CYCLES / COPY-ENGINE CYCLES
(e) read the original file again                       ENGINE BLENDER_EEVEE
(f) -E BOGUS_ENGINE                                    Error: engine not found 'BOGUS_ENGINE'
```

**Observed.** The argument works only when it comes **after** the file name; it does not change the file on disk; a session that saves afterwards writes the overridden engine into the new file; and an unknown name is refused by name. The Linux machine gave the same six.

**Hand test 6 — whether `--cycles-device` changes the device stored in the scene.** A file saved with Cycles and Device CPU, on a machine whose graphics work:

```
(a) no argument                                                  DEVICE CPU
(b) -- --cycles-device METAL, read before any render             DEVICE CPU
(c) -- --cycles-device METAL, read after a render                DEVICE CPU
(d) -- --cycles-device CPU, read after a render                  DEVICE CPU
(f) settings folder saved with the backend METAL, argument CPU   DEVICE CPU   PREF METAL
(e) an unknown device name        Error: NOSUCHDEVICE is not a valid Cycles device.
```

**Observed.** The stored value never moved, before a render or after one, with a working graphics device present and with the argument set against the saved preference as well as against the file. The saved backend preference did not move either. **The argument changes what does the work for that one run and writes nothing down.** Same as the Linux machine.

**Hand test 8 — whether a render started from outside reads the saved preferences.** A settings folder of the test's own, with the Cycles backend set to `METAL` and the preferences saved:

```
(a) SET-PREF METAL / SAVED-PREFS, and a userpref.blend appeared in the folder
(b) read back in a plain background run (no --factory-startup)   READ-PREF METAL
(c) read back with --factory-startup                             READ-PREF METAL
```

At first sight (c) disagrees with the Linux machine, which printed `READ-PREF NONE` there. **It does not, and the control is what shows why.** A brand-new settings folder with no `userpref.blend` in it at all, read with `--factory-startup`, also prints `READ-PREF METAL`; and a folder whose saved backend is `NONE`, read with `--factory-startup`, prints `READ-PREF METAL` as well — that is, factory startup threw the saved `NONE` away.

```
a brand-new settings folder, no userpref.blend, --factory-startup   READ-PREF METAL
the same brand-new folder, no --factory-startup                     READ-PREF METAL
a folder saved with the backend at None, --factory-startup          READ-PREF METAL
a folder saved with the backend at None, no --factory-startup       READ-PREF NONE
```

**Observed.** A plain background run **does** read the saved preferences, and `--factory-startup` **does** stop it — the same as on Linux. What differs is what factory state itself is: **on this Mac the factory value of the Cycles compute backend is `METAL`, where on the Linux machine it is `NONE`.** Also recorded: `CUDA` cannot be set on this machine at all — `bpy_struct: item.attr = val: enum "CUDA" not found in ('NONE', 'METAL')` — so the Linux half of this test could not have been run here in the same words.

**Hand test 20 — whether a start-up script can set a setting that has no argument of its own.** `--python-expr "import bpy; bpy.context.scene.cycles.samples=7"` printed `SAMPLES-SET 7`, and the picture that came out is not the 64-sample picture. **Observed.** A start-up expression reaches the render for a setting with no flag of its own.

**Hand test 28 — what `--cycles-device` does to a render that really can use the graphics processor.** This is the test the Linux machine settled by timing and by watching what the process held on the card. **Neither was available here**, because the Mac was not idle, so it was settled a different way: by **which picture came out**. Hand test 27 above gives two reference pictures for this exact file — one known to have been made by the processor, one known to have been made by Metal — and they differ by about a twentieth while each device repeats itself to within a millionth. That is a fingerprint the arithmetic cannot confuse. **No run below used `--factory-startup`**, so each read the saved preferences.

```
(a) file CPU,  preference None,  no argument               -> matched the PROCESSOR render exactly (MAX-ABS-DIFF 0.0)
(b) file CPU,  preference None,  --cycles-device METAL     -> matched the METAL render (MAX-ABS-DIFF 2.384185791015625e-06)
(c) file GPU,  preference Metal, no argument               -> matched the METAL render
(d) file GPU,  preference Metal, --cycles-device CPU       -> matched the PROCESSOR render exactly (MAX-ABS-DIFF 0.0)
(e) file GPU,  preference None,  no argument               -> matched the PROCESSOR render exactly (MAX-ABS-DIFF 0.0)
(f) file GPU,  preference None,  --cycles-device METAL     -> matched the METAL render
```

and the control that says the reference pictures are the right ones to compare against: the same file and preference as hand test 27's processor run, but without `--factory-startup`, came out `IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0` against it.

**Observed.** **The argument wins, in both directions.** A file saying CPU, in a settings folder naming no backend, renders on Apple's graphics the moment `--cycles-device METAL` is given; and a file saying GPU Compute, with Metal saved in the preferences, is pulled back onto the processor by `--cycles-device CPU`. Neither the file nor the saved preference can stop it. What the file and the preference decide is only what happens when **no** argument is given: the file's Device says whether the graphics processor is used at all, the preference says which backend, and a file saying GPU Compute with the preference at None renders on the processor and never reaches the graphics at all. **This is the same shape the rented Linux machine found with CUDA and OptiX, reached here by a different kind of evidence on a different make of graphics processor.**

**Hand test 26 — and what this means for reading the device off Blender's output.** Nothing in the printed output of any run above named the device, including the Metal ones. **Observed.** On this machine there is no line to read: the timing and the picture are the only evidence there is.

**Hand test 21 — Render Single Layer.** Both view layers were named in Blender's progress lines whether the box was clear or ticked. **Observed.** Not honoured from the command line, as on Linux.

**Hand test 18 — the Viewport-labelled settings.** Six of them pushed to extremes changed no pixel of a render started from outside. **Observed.** They do not reach it.

**Hand test 12 — Resolution Percentage.** A file 160 by 120 at 37 per cent wrote an image of `(59, 44)` from the command line. **Observed.** The percentage reaches a render started from outside.

**One test was not run on this machine, and it is said here rather than left as a silence.** Hand test 3 — whether `-t` changes how much of the processor a Cycles render actually uses — has no evidence except wall-clock time and the share of the processor taken. The Mac was shared with other topics' work throughout, so a figure from it would be worthless. It was run on the rented Linux machine and is recorded above. **Whether the same holds on macOS is left open and is not guessed at.**

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.1/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — what the file's stored Device settles on its own when a render is started from outside.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render. Rendered from the command line with `blender -b <file> -o <folder>/<tag>_ -F OPEN_EXR -x 1 -f 1 --`, no `--factory-startup`, so the saved preferences were read:

```
file CPU,  backend preference NONE, no argument            : the processor's picture (the reference)
file CPU,  backend preference NONE, -- --cycles-device CUDA : the card's picture      (the reference)
file GPU,  backend preference NONE, no argument            : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 against the PROCESSOR
                                                              IDENTICAL-PIXELS False MAX-ABS-DIFF 0.061817049980163574 against the CARD
```

**Observed.** From outside Blender, the file's stored Device is not enough on its own: a file saying GPU Compute renders on the processor unless a backend is named in the preferences or on the command line. The run held no card memory at all, which says the same a second way. This replaces the half of hand test 7 that searched Blender's output for complaints — a search that could not have failed, because no line of Blender's output names the device that rendered.

**Hand test 34 — what `--cycles-device` does when it names a backend this build accepts but has no hardware for.** Six renders of the file that says CPU, with the preference naming nothing, the argument naming each backend in turn:

```
--cycles-device CPU    exit 0, image written : the processor's picture exactly, MAX-ABS-DIFF 0.0
--cycles-device CUDA   exit 0, image written : the card's picture, MAX-ABS-DIFF 2.86102294921875e-06
--cycles-device OPTIX  exit 0, image written : a third picture, matching neither
--cycles-device HIP    exit 1, no image written
--cycles-device ONEAPI exit 1, no image written
--cycles-device METAL  exit 1, no image written
```

and what the three refused runs printed, word for word:

```
00:00.383  reports          | ERROR Found no Cycles device of the specified type
```

and `HIP` printed one line more, before it:

```
00:00.385  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
00:00.388  reports          | ERROR Found no Cycles device of the specified type
```

**Observed.** The argument does not quietly fall back. A backend with no hardware behind it stops the render, writes no image, and returns a non-zero exit code — which is a thing anything driving Blender from outside can actually see, unlike the device that rendered. An unknown name is a different case again and was refused by name in the earlier sitting. **Still owed on the Creator's Mac.**



## 4. Do any of them belong to a particular render engine rather than to every engine?

**LABEL: documented**, and the answer is yes — sharply so.

Blender 5.1 ships three engines: "Blender includes three render engines with different strengths: EEVEE is a physically based realtime renderer. Cycles is a physically based path tracer. Workbench is designed for layout, modeling and previews." (`https://docs.blender.org/manual/en/5.1/render/introduction.html`, read 2026-07-30). The manual adds: "More renderers from third-party developers are available as add-ons. Each renderer has its own render settings to control render quality and performance."

### 4a. Cycles only

Tile Size, Use Spatial Splits, Use Curves BVH, Use Compact BVH, BVH Time Steps, and the Device (CPU / GPU Compute) choice. **Observed**: all six are defined in `Blender Sources/cycles_5.1/properties.py` on the class `CyclesRenderSettings`, which the same file attaches to the scene under the name `cycles`:

> `@classmethod`
> `def register(cls):`
> `    bpy.types.Scene.cycles = PointerProperty(name="Cycles Render Settings", description="Cycles render settings", type=cls,)`

(`properties.py`, `CyclesRenderSettings.register`.) They are Cycles' own, shipped with the Cycles add-on, and they exist only because that add-on registered them. Corroborating this from the other side: I searched the 5.1 API documentation file list for any page named for Cycles and found only `bpy.ops.cycles.txt` and `bpy.types.FModifierCycles.txt` (the latter an animation curve modifier, nothing to do with rendering). **There is no API documentation page for Cycles' render settings at all** — they are not part of Blender's documented API.

The command-line switches `--cycles-device` and `--cycles-print-stats` are likewise Cycles' own. The manual sets them apart under their own heading, "Cycles Render Options", with the rule "Cycles add-on options must be specified following a double dash", and Cycles' own `engine.py` is where they are parsed — **observed**.

Some of them narrow further, to particular *devices* within Cycles. **Observed**, from `Blender Sources/cycles_5.1/ui.py`, class `CYCLES_RENDER_PT_performance_acceleration_structure`:

> `@classmethod`
> `def poll(cls, context):`
> `    return not use_optix(context) or use_multi_device(context)`

and inside `draw`:

> `if use_cpu(context):`
> `    col.prop(cscene, "debug_use_spatial_splits")`
> `    if use_embree:`
> `        col.prop(cscene, "debug_use_compact_bvh")`
> `    else:`
> `        sub = col.column()`
> `        sub.active = not cscene.debug_use_spatial_splits`
> `        sub.prop(cscene, "debug_bvh_time_steps")`
> `        col.prop(cscene, "debug_use_hair_bvh")`
> `        sub = col.column(align=True)`
> `        sub.label(text="Cycles built without Embree support")`
> `        sub.label(text="CPU raytracing performance will be poor")`
> `else:`
> `    col.prop(cscene, "debug_use_spatial_splits")`
> `    sub = col.column()`
> `    sub.active = not cscene.debug_use_spatial_splits`
> `    sub.prop(cscene, "debug_bvh_time_steps")`
> `    col.prop(cscene, "debug_use_hair_bvh")`
> `    # CPU is used in addition to a GPU`
> `    if use_multi_device(context) and use_embree:`
> `        col.prop(cscene, "debug_use_compact_bvh")`

**Observed**, precisely: whether each of these controls is *shown* depends on the device in use and on whether this build has Embree. The whole panel is hidden when OptiX is in use unless a mixed CPU+GPU setup is configured. Compact BVH is shown only where Embree is present. BVH Time Steps is greyed out (`sub.active = not …`) when Spatial Splits is on.

**Inferred, and the step is mine:** a control that the interface does not show for a given device is probably not doing anything for that device. The step a reader must supply is that hidden means inert — the file shows only what is drawn, not what the renderer reads. Do not treat this as established. What *is* established is that the values remain in the file whether shown or not, because they are plain properties.

### 4b. EEVEE only

Shadow Pool and Light Probes Volume Pool. These appear on EEVEE's Performance page and nowhere else, and the API documents them on the EEVEE-specific settings type (`https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html`, read 2026-07-30):

> "gi_irradiance_pool_size
>
> Size of the irradiance pool, a bigger pool size allows for more irradiance grid in the scene but might not fit into GPU memory and decrease performance (default '16')
>
> Type: Literal['16', '32', '64', '128', '256', '512', '1024']"

> "shadow_pool_size
>
> Size of the shadow pool, a bigger pool size allows for more shadows in the scene but might not fit into GPU memory (default '512')
>
> Type: Literal['16', '32', '64', '128', '256', '512', '1024']"

### 4c. Shared by EEVEE and Workbench, but not Cycles

High Quality Normals: it is the only item on Workbench's Performance page and the first on EEVEE's, worded identically on both. It does not appear on Cycles' Performance page.

### 4d. Belonging to Blender, not to any engine

**Threads Mode, Threads and Persistent Data** are documented in the API on `bpy.types.RenderSettings` — Blender's own render settings, not any engine's. But here is the wrinkle worth naming: in 5.1 the *only* place the manual documents them is on the **Cycles** Performance page, and the only place Cycles' own file draws them is Cycles' own panel — **observed**, `Blender Sources/cycles_5.1/ui.py`:

> `col.prop(rd, "threads_mode")`
> `sub = col.column(align=True)`
> `sub.enabled = rd.threads_mode == 'FIXED'`
> `sub.prop(rd, "threads")`

(class `CYCLES_RENDER_PT_performance_threads`, where `rd = scene.render`), and

> `col.prop(rd, "use_persistent_data", text="Persistent Data")`

(class `CYCLES_RENDER_PT_performance_final_render`). Note `rd`, not `cscene` — these are Blender's properties, drawn inside Cycles' panel. Neither appears on EEVEE's or Workbench's Performance page.

**Also engine-neutral**: Simplify (documented under Cycles but described as "Common Settings" affecting the Subdivision Surface modifier and particles generally), Resolution X/Y and Percentage and Render Region (on the Output ‣ Format page, which is not engine-specific), and the Preferences settings — except that Cycles Render Devices, HIP RT, MetalRT and Embree on GPU are all explicitly about Cycles by their own descriptions, while Shader Compilation Method is not.

**Not covered.** Whether EEVEE or Workbench actually *honour* `threads`, `threads_mode` and `use_persistent_data`, given that neither engine's Performance panel exposes them. The API's descriptions are engine-neutral ("Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems)"), but no source I found says which engines read them. This is a real gap and there is a hand-test for it. Nothing here covers third-party engines. The device-dependent visibility in 4a tells you what is drawn, not what is used.

**Outside the interface.** All of this holds outside the interface with one consequence worth stating: the visibility rules in 4a are *interface* rules, and outside the interface there is no panel, so nothing is hidden — a script or a saved file can set any of these values regardless of device. Whether the renderer then reads them is exactly the question the sources do not answer. Separately, EEVEE cannot render headlessly on Windows at all ("Headless rendering is not supported on headless Windows systems"), and the EEVEE limitations page also states "There is currently no support for multiple GPU systems" and, of CPU rendering, "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering."

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Operating system.** The engine split itself is not platform-dependent — all three engines exist everywhere Blender does. What is platform-dependent is which Cycles device backends exist (see the list in answer 1), which in turn drives the visibility rules in 4a: on Apple Silicon there is no OptiX, so the OptiX-hides-the-panel rule never fires there. EEVEE's headless-Windows limitation is the one flat platform difference among the engine-specific settings.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.1/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 22 — whether EEVEE and Workbench render at all on a machine with no display server.** Done to the base scene: `blender -b --factory-startup scene.blend -E BLENDER_EEVEE -o <folder>/e_ -F OPEN_EXR -x 1 -f 1`, then the same with `-E BLENDER_WORKBENCH`. **Both rendered and wrote an image, exit code 0.** EEVEE took `Time: 00:28.94`; Workbench about 1.2 seconds. Workbench also printed one line before saving: `EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).`

**Observed.** This machine had neither a display server nor a usable graphics card, and both engines still produced their image.

**Hand test 23 — whether the thread count changes anything under EEVEE or Workbench.** Done to the base scene with EEVEE Render Samples 16. Twelve runs: each engine, `-t 1` and `-t 24`, three times each, timed by the operating system. Twenty-four logical processors, machine shared with other work:

```
EEVEE     -t 1  : 8.38 s wall, 404% cpu, 1713688 kb / 7.62 s, 521%, 1703356 kb / 8.06 s, 410%, 1705152 kb
EEVEE     -t 24 : 8.66 s wall, 429% cpu, 1727552 kb / 9.02 s, 360%, 1727120 kb / 9.21 s, 340%, 1725424 kb
WORKBENCH -t 1  : 1.79 s wall,  89% cpu,  494744 kb / 1.57 s,  97%,  494676 kb / 1.56 s,  93%,  488996 kb
WORKBENCH -t 24 : 1.52 s wall, 103% cpu,  508576 kb / 1.62 s, 112%,  511412 kb / 1.61 s,  98%,  509308 kb
```

**Observed.** The two sets of times overlap in both engines, and the processor percentages stay put. Under Cycles the same argument changed the time by a factor of fifteen (hand test 3). **This is "no effect seen", not "the engine ignores it"** — a render spending its time elsewhere would look the same.

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench.** Six renders of a scene with a Subdivision Surface modifier at render level 4, each engine with Simplify off and with `simplify_subdivision_render` at 0, compared by pixels:

```
CYCLES            : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.12282812595367432 NUM-DIFFERING-CHANNEL-VALUES 28221 of 76800
BLENDER_EEVEE     : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.105712890625      NUM-DIFFERING-CHANNEL-VALUES 3564 of 76800
BLENDER_WORKBENCH : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.0909423828125     NUM-DIFFERING-CHANNEL-VALUES 3397 of 76800
```

**Observed.** All three engines changed, so that setting is not Cycles-only in effect.

**Hand test 13 — whether Cycles' own values survive the engine being switched away and back.** Done to a file with Tile Size 256, Samples 11 and Use Tiling off. Opened, printed, engine set to EEVEE and saved; reopened, printed, engine set back to Cycles and saved; reopened and printed again:

```
STEP1 engine CYCLES tile_size 256 samples 11 use_auto_tile False
STEP2 engine BLENDER_EEVEE tile_size 256 samples 11 use_auto_tile False
STEP3 engine CYCLES tile_size 256 samples 11 use_auto_tile False
```

**Observed.** Nothing was dropped or reset.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.1/`.

**Asked which compute backends it has, this Blender offers two and refuses the rest:**

```
TYPE NONE  DEVICES []
TYPE METAL DEVICES [('Apple M1 Pro (GPU - 14 cores)', 'METAL', True), ('Apple M1 Pro', 'CPU', False)]
TYPE CUDA  COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "CUDA" not found in ('NONE', 'METAL')
```

and `OPTIX`, `HIP` and `ONEAPI` were refused in exactly the same words. So on this operating system the backend setting has only two values it will take.

**One limit runs through everything below, and it is why not one number here is a time or a memory figure.** The Mac was shared with other topics' work while these ran — another topic's Blender was found running on it partway through — so **nothing was timed and nothing measured memory**. The rented Linux machine carries this round's timings, and it is the kind of computer a Smoozy render actually happens on. Where a test's only possible evidence was a time or a memory figure, it was not run here at all, and that is said under the question it would have answered. Every picture comparison below compares the pixels and never the saved file, because Blender writes the time of day into every image it saves.

The full list of tests, with the procedure for each, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 29 — which Performance sub-panels each engine shows.** This needs a window, and this machine is the only one that has a screen. Blender was started with a window and a script that switched the engine, put the Properties editor on the Render tab, took Blender's own screenshot of that editor with `bpy.ops.screen.screenshot_area`, listed every Render-tab panel Blender itself was willing to draw, and quit at the end. What each engine's Performance panel holds:

```
CYCLES             Performance -> Compositor, Threads, Memory, Acceleration Structure, Final Render, Viewport
BLENDER_EEVEE      Performance -> High Quality Normals drawn straight in it, then Memory, Viewport and Compositor
BLENDER_WORKBENCH  Performance -> High Quality Normals drawn straight in it, then Compositor and nothing else
```

**Observed.** The three engines do not show the same Performance panel. **Threads, Acceleration Structure and Final Render appear under Cycles and under neither of the other two.** Memory and Viewport appear under Cycles and EEVEE but not under Workbench. Compositor appears under all three. High Quality Normals is drawn straight into the Performance panel under EEVEE and Workbench, and is not in Cycles' Performance panel at all.

**One trap is worth recording, because it would have produced a wrong answer.** Asked on its own, the Acceleration Structure panel's own test for whether it should be shown returns **true under all three engines**. It is not drawn under EEVEE or Workbench all the same, because its parent — Cycles' Performance panel — is not shown, and Blender does not draw a child of a panel it is not drawing. The screenshots are what settle it; the panel's own answer, taken alone, does not.

**Hand test 30 — which Acceleration Structure controls appear for each device.** With Cycles selected and a window open, the scene's Device and the Cycles backend preference were moved through all four combinations this machine can have, and Blender's own screenshot taken each time. What the panel drew:

```
Device CPU,         backend None    ->  Use Spatial Splits, Use Compact BVH
Device CPU,         backend Metal   ->  Use Spatial Splits, Use Compact BVH
Device GPU Compute, backend None    ->  Use Spatial Splits, Use Compact BVH   (and the Device field itself greyed out)
Device GPU Compute, backend Metal   ->  Use Spatial Splits, BVH Time Steps, Use Curves BVH
```

**Observed.** The controls are **not** the same in every combination, so the visibility rules do hold on a real build. Two of the four controls swap over: the processor's column shows Use Compact BVH, the graphics column shows BVH Time Steps and Use Curves BVH instead, and Use Spatial Splits is shown to both. And it takes **both** halves to get the graphics column — a file set to GPU Compute with the backend preference naming nothing keeps the processor's controls and greys out the Device field itself, which is the interface saying the same thing hand test 28 found: that run goes to the processor. **OptiX cannot be reached on this machine at all**, so that combination is untested rather than absent, and the Linux machine has no Metal, so neither machine can settle the other's.

**Hand test 9 — whether Cycles' settings can be read with the add-on off, and whether they can be read while EEVEE is the engine.**

```
(a) turn the Cycles add-on off in a settings folder of its own, then read at once
    AFTER-TYPE-HAS-CYCLES False
    AFTER-SCENE-HAS-CYCLES False
    AFTER-READ-FAILED AttributeError 'Scene' object has no attribute 'cycles'
(b) start again against that saved settings folder and read the file
    ADDONS ['bl_pkg', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
    READ-FAILED AttributeError 'Scene' object has no attribute 'cycles'
    ENGINE BLENDER_EEVEE
(c) the same file with the Cycles add-on on
    TILE-SIZE 2048 SAMPLES 64 DEVICE CPU     ENGINE CYCLES
(d) the same file with the engine set to EEVEE, Cycles add-on on
    TILE-SIZE 2048 SAMPLES 64 DEVICE CPU     ENGINE BLENDER_EEVEE
```

**Observed.** Cycles' settings cannot be read at all with the add-on off, and with the add-on off the same file that reads back as `CYCLES` on line (c) reads back as the EEVEE name on line (b) — the engine stored in the file is not readable either. Cycles' values **are** readable while EEVEE is the engine. The same on the Linux machine.

**Hand test 13 — whether Cycles' own values survive the engine being switched away and back.**

```
STEP1 engine CYCLES tile_size 256 samples 11 use_auto_tile False
STEP2 engine BLENDER_EEVEE tile_size 256 samples 11 use_auto_tile False
STEP3 engine CYCLES tile_size 256 samples 11 use_auto_tile False
```

**Observed.** Nothing was dropped or reset. Same as on Linux.

**Hand test 17 — whether Simplify's Render column reaches all three engines.**

```
CYCLES              simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.35849255323410034 NUM-DIFFERING-CHANNEL-VALUES 37720 of 76800
BLENDER_EEVEE       simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.380859375 NUM-DIFFERING-CHANNEL-VALUES 10319 of 76800
BLENDER_WORKBENCH   simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.10693359375 NUM-DIFFERING-CHANNEL-VALUES 8580 of 76800
```

**Observed.** All three engines changed. Simplify's Render column belongs to no one engine. Both machines agree, and EEVEE and Workbench both render in a background run on macOS.

**Hand test 15 — which settings each version has.** The full property sweep, run on macOS in all four versions and compared, produced **exactly the same list of names** as the sweep on Linux. **Observed.** The differences between the versions' findings are differences in Blender, not in the operating system.

**Hand test 31 — the texture cache.**

```
HAS-use_texture_cache False HAS-use_auto_generate_texture_cache False
```

**Observed.** Neither setting exists in this version on macOS either, so the rest of the test cannot be carried out here. The same file rendered twice with nothing changed gave `IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`, and no folder appeared beside the images.

**Two tests were not run on this machine, and the reason is said here rather than left as a silence.** Hand test 23 (whether the thread count changes anything under EEVEE or Workbench) and hand test 24 (whether Persistent Data changes the time or the memory of an animation) rest entirely on times and memory figures. The Mac was shared with other topics' work throughout, so nothing was timed and nothing measured memory here. Both were run on the rented Linux machine and are recorded above. **What they leave open on macOS is left open.**

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.1/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 34 — which Cycles backends this build will even accept, and which have a device behind them.** The citation check sent this back because the two machines had been asked different questions: the Mac was asked which values the backend setting will take, and the Linux machine which backends have a card behind them. Both were asked here, of this machine, in one sitting, with each value set in a **fresh Blender** and the list read with `get_devices_for_type` — the call that takes the backend as its argument — rather than from the `p.devices` collection. All four versions printed the same answer; this is it:

```
TYPE NONE   ACCEPTED   DEVICES-FOR-TYPE []
TYPE CUDA   ACCEPTED   DEVICES-FOR-TYPE [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX  ACCEPTED   DEVICES-FOR-TYPE [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE HIP    ACCEPTED   DEVICES-FOR-TYPE []
TYPE ONEAPI ACCEPTED   DEVICES-FOR-TYPE []
TYPE METAL  COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')
```

**Observed, and the two questions come apart in the answer.** This machine **accepts five of the six names and refuses one by name**; it **has devices behind two of them**; and a render told to use one of the other three stops with `Found no Cycles device of the specified type` and writes no image. Set beside the Creator's Mac, which accepts `NONE` and `METAL` and refuses the other four by name, **which backends a build will take is an operating-system fact**, and the two machines differ in kind and not only in hardware.

**A disagreement is recorded here and is not resolved.** The line above in this document says the Linux machine offers `NONE`, `CUDA` and `OPTIX` — three. The citation check's run of the acceptance question got five; this run got five as well. Both runs are written out in the hand tests, word for word, and **neither has been rubbed out by the other**. It is possible the standing sentence means the narrower question — which backends have a card behind them, where the answer really is two — and this test does not choose. It only makes sure the two questions are asked separately from now on.

**Hand test 35 — which of the settings named in one version's findings exist in the other three, with the sweep written out exactly.** The citation check sent hand test 15 back over its totals, which three sittings had reported three ways. The sweep was run again with the rule stated: five blocks named exactly (`scene.render`, `scene.cycles`, `scene.eevee`, `scene.display`, and the Cycles add-on's preferences), each counted twice, once including its own `rna_type` entry and once excluding it. This version: `scene.render` 98 / 97, `scene.cycles` 106 / 105, `scene.eevee` 55 / 54, `scene.display` 10 / 9, the Cycles preferences 9 / 8 — **278 counting `rna_type`, 273 not counting it**. Counting it gives 290, 276, 278 and 286 across the four versions, which is the citation check's figure; not counting it gives 285, 271, 273 and 281, which is the Mac's; and the first Linux sitting's 286, 272, 274 and 282 are neither and **cannot be reproduced**. **The finding is untouched by that**: the set of names not present in all four versions came back as the same twenty-eight, on the same versions, including `scene.eevee.fast_gi_thickness_far` — in 4.5, 5.0 and 5.1, absent from 5.2. **Still owed on the Creator's Mac**, as a repeat under the stated rule.



## 5. Can their values be read out of a file?

**LABEL: mixed — documented that they can be read from a loaded file through Python; not in the sources for reading a blend-file without Blender.** Three separate answers, because "out of a file" has three cases.

### 5a. Values stored in the blend-file: yes, through Blender's Python API

The API documents every one of the engine-neutral settings as a readable property, with its type, range and default. Quoted from `https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html` (read 2026-07-30):

> "threads — Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) (in [1, 1024], default 1)"
> "threads_mode — Determine the amount of render threads used (default 'AUTO')"
> "use_persistent_data — Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage (default False)"
> "resolution_x — Number of horizontal pixels in the rendered image (in [4, 65536], default 1920)"
> "resolution_y — Number of vertical pixels in the rendered image (in [4, 65536], default 1080)"
> "resolution_percentage — Percentage scale for render resolution (in [1, 32767], default 100)"
> "use_simplify — Enable simplification of scene for quicker preview renders (default False)"
> "simplify_subdivision_render — Global maximum subdivision level during rendering (in [0, 32767], default 0)"
> "simplify_child_particles_render — Global child particles percentage during rendering (in [0, 1], default 0.0)"
> "engine — Engine to use for rendering (default 'BLENDER_EEVEE')"

and the EEVEE pool sizes from `bpy.types.SceneEEVEE`, quoted in answer 4b.

And the access route (`https://docs.blender.org/api/5.1/bpy.data.html`, read 2026-07-30):

> "Data Access (bpy.data)
>
> This module is used for all Blender/Python access."
>
> "# Print all scene names in a list.
> print(bpy.data.scenes.keys())"

**Cycles' settings are readable the same way but are undocumented.** **Observed**, `Blender Sources/cycles_5.1/properties.py`:

> `bpy.types.Scene.cycles = PointerProperty(name="Cycles Render Settings", description="Cycles render settings", type=cls,)`

So `scene.cycles.tile_size`, `scene.cycles.device`, `scene.cycles.debug_use_compact_bvh` and the rest read out exactly like any other property. **Inferred, step stated:** because that pointer is created inside `CyclesRenderSettings.register()`, the `cycles` attribute exists only while the Cycles add-on is registered — the file also contains `del bpy.types.Scene.cycles` in the matching `unregister`. The step a reader supplies is that a blend-file opened with Cycles disabled would therefore have no readable `scene.cycles`. The sources do not state that consequence; they state the registration and deregistration, and I drew the line between them. This is worth a hand-test.

Put together with answer 3a's `--python-expr`, this gives the practical route: start Blender with the file and a Python expression that prints the values. The manual documents both halves. I did not find any worked example of doing exactly this in the 5.1 manual, so **the combination is inferred, not documented.**

### 5b. Values from another blend-file, without making it the current one: documented, with a caveat

> "load(filepath, *, link=False, pack=False, relative=False, set_fake=False, recursive=False, reuse_local_id=False, assets_only=False, clear_asset_data=False, create_liboverrides=False, reuse_liboverrides=False, create_liboverrides_runtime=False)
>
> Returns a context manager which exposes 2 library objects on entering.
> Each object has attributes matching bpy.data which are lists of strings to be linked.
>
> Parameters:
>
> filepath (str | bytes) – The path to a blend file."

Source: `https://docs.blender.org/api/5.1/bpy.types.BlendDataLibraries.html`, read 2026-07-30.

**Not covered, and this is the caveat:** the documentation says the objects expose "lists of strings to be linked" — that is, names. It does not say the settings themselves are readable from that context manager without actually linking the scene in. I did not find any source that walks through reading a render setting this way.

### 5c. Reading a blend-file without running Blender: not in the sources

**LABEL: not in the sources.**

Where I looked, and for what. I searched the whole 5.1 manual's `files/` and `advanced/` sections for *file format*, *SDNA*, *blend-file format* and *binary format*: the only hits were about Alembic, STL, PLY, image and video formats, plus one line on the extensions page about a manifest version, and one on the compatibility page about a "file format version". Nothing describes the structure of a `.blend` or a way to parse one. I searched the API documentation for `open_mainfile` (which exists, but is an operator that loads a file into a running Blender) and read the `bpy.data` and `BlendDataLibraries` pages in full. There is no documented library, tool, or command that reads settings out of a `.blend` without Blender itself.

The closest thing the sources offer is Blender built as a Python module (`https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html`, read 2026-07-30):

> "Blender supports being built as a Python module,
> allowing import bpy to be added to any Python script, providing access to Blender's features."

with two limits that matter here, quoted from the same page:

> "Note
>
> Blender as a Python Module isn't provided on Blender's official download page.
>
> A pre-compiled bpy module is available via PIP."

> "The user's startup and preferences are ignored to prevent your local configuration from impacting script behavior.
> The Python module behaves as if --factory-startup was passed as a command line argument."

That is still Blender doing the reading, just linked into another program.

### 5d. Values that are not in the blend-file at all

The Preferences settings of answer 1c live in `./config/userpref.blend`, not in your scene file (`https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html`). They are readable through `bpy.types.Preferences` — the API documents an `addons` collection there, and Cycles' own files show the access pattern, **observed** in `Blender Sources/cycles_5.1/ui.py`:

> `return context.preferences.addons[__package__].preferences.compute_device_type`

So the chosen Cycles compute backend reads out of the *preferences*, on the running Blender, not out of the scene file.

And the command-line-only settings of answer 1d — `-t`, `--qos`, `--gpu-backend` — are in no file at all. They exist for the length of one run. The API documentation confirms at least one of them has no Python counterpart: "there are some command line arguments that don't have equivalents in Blender's Python API (--threads and --log for example)."

**Not covered.** Whether `bpy.data.libraries.load` reaches render settings without linking. Whether the value of `-t` given on a command line can be read back from anywhere at all — the sources are silent, and the quote above suggests not. Whether `scene.cycles` survives a file opened without the Cycles add-on. Nothing here addresses reading values out of a file that a *newer or older* Blender wrote.

**Outside the interface.** Everything in 5a and 5b is Python and works identically in background mode; that is its main use. 5d's preferences are the exception, and the exception is documented: as a Python module Blender "behaves as if --factory-startup was passed", so the preferences you would read there are the factory ones unless you run `bpy.ops.wm.read_userpref()`, which the same page shows.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Operating system.** No source I read marks any of this as platform-specific. `bpy.data`, `bpy.data.libraries.load`, `--python-expr` and the preferences access are documented without platform notes. The path `./config/userpref.blend` is given relative to "user directories", whose absolute location the manual gives per platform on the same page — I did not need those absolute paths for this answer and did not check them.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.1/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 11 — which settings survive being saved into a file and read back.** Done by setting eighteen settings to unusual values, saving, and reading them back in a fresh run:

```
READBACK render.threads_mode                    FIXED
READBACK render.threads                         3
READBACK render.use_persistent_data             True
READBACK render.resolution_percentage           37
READBACK render.use_simplify                    True
READBACK render.simplify_subdivision_render     1
READBACK render.use_high_quality_normals        True
READBACK render.use_single_layer                True
READBACK cycles.tile_size                       256
READBACK cycles.use_auto_tile                   False
READBACK cycles.samples                         11
READBACK cycles.max_bounces                     2
READBACK cycles.texture_limit_render            512
READBACK cycles.debug_use_spatial_splits        True
READBACK cycles.debug_use_compact_bvh           True
READBACK cycles.debug_use_hair_bvh              False
READBACK cycles.debug_bvh_time_steps            4
READBACK cycles.time_limit                      12.5
READBACK eevee.taa_render_samples               23
```

**Observed.** All eighteen came back at the value written.

**Hand test 10 — whether another file's render settings can be read without making it the current file.** Done by `blender -b --factory-startup --python <script> -- <the other file>`, where the script opens `bpy.data.libraries.load`, tries to reach a render setting on what the block hands back, then assigns `dst.scenes = src.scenes` and reads the settings after the block:

```
ENTRY 'Scene' str
FAILED AttributeError 'str' object has no attribute 'render'
LOADED-SCENE 'Scene.001'
  render.resolution_x 160 threads_mode FIXED threads 3
  cycles.samples 11 tile_size 256 device CPU
  eevee.taa_render_samples 23
CURRENT-FILE-SCENES ['Scene', 'Scene.001']
```

**Observed.** Inside the block what comes back is a name and nothing else. After the block every render setting of the other file's scene reads back at the value saved into it, and that scene is now present in the current session alongside the original.

**Hand test 9 — whether a scene's Cycles settings can be read with the Cycles add-on switched off, and whether they can be read while the engine is EEVEE.** Done in a settings folder of its own, by calling `bpy.ops.preferences.addon_disable(module="cycles")` and saving the preferences, then starting again against that folder:

```
(a) at once, in the same session
    AFTER-TYPE-HAS-CYCLES False
    AFTER-SCENE-HAS-CYCLES False
    AFTER-READ-FAILED AttributeError 'Scene' object has no attribute 'cycles'
(b) a fresh run against that settings folder, reading a file whose engine is Cycles
    ADDONS ['bl_pkg', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
    TYPE-HAS-CYCLES False
    SCENE-HAS-CYCLES False
    READ-FAILED AttributeError 'Scene' object has no attribute 'cycles'
    ENGINE BLENDER_EEVEE
(c) the same file with the add-on on
    TILE-SIZE 2048 SAMPLES 64 DEVICE CPU
    ENGINE CYCLES
(d) a file whose engine is EEVEE, add-on on
    TILE-SIZE 2048 SAMPLES 64 DEVICE CPU
    ENGINE BLENDER_EEVEE
```

**Observed.** Two things. Cycles' settings cannot be read at all with the add-on off. And with the add-on off, the same file that reads back as `CYCLES` on line (c) reads back as `BLENDER_EEVEE` on line (b) — so the engine stored in the file is not readable either. With the add-on on, Cycles' values read back whichever engine the file names.

**Hand test 14 — what each version reads out of a file saved by 4.5 with Use Tiling off.** Done to one file, saved by Blender 4.5.12 with Use Tiling off, Tile Size 256 and Samples 11, opened unchanged by each of the four Blenders. This version read:

```
OLDFILE use_auto_tile True tile_size 8192 samples 11
```

The four together:

```
read by 4.5: OLDFILE use_auto_tile False tile_size 256 samples 11
read by 5.0: OLDFILE use_auto_tile True tile_size 8192 samples 11
read by 5.1: OLDFILE use_auto_tile True tile_size 8192 samples 11
read by 5.2: OLDFILE use_auto_tile True tile_size 8192 samples 11
```

**Observed.** Reading Tile Size out of a file saved by an older Blender does not always give the value that was saved.

**Hand test 2, the reading side — what `-t` does to the stored value.** With a file saved at Threads Mode Fixed and Threads 1, `-t 8` makes the property read back `FIXED 8`, on either side of the file name; `-t 0` leaves it at `FIXED 1`; and with no file at all the property reads `AUTO 24` on this twenty-four-processor machine. So a value read out of a running Blender is not always the value in the file.

**Hand test 6, the reading side — what `--cycles-device` does to the stored value.** Nothing, at any moment: `scene.cycles.device` read back `CPU` before a render and after one, with and without the override.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.1.2 at `/opt/blender/5.1/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.1/` or `/root/render-demands/prefs-card-cuda/5.1/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 6, run again now the card works — what a file's stored Device is worth as a reading.** A file saved with Cycles as the engine and Device CPU was read back with `import bpy; print("DEVICE", bpy.context.scene.cycles.device)`, in runs where `--cycles-device` had moved the work onto the card:

```
(a) no argument                                            : DEVICE CPU
(b) -- --cycles-device OPTIX, read before any render       : DEVICE CPU
(c) -- --cycles-device OPTIX, read after a render          : DEVICE CPU   (and the render ran on the card)
(d) -- --cycles-device CPU, read after a render            : DEVICE CPU
(f) preference CUDA, -- --cycles-device OPTIX, after render: DEVICE CPU   and the preference still reads CUDA
```

**Observed.** The value read out of a file is the value saved in the file, and stays so even when a render in the same session ran somewhere else entirely. That is the answer to this question, and it comes with a warning attached: **a file's stored Device tells you what the file says, not what rendered.** Reading it after a render does not correct it. The same holds for the saved backend preference, which also reads back unchanged after a run that overrode it. Anything that needs to know which device actually did the work cannot get it by reading either of them back.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.1/`.

**Asked which compute backends it has, this Blender offers two and refuses the rest:**

```
TYPE NONE  DEVICES []
TYPE METAL DEVICES [('Apple M1 Pro (GPU - 14 cores)', 'METAL', True), ('Apple M1 Pro', 'CPU', False)]
TYPE CUDA  COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "CUDA" not found in ('NONE', 'METAL')
```

and `OPTIX`, `HIP` and `ONEAPI` were refused in exactly the same words. So on this operating system the backend setting has only two values it will take.

**One limit runs through everything below, and it is why not one number here is a time or a memory figure.** The Mac was shared with other topics' work while these ran — another topic's Blender was found running on it partway through — so **nothing was timed and nothing measured memory**. The rented Linux machine carries this round's timings, and it is the kind of computer a Smoozy render actually happens on. Where a test's only possible evidence was a time or a memory figure, it was not run here at all, and that is said under the question it would have answered. Every picture comparison below compares the pixels and never the saved file, because Blender writes the time of day into every image it saves.

The full list of tests, with the procedure for each, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 11 — which settings survive being saved into a file and read back.** Eighteen settings written to unusual values, saved, and read back out of the file in a fresh run:

```
READBACK render.threads_mode                      FIXED
READBACK render.threads                           3
READBACK render.use_persistent_data               True
READBACK render.resolution_percentage             37
READBACK render.use_simplify                      True
READBACK render.simplify_subdivision_render       1
READBACK render.use_high_quality_normals          True
READBACK render.use_single_layer                  True
READBACK cycles.tile_size                         256
READBACK cycles.use_auto_tile                     False
READBACK cycles.samples                           11
READBACK cycles.max_bounces                       2
READBACK cycles.texture_limit_render              512
READBACK cycles.debug_use_spatial_splits          True
READBACK cycles.debug_use_compact_bvh             True
READBACK cycles.debug_use_hair_bvh                False
READBACK cycles.debug_bvh_time_steps              4
READBACK cycles.time_limit                        12.5
READBACK eevee.taa_render_samples                 23
```

**Observed.** All eighteen came back at the value that was written. The rented Linux machine got the same nineteen lines.

**Hand test 10 — whether another file's render settings can be read without making it the current file.** `bpy.data.libraries.load` against a file whose settings are all unusual:

```
ENTRY 'Scene' str
FAILED AttributeError 'str' object has no attribute 'render'
LOADED-SCENE 'Scene.001'
  render.resolution_x 160 threads_mode FIXED threads 3
  cycles.samples 11 tile_size 256 device CPU
  eevee.taa_render_samples 23
CURRENT-FILE-SCENES ['Scene', 'Scene.001']
```

**Observed.** Inside the block the file hands back a name and nothing else — a plain piece of text with no settings on it. After the block the other file's scene is present and every one of its render settings reads back at the value saved into it, arriving as a second scene alongside the current one. The same eight lines on the Linux machine.

**Hand test 14 — what the newer versions read out of a file saved by 4.5 with Use Tiling switched off.** One file, saved on this Mac by Blender 4.5.12 with Use Tiling off, Tile Size 256 and Samples 11, read by each of the four Blenders on this Mac:

```
read by 4.5: OLDFILE use_auto_tile False tile_size 256 samples 11
read by 5.0: OLDFILE use_auto_tile True tile_size 8192 samples 11
read by 5.1: OLDFILE use_auto_tile True tile_size 8192 samples 11
read by 5.2: OLDFILE use_auto_tile True tile_size 8192 samples 11
```

**Observed.** The same file on disk gives Use Tiling off and Tile Size 256 to 4.5, and Use Tiling on and Tile Size 8192 to the three newer versions. Samples was untouched. **The rented Linux machine got exactly the same four lines**, so this is a change in Blender and not something about one machine.

**Hand test 6 — whether the stored device can be trusted to say what rendered.** With a graphics processor present and working, the scene's stored Device read back `CPU` in every one of five runs — with no argument, with `--cycles-device METAL` read before a render, with it read after a render, with `--cycles-device CPU` after a render, and with the argument set against a settings folder whose saved backend was Metal. The saved backend preference read back unchanged too.

**Observed.** **A file's stored Device tells you what the file says, never what rendered.** Reading it after the render does not correct it. Hand test 28 above shows those runs really did move between the processor and Apple's graphics while this value stayed put. The same on the Linux machine, with its own card.

**Hand test 8 — where the preferences are read from.** A plain background run reads the saved preferences; `--factory-startup` does not. Both were shown here, with a control that separates "read the saved file" from "fell back to the factory value": on this Mac the factory value of the Cycles backend is `METAL`, where on the Linux machine it is `NONE`, so the same test prints different words on the two machines while the rule behind it is the same.

**Hand test 9 — one thing that stops these values being readable at all.** With the Cycles add-on switched off, `bpy.context.scene.cycles` cannot be reached — `AttributeError 'Scene' object has no attribute 'cycles'` — and the engine stored in the file reads back as the EEVEE name rather than `CYCLES`. **Observed.** Reading Cycles' settings out of a file depends on the add-on being on in the Blender doing the reading.

**Hand test 2 — the stored thread setting.** A file saved with Threads Mode Fixed and Threads 1 read back `THREADS FIXED 1`; with `-t 8` given it read `THREADS FIXED 8`; with `-t 0` it read `THREADS FIXED 1` again. **Observed.** This one *is* written by the command line, unlike the device — which is why the two cannot be treated alike.


## Things a person with Blender 5.1 open could settle in a few minutes

This list has been pruned three times. Everything it held that a rented Linux machine could settle was run on 2026-08-06 and is written into the answers above, under the question each one answers. T8, which was left waiting for a working graphics card, was run on 2026-08-07, once that machine's card had been repaired. The one list covering every version, with the full procedure for each test and the results, is `Audits/Blender Research/What a render demands/Hand tests.md`. **A third sitting, on the Creator's own Mac on 2026-08-07, ran every item that was still owed there, including the two that need a window — which no machine on this project but that one has.** Only genuinely open items are left here.

**Nothing on this list is still open, and one thing on it can never be run.**

**Run and answered above:** T1 (threads and the image), T2 (Tile Size and the BVH options and the image), T3 (`-t` against the file), T4 (EEVEE and Workbench honouring Threads and Persistent Data), T5 (`scene.cycles` with the add-on off), T6 (`bpy.data.libraries.load`), **T7 (which acceleration-structure controls appear for which device)** and **T8 (`--cycles-device` against both the file and the preferences)** — which it beats, in both directions.

**On T7, what was run and what was not.** All four combinations this project can reach were run in a window on the Creator's Mac on 2026-08-07 and are recorded under question 4 above. The controls are **not** the same in every combination: the processor's column shows Use Spatial Splits and Use Compact BVH, and a file set to GPU Compute with the Metal backend saved shows Use Spatial Splits, BVH Time Steps and Use Curves BVH instead. **The OptiX half of T7 can never be run by this project**, and that is a fact about the equipment rather than a task still waiting: OptiX exists only on the machine with the NVIDIA card, that machine has no display server at all, and no Apple computer can reach OptiX. So the panel's behaviour under OptiX comes back untested rather than answered.

**All of that has now been run, on the Creator's own Mac on 2026-08-07, and is recorded above under the question each one answers.** Apple's own graphics render a different picture from the processor, exactly as the NVIDIA card does; `--cycles-device METAL` beats both the file and the saved preferences, in both directions; the three engines really do show different Performance panels; and the Acceleration Structure controls really do change with the device. **Nothing on this subject is still owed on either machine.**

Three tests were deliberately **not** repeated on the Mac, and the reason is written here rather than left as a silence: the ones whose only possible evidence is a time or a memory figure — whether `-t` changes how much of the processor a Cycles render uses, whether the thread count changes anything under EEVEE or Workbench, and whether Persistent Data changes the time or the memory of an animation. The Mac was shared with other topics' work throughout, so a figure taken there would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. What they leave open on macOS is left open and is not guessed at.

---

2026-08-06 — Opus 5 (Claude Code) — What a render demands, Blender 5.1

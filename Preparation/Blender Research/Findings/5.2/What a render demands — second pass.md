# What a render demands — Blender 5.2 — second pass

Sources consulted: Blender's 5.2 manual, Blender's 5.2 Python API documentation, the 5.2 release notes, and Cycles' own files as shipped with 5.2 — all four read on **2026-07-30**. Blender's requirements page was read on **2026-08-01**.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

A note that applies throughout: the requirements page (`https://www.blender.org/download/requirements/`, read 2026-08-01) **never names Blender 5.2**. I searched it for the string `5.2` and found no occurrence; the only version it names in so many words is `4.5 LTS`. So it cannot speak for the version researched here, and I have not used it as evidence for any answer below. Where a question needed operating-system facts, I took them from the 5.2 manual only.

---

## 1. What settings, if any, change how much a render demands of the computer it runs on?

**LABEL: documented** (the individual settings and what they cost), with the groupings and the "these are the ones" framing being my own arrangement of documented items.

There are many. Blender collects most of them into a panel that says so outright.

### 1a. The panel that exists for this purpose

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
> Uses settings to decrease memory consumption at the cost of slower renders."

Source: manual, `https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html`. Read 2026-07-30.

Cycles' own files show exactly which properties that preset writes out — this is the closest thing to a definitive list of "the demand settings" that exists in the sources. **Label for this item: observed.**

> ```
>     preset_values = [
>         "render.threads_mode",
>         "render.use_persistent_data",
>         "cycles.debug_use_spatial_splits",
>         "cycles.debug_use_compact_bvh",
>         "cycles.debug_use_hair_bvh",
>         "cycles.debug_bvh_time_steps",
>         "cycles.tile_size",
>     ]
> ```

File: `Blender Sources/cycles_5.2/presets.py`, class `AddPresetPerformance`, lines 103–111. Read 2026-07-30.

### 1b. Settings that belong to the scene, not to any one engine

From the Cycles Performance page (address as above) and the EEVEE and Workbench Performance pages (`.../render/eevee/render_settings/performance.html`, `.../render/workbench/performance.html`), all read 2026-07-30:

- **Threads Mode** — "Method to determine the maximum number of CPU cores to use while rendering." Auto-Detect "chooses the amount of threads to match the number of logical processors on your computer"; Fixed lets you "Manually choose the maximum number threads to use for rendering. This can be useful for example, if you want to use your computer while rendering, you can set the property to a thread count lower the amount of logical processors on your computer."
- **Threads** — "The maximum number of CPU cores to use simultaneously while rendering."
- **Persistent Data** — "Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage while performing other tasks in Blender."
- **Texture Cache** — "Reduce memory usage and startup time on scenes with many image textures, at the cost of a small rendering performance impact and increased disk space usage." With **Auto Generate**: "Automatically generate and update .tx files for images when rendering."
- **Compositor → Device** (CPU or GPU) and **Precision GPU**; **Denoise Nodes → Denoising Device**, **Preview Quality**, **Final Quality** ("High: Produces the highest quality output at the cost of long processing times… Fast: Produces an output quickly at a noticeable cost of quality").
- **High Quality Normals** (EEVEE/Workbench pages) — "Uses higher precision normals and tangents which can improve visual quality for dense meshes with high frequency textures at the cost of memory."
- **Anisotropic Filtering** (EEVEE/Workbench pages) — "Increasing the setting may have a small impact on performance."

The API documentation confirms these are properties of the scene's render settings and gives their ranges and defaults (`https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html`, read 2026-07-30):

> "threads
>
> Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) (in [1, 1024], default 1)"

> "threads_mode
>
> Determine the amount of render threads used (default 'AUTO')"

> "use_persistent_data
>
> Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage (default False)"

> "use_texture_cache
>
> Load texture tiles at appropriate resolution on demand to reduce memory usage. This avoids loading all textures into memory, at the cost of extra disk space and some performance (default True)"

Also scene-level, from other manual pages:

- **Resolution X, Y** and **Percentage** — "The number of pixels horizontally and vertically in the image"; "Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image." (`.../render/output/properties/format.html`). API: `resolution_x` "(in [4, 65536], default 1920)", `resolution_percentage` "Percentage scale for render resolution (in [1, 32767], default 100)".
- **Render Region** and **Crop to Render Region** — "Renders just a portion of the view instead of the entire frame." (same page)
- **Simplify** — "Max Subdivision: Maximum number of subdivision by the Subdivision Surface modifiers"; "Child Particles: Show only a subset of all child hairs and particles"; "Texture Resolution: Scale down image textures to reduce memory usage"; "Texture Size Limit: Automatically scales textures down so that they are no larger than the values chosen. This can help reduce computer memory resources when rendering large scenes with huge textures." (`.../render/cycles/render_settings/simplify.html`)
- **Render passes** — "These passes increase memory usage and render time. Enable them only when denoising in the Compositor is required." (`.../render/layers/passes.html`)
- **View Layer → Samples override** — "Lowering the number of samples for background or utility passes can speed up render times, while still using higher samples for beauty passes." (`.../render/layers/override.html`)
- **Lock Interface** — "Lock interface during rendering in favor of giving more memory to the renderer." (`.../interface/window_system/topbar.html`)

### 1c. Settings that belong to Cycles

From `.../render/cycles/render_settings/performance.html`:

- **Tile Size** — "Render high resolution images in tiles of this size, to reduce memory usage of render passes. Tiles are cached to disk while rendering to save memory. Lowering the tile size also reduces texture cache memory usage… Tile size has an impact on performance, and the default 2048 pixels provides a good balance for most scenes. When running out of memory, lowering to 1024 or 512 pixels helps reduce memory usage, typically at a small performance cost."
- **Use Spatial Splits** — "Spatial splits improve the rendering performance in scenes with a mix of large and small polygons. The downsides are longer BVH build times and slightly increased memory usage."
- **Use Curves BVH** — "Disabling this option will reduce memory, at the cost of increasing curve render time."
- **BVH Time Steps** — "Split BVH primitives by this number of time steps to speed up render time at the expense of memory."
- **Use Compact BVH** — "Use a more compact BVH structure, which can reduce RAM usage but render slower."

Their kinds, defaults and ranges, **observed** in `Blender Sources/cycles_5.2/properties.py` (read 2026-07-30):

> ```
>     debug_use_spatial_splits: BoolProperty(
>         name="Use Spatial Splits",
>         description="Use BVH spatial splits: longer builder time, faster render",
>         default=False,
>     )
>     debug_use_hair_bvh: BoolProperty(
>         name="Use Curves BVH",
>         description="Use special type BVH optimized for curves (uses more ram but renders faster)",
>         default=True,
>     )
>     debug_use_compact_bvh: BoolProperty(
>         name="Use Compact BVH",
>         description="Use compact BVH structure (uses less ram but renders slower)",
>         default=False,
>     )
>     debug_bvh_time_steps: IntProperty(
>         name="BVH Time Steps",
>         description="Split BVH primitives by this number of time steps to speed up render time in cost of memory",
>         default=0,
>         min=0, max=16,
>     )
> ```
(lines 932–952)

> ```
>     tile_size: IntProperty(
>         name="Tile Size",
>         default=2048,
>         description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
>         min=8,
>         max=8192,
>     )
> ```
(lines 1084–1090)

From `.../render/cycles/render_settings.html` — the choice of processor:

> "Device
>
> Specifies the compute device used for rendering in Cycles. The choice of device can significantly affect render performance, GPU Compute is often faster ."

Observed, `properties.py` lines 30–34 and 387–393:

> ```
> enum_devices = (
>     ('CPU', "CPU", "Use CPU for rendering"),
>     ('GPU', "GPU Compute",
>         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
> )
> ```
> ```
>     device: EnumProperty(
>         name="Device",
>         description="Device to use for rendering",
>         items=enum_devices,
>         default='CPU',
>         update=update_render_passes,
>     )
> ```

From `.../render/cycles/render_settings/sampling.html`:

- **Render (Max) Samples** — "The number of paths to trace per pixel in the final render. A higher number results in a cleaner image at the cost of a longer render time." Observed default: `default=4096`, `min=1, max=(1 << 24)` (`properties.py` lines 485–490).
- **Time Limit** — "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit." Observed: `time_limit: FloatProperty(… default=0.0 …)` (`properties.py` lines 520–528).
- **Noise Threshold / Min Samples** (adaptive sampling) — "cutting short the sampling process in areas that have become less noisy than the specified threshold value"; "lower values meaning better quality but longer render times."
- **Denoise (Render)**, **Denoiser**, **Passes**, **Prefilter**, **Quality**, **Use GPU** — "Use GPU: Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled."
- **Path Guiding / Training Samples** — "Higher values can lead to a minor increases in guiding quality but with increased render times." Note: "Path guiding is only available when rendering on a CPU."
- **Light Tree** — "This can significantly reduce noise, at the cost of a somewhat longer render time per sample."
- **Light Threshold** — "Probabilistically terminates light samples when the light contribution is below the threshold. This avoids wasting time on lights that contribute little to the image."
- **Scrambling Distance / Multiplier** — "A technique that reduces the randomness between pixels in an attempt to improve GPU rendering performance, at the cost of potential artifacts."
- **Min Light Bounces** — "Setting this higher gives less noise, but may also increase render time considerably."
- **Sample Subset (Offset, Length)** — "Only render a subset of the samples… A typical use case is to distribute the rendering of a single frame over multiple machines."
- **Layer Samples** — how per-view-layer sample overrides are applied.

From `.../render/cycles/render_settings/light_paths.html`:

- **Max Bounces (Total, Diffuse, Glossy, Transmission, Volume)** — "Maximum number of light bounces. For best quality, this should be set to the maximum. However, in practice, it may be good to set it to lower values for faster rendering."
  Observed: `max_bounces: IntProperty(… min=0, max=1024, default=12)` (`properties.py` lines 751–756).

From `.../render/cycles/render_settings/volumes.html`:

- **Step Rate Render** — "Global multiplier on the step size for all volumes in renders. Increase to reduce render time, at the cost of losing detail."
- **Max Steps** — "Maximum number of steps through the volume before giving up, to protect from extremely long render times with big objects or small step sizes."

From `.../render/cycles/render_settings/subdivision.html`:

- **Dicing Rate Render**, **Offscreen Scale** ("Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory"), **Max Subdivisions**.

From `.../render/cycles/render_settings.html` — **Open Shading Language**: "Rendering performance is typically slower than with built-in nodes due to shader compilation and interpretation."

### 1d. Settings that belong to EEVEE

From `.../render/eevee/render_settings/performance.html`:

- **Shadow Pool** — "A bigger pool size allows for more shadows in the scene but might not fit into GPU memory and decreases performance."
- **Light Probes Volume Pool** — "A bigger pool size allows for more irradiance grids in the scene but might not fit into GPU memory and decreases performance."

From `.../render/eevee/render_settings/sampling.html`:

- **Render → Samples**, **Shadows → Rays**, **Steps** ("Higher step count results in softer shadows but have a higher cost"), **Volumetric Shadows** ("This is a very computationally expensive option"), **Resolution** ("Resolution percentage of shadow maps"), **Light Threshold** ("Lower values improve performance").

From `.../render/eevee/render_settings/raytracing.html`:

- **Method** (Light Probe "has the lowest tracing cost"; Screen-Trace), **Resolution** ("Lower options will be faster and use less memory but will produce blurrier results"), **Precision** ("increase the performance cost"), and the Fast GI **Resolution**, **Rays**, **Steps**, **Precision** ("Increased precision also increases performance cost").

From `.../render/eevee/render_settings/volumes.html`:

- **Resolution** and **Steps** — "EEVEE uses several 3D textures which have a high video memory usage. The texture dimensions can be tweaked using the Resolution and Steps parameters."

From `.../render/eevee/render_settings/motion_blur.html`:

- **Steps** — "This controls the number of steps used by the accumulation blur and thus its accuracy. More steps means longer render time." and "Each step corresponds to a full scene re-evaluation and can add a lot of overhead to the render time."

From `.../render/eevee/light_probes/volume.html` — light probe volume resolution: "Higher values increase quality, but have a huge impact on memory usage."

API confirmation of the EEVEE ones (`https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html`, read 2026-07-30):

> "shadow_pool_size
>
> Size of the shadow pool, a bigger pool size allows for more shadows in the scene but might not fit into GPU memory (default '512')"

> "taa_render_samples
>
> Number of samples per pixel for rendering (in [1, inf], default 64)"

> "volumetric_samples
>
> Number of steps to compute volumetric effects. Higher step count increase VRAM usage and quality. (in [1, 256], default 64)"

### 1e. Settings that belong to Workbench

`.../render/workbench/sampling.html`: the anti-aliasing method — "No Anti-Aliasing", "Single Pass Anti-Aliasing", "Multisample" with "5, 8, 11, 16, 32" samples. "The setting for viewport rendering and final rendering is saved per scene."

### 1f. Settings that are not in the scene file at all

These live in the Preferences (`.../editors/preferences/system.html`, read 2026-07-30):

- **Cycles Render Devices** — "Changes the Computing Device the Cycles render engine uses to render images."
- **Distribute Memory Across Devices** — "Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes."
- **Embree on GPU** — "Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance."
- **HIP RT** — "Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above."
- **MetalRT** — "MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases."
- Shader compilation **Threads / Subprocesses** — "Increasing the number can reduce shader compilation time at the cost of higher memory usage."
- **Memory Cache Limit**, **Undo Memory Limit**.

### 1g. Settings given only when Blender is started

From `.../advanced/command_line/arguments.html`, read 2026-07-30:

- `-t, --threads <threads>` — "Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count."
- `--cycles-device <device>` — "Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. Append +CPU to a GPU device to render on both CPU and GPU."
- `-E, --engine <engine>` — "Specify the render engine."
- `--gpu-backend` — "Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."
- `--qos <level>` — "Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only). default: Uses the default behavior of the OS. high: Always makes use of performance cores. eco: Schedules Blender threads exclusively to efficiency cores."
- `--disable-depsgraph-on-file-load` — "Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode".

**Release-note statement for 5.2.** The texture cache is new in this release:

> "On scenes with many image textures, the texture cache significantly reduces memory usage and startup time. This comes at the cost of a small rendering performance impact, and increase disk space usage."
> "Tile Size under Performance > Memory can be tweaked to further reduce memory usage. For each render tile only the required image textures are loaded."
> "In the Simplify panel, there is a new Texture Resolution percentage setting, in addition to the existing Texture Limit."

Source: release notes, `https://developer.blender.org/docs/release_notes/5.2/cycles/`. Read 2026-07-30.

**Not covered.** The sources say a setting costs "more memory" or "longer render time" but never give a magnitude, a formula, or a way to predict the demand of a given scene before rendering it. Nothing here settles peak memory, disk use, or how any of these settings interact. I have listed the settings the documentation describes in terms of speed or memory; I cannot show that no other setting anywhere in Blender affects demand — almost any scene content does.

**Outside the interface.** The settings above are scene or preference data and are used by a render started from outside Blender; the manual states this for a background animation render: "-a Render the whole animation using all the settings saved in the blend-file" (`.../advanced/command_line/render.html`, read 2026-07-30). The exceptions are the ones the documentation labels Viewport — Viewport (Max) Samples, Viewport Pixel Size, Simplify's Viewport column, Volume Resolution ("Resolution percentage of volume objects in the viewport"), viewport dicing rate, and Workbench's 3D Viewport anti-aliasing, which the manual calls "a user preference". Those describe the 3D Viewport, not the final render. **Label for this exception: inferred** — the step is that a render started from outside has no 3D Viewport, so a setting the manual scopes to the viewport does not apply to it; the sources do not say this in so many words. One documented limit: "Headless rendering is not supported on headless Windows systems" (`.../render/eevee/limitations/limitations.html`, read 2026-07-30).

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

**Operating system.** Most of the list is stated without any operating-system qualification. Named exceptions in the sources: `--qos` is "(Windows only)"; the Cycles device backends are OS-bound — "CUDA is supported on Windows and Linux", "OptiX is supported on Windows and Linux", "HIP is supported on Windows and Linux", "oneAPI … is supported on Windows and Linux", "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features" (`.../render/cycles/gpu_rendering.html`, read 2026-07-30); MetalRT is Metal-only, HIP RT HIP-only, Embree on GPU oneAPI-only; and EEVEE's headless limit above is Windows-specific. The requirements page does not name 5.2 and so is not evidence here.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

**Where I looked.** Searched the whole 5.2 manual (case-insensitive) for `performance`, `render time`, `memory usage`, `render speed`, `threads`, `tile`, `persistent`, `simplify`, `texture cache`, `headless`, `background mode`, `lock interface`. Opened: `render/cycles/render_settings/performance.txt`, `index.txt`, `sampling.txt`, `simplify.txt`, `light_paths.txt`, `volumes.txt`, `subdivision.txt`, `motion_blur.txt`, `gpu_rendering.txt`; `render/eevee/render_settings/performance.txt`, `sampling.txt`, `raytracing.txt`, `volumes.txt`, `motion_blur.txt`, `render/eevee/limitations/limitations.txt`, `render/eevee/light_probes/volume.txt`; `render/workbench/performance.txt`, `sampling.txt`; `render/output/properties/format.txt`; `render/layers/passes.txt`, `render/layers/override.txt`; `render/render_engines.txt`; `editors/preferences/system.txt`, `editors/preferences/introduction.txt`; `interface/window_system/topbar.txt`; `advanced/command_line/arguments.txt`, `render.txt`; `advanced/blender_directory_layout.txt`; `advanced/scripting/security.txt`. Searched the 5.2 API documentation for `threads`, `persistent`, `simplify`, `texture_cache`, `resolution`, `compositor_device`, `shadow_pool_size`, `taa_render_samples`, `resolution_scale`, and read `bpy.types.RenderSettings.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.RaytraceEEVEE.txt`. Searched all 5.2 release-note files for `performance`, `memory`, `threads`, `tile`, `persistent`, `device`, `faster`, `speed`, and read `cycles.md`. Searched Cycles' files for `tile_size`, `spatial_splits`, `hair_bvh`, `compact_bvh`, `bvh_time_steps`, `texture_cache`, `device`, `persistent`, `samples`, `time_limit`, `max_bounces`, `volume_step_rate`, `dicing_rate`, and read `presets.py` in full and the relevant parts of `properties.py` and `ui.py`. I ruled out the modelling, sculpting, animation, video-editing and grease-pencil chapters after their `performance` hits proved to be about editing tools and playback rather than about a render.

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.2/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 3 — whether `-t` changes how much of the processor a Cycles render actually uses.** Done to a file built from a script and saved: Cycles, Device CPU, 320 by 240, 256 samples, seed 7, denoising off, adaptive sampling off, Threads Mode Auto-Detect; and to a copy of it with Threads Mode Fixed and Threads 1. Six runs of `/usr/bin/time -v blender -b --factory-startup <file> -f 1 -o <folder>/x_ -F OPEN_EXR -x 1`, varying only `-t`. What came back:

```
auto-detect, no -t : Percent of CPU 1191%, wall 0:03.21, peak 327368 kb
-t 1               : Percent of CPU 99%,   wall 0:38.02, peak 315604 kb
-t 2               : Percent of CPU 193%,  wall 0:19.31, peak 317100 kb
-t 0               : Percent of CPU 1260%, wall 0:03.02, peak 327720 kb
Fixed 1 in the file: Percent of CPU 99%,   wall 0:37.40, peak 325236 kb
Fixed 1 with -t 8  : Percent of CPU 678%,  wall 0:05.52, peak 318636 kb
```

**Observed.** The argument reaches the renderer: one thread takes about fifteen times as long as all of them, two threads about half of that, and `-t 0` matches the unrestricted run. The file's own Fixed 1 also reaches it. Note what the processor percentage does and does not show here: with the file asking for one thread, this version reported 99% while still taking the single-thread wall-clock time.

**Hand test 15 — which of the settings named in one version's findings exist in the others.** Done by walking `bl_rna.properties` of `scene.render`, `scene.cycles`, `scene.eevee`, `scene.display` and the Cycles preferences in all four researched versions and comparing the five lists. This version's dump holds 282 properties. Every name that is not in all four:

```
Present in 5.2 and in none of 4.5, 5.0, 5.1:
  scene.render.use_texture_cache, scene.render.use_auto_generate_texture_cache,
  scene.cycles.texture_resolution, scene.cycles.texture_resolution_render,
  scene.cycles.debug_use_texture_cache_eviction, scene.cycles.debug_texture_cache_preserve_unused,
  scene.render.anisotropic_filter, scene.render.save_output, scene.cycles.use_pixel_jitter
Present in 5.1 and 5.2 only: scene.eevee.direct_light_intensity, scene.eevee.indirect_light_intensity
Present in 4.5, 5.0 and 5.1, gone here: scene.eevee.fast_gi_thickness_far
Present in 4.5 only: scene.cycles.feature_set; scene.eevee.gtao_distance, gtao_quality, use_gtao;
  and the eleven bake settings on scene.render
Values that differ: scene.render.compositor_device reads 'GPU' here and 'CPU' in 4.5, 5.0 and 5.1;
  the Cycles preference use_hiprt reads True here and in 5.1, and False in 4.5 and 5.0.
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

**Hand test 7 — what Blender does when the file asks for the graphics card and none can be reached.** Done to a file saved with Cycles as the engine and Device set to **GPU Compute**, on this machine whose card was unreachable. `blender -b --factory-startup gpu.blend -o <folder>/g_ -F OPEN_EXR -x 1 -f 1 -- --cycles-print-stats`, with the Cycles backend preference at its factory value of None. **Exit code 0, and `g_0001.exr` written, 310002 bytes.** A search of the whole output for device, CUDA, OptiX, GPU, error, warning, fall and CPU returned only ordinary progress lines — nothing named a missing card. *Flagged by the citation check, 2026-08-07:* **this half of the test could not have failed in the direction that matters.** "Blender did not complain" and "Blender did not use the card" are different things, and searching printed output cannot tell them apart — hand test 26 establishes that Blender never names the device that rendered, which is exactly why looking for it cannot work. What settles the point is which picture came out, and the citation check ran that on the repaired card: a file saved with Device GPU Compute, in a settings folder naming no backend, matched the processor render exactly (`IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`) and did not match the card render. So the conclusion holds, but it rests on hand test 28 and not on this. Sent to the hand tests to be rebuilt as a picture comparison. Repeated with `CUDA` saved in the preferences, the run again exited 0 and wrote the image, and printed one line about it:

```
00:00.541  cycles           | ERROR CUDA cuInit: Unknown error
```

**Observed.** A render started from outside, asking for the graphics card on a machine that has none it can use, produces the image and — unless the preference names a backend — says nothing at all.

**Hand test 19 — whether a Cycles Time Limit produces a usable image.** Done to a 640 by 480 Cycles scene at 64 samples. Rendered once with Time Limit 0, then again with `--python-expr "import bpy; bpy.context.scene.cycles.time_limit=0.3"`. What came back: no limit `Time: 00:01.66`; limit `Time: 00:00.41`. Both files 4923762 bytes. Compared by pixels, the two are not the same picture: `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.49512332677841187`.

**Observed.** A time limit writes a complete file of the usual size, holding a picture cut short rather than a blank or a missing one.

**Hand test 24 — whether Persistent Data changes the time or the memory of an animation.** Done by rendering the whole ten frames with `-a`, once with `use_persistent_data` off and once on, under EEVEE and under Cycles, timed by the operating system. One run of each, on a shared machine:

```
EEVEE, ten frames, Persistent Data off : 64.03 s wall, 663% cpu, 1832748 kb peak
EEVEE, ten frames, Persistent Data on  : 64.67 s wall, 634% cpu, 1833620 kb peak
Cycles, ten frames, Persistent Data off: 19.14 s wall, 2216% cpu, 370576 kb peak
Cycles, ten frames, Persistent Data on : 16.36 s wall, 2264% cpu, 356792 kb peak
```

**Observed.** On a scene this small and a machine this busy, one run of each cannot separate the time differences from noise. What can be said is that peak memory did not go up when Persistent Data was switched on; under Cycles it came out lower.

**Hand test 16, cost side — what tile size and the acceleration-structure options cost.** Same 640 by 480 scene, one run each, shared machine:

```
base      2.76 s wall, 1519% cpu, 342848 kb peak
tile 64   4.11 s wall, 1092% cpu, 361816 kb peak
tile 8192 2.93 s wall, 1410% cpu, 343336 kb peak
tiling off 3.08 s wall, 1348% cpu, 342308 kb peak
compact BVH 3.27 s wall, 1279% cpu, 343184 kb peak
spatial splits 3.63 s wall, 1142% cpu, 344504 kb peak
```

**Observed.** Recorded as measured; a single run each on a busy machine is not enough to separate small differences from noise.

**Hand test 31 — the texture cache.** Done to a copy of the 640 by 480 Cycles scene with three 1024 by 1024 PNG images written next to it and wired into three materials, 52 kB of image files in all. First a render with nothing changed: `2.65 s wall, 373760 kb peak-rss`. Then the two settings were asked for by name:

```
HAS-use_texture_cache True HAS-use_auto_generate_texture_cache True
```

Then the same frame rendered with both switched on:

```
CACHE-SET True True
2.85 s wall, 399192 kb peak-rss
```

and the folder holding the images was looked at again:

```
drwxr-xr-x 2 root root  4096 Aug  7 00:18 blender_tx
-rw-r--r-- 1 root root 16249 Aug  7 00:18 tex0.png
-rw-r--r-- 1 root root 16248 Aug  7 00:18 tex1.png
-rw-r--r-- 1 root root 16249 Aug  7 00:18 tex2.png
272K	/root/render-demands/t31/5.2/tex
```

The two pictures were then compared by pixels: `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.06419456005096436`.

**Observed.** A folder named `blender_tx` appeared beside the images and the folder grew from 52 kB to 272 kB. On a scene whose textures are far smaller than the machine's memory, peak memory went up rather than down (373760 kb to 399192 kb) and the wall-clock time went up slightly. **The picture changed.** Whether the setting behaves differently on a scene whose textures exceed the machine's memory is a separate question and was not run.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.2/` or `/root/render-demands/prefs-card-cuda/5.2/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 27, cost side — what the card costs and what the processor costs.** One file rendered three times, changing only which device did the work: once with Device CPU and the Cycles backend preference at None, once with Device GPU Compute and the backend CUDA, once with Device GPU Compute and the backend OptiX. In the two card runs only the card was ticked and the processor was left unticked. The file: Cycles, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, OpenEXR at full float with no compression. Timed by the operating system:

```
processor   : Cycles reported Time: 00:01.76, 1518% of a processor, peak memory 348540 kb
card, CUDA  : Cycles reported Time: 00:00.80, 120% of a processor, peak memory 489404 kb
card, OptiX : Cycles reported Time: 00:00.82, 80% of a processor, peak memory 673236 kb
```

**Observed.** Choosing the card rather than the processor changes what the render demands in three ways at once, all in the same direction in this version. It takes less time. It stops asking for the whole processor — about fifteen hundred per cent of one core drops to about one hundred per cent, which is to say the render stops being a processor job. And it asks for **more** ordinary memory, not less, because the card's own working memory is held in addition. This is one small scene on one busy machine; the shape is what is being recorded, not the seconds.

**Hand test 28, memory side — what the saved backend preference takes off the card even when the processor does the work.** Through every run of hand test 28 this Blender was watched for how much card memory it held, by asking the driver which processes held work on the card and how much each held — a reading about this process alone, so the other topic's Blender could not confuse it. On a heavier version of the same file, 1024 samples:

```
backend preference None, render on the processor : never held any card memory at all
backend preference CUDA, render on the processor : held card memory, peak 248 MiB
render on the card                               : held card memory, peak 1714 MiB
```

**Observed.** A saved backend preference costs card memory on its own. With the preference naming CUDA, this Blender took about 248 MB of the card for the whole length of a render that ran entirely on the processor; with the preference at None it took none. That is a cost the preference exacts even in a run where it makes no difference at all to which device renders.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.2/`.

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
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
EXIT=0
```

The command returned cleanly. Without `-b` the same three names were printed and the run also exited 0 — this machine has a screen, where the Linux machine has no display server and the same command fails there before reaching the list.

**Observed.** This build offers three engines and names EEVEE `BLENDER_EEVEE`, the same name the rented Linux machine got from it.

**Hand test 15 — which settings exist on this operating system.** The same sweep as on Linux: every property of `scene.render`, `scene.cycles`, `scene.eevee`, `scene.display` and the Cycles preferences, read out of a running Blender and compared across all four researched versions on this machine. This version's dump holds **281** properties.

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

**Hand test 19 — what a Cycles Time Limit produces.** The 640 by 480, 64-sample scene rendered once with no limit and once with `time_limit` set to 0.3 seconds. Both files came out **4923801 bytes**, and the two pictures are not the same picture:

```
no limit vs a 0.3-second limit : IDENTICAL-PIXELS False MAX-ABS-DIFF 3.6866745948791504 NUM-DIFFERING-CHANNEL-VALUES 921599 of 1228800
```

**Observed.** A time limit writes a complete file of the usual size holding a picture cut short, not a blank one and not a missing one. How long each render took was **not measured**, because the machine was not idle; the Linux run carries the times.

**Hand test 31 — the texture cache.** A copy of the same scene with three 1024 by 1024 PNG images written beside it and wired into the three materials.

```
HAS-use_texture_cache True HAS-use_auto_generate_texture_cache True
CACHE-SET True True
```

What was beside the images before the cache was switched on, and after:

```
before:  tex0.png 213234   tex1.png 34672   tex2.png 213234      folder 584K
after:   blender_tx (a new folder)  tex0.png 213234  tex1.png 34672  tex2.png 213234   folder 1.7M
```

and the picture:

```
cache off vs cache on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.7571094036102295 NUM-DIFFERING-CHANNEL-VALUES 711196 of 1228800
```

**Observed.** The two settings exist in this version on macOS, a folder named `blender_tx` appeared beside the images, the folder holding them grew from 584 kB to 1.7 MB, and the picture is not the same picture. The same three things happened on the rented Linux machine. Peak memory was not measured here, because the Mac was not idle; the Linux run carries that figure.

**Three tests were not run on this machine at all, and the reason is the same for all three.** Hand test 3 (whether `-t` changes how much of the processor a Cycles render actually uses), hand test 23 (whether the thread count changes anything under EEVEE or Workbench) and hand test 24 (whether Persistent Data changes the time or the memory of an animation) have no evidence except a time or a memory figure. The Mac was shared with other topics' work throughout, so a figure taken here would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. **What they leave open on macOS is left open and is not guessed at.**

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.2.0 at `/opt/blender/5.2/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.2/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — which picture comes out when the file asks for the graphics card and the settings folder names no backend. This is hand test 7 rebuilt, because hand test 7 could not have failed.** That test searched Blender's printed output for complaints about a missing card and found none; "Blender did not complain" and "Blender did not use the card" are different things, and hand test 26 in this same document is why no search of output can tell them apart. **What changed is the evidence: the question is now settled by which picture came out.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render.

Five renders, none of them with `--factory-startup`, so the saved preferences were read every time: `blender -b <file> -o <folder>/<tag>_ -F OPEN_EXR -x 1 -f 1 --`. (a) the file saying CPU, no argument — the **processor reference**; (b) the same again, unchanged; (c) the file saying CPU with `-- --cycles-device CUDA` — the **card reference**; (d) the same again, unchanged; (e) **the test itself**, the file saying GPU Compute with the backend preference naming nothing.

The two controls first, and the size of the gap this has to judge against:

```
processor run 1 vs run 2 : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0                  DIFFERING 0 of 1228800
card run 1 vs run 2      : IDENTICAL-PIXELS False MAX-ABS-DIFF 2.86102294921875e-06    DIFFERING 467332 of 1228800
processor vs card        : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.08292007446289062     DIFFERING 600651 of 1228800
```

and then the test:

```
(e) file GPU Compute, backend preference NONE, no argument
    against the processor reference : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0              DIFFERING 0 of 1228800
    against the card reference      : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.08292007446289062  DIFFERING 600651 of 1228800
```

**Observed.** A file asking for GPU Compute, in a settings folder naming no backend, renders on the **processor** — not approximately but to the very same numbers — and misses the card by the whole device gap, which is some twenty thousand times the difference between two runs on the same device. The test could have come out the other way and did not. A second reading says the same without any reference to the pictures: what each run held on the card, this Blender alone.

```
              Cycles reported   % of a processor   card memory held by this run
processor        Time: 00:02.40      1246%          none at all
card (CUDA)      Time: 00:01.78      57%            1692 MiB
file GPU, no backend named   Time: 00:02.41   1218%   none at all
```

The wall-clock times were 0:03.13, 0:02.82 and 0:03.19; they are a busy machine's times and carry nothing on their own. **The card memory carries it: the run never touched the card.** The five renders were made twice, ten minutes apart, and both passes gave pixel-identical pictures (`IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`), so this is not one moment's accident. **Still owed on the Creator's Mac**, where the question is sharper because the graphics there work perfectly.

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
--cycles-device CUDA   exit 0, image written : matched the CARD reference, MAX-ABS-DIFF 2.86102294921875e-06; against the processor 0.08292007446289062
--cycles-device OPTIX  exit 0, image written : matched NEITHER — against the processor 0.08292007446289062, against the CUDA card 0.0625619888305664
--cycles-device HIP    exit 1, no image written
--cycles-device ONEAPI exit 1, no image written
--cycles-device METAL  exit 1, no image written
```

OptiX being a third picture is not a fault in the test: hand test 27 already found the two backends on the same card disagreeing with each other. What matters is that it is not the processor's picture. What the three refused runs printed, word for word:

```
00:00.569  reports          | ERROR Found no Cycles device of the specified type
```

and `HIP` printed one line more, before it:

```
00:00.531  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
00:00.534  reports          | ERROR Found no Cycles device of the specified type
```

Card memory, this Blender alone: `CPU`, `HIP`, `ONEAPI` and `METAL` held **none at all**; `CUDA` held 302 MiB and `OPTIX` 1712 MiB. (The card was sampled five times a second and these renders last about two seconds, so a peak can be missed — the reading that carries weight is nothing against something.)

**Observed.** Naming a backend this build accepts but has no hardware for does **not** quietly fall back to the processor: the render stops, writes no image, and says `Found no Cycles device of the specified type`. So a program that writes `HIP` or `ONEAPI` into this setting on this machine gets no error when it writes it, and finds out at render time. **Still owed on the Creator's Mac** for the device-listing half and the render half.

**Hand test 35 — the property sweep written out exactly, so its totals can be checked.** The citation check sent hand test 15 back over its numbers, not its finding: three sittings had reported three different totals and the procedure as written did not say whether each block's own `rna_type` entry is counted. The sweep was run again with that stated — `blender -b --factory-startup <file> --python <script>`, walking `bl_rna.properties` of five blocks named exactly (`scene.render`, `scene.cycles`, `scene.eevee`, `scene.display`, and the Cycles add-on's preferences), counting each block twice, once including `rna_type` and once excluding it. This version: `scene.render` 102 / 101, `scene.cycles` 111 / 110, `scene.eevee` 54 / 53, `scene.display` 10 / 9, the Cycles preferences 9 / 8 — **286 counting `rna_type`, 281 not counting it**.

**Observed.** The three totals on the record are now told apart. Counting `rna_type` gives 290, 276, 278 and 286 across the four versions, which is the citation check's figure for 4.5 exactly. Not counting it gives 285, 271, 273 and 281, which is the Mac sitting's figures exactly. The first Linux sitting's 286, 272, 274 and 282 are **neither** — one more than the not-counting figure in every version — so **they cannot be reproduced from any stated rule and should be read as decoration**. This version's earlier figure was 282. **The finding itself reproduced exactly**: the set of property names not present in all four versions came back as the same twenty-eight, on the same versions, `scene.eevee.fast_gi_thickness_far` among them — present in 4.5, 5.0 and 5.1 and absent from 5.2 — so the count and the written-out list now agree.



## 2. For each of them, what does it change about what the render produces?

**LABEL: documented**, with one group **inferred** where I say so.

The documentation splits cleanly into three groups. All quotes below are from the pages already cited in answer 1, read 2026-07-30.

### Group A — the sources describe a cost only, and say nothing about the image changing

- **Threads Mode / Threads**: "The maximum number of CPU cores to use simultaneously while rendering." No effect on output is described.
- **Persistent Data**: "Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage".
- **Tile Size**: "Render high resolution images in tiles of this size, to reduce memory usage of render passes."
- **Use Spatial Splits**: "longer BVH build times and slightly increased memory usage."
- **Use Curves BVH**: "reduce memory, at the cost of increasing curve render time."
- **Use Compact BVH**: "can reduce RAM usage but render slower."
- **BVH Time Steps**: "speed up render time at the expense of memory."
- **Texture Cache**: "Reduce memory usage and startup time… at the cost of a small rendering performance impact and increased disk space usage."
- **Lock Interface**: "in favor of giving more memory to the renderer."
- **Preferences** items (Distribute Memory Across Devices, Embree on GPU, HIP RT, MetalRT): described as memory and speed only.
- **`--qos`**, **`-t/--threads`**: described as scheduling and thread count only.

For this whole group the honest statement is: **the sources do not say the rendered image changes, and they do not say it stays the same either.** Reading "no output change" out of silence would be inference, and I am not making it.

Two of these do have a stated side effect that is not the image: the texture cache writes files — "For every image, a corresponding tx file will be generated in the `blender_tx/` folder next to the image file" (release notes, `https://developer.blender.org/docs/release_notes/5.2/cycles/`), and tiling writes to disk — "Tiles are cached to disk while rendering to save memory."

### Group B — the sources say the image changes, and how

- **Render (Max) Samples**: "A higher number results in a cleaner image at the cost of a longer render time." Fewer samples means a noisier image.
- **Time Limit**: "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet" — so the image is whatever had converged when time ran out.
- **Noise Threshold**: "lower values meaning better quality but longer render times."
- **Max Bounces**: "A value of 0 bounces results in direct lighting only." Lower values leave out light that would otherwise be there.
- **Min Light Bounces**: "Setting this higher gives less noise, but may also increase render time considerably."
- **Light Tree**: "can significantly reduce noise, at the cost of a somewhat longer render time per sample"; and it is not neutral — "Certain lighting properties are not accounted for in the light tree… This can result in an increase in noise in some scenes that make use of these features." *Quote restored in full by the citation check, 2026-08-07: the closing words "that make use of these features" had been dropped, which made the claim wider than the source.*
- **Light Threshold**: "Probabilistically terminates light samples when the light contribution is below the threshold."
- **Scrambling Distance**: "improve GPU rendering performance, at the cost of potential artifacts."
- **Denoising** (on/off, denoiser, passes, prefilter, quality): "Denoising uses a specialized algorithm to get a less noisy image without requiring more samples"; "None can blur out details, especially at lower sample counts"; Fast prefilter "produces a blurrier result"; Quality Fast "Sacrifices quality for speed".
- **Denoise Nodes Final Quality** (compositor): "Fast: Produces an output quickly at a noticeable cost of quality."
- **Volume Step Rate Render**: "Increase to reduce render time, at the cost of losing detail."
- **Volume Max Steps**: "Maximum number of steps through the volume before giving up".
- **Subdivision Dicing Rate / Max Subdivisions / Offscreen Scale**: "Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory."
- **Simplify Max Subdivision / Child Particles / Texture Resolution / Texture Size Limit**: "Show only a subset of all child hairs and particles"; "Scale down image textures"; "Automatically scales textures down so that they are no larger than the values chosen."
- **Resolution X, Y and Percentage**: the image is that many pixels — "Slider to reduce or increase the size of the rendered image relative to the Resolution values."
- **Render Region / Crop to Render Region**: "Renders just a portion of the view instead of the entire frame"; crop "Crops the rendered image to the size of the render region, instead of rendering a transparent background around it."
- **Render passes**: extra passes are extra outputs — "These passes increase memory usage and render time."
- **View Layer Samples override**: a different sample count for that layer, so a different noise level.
- **Sample Subset (Offset, Length)**: renders part of the sample sequence; "Multiple subset renders can be combined into a full one"; and "it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets – otherwise, the subsets will have incompatible noise and combining them will give a worse result."
- **High Quality Normals**: "can improve visual quality for dense meshes with high frequency textures at the cost of memory."
- **Anisotropic Filtering**: "Higher values improve the sharpness of textures viewed at a grazing angle".
- **EEVEE Shadow Rays / Steps / Resolution**: "Higher values reduces the noise caused by random shadow sampling"; "Higher step count results in softer shadows but have a higher cost"; "Resolution percentage of shadow maps."
- **EEVEE Shadow Pool**: too small a pool has a visible failure — "Increasing the size might fix the Shadow buffer full error."
- **EEVEE Volume Resolution / Steps** and **Ray-tracing Resolution**: "Lower options will be faster and use less memory but will produce blurrier results."
- **EEVEE Fast GI Rays / Steps / Precision / Distance**: "Higher values will reduce noise"; "Higher values increase the precision of the scene intersections".
- **EEVEE Motion Blur Steps**: "controls the number of steps used by the accumulation blur and thus its accuracy."
- **Workbench anti-aliasing**: chooses whether and how edges are smoothed.
- **Open Shading Language**: enables shader scripts, and "Rendering performance is typically slower".

### Group C — the device, which is its own case

**LABEL: split — documented for one option, inferred for the general claim.** *Corrected by the citation check, 2026-08-07: this used to read "LABEL: documented" for the whole claim, resting on the "may have limitations" note below. A limitation is a feature a backend does not offer; getting from that to "the picture differs" is a step the reader has to supply, and the 5.1 findings label exactly this reasoning inferred. The documented version of the claim was available and was missed — it is the Curves sentence now quoted first.* **Documented**, for one curve shape option: "This is an optimization to take advantage of hardware accelerated curve intersection on GPUs. Results will not look exactly the same on different CPU and GPU devices, so this is not suitable for multi device rendering or render farms with mixed hardware." — `https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/hair.html`, read 2026-07-30. **Inferred**, for anything wider than that: the manual does not claim the image is identical on CPU and GPU, and it notes limitations, but a limitation is not by itself a statement that the picture differs. The general remarks are:

> "Note
>
> GPU rendering may have limitations compared to CPU rendering, depending on the backend."

and names specific ones:

> "Path Guiding is not supported on any GPU."

> "Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

Sources: `.../render/cycles/render_settings.html` and `.../render/cycles/gpu_rendering.html`, read 2026-07-30. Also, the Render Time debug pass "is not supported on GPU rendering" (`.../render/layers/passes.html`).

**Not covered.** For Group A the sources do not settle whether the output is bit-for-bit identical — that is exactly the question a document cannot answer and a test can. For Group B the sources give direction (noisier, blurrier, softer) but never a quantity. Nothing here says how two settings combine.

**Outside the interface.** The stated effects are properties of the render, not of the window, so they hold for a render started from outside Blender — with the same viewport-only exception noted in answer 1 (**inferred**, same step). The sources do not say any of these produce a different image in background mode.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Operating system.** The sources state no operating-system dependence for what these settings do to the image, other than by way of which devices and backends exist on which system (quoted in answer 1). Denoiser availability is hardware- and OS-bound: "GPU accelerated denoising is available on discrete Radeon RX 6000 and newer GPUs on Linux, and discrete Radeon RX 7000 and newer GPUs on Windows" (`.../render/cycles/gpu_rendering.html`, read 2026-07-30).

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Where I looked.** Same pages as answer 1; for each setting I re-read its own paragraph looking specifically for a statement about the resulting image. Additional searches across the manual for `noise`, `blurrier`, `artifacts`, `quality`, `detail` within `render/` to catch output-side wording I might have missed.

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.2/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 16 — whether thread count, tile size, Use Tiling, the three acceleration-structure options, BVH Time Steps, Persistent Data or High Quality Normals change the picture.** Done to a scene built from a script and saved: Cycles, Device CPU, 640 by 480 at 100 per cent, 64 samples, Seed 7, Use Animated Seed off, Denoise off, Adaptive Sampling off, Max Bounces 4, stamping off, saved as OpenEXR at full float with no compression. Twelve renders of frame 1, each changing exactly one thing — nothing; `-t 1`; `-t 8`; Tile Size 64; Tile Size 8192; Use Tiling off; Use Spatial Splits on; Use Compact BVH on; Use Curves BVH off; BVH Time Steps 4; Persistent Data on; High Quality Normals on — then each compared against the base **by pixels**, by loading both images and subtracting them channel by channel. What came back, for every one of the eleven pairs:

```
IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
```

Two control renders were made to show the comparison can tell a difference at all — without them no result from it would mean anything:

```
seed 7 → 8: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.24828141927719116 NUM-DIFFERING-CHANNEL-VALUES 862887 of 1228800
64 samples → 63: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.24280476570129395 NUM-DIFFERING-CHANNEL-VALUES 863070 of 1228800
```

**Observed.** None of those eleven changes altered a single channel value, on this scene, on this machine, in this version.

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench as well as Cycles.** Done to a scene with a Subdivision Surface modifier at render level 4. Six renders: each of the three engines with Simplify off, and each with `use_simplify` on and `simplify_subdivision_render` set to 0, compared by pixels:

```
CYCLES            : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.10609644651412964 NUM-DIFFERING-CHANNEL-VALUES 28272 of 76800
BLENDER_EEVEE     : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.10595703125       NUM-DIFFERING-CHANNEL-VALUES 3556 of 76800
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

**Hand test 19 — what a Time Limit does to the picture.** Same scene rendered with Time Limit 0 (`Time: 00:01.66`) and with Time Limit 0.3 seconds (`Time: 00:00.41`). Both files 4923762 bytes; compared by pixels, `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.49512332677841187`.

**Observed.** A time limit changes the picture as well as the cost.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.2/` or `/root/render-demands/prefs-card-cuda/5.2/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 27 — whether the same file renders to the same picture on the processor and on the card.** One file, three renders, changing only the device: Device CPU with the backend preference at None; Device GPU Compute with the backend CUDA; Device GPU Compute with the backend OptiX. The file was built to leave nothing else free to move: Cycles, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, no path guiding, no Open Shading Language, four bounces, OpenEXR at full float with no compression. The three images were then compared **by pixels** — both loaded and subtracted channel by channel — and never by comparing the saved files. What came back:

```
processor vs card (CUDA)  : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.04148709774017334  DIFFERING 751867 of 1228800  MEAN-ABS-DIFF 0.00015436025569215417
processor vs card (OptiX) : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.04723680019378662  DIFFERING 753666 of 1228800  MEAN-ABS-DIFF 0.00016594311455264688
card CUDA vs card OptiX   : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.042746663093566895  DIFFERING 673953 of 1228800  MEAN-ABS-DIFF 0.00013123075768817216
```

The control, which is what makes those numbers mean anything: each of the three renders was run a second time, unchanged, and compared with its first run.

```
processor, run 1 vs run 2  : IDENTICAL-PIXELS True   MAX-ABS-DIFF 0.0
card CUDA, run 1 vs run 2  : IDENTICAL-PIXELS False  MAX-ABS-DIFF 5.960464477539062e-07  DIFFERING 614319 of 1228800
card OptiX, run 1 vs run 2 : IDENTICAL-PIXELS False  MAX-ABS-DIFF 5.960464477539062e-07  DIFFERING 611985 of 1228800
```

**Observed.** **The device changes the picture.** Which device did the work is not only a matter of what the render costs; the image that comes out is a different image. Two things say how much of a difference it is. It is small in size — the largest single channel value moves by about 0.0414 on a nought-to-one scale, and the average move is about a ten-thousandth — but it is spread over more than half of the image. And it is far above the noise of the machine: the processor repeats its own render exactly, and the card repeats its own render to within about a millionth, which is about the finest step a number of this kind can take, while the difference between processor and card is some tens of thousands of times larger than that. The two card backends do not agree with each other either. So a frame rendered on the processor and a frame rendered on the card are not interchangeable, and nor are frames from two different backends on the same card.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.2/`.

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
base vs seed8 (seed 7 -> 8):    IDENTICAL-PIXELS False MAX-ABS-DIFF 0.6312932968139648 NUM-DIFFERING-CHANNEL-VALUES 714774 of 1228800
base vs s63 (64 samples -> 63): IDENTICAL-PIXELS False MAX-ABS-DIFF 0.6271488666534424 NUM-DIFFERING-CHANNEL-VALUES 921598 of 1228800
```

**Observed.** Thread count, tile size, Use Tiling, the three acceleration-structure options, BVH Time Steps, Persistent Data and High Quality Normals all left the picture untouched, on a machine with a different processor from the one the same eleven were shown clean on. The rented Linux machine gave the same eleven identical results. **The two machines agree.**

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench.** A scene with a Subdivision Surface modifier at render level 4, rendered under each of the three engines with Simplify off, then again with Simplify on and Max Subdivision (render) at 0, each pair compared by pixels:

```
CYCLES              simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.41278624534606934 NUM-DIFFERING-CHANNEL-VALUES 37699 of 76800
BLENDER_EEVEE       simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.380859375 NUM-DIFFERING-CHANNEL-VALUES 10338 of 76800
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
processor vs card (Metal) : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.05070698261260986 NUM-DIFFERING-CHANNEL-VALUES 636611 of 1228800 MEAN 7.927500492617886e-07
```

**And the control, without which none of that means anything** — each of the two renders repeated, unchanged, and compared with its own first run:

```
processor run 1 vs run 2   : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0
card Metal run 1 vs run 2  : IDENTICAL-PIXELS False MAX-ABS-DIFF 4.291534423828125e-06 NUM-DIFFERING-CHANNEL-VALUES 396701 of 1228800
```

**Observed.** **The device changes the picture on this machine too.** More than half of the channel values differ, the largest by about three to five hundredths on a nought-to-one scale. The processor repeats its own render exactly; Apple's graphics do not, but only by about a millionth, which is some tens of thousands of times smaller than the gap between the two devices. The rented Linux machine found the same thing with an NVIDIA card through CUDA and through OptiX. **Two different makes of graphics processor, two operating systems, the same answer: choosing the device changes the image, not only what the image costs.** What it costs was not measured here; the Linux machine carries that.

**Hand test 20 — a start-up script setting Samples.** `--python-expr "import bpy; bpy.context.scene.cycles.samples=7"` against the 64-sample file:

```
SAMPLES-SET 7
the 64-sample base vs the 7-sample render : IDENTICAL-PIXELS False MAX-ABS-DIFF 2.5411667823791504 NUM-DIFFERING-CHANNEL-VALUES 921598 of 1228800
```

**Observed.** The assignment reaches the render and changes the picture. The render time was not recorded here.

**Hand test 31 — the texture cache and the picture.**

```
HAS-use_texture_cache True HAS-use_auto_generate_texture_cache True
CACHE-SET True True
```

What was beside the images before the cache was switched on, and after:

```
before:  tex0.png 213234   tex1.png 34672   tex2.png 213234      folder 584K
after:   blender_tx (a new folder)  tex0.png 213234  tex1.png 34672  tex2.png 213234   folder 1.7M
```

and the picture:

```
cache off vs cache on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.7571094036102295 NUM-DIFFERING-CHANNEL-VALUES 711196 of 1228800
```

**Observed.** The two settings exist in this version on macOS, a folder named `blender_tx` appeared beside the images, the folder holding them grew from 584 kB to 1.7 MB, and the picture is not the same picture. The same three things happened on the rented Linux machine. Peak memory was not measured here, because the Mac was not idle; the Linux run carries that figure.

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.2.0 at `/opt/blender/5.2/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.2/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — what the device does to the picture, and what a file asking for a card it has not been given produces.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render.

Two renders of it, changing only the device, plus a repeat of each: the file saying CPU with no argument, and the same file with `-- --cycles-device CUDA`. Compared on the pixels, never on the saved files:

```
processor run 1 vs run 2 : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0                  DIFFERING 0 of 1228800
card run 1 vs run 2      : IDENTICAL-PIXELS False MAX-ABS-DIFF 2.86102294921875e-06    DIFFERING 467332 of 1228800
processor vs card        : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.08292007446289062     DIFFERING 600651 of 1228800
```

**Observed.** The device changes the picture, on a second scene and in a second sitting: about half the channel values differ, the largest by some hundredths on a nought-to-one scale, where the processor repeats its own render exactly and the card repeats its own to about three millionths. This is hand test 27's result standing up on a scene built for a different question.

**And the one that answers what the file's own Device produces.** The same scene saved with Device **GPU Compute**, rendered with the backend preference naming nothing:

```
against the processor reference : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0              DIFFERING 0 of 1228800
against the card reference      : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.08292007446289062  DIFFERING 600651 of 1228800
```

**Observed.** It produces the **processor's** picture, to the very last number. So a file whose Device says GPU Compute does not, on its own, produce the graphics card's output; what it produces depends on whether a backend has been named somewhere else, and this is settled here by which picture came out rather than by what Blender printed, which names no device at all.

**One more thing about the picture, from hand test 34.** The same file rendered with `-- --cycles-device OPTIX` matched **neither** reference — against the processor 0.08292007446289062, against the CUDA card 0.0625619888305664. Three devices, three pictures. **Still owed on the Creator's Mac.**



## 3. Which of them can be settled at the moment a render is started from outside Blender, and which only by changing the file?

**LABEL: documented** for the list of arguments; **inferred** for the general Python route, and I write the step out.

### Settable directly by an argument at start-up

From `.../advanced/command_line/arguments.html`, read 2026-07-30 — these are the complete set of documented arguments that bear on demand:

| Argument | Documented wording |
|---|---|
| `-t, --threads <threads>` | "Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count." |
| `--cycles-device <device>` | "Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. Append +CPU to a GPU device to render on both CPU and GPU." |
| `-E, --engine <engine>` | "Specify the render engine. Use -E help to list available engines." |
| `--gpu-backend` | "Force to use a specific GPU backend. Valid options: vulkan, metal, opengl." |
| `--qos <level>` | "Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only)." |
| `--disable-depsgraph-on-file-load` | "Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)." |
| `-b, --background` | "Run in background (often used for UI-less rendering)." |
| `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-F`, `-x` | choose the scene, which frames, where to write and in what format — they set how much work is done, not how hard each frame is. |
| `--cycles-print-stats` | "Log statistics about render memory and time usage." |

The Cycles ones need a separator: "Cycles add-on options must be specified following a double dash", with the example `blender -b file.blend -f 20 -- --cycles-device OPTIX`.

The manual's own worked example uses several at once:

> "blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a"
> "-t 2
>
> Use only two threads."

Source: `.../advanced/command_line/render.html`, read 2026-07-30.

Order matters: "Arguments are executed in the order they are given"; and "loading the blend-file overwrites the render output that was set", so arguments that set scene state must come after the file (`.../advanced/command_line/arguments.html`).

### Not settable by any argument

**LABEL: not in the sources — a demonstrated absence.** I searched the whole of `advanced/command_line/arguments.txt` for `sample`, `tile`, `resolution`, `bounce`, `denois`, `persistent`, `simplify`. The only hit anywhere in the file was the phrase "Do not use native pixel size, for high resolution displays (MacBook Retina)" under `--no-native-pixels`, which is about the interface. There is **no** command-line argument for samples, tile size, resolution, bounces, denoising, persistent data, simplify, the BVH options, the texture cache, or any EEVEE setting. Those are file settings.

### The general route: Python at start-up

Blender will run arbitrary Python before rendering:

> "-P, --python <filepath>
>
> Run the given Python script file."

> "--python-expr <expression>
>
> Run the given expression as a Python script.
>
> The expression may be a complete multi-line script; you are limited only by the platform's maximum argument length."

(`.../advanced/command_line/arguments.html`, read 2026-07-30.)

And Python reaches the settings, because they are data-block properties:

> "This module is used for all Blender/Python access."
> "# Print all scene names in a list.
> print(bpy.data.scenes.keys())"

(`https://docs.blender.org/api/5.2/bpy.data.html`, read 2026-07-30.)

**This is inferred.** The step a reader must supply: the sources say Python scripts can be run at start-up, and separately that render settings are properties reachable through `bpy.data`; neither page says "therefore you may set the tile size from the command line". I did not find any page that says so. The manual does show the pattern in a neighbouring form — its own texture-cache tool works this way (`bpy.context.preferences.filepaths.texture_cache_directory = args.cache_dir`, observed in `Blender Sources/cycles_5.2/maketx.py` line 103) — but that is Cycles' code, not a documented promise.

One documented condition on the Python route:

> "Enable with -y or --enable-autoexec"
> "Disable with -Y or --disable-autoexec"
> "To render an animation in background mode, allowing drivers and other scripts to run:
> blender --background --enable-autoexec my_movie.blend --render-anim"

(`.../advanced/scripting/security.html`, read 2026-07-30.) Note the manual lists `-Y` as "(default)" in the arguments page.

### The third category: settings that are in neither the file nor an argument

The Preferences items in answer 1f live in a separate file — "./config/userpref.blend — User preferences." (`.../advanced/blender_directory_layout.html`, read 2026-07-30) — so they follow the machine, not the scene. For Cycles' device the sources give a start-up override anyway (`--cycles-device`); for MetalRT, HIP RT, Embree on GPU and Distribute Memory Across Devices they do not name one.

**Not covered.** The sources do not say what happens when an argument and a file setting disagree — specifically, whether `-t 2` overrides a scene saved with Threads Mode "Auto-Detect", or whether `--cycles-device` overrides the scene's Device. Both are stated as "set" or "use" without saying what they set. I could not settle this from the documentation and am not guessing. Nor do the sources say whether `-t` limits only rendering or all of Blender's threads beyond the phrase "for rendering and other operations".

**Outside the interface.** This whole answer is about that case. The one thing to add is that `-b` is described as "often used for UI-less rendering", and `-c/--command` "implies --background mode".

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Operating system.** `--qos` is "(Windows only)". `-r/--register` and friends are "(Windows & Linux only)" but are unrelated to demand. `--gpu-backend`'s valid options are named without an OS qualifier, though metal plainly presupposes Apple hardware; the sources do not spell that out. Everything else in the table is stated without OS qualification.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Where I looked.** Read `advanced/command_line/arguments.txt` in full (691 lines) and `advanced/command_line/render.txt` in full; read `advanced/command_line/extension_arguments.txt` headings and ruled it out as being about extensions, not rendering. Searched the manual for `--command` and confirmed the documented commands are `extension` and `maketx`. Searched the manual for environment variables (`BLENDER_*`, `OMP_NUM_THREADS`, `CYCLES_*`) — the only render-related one found is `CYCLES_CUDA_EXTRA_CFLAGS`, which is a compiler flag, not a demand setting. Read `advanced/scripting/security.txt` and `advanced/blender_directory_layout.txt`. Read `bpy.data.txt` and `bpy.ops.wm.txt` (the `open_mainfile` entry) in the API documentation.

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.2/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

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
    wall 0:13.49
(b) the same with the script before -t 3
    SET-BY-SCRIPT FIXED 3
    FINAL FIXED 3
    wall 0:13.33
(c) the script alone, no -t
    SET-BY-SCRIPT FIXED 5
    FINAL FIXED 5
    wall 0:08.43
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
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
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
00:00.674  cycles           | ERROR CUDA cuInit: Unknown error
DEVICE CPU
(d) -f 1 first, then read, with --cycles-device CPU      DEVICE CPU
(e) -- --cycles-device NOSUCHDEVICE
00:00.346  cycles           | ERROR NOSUCHDEVICE is not a valid Cycles device.
```

**Observed.** The stored value never moved, at either moment. The machine's card was unreachable, so what the override does to a render that can use a card is not settled here.

**Hand test 8 — whether a render started from outside reads the saved preferences.** Done in a settings folder of its own. The Cycles compute backend was set to `CUDA` and `bpy.ops.wm.save_userpref()` called, producing `config/userpref.blend` of 180297 bytes. Then read back:

```
plain background run (no --factory-startup)   READ-PREF CUDA
the same with --factory-startup               READ-PREF NONE
```

**Observed.** A background run does read the saved preferences; `--factory-startup` stops it.

**Hand test 20 — whether a start-up script can set a setting that has no argument of its own.** Done to the 640 by 480 scene, whose Samples is 64: `blender -b --factory-startup med.blend --python-expr "import bpy; bpy.context.scene.cycles.samples=7; print('SAMPLES-SET', bpy.context.scene.cycles.samples)" -o <folder>/s7_ -F OPEN_EXR -x 1 -f 1`. What came back: `SAMPLES-SET 7`, `Time: 00:00.31` against 00:01.66 for the unchanged render, and the picture differs from the 64-sample one — `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.7329639196395874`.

**Observed.** The assignment reaches the render.

**Hand test 21 — whether "Render Single Layer" is honoured from the command line.** Done to a file with two view layers, `ViewLayer` and `Second`, saved once with the setting off and once on, each rendered with `-f 1`. In both runs Blender named both view layers in its own progress lines (`Scene, Second` and `Scene, ViewLayer`).

**Observed.** The setting made no difference to a render started from the command line.

**One thing learned by getting a command wrong, recorded because it bears on every command in this answer.** An earlier run gave the arguments as `-f 1 -o <folder>/r_ -F OPEN_EXR -x 1`, with the render before the output path, and the image went to `/tmp/0001.exr` instead. `-o`, `-F` and `-x` have to come before `-f`.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.2/` or `/root/render-demands/prefs-card-cuda/5.2/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 28 — what `--cycles-device` does to a render that really can use the card.** This is the test the earlier sitting could not run at all. Six timed runs, none of them using `--factory-startup`, so the settings folder's saved preferences were read every time. Two files, both Cycles at 640 by 480 with 1024 samples: one saved with Device **CPU**, one saved with Device **GPU Compute**. Two settings folders: one with the Cycles backend preference saved as **None**, one with it saved as **CUDA**. Each run was `blender -b <file> -o <folder>/x_ -F OPEN_EXR -x 1 -f 1 -- --cycles-print-stats`, with the device argument added after `--cycles-print-stats` where one was given. Alongside the timing, this Blender alone was watched for whether it held work on the card.

```
file CPU,  preference None, no argument            : Time 00:28.80, wall 29.63 s, 1967% of a processor — never held any card memory at all
file CPU,  preference None, --cycles-device OPTIX  : Time 00:03.43, wall 4.42 s, 87% of a processor — held card memory, peak 1714 MiB
file CPU,  preference None, --cycles-device CUDA   : Time 00:03.61, wall 4.58 s, 93% of a processor — held card memory, peak 1702 MiB
file CPU,  preference CUDA, no argument            : Time 00:26.51, wall 27.56 s, 2110% of a processor — held card memory, peak 248 MiB
file GPU,  preference CUDA, no argument            : Time 00:03.91, wall 5.89 s, 99% of a processor — held card memory, peak 1702 MiB
file GPU,  preference CUDA, --cycles-device CPU    : Time 00:25.95, wall 26.92 s, 2185% of a processor — held card memory, peak 248 MiB
```

Two more runs were added, because this is the exact pair the 4.5 audit asked for — the file already asking for the card, with the preferences naming no backend at all:

```
file GPU,  preference None, no argument            : Time 00:28.39, wall 29.25 s, 2243% of a processor — never held any card memory at all
file GPU,  preference None, --cycles-device OPTIX  : Time 00:03.42, wall 4.45 s, 86% of a processor — held card memory, peak 1714 MiB
```

**Observed.** **The argument wins, in both directions.** A file that says CPU, with preferences that name no backend at all, renders on the card the moment `--cycles-device` names one — about seven times faster, the processor left almost idle, and the card holding well over a gigabyte. And a file that says GPU Compute, with CUDA saved in the preferences, is dragged back onto the processor by `--cycles-device CPU`. So neither the file's own Device setting nor the saved backend preference can stop the argument. What the two of them do decide is what happens when no argument is given: the file's Device says whether the card is used at all, and the preference says which backend. A file saying GPU Compute with the preference at None renders on the processor and never touches the card — which is the earlier sitting's result, confirmed here with a working card rather than an unreachable one — and a file saying CPU renders on the processor however the preference is set.

**A correction to hand test 26, which was run with no reachable card.** That test found that nothing Blender printed named the device that rendered. With a working card that holds for CUDA — a card render through CUDA printed no word about the card anywhere — but not quite for OptiX, whose renders name it in their ordinary progress lines while the acceleration structure is being built:

```
00:02.022  render           | Fra: 1 | Mem: 2M | Updating Scene BVH | Building OptiX acceleration structure
```

Those lines say OptiX built the structure. They are not a statement of which device rendered, and no line anywhere says that. Anything reading the device off Blender's output still cannot settle it; the timing, and what the process holds on the card, remain the evidence.

**Hand test 6, run again now the card works — whether `--cycles-device` changes the device stored in the scene.** The earlier sitting could only show this on a machine where the named device was absent, which left the obvious objection open. Repeated here against a file saved with Cycles and Device CPU, with the card present and working:

```
(a) no argument                                            : DEVICE CPU
(b) -- --cycles-device OPTIX, read before any render       : DEVICE CPU
(c) -- --cycles-device OPTIX, read after a render          : DEVICE CPU   (and the render ran on the card)
(d) -- --cycles-device CPU, read after a render            : DEVICE CPU
(e) -- --cycles-device NOSUCHDEVICE                        : 00:00.284  cycles           | ERROR NOSUCHDEVICE is not a valid Cycles device.
(f) preference CUDA, -- --cycles-device OPTIX, after render: DEVICE CPU   and the preference still reads CUDA
```

**Observed.** The stored value never moves, before a render or after one, whether or not the named device exists and whether or not it is used. The argument changes what does the work for that one run and writes nothing down — not into the scene, not into the saved preferences. An unknown device name is still refused by name. One thing changed from the earlier sitting: with no reachable card, asking for OptiX also produced `CUDA cuInit: Unknown error`; with the card working, that line is gone and the render simply runs on the card.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.2/`.

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
(b) file CPU,  preference None,  --cycles-device METAL     -> matched the METAL render (MAX-ABS-DIFF 4.291534423828125e-06)
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

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.2.0 at `/opt/blender/5.2/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.2/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — what the file's stored Device settles on its own when a render is started from outside.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render. Rendered from the command line with `blender -b <file> -o <folder>/<tag>_ -F OPEN_EXR -x 1 -f 1 --`, no `--factory-startup`, so the saved preferences were read:

```
file CPU,  backend preference NONE, no argument            : the processor's picture (the reference)
file CPU,  backend preference NONE, -- --cycles-device CUDA : the card's picture      (the reference)
file GPU,  backend preference NONE, no argument            : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 against the PROCESSOR
                                                              IDENTICAL-PIXELS False MAX-ABS-DIFF 0.08292007446289062 against the CARD
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
00:00.569  reports          | ERROR Found no Cycles device of the specified type
```

and `HIP` printed one line more, before it:

```
00:00.531  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
00:00.534  reports          | ERROR Found no Cycles device of the specified type
```

**Observed.** The argument does not quietly fall back. A backend with no hardware behind it stops the render, writes no image, and returns a non-zero exit code — which is a thing anything driving Blender from outside can actually see, unlike the device that rendered. An unknown name is a different case again and was refused by name in the earlier sitting. **Still owed on the Creator's Mac.**



## 4. Do any of them belong to a particular render engine rather than to every engine?

**LABEL: documented**, with the engine-by-engine allocation partly **observed** from Cycles' files.

Blender 5.2 ships three engines:

> "Blender provides the following render engines:
>
> EEVEE
>
> Cycles
>
> Workbench"

and "More renderers from third-party developers are available as add-ons. Each renderer has its own render settings to control render quality and performance." (`.../render/render_engines.html`, read 2026-07-30.)

**Cycles only.** Device, Samples, Time Limit, adaptive sampling, denoising settings, path guiding, light tree, light threshold, scrambling distance, max bounces, volume step rate and max steps, adaptive-subdivision dicing, tile size, the acceleration-structure options, sample subset, OSL. These are documented under the Cycles chapter, and they are stored on a Cycles-owned block — **observed** in `Blender Sources/cycles_5.2/properties.py` lines 1140–1146:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

That is Cycles attaching its own settings block to the scene when the add-on registers.

**EEVEE only.** Shadow Pool, Light Probes Volume Pool, shadow rays/steps/resolution, volumetric resolution/steps/shadows, the ray-tracing and Fast GI settings, accumulation motion-blur steps, light-probe resolutions. Documented under the EEVEE chapter; the API puts them on a separate block, `bpy.types.SceneEEVEE` (`https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html`, read 2026-07-30).

**Workbench only.** The anti-aliasing method and its sample counts (`.../render/workbench/sampling.html`).

**Shared across engines.** Threads Mode, Threads, Persistent Data, Texture Cache, resolution and percentage, render region, Simplify, compositor device and precision, denoise-node device and quality, High Quality Normals, Anisotropic Filtering, Lock Interface. These are all documented as properties of `bpy.types.RenderSettings` — "Rendering settings for a Scene data-block" — rather than of any engine (`https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html`, read 2026-07-30).

One qualification that matters, and it is **observed**, not documented: several of these shared settings are *shown* only inside a particular engine's panel. Cycles draws the Threads, Texture Cache and Persistent Data controls itself, reading them off the shared render settings:

> ```
>         col.prop(rd, "threads_mode")
>         sub = col.column(align=True)
>         sub.enabled = rd.threads_mode == 'FIXED'
>         sub.prop(rd, "threads")
> ```
> ```
>         self.layout.prop(rd, "use_texture_cache", text="")
> ```
> ```
>         col.prop(rd, "use_persistent_data", text="Persistent Data")
> ```

File: `Blender Sources/cycles_5.2/ui.py`, classes `CYCLES_RENDER_PT_performance_threads` (begins line 853), `CYCLES_RENDER_PT_performance_texture_cache` (begins line 888) and `CYCLES_RENDER_PT_performance_final_render` (begins line 974), where `rd` is `scene.render`. Read 2026-07-30. The EEVEE and Workbench Performance pages of the manual do not list a Threads panel, a Texture Cache panel or Persistent Data.

**Inferred, with the step written out:** because the property lives on the shared render settings but only Cycles draws it, the value is still present in the file when another engine is active — it simply has no control shown. What that value *does* under EEVEE or Workbench, the sources do not say. Do not read this as "threads has no effect in EEVEE"; read it as "the sources describe the threads setting only in Cycles' documentation."

Also engine-bound, one level down: "Path guiding is only available when rendering on a CPU"; "OSL is supported only when rendering with the CPU or with the OptiX GPU Compute backend"; "The Acceleration Structure panel will not be present when using an Optix compute device."

**Not covered.** I cannot say what an engine supplied as a third-party add-on offers; the manual only says each brings its own settings. I have not settled which shared settings other engines actually honour.

**Outside the interface.** The engine is chosen in the file and can be overridden at start-up with `-E, --engine`, so which set of settings applies to a headless render is decidable from outside. Panel visibility is an interface fact and does not apply.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Operating system.** Engine ownership of settings is stated without any operating-system qualification. What varies by system is which Cycles device backends exist (quoted in answer 1) and EEVEE's "Headless rendering is not supported on headless Windows systems."

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Where I looked.** `render/render_engines.txt`; the `render/cycles/`, `render/eevee/` and `render/workbench/` chapters; `bpy.types.RenderSettings.txt` and `bpy.types.SceneEEVEE.txt`; `cycles_5.2/properties.py` (registration block) and `cycles_5.2/ui.py` (all Performance panel classes, lines 831–1000). Searched the manual for `Threads` outside the Cycles chapter and found no EEVEE or Workbench threads documentation.

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.2/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 22 — whether EEVEE and Workbench render at all on a machine with no display server.** Done to the base scene: `blender -b --factory-startup scene.blend -E BLENDER_EEVEE -o <folder>/e_ -F OPEN_EXR -x 1 -f 1`, then the same with `-E BLENDER_WORKBENCH`. **Both rendered and wrote an image, exit code 0.** EEVEE took `Time: 00:28.52`; Workbench about 1.2 seconds. Workbench also printed one line before saving: `EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).`

**Observed.** This machine had neither a display server nor a usable graphics card, and both engines still produced their image.

**Hand test 23 — whether the thread count changes anything under EEVEE or Workbench.** Done to the base scene with EEVEE Render Samples 16. Twelve runs: each engine, `-t 1` and `-t 24`, three times each, timed by the operating system. Twenty-four logical processors, machine shared with other work:

```
EEVEE     -t 1  : 9.39 s wall, 326% cpu, 1736616 kb / 9.43 s, 327%, 1736964 kb / 9.56 s, 337%, 1777128 kb
EEVEE     -t 24 : 8.97 s wall, 358% cpu, 1758124 kb / 9.23 s, 332%, 1757040 kb / 8.38 s, 472%, 1800792 kb
WORKBENCH -t 1  : 1.79 s wall,  95% cpu,  520208 kb / 1.64 s,  98%,  515716 kb / 1.61 s, 106%,  515852 kb
WORKBENCH -t 24 : 1.57 s wall, 132% cpu,  538608 kb / 1.59 s, 119%,  539072 kb / 1.58 s, 113%,  536236 kb
```

**Observed.** The two sets of times overlap in both engines, and the processor percentages stay put. Under Cycles the same argument changed the time by a factor of fifteen (hand test 3). **This is "no effect seen", not "the engine ignores it"** — a render spending its time elsewhere would look the same.

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench.** Six renders of a scene with a Subdivision Surface modifier at render level 4, each engine with Simplify off and with `simplify_subdivision_render` at 0, compared by pixels:

```
CYCLES            : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.10609644651412964 NUM-DIFFERING-CHANNEL-VALUES 28272 of 76800
BLENDER_EEVEE     : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.10595703125       NUM-DIFFERING-CHANNEL-VALUES 3556 of 76800
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

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.2/`.

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
CYCLES             Performance -> Compositor, Threads, Memory, Texture Cache, Acceleration Structure, Final Render, Viewport
BLENDER_EEVEE      Performance -> High Quality Normals and Anisotropic Filtering drawn straight in it, then Memory, Viewport and Compositor
BLENDER_WORKBENCH  Performance -> High Quality Normals and Anisotropic Filtering drawn straight in it, then Compositor and nothing else
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
CYCLES              simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.41278624534606934 NUM-DIFFERING-CHANNEL-VALUES 37699 of 76800
BLENDER_EEVEE       simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.380859375 NUM-DIFFERING-CHANNEL-VALUES 10338 of 76800
BLENDER_WORKBENCH   simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.10693359375 NUM-DIFFERING-CHANNEL-VALUES 8580 of 76800
```

**Observed.** All three engines changed. Simplify's Render column belongs to no one engine. Both machines agree, and EEVEE and Workbench both render in a background run on macOS.

**Hand test 15 — which settings each version has.** The full property sweep, run on macOS in all four versions and compared, produced **exactly the same list of names** as the sweep on Linux. **Observed.** The differences between the versions' findings are differences in Blender, not in the operating system.

**Hand test 31 — the texture cache.**

```
HAS-use_texture_cache True HAS-use_auto_generate_texture_cache True
CACHE-SET True True
```

What was beside the images before the cache was switched on, and after:

```
before:  tex0.png 213234   tex1.png 34672   tex2.png 213234      folder 584K
after:   blender_tx (a new folder)  tex0.png 213234  tex1.png 34672  tex2.png 213234   folder 1.7M
```

and the picture:

```
cache off vs cache on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.7571094036102295 NUM-DIFFERING-CHANNEL-VALUES 711196 of 1228800
```

**Observed.** The two settings exist in this version on macOS, a folder named `blender_tx` appeared beside the images, the folder holding them grew from 584 kB to 1.7 MB, and the picture is not the same picture. The same three things happened on the rented Linux machine. Peak memory was not measured here, because the Mac was not idle; the Linux run carries that figure.

**Two tests were not run on this machine, and the reason is said here rather than left as a silence.** Hand test 23 (whether the thread count changes anything under EEVEE or Workbench) and hand test 24 (whether Persistent Data changes the time or the memory of an animation) rest entirely on times and memory figures. The Mac was shared with other topics' work throughout, so nothing was timed and nothing measured memory here. Both were run on the rented Linux machine and are recorded above. **What they leave open on macOS is left open.**

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.2.0 at `/opt/blender/5.2/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.2/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

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

**Hand test 35 — which of the settings named in one version's findings exist in the other three, with the sweep written out exactly.** The citation check sent hand test 15 back over its totals, which three sittings had reported three ways. The sweep was run again with the rule stated: five blocks named exactly (`scene.render`, `scene.cycles`, `scene.eevee`, `scene.display`, and the Cycles add-on's preferences), each counted twice, once including its own `rna_type` entry and once excluding it. This version: `scene.render` 102 / 101, `scene.cycles` 111 / 110, `scene.eevee` 54 / 53, `scene.display` 10 / 9, the Cycles preferences 9 / 8 — **286 counting `rna_type`, 281 not counting it**. Counting it gives 290, 276, 278 and 286 across the four versions, which is the citation check's figure; not counting it gives 285, 271, 273 and 281, which is the Mac's; and the first Linux sitting's 286, 272, 274 and 282 are neither and **cannot be reproduced**. **The finding is untouched by that**: the set of names not present in all four versions came back as the same twenty-eight, on the same versions, including `scene.eevee.fast_gi_thickness_far` — in 4.5, 5.0 and 5.1, absent from 5.2. **Still owed on the Creator's Mac**, as a repeat under the stated rule.



## 5. Can their values be read out of a file?

**LABEL: inferred** for the working method; **documented** for each piece it is built from. I write the step out.

Every piece is documented:

- The settings are properties of data-blocks, and the API documentation lists each one with its type, range and default — for example "resolution_x — Number of horizontal pixels in the rendered image (in [4, 65536], default 1920)" and "threads — Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) (in [1, 1024], default 1)" (`https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html`, read 2026-07-30).
- Those data-blocks are reachable: "This module is used for all Blender/Python access", with the example "print(bpy.data.scenes.keys())" (`https://docs.blender.org/api/5.2/bpy.data.html`, read 2026-07-30).
- A file can be opened from a script: "bpy.ops.wm.open_mainfile(…) — Open a Blender file" (`https://docs.blender.org/api/5.2/bpy.ops.wm.html`, read 2026-07-30).
- A script can be run without anyone at the keyboard: `-b`, `-P` and `--python-expr` (`.../advanced/command_line/arguments.html`, read 2026-07-30).

**The step:** the sources never say "you can read a blend-file's render settings out of it". They say the settings are properties, that properties are reachable through `bpy.data`, that a file can be opened by an operator, and that scripts run in background mode. Putting those four together is my inference. It is a short step, but it is a step.

Two documented refinements worth having:

**Reading without disturbing the open file.** The API documents a context manager for exactly this shape of work:

> "static temp_data(*, filepath=None)
>
> A context manager that temporarily creates blender file data.
>
> Parameters:
>
> filepath (str | bytes | None) – The file path for the newly temporary data. When None, the path of the currently open file is used.
>
> Returns:
>
> Blend file data which is freed once the context exits."

(`https://docs.blender.org/api/5.2/bpy.types.BlendData.html`, read 2026-07-30.)

**What linking gives you, and what it does not.** The other documented way into a file's contents is the library loader, and it does *not* give property values:

> "The input contains the data-blocks available in the loaded blend-file library and should be treated as read-only."
> "Attributes matching bpy.data, which are lists of strings representing linkable data-blocks."
> "A version attribute, representing the version of the loaded library blend-file (for the input) or the version of the current Blender (for the output)."

(`https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html`, read 2026-07-30.) So `libraries.load` yields **names** of data-blocks and the file's version, not the value of `tile_size`. Reading a value means loading the data, not just listing it.

Blender also records the file's own format version, readable once loaded: "version — File format version the .blend file was saved with (array of 3 items, in [0, inf], default (0, 0, 0), readonly)" (`bpy.types.BlendData`, same address and date).

**The exception that matters.** The settings in answer 1f are not in the scene file at all. "./config/userpref.blend — User preferences." (`.../advanced/blender_directory_layout.html`, read 2026-07-30). Which GPU Cycles will actually use, MetalRT, HIP RT and Distribute Memory Across Devices come from the machine's preferences, so no amount of reading the scene file will tell you them. Cycles' own code confirms the shape — the device list is queried from the running Blender's preferences, **observed** in `Blender Sources/cycles_5.2/properties.py`, where `class CyclesPreferences(bpy.types.AddonPreferences)` (begins line 1625) holds `compute_device_type` (line 1655) and a collection of device entries whose members are declared as `id`, `name` and `use` (lines 1619–1621).

**Not covered.** The sources describe no way to read a blend-file's settings **without Blender** — no documented file-format reader, no "print the settings" command. I searched for one and found none (see below). Nor do the sources say what happens if the file was written by a different Blender version, beyond the `version` attributes above. And they do not say whether opening a file to read it is cheap; `--disable-depsgraph-on-file-load` exists precisely because loading normally does more work, but the sources do not quantify it.

**Outside the interface.** This works better outside than inside: `-b` with `-P` or `--python-expr` is the documented way to run such a script with no window, and `--python-exit-code` gives it a way to report failure ("Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line)"). Note the manual's stated default: automatic Python execution is off — "-Y, --disable-autoexec … (default)" — though that concerns scripts *inside* the file rather than the one you pass with `-P`. The sources do not state that distinction outright; treating them as separate is **inferred**.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Operating system.** Nothing in the Python API or the command-line documentation for any of this is qualified by operating system. The only OS-specific note nearby is that temporary files go to `TEMP` on Windows and `TMPDIR` on Unix systems (`.../advanced/command_line/arguments.html`).

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Where I looked.** Searched the whole 5.2 API documentation for `texture_cache` to locate which block owns it; read `bpy.data.txt`, `bpy.types.BlendData.txt`, `bpy.types.BlendDataLibraries.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.SceneEEVEE.txt`, and the `open_mainfile` entry in `bpy.ops.wm.txt`. Searched the manual for `userpref.blend` and read `advanced/blender_directory_layout.txt` and `editors/preferences/introduction.txt`. Searched the manual's `advanced/` chapter for `--command` to check whether Blender ships a command that prints file contents — the documented commands are `extension` (extension management) and `maketx` (texture generation); neither reports settings. Searched Cycles' files for `argv`, `maketx` and preference access. I found no documented external reader for a blend file and no documented "dump settings" facility.

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.2/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

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

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.2/` or `/root/render-demands/prefs-card-cuda/5.2/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

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

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.2/`.

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


## Things a person with Blender open could settle in a few minutes

This list has been pruned three times. Everything it held that a rented Linux machine could settle was run on 2026-08-06 and is written into the answers above, under the question each one answers. Items 2 and 4, which were left waiting for a working graphics card, were run on 2026-08-07, once that machine's card had been repaired. The one list covering every version, with the full procedure for each test and the results, is `Audits/Blender Research/What a render demands/Hand tests.md`. **A third sitting, on the Creator's own Mac on 2026-08-07, ran every item that was still owed there, including the two that need a window — which no machine on this project but that one has.** Only genuinely open items are left here.

**Still open.**

6. **Where does the texture cache write, and how much, on a scene large enough to press on memory?** The small-scene half has been run and is under question 1 above. What is left is the case where the textures far exceed the machine's memory.
   *Negative result:* no folder appears, or peak memory is unchanged or higher.

**Run and answered above:** 1 (`-t`), 3 (tile size and the picture), 5 (Threads Mode under EEVEE and Workbench), 7 (Persistent Data across a sequence), 8 (`blender -E help`), **2 (`--cycles-device` against the scene's Device)** and **4 (the same file on the processor and on the card)**. Both halves of 2 were run — the file set to CPU with the argument naming a card backend, and the file set to GPU Compute with `--cycles-device CPU` — and the argument won each time. On 4, the two pictures are not the same. The card backends on the Linux machine are CUDA and OptiX rather than Metal.

**All of that has now been run, on the Creator's own Mac on 2026-08-07, and is recorded above under the question each one answers.** Apple's own graphics render a different picture from the processor, exactly as the NVIDIA card does; `--cycles-device METAL` beats both the file and the saved preferences, in both directions; the three engines really do show different Performance panels; and the Acceleration Structure controls really do change with the device. **Nothing on this subject is still owed on either machine.**

Three tests were deliberately **not** repeated on the Mac, and the reason is written here rather than left as a silence: the ones whose only possible evidence is a time or a memory figure — whether `-t` changes how much of the processor a Cycles render uses, whether the thread count changes anything under EEVEE or Workbench, and whether Persistent Data changes the time or the memory of an animation. The Mac was shared with other topics' work throughout, so a figure taken there would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. What they leave open on macOS is left open and is not guessed at.

---

2026-08-06 — Opus 5 (Claude Code) — What a render demands, Blender 5.2

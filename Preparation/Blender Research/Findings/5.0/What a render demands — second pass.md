# What a render demands — Blender 5.0 — second pass

**Sources consulted, and when.** Blender's manual for 5.0, Blender's Python API documentation for 5.0, the Blender 5.0 release notes, and Cycles' own files as shipped with 5.0 — all four read from the snapshot taken on **2026-07-30**. Blender's requirements page, read from the snapshot taken on **2026-08-01**.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

A note on how I use the two words that matter most below. **Documented** means the source says it outright. **Observed** means I read it in a file Cycles ships rather than in a written page — what the file *says* about a setting (its name, its kind, its default, its range, the words the interface shows) is solid; what a render actually *does* when that setting is used is a further step, and where I take that step I say **inferred** and write the step out.

---

## 1. What settings, if any, change how much a render demands of the computer it runs on?

**Label: documented, with the Cycles list also observed in Cycles' own files.**

Blender gathers most of these under a panel it calls Performance, and describes that panel in exactly the terms the question asks about.

> "Properties that affect render speeds or memory consumption.
> There are several presets available to help choose between different trade offs:
> Default: Balances memory saving and faster rendering settings.
> Faster Render: Uses settings to render faster at the cost of higher memory consumption.
> Lower Memory: Uses settings to decrease memory considered at the cost of slower renders."

Blender manual, Cycles Performance page — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html — read 2026-07-30.

What follows is the list. I group it by where the setting lives, because that turns out to matter for questions 3, 4 and 5.

### A. On the scene's render settings (the same place whatever engine is chosen)

**Threads Mode, and Threads.**

> "Threads Mode
> Method to determine the maximum number of CPU cores to use while rendering.
> Auto-Detect: Automatically chooses the amount of threads to match the number of logical processors on your computer.
> Fixed: Manually choose the maximum number threads to use for rendering. This can be useful for example, if you want to use your computer while rendering, you can set the property to a thread count lower the amount of logical processors on your computer.
> Threads
> The maximum number of CPU cores to use simultaneously while rendering."

Same page and date as above.

The API documentation gives the machine-readable form and the range:

> "threads
> Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems)
> Type: int in [1, 1024], default 1"

> "threads_mode
> Determine the amount of render threads used
> AUTO Auto-Detect – Automatically determine the number of threads, based on CPUs.
> FIXED Fixed – Manually determine the number of threads.
> Type: enum in ['AUTO', 'FIXED'], default 'AUTO'"

Blender Python API — https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

**Persistent Data.**

> "Persistent Data
> Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage while performing other tasks in Blender.
> When using multiple View Layers, only data from a single view layer is preserved to keep memory usage within bounds; however, objects shared between view layers are preserved."

Cycles Performance page, address and date as above. The API gives it as `use_persistent_data`, "boolean, default False", described as "Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage" (RenderSettings page, same date).

**Simplify — the Render column.** Simplify has separate viewport and render values; only the render ones bear on a render.

> "Max Subdivision
> Maximum number of subdivision by the Subdivision Surface modifiers.
> Child Particles
> Show only a subset of all child hairs and particles.
> Texture Limit
> Automatically scales textures down so that they are no larger than the values chosen. This can help reduce computer memory resources when rendering large scenes with huge textures."

Blender manual, Simplify page — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/simplify.html — read 2026-07-30.

The render-time properties are named separately from the viewport ones: `simplify_subdivision_render` ("Global maximum subdivision level during rendering", "int in [0, 32767], default 0") and `simplify_child_particles_render` ("Global child particles percentage during rendering", "float in [0, 1], default 0.0"), both switched on by `use_simplify` ("Enable simplification of scene for quicker preview renders", "boolean, default False"). RenderSettings page, same date. Note the two render defaults are 0 — Simplify is off by default, so those zeros do nothing until it is switched on.

Simplify also has Culling:

> "Camera Cull
> Automatically culls objects based on the camera frustum defined by the Margin.
> Distance Cull
> Automatically culls objects based on their distance from the active camera. This is set via the Distance property."

Simplify page, same date.

**High Quality Normals.** The manual documents this under both EEVEE's and Workbench's Performance panel:

> "High Quality Normals
> Uses higher precision normals and tangents which can improve visual quality for dense meshes with high frequency textures at the cost of memory."

Blender manual, EEVEE Performance page — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/performance.html — and Workbench Performance page — https://docs.blender.org/manual/en/5.0/render/workbench/performance.html — both read 2026-07-30. The property itself sits on the shared render settings: "use_high_quality_normals — Use high quality tangent space at the cost of lower performance — boolean, default False" (RenderSettings page, same date).

**Compositor Device, Precision, and the Denoise-node settings.** These are documented word-for-word identically on both the Cycles and the EEVEE Performance pages.

> "Device
> The device used for compositing.
> CPU: Use the CPU for compositing.
> GPU: Use the GPU for compositing."

> "Precision GPU
> The precision of compositor intermediate result.
> Auto: Use full precision for final renders, half precision otherwise.
> Full: Use full precision for final renders and viewport."

> "Denoising Device
> The device to use to process Denoise nodes in the compositor.
> Auto: Use the same device used by the compositor to process the denoise node.
> CPU: Use the CPU to process the denoise node.
> GPU: Use the GPU to process the denoise node if available, otherwise fallback to CPU."

> "Final Quality
> The quality used by Denoise nodes during the final render if their quality is set to Follow Scene.
> High: Produces the highest quality output at the cost of long processing times.
> Balanced: Balanced between performance and quality, typically processing in half the time as High, while retaining most of the quality.
> Fast: Produces an output quickly at a noticeable cost of quality."

Cycles Performance page and EEVEE Performance page, addresses and date as above. In the API these are `compositor_device` ("enum in ['CPU', 'GPU'], default 'CPU'") and `compositor_precision` ("enum in ['AUTO', 'FULL'], default 'AUTO'") on the RenderSettings page, same date.

**Image size.** Resolution X and Y and the Percentage slider set how many pixels a render has to produce.

> "Resolution X, Y
> The number of pixels horizontally and vertically in the image.
> Percentage
> Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image."

Blender manual, Format page — https://docs.blender.org/manual/en/5.0/render/output/properties/format.html — read 2026-07-30. The API gives `resolution_x` and `resolution_y` as "int in [4, 65536]", defaults 1920 and 1080, and `resolution_percentage` as "int in [1, 32767], default 100" (RenderSettings page, same date).

This page states no resource cost, so I am not claiming one on its authority. The link to demand is **inferred**, and here is the step: the Cycles Performance page says "Render high resolution images in tiles of this size, to reduce memory usage", which only makes sense if a higher resolution costs more memory. That inference gives the direction, not any amount.

**Render Region.** Renders fewer pixels than the full frame.

> "Render Region
> Renders just a portion of the view instead of the entire frame."

Format page, same date. In the API, `use_border` — "Render a user-defined render region, within the frame size" — with `border_min_x/y` and `border_max_x/y` as "float in [0, 1]" (RenderSettings page, same date). Again the resource claim is **inferred** from "just a portion … instead of the entire frame", not stated.

**Preview Pixel Size** appears on both engines' Performance panels but is explicitly about the viewport, so it is not a demand of a render as such:

> "Pixel Size
> Option to control the resolution for viewport rendering. Allows you to speed up viewport rendering, which is especially useful for displays with high DPI."

Cycles and EEVEE Performance pages, same date.

### B. Cycles' own settings

Cycles ships as an add-on, so its settings exist as files I can read directly. What Cycles itself counts as its Performance settings is written down exactly — this is the list its Performance preset saves:

```
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

**Observed** — Cycles' own files, `cycles_5.0/presets.py`, class `AddPresetPerformance`, lines 102–110.

**Device.**

> "Device
> Specifies the compute device used for rendering in Cycles. The choice of device can significantly affect render performance, GPU Compute is often faster .
> CPU: Render using the system's central processing unit. This mode is compatible with all Cycles features. The number of CPU threads used can be changed in the Performance Properties.
> GPU Compute: Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices. Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."

Blender manual, Cycles Render Settings page — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html — read 2026-07-30.

In Cycles' files the same setting reads, **observed**, `cycles_5.0/properties.py`, lines 30–35 and 381–387:

```
enum_devices = (
    ('CPU', "CPU", "Use CPU for rendering"),
    ('GPU', "GPU Compute",
        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
)
```

```
    device: EnumProperty(
        name="Device",
        description="Device to use for rendering",
        items=enum_devices,
        default='CPU',
```

Note what this says and does not say: the file holds only the choice between CPU and "GPU Compute". *Which* GPU, and by which backend, comes from the user preferences.

**Tile Size.**

> "Tile Size
> Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory."
> "Note
> In some cases changing the Tile Size can result in increased performance. For example when a small object renders slowly compared to other objects, using a small Tiles Size can lead to an increase in performance."

Cycles Performance page, same date. **Observed** in `cycles_5.0/properties.py`, lines 1056–1062:

```
    tile_size: IntProperty(
        name="Tile Size",
        default=2048,
        description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
        min=8,
        max=8192,
    )
```

The release notes record that tiling stopped being optional in this version:

> "Remove Use Tiling option, it is now always assumed to be on. The tile size can be increased to disabled tiling."

Blender 5.0 release notes, Cycles — https://developer.blender.org/docs/release_notes/5.0/cycles/ — read 2026-07-30. Consistent with that, `use_auto_tile` survives in the file only as a stub: "Deprecated, tiling is always enabled" (**observed**, `cycles_5.0/properties.py`, lines 1050–1055).

**Acceleration Structure — four settings.**

> "Use Spatial Splits
> Spatial splits improve the rendering performance in scenes with a mix of large and small polygons. The downsides are longer BVH build times and slightly increased memory usage.
> Use Curves BVH
> Use a special type of BVH for rendering curves. The bounding boxes are not axis aligned allowing a spatially closer fit to the curve geometry. Disabling this option will reduce memory, at the cost of increasing curve render time.
> BVH Time Steps
> Split BVH primitives by this number of time steps to speed up render time at the expense of memory.
> Use Compact BVH
> Use a more compact BVH structure, which can reduce RAM usage but render slower."

Cycles Performance page, same date. **Observed** in `cycles_5.0/properties.py`, lines 920–940, with defaults and range: `debug_use_spatial_splits` default False; `debug_use_hair_bvh` default True; `debug_use_compact_bvh` default False; `debug_bvh_time_steps` default 0, "min=0, max=16".

**Render Texture Limit.** Separate from the Simplify texture limit, and Cycles' own. **Observed**, `cycles_5.0/properties.py`, lines 1016–1021 and 181–191:

```
    texture_limit_render: EnumProperty(
        name="Render Texture Limit",
        default='OFF',
        description="Limit texture size used by final rendering",
        items=enum_texture_limit
    )
```

with the choices "No Limit", 128, 256, 512, 1024, 2048, 4096, 8192.

**Samples and Time Limit.**

> "Render (Max) Samples
> The number of paths to trace per pixel in the final render. A higher number results in a cleaner image at the cost of a longer render time.
> Time Limit
> Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit."
> "Note
> The time limit does not include pre-render processing time, only render time."

Blender manual, Cycles Sampling page — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30. **Observed**: `samples` is "min=1, max=(1 << 24), default=4096" and `time_limit` is a float with "min=0.0, default=0.0" and "unit='TIME_ABSOLUTE'" (`cycles_5.0/properties.py`, lines 479–484 and 514–522).

Time Limit is the only setting I found in any of these sources that puts a ceiling on a render's *duration*.

**Adaptive Sampling.**

> "Noise Threshold
> The threshold to decide whether to continue sampling a pixel. Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times. Setting this to 0 makes Cycles guess an automatic value based on the total sample count.
> Min Samples
> The minimum number of samples a pixel receives before adaptive sampling is applied."

Cycles Sampling page, same date.

**Denoising, and where denoising runs.**

> "Use GPU
> Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled."

Cycles Sampling page, same date. Also **observed** as `denoising_use_gpu`, "default=False", `cycles_5.0/properties.py`, lines 432–436.

**Path Guiding.**

> "Note
> Path guiding is only available when rendering on a CPU."
> "Training Samples
> The maximum number of samples to use for training. A value of 0 will keep training until the end of the render."

Cycles Sampling page, same date. And under GPU rendering: "Path Guiding is not supported on any GPU." (Blender manual, GPU Rendering page — https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.)

**Light Tree.**

> "Light Tree
> Use a light tree to more effectively sample lights in the scene, taking into account distance and estimated intensity. This can significantly reduce noise, at the cost of a somewhat longer render time per sample."

Cycles Sampling page, same date.

**Bounces, Fast GI, volume detail, subdivision, motion blur.** These are quality controls whose descriptions state a cost. **Observed** in `cycles_5.0/properties.py`:

- `max_bounces`, "Total maximum number of bounces", "min=0, max=1024, default=12" (lines 744–749).
- `use_fast_gi`, "Approximate diffuse indirect light with background tinted ambient occlusion. This provides fast alternative to full global illumination, for interactive viewport rendering or final renders with reduced quality", default False (lines 1023–1028); with `ao_bounces_render`, "After this number of light bounces, use approximate global illumination. 0 disables this feature", "min=0, max=1024", default 1 (lines 1044–1049).
- `volume_step_rate`, "Globally adjust detail for volume rendering, on top of automatically estimated step size. Higher values reduce render time, lower values render with more detail", "default=1.0, min=0.01, max=100.0" (lines 783–789).
- `dicing_rate`, "Multiplier for per object adaptive subdivision size", "min=0.1, max=1000.0", default 1.0 (lines 812–817).
- motion blur steps, "Control accuracy of motion blur, more steps gives more memory usage (actual number of steps is 2^(steps - 1))" (line 1381).

### C. EEVEE's own settings

**Memory pools.**

> "Shadow Pool
> A bigger pool size allows for more shadows in the scene but might not fit into GPU memory and decreases performance. Increasing the size might fix the Shadow buffer full error.
> Light Probes Volume Pool
> A bigger pool size allows for more irradiance grids in the scene but might not fit into GPU memory and decreases performance."

EEVEE Performance page, address and date as above. The API gives both as enums of "['16', '32', '64', '128', '256', '512', '1024']", `shadow_pool_size` default '512' and `gi_irradiance_pool_size` default '16' — https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html — read 2026-07-30.

**Samples.**

> "TAA is sample based so the more samples the more aliasing is reduced at the cost of performance."
> "Render
> Samples
> The number of samples to use in the final render."

Blender manual, EEVEE Sampling page — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html — read 2026-07-30. The API property is `taa_render_samples`, "Number of samples per pixel for rendering", "int in [1, inf], default 64" (SceneEEVEE page, same date).

**Shadows and lights.**

> "Rays
> Number of rays to trace for each light. Higher values reduces the noise caused by random shadow sampling.
> Steps
> Number of shadow map sample per shadow ray. Higher step count results in softer shadows but have a higher cost.
> Volumetric Shadows
> Approximate light absorption of the surrounding volume objects. This makes the volumes more opaque to light. This is a very computationally expensive option and has limitations.
> Resolution
> Resolution percentage of shadow maps."
> "Light Threshold
> Minimum light intensity for a light to contribute to the lighting. Used to compute the distance at which to cut-off lights influence. Lower values improve performance."

EEVEE Sampling page, same date.

**Volumes.**

> "To achieve this, EEVEE uses several 3D textures which have a high video memory usage. The texture dimensions can be tweaked using the Resolution and Steps parameters.
> Resolution
> Controls the quality of the volumetric effects. Lower resolution increases video memory usage and quality.
> Steps
> Number of steps to compute volumetric effects. Higher count increases video memory usage and quality."

Blender manual, EEVEE Volumes page — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/volumes.html — read 2026-07-30.

(The Resolution sentence as printed reads oddly — "Lower resolution increases video memory usage and quality" — and the API says the opposite way round for the same property: `volumetric_tile_size`, "Control the quality of the volumetric effects. Higher resolution uses more memory." I quote both and take neither as settling the direction.)

**Ray tracing.**

> "Method
> Determine the tracing method used to find scene-ray intersections and indirect lighting.
> Light Probe: Use light-probe spheres and planes to find scene intersection. This option has the lowest tracing cost but relies on manually placed light-probes.
> Screen-Trace: Trace ray against the screen depth buffer. Fallback to light-probes if ray exits the view.
> Resolution
> Resolution at which the ray-tracing is performed. Lower options will be faster and use less memory but will produce blurrier results."
> "Precision
> Higher values increase precision of the screen space ray-tracing but lower the maximum trace distance. Increased precision also increases performance cost."

Blender manual, EEVEE Raytracing page — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/raytracing.html — read 2026-07-30. The same page gives Fast GI Approximation its own Resolution, Rays, Steps and Precision, each with a stated cost, and describes disabling ray tracing entirely: "When disabled, it is replaced by a faster pipeline that uses pre-filtered light-probes."

### D. Settings that are not in the file at all — the user preferences

These change what a render demands but are stored with the user's preferences rather than with the scene.

> "Cycles Render Devices
> Changes the Computing Device the Cycles render engine uses to render images."
> "Distribute Memory Across Devices
> Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes."
> "Embree on GPU
> Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance."
> "HIP RT
> Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above."
> "MetalRT
> MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases."

Blender manual, Preferences System page — https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30.

I also opened the picture on that page, `_images/editors_preferences_section_system.webp`. **Documented** from the picture: the Cycles Render Devices panel is a row of backend buttons — on the machine pictured, "None", "CUDA" and "HIP" — sitting in Preferences, not in any scene panel. That is what is visible; which backends appear on any other machine is not something the picture can show.

One more preference is in Cycles' files but has no manual page I could find. **Observed**, `cycles_5.0/properties.py`, lines 1638–1647:

```
    kernel_optimization_level: EnumProperty(
        name="Kernel Optimization",
        description="Kernels can be optimized based on scene content. Optimized kernels are requested at the start of a render. "
                    "If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set "
                    "is available in the cache. This can result in additional CPU usage for a brief time (tens of seconds)",
        default='FULL',
```

with the choices "Off — Disable kernel optimization. Slowest rendering, no extra background CPU usage", "Intersection only — Optimize only intersection kernels. Faster rendering, negligible extra background CPU usage", and "Full — Optimize all kernels. Fastest rendering, may result in extra background CPU usage".

Also in the preferences, and relevant to how much of a machine Blender takes:

> "Shader Compilation Method
> Defines the method used for compiling GPU shaders in parallel.
> Thread: Uses multiple threads within a single process to compile shaders concurrently. This method is more memory-efficient but may be slower on some systems.
> Subprocess: Uses multiple separate subprocesses to compile shaders in parallel. This can be faster, especially on systems with high core counts, but consumes significantly more RAM.
> Threads / Subprocesses
> The number of shader compilation threads or subprocesses to use. The maximum value is limited by the number of logical CPU cores on the system. Increasing the number can reduce shader compilation time at the cost of higher memory usage. A value of 0 lets Blender automatically choose a suitable number based on the system configuration."

Preferences System page, same date. That page also notes it "is not available on macOS and requires using OpenGL backend."

### E. Things settable only when Blender is started

Two command-line options change demand and correspond to no scene setting at all:

> "--qos <level>
> Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).
> default: Uses the default behavior of the OS.
> high: Always makes use of performance cores.
> eco: Schedules Blender threads exclusively to efficiency cores."

Blender manual, Command Line Arguments page — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

> "-b, --background
> Run in background (often used for UI-less rendering)."

Same page and date.

**Not covered.** I have listed the settings whose own descriptions tie them to speed, memory or which processor does the work. I have not established, for any of them, *how much* difference they make — no source I was given gives a number, a benchmark, or a rule of thumb. Nor have I covered scene content, which plainly drives demand but is not a setting. I have not gone through every material, modifier, particle or physics option that might cost time; I stopped at the render settings, the Simplify block and the preferences, because that is where the sources put the resource discussion. Nothing here settles what happens when two of these settings are combined.

**Outside the interface.** Every setting above except Preview Pixel Size and the viewport halves of Simplify and Sampling applies to a final render, and the manual's command-line page describes rendering with "-a — Render the whole animation using all the settings saved in the blend-file" (Blender manual, Rendering From The Command Line page — https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30). So the file's settings are what a driven render uses. The sources say nothing about any of these settings behaving differently when Blender is driven from outside, with one documented exception I found, which is not a Performance setting: `use_single_layer` — "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line." (RenderSettings page, read 2026-07-30). The preferences-side settings (section D) still apply when driven from outside only if Blender reads the same preferences file; the sources do not say whether it does, and I return to this at question 3.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

**Operating system.** Several items are stated to be operating-system-specific. `--qos` is "(Windows only)". Shader Compilation Method "is not available on macOS and requires using OpenGL backend." The GPU backends divide by system: "CUDA is supported on Windows and Linux", "OptiX is supported on Windows and Linux", "HIP is supported on Windows and Linux", "oneAPI … is supported on Windows and Linux", "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." (GPU Rendering page, read 2026-07-30). And the requirements page names this version: "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." (https://www.blender.org/download/requirements/, read 2026-08-01). Threads, Persistent Data, Tile Size, Simplify and the BVH settings carry no system restriction in any source I read.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.0/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 3 — whether `-t` changes how much of the processor a Cycles render actually uses.** Done to a file built from a script and saved: Cycles, Device CPU, 320 by 240, 256 samples, seed 7, denoising off, adaptive sampling off, Threads Mode Auto-Detect; and to a copy of it with Threads Mode Fixed and Threads 1. Six runs of `/usr/bin/time -v blender -b --factory-startup <file> -f 1 -o <folder>/x_ -F OPEN_EXR -x 1`, varying only `-t`. What came back:

```
auto-detect, no -t : Percent of CPU 1710%, wall 0:02.26, peak 443180 kb
-t 1               : Percent of CPU 100%,  wall 0:37.66, peak 307392 kb
-t 2               : Percent of CPU 196%,  wall 0:18.78, peak 308372 kb
-t 0               : Percent of CPU 1694%, wall 0:02.28, peak 439372 kb
Fixed 1 in the file: Percent of CPU 1675%, wall 0:51.34, peak 422720 kb
Fixed 1 with -t 8  : Percent of CPU 726%,  wall 0:05.13, peak 341332 kb
```

**Observed.** The argument reaches the renderer: one thread takes about fifteen times as long as all of them, two threads about half of that, and `-t 0` matches the unrestricted run. The file's own Fixed 1 also reaches it. Note what the processor percentage does and does not show here: with the file asking for one thread, this version reported 1675% while still taking the single-thread wall-clock time.

**Hand test 15 — which of the settings named in one version's findings exist in the others.** Done by walking `bl_rna.properties` of `scene.render`, `scene.cycles`, `scene.eevee`, `scene.display` and the Cycles preferences in all four researched versions and comparing the five lists. This version's dump holds 272 properties. Every name that is not in all four:

```
Present in 4.5 and gone by 5.0:
  scene.cycles.feature_set; scene.eevee.gtao_distance, gtao_quality, use_gtao;
  scene.render.bake_bias, bake_margin, bake_margin_type, bake_samples, bake_type, bake_user_scale,
    use_bake_clear, use_bake_lores_mesh, use_bake_multires, use_bake_selected_to_active, use_bake_user_scale
New in 5.0 (absent from 4.5), still present in 5.1 and 5.2:
  scene.cycles.volume_biased
Absent here and present in 5.2 only:
  scene.render.use_texture_cache, scene.render.use_auto_generate_texture_cache,
  scene.cycles.texture_resolution, scene.cycles.texture_resolution_render,
  scene.cycles.debug_use_texture_cache_eviction, scene.cycles.debug_texture_cache_preserve_unused,
  scene.render.anisotropic_filter, scene.render.save_output, scene.cycles.use_pixel_jitter
Absent here and present in 5.1 and 5.2:
  scene.eevee.direct_light_intensity, scene.eevee.indirect_light_intensity
Values that differ: scene.render.compositor_device reads 'CPU' here and 'GPU' in 5.2;
  the Cycles preference use_hiprt reads False here and True in 5.1 and 5.2.
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
00:00.367  cycles           | ERROR CUDA cuInit: Unknown error
```

**Observed.** A render started from outside, asking for the graphics card on a machine that has none it can use, produces the image and — unless the preference names a backend — says nothing at all.

**Hand test 19 — whether a Cycles Time Limit produces a usable image.** Done to a 640 by 480 Cycles scene at 64 samples. Rendered once with Time Limit 0, then again with `--python-expr "import bpy; bpy.context.scene.cycles.time_limit=0.3"`. What came back: no limit `Time: 00:01.63`; limit `Time: 00:00.41`. Both files 4923758 bytes. Compared by pixels, the two are not the same picture: `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.39824867248535156`.

**Observed.** A time limit writes a complete file of the usual size, holding a picture cut short rather than a blank or a missing one.

**Hand test 24 — whether Persistent Data changes the time or the memory of an animation.** Done by rendering the whole ten frames with `-a`, once with `use_persistent_data` off and once on, under EEVEE and under Cycles, timed by the operating system. One run of each, on a shared machine:

```
EEVEE, ten frames, Persistent Data off : 71.36 s wall, 406% cpu, 2228096 kb peak
EEVEE, ten frames, Persistent Data on  : 59.66 s wall, 699% cpu, 2246692 kb peak
Cycles, ten frames, Persistent Data off: 19.41 s wall, 2149% cpu, 493592 kb peak
Cycles, ten frames, Persistent Data on : 19.03 s wall, 1987% cpu, 466576 kb peak
```

**Observed.** On a scene this small and a machine this busy, one run of each cannot separate the time differences from noise. What can be said is that peak memory did not go up when Persistent Data was switched on; under Cycles it came out lower.

**Hand test 16, cost side — what tile size and the acceleration-structure options cost.** Same 640 by 480 scene, one run each, shared machine:

```
base      2.93 s wall, 1274% cpu, 457100 kb peak
tile 64   3.75 s wall, 1131% cpu, 516060 kb peak
tile 8192 2.66 s wall, 1402% cpu, 456728 kb peak
tiling off 2.41 s wall, 1542% cpu, 454800 kb peak
compact BVH 2.35 s wall, 1632% cpu, 458016 kb peak
spatial splits 2.38 s wall, 1566% cpu, 458664 kb peak
```

**Observed.** Recorded as measured; a single run each on a busy machine is not enough to separate small differences from noise.

**Hand test 31 — the texture cache.** Done to a copy of the 640 by 480 Cycles scene with three 1024 by 1024 PNG images written next to it and wired into three materials, 52 kB of image files in all. First a render with nothing changed: `2.22 s wall, 552900 kb peak-rss`. Then the two settings were asked for by name:

```
HAS-use_texture_cache False HAS-use_auto_generate_texture_cache False
```

**Observed.** Neither setting exists in this version, so the rest of the test cannot be carried out here. Hand test 15 above establishes the same by a different route.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.0/` or `/root/render-demands/prefs-card-cuda/5.0/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 27, cost side — what the card costs and what the processor costs.** One file rendered three times, changing only which device did the work: once with Device CPU and the Cycles backend preference at None, once with Device GPU Compute and the backend CUDA, once with Device GPU Compute and the backend OptiX. In the two card runs only the card was ticked and the processor was left unticked. The file: Cycles, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, OpenEXR at full float with no compression. Timed by the operating system:

```
processor   : Cycles reported Time: 00:01.70, 1734% of a processor, peak memory 457560 kb
card, CUDA  : Cycles reported Time: 00:00.68, 127% of a processor, peak memory 580184 kb
card, OptiX : Cycles reported Time: 00:00.69, 119% of a processor, peak memory 775296 kb
```

**Observed.** Choosing the card rather than the processor changes what the render demands in three ways at once, all in the same direction in this version. It takes less time. It stops asking for the whole processor — about fifteen hundred per cent of one core drops to about one hundred per cent, which is to say the render stops being a processor job. And it asks for **more** ordinary memory, not less, because the card's own working memory is held in addition. This is one small scene on one busy machine; the shape is what is being recorded, not the seconds.

**Hand test 28, memory side — what the saved backend preference takes off the card even when the processor does the work.** Through every run of hand test 28 this Blender was watched for how much card memory it held, by asking the driver which processes held work on the card and how much each held — a reading about this process alone, so the other topic's Blender could not confuse it. On a heavier version of the same file, 1024 samples:

```
backend preference None, render on the processor : never held any card memory at all
backend preference CUDA, render on the processor : held card memory, peak 248 MiB
render on the card                               : held card memory, peak 1782 MiB
```

**Observed.** A saved backend preference costs card memory on its own. With the preference naming CUDA, this Blender took about 248 MB of the card for the whole length of a render that ran entirely on the processor; with the preference at None it took none. That is a cost the preference exacts even in a run where it makes no difference at all to which device renders.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.0/`.

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
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
EXIT=0
```

The command returned cleanly. Without `-b` the same three names were printed and the run also exited 0 — this machine has a screen, where the Linux machine has no display server and the same command fails there before reaching the list.

**Observed.** This build offers three engines and names EEVEE `BLENDER_EEVEE`, the same name the rented Linux machine got from it.

**Hand test 15 — which settings exist on this operating system.** The same sweep as on Linux: every property of `scene.render`, `scene.cycles`, `scene.eevee`, `scene.display` and the Cycles preferences, read out of a running Blender and compared across all four researched versions on this machine. This version's dump holds **271** properties.

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
no limit vs a 0.3-second limit : IDENTICAL-PIXELS False MAX-ABS-DIFF 3.1694188117980957 NUM-DIFFERING-CHANNEL-VALUES 921600 of 1228800
```

**Observed.** A time limit writes a complete file of the usual size holding a picture cut short, not a blank one and not a missing one. How long each render took was **not measured**, because the machine was not idle; the Linux run carries the times.

**Hand test 31 — the texture cache.** A copy of the same scene with three 1024 by 1024 PNG images written beside it and wired into the three materials.

```
HAS-use_texture_cache False HAS-use_auto_generate_texture_cache False
```

**Observed.** Neither setting exists in this version on macOS either, so the rest of the test cannot be carried out here. The same file rendered twice with nothing changed gave `IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`, and no folder appeared beside the images.

**Three tests were not run on this machine at all, and the reason is the same for all three.** Hand test 3 (whether `-t` changes how much of the processor a Cycles render actually uses), hand test 23 (whether the thread count changes anything under EEVEE or Workbench) and hand test 24 (whether Persistent Data changes the time or the memory of an animation) have no evidence except a time or a memory figure. The Mac was shared with other topics' work throughout, so a figure taken here would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. **What they leave open on macOS is left open and is not guessed at.**

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.0/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — which picture comes out when the file asks for the graphics card and the settings folder names no backend. This is hand test 7 rebuilt, because hand test 7 could not have failed.** That test searched Blender's printed output for complaints about a missing card and found none; "Blender did not complain" and "Blender did not use the card" are different things, and hand test 26 in this same document is why no search of output can tell them apart. **What changed is the evidence: the question is now settled by which picture came out.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render.

Five renders, none of them with `--factory-startup`, so the saved preferences were read every time: `blender -b <file> -o <folder>/<tag>_ -F OPEN_EXR -x 1 -f 1 --`. (a) the file saying CPU, no argument — the **processor reference**; (b) the same again, unchanged; (c) the file saying CPU with `-- --cycles-device CUDA` — the **card reference**; (d) the same again, unchanged; (e) **the test itself**, the file saying GPU Compute with the backend preference naming nothing.

The two controls first, and the size of the gap this has to judge against:

```
processor run 1 vs run 2 : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0                  DIFFERING 0 of 1228800
card run 1 vs run 2      : IDENTICAL-PIXELS False MAX-ABS-DIFF 2.86102294921875e-06    DIFFERING 469939 of 1228800
processor vs card        : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.06181693077087402     DIFFERING 601048 of 1228800
```

and then the test:

```
(e) file GPU Compute, backend preference NONE, no argument
    against the processor reference : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0              DIFFERING 0 of 1228800
    against the card reference      : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.06181693077087402  DIFFERING 601048 of 1228800
```

**Observed.** A file asking for GPU Compute, in a settings folder naming no backend, renders on the **processor** — not approximately but to the very same numbers — and misses the card by the whole device gap, which is some twenty thousand times the difference between two runs on the same device. The test could have come out the other way and did not. A second reading says the same without any reference to the pictures: what each run held on the card, this Blender alone.

```
              Cycles reported   % of a processor   card memory held by this run
processor        Time: 00:02.26      1375%          none at all
card (CUDA)      Time: 00:00.56      87%            1760 MiB
file GPU, no backend named   Time: 00:02.19   1399%   none at all
```

The wall-clock times were 0:02.82, 0:01.56 and 0:02.77; they are a busy machine's times and carry nothing on their own. **The card memory carries it: the run never touched the card.** The five renders were made twice, ten minutes apart, and both passes gave pixel-identical pictures (`IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`), so this is not one moment's accident. **Still owed on the Creator's Mac**, where the question is sharper because the graphics there work perfectly.

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
--cycles-device CUDA   exit 0, image written : matched the CARD reference, MAX-ABS-DIFF 3.337860107421875e-06; against the processor 0.06181693077087402
--cycles-device OPTIX  exit 0, image written : matched NEITHER — against the processor 0.061817288398742676, against the CUDA card 0.05098986625671387
--cycles-device HIP    exit 1, no image written
--cycles-device ONEAPI exit 1, no image written
--cycles-device METAL  exit 1, no image written
```

OptiX being a third picture is not a fault in the test: hand test 27 already found the two backends on the same card disagreeing with each other. What matters is that it is not the processor's picture. What the three refused runs printed, word for word:

```
00:00.286  reports          | ERROR Found no Cycles device of the specified type
```

and `HIP` printed one line more, before it:

```
00:00.285  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
00:00.287  reports          | ERROR Found no Cycles device of the specified type
```

Card memory, this Blender alone: `CPU`, `HIP`, `ONEAPI` and `METAL` held **none at all**; `CUDA` held 264 MiB and `OPTIX` 1774 MiB. (The card was sampled five times a second and these renders last about two seconds, so a peak can be missed — the reading that carries weight is nothing against something.)

**Observed.** Naming a backend this build accepts but has no hardware for does **not** quietly fall back to the processor: the render stops, writes no image, and says `Found no Cycles device of the specified type`. So a program that writes `HIP` or `ONEAPI` into this setting on this machine gets no error when it writes it, and finds out at render time. **Still owed on the Creator's Mac** for the device-listing half and the render half.

**Hand test 35 — the property sweep written out exactly, so its totals can be checked.** The citation check sent hand test 15 back over its numbers, not its finding: three sittings had reported three different totals and the procedure as written did not say whether each block's own `rna_type` entry is counted. The sweep was run again with that stated — `blender -b --factory-startup <file> --python <script>`, walking `bl_rna.properties` of five blocks named exactly (`scene.render`, `scene.cycles`, `scene.eevee`, `scene.display`, and the Cycles add-on's preferences), counting each block twice, once including `rna_type` and once excluding it. This version: `scene.render` 98 / 97, `scene.cycles` 106 / 105, `scene.eevee` 53 / 52, `scene.display` 10 / 9, the Cycles preferences 9 / 8 — **276 counting `rna_type`, 271 not counting it**.

**Observed.** The three totals on the record are now told apart. Counting `rna_type` gives 290, 276, 278 and 286 across the four versions, which is the citation check's figure for 4.5 exactly. Not counting it gives 285, 271, 273 and 281, which is the Mac sitting's figures exactly. The first Linux sitting's 286, 272, 274 and 282 are **neither** — one more than the not-counting figure in every version — so **they cannot be reproduced from any stated rule and should be read as decoration**. This version's earlier figure was 272. **The finding itself reproduced exactly**: the set of property names not present in all four versions came back as the same twenty-eight, on the same versions, `scene.eevee.fast_gi_thickness_far` among them — present in 4.5, 5.0 and 5.1 and absent from 5.2 — so the count and the written-out list now agree.



## 2. For each of them, what does it change about what the render produces?

**Label: documented for those whose pages say so; not in the sources for several, and I say which.**

The sources split into three groups. I keep the order of question 1.

### Described as a cost only, with the image never mentioned

*Heading corrected by the citation check, 2026-08-07: it used to read "Stated to leave the picture alone", which the section's own opening sentence contradicts — only one setting is stated outright, and the rest is inference with the step written out below.*

Only one setting is stated outright to be a pure trade of one resource for another with no mention of the image at all. Several others are described the same way — cost against cost, image never mentioned — which is weaker.

- **Threads Mode / Threads** — "Method to determine the maximum number of CPU cores to use while rendering". The description is entirely about cores and about being able to use your computer while rendering. Nothing about the image. That it leaves the image alone is **inferred**, and the step is: the description names no effect on output, and the whole Performance panel is introduced as "Properties that affect render speeds or memory consumption". A reader has to supply "and nothing else", so this is inference, not documentation.

- **Persistent Data** — "Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage while performing other tasks in Blender." Speed and memory only. Same inference, same step.

- **Tile Size** — "Render high resolution images in tiles of this size, to reduce memory usage." Memory and, per the note, sometimes speed. Same inference, same step.

- **Use Spatial Splits** — "improve the rendering performance … The downsides are longer BVH build times and slightly increased memory usage." Time and memory only.

- **Use Curves BVH** — "Disabling this option will reduce memory, at the cost of increasing curve render time." Memory and time. But this one carries a hint that it is not purely neutral: "The bounding boxes are not axis aligned allowing a spatially closer fit to the curve geometry." The page does not say whether the fit changes the picture.

- **BVH Time Steps** — "speed up render time at the expense of memory."

- **Use Compact BVH** — "can reduce RAM usage but render slower."

- **Kernel Optimization** — the choices differ only in "Slowest / Faster / Fastest rendering" and background CPU usage (**observed**, `cycles_5.0/properties.py`).

- **Distribute Memory Across Devices** — "effectively freeing up space for larger scenes."

- **`--qos`** — about which CPU cores Blender's threads are scheduled on.

- **`-b, --background`** — "Run in background (often used for UI-less rendering)."

For every item in this group the honest statement is: **the sources do not say these change the picture, and do not say they leave it unchanged either.** Absence of a claim is what I have.

### Stated to change the picture

- **Simplify — Max Subdivision** limits "Maximum number of subdivision by the Subdivision Surface modifiers", so surfaces are coarser. **Child Particles** shows "only a subset of all child hairs and particles". **Texture Limit** "Automatically scales textures down so that they are no larger than the values chosen." **Camera Cull** and **Distance Cull** remove objects from the render. All four change what is in the image, by their own descriptions. And the whole block is introduced in the API as "Enable simplification of scene for quicker preview renders".

- **Render Texture Limit** (Cycles) — "Limit texture size used by final rendering" (**observed**). Same kind of change as Simplify's texture limit, aimed at the final render.

- **High Quality Normals** — "can improve visual quality for dense meshes with high frequency textures at the cost of memory."

- **Resolution X, Y and Percentage** — "The number of pixels horizontally and vertically in the image", and a slider "to reduce or increase the size of the rendered image". The output file's dimensions change.

- **Render Region** — "Renders just a portion of the view instead of the entire frame", and with Crop to Render Region off, the rest is "a transparent background around it."

- **Render Samples** (Cycles) — "A higher number results in a cleaner image at the cost of a longer render time."

- **Time Limit** (Cycles) — "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet." So the picture may be noisier than asked for. That the picture is noisier is **inferred** from that sentence plus "A higher number results in a cleaner image"; the step is joining the two.

- **Noise Threshold** — "lower values meaning better quality but longer render times."

- **Denoise Nodes: Final Quality** — "High: Produces the highest quality output at the cost of long processing times. … Fast: Produces an output quickly at a noticeable cost of quality."

- **Compositor Precision GPU** — "The precision of compositor intermediate result", Auto being "full precision for final renders, half precision otherwise". Precision of the result is by definition the result.

- **Light Tree** — "can significantly reduce noise, at the cost of a somewhat longer render time per sample."

- **Fast GI Approximation** (Cycles) — "final renders with reduced quality" (**observed**).

- **Volume Step Rate** — "Higher values reduce render time, lower values render with more detail" (**observed**).

- **Motion blur steps** — "Control accuracy of motion blur, more steps gives more memory usage" (**observed**).

- **EEVEE Render Samples** — "the more samples the more aliasing is reduced at the cost of performance."

- **EEVEE Shadow Steps** — "Higher step count results in softer shadows but have a higher cost." **Shadow Rays** — "Higher values reduces the noise". **Shadow Resolution** — "Resolution percentage of shadow maps." **Light Threshold** — "Minimum light intensity for a light to contribute to the lighting … Lower values improve performance", so raising it drops light contributions.

- **EEVEE Volumes Resolution and Steps** — both stated to trade quality against video memory.

- **EEVEE Ray-tracing Method, Resolution, Precision, and the Fast GI settings** — "Lower options will be faster and use less memory but will produce blurrier results", and turning ray tracing off gives "a more visually stable and optimized alternative when visual fidelity is not the primary goal."

- **EEVEE Shadow Pool and Light Probes Volume Pool** — indirectly: "Increasing the size might fix the Shadow buffer full error", so too small a pool shows as an error rather than a silently different picture. What the picture looks like when the pool is too small is not stated.

### Where the answer genuinely is not in the sources

- **Cycles Device (CPU versus GPU Compute).** The manual says "GPU rendering may have limitations compared to CPU rendering, depending on the backend" and "This mode is compatible with all Cycles features" of CPU. It names two things a GPU cannot do — "Path Guiding is not supported on any GPU" and "Open Shading Language is only supported for OptiX". The manual does carry one such statement, on the Cycles Curves page, under Linear 3D Curves: "This is an optimization to take advantage of hardware accelerated curve intersection on GPUs. Results will not look exactly the same on different CPU and GPU devices, so this is not suitable for multi device rendering or render farms with mixed hardware." — Blender manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/hair.html, read 2026-07-30. It is narrow — it is about one curve shape option, not about rendering in general — but it is a statement, in these sources, that the same scene does not come out the same on the two devices, and it names render farms with mixed hardware in so many words. Beyond that one option, it does **not** say anywhere I could find whether the same scene rendered on CPU and on GPU produces the same pixels. **Partly documented; not in the sources for the general case.** *Corrected by the citation check, 2026-08-07: this entry used to read "Not in the sources" outright. The Curves sentence was in the 5.0 snapshot the whole time, and this document's own companion, `What a render demands.md` for 5.0, quotes it correctly under question 2 group (a).*

- **Compositor Device (CPU versus GPU).** Same silence. Precision is documented; whether CPU and GPU compositing agree is not.

- **Denoising Device (Auto / CPU / GPU), and Cycles' "Use GPU" denoising.** Both are described purely as speed and memory. Whether the denoised image is the same either way is not stated.

- **MetalRT, HIP RT, Embree on GPU.** Described as memory and performance. Nothing on the picture.

**Not covered.** I have not established, for any setting in the first group, that it truly leaves the picture untouched — only that the sources do not claim it changes it. That is a weaker statement and should not be read as the stronger one. For settings in the second group I have the direction of the change and no measure of it.

**Outside the interface.** Nothing in any of these descriptions is conditioned on a person being at the screen; they describe what the render produces. The one setting whose behaviour is stated to differ is `use_single_layer`, quoted at question 1, and that is about which layers render, not about the Performance settings. The sources do not otherwise say.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Operating system.** The image effects described above carry no operating-system qualification anywhere I read. The availability of some of the settings does — MetalRT only on Apple, HIP RT only with AMD, Embree on GPU "Only supported with oneAPI rendering devices", and the backend-by-system list at question 1. Whether the same scene renders to the same pixels on two different operating systems is **not in the sources**; I looked and found nothing addressing it.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.0/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 16 — whether thread count, tile size, Use Tiling, the three acceleration-structure options, BVH Time Steps, Persistent Data or High Quality Normals change the picture.** Done to a scene built from a script and saved: Cycles, Device CPU, 640 by 480 at 100 per cent, 64 samples, Seed 7, Use Animated Seed off, Denoise off, Adaptive Sampling off, Max Bounces 4, stamping off, saved as OpenEXR at full float with no compression. Twelve renders of frame 1, each changing exactly one thing — nothing; `-t 1`; `-t 8`; Tile Size 64; Tile Size 8192; Use Tiling off; Use Spatial Splits on; Use Compact BVH on; Use Curves BVH off; BVH Time Steps 4; Persistent Data on; High Quality Normals on — then each compared against the base **by pixels**, by loading both images and subtracting them channel by channel. What came back, for every one of the eleven pairs:

```
IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
```

Two control renders were made to show the comparison can tell a difference at all — without them no result from it would mean anything:

```
seed 7 → 8: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.30967599153518677 NUM-DIFFERING-CHANNEL-VALUES 862779 of 1228800
64 samples → 63: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.04032999277114868 NUM-DIFFERING-CHANNEL-VALUES 862791 of 1228800
```

**Observed.** None of those eleven changes altered a single channel value, on this scene, on this machine, in this version.

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench as well as Cycles.** Done to a scene with a Subdivision Surface modifier at render level 4. Six renders: each of the three engines with Simplify off, and each with `use_simplify` on and `simplify_subdivision_render` set to 0, compared by pixels:

```
CYCLES            : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.1228286623954773 NUM-DIFFERING-CHANNEL-VALUES 28215 of 76800
BLENDER_EEVEE     : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.105712890625     NUM-DIFFERING-CHANNEL-VALUES 3564 of 76800
BLENDER_WORKBENCH : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.0909423828125    NUM-DIFFERING-CHANNEL-VALUES 3398 of 76800
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

**Hand test 19 — what a Time Limit does to the picture.** Same scene rendered with Time Limit 0 (`Time: 00:01.63`) and with Time Limit 0.3 seconds (`Time: 00:00.41`). Both files 4923758 bytes; compared by pixels, `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.39824867248535156`.

**Observed.** A time limit changes the picture as well as the cost.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.0/` or `/root/render-demands/prefs-card-cuda/5.0/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 27 — whether the same file renders to the same picture on the processor and on the card.** One file, three renders, changing only the device: Device CPU with the backend preference at None; Device GPU Compute with the backend CUDA; Device GPU Compute with the backend OptiX. The file was built to leave nothing else free to move: Cycles, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, no path guiding, no Open Shading Language, four bounces, OpenEXR at full float with no compression. The three images were then compared **by pixels** — both loaded and subtracted channel by channel — and never by comparing the saved files. What came back:

```
processor vs card (CUDA)  : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.04754999279975891  DIFFERING 703976 of 1228800  MEAN-ABS-DIFF 0.0001513283932581544
processor vs card (OptiX) : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.057655155658721924  DIFFERING 703891 of 1228800  MEAN-ABS-DIFF 0.00016245522419922054
card CUDA vs card OptiX   : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.05639088153839111  DIFFERING 674217 of 1228800  MEAN-ABS-DIFF 0.0001313407556153834
```

The control, which is what makes those numbers mean anything: each of the three renders was run a second time, unchanged, and compared with its first run.

```
processor, run 1 vs run 2  : IDENTICAL-PIXELS True   MAX-ABS-DIFF 0.0
card CUDA, run 1 vs run 2  : IDENTICAL-PIXELS False  MAX-ABS-DIFF 7.152557373046875e-07  DIFFERING 614743 of 1228800
card OptiX, run 1 vs run 2 : IDENTICAL-PIXELS False  MAX-ABS-DIFF 5.960464477539062e-07  DIFFERING 612386 of 1228800
```

**Observed.** **The device changes the picture.** Which device did the work is not only a matter of what the render costs; the image that comes out is a different image. Two things say how much of a difference it is. It is small in size — the largest single channel value moves by about 0.0475 on a nought-to-one scale, and the average move is about a ten-thousandth — but it is spread over more than half of the image. And it is far above the noise of the machine: the processor repeats its own render exactly, and the card repeats its own render to within about a millionth, which is about the finest step a number of this kind can take, while the difference between processor and card is some tens of thousands of times larger than that. The two card backends do not agree with each other either. So a frame rendered on the processor and a frame rendered on the card are not interchangeable, and nor are frames from two different backends on the same card.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.0/`.

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
base vs seed8 (seed 7 -> 8):    IDENTICAL-PIXELS False MAX-ABS-DIFF 0.6618762016296387 NUM-DIFFERING-CHANNEL-VALUES 714816 of 1228800
base vs s63 (64 samples -> 63): IDENTICAL-PIXELS False MAX-ABS-DIFF 0.300076961517334 NUM-DIFFERING-CHANNEL-VALUES 921588 of 1228800
```

**Observed.** Thread count, tile size, Use Tiling, the three acceleration-structure options, BVH Time Steps, Persistent Data and High Quality Normals all left the picture untouched, on a machine with a different processor from the one the same eleven were shown clean on. The rented Linux machine gave the same eleven identical results. **The two machines agree.**

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench.** A scene with a Subdivision Surface modifier at render level 4, rendered under each of the three engines with Simplify off, then again with Simplify on and Max Subdivision (render) at 0, each pair compared by pixels:

```
CYCLES              simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.35849910974502563 NUM-DIFFERING-CHANNEL-VALUES 37740 of 76800
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
processor vs card (Metal) : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.05019509792327881 NUM-DIFFERING-CHANNEL-VALUES 638407 of 1228800 MEAN 9.153825214980315e-07
```

**And the control, without which none of that means anything** — each of the two renders repeated, unchanged, and compared with its own first run:

```
processor run 1 vs run 2   : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0
card Metal run 1 vs run 2  : IDENTICAL-PIXELS False MAX-ABS-DIFF 2.384185791015625e-06 NUM-DIFFERING-CHANNEL-VALUES 257832 of 1228800
```

**Observed.** **The device changes the picture on this machine too.** More than half of the channel values differ, the largest by about three to five hundredths on a nought-to-one scale. The processor repeats its own render exactly; Apple's graphics do not, but only by about a millionth, which is some tens of thousands of times smaller than the gap between the two devices. The rented Linux machine found the same thing with an NVIDIA card through CUDA and through OptiX. **Two different makes of graphics processor, two operating systems, the same answer: choosing the device changes the image, not only what the image costs.** What it costs was not measured here; the Linux machine carries that.

**Hand test 20 — a start-up script setting Samples.** `--python-expr "import bpy; bpy.context.scene.cycles.samples=7"` against the 64-sample file:

```
SAMPLES-SET 7
the 64-sample base vs the 7-sample render : IDENTICAL-PIXELS False MAX-ABS-DIFF 1.968954086303711 NUM-DIFFERING-CHANNEL-VALUES 921600 of 1228800
```

**Observed.** The assignment reaches the render and changes the picture. The render time was not recorded here.

**Hand test 31 — the texture cache and the picture.**

```
HAS-use_texture_cache False HAS-use_auto_generate_texture_cache False
```

**Observed.** Neither setting exists in this version on macOS either, so the rest of the test cannot be carried out here. The same file rendered twice with nothing changed gave `IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`, and no folder appeared beside the images.

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.0/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — what the device does to the picture, and what a file asking for a card it has not been given produces.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render.

Two renders of it, changing only the device, plus a repeat of each: the file saying CPU with no argument, and the same file with `-- --cycles-device CUDA`. Compared on the pixels, never on the saved files:

```
processor run 1 vs run 2 : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0                  DIFFERING 0 of 1228800
card run 1 vs run 2      : IDENTICAL-PIXELS False MAX-ABS-DIFF 2.86102294921875e-06    DIFFERING 469939 of 1228800
processor vs card        : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.06181693077087402     DIFFERING 601048 of 1228800
```

**Observed.** The device changes the picture, on a second scene and in a second sitting: about half the channel values differ, the largest by some hundredths on a nought-to-one scale, where the processor repeats its own render exactly and the card repeats its own to about three millionths. This is hand test 27's result standing up on a scene built for a different question.

**And the one that answers what the file's own Device produces.** The same scene saved with Device **GPU Compute**, rendered with the backend preference naming nothing:

```
against the processor reference : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0              DIFFERING 0 of 1228800
against the card reference      : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.06181693077087402  DIFFERING 601048 of 1228800
```

**Observed.** It produces the **processor's** picture, to the very last number. So a file whose Device says GPU Compute does not, on its own, produce the graphics card's output; what it produces depends on whether a backend has been named somewhere else, and this is settled here by which picture came out rather than by what Blender printed, which names no device at all.

**One more thing about the picture, from hand test 34.** The same file rendered with `-- --cycles-device OPTIX` matched **neither** reference — against the processor 0.061817288398742676, against the CUDA card 0.05098986625671387. Three devices, three pictures. **Still owed on the Creator's Mac.**



## 3. Which of them can be settled at the moment a render is started from outside Blender, and which only by changing the file?

**Label: documented for the list of options; observed for how Cycles reads its own; not in the sources for whether an option overrides the file's value.**

### Settled at start, by their own command-line option

The Command Line Arguments page lists these, and they are the whole set that bears on demand:

> "-E, --engine <engine>
> Specify the render engine. Use -E help to list available engines."

> "-t, --threads <threads>
> Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count."

> "Cycles Render Options
> Cycles add-on options must be specified following a double dash.
> --cycles-device <device>
> Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
> Append +CPU to a GPU device to render on both CPU and GPU.
> Example:
> blender -b file.blend -f 20 -- --cycles-device OPTIX"

> "--qos <level>
> Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only)."

> "-b, --background
> Run in background (often used for UI-less rendering)."

> "--gpu-backend
> Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."

Command Line Arguments page, address and date as at question 1. The Rendering From The Command Line page repeats two of them with worked examples: "blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a", explained as "-E CYCLES — Use the "Cycles Render" engine" and "-t 2 — Use only two threads."

`--cycles-device` is worth reading in Cycles' own file, because it says something the manual does not. **Observed**, `cycles_5.0/engine.py`, lines 16–20:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

That help text states it overrides *both* the preferences and the scene setting — a claim the manual page does not make. And the same file shows why the double dash is required, lines 24–29:

```
def _parse_command_line():
    import sys

    argv = sys.argv
    if "--" not in argv:
        return
```

**Observed**: if `--` is absent from the arguments, Cycles parses nothing, so `--cycles-device` is only read when it follows a `--`.

`--gpu-backend` I list for completeness but flag: the Preferences page ties Backend to "the graphics API used for drawing the interface and rendering display content", which is display, not Cycles rendering. I am not claiming it changes what a Cycles render demands.

### Settled at start, by running a script

The command line can run arbitrary Python before rendering:

> "-P, --python <filepath>
> Run the given Python script file."
> "--python-expr <expression>
> Run the given expression as a Python script."

Command Line Arguments page, same date. Every setting listed at question 1 exists as a writable Python property — for example `threads`, `threads_mode`, `use_persistent_data`, `resolution_percentage`, `use_simplify` on the RenderSettings page, `taa_render_samples` and `shadow_pool_size` on the SceneEEVEE page, and Cycles' own `device`, `tile_size`, `samples` and the rest, **observed** as `EnumProperty`, `IntProperty` and `BoolProperty` definitions in `cycles_5.0/properties.py`. That a script run this way can set them before the render is **inferred**; the step is joining "run the given script" to "these are settable properties". The manual does not spell out this route for render settings.

Order matters, and the manual is explicit about it:

> "Arguments are executed in the order they are given."
> "blender --background --render-output /tmp test.blend --render-frame 1
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

Command Line Arguments page, same date. And: "Always position -f or -a as the last arguments." (Rendering From The Command Line page, same date.)

### Only by changing the file

Everything at question 1 that is not in the two lists above. There is no command-line option for Tile Size, Persistent Data, any of the four Acceleration Structure settings, Simplify, Texture Limit, samples, Time Limit, the noise threshold, denoising, the compositor device, resolution, render region, or any EEVEE setting. I searched the whole arguments page and the whole manual for each; the record is below.

Which leaves a middle case worth naming plainly: those settings can still be *set* at start without editing the .blend, by running a script that changes them in memory after the file loads. The file on disk is untouched; the render uses the new value. The sources do not describe this workflow, so I mark that combination **inferred**, with the step written out above.

### Neither — set by the preferences

The section D settings at question 1 are in neither place. The manual points at the preferences for the GPU backend:

> "To enable GPU rendering:
> Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.
> Enable the available devices for the chosen backend.
> Configure each scene to use the GPU Compute Device."

GPU Rendering page, address and date as at question 1. Three steps, and only the third is in the .blend. The preferences live in their own file:

> "./config/userpref.blend
> User preferences."

Blender manual, Blender's Directory Layout page — https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html — read 2026-07-30, in the section headed "Located in user directories."

The one exception is `--cycles-device`, which per Cycles' own help text overrides them.

**Not covered.** Three real gaps here, and they are the ones that matter most for driving a render from outside.

1. **Whether `-t` overrides the file's Threads Mode and Threads, or is overwritten by loading the file.** The manual states the ordering rule for `--render-output` only: "loading the blend-file overwrites the render output that was set." It says nothing about `-t`. I searched the arguments page and the whole manual for a statement either way and found none. **Not in the sources.**

2. **Whether `-E` overrides the file's engine, and what happens if the named engine differs from the one the file's settings were written for.** The page says only "Specify the render engine."

3. **Whether a background render reads the user's preferences at all**, and what `--factory-startup` does to them. The page says only "--factory-startup — Skip reading the startup.blend in the users home directory" — startup.blend, not userpref.blend, and the Directory Layout page lists those as two separate files. Whether preferences are still read is **not in the sources**.

Also not covered: `--cycles-device` names a backend, not an individual card. Nothing in the sources offers a way to pick *which* GPU from the command line; the manual sends you to "Preferences ‣ System ‣ Compute Device Panel" for that.

**Outside the interface.** This whole answer is about the outside-the-interface case. The one thing to add: the arguments above are read at start-up, before or around the file load, and the manual's ordering warning shows that a value set before the file loads can be replaced by the file. Which of these options that applies to is exactly gap 1.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Operating system.** `--qos` is "(Windows only)"; `-con, --start-console` is likewise "(Windows only)". `--cycles-device` accepts METAL, which per the GPU Rendering page only exists on Apple Silicon macOS, and CUDA/OptiX/HIP/oneAPI, which per the same page are Windows and Linux. `-t`, `-E`, `-P` and `-b` carry no system restriction on the arguments page. The three ways to launch Blender from a command line have their own pages per system (Linux, macOS, Windows) which I did not need to open for this.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.0/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

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
    wall 0:15.04
(b) the same with the script before -t 3
    SET-BY-SCRIPT FIXED 3
    FINAL FIXED 3
    wall 0:14.94
(c) the script alone, no -t
    SET-BY-SCRIPT FIXED 5
    FINAL FIXED 5
    wall 0:09.93
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
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
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
00:00.433  cycles           | ERROR CUDA cuInit: Unknown error
DEVICE CPU
(d) -f 1 first, then read, with --cycles-device CPU      DEVICE CPU
(e) -- --cycles-device NOSUCHDEVICE
00:00.281  cycles           | ERROR NOSUCHDEVICE is not a valid Cycles device.
```

**Observed.** The stored value never moved, at either moment. The machine's card was unreachable, so what the override does to a render that can use a card is not settled here.

**Hand test 8 — whether a render started from outside reads the saved preferences.** Done in a settings folder of its own. The Cycles compute backend was set to `CUDA` and `bpy.ops.wm.save_userpref()` called, producing `config/userpref.blend` of 175237 bytes. Then read back:

```
plain background run (no --factory-startup)   READ-PREF CUDA
the same with --factory-startup               READ-PREF NONE
```

**Observed.** A background run does read the saved preferences; `--factory-startup` stops it.

**Hand test 20 — whether a start-up script can set a setting that has no argument of its own.** Done to the 640 by 480 scene, whose Samples is 64: `blender -b --factory-startup med.blend --python-expr "import bpy; bpy.context.scene.cycles.samples=7; print('SAMPLES-SET', bpy.context.scene.cycles.samples)" -o <folder>/s7_ -F OPEN_EXR -x 1 -f 1`. What came back: `SAMPLES-SET 7`, `Time: 00:00.29` against 00:01.63 for the unchanged render, and the picture differs from the 64-sample one — `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.5998931527137756`.

**Observed.** The assignment reaches the render.

**Hand test 21 — whether "Render Single Layer" is honoured from the command line.** Done to a file with two view layers, `ViewLayer` and `Second`, saved once with the setting off and once on, each rendered with `-f 1`. In both runs Blender named both view layers in its own progress lines (`Scene, Second` and `Scene, ViewLayer`).

**Observed.** The setting made no difference to a render started from the command line.

**One thing learned by getting a command wrong, recorded because it bears on every command in this answer.** An earlier run gave the arguments as `-f 1 -o <folder>/r_ -F OPEN_EXR -x 1`, with the render before the output path, and the image went to `/tmp/0001.exr` instead. `-o`, `-F` and `-x` have to come before `-f`.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.0/` or `/root/render-demands/prefs-card-cuda/5.0/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 28 — what `--cycles-device` does to a render that really can use the card.** This is the test the earlier sitting could not run at all. Six timed runs, none of them using `--factory-startup`, so the settings folder's saved preferences were read every time. Two files, both Cycles at 640 by 480 with 1024 samples: one saved with Device **CPU**, one saved with Device **GPU Compute**. Two settings folders: one with the Cycles backend preference saved as **None**, one with it saved as **CUDA**. Each run was `blender -b <file> -o <folder>/x_ -F OPEN_EXR -x 1 -f 1 -- --cycles-print-stats`, with the device argument added after `--cycles-print-stats` where one was given. Alongside the timing, this Blender alone was watched for whether it held work on the card.

```
file CPU,  preference None, no argument            : Time 00:25.37, wall 26.72 s, 2193% of a processor — never held any card memory at all
file CPU,  preference None, --cycles-device OPTIX  : Time 00:03.83, wall 4.39 s, 79% of a processor — held card memory, peak 1782 MiB
file CPU,  preference None, --cycles-device CUDA   : Time 00:03.59, wall 4.28 s, 90% of a processor — held card memory, peak 1768 MiB
file CPU,  preference CUDA, no argument            : Time 00:27.94, wall 28.78 s, 2040% of a processor — held card memory, peak 248 MiB
file GPU,  preference CUDA, no argument            : Time 00:03.17, wall 4.11 s, 92% of a processor — held card memory, peak 1768 MiB
file GPU,  preference CUDA, --cycles-device CPU    : Time 00:27.91, wall 28.64 s, 2047% of a processor — held card memory, peak 248 MiB
```

Two more runs were added, because this is the exact pair the 4.5 audit asked for — the file already asking for the card, with the preferences naming no backend at all:

```
file GPU,  preference None, no argument            : Time 00:29.62, wall 30.21 s, 2267% of a processor — never held any card memory at all
file GPU,  preference None, --cycles-device OPTIX  : Time 00:03.35, wall 4.18 s, 87% of a processor — held card memory, peak 1780 MiB
```

**Observed.** **The argument wins, in both directions.** A file that says CPU, with preferences that name no backend at all, renders on the card the moment `--cycles-device` names one — about seven times faster, the processor left almost idle, and the card holding well over a gigabyte. And a file that says GPU Compute, with CUDA saved in the preferences, is dragged back onto the processor by `--cycles-device CPU`. So neither the file's own Device setting nor the saved backend preference can stop the argument. What the two of them do decide is what happens when no argument is given: the file's Device says whether the card is used at all, and the preference says which backend. A file saying GPU Compute with the preference at None renders on the processor and never touches the card — which is the earlier sitting's result, confirmed here with a working card rather than an unreachable one — and a file saying CPU renders on the processor however the preference is set.

**A correction to hand test 26, which was run with no reachable card.** That test found that nothing Blender printed named the device that rendered. With a working card that holds for CUDA — a card render through CUDA printed no word about the card anywhere — but not quite for OptiX, whose renders name it in their ordinary progress lines while the acceleration structure is being built:

```
00:02.355  render           | Fra: 1 | Mem: 2M | Updating Scene BVH | Building OptiX acceleration structure
```

Those lines say OptiX built the structure. They are not a statement of which device rendered, and no line anywhere says that. Anything reading the device off Blender's output still cannot settle it; the timing, and what the process holds on the card, remain the evidence.

**Hand test 6, run again now the card works — whether `--cycles-device` changes the device stored in the scene.** The earlier sitting could only show this on a machine where the named device was absent, which left the obvious objection open. Repeated here against a file saved with Cycles and Device CPU, with the card present and working:

```
(a) no argument                                            : DEVICE CPU
(b) -- --cycles-device OPTIX, read before any render       : DEVICE CPU
(c) -- --cycles-device OPTIX, read after a render          : DEVICE CPU   (and the render ran on the card)
(d) -- --cycles-device CPU, read after a render            : DEVICE CPU
(e) -- --cycles-device NOSUCHDEVICE                        : 00:00.236  cycles           | ERROR NOSUCHDEVICE is not a valid Cycles device.
(f) preference CUDA, -- --cycles-device OPTIX, after render: DEVICE CPU   and the preference still reads CUDA
```

**Observed.** The stored value never moves, before a render or after one, whether or not the named device exists and whether or not it is used. The argument changes what does the work for that one run and writes nothing down — not into the scene, not into the saved preferences. An unknown device name is still refused by name. One thing changed from the earlier sitting: with no reachable card, asking for OptiX also produced `CUDA cuInit: Unknown error`; with the card working, that line is gone and the render simply runs on the card.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.0/`.

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
(b) file CPU,  preference None,  --cycles-device METAL     -> matched the METAL render (MAX-ABS-DIFF 1.9073486328125e-06)
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

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.0/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — what the file's stored Device settles on its own when a render is started from outside.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render. Rendered from the command line with `blender -b <file> -o <folder>/<tag>_ -F OPEN_EXR -x 1 -f 1 --`, no `--factory-startup`, so the saved preferences were read:

```
file CPU,  backend preference NONE, no argument            : the processor's picture (the reference)
file CPU,  backend preference NONE, -- --cycles-device CUDA : the card's picture      (the reference)
file GPU,  backend preference NONE, no argument            : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 against the PROCESSOR
                                                              IDENTICAL-PIXELS False MAX-ABS-DIFF 0.06181693077087402 against the CARD
```

**Observed.** From outside Blender, the file's stored Device is not enough on its own: a file saying GPU Compute renders on the processor unless a backend is named in the preferences or on the command line. The run held no card memory at all, which says the same a second way. This replaces the half of hand test 7 that searched Blender's output for complaints — a search that could not have failed, because no line of Blender's output names the device that rendered.

**Hand test 34 — what `--cycles-device` does when it names a backend this build accepts but has no hardware for.** Six renders of the file that says CPU, with the preference naming nothing, the argument naming each backend in turn:

```
--cycles-device CPU    exit 0, image written : the processor's picture exactly, MAX-ABS-DIFF 0.0
--cycles-device CUDA   exit 0, image written : the card's picture, MAX-ABS-DIFF 3.337860107421875e-06
--cycles-device OPTIX  exit 0, image written : a third picture, matching neither
--cycles-device HIP    exit 1, no image written
--cycles-device ONEAPI exit 1, no image written
--cycles-device METAL  exit 1, no image written
```

and what the three refused runs printed, word for word:

```
00:00.286  reports          | ERROR Found no Cycles device of the specified type
```

and `HIP` printed one line more, before it:

```
00:00.285  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
00:00.287  reports          | ERROR Found no Cycles device of the specified type
```

**Observed.** The argument does not quietly fall back. A backend with no hardware behind it stops the render, writes no image, and returns a non-zero exit code — which is a thing anything driving Blender from outside can actually see, unlike the device that rendered. An unknown name is a different case again and was refused by name in the earlier sitting. **Still owed on the Creator's Mac.**



## 4. Do any of them belong to a particular render engine rather than to every engine?

**Label: documented, with the sharpest evidence observed in Cycles' own files.**

Yes. The split runs three ways, and it is not the same as the split between "in the scene's render settings" and "in Cycles' settings".

### Belong to Cycles alone

Everything on `scene.cycles`: Device, Tile Size, the four Acceleration Structure settings, Render Texture Limit, Samples, Time Limit, adaptive sampling, denoising, path guiding, light tree, bounces, Fast GI, volume step rate, dicing rate. These exist because the Cycles add-on registers them:

```
    @classmethod
    def register(cls):
        bpy.types.Scene.cycles = PointerProperty(
            name="Cycles Render Settings",
            description="Cycles render settings",
            type=cls,
        )
```

**Observed** — `cycles_5.0/properties.py`, lines 1101–1107. That they cease to exist when Cycles is not registered is **inferred** from the matching `unregister` two lines below, "del bpy.types.Scene.cycles"; the step is reading a deletion as removal of the property.

### Belong to EEVEE alone

Everything on `scene.eevee`: Shadow Pool, Light Probes Volume Pool, render samples, shadow rays and steps, volumes resolution and steps, the ray-tracing block. The manual's EEVEE Render Settings index lists "Sampling, Clamping, Raytracing, Volumes, Curves, Depth of Field, Motion Blur, Film, Performance, Grease Pencil" — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/index.html — read 2026-07-30. The Cycles index lists a different set, including "Simplify", "Performance ‣ Threads", "Performance ‣ Memory", "Performance ‣ Acceleration Structure" and "Performance ‣ Final Render" — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html — same date.

### Live in the shared render settings but are only shown by one engine

This is the case that would be easy to get wrong. **Threads Mode, Threads and Persistent Data are properties of `scene.render`, not of Cycles** — the API documents them on RenderSettings, which belongs to every scene whatever the engine. But the panels that show them are Cycles' own. **Observed**, `cycles_5.0/ui.py`:

```
class CYCLES_RENDER_PT_performance_threads(CyclesButtonsPanel, Panel):
    bl_label = "Threads"
    bl_parent_id = "CYCLES_RENDER_PT_performance"
...
        col.prop(rd, "threads_mode")
        sub = col.column(align=True)
        sub.enabled = rd.threads_mode == 'FIXED'
        sub.prop(rd, "threads")
```

(lines 846–864), and

```
class CYCLES_RENDER_PT_performance_final_render(CyclesButtonsPanel, Panel):
    bl_label = "Final Render"
    bl_parent_id = "CYCLES_RENDER_PT_performance"
...
        col.prop(rd, "use_persistent_data", text="Persistent Data")
```

(lines 930–941). Note also `sub.enabled = rd.threads_mode == 'FIXED'` — **observed**: the Threads number is greyed out unless Threads Mode is Fixed.

Matching this, the manual's EEVEE Performance page has no Threads, no Persistent Data, no Memory and no Acceleration Structure section, and the word "Simplify" does not appear anywhere under EEVEE or Workbench in the manual (I searched; record below). So: **the property exists for every engine, the interface for it is Cycles'.** Whether EEVEE and Workbench honour `threads`, `use_persistent_data` and the Simplify render values at all is **not in the sources** — no page I found says either way.

Simplify is the same shape: `use_simplify`, `simplify_subdivision_render`, `simplify_child_particles_render` and the rest are on RenderSettings, but the manual documents Simplify only as a Cycles page.

**High Quality Normals** runs the other way: the property `use_high_quality_normals` is on RenderSettings, and the manual documents it on the **EEVEE** and **Workbench** Performance pages and not on the Cycles one.

**Compositor Device, Precision and the Denoise-node settings** are documented identically on both the Cycles and the EEVEE Performance pages — the same words on both — so they are shared in fact as well as in storage. That fits: compositing happens after the engine has finished.

**Resolution X/Y, Percentage and Render Region** are on the Output properties, documented once, with no engine named.

### Within Cycles, some belong to particular devices

The Acceleration Structure settings are not all offered for every device. **Observed**, `cycles_5.0/ui.py`, lines 881–927: the whole panel is skipped for OptiX unless a second device is in play —

```
    @classmethod
    def poll(cls, context):
        return not use_optix(context) or use_multi_device(context)
```

— and inside it, `debug_use_compact_bvh` is shown only where Embree is present, with `debug_bvh_time_steps` and `debug_use_hair_bvh` shown in its place otherwise, alongside the message "Cycles built without Embree support" / "CPU raytracing performance will be poor".

Matching that from the manual's side: "Path Guiding is not supported on any GPU", "Open Shading Language is only supported for OptiX", "Path guiding is only available when rendering on a CPU" (GPU Rendering and Cycles Sampling pages, dates as above).

**Not covered.** The sheet's question is about engines; I have answered for Cycles, EEVEE and Workbench because those are the three the manual documents. Whether a third-party engine sees or honours any of the shared properties is not something the sources address. And, as said above, I could not establish whether EEVEE or Workbench actually use `threads`, `use_persistent_data` or the render-time Simplify values.

**Outside the interface.** The Cycles/EEVEE split is a split in the data and in the code that registers it, not in the interface, so it holds however the render is started. The one part of this answer that *is* about the interface is which panels appear — that plainly has no meaning in a driven render. The properties themselves remain readable and writable either way; that is **inferred** from their being ordinary Python properties, with the step being that Cycles registers them at add-on registration time (`cycles_5.0/properties.py`, `register()`), not at window-drawing time.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Operating system.** The device-specific parts vary by system exactly as at question 1 — MetalRT only where Metal exists, HIP RT only with AMD, Embree on GPU only with oneAPI, and Metal only on Apple Silicon macOS for 5.0 per the requirements page. The engine split itself carries no system qualification in any source I read.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.0/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 22 — whether EEVEE and Workbench render at all on a machine with no display server.** Done to the base scene: `blender -b --factory-startup scene.blend -E BLENDER_EEVEE -o <folder>/e_ -F OPEN_EXR -x 1 -f 1`, then the same with `-E BLENDER_WORKBENCH`. **Both rendered and wrote an image, exit code 0.** EEVEE took `Time: 00:31.05`; Workbench about 1.2 seconds. Workbench also printed one line before saving: `EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).`

**Observed.** This machine had neither a display server nor a usable graphics card, and both engines still produced their image.

**Hand test 23 — whether the thread count changes anything under EEVEE or Workbench.** Done to the base scene with EEVEE Render Samples 16. Twelve runs: each engine, `-t 1` and `-t 24`, three times each, timed by the operating system. Twenty-four logical processors, machine shared with other work:

```
EEVEE     -t 1  : 8.62 s wall, 359% cpu, 1664108 kb / 7.96 s, 377%, 1671688 kb / 8.15 s, 358%, 1670320 kb
EEVEE     -t 24 : 7.81 s wall, 374% cpu, 1850324 kb / 7.88 s, 372%, 1848404 kb / 8.30 s, 356%, 1805932 kb
WORKBENCH -t 1  : 1.38 s wall,  92% cpu,  583076 kb / 1.34 s,  94%,  571280 kb / 1.41 s,  89%,  575168 kb
WORKBENCH -t 24 : 1.39 s wall, 115% cpu,  724828 kb / 1.39 s, 107%,  709292 kb / 1.42 s, 109%,  711736 kb
```

**Observed.** The two sets of times overlap in both engines, and the processor percentages stay put. Under Cycles the same argument changed the time by a factor of fifteen (hand test 3). **This is "no effect seen", not "the engine ignores it"** — a render spending its time elsewhere would look the same.

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench.** Six renders of a scene with a Subdivision Surface modifier at render level 4, each engine with Simplify off and with `simplify_subdivision_render` at 0, compared by pixels:

```
CYCLES            : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.1228286623954773 NUM-DIFFERING-CHANNEL-VALUES 28215 of 76800
BLENDER_EEVEE     : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.105712890625     NUM-DIFFERING-CHANNEL-VALUES 3564 of 76800
BLENDER_WORKBENCH : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.0909423828125    NUM-DIFFERING-CHANNEL-VALUES 3398 of 76800
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

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.0/`.

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
CYCLES              simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.35849910974502563 NUM-DIFFERING-CHANNEL-VALUES 37740 of 76800
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

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/5.0/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

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

**Hand test 35 — which of the settings named in one version's findings exist in the other three, with the sweep written out exactly.** The citation check sent hand test 15 back over its totals, which three sittings had reported three ways. The sweep was run again with the rule stated: five blocks named exactly (`scene.render`, `scene.cycles`, `scene.eevee`, `scene.display`, and the Cycles add-on's preferences), each counted twice, once including its own `rna_type` entry and once excluding it. This version: `scene.render` 98 / 97, `scene.cycles` 106 / 105, `scene.eevee` 53 / 52, `scene.display` 10 / 9, the Cycles preferences 9 / 8 — **276 counting `rna_type`, 271 not counting it**. Counting it gives 290, 276, 278 and 286 across the four versions, which is the citation check's figure; not counting it gives 285, 271, 273 and 281, which is the Mac's; and the first Linux sitting's 286, 272, 274 and 282 are neither and **cannot be reproduced**. **The finding is untouched by that**: the set of names not present in all four versions came back as the same twenty-eight, on the same versions, including `scene.eevee.fast_gi_thickness_far` — in 4.5, 5.0 and 5.1, absent from 5.2. **Still owed on the Creator's Mac**, as a repeat under the stated rule.



## 5. Can their values be read out of a file?

**Label: documented for reading through Blender; not in the sources for reading a .blend without Blender.**

### Through Blender

Yes for everything stored in the scene. The API documents loading data-blocks straight out of another blend file, by name, without opening it as the current file:

> "load(filepath, *, link=False, relative=False, set_fake=False, recursive=False, reuse_local_id=False, assets_only=False, clear_asset_data=False, create_liboverrides=False, reuse_liboverrides=False, create_liboverrides_runtime=False)
> Returns a context manager which exposes 2 library objects on entering. Each object has attributes matching bpy.data which are lists of strings to be linked.
> Parameters:
> filepath (str | bytes) – The path to a blend file."

and the page's own example loads a scene:

> "# Load a single scene we know the name of.
> with bpy.data.libraries.load(filepath) as (data_src, data_dst):
> data_dst.scenes = ["Scene"]"

Blender Python API — https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html — read 2026-07-30.

Once a scene is in hand, the values are plain properties: "threads", "threads_mode", "use_persistent_data", "resolution_x", "resolution_y", "resolution_percentage", "use_simplify", "simplify_subdivision_render", "compositor_device" and the rest on the RenderSettings page; "taa_render_samples", "shadow_pool_size", "gi_irradiance_pool_size", "volumetric_samples", "ray_tracing_method" and the rest on the SceneEEVEE page. Both read 2026-07-30. Each entry gives a type, a range and a default — for instance "resolution_percentage … int in [1, 32767], default 100".

Reading the scene's settings from a blend file this way is **inferred** rather than documented: the documentation shows a scene being loaded and separately documents the properties on a scene, and the step is joining the two. It does not spell out "here is how to read a file's render settings".

Cycles' values come with a condition. They are stored under `scene.cycles`, which only exists while the Cycles add-on is registered (question 4). Cycles' own file gives strong **observed** evidence that those values are genuinely in the blend file and are read back out of it — this is its versioning code, which reaches into freshly loaded scenes and repairs old values, `cycles_5.0/version_update.py`, lines 109–121:

```
        # Scenes
        for scene in bpy.data.scenes:
            if scene.library not in libraries:
                continue

            # Auto tiling is always enabled now
            if version <= (5, 0, 77):
                cscene = scene.cycles
                if not cscene.use_auto_tile:
                    cscene.use_auto_tile = True
                    cscene.tile_size = 8192
```

Two things follow. That per-scene Cycles values persist in the file is **inferred** from this code reading and writing them per loaded scene — the step being that there would be nothing to version if they did not. And, more sharply: **a Tile Size read out of an older file is not necessarily the Tile Size that was saved.** In a file written before version (5, 0, 77) with tiling switched off, this code sets `tile_size` to 8192 on load. Anything reading the value through Blender sees 8192, not what the author set. That is **observed** in the file quoted; that it applies to real files of that age is **inferred**, the step being that `do_versions` is registered as a load handler (`from bpy.app.handlers import persistent`, and `@persistent` above `do_versions`, same file, lines 9 and 66).

The same file also shows the file's own version is readable: "bpy.data.version" and "library.version" (lines 99–101).

### Through the file itself, without Blender

**Not in the sources** for how, and the release notes give a warning that bears directly on it:

> "The .blend file format has been changed to support larger buffers (blender/blender@59a759b98d1). This is necessary to be able to store e.g. meshes with more than a few hundred million vertices. … This change affects the structure of the file header as well as the header of each block in the file. So any code that is parsing .blend files has to be updated."

Blender 5.0 release notes, Core — https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30. The two files it points at for details are on Blender's own site and are not in the sources I was given, so I cannot say what the new headers look like.

### What cannot be read out of a file, because it is not in one

The preferences-side settings from question 1 section D — the Cycles compute backend, which devices are enabled, Distribute Memory Across Devices, MetalRT, HIP RT, Embree on GPU, Kernel Optimization, Shader Compilation Method — are not in the scene file. They are in the user's own preferences file, "./config/userpref.blend — User preferences" (Directory Layout page, read 2026-07-30), whose location is per operating system and given on that same page: `$HOME/.config/blender/5.0/` on Linux, `/Users/$USER/Library/Application Support/Blender/5.0/` on macOS, `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.0\` on Windows.

So a scene file read on its own tells you that Cycles is set to "GPU Compute" but not which card that will be, nor whether any card is enabled at all.

Likewise the command-line-only settings — `--qos`, `-t` if it overrides, `--cycles-device` — are by their nature not in any file.

**Not covered.** I have not established that reading a value through Blender always returns what was saved — the Tile Size case above is one counter-example, and there may be others in Blender's own versioning code, which is not in the sources I was given. I have not established what happens when a scene is loaded from a file written by a *newer* Blender. And I have said nothing about reading a file's settings without a working Blender installation, beyond the release note's warning that any such reader has to be updated for 5.0.

**Outside the interface.** Reading through Python is the same whether Blender has a window or not; `bpy.data.libraries.load` is documented with no such condition, and the API records "bpy.app.background — Boolean, True when blender is running without a user interface (started with -b)" (https://docs.blender.org/api/5.0/bpy.app.html, read 2026-07-30) as something a script can simply check. One caveat sits in the arguments page and is worth carrying: "--disable-depsgraph-on-file-load — Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)." That concerns evaluated data, not the settings values asked about here, but it is the one documented difference on file load in background mode.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Operating system.** Reading through Python carries no system qualification. Where the *preferences* file sits does differ by system, and the three paths are quoted above.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/5.0/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

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

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 5.0.1 at `/opt/blender/5.0/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/5.0/` or `/root/render-demands/prefs-card-cuda/5.0/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

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

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/5.0/`.

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

This list has been pruned three times. Everything it held that a rented Linux machine could settle was run on 2026-08-06 and is written into the answers above, under the question each one answers. The two items that were left waiting for a working graphics card were run on 2026-08-07, once that machine's card had been repaired. The one list covering every version, with the full procedure for each test and the results, is `Audits/Blender Research/What a render demands/Hand tests.md`. **A third sitting, on the Creator's own Mac on 2026-08-07, ran every item that was still owed there, including the two that need a window — which no machine on this project but that one has.** Only genuinely open items are left here.

**Still open.**

3. **Is the Threads number really inert unless Threads Mode is Fixed?** Set Threads Mode to Auto-Detect and Threads to 1, render, and watch a process monitor.
   *Negative result:* CPU use is confined to one core — meaning the value applies despite the mode. (What has been shown is only that under Auto-Detect the property reports the machine's processor count; the case of Auto-Detect with a small number written into it was not run.) **Still not run after the Mac sitting of 2026-08-07, and the reason is worth stating: this item's only evidence is how much of the processor the render takes, and the Mac was shared with other topics' work throughout, so no figure of that kind was taken there.**

**Run and answered above:** `-t` against the file's Threads setting; `-E` against the file's engine; whether EEVEE and Workbench honour Threads and the render-time Simplify values; what Tile Size reads as out of a file saved by an older Blender; whether a background render reads the user preferences; **1 (`--cycles-device` against both the preferences and the scene)**, which it overrides in both directions — though the log names no device, so the evidence is the timing and what the process holds on the card; and **2 (processor against card, pixel for pixel)**, which do not match.

**All of that has now been run, on the Creator's own Mac on 2026-08-07, and is recorded above under the question each one answers.** Apple's own graphics render a different picture from the processor, exactly as the NVIDIA card does; `--cycles-device METAL` beats both the file and the saved preferences, in both directions; the three engines really do show different Performance panels; and the Acceleration Structure controls really do change with the device. **Nothing on this subject is still owed on either machine.**

Three tests were deliberately **not** repeated on the Mac, and the reason is written here rather than left as a silence: the ones whose only possible evidence is a time or a memory figure — whether `-t` changes how much of the processor a Cycles render uses, whether the thread count changes anything under EEVEE or Workbench, and whether Persistent Data changes the time or the memory of an animation. The Mac was shared with other topics' work throughout, so a figure taken there would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. What they leave open on macOS is left open and is not guessed at.

---

2026-08-06 — Opus 5 (Claude Code) — What a render demands, Blender 5.0

# What a render demands — Blender 5.0

**Sources consulted, and when:** Blender 5.0 manual (`Blender Sources/manual_5.0`, including one image from `_images/`), Blender 5.0 Python API reference (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own add-on files (`Blender Sources/cycles_5.0`) — all read 2026-07-30. Blender's requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`) — read 2026-08-01.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## A note on how I searched

Everything below rests on searches run across the whole of `manual_5.0` and `api_5.0` at once, not on opening likely files. The searches I ran, and which are referenced by number in the per-question records:

- **S1** — `memory` across the whole manual (110 files matched; I opened the render, preferences, command-line and troubleshooting ones).
- **S2** — `render time|render times|memory usage|renders faster|render faster|slower render|less memory|more memory|performance cost|at the cost of memory` across `manual_5.0/render`, `manual_5.0/scene_layout`, `manual_5.0/editors/preferences`.
- **S3** — `thread|--threads|cycles-device|device|engine|memory|gpu|frame|scene|python-expr` across `manual_5.0/advanced/command_line/arguments.txt`.
- **S4** — `memory limit|limit memory|max memory|memory cap` across the whole manual, plus `memory` across `cycles_5.0/properties.py` and `api_5.0/bpy.types.RenderSettings.txt`.
- **S5** — `use_persistent_data|Persistent Data` across manual, API and release notes.
- **S6** — `bpy.data.scenes` across the whole API reference.
- **S7** — `libraries.load|load(filepath` across the API reference.
- **S8** — `blender --command|-c help|--command` across the whole manual.
- **S9** — `userpref.blend` across the whole manual.
- **S10** — targeted property lookups in `cycles_5.0/properties.py` and `cycles_5.0/ui.py` by property name (`tile_size`, `use_persistent_data`, `threads_mode`, `debug_use_spatial_splits`, `debug_use_hair_bvh`, `debug_use_compact_bvh`, `debug_bvh_time_steps`, `samples`, `time_limit`, `use_adaptive_sampling`, `use_light_tree`, `use_guiding`, `scrambling_distance`, `max_bounces`, `volume_step_rate`, `dicing_rate`, `denoising_use_gpu`, `device`, `compute_device_type`, `peer_memory`).
- **S11** — `cycles-device|cycles_device|argv|print-stats|CYCLES_` across `cycles_5.0/__init__.py`, `engine.py`, `properties.py`.
- **S12** — property-name lookups in `api_5.0/bpy.types.RenderSettings.txt` and `api_5.0/bpy.types.SceneEEVEE.txt`.

Images opened: `manual_5.0/_images/editors_preferences_section_system.webp` (Preferences System section). I also noted, but did not need to open, `manual_5.0/_images/render_output_properties_format_panel.png`; there is **no** image of the Cycles or EEVEE Performance panel in `_images` (searched the image folder for `performance`).

---

## 1. What settings, if any, change how much a render demands of the computer it runs on?

**Answer.** Many. They fall into four groups by where they live: settings that belong to the scene and are shared by every engine; settings that belong to Cycles; settings that belong to EEVEE; and settings that live in Blender's Preferences rather than in the file at all. The manual has panels named "Performance" for exactly this purpose.

The Cycles Performance panel states its own purpose:

> "Properties that affect render speeds or memory consumption. There are several presets available to help choose between different trade offs:"
>
> "Default: Balances memory saving and faster rendering settings."
> "Faster Render: Uses settings to render faster at the cost of higher memory consumption."
> "Lower Memory: Uses settings to decrease memory considered at the cost of slower renders."

— Blender 5.0 manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html (read 2026-07-30)

### Group A — scene settings, not tied to one engine

**Resolution and Percentage**

> "Resolution X, Y — The number of pixels horizontally and vertically in the image."
> "Percentage — Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image."

— https://docs.blender.org/manual/en/5.0/render/output/properties/format.html (read 2026-07-30)

The API gives the limits:

> "resolution_x — Number of horizontal pixels in the rendered image — Type: int in [4, 65536], default 1920"
> "resolution_percentage — Percentage scale for render resolution — Type: int in [1, 32767], default 100"

— https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html (read 2026-07-30)

**Render Region**

> "Render Region — Renders just a portion of the view instead of the entire frame."
> "Crop to Render Region — Crops the rendered image to the size of the render region, instead of rendering a transparent background around it."

— https://docs.blender.org/manual/en/5.0/render/output/properties/format.html (read 2026-07-30)

**Threads Mode and Threads** (shown in the Cycles Performance panel, but stored on the scene's render settings — see Q4)

> "Threads Mode — Method to determine the maximum number of CPU cores to use while rendering."
> "Auto-Detect: Automatically chooses the amount of threads to match the number of logical processors on your computer."
> "Fixed: Manually choose the maximum number threads to use for rendering. This can be useful for example, if you want to use your computer while rendering, you can set the property to a thread count lower the amount of logical processors on your computer."
> "Threads — The maximum number of CPU cores to use simultaneously while rendering."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html (read 2026-07-30)

> "threads — Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) — Type: int in [1, 1024], default 1"
> "threads_mode — Determine the amount of render threads used … Type: enum in ['AUTO', 'FIXED'], default 'AUTO'"

— https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html (read 2026-07-30)

**Persistent Data**

> "Persistent Data — Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage while performing other tasks in Blender."
> "When using multiple View Layers, only data from a single view layer is preserved to keep memory usage within bounds; however, objects shared between view layers are preserved."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html (read 2026-07-30)

**Simplify** (Max Subdivision, Child Particles, Texture Limit, Volume Resolution, Culling)

> "Max Subdivision — Maximum number of subdivision by the Subdivision Surface modifiers."
> "Child Particles — Show only a subset of all child hairs and particles."
> "Texture Limit — Automatically scales textures down so that they are no larger than the values chosen. This can help reduce computer memory resources when rendering large scenes with huge textures."
> "Camera Cull — Automatically culls objects based on the camera frustum defined by the Margin."
> "Distance Cull — Automatically culls objects based on their distance from the active camera."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/simplify.html (read 2026-07-30)

**Lock Interface**

> "Lock Interface — Lock interface during rendering in favor of giving more memory to the renderer."

— https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html (read 2026-07-30)

**High Quality Normals**

> "use_high_quality_normals — Use high quality tangent space at the cost of lower performance — Type: boolean, default False"

— https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html (read 2026-07-30)

**View Layer sample override**

> "Samples — Allows the view layer to use a different number of samples than the main scene. This is controlled by the Layer Samples when using the Cycles renderer. When using EEVEE, the Samples override is always used."
> "Lowering the number of samples for background or utility passes can speed up render times, while still using higher samples for beauty passes."

— https://docs.blender.org/manual/en/5.0/render/layers/override.html (read 2026-07-30)

**Compositor device and denoise-node quality** (documented identically on the Cycles and the EEVEE Performance pages)

> "Device — The device used for compositing. CPU: Use the CPU for compositing. GPU: Use the GPU for compositing."
> "Precision GPU — The precision of compositor intermediate result. Auto: Use full precision for final renders, half precision otherwise."
> "Denoising Device — The device to use to process Denoise nodes in the compositor."
> "Final Quality — The quality used by Denoise nodes during the final render if their quality is set to Follow Scene. High: Produces the highest quality output at the cost of long processing times. Balanced: Balanced between performance and quality, typically processing in half the time as High, while retaining most of the quality. Fast: Produces an output quickly at a noticeable cost of quality."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html and https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/performance.html (read 2026-07-30)

### Group B — Cycles only

**Device**

> "Device — Specifies the compute device used for rendering in Cycles. The choice of device can significantly affect render performance, GPU Compute is often faster ."
> "CPU: Render using the system's central processing unit. This mode is compatible with all Cycles features. The number of CPU threads used can be changed in the Performance Properties."
> "GPU Compute: Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/ (read 2026-07-30)

**observed** — in Cycles' own file:

> ```
>     device: EnumProperty(
>         name="Device",
>         description="Device to use for rendering",
>         items=enum_devices,
>         default='CPU',
>         update=update_render_passes,
>     )
> ```

— `Blender Sources/cycles_5.0/properties.py`, lines 381–387, in class `CyclesRenderSettings` (read 2026-07-30)

**Samples and Time Limit**

> "Render (Max) Samples — The number of paths to trace per pixel in the final render. A higher number results in a cleaner image at the cost of a longer render time."
> "Time Limit — Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit."
> "Note — The time limit does not include pre-render processing time, only render time."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-07-30)

**observed** —

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

— `Blender Sources/cycles_5.0/properties.py`, lines 479–484 (read 2026-07-30)

> ```
>     time_limit: FloatProperty(
>         name="Time Limit",
>         description="Limit the render time (excluding synchronization time). "
>         "Zero disables the limit",
>         min=0.0,
>         default=0.0,
> ```

— `Blender Sources/cycles_5.0/properties.py`, lines 514–519 (read 2026-07-30)

**Adaptive Sampling** (Noise Threshold, Min Samples)

> "If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling, cutting short the sampling process in areas that have become less noisy than the specified threshold value"
> "Noise Threshold — The threshold to decide whether to continue sampling a pixel. Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times."
> "Min Samples — The minimum number of samples a pixel receives before adaptive sampling is applied."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-07-30)

**Denoising** (on/off, denoiser, prefilter, quality, input passes, Use GPU)

> "Denoising uses a specialized algorithm to get a less noisy image without requiring more samples."
> "Quality OpenImageDenoise — Overall denoising quality. … High: Produces the highest quality at the cost of time. … Fast: Sacrifices quality for speed (ideal for viewport rendering)."
> "Use GPU — Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled."
> "Accurate: Prefilters the input passes before denoising to reduce noise. This option usually produces more detailed results than Fast, but with increased processing time."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-07-30)

**Path Guiding**

> "Note — Path guiding is only available when rendering on a CPU."
> "Training Samples — The maximum number of samples to use for training. … Higher values can lead to a minor increases in guiding quality but with increased render times."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-07-30)

**Light Tree and Light Threshold**

> "Light Tree — Use a light tree to more effectively sample lights in the scene, taking into account distance and estimated intensity. This can significantly reduce noise, at the cost of a somewhat longer render time per sample."
> "Light Threshold — Probabilistically terminates light samples when the light contribution is below the threshold. This avoids wasting time on lights that contribute little to the image. Zero disables the test."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-07-30)

**Sampling Pattern, Seed, Scrambling Distance, Min Bounces**

> "Scrambling Distance — A technique that reduces the randomness between pixels in an attempt to improve GPU rendering performance, at the cost of potential artifacts. Not compatible with the Blue-Noise sampling pattern."
> "Multiplier — A multiplier for the scrambling distance. Values below one will reduce the distance, having the potential to improve GPU rendering performance but increase the visibility of artifacts."
> "Min Light Bounces — Minimum number of light bounces for each path, after which the integrator uses Russian Roulette to terminate paths that contribute less to the image. Setting this higher gives less noise, but may also increase render time considerably."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-07-30)

**Sample Subset** (Offset, Length) — the sheet's subject matter touches this directly, because the manual presents it as a way of splitting one frame's work:

> "Sample Subset — Only render a subset of the samples. Multiple subset renders can be combined into a full one by running the following in the Python Console:"
> "bpy.ops.cycles.merge_images(input_filepath1=r"1.exr", input_filepath2=r"2.exr", output_filepath=r"combined.exr")"
> "A typical use case is to distribute the rendering of a single frame over multiple machines."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-07-30)

**Light Paths** (Max Bounces, Clamping, Filter Glossy, Caustics, Fast GI)

> "Total — Maximum number of light bounces. For best quality, this should be set to the maximum. However, in practice, it may be good to set it to lower values for faster rendering. A value of 0 bounces results in direct lighting only."
> "Light paths are terminated probabilistically when specifying a minimum number of light bounces lower than the maximum. … This will still converge to the same image, but renders faster while possibly being noisier."
> "Reflective — … This option can be unchecked, to disable reflective caustics."
> "Fast GI Approximation … This provides fast alternative to full global illumination (GI), for interactive viewport rendering or final renders with reduced quality."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/light_paths.html (read 2026-07-30)

**Volumes** (Biased, Step Rate, Max Steps)

> "Biased — Default volume rendering algorithm is based on null scattering, which is unbiased and has less artifacts, but could be noisier. Biased option uses ray marching, with controls for steps size and max steps."
> "Render time can be reduced by increasing the step size, at the cost of potentially losing some volume detail."
> "Step Rate Render Biased — Global multiplier on the step size for all volumes in renders. Increase to reduce render time, at the cost of losing detail."
> "Max Steps Biased — Maximum number of steps through the volume before giving up, to protect from extremely long render times with big objects or small step sizes."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/volumes.html (read 2026-07-30)

The release notes record that this is a 5.0 change:

> "A new volume rendering algorithm based on null scattering has been added as the default. It is unbiased, removes blocky artifacts in overlapping volumes, and eliminates the need for step size, max steps, and homogeneous volume settings."
> "For more control over rendering quality, the previous biased ray marching algorithm can be enabled with the `Render -> Volume -> Biased` option; this also restores the previous step size and max steps options."
> "Smoke and fire simulations are now rendered with NanoVDB. This reduces memory usage at the cost of some performance."

— https://developer.blender.org/docs/release_notes/5.0/cycles/ (read 2026-07-30)

**Subdivision** (Dicing Rate, Offscreen Scale, Max Subdivisions)

> "Offscreen Scale — Multiplier for dicing rate of geometry outside of the camera view. … Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/subdivision.html (read 2026-07-30)

**Curves shape**

> "Linear 3D Curves: Render curves as 3D circular geometry with linear interpolation between control points. This is faster than full 3D curves but less smooth, and may show visible corners at low resolutions."
> "This is an optimization to take advantage of hardware accelerated curve intersection on GPUs. Results will not look exactly the same on different CPU and GPU devices, so this is not suitable for multi device rendering or render farms with mixed hardware."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/hair.html (read 2026-07-30)

**Tile Size**

> "Tile Size — Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory."
> "Note — In some cases changing the Tile Size can result in increased performance. For example when a small object renders slowly compared to other objects, using a small Tiles Size can lead to an increase in performance."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html (read 2026-07-30)

**observed** — exact kind, default and range:

> ```
>     tile_size: IntProperty(
>         name="Tile Size",
>         default=2048,
>         description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
>         min=8,
>         max=8192,
>     )
> ```

— `Blender Sources/cycles_5.0/properties.py`, lines 1056–1062 (read 2026-07-30)

Also observed, immediately above it, the deprecated companion:

> ```
>     use_auto_tile: BoolProperty(
>         name="Auto Tile",
>         description="Deprecated, tiling is always enabled",
>         default=True,
>     )
> ```

— `Blender Sources/cycles_5.0/properties.py`, lines 1051–1055 (read 2026-07-30)

The release notes state the same change:

> "Remove Use Tiling option, it is now always assumed to be on. The tile size can be increased to disabled tiling."

— https://developer.blender.org/docs/release_notes/5.0/cycles/ (read 2026-07-30)

**Acceleration Structure** (four settings)

> "Use Spatial Splits — Spatial splits improve the rendering performance in scenes with a mix of large and small polygons. The downsides are longer BVH build times and slightly increased memory usage."
> "Use Curves BVH — Use a special type of BVH for rendering curves. … Disabling this option will reduce memory, at the cost of increasing curve render time."
> "BVH Time Steps — Split BVH primitives by this number of time steps to speed up render time at the expense of memory."
> "Use Compact BVH — Use a more compact BVH structure, which can reduce RAM usage but render slower."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html (read 2026-07-30)

**observed** — the exact names, kinds, defaults and ranges:

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

— `Blender Sources/cycles_5.0/properties.py`, lines 920–940 (read 2026-07-30)

**Open Shading Language**

> "OSL is supported only when rendering with the CPU or with the OptiX GPU Compute backend."
> "Rendering performance is typically slower than with built-in nodes due to shader compilation and interpretation."

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/ (read 2026-07-30)

### Group C — EEVEE only

> "Samples — The number of samples to use in the final render." (Render)
> "Rays — Number of rays to trace for each light. Higher values reduces the noise caused by random shadow sampling."
> "Steps — Number of shadow map sample per shadow ray. Higher step count results in softer shadows but have a higher cost."
> "Volumetric Shadows — … This is a very computationally expensive option and has limitations."
> "Resolution — Resolution percentage of shadow maps."
> "Light Threshold — Minimum light intensity for a light to contribute to the lighting. … Lower values improve performance."

— https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html (read 2026-07-30)

> "Shadow Pool — A bigger pool size allows for more shadows in the scene but might not fit into GPU memory and decreases performance."
> "Light Probes Volume Pool — A bigger pool size allows for more irradiance grids in the scene but might not fit into GPU memory and decreases performance."
> "High Quality Normals — Uses higher precision normals and tangents which can improve visual quality for dense meshes with high frequency textures at the cost of memory."

— https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/performance.html (read 2026-07-30)

> "Resolution — Resolution at which the ray-tracing is performed. Lower options will be faster and use less memory but will produce blurrier results."
> "Precision — Higher values increase precision of the screen space ray-tracing but lower the maximum trace distance. Increased precision also increases performance cost."
> "Light Probe: Use light-probe spheres and planes to find scene intersection. This option has the lowest tracing cost but relies on manually placed light-probes."

— https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/raytracing.html (read 2026-07-30)

> "To achieve this, EEVEE uses several 3D textures which have a high video memory usage. The texture dimensions can be tweaked using the Resolution and Steps parameters."
> "Steps — Number of steps to compute volumetric effects. Higher count increases video memory usage and quality."

— https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/volumes.html (read 2026-07-30)

The API gives the exact kinds and defaults:

> "shadow_pool_size — Size of the shadow pool, a bigger pool size allows for more shadows in the scene but might not fit into GPU memory — Type: enum in ['16', '32', '64', '128', '256', '512', '1024'], default '512'"
> "taa_render_samples — Number of samples per pixel for rendering — Type: int in [1, inf], default 64"
> "volumetric_samples — Number of steps to compute volumetric effects. Higher step count increase VRAM usage and quality. — Type: int in [1, 256], default 64"
> "shadow_ray_count — Amount of shadow ray to trace for each light — Type: int in [1, 4], default 1"
> "shadow_step_count — Amount of shadow map sample per shadow ray — Type: int in [1, 16], default 6"

— https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html (read 2026-07-30)

### Group D — Preferences, not the file

These are set in Preferences, and the screenshot in the manual shows them sitting there under "Cycles Render Devices":

> "Cycles Render Devices — Changes the Computing Device the Cycles render engine uses to render images."
> "Distribute Memory Across Devices — Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes."
> "HIP RT — Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above."
> "MetalRT — MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases."
> "Embree on GPU — Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance."

— https://docs.blender.org/manual/en/5.0/editors/preferences/system.html (read 2026-07-30)

**documented (from a picture)** — the image `_images/editors_preferences_section_system.webp`, shown on that page as "Preferences System section", shows a "Cycles Render Devices" panel at the top of the System section with a row of buttons reading "None", "CUDA", "HIP", with "None" selected. Below it are "Display Graphics" with a "Backend" dropdown reading "OpenGL", and a "Memory & Limits" panel with "Undo Steps 32", "Undo Memory Limit 0", "Global Undo" ticked, "Console Scrollback Lines 256", "Texture Time Out 120", "Garbage Collection Rate 60". What is visible is visible; I draw nothing further from it.

**observed** — Cycles' own file confirms these live in preferences rather than the scene:

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
>
>
> class CyclesPreferences(bpy.types.AddonPreferences):
> ```

— `Blender Sources/cycles_5.0/properties.py`, lines 1558–1569 (read 2026-07-30)

**LABEL: documented** (for the group A/B/C/D lists above, each with its own quote), with the parts drawn from `cycles_5.0/properties.py` and `ui.py` labelled **observed**, and the panel screenshot labelled **documented**.

**LABEL: inferred** for a smaller set inside those lists. For these the quote says what the setting is and states no cost in time or memory at all, so placing them on a list of things that change what a render demands rests on a step of my own rather than on the words quoted: Resolution X/Y, Percentage, Render Region and Crop to Render Region, Simplify's Max Subdivision and Child Particles, EEVEE's Render Samples and its shadow Resolution, and Cycles' Sample Subset.

**Not covered.** This settles what exists and what the manual says each is for. It does not settle by how much any of them changes demand, in seconds or in bytes, on any given scene or machine — the sources give no numbers of that kind. It also does not settle whether the list is exhaustive: I searched for the language the manual uses about cost ("render time", "memory usage", "faster", "less memory", "performance cost"), so a setting that changes demand without the manual saying so would not appear here. Nothing in the sources places a **ceiling** on how much memory a render may use — search S4 found only "Undo Memory Limit" (Preferences, undo), the Video Sequencer "Memory Cache Limit" (Preferences, sequencer/clip cache) and the animation player's `-c <cache_memory>` flag, none of which are a render memory cap. There is no per-render memory limit in the sources.

**Outside the interface.** Every setting above except the ones explicitly labelled "Viewport" is a final-render setting, and from that I conclude it applies when Blender is driven from outside. **LABEL: inferred** for that conclusion — no source joins the two, and it is the same step Q5 labels inferred for reading a value in a background run. The viewport-only ones — Viewport (Max) Samples, Pixel Size, viewport denoising, Viewport Dicing Rate, Viewport BVH Type, the Simplify "Viewport" column — describe the 3D Viewport in Rendered shading mode, which the sources do not describe as existing in background mode. One setting is documented as behaving differently: `use_single_layer` — "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line." (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30). "Lock Interface" concerns an interface that a background run does not have; the sources do not say what it does in background mode. The Preferences-based settings (Group D) are read from the user's preferences file, which the sources do not describe as being affected by whether Blender is driven from outside.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

**Operating system.** Not uniformly. The device-related ones are constrained by platform: "CUDA is supported on Windows and Linux"; "OptiX is supported on Windows and Linux"; "HIP is supported on Windows and Linux"; "oneAPI is a computation library that is supported on Windows and Linux"; "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." (https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html, read 2026-07-30). The Preferences "Shader Compilation Method" and its thread count are stated as "This option is not available on macOS and requires using OpenGL backend." (https://docs.blender.org/manual/en/5.0/editors/preferences/system.html, read 2026-07-30). The requirements page, where it names this version in so many words, says: "Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." (https://www.blender.org/download/requirements/, read 2026-08-01). For the rest — samples, tile size, bounces, resolution, threads, persistent data — the sources state no operating-system difference.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

**Where I looked.** S1, S2, S4, S10, S12; plus the whole of `manual_5.0/render/cycles/render_settings/` (all 12 pages), `manual_5.0/render/eevee/render_settings/` (all 11 pages), `manual_5.0/render/workbench/performance.txt`, `manual_5.0/render/output/properties/format.txt`, `manual_5.0/render/cycles/gpu_rendering.txt`, `manual_5.0/render/cycles/optimizations/index.txt`, `manual_5.0/render/layers/override.txt`, `manual_5.0/editors/preferences/system.txt`, `manual_5.0/editors/preferences/introduction.txt`, `manual_5.0/advanced/blender_directory_layout.txt`, `api_5.0/bpy.types.RenderSettings.txt`, `api_5.0/bpy.types.SceneEEVEE.txt`, `cycles_5.0/properties.py`, `cycles_5.0/ui.py`, `cycles_5.0/engine.py`, and all five relevant 5.0 release-note pages (cycles, eevee, rendering, core, python_api). I ruled out the Freestyle, Grease Pencil, physics and video-editing sections as being about other subjects; I ruled out `manual_5.0/render/workbench/` beyond its Performance page for the same reason — Workbench's entire Performance page contains only High Quality Normals.

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

**Answer.** The sources split cleanly into three kinds, and the honest answer differs by kind. A fourth group, (d), holds the settings this answer never reached at all.

### (a) Settings the sources tie to what the render produces

For most of these the quote also says how. For three it does not, and "and how" would overstate them: **Light Threshold**, **Max Steps (Biased)**, and the two EEVEE pools, whose quotes speak of the render's working or of an error message rather than of the picture.

- **Render (Max) Samples** — "A higher number results in a cleaner image at the cost of a longer render time." (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html)
- **Time Limit** — "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet." (same page). That the image is therefore whatever had accumulated when time ran out is a step of mine and not the source's — **LABEL: inferred** for that sentence.
- **Noise Threshold** — "Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times." (same page)
- **Resolution / Percentage** — "Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image." (https://docs.blender.org/manual/en/5.0/render/output/properties/format.html) The output is a different pixel size.
- **Render Region / Crop** — "Renders just a portion of the view instead of the entire frame." and "Crops the rendered image to the size of the render region, instead of rendering a transparent background around it." (same page)
- **Scrambling Distance** — "at the cost of potential artifacts"; "Values below one … increase the visibility of artifacts." (sampling page)
- **Denoiser quality / prefilter** — "Fast: Sacrifices quality for speed"; "None: … assumes the input passes are noise free which may require a high sample count. If the input passes aren't noise free, then noise will remain in the image after denoising." (sampling page)
- **Denoiser input Passes** — "Generally, the more passes the denoiser has access to, the better the result. It is recommended to use at least Albedo as None can blur out details, especially at lower sample counts." (sampling page)
- **Max Bounces** — "A value of 0 bounces results in direct lighting only." (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/light_paths.html)
- **Min Light Bounces / probabilistic termination** — "This will still converge to the same image, but renders faster while possibly being noisier." (same page) This is the one case where the manual states outright that the converged image is unchanged and only the noise differs.
- **Clamping** — "It reduces noise at the cost of accuracy. … as you clamp out such values, other bright lights/reflections will be dimmed as well." (same page)
- **Filter Glossy** — "this will blur glossy reflections after blurry bounces, to reduce noise at the cost of accuracy." (same page)
- **Caustics Reflective / Refractive** — "This option can be unchecked, to disable reflective caustics." (same page) The caustics are removed from the image.
- **Light Tree** — "This can significantly reduce noise, at the cost of a somewhat longer render time per sample." and "This can result in an increase in noise in some scenes that make use of these features." (sampling page)
- **Light Threshold** — "Probabilistically terminates light samples when the light contribution is below the threshold." (sampling page)
- **Volume Step Rate (Biased)** — "Increase to reduce render time, at the cost of losing detail." (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/volumes.html)
- **Max Steps (Biased)** — "Maximum number of steps through the volume before giving up" (same page)
- **Biased vs unbiased volumes** — "Default volume rendering algorithm is based on null scattering, which is unbiased and has less artifacts, but could be noisier." (same page); and from the release notes, "The performance and noise level can be different than before and is scene-dependent." (https://developer.blender.org/docs/release_notes/5.0/cycles/)
- **Subdivision Offscreen Scale** — "Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory." (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/subdivision.html)
- **Curves shape: Linear 3D Curves** — "This is faster than full 3D curves but less smooth, and may show visible corners at low resolutions." and "Results will not look exactly the same on different CPU and GPU devices, so this is not suitable for multi device rendering or render farms with mixed hardware." (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/hair.html)
- **Simplify Texture Limit** — "Automatically scales textures down so that they are no larger than the values chosen." (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/simplify.html)
- **Simplify Culling** — "Automatically culls objects based on the camera frustum" / "based on their distance from the active camera." (same page)
- **View Layer Samples override** — "Lowering the number of samples for background or utility passes can speed up render times, while still using higher samples for beauty passes." (https://docs.blender.org/manual/en/5.0/render/layers/override.html)
- **Sample Subset** — "While this overrides (Max) Samples in terms of the samples that will get rendered, it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets – otherwise, the subsets will have incompatible noise and combining them will give a worse result." (sampling page)
- **High Quality Normals** — "Uses higher precision normals and tangents which can improve visual quality for dense meshes with high frequency textures at the cost of memory." (https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/performance.html)
- **EEVEE Shadow Rays/Steps** — "Higher values reduces the noise caused by random shadow sampling." / "Higher step count results in softer shadows but have a higher cost." (https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html)
- **EEVEE Shadow Pool / Light Probes Volume Pool** — "Increasing the size might fix the Shadow buffer full error." (EEVEE performance page)
- **EEVEE Ray-tracing Resolution and Fast GI Resolution** — "Lower options will be faster and use less memory but will produce blurrier results." (https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/raytracing.html)
- **EEVEE Volumes Resolution / Steps** — "Higher count increases video memory usage and quality." (https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/volumes.html)
- **Compositor Precision GPU** — "Auto: Use full precision for final renders, half precision otherwise." (Cycles/EEVEE performance pages)
- **Compositor Denoise Nodes Final Quality** — "Fast: Produces an output quickly at a noticeable cost of quality." (same)
- **Open Shading Language** — "Some features, such as baking and certain volume shaders, may not work with OSL." (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/)

All read 2026-07-30.

### (b) Settings the sources describe purely as a speed/memory trade, saying nothing about the image

For these the manual and Cycles' own file speak only of time and memory, never of the result:

- **Threads Mode / Threads** — "The maximum number of CPU cores to use simultaneously while rendering." Nothing about the image.
- **Tile Size** — "Render high resolution images in tiles of this size, to reduce memory usage." Nothing about the image.
- **Persistent Data** — "Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage". Nothing about the image.
- **Use Spatial Splits** — "longer builder time, faster render" (`cycles_5.0/properties.py` line 922).
- **Use Curves BVH** — "uses more ram but renders faster" (line 927).
- **Use Compact BVH** — "uses less ram but renders slower" (line 932).
- **BVH Time Steps** — "speed up render time in cost of memory" (line 937).
- **Lock Interface** — "in favor of giving more memory to the renderer".
- **Denoise Use GPU** — "significantly faster than on CPU, but requires additional GPU memory".
- **Distribute Memory Across Devices** — "effectively freeing up space for larger scenes … at a small performance cost".

All read 2026-07-30.

**LABEL for group (b): not in the sources.** The sources do not say whether these change the image. It would be a step of my own to conclude "therefore the image is identical", and I am not taking it — a setting can be silent in the documentation and still change the result.

### (c) Device, where the sources are more careful than a simple yes or no

The Device choice (CPU vs GPU Compute) is described as a performance choice — "The choice of device can significantly affect render performance, GPU Compute is often faster" — but the same page adds "Note — GPU rendering may have limitations compared to CPU rendering, depending on the backend.", and lists two: "Path Guiding is not supported on any GPU." and "Open Shading Language is only supported for OptiX". The Curves page states outright that one shape option gives different results across devices. So the sources do not present device as image-neutral.

### (d) Settings from Q1 that neither list above reaches

The question asks about each of the settings in Q1, and this answer is short of that by exactly the following, which appear in no entry above: Adaptive Sampling's Min Samples; Path Guiding and its Training Samples; Sampling Pattern and Seed; Cycles' own Fast GI Approximation; Dicing Rate and Max Subdivisions; Simplify's Max Subdivision, Child Particles and Volume Resolution; the Compositor's Device and its Denoising Device; EEVEE's Render Samples, shadow Resolution, Volumetric Shadows, Light Threshold, ray-tracing Precision, ray-tracing method and ray-tracing denoising; and the Preferences settings HIP RT, MetalRT and Embree on GPU.

**LABEL: none. This is a gap, not an answer.** It has been sent back to a fresh researcher rather than filled in here.

**LABEL: documented** for (a) and (c); **not in the sources** for (b).

**Not covered.** Even in group (a), the sources describe direction ("cleaner", "blurrier", "noisier"), never magnitude. Nothing here settles whether two renders made with the same settings on the same machine are bit-for-bit identical, nor whether two machines with different thread counts produce identical pixels — the sources are silent on determinism, and I searched for it (S2, plus reading the whole sampling, performance and light-paths pages).

**Outside the interface.** The output effects described are properties of the render itself and the sources do not tie them to the interface. **LABEL: inferred** for that first sentence — what it rests on is the sources' silence, not a statement in them. The exception is the Simplify "Viewport" column and the viewport-prefixed settings, which the manual scopes to the 3D Viewport.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Operating system.** The sources state no operating-system difference in what any of these settings does to the image, with the device-availability exception noted in Q1.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Where I looked.** The same pages as Q1, read in full rather than searched, since this question is about what each page says a setting does. S2 was the search used to be sure I had not missed a page that states an output consequence.

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

**Answer.** A small, fixed set has its own command-line switch. Everything else has to be set some other way — either by editing and saving the file, or by running a Python script or expression at start-up, which the command line supports.

### Settled directly by a command-line switch

> "-E, --engine <engine> — Specify the render engine. Use -E help to list available engines."
> "-t, --threads <threads> — Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count."
> "--cycles-device <device> — Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. Append +CPU to a GPU device to render on both CPU and GPU."
> "Example: blender -b file.blend -f 20 -- --cycles-device OPTIX"
> "--cycles-print-stats — Log statistics about render memory and time usage."
> "--gpu-backend — Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."
> "--qos <level> — Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only). default: Uses the default behavior of the OS. high: Always makes use of performance cores. eco: Schedules Blender threads exclusively to efficiency cores."
> "-S, --scene <name> — Set the active scene <name> for rendering."

— Blender 5.0 manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (read 2026-07-30)

Note the placement rule for the Cycles ones:

> "Cycles Render Options — Cycles add-on options must be specified following a double dash."

— same page (read 2026-07-30)

**observed** — Cycles' own file confirms both the double-dash rule and, importantly, what `--cycles-device` overrides:

> ```
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)
> ```

and

> ```
>     argv = sys.argv
>     if "--" not in argv:
>         return
>
>     parser = _configure_argument_parser()
>     args, _ = parser.parse_known_args(argv[argv.index("--") + 1:])
> ```

— `Blender Sources/cycles_5.0/engine.py`, lines 16–20 and 27–32 (read 2026-07-30)

That help text — "overriding user preferences and the scene setting" — is the clearest statement in any of my sources that a command-line switch beats what the file says. What is **observed** is that those words sit in that file. That the switch really does beat the file when a render runs is a step past reading a description of it, so **LABEL: inferred** for the behaviour. Hand test 3 below is aimed at exactly this.

### Settled indirectly, at start-up, by running Python

The command line can run a script or an expression:

> "-P, --python <filepath> — Run the given Python script file."
> "--python-expr <expression> — Run the given expression as a Python script."

— https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (read 2026-07-30)

and Blender's settings are reachable as ordinary Python attributes:

> ">>> bpy.data.scenes[0].render.resolution_percentage
> 100"

— Blender 5.0 Python API, https://docs.blender.org/api/5.0/info_quickstart.html (read 2026-07-30)

**LABEL: inferred** for this part. The step I am taking: the manual documents a switch that runs an arbitrary Python expression at start-up; the API documents that render settings are reachable and readable through `bpy.data.scenes[...].render`; the API also lists these properties without a "(readonly)" marker, unlike properties it does mark that way (for example "stereo_views — … bpy_prop_collection of SceneRenderView, (readonly)"). From those three facts together I conclude that any of the settings in Q1 that lives on the scene can be set at start-up via `--python-expr` or `-P`. No single source says this in so many words, and a reader has to supply the joining step, so this is inferred, not documented.

### Ordering matters, and the manual is explicit about it

> "Arguments are executed in the order they are given. eg:"
> "blender --background test.blend --render-frame 1 --render-output "/tmp""
> "…will not render to /tmp because --render-frame 1 renders before the output path is set."
> "blender --background --render-output /tmp test.blend --render-frame 1"
> "…will not render to /tmp because loading the blend-file overwrites the render output that was set."
> "blender --background test.blend --render-output /tmp --render-frame 1"
> "…works as expected."

— https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (read 2026-07-30)

**LABEL: inferred** for the consequence: anything set before the blend-file is loaded can be undone by loading it, so a Python expression that changes a scene setting has to come after the file and before the render. The manual demonstrates this for the output path specifically; extending it to other scene settings is my step.

### Settled only by changing the file (or by the Python route above)

Everything else in Q1 has no switch of its own. Reading the whole of the command-line page — all 662 lines, in three passes covering every section — turned up no switch for: samples, tile size, persistent data, adaptive sampling and noise threshold, denoising and its quality/prefilter/passes, time limit, light tree, light threshold, sampling pattern, scrambling distance, bounce limits, clamping, filter glossy, caustics, fast GI, volume step rate and max steps, dicing rate, curves shape, all four acceleration-structure settings, simplify and its texture limit, resolution and percentage, render region, lock interface, high quality normals, and every EEVEE setting including the pools.

### A third category the question does not name: settings that live in neither place

The Cycles compute device *type* and the per-device on/off switches are Preferences, and Preferences are stored in their own file:

> "./config/userpref.blend — User preferences."

— https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html (read 2026-07-30)

So "changing the file" would not reach them; changing preferences would, or `--cycles-device` at start-up, which overrides them.

**LABEL: documented** for the switch list and the ordering rule; **observed** for the words `--cycles-device` carries in Cycles' own file, **inferred** for what it actually overrides when a render runs; **inferred** for the Python route and its ordering consequence.

**Not covered.** The sources do not say how `-t/--threads` interacts with a file that has Threads Mode set to Fixed with its own thread count — which wins, or whether `-t` changes `threads_mode`. I searched the command-line page, the Cycles performance page, `api_5.0/bpy.types.RenderSettings.txt` and `cycles_5.0/properties.py` and `ui.py` for any statement of precedence and found none. Nor do the sources say whether `--gpu-backend` or `--qos` affect a Cycles render's speed, only what they set. Nor is there any switch documented for setting resolution, samples or tile size directly. There is also no documented switch to cap memory (see Q1, S4).

**Outside the interface.** This whole question is about outside the interface, so the answer is native to it. The one place the sources note a difference in the other direction is `use_single_layer` — "Only affects rendering from the interface, ignored for rendering from command line." (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30). Also relevant: "-b, --background — Run in background (often used for UI-less rendering). The audio device is disabled in background-mode by default" (command-line page, read 2026-07-30).

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Operating system.** One switch is platform-limited in so many words: "--qos <level> — Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only)." Also "-con, --start-console — Start with the console window open (ignored if -b is set), (Windows only)." The `--gpu-backend` options include `metal`, which by the requirements and GPU pages means Apple hardware. The temporary-files environment variables differ by platform: "TEMP: Store temporary files here (MS-Windows)." / "TMPDIR: Store temporary files here (UNIX Systems)." For `-t`, `-E`, `--cycles-device` and the Python switches, the sources state no operating-system difference — except that the valid `--cycles-device` values are themselves constrained by which backends run on which platform (Q1, operating-system line).

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Where I looked.** S3 and S11; the whole of `manual_5.0/advanced/command_line/arguments.txt` (662 lines, read in three passes covering every section: Render, Cycles Render, Format, Animation Playback, Window, Python, Network, Logging, Debug, GPU, Misc, Other, Argument Parsing, Environment Variables); `manual_5.0/advanced/command_line/extension_arguments.txt` (ruled out — it documents `blender --command extension …` for managing add-on packages, nothing about render demand); `cycles_5.0/engine.py` in full; `api_5.0/info_quickstart.txt`; `manual_5.0/advanced/blender_directory_layout.txt`. S8 was run to check whether any `--command` subcommand touches render settings; the only documented subcommands are the extension ones. One caution about that record: S3's terms are `thread|--threads|cycles-device|device|engine|memory|gpu|frame|scene|python-expr`, which do not include the names of the settings listed above as having no switch. That absence therefore rests on the full read of the page, not on S3.

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

**Answer.** Yes, and the split is three ways.

**Belongs to Cycles.** Device (CPU / GPU Compute), samples, time limit, adaptive sampling and noise threshold, all the denoising settings, path guiding, light tree and light threshold, sampling pattern, seed, scrambling distance, min light/transparent bounces, sample subset, all the bounce and clamping and caustics settings, fast GI approximation, the volume settings, adaptive subdivision dicing, curves shape, **tile size**, and all four acceleration-structure settings. These are defined in the Cycles add-on's own file and attached to the scene by the add-on:

**observed** —

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

— `Blender Sources/cycles_5.0/properties.py`, lines 1101–1107, in class `CyclesRenderSettings` (read 2026-07-30)

Corroborating this: the Blender 5.0 Python API reference has **no** page for Cycles' render settings at all.

**observed** — what was done: listed the file names in the supplied copy `Blender Sources/api_5.0` and kept those containing "cycles". What came back: `bpy.ops.cycles.txt` and `bpy.types.FModifierCycles.txt`, and nothing else (2026-07-30). Neither is the render-settings type. This is a listing of a supplied copy rather than a test on a running Blender, so no machine bears on it. The Cycles settings are not part of Blender's own documented types.

**Belongs to EEVEE.** Render Samples, shadow Rays / Steps / Resolution / Volumetric Shadows, the ray-tracing method, resolution, precision and denoising, Fast GI Approximation and its resolution/rays/steps/precision (that name appears in the Cycles list above as well; the sources quoted here do not say whether the two are one setting or two settings sharing a name), volumes resolution and steps, Shadow Pool, Light Probes Volume Pool, and EEVEE's Light Threshold. These live on their own type:

> "shadow_pool_size … taa_render_samples … volumetric_samples …"

— https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html (read 2026-07-30)

Note that EEVEE's engine identifier changed in this version:

> "EEVEE's engine identifier was changed from `BLENDER_EEVEE_NEXT` to `BLENDER_EEVEE`."

— https://developer.blender.org/docs/release_notes/5.0/eevee/ (read 2026-07-30)

**Belongs to no single engine.** Resolution X/Y and Percentage, Render Region and Crop, Threads Mode and Threads, Persistent Data, Simplify and its Texture Limit and Culling, Lock Interface, High Quality Normals, and the choice of engine itself. These sit on Blender's own render-settings type, which is not engine-specific:

> "threads … threads_mode … use_persistent_data … use_simplify … resolution_x … resolution_percentage … use_high_quality_normals … engine"

— https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html (read 2026-07-30)

There is a wrinkle worth naming. Three of these — Threads, Persistent Data and Pixel Size — are documented **only** on the Cycles Performance page in the manual, even though they belong to Blender's own type. Cycles' interface file shows it drawing them from Blender's render data (`rd`), not its own (`cscene`):

**observed** —

> ```
>         col.prop(rd, "threads_mode")
> ```
> ```
>         col.prop(rd, "use_persistent_data", text="Persistent Data")
> ```
> ```
>         col.prop(cscene, "tile_size")
> ```

— `Blender Sources/cycles_5.0/ui.py`, lines 860, 944 and 878 respectively, inside `CYCLES_RENDER_PT_performance_threads`, `CYCLES_RENDER_PT_performance_final_render` and `CYCLES_RENDER_PT_performance_memory` (read 2026-07-30)

So Threads and Persistent Data are Blender's, drawn inside a Cycles panel; Tile Size is Cycles' own. **LABEL for that distinction: observed.**

**Belongs to Workbench.** Only one: "High Quality Normals". Workbench's entire Performance page contains that setting and nothing else (https://docs.blender.org/manual/en/5.0/render/workbench/performance.html, read 2026-07-30) — and that setting is on Blender's own type, so it is shared with EEVEE rather than exclusive.

**Belongs to Preferences, across engines but tied to Cycles' use of hardware.** Cycles Render Devices, Distribute Memory Across Devices, Embree on GPU, HIP RT, MetalRT — see Q1 Group D.

**LABEL: documented** (from the manual pages and API type pages), with the placement facts from `cycles_5.0/properties.py` and `ui.py` labelled **observed**.

**Not covered.** This settles which engine each setting belongs to among the three engines the 5.0 manual documents (Cycles, EEVEE, Workbench). It does not settle what a third-party engine does, since the manual documents none. It also does not settle what happens to a Cycles-only value when the file's engine is EEVEE — whether it is retained, ignored, or discarded; the sources do not say, and I searched (S5, S10, S12).

**Outside the interface.** The ownership of a setting is a fact about where it is stored, and the sources give no reason for it to differ outside the interface. **LABEL: inferred** for that — no source says it, and the silence is what it rests on. The panels named above are interface objects, but the underlying types (`bpy.types.RenderSettings`, `bpy.types.SceneEEVEE`, `Scene.cycles`) are not.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Operating system.** The sources state no operating-system difference in which engine owns which setting. Which *values* a Cycles device setting may take does differ by platform (Q1).

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Where I looked.** S10, S12; directory listing of `api_5.0` filtered for "cycles"; `cycles_5.0/properties.py` (registration blocks at lines 1101–1111, 1468–1476, 1547–1555); `cycles_5.0/ui.py` (panel classes at lines 824–961); `api_5.0/bpy.types.RenderSettings.txt` and `api_5.0/bpy.types.SceneEEVEE.txt` in full for the property lists; the Cycles, EEVEE and Workbench manual sections.

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

**Answer.** Yes, for everything stored in the file, by opening the file in Blender and reading the property through Python. No, for the ones stored in Preferences, because they are not in the file at all. And the sources do not describe any way to read them without opening the file in Blender.

### Reading them, once the file is open

The API reference shows exactly this, reading a render setting and getting its value back:

> ">>> bpy.data.scenes[0].render.resolution_percentage
> 100"

and, immediately before it:

> "Once you have a data-block, such as a material, object, collection, etc., its attributes can be accessed much like you would change a setting using the graphical interface. In fact, the tooltip for each button also displays the Python attribute which can help in finding what settings to change in a script."

— Blender 5.0 Python API, https://docs.blender.org/api/5.0/info_quickstart.html (read 2026-07-30)

**LABEL: documented** for `render.resolution_percentage` specifically — the source shows the read and the value returned.

**LABEL: inferred** for the rest. The step: the API reference lists every setting from Q1 groups A and C as a property of `bpy.types.RenderSettings` or `bpy.types.SceneEEVEE` with a type and a default (for example "use_persistent_data — … Type: boolean, default False"), and marks properties that cannot be written or are collections with "(readonly)" where that applies; the quickstart shows that such properties are read by ordinary attribute access. From that I conclude the same access works for `render.threads`, `render.use_persistent_data`, `render.resolution_x`, `eevee.taa_render_samples` and so on. No source states this for each property individually.

For the Cycles ones, the same route applies through the pointer the add-on registers:

**observed** — `bpy.types.Scene.cycles` is registered as a pointer to `CyclesRenderSettings` (`Blender Sources/cycles_5.0/properties.py`, lines 1101–1107, quoted in Q4), and that class holds `device`, `samples`, `tile_size`, `debug_use_compact_bvh` and the rest, each with a declared kind, default and range as quoted in Q1. **LABEL: inferred** for the conclusion that `scene.cycles.tile_size` is therefore readable — the file shows the property being declared and attached, not a read of it being performed.

The manual itself relies on this route in one place, showing a Cycles operator being driven from the Python Console:

> "bpy.ops.cycles.merge_images(input_filepath1=r"1.exr", input_filepath2=r"2.exr", output_filepath=r"combined.exr")"

— https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-07-30)

### Reading them without opening the file

The one API facility for reaching into a blend-file from outside is:

> "load(filepath, *, link=False, relative=False, …) — Returns a context manager which exposes 2 library objects on entering. Each object has attributes matching bpy.data which are lists of strings to be linked."

and the worked example:

> "# Load a single scene we know the name of.
> with bpy.data.libraries.load(filepath) as (data_src, data_dst):
> data_dst.scenes = ["Scene"]"

— https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html (read 2026-07-30)

**LABEL: inferred.** The step: the source says what `data_src` exposes is "lists of strings", that is, names — not the settings on those data-blocks. To reach a scene's render settings you must actually link or append the scene, at which point it is in `bpy.data` and read as above. So there is a documented way to list what is in a file without fully loading it, but not to read a render setting's value that way.

The API says as much itself, in the page on using Blender as a Python module:

> "In some cases the library API may be an alternative to starting separate processes, although this API operates on reading and writing ID data-blocks and isn't a complete substitute for loading .blend files, see:"
> "bpy.types.BlendDataLibraries.load()"

and, on the same page:

> "Single Blend File Restriction — Only a single .blend file can be edited at a time."

— https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html (read 2026-07-30)

**LABEL: documented** for those two statements — that the library API works on ID data-blocks and is not a substitute for loading a file, and that only one blend-file is open at a time.

### The ones that are not in the file

The Cycles compute device type, the per-device on/off switches, Distribute Memory Across Devices, MetalRT, HIP RT and Embree on GPU are preferences:

**observed** — "# Properties saved in preferences" immediately above `id`/`name`/`use`/`type` in `CyclesDeviceSettings`, and `class CyclesPreferences(bpy.types.AddonPreferences)` — `Blender Sources/cycles_5.0/properties.py`, lines 1558–1569 (quoted in full in Q1) (read 2026-07-30)

and preferences live in their own file:

> "./config/userpref.blend — User preferences."

— https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html (read 2026-07-30)

**LABEL: documented** that these are preferences and that preferences are stored in `userpref.blend`; **inferred** that they therefore cannot be read out of a scene's blend-file.

**Not covered.** This settles that the values are reachable through Blender's Python. It does not settle whether they can be read by any tool other than Blender, nor whether a blend-file's bytes can be parsed directly — the sources say nothing about the blend-file's internal layout, and I searched (S6, S7, S9, plus `blend|file format` across the API's `info_*` pages). It also does not settle what a read returns for a Cycles property when the Cycles add-on is not enabled — the properties are registered by the add-on, so the question is real, but the sources do not answer it. And it does not settle whether reading is possible without Blender launching a full session; `-b`/`--background` is documented as a run mode, not as a read mode.

**Outside the interface.** This is the case that matters here and it holds: the quickstart's read is shown in the Python Console, but `-P, --python <filepath>` and `--python-expr <expression>` are documented command-line switches (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30), and `-b, --background` is documented as "Run in background (often used for UI-less rendering)". **LABEL: inferred** for the conclusion that the same read therefore works in a background run — the sources document the switches and the read separately, not the two together. One documented caveat about background mode is worth carrying: "--disable-depsgraph-on-file-load — Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)." That flag is opt-in, so the default behaviour is unaffected, but it shows background mode does differ from interface mode in what it evaluates on load.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Operating system.** The sources state no operating-system difference in reading property values. The *location* of `userpref.blend` differs by platform — the directory-layout page lists per-platform user directories — but the file's name and role do not.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Where I looked.** S6, S7, S9; `api_5.0/info_quickstart.txt` (lines 85–180 in full); `api_5.0/bpy.types.BlendDataLibraries.txt` in full; `api_5.0/bpy.data.txt`; `api_5.0/bpy.types.RenderSettings.txt` and `api_5.0/bpy.types.SceneEEVEE.txt` for the readonly markers; `cycles_5.0/properties.py` registration and preferences blocks; `manual_5.0/advanced/blender_directory_layout.txt`; `manual_5.0/editors/preferences/introduction.txt`. I searched `api_5.0/info_gotchas_internal_data_and_python_objects.txt` and `api_5.0/info_advanced_blender_as_bpy.txt` for `blend file|blend-file|file format|read`; the first turned out to be about naming collisions and stale references and is ruled out, the second yielded the two quotes above and is used. Neither describes reading a setting's value out of an unopened file.

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


## Things only a running Blender can settle

This list has been pruned three times. Everything it held that a rented Linux machine could settle was run on 2026-08-06 and is written into the answers above, under the question each one answers. The item that was left waiting for a working graphics card was run on 2026-08-07, once that machine's card had been repaired. The one list covering every version, with the full procedure for each test and the results, is `Audits/Blender Research/What a render demands/Hand tests.md`. **A third sitting, on the Creator's own Mac on 2026-08-07, ran every item that was still owed there, including the two that need a window — which no machine on this project but that one has.**

**Nothing on this list is still open.**

**Run and answered above:** `-t` against the file's Threads setting and against the thread count actually used; Tile Size, Threads, the acceleration-structure toggles and Persistent Data against the picture; what `--cycles-print-stats` prints; whether a Cycles property is readable while EEVEE is the engine; reading another file with `bpy.data.libraries.load`; Resolution Percentage in a command-line render; and **whether `--cycles-device` overrides both the preference and the scene** — it does, in both directions, and the exact run this item asked for (preference on a card backend, the file set to GPU Compute, `--cycles-device CPU`) put the frame back on the processor.

**All of that has now been run, on the Creator's own Mac on 2026-08-07, and is recorded above under the question each one answers.** Apple's own graphics render a different picture from the processor, exactly as the NVIDIA card does; `--cycles-device METAL` beats both the file and the saved preferences, in both directions; the three engines really do show different Performance panels; and the Acceleration Structure controls really do change with the device. **Nothing on this subject is still owed on either machine.**

Three tests were deliberately **not** repeated on the Mac, and the reason is written here rather than left as a silence: the ones whose only possible evidence is a time or a memory figure — whether `-t` changes how much of the processor a Cycles render uses, whether the thread count changes anything under EEVEE or Workbench, and whether Persistent Data changes the time or the memory of an animation. The Mac was shared with other topics' work throughout, so a figure taken there would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. What they leave open on macOS is left open and is not guessed at.

### Cannot be run

**Does `--qos eco` change render speed?** The switch is documented as Windows only and no Windows machine has run it, so it was not run. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** The question comes back untested rather than answered.

---

2026-08-06 — Opus 5 (Claude Code) — What a render demands, Blender 5.0

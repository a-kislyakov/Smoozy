# What a render demands — Blender 5.1 — second pass

**Sources consulted, and when.** Blender's manual for 5.1, Blender's Python API documentation for 5.1, Blender's 5.1 release notes, and Cycles' own files as shipped with 5.1 — all read on **2026-07-30** (the date the copies were taken). Blender's requirements page, read on **2026-08-01**.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

A note on the requirements page before anything else: it carries no version, and it names Blender 4.5 LTS, 3.6 LTS, 3.4, 2.92, 2.90, 2.83 LTS, 2.79 and 2.76 by name. It does **not** name 5.1. The single place it speaks for a range that includes 5.1 is the macOS note quoted under question 1's operating-system line. Everywhere else on that page I have treated it as describing some other release, and have not used it.

A note on readings. Three of the five questions could be read two ways. I say which reading I answered, and what the other reading was, in each answer.

---

## 1. What settings, if any, change how much a render demands of the computer it runs on?

**Reading I answered.** "Demands of the computer" = how much processing work, how much time, and how much memory the render uses. **The other reading** would be "what a computer must have before Blender will render at all" — the minimum-hardware reading. I answer the first because the sheet's own preamble says these questions are about "how hard it works the computer and how much of it the render takes up", not about whether it runs.

**LABEL: documented.**

Blender's manual has a page for exactly this, one per engine, and states its purpose outright.

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

— Blender manual, Cycles Performance panel, <https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html>, read 2026-07-30.

The settings the sources tie to speed, memory or processing work, grouped by where they live. Every one is quoted.

### A. Shared by every engine (they sit on the scene's render settings)

**Threads Mode and Threads.**

> "Threads Mode
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
> The maximum number of CPU cores to use simultaneously while rendering."

— Cycles Performance panel page, address as above, read 2026-07-30.

The API documentation gives the kind and range:

> "threads
>
> Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) (in [1, 1024], default 1)"

and

> "threads_mode
>
> Determine the amount of render threads used (default 'AUTO')"

— Blender Python API, <https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html>, read 2026-07-30.

**Persistent Data.**

> "Persistent Data
>
> Keep render data in memory after rendering for faster re-renders and animation renders
> at the cost of extra memory usage while performing other tasks in Blender.
>
> When using multiple View Layers,
> only data from a single view layer is preserved to keep memory usage within bounds;
> however, objects shared between view layers are preserved."

— Cycles Performance panel page, read 2026-07-30. The API entry reads:

> "use_persistent_data
>
> Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage (default False)"

— <https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html>, read 2026-07-30.

**Resolution and the resolution percentage.**

> "Resolution X, Y
>
> The number of pixels horizontally and vertically in the image.
>
> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image."

— Blender manual, Format panel, <https://docs.blender.org/manual/en/5.1/render/output/properties/format.html>, read 2026-07-30. The picture on that page (`_images/render_output_properties_format_panel.png`, in the manual's `_images` folder, read 2026-07-30) shows the panel with Resolution X 1920, Y 1080 and % set to 100%. **Documented** (it is part of the manual). What the picture shows is those three fields sitting together in one panel; anything about their effect comes from the text, not the picture.

The manual does not say on that page that a smaller percentage costs less. It says it is "useful for small test renders". Ranges, from the API documentation:

> "resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)"
>
> "resolution_x
>
> Number of horizontal pixels in the rendered image (in [4, 65536], default 1920)"

— <https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html>, read 2026-07-30.

**Render Region and Crop to Render Region.**

> "Render Region
>
> Renders just a portion of the view instead of the entire frame."

— Format panel page, read 2026-07-30.

**Simplify.** The manual explicitly ties one of these to computer resources:

> "Texture Limit
>
> Automatically scales textures down so that they are no larger than the values chosen.
> This can help reduce computer memory resources when rendering large scenes with huge textures."

and

> "Max Subdivision
>
> Maximum number of subdivision by the Subdivision Surface modifiers."
>
> "Child Particles
>
> Show only a subset of all child hairs and particles."

— Blender manual, Simplify, <https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/simplify.html>, read 2026-07-30. Simplify has separate Viewport and Render columns ("Viewport / See Common Settings above. … Render / See Common Settings above."), and a Culling section:

> "Camera Cull
>
> Automatically culls objects based on the camera frustum defined by the Margin.
>
> Distance Cull
>
> Automatically culls objects based on their distance from the active camera."

— same page, read 2026-07-30. Simplify as a whole is described in the API documentation as:

> "use_simplify
>
> Enable simplification of scene for quicker preview renders (default False)"

— <https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html>, read 2026-07-30.

**Compositor device and precision** (these sit in the Performance panel of both Cycles and EEVEE):

> "Device
>
> The device used for compositing.
>
> CPU:
>
> Use the CPU for compositing.
>
> GPU:
>
> Use the GPU for compositing."
>
> "Precision GPU
>
> The precision of compositor intermediate result.
>
> Auto:
>
> Use full precision for final renders, half precision otherwise."

— Cycles Performance panel page and EEVEE Performance panel page (<https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/performance.html>), read 2026-07-30.

**Denoise-node quality in the compositor**, on the same two pages:

> "Final Quality
>
> The quality used by Denoise nodes during the final
> render if their quality is set to Follow Scene.
>
> High:
>
> Produces the highest quality output at the cost of long processing times.
>
> Balanced:
>
> Balanced between performance and quality, typically processing in half the time as High,
> while retaining most of the quality.
>
> Fast:
>
> Produces an output quickly at a noticeable cost of quality."

— read 2026-07-30.

**Per-view-layer sample override:**

> "Samples
>
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples
> when using the Cycles renderer. When using EEVEE, the Samples override is always used.
>
> Lowering the number of samples for background or utility passes can speed up render times,
> while still using higher samples for beauty passes."

— Blender manual, View Layer Override, <https://docs.blender.org/manual/en/5.1/render/layers/override.html>, read 2026-07-30.

### B. Cycles only

**Device (CPU or GPU Compute).**

> "Device
>
> Specifies the compute device used for rendering in Cycles.
> The choice of device can significantly affect render performance, GPU Compute is often faster ."

— Blender manual, Cycles Render Settings, <https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html>, read 2026-07-30. In Cycles' own files this is **observed** as a two-value choice:

> ```
> enum_devices = (
>     ('CPU', "CPU", "Use CPU for rendering"),
>     ('GPU', "GPU Compute",
>         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
> )
> ```

— `Blender Sources/cycles_5.1/properties.py`, lines 30–34, read 2026-07-30. And the property itself:

> ```
>     device: EnumProperty(
>         name="Device",
>         description="Device to use for rendering",
>         items=enum_devices,
>         default='CPU',
>         update=update_render_passes,
>     )
> ```

— same file, lines 381–387, read 2026-07-30.

**Tile Size.**

> "Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage.
> Tiles are cached to disk while rendering to save memory."
>
> "Note
>
> In some cases changing the Tile Size can result in increased performance.
> For example when a small object renders slowly compared to other objects,
> using a small Tiles Size can lead to an increase in performance."

— Cycles Performance panel page, read 2026-07-30. **Observed**, its kind, default and range:

> ```
>     tile_size: IntProperty(
>         name="Tile Size",
>         default=2048,
>         description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
>         min=8,
>         max=8192,
>     )
> ```

— `Blender Sources/cycles_5.1/properties.py`, lines 1056–1062, read 2026-07-30. Immediately above it sits a companion switch that the file itself marks as no longer meaningful — **observed**:

> ```
>     use_auto_tile: BoolProperty(
>         name="Auto Tile",
>         description="Deprecated, tiling is always enabled",
>         default=True,
>     )
> ```

— same file, lines 1051–1055, read 2026-07-30.

**Acceleration structure (four settings).**

> "Use Spatial Splits
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
> Use a more compact BVH structure, which can reduce RAM usage but render slower."

— Cycles Performance panel page, read 2026-07-30. **Observed** defaults and ranges, `Blender Sources/cycles_5.1/properties.py`, lines 920–940, read 2026-07-30: `debug_use_spatial_splits` default `False`; `debug_use_hair_bvh` default `True`; `debug_use_compact_bvh` default `False`; `debug_bvh_time_steps` default `0`, `min=0, max=16`.

**Samples, time limit, adaptive sampling.**

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time.
>
> Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit."
>
> "Note
>
> The time limit does not include pre-render processing time, only render time."
>
> "Noise Threshold
>
> The threshold to decide whether to continue sampling a pixel.
> Typical values are in the range from 0.1 to 0.001, with lower values meaning better
> quality but longer render times.
> Setting this to 0 makes Cycles guess an automatic value based on the total sample count.
>
> Min Samples
>
> The minimum number of samples a pixel receives before adaptive sampling is applied.
> When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold."

— Blender manual, Cycles Sampling, <https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html>, read 2026-07-30. **Observed**: `samples` default 4096, `min=1, max=(1 << 24)`; `adaptive_threshold` default 0.01; `adaptive_min_samples` default 0, `max=4096`; `time_limit` default 0.0 — `Blender Sources/cycles_5.1/properties.py`, lines 479–608, read 2026-07-30.

**Sample Subset** — this one exists specifically to split one frame's work across machines:

> "Sample Subset
>
> Only render a subset of the samples. Multiple subset renders can be combined into a full
> one by running the following in the Python Console:
>
> bpy.ops.cycles.merge_images(input_filepath1=r"1.exr", input_filepath2=r"2.exr",
> output_filepath=r"combined.exr")
>
> A typical use case is to distribute the rendering of a single frame over multiple machines."

— Cycles Sampling page, read 2026-07-30. **Observed**: `use_sample_subset` default `False`, described as "Render a subset of the specified max samples. Typically used for distributed rendering across multiple devices"; `sample_offset` default 0; `sample_subset_length` default 2048 — `properties.py`, lines 494–512, read 2026-07-30.

**Denoising, and where denoising runs.**

> "Use GPU
>
> Perform denoising on the GPU.
> This is significantly faster than on CPU, but requires additional GPU memory.
> When large scenes need more GPU memory, this option can be disabled."

and

> "Quality OpenImageDenoise
>
> Overall denoising quality.
> Visible only when using OpenImageDenoise.
>
> High:
>
> Produces the highest quality at the cost of time."

and

> "None:
>
> Does not apply any prefiltering to the input passes. This option retains the most detail and
> is the fastest, but assumes the input passes are noise free which may require a high sample count."

— Cycles Sampling page, read 2026-07-30. **Observed**: `denoising_use_gpu` default `False`, `denoising_quality` default `'HIGH'`, `denoising_prefilter` default `'ACCURATE'` — `properties.py`, lines 414–436, read 2026-07-30.

**Bounce limits and clamping.**

> "Total
>
> Maximum number of light bounces. For best quality, this should be set to the maximum.
> However, in practice, it may be good to set it to lower values for faster rendering.
> A value of 0 bounces results in direct lighting only."

and

> "Light paths are terminated probabilistically when specifying a minimum number of light bounces
> lower than the maximum. In that case, paths longer than minimum will be randomly stopped when
> they are expected to contribute less light to the image.
> This will still converge to the same image, but renders faster while possibly being noisier."

and

> "Note that, while semantically the ray passes through as if no geometry was hit,
> rendering performance is affected as each transparency step requires executing the shader and tracing a ray."

— Blender manual, Cycles Light Paths, <https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/light_paths.html>, read 2026-07-30. **Observed** defaults: `max_bounces` 12, `diffuse_bounces` 4, `glossy_bounces` 4, `transmission_bounces` 12, `volume_bounces` 0, `transparent_max_bounces` 8, all `min=0, max=1024` — `properties.py`, lines 744–781, read 2026-07-30.

**Fast GI Approximation.**

> "Approximate diffuse indirect light with background tinted ambient occlusion.
> This provides fast alternative to full global illumination (GI),
> for interactive viewport rendering or final renders with reduced quality."
>
> "Render Bounces
>
> Number of bounces when rendering final renders."

— Cycles Light Paths page, read 2026-07-30.

**Caustics and Filter Glossy.**

> "Reflective
>
> While in principle path tracing supports rendering of caustics with a sufficient number of samples,
> in practice it may be inefficient to the point that there is just too much noise.
> This option can be unchecked, to disable reflective caustics."

— Cycles Light Paths page, read 2026-07-30.

**Light Tree and Light Threshold.**

> "Light Tree
>
> Use a light tree to more effectively sample lights in the scene, taking into account
> distance and estimated intensity.
> This can significantly reduce noise, at the cost of a somewhat longer render time per sample."
>
> "Light Threshold
>
> Probabilistically terminates light samples when the light contribution is below the threshold.
> This avoids wasting time on lights that contribute little to the image.
> Zero disables the test."

— Cycles Sampling page, read 2026-07-30.

**Scrambling Distance.**

> "A technique that reduces the randomness between pixels in an attempt to improve GPU rendering performance,
> at the cost of potential artifacts. Not compatible with the Blue-Noise sampling pattern."

— Cycles Sampling page, read 2026-07-30.

**Volumes.**

> "Render time can be reduced by increasing the step size, at the cost of
> potentially losing some volume detail."
>
> "Step Rate Render Biased
>
> Global multiplier on the step size for all volumes in renders.
> Increase to reduce render time, at the cost of losing detail.
>
> Max Steps Biased
>
> Maximum number of steps through the volume before giving up,
> to protect from extremely long render times with big objects or small step sizes."

— Blender manual, Cycles Volumes, <https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/volumes.html>, read 2026-07-30. **Observed**: `volume_step_rate` default 1.0 (`min=0.01, max=100.0`), `volume_max_steps` default 1024 (`min=2, max=65536`), `volume_biased` default `False` — `properties.py`, lines 783–810, read 2026-07-30.

**Subdivision (dicing).**

> "Offscreen Scale
>
> Multiplier for dicing rate of geometry outside of the camera view.
> The dicing rate of objects is gradually increased the further they are outside the camera view.
> Lower values provide higher quality reflections and shadows for off screen objects,
> while higher values use less memory.
>
> Max Subdivisions
>
> Stop subdividing when this level is reached even if the dicing rate would produce finer Tessellation."

— Blender manual, Cycles Subdivision, <https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/subdivision.html>, read 2026-07-30.

**Curve shape.**

> "Rounded Ribbons:
>
> Render curves as flat ribbon with rounded normals, for fast rendering."
>
> "Linear 3D Curves:
>
> Render curves as 3D circular geometry with linear interpolation between control points.
> This is faster than full 3D curves but less smooth, and may show visible corners at low resolutions.
>
> This is an optimization to take advantage of hardware accelerated curve intersection on GPUs.
> Results will not look exactly the same on different CPU and GPU devices, so this is not
> suitable for multi device rendering or render farms with mixed hardware."

— Blender manual, Cycles Curves, <https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/hair.html>, read 2026-07-30.

**Open Shading Language.**

> "Rendering performance is typically slower than with built-in nodes due to shader compilation and interpretation."

— Cycles Render Settings page, read 2026-07-30.

**Cycles settings that are not in the file at all but in the program's preferences.** These change the demand and are **observed** in `Blender Sources/cycles_5.1/properties.py`, class `CyclesPreferences`, lines 1569–1649, read 2026-07-30: `compute_device_type` (which backend), the `devices` collection (which individual devices are ticked, each with a `use` flag defaulting to `True`), `peer_memory`, `metalrt`, `use_hiprt`, `use_oneapirt` and `kernel_optimization_level`. Two of them state their cost outright:

> ```
>     peer_memory: BoolProperty(
>         name="Distribute memory across devices",
>         description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it",
>         default=False,
>     )
> ```

and

> ```
>         items=(
>             ('OFF', "Off", "Disable kernel optimization. Slowest rendering, no extra background CPU usage"),
>             ('INTERSECT', "Intersection only", "Optimize only intersection kernels. Faster rendering, negligible extra background CPU usage"),
>             ('FULL', "Full", "Optimize all kernels. Fastest rendering, may result in extra background CPU usage"),
>         ),
> ```

— same file, lines 1608–1612 and 1644–1648, read 2026-07-30.

The manual describes the same things from the outside:

> "Distribute Memory Across Devices
>
> Allocates resources across multiple GPUs rather than duplicating data,
> effectively freeing up space for larger scenes. Note that in order for this option to be available,
> the GPUs must be connected together with a high bandwidth communication protocol.
>
> Currently only NVLink on NVIDIA GPUs is supported."

— Blender manual, Preferences ‣ System, <https://docs.blender.org/manual/en/5.1/editors/preferences/system.html>, read 2026-07-30.

### C. EEVEE only

> "Shadow Pool
>
> A bigger pool size allows for more shadows in the scene
> but might not fit into GPU memory and decreases performance.
> Increasing the size might fix the Shadow buffer full error.
>
> Light Probes Volume Pool
>
> A bigger pool size allows for more irradiance grids in the scene
> but might not fit into GPU memory and decreases performance."

and

> "High Quality Normals
>
> Uses higher precision normals and tangents which can improve
> visual quality for dense meshes with high frequency textures at the cost of memory."

— Blender manual, EEVEE Performance panel, <https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/performance.html>, read 2026-07-30.

> "TAA is sample based so the more samples the more aliasing is reduced at the cost of performance."
>
> "Steps
>
> Number of shadow map sample per shadow ray.
> Higher step count results in softer shadows but have a higher cost.
>
> Volumetric Shadows
>
> Approximate light absorption of the surrounding volume objects. This makes the volumes more opaque to light.
> This is a very computationally expensive option and has limitations."
>
> "Light Threshold
>
> Minimum light intensity for a light to contribute to the lighting.
> Used to compute the distance at which to cut-off lights influence.
> Lower values improve performance."

— Blender manual, EEVEE Sampling, <https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html>, read 2026-07-30.

> "Method
>
> Determine the tracing method used to find scene-ray intersections and indirect lighting.
>
> Light Probe:
>
> Use light-probe spheres and planes to find scene intersection.
> This option has the lowest tracing cost but relies on manually placed light-probes."
>
> "Resolution
>
> Resolution at which the ray-tracing is performed.
> Lower options will be faster and use less memory but will produce blurrier results."
>
> "Precision
>
> Higher values increase precision of the screen space ray-tracing but lower the maximum trace distance.
> Increased precision also increases performance cost."

— Blender manual, EEVEE Raytracing, <https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/raytracing.html>, read 2026-07-30.

From the API documentation, with their kinds and ranges (all read 2026-07-30 at <https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html>):

> "taa_render_samples
>
> Number of samples per pixel for rendering (in [1, inf], default 64)"
>
> "shadow_pool_size
>
> Size of the shadow pool, a bigger pool size allows for more shadows in the scene but might not fit into GPU memory (default '512')"
>
> "shadow_ray_count
>
> Amount of shadow ray to trace for each light (in [1, 4], default 1)"
>
> "shadow_step_count
>
> Amount of shadow map sample per shadow ray (in [1, 16], default 6)"
>
> "shadow_resolution_scale
>
> Resolution percentage of shadow maps (in [0, 1], default 1.0)"
>
> "volumetric_samples
>
> Number of steps to compute volumetric effects. Higher step count increase VRAM usage and quality. (in [1, 256], default 64)"
>
> "volumetric_ray_depth
>
> Maximum surface intersection count used by the accurate volume intersection method. Will create artifact if it is exceeded. Higher count increases VRAM usage. (in [1, 16], default 16)"
>
> "use_volumetric_shadows
>
> Cast shadows from volumetric materials onto volumetric materials (Very expensive) (default False)"
>
> "use_shadows
>
> Enable shadow casting from lights (default True)"
>
> "use_raytracing
>
> Enable the ray-tracing module (default False)"

### D. Workbench only

> "The quality of the renders can be adjusted by changing the Anti-Aliasing method.
> A different one can be selected for the 3D Viewport, viewport rendering and
> for final rendering."
>
> "Multisample
>
> The scene will be rendered multiple times with a slight offset.
> The anti-aliasing will be gathered from the multiple renders.
> The number of samples are predefined so it uses the best distribution of the samples.
>
> 5, 8, 11, 16, 32"

— Blender manual, Workbench Sampling, <https://docs.blender.org/manual/en/5.1/render/workbench/sampling.html>, read 2026-07-30. Workbench's Performance page holds only High Quality Normals: "Uses higher precision normals and tangents which can improve visual quality for dense meshes with high frequency textures at the cost of memory." — <https://docs.blender.org/manual/en/5.1/render/workbench/performance.html>, read 2026-07-30.

### E. The 5.1 release notes on this subject

> "* Improved GPU rendering performance by 5-10% on various benchmark scenes.
> * Improved CPU rendering performance on Windows, often around 5-20%.
> * For AMD GPUs, hardware ray-tracing is now enabled by default through HIP RT."

— Blender 5.1 release notes, Cycles, <https://developer.blender.org/docs/release_notes/5.1/cycles/>, read 2026-07-30. The last of those three matches what is in the file: `use_hiprt: BoolProperty(name="HIP RT", …, default=True,)` — `Blender Sources/cycles_5.1/properties.py`, lines 1626–1630, **observed**, read 2026-07-30.

EEVEE's notes report memory changes rather than new settings:

> "EEVEE saves some texture memory by overlapping framebuffer and render textures at
> different times in a frame."

— Blender 5.1 release notes, EEVEE & Viewport, <https://developer.blender.org/docs/release_notes/5.1/eevee/>, read 2026-07-30.

**NOT COVERED.** This answer lists the settings the sources connect to speed, memory or processing work. It does not settle how much any of them changes the demand, in what proportion, or how they interact — the sources give directions ("faster", "less memory") and almost never numbers. It also does not cover per-object settings (each object has its own visibility, motion-blur, cull and AO settings), per-material settings, or the scene content itself, all of which the sources touch on but which are not "render settings". It does not settle whether this list is complete: I collected it by searching for the language of cost, and a setting whose description never mentions speed or memory would not have been caught.

**Outside the interface.** Every setting above lives on the scene or on the preferences, not on a window, so it is present whether or not Blender is showing an interface — this follows from where the sources place them (on `Scene.render`, `Scene.cycles`, `Scene.eevee`, and the Cycles add-on preferences), so **inferred**, the step being that a value stored on the scene is still stored on the scene when there is no window. Two documented differences do apply:

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

- Everything the sources label "Viewport" is about the 3D viewport, not the final render — for example "Pixel Size / Option to control the resolution for viewport rendering", "Viewport (Max) Samples / The number of paths to trace per pixel in the 3D Viewport (when using the Rendered shading mode)", and Simplify's separate Viewport and Render columns. Cycles keeps a separate second copy of many settings for the viewport (`preview_samples`, `preview_dicing_rate`, `texture_limit` versus `texture_limit_render`, `ao_bounces` versus `ao_bounces_render`, `volume_preview_step_rate` — **observed**, `Blender Sources/cycles_5.1/properties.py`, read 2026-07-30). That a background render is not a viewport render is **inferred** from those words.
- One setting is documented as ignored outside the interface: "Render Single Layer / Only render the active view layer." followed by "Note / This option is ignored when rendering from the command line." — Blender manual, View Layer, <https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html>, read 2026-07-30. **Documented.**

**Operating system.** Mostly yes, but not entirely.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

- Which Cycles backends exist at all depends on the operating system: "CUDA is supported on Windows and Linux"; "OptiX is supported on Windows and Linux"; "HIP is supported on Windows and Linux"; "oneAPI is a computation library that is supported on Windows and Linux"; "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." — Blender manual, GPU Rendering, <https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html>, read 2026-07-30. **Documented.**
- EEVEE: "Headless rendering is not supported on headless Windows systems." — Blender manual, EEVEE Limitations, <https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html>, read 2026-07-30. **Documented.**
- The preferences setting that governs how shaders are compiled: "This option is not available on macOS and requires using OpenGL backend." — Preferences ‣ System page, read 2026-07-30. **Documented.**
- The requirements page, read 2026-08-01 at <https://www.blender.org/download/requirements/>, is the one place naming a range that takes in 5.1: "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." I flag that "5.0 and later" is a range rather than the words "5.1"; treat it as weaker than a page that names the version.

**Where I looked.** Searched the whole of `manual_5.1`, `api_5.1` and `release_notes_5.1` for: performance, memory, RAM, VRAM, speed, faster, slower, render time, threads, cores, CPU, GPU, device, tile, persistent, BVH, samples, bounces, denoise, simplify, cull, texture limit, dicing, step rate, shadow pool, memory limit, out of core, host memory, affinity, process priority, nice level. Opened and read in full: the Cycles Performance, Sampling, Light Paths, Simplify, Subdivision, Volumes, Curves, Motion Blur and Render Settings index pages; the EEVEE Performance, Sampling and Raytracing pages; the Workbench Performance and Sampling pages; the GPU Rendering page; the Format page; the View Layer and View Layer Override pages; the Preferences ‣ System page; `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE` and `bpy.types.Scene`. In Cycles' own files: `properties.py` (all 1994 lines scanned by property name, and the relevant blocks read line by line), `ui.py` performance panels, `presets.py`, `engine.py`, `__init__.py`. Ruled out: the Freestyle, materials, shader-node, compositing-node, modelling, sculpting and video-editing sections, after searching them for the same terms and finding only per-feature costs, not render-wide settings. Searched for "memory limit", "process priority", "affinity", "nice level", "RAM limit", "out-of-core", "out of core", "host memory" across all three documentation sets: the only hits were the preferences page (Undo Memory Limit and the sequencer's Memory Cache Limit, neither of which is a render setting), the sequencer cache page, and a Python crash-gotchas page — so Blender 5.1 has no documented ceiling you can put on a render's memory use.

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

**LABEL: mixed — documented for some, not in the sources for others.** I give both groups.

### Settings the sources say change the image

- **Samples**: "A higher number results in a cleaner image at the cost of a longer render time." (Cycles Sampling page.)
- **Time Limit**: "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet." (Cycles Sampling page.) The image is whatever had been reached; the source does not say more.
- **Noise Threshold**: "lower values meaning better quality but longer render times". (Cycles Sampling page.)
- **Denoising quality and prefilter**: "High: Produces the highest quality at the cost of time." and "Fast: Assumes the input passes are not noise free, yet does not apply prefiltering to the input passes. This option is faster than Accurate but produces a blurrier result." (Cycles Sampling page.)
- **Denoise on GPU**: the description does not claim the image differs, only "This is significantly faster than on CPU, but requires additional GPU memory." (Cycles Sampling page.)
- **Bounce limits**: "For best quality, this should be set to the maximum. However, in practice, it may be good to set it to lower values for faster rendering. A value of 0 bounces results in direct lighting only." (Cycles Light Paths page.)
- **Minimum bounces**: "This will still converge to the same image, but renders faster while possibly being noisier." (Cycles Light Paths page.) That is a rare explicit statement that the image is unchanged in the limit.
- **Clamping**: "It reduces noise at the cost of accuracy." and "note that as you clamp out such values, other bright lights/reflections will be dimmed as well." (Cycles Light Paths page.)
- **Filter Glossy**: "this will blur glossy reflections after blurry bounces, to reduce noise at the cost of accuracy." (Cycles Light Paths page.)
- **Caustics off**: "This option can be unchecked, to disable reflective caustics." (Cycles Light Paths page.)
- **Fast GI**: "final renders with reduced quality" — it replaces global illumination with ambient occlusion, "Replace: Replace global illumination with ambient occlusion after a specified number of bounces." (Cycles Light Paths page.)
- **Light Tree**: "This can significantly reduce noise, at the cost of a somewhat longer render time per sample." with the caveat "Certain lighting properties are not accounted for in the light tree… This can result in an increase in noise in some scenes that make use of these features." (Cycles Sampling page.)
- **Light Threshold**: "Probabilistically terminate light samples when the light contribution is below this threshold (more noise but faster rendering)." — **observed**, `properties.py` line 559, read 2026-07-30.
- **Scrambling Distance**: "at the cost of possible rendering artifacts if set too low" — **observed**, `properties.py` line 536, read 2026-07-30; and in the manual "at the cost of potential artifacts".
- **Sampling Pattern**: "Blue-Noise: … If the full number of samples is rendered, the output typically appears smoother than Classic despite not actually reducing the overall noise." (Cycles Sampling page.)
- **Seed / Animated Seed**: "Seed value for integrator to get different noise patterns." (Cycles Sampling page.)
- **Sample Subset**: "it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets – otherwise, the subsets will have incompatible noise and combining them will give a worse result." (Cycles Sampling page.)
- **Texture Limit**: "Automatically scales textures down so that they are no larger than the values chosen." (Simplify page.)
- **Max Subdivision, Child Particles, Culling**: Simplify is "simplification of scene for quicker preview renders" (API, `RenderSettings.use_simplify`); culling removes objects — "Automatically culls objects based on the camera frustum" (Simplify page).
- **Volume step rate**: "Increase to reduce render time, at the cost of losing detail." **Max Steps**: "Maximum number of steps through the volume before giving up" (Cycles Volumes page).
- **Offscreen Dicing Scale**: "Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory." (Cycles Subdivision page.)
- **Max Subdivisions (Cycles dicing)**: "Stop subdividing when this level is reached even if the dicing rate would produce finer Tessellation." (Cycles Subdivision page.)
- **Curve shape**: "This is faster than full 3D curves but less smooth, and may show visible corners at low resolutions… Results will not look exactly the same on different CPU and GPU devices, so this is not suitable for multi device rendering or render farms with mixed hardware." (Cycles Curves page.) This is the one place the sources say the **device** can change the image.
- **Resolution and percentage**: "Slider to reduce or increase the size of the rendered image relative to the Resolution values." (Format page.)
- **Render Region / Crop**: "Renders just a portion of the view instead of the entire frame." and "Crops the rendered image to the size of the render region, instead of rendering a transparent background around it." (Format page.)
- **Per-view-layer sample override**: "Lowering the number of samples for background or utility passes can speed up render times, while still using higher samples for beauty passes." (View Layer Override page.)
- **Compositor Precision**: "The precision of compositor intermediate result." with "Auto: Use full precision for final renders, half precision otherwise." (Cycles and EEVEE Performance pages.)
- **Compositor Denoise-node Final Quality**: "Fast: Produces an output quickly at a noticeable cost of quality." (Cycles and EEVEE Performance pages.)
- **EEVEE samples**: "TAA is sample based so the more samples the more aliasing is reduced at the cost of performance." (EEVEE Sampling page.)
- **EEVEE shadow steps**: "Higher step count results in softer shadows but have a higher cost." **Shadow Resolution**: "Resolution percentage of shadow maps." (EEVEE Sampling page.)
- **EEVEE Light Threshold**: "Minimum light intensity for a light to contribute to the lighting. Used to compute the distance at which to cut-off lights influence. Lower values improve performance." (EEVEE Sampling page.)
- **EEVEE ray-tracing resolution and precision**: "Lower options will be faster and use less memory but will produce blurrier results." and "Higher values increase precision of the screen space ray-tracing but lower the maximum trace distance." (EEVEE Raytracing page.)
- **EEVEE volumetric ray depth**: "Will create artifact if it is exceeded. Higher count increases VRAM usage." (API, `SceneEEVEE.volumetric_ray_depth`.)
- **EEVEE High Quality Normals**: "can improve visual quality for dense meshes with high frequency textures at the cost of memory." (EEVEE Performance page.)
- **Workbench anti-aliasing**: "With this option selected no anti-aliasing will be applied." (Workbench Sampling page.)
- **Open Shading Language**: "Some features, such as baking and certain volume shaders, may not work with OSL." (Cycles Render Settings page.)

### Settings whose effect on the image the sources never state

For these, the descriptions speak only of speed and memory, and say nothing at all about the picture. **LABEL for this part: not in the sources.** I am reporting a silence, not asserting that the image is identical.

- **Threads Mode / Threads** — the entire description is about how many cores are used.
- **Tile Size** — "to reduce memory usage. Tiles are cached to disk while rendering to save memory", plus the note about performance. Nothing about the picture.
- **Persistent Data** — "for faster re-renders and animation renders at the cost of extra memory usage".
- **Use Spatial Splits / Use Curves BVH / BVH Time Steps / Use Compact BVH** — all four descriptions are about build time, render time and memory only.
- **Device (CPU or GPU Compute)** in general — the manual says only "The choice of device can significantly affect render performance". The exception is Linear 3D Curves, quoted above, where it says the results differ. It also warns generally: "Note / GPU rendering may have limitations compared to CPU rendering, depending on the backend." (Cycles Render Settings page.)
- **Compositor Device (CPU or GPU)** — described only as which device is used.
- **Distribute Memory Across Devices, HIP RT, MetalRT, Embree on GPU, Kernel Optimization** — the preferences descriptions are about memory and speed.
- **EEVEE Shadow Pool and Light Probes Volume Pool** — "A bigger pool size allows for more shadows in the scene but might not fit into GPU memory and decreases performance. Increasing the size might fix the Shadow buffer full error." That names an error, not a change in the picture.

**NOT COVERED.** For every entry in the first group, the sources give a direction, not a magnitude — nothing here settles how much cleaner, how much blurrier, or from what value the change becomes visible. For the second group, this answer settles only that the sources are silent; it does not settle that the image is byte-for-byte identical, and the Linear-3D-Curves warning is direct evidence that "device does not change the image" would be wrong as a blanket claim.

**Outside the interface.** Nothing in any of the quoted descriptions ties the effect on the image to whether an interface is present. **Inferred**, one step: the descriptions are of what the setting does to the render, and none of them mentions the window. The viewport-only settings are the exception already noted under question 1 — they describe the 3D viewport's picture, not the final render's.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Operating system.** The sources do not say for any of these, with two exceptions that touch the image indirectly: the curve-shape warning about "render farms with mixed hardware" is about hardware rather than operating system, and which backends exist per operating system (quoted under question 1) decides which of these paths you can take at all. Otherwise: the sources do not say.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Where I looked.** For each setting named in question 1 I read the full description on its manual page and, where Cycles owns it, the `description=` text in `Blender Sources/cycles_5.1/properties.py`, checking specifically for any words about the resulting image (quality, noise, blur, detail, artifacts, accuracy, appears, looks, result). Searched the manual for "identical", "same image", "no visual difference", "does not affect the image", "lossless"; the only near-hit is the Light Paths sentence "This will still converge to the same image", already quoted, and Fast GI's "This can reduce noise in interior scenes with little visual difference" about viewport bounces.

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

**Reading I answered.** "From outside Blender" = when Blender is launched by something else, e.g. a command line. **The other reading** would be "from another program talking to a running Blender", which the sources do not describe.

**LABEL: documented**, for the list of arguments; **inferred** where noted.

### Settable at launch, by a dedicated argument

The manual's Command Line Arguments page is headed "Blender 5.1" and lists, among the render options:

> "-t, --threads <threads>
>
> Use amount of <threads> for rendering and other operations
> [1-1024], 0 to use the systems processor count."
>
> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."
>
> "-S, --scene <name>
>
> Set the active scene <name> for rendering."

and under Cycles Render Options:

> "Cycles add-on options must be specified following a double dash.
>
> --cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU.
>
> Example:
>
> blender -b file.blend -f 20 -- --cycles-device OPTIX
>
> --cycles-print-stats
>
> Log statistics about render memory and time usage."

and under Misc Options:

> "--qos <level>
>
> Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).
>
> default: Uses the default behavior of the OS.
> high: Always makes use of performance cores.
> eco: Schedules Blender threads exclusively to efficiency cores."

and under GPU Options:

> "--gpu-backend
>
> Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."
>
> "--gpu-compilation-subprocesses
>
> Override the Max Compilation Subprocesses setting (OpenGL only)."

— Blender manual, Command Line Arguments, <https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html>, read 2026-07-30.

The `--cycles-device` argument's reach is stated more strongly inside Cycles' own file — **observed**:

> ```
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)
> ```

— `Blender Sources/cycles_5.1/engine.py`, lines 16–20, read 2026-07-30. The words "overriding user preferences and the scene setting" are the file's own; that Cycles reads this at start-up is also in the file, at lines 24–41 (`_parse_command_line`, called from `init()`).

The manual's worked example confirms the pattern:

> "blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a"
>
> "-E CYCLES
>
> Use the "Cycles Render" engine.
> For a list of available render engines, run blender -E help.
>
> -s 10 -e 500
>
> Set the start frame to 10 and the end frame to 500.
>
> -t 2
>
> Use only two threads."

— Blender manual, Rendering From The Command Line, <https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html>, read 2026-07-30. And a warning that the order of arguments matters:

> "Arguments are executed in the order they are given. eg:
>
> blender --background test.blend --render-frame 1 --render-output "/tmp"
>
> …will not render to /tmp because --render-frame 1 renders before the output path is set.
>
> blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

— Command Line Arguments page, read 2026-07-30.

So, by dedicated argument: **thread count, render engine, active scene, Cycles device, Cycles statistics logging, Windows core scheduling, GPU backend, and the OpenGL shader-compilation subprocess count.** That is the whole list. Nothing else on the sheet's subject has one.

### Settable at launch, but only by running Python

The same page documents:

> "-P, --python <filepath>
>
> Run the given Python script file.
>
> --python-text <name>
>
> Run the given Python script text block.
>
> --python-expr <expression>
>
> Run the given expression as a Python script.
>
> The expression may be a complete multi-line script;
> you are limited only by the platform's maximum argument length."

— Command Line Arguments page, read 2026-07-30. **Documented** that these run arbitrary Python at launch. That this therefore lets you set samples, tile size, persistent data, bounce limits, simplify, resolution percentage and the rest is **inferred** — the step being that those settings are Python properties on `Scene.render`, `Scene.cycles` and `Scene.eevee` (which the API documentation lists, and which the Cycles preset file writes to by exactly that route, see below), and that a Python script can assign to them. The sources do not spell out that step anywhere I found.

The evidence that these are ordinary assignable Python properties, **observed**:

> ```
>     preset_defines = [
>         "render = bpy.context.scene.render",
>         "cycles = bpy.context.scene.cycles",
>     ]
>
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

— `Blender Sources/cycles_5.1/presets.py`, lines 97–110, read 2026-07-30. This is the Performance preset, and it is the exact set of settings Blender itself writes and reads back as Python.

### Only by changing the file (or the preferences)

Everything else in question 1's list: samples, time limit, noise threshold, minimum samples, denoising and its quality/prefilter/GPU switch, all bounce limits, clamping, filter glossy, caustics, fast GI, light tree, light threshold, scrambling distance, sampling pattern, seed, sample subset, tile size, persistent data, all four acceleration-structure options, simplify and its texture limit and culling, volume step rate and max steps, dicing rate and max subdivisions and offscreen scale, curve shape, resolution and percentage, render region, compositor device and precision, denoise-node quality, and every EEVEE and Workbench setting. **LABEL for this: not in the sources** — meaning the sources give no argument for them; I searched the whole arguments page and found none (see the record below).

One important separate case: the Cycles **compute device type and the tick-boxes for individual devices** are not in the .blend at all. They live in the add-on's preferences — **observed**, `Blender Sources/cycles_5.1/properties.py`, line 1562 comment "# Properties saved in preferences" above `CyclesDeviceSettings`, and class `CyclesPreferences(bpy.types.AddonPreferences)` at line 1569, read 2026-07-30. The manual agrees: "To enable GPU rendering: / Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference. / Enable the available devices for the chosen backend. / Configure each scene to use the GPU Compute Device." — GPU Rendering page, read 2026-07-30. So the *backend and devices* come from the machine's preferences, the *CPU-or-GPU choice* comes from the file, and `--cycles-device` at launch overrides both.

**NOT COVERED.** This answer does not settle what happens when an argument and the file disagree, beyond the two cases the sources state (`--cycles-device` says it overrides both; `-E` and `-t` the manual calls "Specify" and "Use only two threads" without saying what is written back). It does not settle whether a value set by `--python-expr` survives to the render in every case, nor the interaction with `--disable-autoexec` (the page says "-Y, --disable-autoexec / Disable automatic Python script execution (Python-drivers & startup scripts), (default)" — that is about scripts in the file, not about `-P`/`--python-expr`, and I did not find a source settling whether the two interact). It does not cover environment variables beyond noting the page lists them.

**Outside the interface.** This whole answer is about outside the interface, so it holds by construction. Two documented points bear on it directly:

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

> "-b, --background
>
> Run in background (often used for UI-less rendering).
>
> The audio device is disabled in background-mode by default
> and can be re-enabled by passing in -setaudio Default afterwards."

— Command Line Arguments page, read 2026-07-30. And:

> "One advantage of using the command line is that we do not need a graphical display
> (no need for X server on Linux for example)
> and consequently we can render via a remote shell (typically SSH)."

— Rendering From The Command Line page, read 2026-07-30.

There is a third way to drive Blender from outside, and it loses the arguments entirely:

> "Command Line Arguments Unsupported
>
> Functionality controlled by command line arguments (shown by calling blender --help) isn't accessible.
>
> Typically this isn't such a limitation although there are some command line arguments that don't have
> equivalents in Blender's Python API (--threads and --log for example)."

and

> "The user's startup and preferences are ignored to prevent your local configuration from impacting script behavior.
> The Python module behaves as if --factory-startup was passed as a command line argument."

— Blender Python API, Blender as a Python Module, <https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html>, read 2026-07-30. **Documented**, and it names `--threads` as having no Python equivalent. Note also what the second quote implies for the Cycles device: with the preferences ignored, the machine's chosen backend is not in force.

**Operating system.** Two of the arguments are limited by operating system, in the page's own words: `--qos <level>` is "(Windows only)", and `--gpu-compilation-subprocesses` is "(OpenGL only)" — which the Preferences ‣ System page pairs with "This option is not available on macOS and requires using OpenGL backend." The rest of the render arguments carry no operating-system note. `--gpu-backend`'s valid options are "vulkan, metal, opengl", and Metal is documented elsewhere as Apple-only. **Documented.**

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Where I looked.** Read the whole of `manual_5.1/advanced/command_line/arguments.txt` (692 lines, every section) and `manual_5.1/advanced/command_line/render.txt`. Searched that arguments page specifically for: sample, samples, resolution, tile, memory, denoise, bounce, persistent, simplify, cycles, device, engine, threads — the only hits were the ones quoted, plus unrelated ones (`-c <cache_memory>` for the animation player, `--log-show-memory`, `--debug-memory`, `--debug-gpu-no-texture-pool`, `--no-native-pixels`). Read `manual_5.1/advanced/command_line/extension_arguments.txt` in full to rule out the `--command` route: it is entirely about installing and building extensions and offers nothing on render settings. Read `Blender Sources/cycles_5.1/engine.py` in full — it defines exactly two Cycles arguments and no others. Read `api_5.1/info_advanced_blender_as_bpy.txt` and `api_5.1/info_tips_and_tricks.txt` in full.

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

**LABEL: documented.** Yes — most of them do, and the manual says so as a principle.

> "Blender includes three render engines with different strengths:
>
> EEVEE is a physically based realtime renderer.
>
> Cycles is a physically based path tracer.
>
> Workbench is designed for layout, modeling and previews."
>
> "Each renderer has its own render settings to control render quality and performance."

— Blender manual, Render Introduction, <https://docs.blender.org/manual/en/5.1/render/introduction.html>, read 2026-07-30.

The split shows in the documentation's own shape: there is one Performance page per engine, at three different addresses, with three different contents (Cycles: threads, tile size, acceleration structure, persistent data, viewport pixel size, compositor. EEVEE: high quality normals, shadow pool, light-probe volume pool, viewport pixel size, compositor. Workbench: high quality normals only). Compare the three pages linked in question 1.

It shows in the Python API too. `Scene` carries three separate pointers, **documented**:

> "cycles
>
> Cycles render settings (readonly)
>
> Type:
>
> CyclesRenderSettings"

> "eevee
>
> EEVEE settings for the scene (readonly)
>
> Type:
>
> SceneEEVEE"

> "render
>
> (readonly, never None)
>
> Type:
>
> RenderSettings"

— Blender Python API, <https://docs.blender.org/api/5.1/bpy.types.Scene.html>, read 2026-07-30.

Sorting question 1's list against that:

**Belong to every engine** (they live on `Scene.render`): Threads Mode and Threads; Persistent Data; Resolution X/Y and Percentage; Render Region and Crop; the whole Simplify group including Texture Limit, Max Subdivision, Child Particles and the Grease Pencil options; Compositor Device and Precision; the compositor Denoise-node Preview and Final Quality; viewport Pixel Size. Evidence that these are shared: the Cycles Performance panel's own code reads them from `scene.render`, not from `scene.cycles` — **observed**:

> ```
>         scene = context.scene
>         rd = scene.render
>
>         col = layout.column()
>
>         col.prop(rd, "threads_mode")
>         sub = col.column(align=True)
>         sub.enabled = rd.threads_mode == 'FIXED'
>         sub.prop(rd, "threads")
> ```

— `Blender Sources/cycles_5.1/ui.py`, lines 855–863 (class `CYCLES_RENDER_PT_performance_threads`), read 2026-07-30. Same for Persistent Data at lines 939–944, and viewport Pixel Size at lines 956–961. Whereas Tile Size is read from `scene.cycles`:

> ```
>         scene = context.scene
>         cscene = scene.cycles
>
>         layout.prop(cscene, "tile_size")
> ```

— same file, lines 875–878 (class `CYCLES_RENDER_PT_performance_memory`), read 2026-07-30.

Two cautions about "every engine". First, Simplify's culling section is documented on the *Cycles* Simplify page, and the Cycles add-on registers the culling panel itself (`CYCLES_RENDER_PT_simplify_culling`, `Blender Sources/cycles_5.1/ui.py` line 2328, **observed**) — so Camera Cull and Distance Cull are Cycles', even though the rest of Simplify is shared. The Cycles properties confirm it: `use_camera_cull`, `camera_cull_margin`, `use_distance_cull`, `distance_cull_margin` all sit in `CyclesRenderSettings` (`properties.py`, lines 962–988, read 2026-07-30). Second, EEVEE cannot use the CPU at all, so the thread setting means something different there:

> "Being a rasterization engine, EEVEE only uses the power of
> the GPU to render.
> There is no plan to support CPU (software) rendering
> as it would be very inefficient. CPU power is still needed to handle high scene complexity
> as the geometry must be prepared by the CPU before rendering each frame."

and

> "Multiple GPU Support
>
> There is currently no support for
> multiple GPU systems."

— Blender manual, EEVEE Limitations, <https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html>, read 2026-07-30. **Documented.**

**Belong to Cycles alone**: Device (CPU / GPU Compute); Tile Size; Use Spatial Splits, Use Curves BVH, BVH Time Steps, Use Compact BVH; Samples, Time Limit, Noise Threshold, Min Samples; Sample Subset (Offset, Length); Denoiser choice, Passes, Prefilter, Quality, Use GPU; Path Guiding and its Training Samples; Light Tree and Light Threshold; Sampling Pattern, Seed, Scrambling Distance; Min Light Bounces and Min Transparent Bounces; Layer Samples; all bounce maxima; Clamping; Filter Glossy; Caustics; Fast GI Approximation; Volumes (Biased, Step Rate, Max Steps); Subdivision (Dicing Rate, Offscreen Scale, Max Subdivisions, Dicing Camera); Curves shape and subdivisions; Camera and Distance Cull; Open Shading Language; plus everything in the Cycles preferences (compute device type, device tick-boxes, Distribute Memory Across Devices, MetalRT, HIP RT, Embree on GPU, Kernel Optimization). All of these are properties of `CyclesRenderSettings` or `CyclesPreferences` — **observed**, `Blender Sources/cycles_5.1/properties.py`, lines 378–1112 and 1558–1649, read 2026-07-30.

**Belong to EEVEE alone**: Samples (`taa_render_samples`); Temporal Reprojection; Jittered Shadows; Shadow Rays, Steps and Resolution; Volumetric Shadows and their Steps; Light Threshold; Shadow Pool; Light Probes Volume Pool; High Quality Normals; the whole Raytracing panel (Method, Resolution, Precision, Thickness, Denoising, Spatial Reuse, Temporal Accumulation, Bilateral Filter) and EEVEE's own Fast GI Approximation group; the volumetric settings. All properties of `SceneEEVEE` — <https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html>, read 2026-07-30.

**Belong to Workbench alone**: the anti-aliasing method for final render, and High Quality Normals. Workbench Sampling page, read 2026-07-30.

A wrinkle worth naming: Cycles is itself an add-on, not part of Blender's core. **Observed**:

> ```
> bl_info = {
>     "name": "Cycles Render Engine",
>     "author": "",
>     "blender": (2, 80, 0),
>     "description": "Cycles renderer integration",
> ```
> …
> ```
> class CyclesRender(bpy.types.RenderEngine):
>     bl_idname = 'CYCLES'
>     bl_label = "Cycles"
> ```

— `Blender Sources/cycles_5.1/__init__.py`, lines 7–11 and 42–44, read 2026-07-30. And the API documentation's `RenderSettings.engine` entry gives the type as:

> "engine
>
> Engine to use for rendering (default 'BLENDER_EEVEE')
>
> Type:
>
> Literal['BLENDER_EEVEE']"

— <https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html>, read 2026-07-30. That listing shows only `BLENDER_EEVEE`, because Cycles registers itself as an add-on and so is not in the built-in list. The manual's command-line page says the way to find out what a given machine offers is to ask it: "Use -E help to list available engines." **Documented.** There is also a read-only flag: "has_multiple_engines / More than one rendering engine is available (default False, readonly)" — same API page.

**NOT COVERED.** This answer covers the three engines the manual names. The manual also says "More renderers from third-party developers are available as add-ons", and nothing here says anything about those or about what settings they might add. It also does not settle what a Cycles-only setting does when the engine is EEVEE (whether it is ignored, hidden, or simply idle) — the sources do not say.

**Outside the interface.** The split is a property of where the settings are stored, not of whether a window is open, so it holds — **inferred**, one step from the storage locations above. The one thing that changes outside the interface is that the panels are not drawn, so the only way to see which settings an engine offers is through the Python properties rather than through the panels.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Operating system.** The engine split itself carries no operating-system condition in the sources. What does carry one is EEVEE's "Headless rendering is not supported on headless Windows systems." (EEVEE Limitations page, read 2026-07-30) and which Cycles backends exist per operating system (quoted under question 1). **Documented.**

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Where I looked.** Compared the three Performance pages and the three Sampling pages side by side. Read `manual_5.1/render/introduction.txt`, `manual_5.1/render/cycles/render_settings/index.txt`, `manual_5.1/render/eevee/limitations/limitations.txt`, `manual_5.1/render/workbench/*`. In the API documentation read the property lists of `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE` and the `cycles`/`eevee`/`render` entries of `bpy.types.Scene`. In Cycles' files, enumerated every property of `CyclesRenderSettings` and `CyclesPreferences` in `properties.py` and every `CYCLES_RENDER_PT_*` panel class in `ui.py` to see which settings each panel reads from `scene.render` and which from `scene.cycles`. Searched the API documentation folder for a `CyclesRenderSettings` page — there is none (only `bpy.ops.cycles.txt` and an unrelated `bpy.types.FModifierCycles.txt`), which is consistent with Cycles being an add-on.

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

**Reading I answered.** "Out of a file" = out of a .blend file. **The other reading** would be Blender's own configuration files on disk, which the sources describe only as directories (`BLENDER_USER_CONFIG`, "Directory for user configuration files" — Command Line Arguments page), never as a readable format.

**LABEL: mixed. Documented** that these are readable Python properties, and that Blender can be run over a file without an interface to run a script. **Inferred**, one step, that this amounts to reading the values out of a file. **Not in the sources** for reading a .blend without running Blender.

### What is documented

Every setting in question 1 is a named property on a scene, and the API documentation lists each with its type and default — for example "threads / Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) (in [1, 1024], default 1)", "resolution_percentage / Percentage scale for render resolution (in [1, 32767], default 100)", "use_persistent_data / Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage (default False)" (<https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html>); and "taa_render_samples / Number of samples per pixel for rendering (in [1, inf], default 64)" (<https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html>). Both read 2026-07-30.

The scene's pointers to them are marked readable:

> "cycles
>
> Cycles render settings (readonly)"
>
> "eevee
>
> EEVEE settings for the scene (readonly)"

— <https://docs.blender.org/api/5.1/bpy.types.Scene.html>, read 2026-07-30. ("readonly" there marks the pointer, not the settings inside it; Cycles' own preset file writes to `bpy.context.scene.cycles.tile_size` and the rest, quoted under question 3, so the values within are writable.)

Blender can be run over a file with a script and no interface:

> "blender myscene.blend --background --python myscript.py"

and, on what to do with what the script finds:

> "If the results can be displayed as text then print them or write them to a file."

— Blender Python API, Tips and Tricks, <https://docs.blender.org/api/5.1/info_tips_and_tricks.html>, read 2026-07-30. The matching arguments are documented on the Command Line Arguments page: "-b, --background / Run in background (often used for UI-less rendering)." and "-P, --python <filepath> / Run the given Python script file." and "--python-expr <expression> / Run the given expression as a Python script.", read 2026-07-30.

There is also a documented way to pull data out of *another* .blend while a different one is open:

> "load(filepath, *, link=False, pack=False, relative=False, set_fake=False, recursive=False, reuse_local_id=False, assets_only=False, clear_asset_data=False, create_liboverrides=False, reuse_liboverrides=False, create_liboverrides_runtime=False)
>
> Returns a context manager which exposes 2 library objects on entering.
> Each object has attributes matching bpy.data which are lists of strings to be linked."

with the documentation's own example:

> "# Load a single scene we know the name of.
> with bpy.data.libraries.load(filepath) as (data_src, data_dst):
> data_dst.scenes = ["Scene"]"

— Blender Python API, <https://docs.blender.org/api/5.1/bpy.types.BlendDataLibraries.html>, read 2026-07-30. Since scenes can be loaded this way, and the render settings hang off a scene, this is a documented route to another file's settings — though the API documentation itself warns it is not a full substitute:

> "In some cases the library API may be an alternative to starting separate processes,
> although this API operates on reading and writing ID data-blocks and isn't
> a complete substitute for loading .blend files"

— Blender as a Python Module page, read 2026-07-30.

### What the sources do not offer

**Not in the sources**: any way to read a .blend's render settings *without running Blender*. There is no documented file-format description, no reader library, and no command that prints settings. The nearest things are `--cycles-print-stats`, which "Log statistics about render memory and time usage" — that is a report from a render that has run, not a reading of settings — and `-c, --command <command>`, whose only documented subject is extensions.

Two things are **not in the file at all**, so no amount of reading the file will find them: the Cycles compute device type and the per-device tick-boxes, which are marked "# Properties saved in preferences" in `Blender Sources/cycles_5.1/properties.py` line 1562 (**observed**, read 2026-07-30), and the shader-compilation and memory settings on the Preferences ‣ System page.

**NOT COVERED.** This answer does not settle whether the values you read back are the ones the render will use. Three documented things stand between them: arguments given at launch can override what is in the file (`--cycles-device` "overriding user preferences and the scene setting", `-E`, `-t`); preferences supply the Cycles backend and are ignored entirely when Blender is used as a Python module ("The Python module behaves as if --factory-startup was passed"); and a file may hold several scenes, only one of which is rendered ("-S, --scene <name> / Set the active scene <name> for rendering"). It also does not settle what happens with values written by an older Blender — Cycles ships a `version_update.py`, which I did not read past its name, so I make no claim about it. And it does not settle whether every setting is stored in the .blend at all: the sources never state which properties are saved, and I found no page that does.

**Outside the interface.** This is the case that works best outside the interface: the documented route is `blender myscene.blend --background --python myscript.py`, which is outside the interface by definition. **Documented.** The exception is the Python-module route, where "Functionality controlled by command line arguments … isn't accessible" and preferences are ignored (Blender as a Python Module page, read 2026-07-30).

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Operating system.** The sources give no operating-system condition on reading properties or on `--background`, `--python` or `bpy.data.libraries.load`. The nearest relevant note is a general one about where Blender's Python comes from: "The Blender releases distributed from blender.org include a complete Python installation on all platforms" (Tips and Tricks page, read 2026-07-30). So: the sources do not say that this differs by operating system, and I found nothing suggesting it does.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Where I looked.** Read `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE`, `bpy.types.Scene`, `bpy.types.BlendDataLibraries`, `info_advanced_blender_as_bpy.txt` and `info_tips_and_tricks.txt` in the 5.1 API documentation. Searched `manual_5.1` and `api_5.1` for: "blendfile module", "inspect a blend", "read a blend-file without", "blend-file structure", "libraries.load", "temp_data", "print", "dump", "query" — nothing describing a reader outside Blender. Read the whole Command Line Arguments page looking for a print-or-report argument: the only candidates are `--cycles-print-stats`, `-v/--version`, `-h/--help`, `-E help` and `--log-list-categories`, none of which reports scene settings. Read `extension_arguments.txt` in full to rule out `--command`. Checked `Blender Sources/cycles_5.1/properties.py` for where the preferences boundary falls.

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


## Things a person with Blender open could settle in a few minutes

This list has been pruned three times. Everything it held that a rented Linux machine could settle was run on 2026-08-06 and is written into the answers above, under the question each one answers. Items 6 and 7, which were left waiting for a working graphics card, were run on 2026-08-07, once that machine's card had been repaired. The one list covering every version, with the full procedure for each test and the results, is `Audits/Blender Research/What a render demands/Hand tests.md`. **A third sitting, on the Creator's own Mac on 2026-08-07, ran every item that was still owed there, including the two that need a window — which no machine on this project but that one has.** Only genuinely open items are left here.

**Not run.**

12. **Does `--qos eco` change anything measurable?** Needs Windows with a hybrid processor. No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

13. **Does EEVEE render at all with no display attached?** Answered on Linux (see question 4 above). The Windows half was not run, for the same reason.

15. **What does Blender-as-a-module do with the thread setting?** The module would have to be fetched from outside, with no way to be sure the one that arrives is the same build as the four this project researches, so a result from it would not speak for any of them.

**Run and answered above:** 1 (`-t`), 2 (`-E`), 3 (`blender -E help`), 4 (`--python-expr` setting a setting with no argument), 5 (which settings survive a save), 8 and 9 (Use Tiling and Tile Size), 10 (Render Single Layer), 11 (the Viewport settings), 14 (reading another file's settings), **6 (is the Cycles device really outside the file)** and **7 (`--cycles-device` against both)**.

On 6, a second machine turned out not to be needed: two settings folders on the one machine do the same job. The file's Device is stored in the file and reads back as saved; the backend comes from the preferences; a file asking for GPU Compute renders on the processor when the preference names no backend, and on the card when it names one. On 7, the argument overrides both — Blender does not exit, and the statistics still name no device, so the evidence is the timing and what the process holds on the card.

**All of that has now been run, on the Creator's own Mac on 2026-08-07, and is recorded above under the question each one answers.** Apple's own graphics render a different picture from the processor, exactly as the NVIDIA card does; `--cycles-device METAL` beats both the file and the saved preferences, in both directions; the three engines really do show different Performance panels; and the Acceleration Structure controls really do change with the device. **Nothing on this subject is still owed on either machine.**

Three tests were deliberately **not** repeated on the Mac, and the reason is written here rather than left as a silence: the ones whose only possible evidence is a time or a memory figure — whether `-t` changes how much of the processor a Cycles render uses, whether the thread count changes anything under EEVEE or Workbench, and whether Persistent Data changes the time or the memory of an animation. The Mac was shared with other topics' work throughout, so a figure taken there would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. What they leave open on macOS is left open and is not guessed at.

---

2026-08-06 — Opus 5 (Claude Code) — What a render demands, Blender 5.1

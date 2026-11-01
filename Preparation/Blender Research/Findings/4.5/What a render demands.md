# What a render demands — Blender 4.5

**Sources consulted, and when.** Blender's 4.5 manual, Blender's 4.5 Python API documentation, Blender's 4.5 release notes and Cycles' own add-on files, all read on 2026-07-30 (the date the copies in `Blender Sources/` were taken). Blender's requirements page, read on 2026-08-01 (the date in its file name).

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

Two notes, so that line can be trusted rather than just believed:

- I listed the names of the files in the folder I was told to save into, in order to create that folder. I did not open any of them.
- I did not open the project's `CLAUDE.md`, but its text was placed into my working context automatically by the tool I was run with, before I did anything. It is a signpost: it says it contains no rules of its own, and it tells Blender researchers to stop and read nothing else. It makes no claim about how Blender behaves, so nothing below can be an echo of it. I did not follow it to `Preparation/Groundwork/Start Here.md` or anywhere else.
- I read `Blender Sources/About this folder.md`, which is inside the folder I was allowed to read. It describes what the folder holds. It links to a document in `Preparation/`; I did not follow that link.

---

## 1. What settings, if any, change how much a render demands of the computer it runs on?

**LABEL: documented** (for the named settings and the demand each is said to change), with **observed** additions from Cycles' own files where noted.

The sources treat this as a named subject. There is a panel called **Performance** in each of the three render engines, and the Cycles one opens by saying what the panel is for:

> "Properties that affect render speeds or memory consumption."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html, read 2026-07-30

Below is what the sources put in that category, grouped by where the setting lives. I have kept to settings the sources themselves connect to speed, memory or how much of the machine is used. I have not listed every render setting.

### 1a. Settings that belong to Blender itself, not to one engine

These live on `bpy.types.RenderSettings` — reachable in the Python API as a scene's `render`.

**Threads.** Two settings, drawn in the Cycles Performance panel but read from Blender's own render settings.

> "Threads Mode / Method to determine the maximum number of CPU cores to use while rendering. / Auto-Detect: Automatically chooses the amount of threads to match the number of logical processors on your computer. / Fixed: Manually choose the maximum number threads to use for rendering. This can be useful for example, if you want to use your computer while rendering, you can set the property to a thread count lower the amount of logical processors on your computer. / Threads / The maximum number of CPU cores to use simultaneously while rendering."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html, read 2026-07-30

> "threads / Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) / Type: int in [1, 1024], default 1"
> "threads_mode / Determine the amount of render threads used / AUTO Auto-Detect – Automatically determine the number of threads, based on CPUs. / FIXED Fixed – Manually determine the number of threads. / Type: enum in ['AUTO', 'FIXED'], default 'AUTO'"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30

That these two are Blender's own and not Cycles' is **observed** in Cycles' files: the panel reads them off `scene.render`, not off `scene.cycles`.

> ```
> class CYCLES_RENDER_PT_performance_threads(CyclesButtonsPanel, Panel):
>     bl_label = "Threads"
>     bl_parent_id = "CYCLES_RENDER_PT_performance"
> ...
>         rd = scene.render
> ...
>         col.prop(rd, "threads_mode")
>         sub = col.column(align=True)
>         sub.enabled = rd.threads_mode == 'FIXED'
>         sub.prop(rd, "threads")
> ```
> — Cycles' own files, `ui.py`, lines 829–846, read 2026-07-30

**Persistent Data.**

> "Persistent Data / Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage while performing other tasks in Blender. / When using multiple View Layers, only data from a single view layer is preserved to keep memory usage within bounds; however, objects shared between view layers are preserved."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html, read 2026-07-30

> "use_persistent_data / Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage / Type: boolean, default False"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30

Again **observed** as Blender's own, not Cycles': `col.prop(rd, "use_persistent_data", text="Persistent Data")` — Cycles' own files, `ui.py`, line 931, read 2026-07-30.

**Image size.** Resolution and the percentage scale set how many pixels get rendered.

> "resolution_x / Number of horizontal pixels in the rendered image / Type: int in [4, 65536], default 1920"
> "resolution_y / Number of vertical pixels in the rendered image / Type: int in [4, 65536], default 1080"
> "resolution_percentage / Percentage scale for render resolution / Type: int in [1, 32767], default 100"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30

> "Percentage / Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/output/properties/format.html, read 2026-07-30

**Render Region.** Renders only part of the frame.

> "Allows you to limit rendering to a 2D rectangular area. If you're busy tweaking just a small part of the scene, it can be quite wasteful to have the whole viewport in Rendered shading mode or make full-frame renders, so this feature lets you save time."
> — Blender manual, https://docs.blender.org/manual/en/4.5/editors/3dview/navigate/regions.html, read 2026-07-30

> "use_border / Render a user-defined render region, within the frame size / Type: boolean, default False"
> "use_crop_to_border / Crop the rendered frame to the defined render region size / Type: boolean, default False"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30

**Simplify.** A switch plus a set of ceilings, with separate render and viewport values.

> "Enable simplification of scene for quicker preview renders" (`use_simplify`)
> "simplify_subdivision_render / Global maximum subdivision level during rendering / Type: int in [0, 32767], default 0"
> "simplify_child_particles_render / Global child particles percentage during rendering / Type: float in [0, 1], default 0.0"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30

> "Texture Limit / Automatically scales textures down so that they are no larger than the values chosen. This can help reduce computer memory resources when rendering large scenes with huge textures."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/simplify.html, read 2026-07-30

Also under Simplify, culling:

> "Camera Cull / Automatically culls objects based on the camera frustum defined by the Margin. / Distance Cull / Automatically culls objects based on their distance from the active camera."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/simplify.html, read 2026-07-30

**High Quality Normals.** This is the whole of the Workbench Performance panel, and it is also the EEVEE one's first entry.

> "High Quality Normals / Uses higher precision normals and tangents which can improve visual quality for dense meshes with high frequency textures at the cost of memory."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/workbench/performance.html and https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/performance.html, read 2026-07-30

> "use_high_quality_normals / Use high quality tangent space at the cost of lower performance / Type: boolean, default False"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30

**Lock Interface.**

> "use_lock_interface / Lock interface during rendering in favor of giving more memory to the renderer / Type: boolean, default False"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30

**Compositing after the render.** The compositor's device, precision and denoise settings sit in the same Performance panel for both Cycles and EEVEE.

> "Compositor / Device / The device used for compositing. / CPU: Use the CPU for compositing. / GPU: Use the GPU for compositing. / Precision GPU / The precision of compositor intermediate result. / Auto: Use full precision for final renders, half precision otherwise. / Full: Use full precision for final renders and viewport."
> "Denoise Nodes / Denoising Device / The device to use to process Denoise nodes in the compositor. … Final Quality / The quality used by Denoise nodes during the final render if their quality is set to Follow Scene. / High: Produces the highest quality output at the cost of long processing times. / Balanced: Balanced between performance and quality, typically processing in half the time as High, while retaining most of the quality. / Fast: Produces an output quickly at a noticeable cost of quality."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html, read 2026-07-30

Whether the compositor runs at all is itself a switch: "use_compositing / Process the render result through the compositing pipeline, if compositing nodes are enabled" and "use_sequencer / Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist" — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

**Which engine.** `engine` on the render settings decides which engine renders. That the three engines carry different settings is set out in question 4; no source I read compares what one engine costs against another. The API page shows the property but lists only one value, which I take up in question 5.

> "engine / Engine to use for rendering / Type: enum in ['BLENDER_EEVEE_NEXT'], default 'BLENDER_EEVEE_NEXT'"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30

**Per view layer sample override.**

> "samples / Override number of render samples for this view layer, 0 will use the scene setting"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html, read 2026-07-30

### 1b. Settings that belong to Cycles

These live on `scene.cycles` and are defined in Cycles' own `properties.py`. Everything in this section is **observed** from those files unless a manual address is given.

**Device.**

> ```
> enum_devices = (
>     ('CPU', "CPU", "Use CPU for rendering"),
>     ('GPU', "GPU Compute",
>         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
> )
> ```
> — Cycles' own files, `properties.py`, lines 30–34, read 2026-07-30

> ```
>     device: EnumProperty(
>         name="Device",
>         description="Device to use for rendering",
>         items=enum_devices,
>         default='CPU',
>     )
> ```
> — Cycles' own files, `properties.py`, lines 378–383, read 2026-07-30

The manual adds what the choice buys and costs:

> "GPU rendering makes it possible to use your graphics card for rendering, instead of the CPU. This can speed up rendering because modern GPUs are designed to do quite a lot of number crunching. On the other hand, they also have some limitations in rendering complex scenes, due to more limited memory, and issues with interactivity when using the same graphics card for display and rendering."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30

**Tiling.**

> "Use Tiling / Render high resolution images in tiles to reduce memory usage. Tiles are cached to disk while rendering to save memory. / Tile Size / This value is used to control the size of the tile used for rendering; decreasing the size reduces memory usage."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html, read 2026-07-30

> ```
>     use_auto_tile: BoolProperty(
>         name="Use Tiling",
>         description="Render high resolution images in tiles to reduce memory usage, using the specified tile size. Tiles are cached to disk while rendering to save memory",
>         default=True,
>     )
>     tile_size: IntProperty(
>         name="Tile Size",
>         default=2048,
>         description="",
>         min=8, max=8192,
>     )
> ```
> — Cycles' own files, `properties.py`, lines 1051–1061, read 2026-07-30

**Acceleration structure (BVH).** Four settings, all speed-against-memory.

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
> — Cycles' own files, `properties.py`, lines 920–940, read 2026-07-30

**Samples and time limit.**

> "Render (Max) Samples / The number of paths to trace per pixel in the final render. A higher number results in a cleaner image at the cost of a longer render time. / Time Limit / Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```
> — Cycles' own files, `properties.py`, lines 482–487, read 2026-07-30

> ```
>     time_limit: FloatProperty(
>         name="Time Limit",
>         description="Limit the render time (excluding synchronization time). "
>         "Zero disables the limit",
>         min=0.0,
>         default=0.0,
> ```
> — Cycles' own files, `properties.py`, lines 517–522, read 2026-07-30

**Adaptive sampling.**

> "If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling, cutting short the sampling process in areas that have become less noisy than the specified threshold value – in other words, not wasting time further refining areas that already look good enough."
> "Noise Threshold / … Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30

**Sample subset.** This one exists precisely to split one frame's demand across machines.

> "use_sample_subset … description="Render a subset of the specified max samples. Typically used for distributed rendering across multiple devices""
> — Cycles' own files, `properties.py`, lines 497–501, read 2026-07-30

> "A typical use case is to distribute the rendering of a single frame over multiple machines. Say you want to render 2048 samples in total, but split this work over two machines because doing it on one would take too long"
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30

**Denoising, and where it runs.**

> "Use GPU / Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled."
> "Quality OpenImageDenoise / … High: Produces the highest quality at the cost of time. … Fast: Sacrifices quality for speed (ideal for viewport rendering)."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30

**Bounce ceilings.**

> "Max Bounces / Total / Maximum number of light bounces. For best quality, this should be set to the maximum. However, in practice, it may be good to set it to lower values for faster rendering."
> "Light paths are terminated probabilistically when specifying a minimum number of light bounces lower than the maximum. … This will still converge to the same image, but renders faster while possibly being noisier."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/light_paths.html, read 2026-07-30

**Light threshold and light tree.**

> "Light Tree / Use a light tree to more effectively sample lights in the scene, taking into account distance and estimated intensity. This can significantly reduce noise, at the cost of a somewhat longer render time per sample."
> "Light Threshold / Probabilistically terminates light samples when the light contribution is below the threshold. This avoids wasting time on lights that contribute little to the image."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30

**Scrambling Distance.**

> "A technique that reduces the randomness between pixels in an attempt to improve GPU rendering performance, at the cost of potential artifacts."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30

**Volume step rate and max steps.**

> "Render time can be reduced by increasing the step size, at the cost of potentially losing some volume detail."
> "Max Steps / Maximum number of steps through the volume before giving up, to protect from extremely long render times with big objects or small step sizes."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/volumes.html, read 2026-07-30

**Fast GI approximation.**

> ```
>     use_fast_gi: BoolProperty(
>         name="Fast GI Approximation",
>         description="Approximate diffuse indirect light with background tinted ambient occlusion. "
>                     "This provides fast alternative to full global illumination, for interactive viewport rendering or final renders with reduced quality",
>         default=False,
>     )
> ```
> — Cycles' own files, `properties.py`, lines 1023–1028, read 2026-07-30

**Render texture limit.**

> ```
>     texture_limit_render: EnumProperty(
>         name="Render Texture Limit",
>         default='OFF',
>         description="Limit texture size used by final rendering",
>         items=enum_texture_limit
>     )
> ```
> — Cycles' own files, `properties.py`, lines 1016–1021, read 2026-07-30

**Adaptive subdivision dicing** (experimental feature set only).

> "Offscreen Scale / Multiplier for dicing rate of geometry outside of the camera view. … Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/subdivision.html, read 2026-07-30

**Undocumented debug flags in Cycles' files.** These exist as properties but I found no manual page for them, so I record them as **observed** only:

> ```
>     debug_use_cpu_avx2: BoolProperty(name="AVX2", default=True)
>     debug_use_cpu_sse42: BoolProperty(name="SSE42", default=True)
>     debug_bvh_layout: EnumProperty(
>         name="BVH Layout",
>         items=enum_bvh_layouts,
>         default='EMBREE',
>     )
> ```
> — Cycles' own files, `properties.py`, lines 1070–1076, read 2026-07-30

What is **observed** there is only that these properties exist, under these names, with these defaults. Nothing quoted says any of them changes what a render demands, and no source I read describes what a render does with them. They are listed here because they sit among the settings above, not because anything establishes their effect.

**What Cycles counts as a "performance" setting.** Cycles' own preset code names exactly which settings a Performance preset stores, which is the closest thing in the sources to Cycles' own list:

> ```
>     preset_values = [
>         "render.threads_mode",
>         "render.use_persistent_data",
>         "cycles.debug_use_spatial_splits",
>         "cycles.debug_use_compact_bvh",
>         "cycles.debug_use_hair_bvh",
>         "cycles.debug_bvh_time_steps",
>         "cycles.use_auto_tile",
>         "cycles.tile_size",
>     ]
> ```
> — Cycles' own files, `presets.py`, lines 102–111, read 2026-07-30

The manual names three such presets, though it does not give their values:

> "There are several presets available to help choose between different trade offs: / Default: Balances memory saving and faster rendering settings. / Faster Render: Uses settings to render faster at the cost of higher memory consumption. / Lower Memory: Uses settings to decrease memory considered at the cost of slower renders."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html, read 2026-07-30

### 1c. Settings that belong to EEVEE

These live on `scene.eevee`.

> "Shadow Pool / A bigger pool size allows for more shadows in the scene but might not fit into GPU memory and decreases performance. Increasing the size might fix the Shadow buffer full error. / Light Probes Volume Pool / A bigger pool size allows for more irradiance grids in the scene but might not fit into GPU memory and decreases performance."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/performance.html, read 2026-07-30

> "Render / Samples / The number of samples to use in the final render."
> "Shadows / Rays / Number of rays to trace for each light. … Steps / Number of shadow map sample per shadow ray. Higher step count results in softer shadows but have a higher cost."
> "Volumetric Shadows / Approximate light absorption of the surrounding volume objects. … This is a very computationally expensive option and has limitations."
> "Light Threshold / Minimum light intensity for a light to contribute to the lighting. Used to compute the distance at which to cut-off lights influence. Lower values improve performance."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html, read 2026-07-30

> "Resolution / Resolution at which the ray-tracing is performed. Lower options will be faster and use less memory but will produce blurrier results."
> "Precision / Higher values increase precision of the screen space ray-tracing but lower the maximum trace distance. Increased precision also increases performance cost."
> "Method / … Light Probe: … This option has the lowest tracing cost but relies on manually placed light-probes."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/raytracing.html, read 2026-07-30

> "To achieve this, EEVEE uses several 3D textures which have a high video memory usage. The texture dimensions can be tweaked using the Resolution and Steps parameters."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/volumes.html, read 2026-07-30

### 1d. Settings that belong to Workbench

> "Multisample / The scene will be rendered multiple times with a slight offset. The anti-aliasing will be gathered from the multiple renders. The number of samples are predefined so it uses the best distribution of the samples. / 5, 8, 11, 16, 32"
> "The setting for viewport rendering and final rendering is saved per scene."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/workbench/sampling.html, read 2026-07-30

Workbench's own Performance panel holds only High Quality Normals (quoted in 1a).

### 1e. Settings that are not in the scene at all — they are in Preferences

These change what a render demands but are not part of the file being rendered. They are named in Preferences → System.

> "Cycles Render Device / Changes the computing device the Cycles render engine uses to render images."
> "Distribute Memory Across Devices / Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes."
> "Embree on GPU / Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance."
> "HIP RT / Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above."
> "MetalRT / MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases."
> — Blender manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html, read 2026-07-30

Cycles' own files add one more that the manual page I read does not name, kernel optimization, and it says outright that it costs CPU time in the background:

> ```
>     kernel_optimization_level: EnumProperty(
>         name="Kernel Optimization",
>         description="Kernels can be optimized based on scene content. Optimized kernels are requested at the start of a render. "
>                     "If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set "
>                     "is available in the cache. This can result in additional CPU usage for a brief time (tens of seconds)",
>         default='FULL',
>         items=(
>             ('OFF', "Off", "Disable kernel optimization. Slowest rendering, no extra background CPU usage"),
>             ('INTERSECT', "Intersection only", "Optimize only intersection kernels. Faster rendering, negligible extra background CPU usage"),
>             ('FULL', "Full", "Optimize all kernels. Fastest rendering, may result in extra background CPU usage"),
>         ),
>     )
> ```
> — Cycles' own files, `properties.py`, lines 1624–1635, read 2026-07-30

And, still in Preferences, the shader compilation settings and the memory limits:

> "Shader Compilation Method / Defines the method used for compiling GPU shaders in parallel. … Thread: Uses multiple threads within a single process to compile shaders concurrently. This method is more memory-efficient but may be slower on some systems. / Subprocess: Uses multiple separate subprocesses to compile shaders in parallel. This can be faster, especially on systems with high core counts, but consumes significantly more RAM."
> "Threads / Subprocesses / The number of shader compilation threads or subprocesses to use. The maximum value is limited by the number of logical CPU cores on the system. / Increasing the number can reduce shader compilation time at the cost of higher memory usage. A value of 0 lets Blender automatically choose a suitable number based on the system configuration."
> "Undo Memory Limit / Maximum memory usage in Mb (0 is unlimited)."
> "Memory Cache Limit / Upper limit of the Video Sequencer and Movie Clip Editor memory cache (in megabytes)."
> — Blender manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html, read 2026-07-30

I also opened the picture on that page, `_images/editors_preferences_section_system.png` (Blender manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html, read 2026-07-30). **documented** from what is visible in it: the System section is laid out as headed blocks, one of which is "Cycles Render Devices" (shown collapsed) and one "Memory & Limits", showing Undo Steps 32, Undo Memory Limit 0, Global Undo ticked, Console Scrollback Lines 256, Texture Time Out 120, Garbage Collection Rate 60, Vbo Time Out 120, Garbage Collection Rate 60; below that "Video Sequencer" with Memory Cache Limit 4096, Use Disk Cache unticked, Cache Limit 100, Compression None, Proxy Setup Automatic. The picture does not show the Display Graphics, Network or Shader Compilation items the page's text describes.

### What is NOT there

I searched for a setting that caps how much memory a render may use, and for anything that sets the process's priority or which cores it is pinned to. There is none in the sources. Searches run across the whole manual and API text: `memory limit`, `RAM limit`, `max memory`, `memory cap`, `priority`, `affinity`, `nice value`, `process priority`. The only memory ceilings that turned up are the undo memory limit and the video sequencer cache limit, neither of which is about the render. The nearest the manual comes is advice about the operating system, not a Blender setting:

> "Rendering takes all available CPU time; you should render overnight, when the computer is not needed, or set Blender to a low priority while rendering, and work on other things (be careful with the RAM space!)."
> — Blender manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html, read 2026-07-30

**Not covered.** How much any of these actually shifts the load on a given machine. No source I read gives a figure, a formula or a measurement. The list is also not closed: I have listed what the sources tie to speed or memory, and there will be scene content settings (geometry, shaders, texture sizes) that change demand without ever being described as performance settings.

**Outside the interface.** These are scene settings and Preferences settings, and neither is described as behaving differently when Blender is driven from outside its window. Two things I did find that differ. First, the manual states that Preferences still apply: "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them" (Blender manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html, read 2026-07-30). Second, one render setting is explicitly ignored: "use_single_layer / Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line." (Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30). Beyond that: every setting above whose name or description says "Viewport" or "Preview" governs the interactive viewport, which does not exist in a background render; the sources do not say what those settings do in that case, and I will not guess.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

**Operating system.** Most of the list is not tied to an operating system. The device settings are. "CUDA is supported on Windows and Linux", "OptiX is supported on Windows and Linux", "HIP is supported on Windows and Linux", "oneAPI is a computation library that is supported on Windows and Linux", "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." (Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30). Shader compilation: "This option is not available on macOS and requires using OpenGL backend." (Blender manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html, read 2026-07-30). MetalRT is Apple-only and HIP RT is AMD-only by the same page's wording. On the machines themselves, the requirements page names 4.5 in so many words: "Blender 4.5 LTS supports NVIDIA GeForce 400 and newer. AMD GCN 1st gen and newer. Intel Broadwell architecture and newer." (Windows and Linux entries) and "Blender 4.5 LTS supports Intel chips, macOS 11.2 (Big Sur), requires GPU with Metal 2.2 (AMD, Apple Silicon, or Intel Skylake and newer)" — https://www.blender.org/download/requirements/, read 2026-08-01.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/4.5/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 3 — whether `-t` changes how much of the processor a Cycles render actually uses.** Done to a file built from a script and saved: Cycles, Device CPU, 320 by 240, 256 samples, seed 7, denoising off, adaptive sampling off, Threads Mode Auto-Detect; and to a copy of it with Threads Mode Fixed and Threads 1. Six runs of `/usr/bin/time -v blender -b --factory-startup <file> -f 1 -o <folder>/x_ -F OPEN_EXR -x 1`, varying only `-t`. What came back:

```
auto-detect, no -t : Percent of CPU 1555%, wall 0:02.06, peak 419484 kb
-t 1               : Percent of CPU 100%,  wall 0:30.82, peak 297680 kb
-t 2               : Percent of CPU 196%,  wall 0:15.76, peak 295188 kb
-t 0               : Percent of CPU 1500%, wall 0:02.19, peak 425080 kb
Fixed 1 in the file: Percent of CPU 2080%, wall 0:35.83, peak 406928 kb
Fixed 1 with -t 8  : Percent of CPU 718%,  wall 0:04.40, peak 332060 kb
```

**Observed.** The argument reaches the renderer: one thread takes about fifteen times as long as all of them, two threads about half of that, and `-t 0` matches the unrestricted run. The file's own Fixed 1 also reaches it. Note what the processor percentage does and does not show here: with the file asking for one thread, this version reported 2080% while still taking the single-thread wall-clock time.

**Hand test 15 — which of the settings named in one version's findings exist in the others.** Done by walking `bl_rna.properties` of `scene.render`, `scene.cycles`, `scene.eevee`, `scene.display` and the Cycles preferences in all four researched versions and comparing the five lists. This version's dump holds 286 properties. Every name that is not in all four:

```
Present in 4.5 and in none of 5.0, 5.1, 5.2:
  scene.cycles.feature_set
  scene.eevee.gtao_distance, scene.eevee.gtao_quality, scene.eevee.use_gtao
  scene.render.bake_bias, bake_margin, bake_margin_type, bake_samples, bake_type, bake_user_scale,
    use_bake_clear, use_bake_lores_mesh, use_bake_multires, use_bake_selected_to_active, use_bake_user_scale
Absent from 4.5 and present in 5.2 only:
  scene.render.use_texture_cache, scene.render.use_auto_generate_texture_cache,
  scene.cycles.texture_resolution, scene.cycles.texture_resolution_render,
  scene.cycles.debug_use_texture_cache_eviction, scene.cycles.debug_texture_cache_preserve_unused,
  scene.render.anisotropic_filter, scene.render.save_output, scene.cycles.use_pixel_jitter
Absent from 4.5 and present in 5.0, 5.1 and 5.2:
  scene.cycles.volume_biased
Absent from 4.5 and present in 5.1 and 5.2:
  scene.eevee.direct_light_intensity, scene.eevee.indirect_light_intensity
Values that differ: scene.render.compositor_device reads 'CPU' here and 'GPU' in 5.2.
  The Cycles preference use_hiprt reads False here and in 5.0, and True in 5.1 and 5.2.
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

**Hand test 7 — what Blender does when the file asks for the graphics card and none can be reached.** Done to a file saved with Cycles as the engine and Device set to **GPU Compute**, on this machine whose card was unreachable. `blender -b --factory-startup gpu.blend -o <folder>/g_ -F OPEN_EXR -x 1 -f 1 -- --cycles-print-stats`, with the Cycles backend preference at its factory value of None. **Exit code 0, and `g_0001.exr` written, 309878 bytes.** A search of the whole output for device, CUDA, OptiX, GPU, error, warning, fall and CPU returned only ordinary progress lines — nothing named a missing card. *Flagged by the citation check, 2026-08-07:* **this half of the test could not have failed in the direction that matters.** "Blender did not complain" and "Blender did not use the card" are different things, and searching printed output cannot tell them apart — hand test 26 establishes that Blender never names the device that rendered, which is exactly why looking for it cannot work. What settles the point is which picture came out, and the citation check ran that on the repaired card: a file saved with Device GPU Compute, in a settings folder naming no backend, matched the processor render exactly (`IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`) and did not match the card render. So the conclusion holds, but it rests on hand test 28 and not on this. Sent to the hand tests to be rebuilt as a picture comparison. Repeated with `CUDA` saved in the preferences, the run again exited 0 and wrote the image, and printed one line about it:

```
CUDA cuInit: Unknown error
```

**Observed.** A render started from outside, asking for the graphics card on a machine that has none it can use, produces the image and — unless the preference names a backend — says nothing at all.

**Hand test 19 — whether a Cycles Time Limit produces a usable image.** Done to a 640 by 480 Cycles scene at 64 samples. Rendered once with Time Limit 0, then again with `--python-expr "import bpy; bpy.context.scene.cycles.time_limit=0.3"`. What came back: no limit `Time: 00:01.40`; limit `Time: 00:00.40`. Both files 4923638 bytes. Compared by pixels, the two are not the same picture: `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.35095858573913574`.

**Observed.** A time limit writes a complete file of the usual size, holding a picture cut short rather than a blank or a missing one.

**Hand test 24 — whether Persistent Data changes the time or the memory of an animation.** Done by rendering the whole ten frames with `-a`, once with `use_persistent_data` off and once on, under EEVEE and under Cycles, timed by the operating system. One run of each, on a shared machine:

```
EEVEE, ten frames, Persistent Data off : 68.89 s wall, 463% cpu, 2226244 kb peak
EEVEE, ten frames, Persistent Data on  : 73.29 s wall, 411% cpu, 2230412 kb peak
Cycles, ten frames, Persistent Data off: 21.46 s wall, 1639% cpu, 473976 kb peak
Cycles, ten frames, Persistent Data on : 21.25 s wall, 1642% cpu, 449908 kb peak
```

**Observed.** On a scene this small and a machine this busy, one run of each cannot separate the time differences from noise. What can be said is that peak memory did not go up when Persistent Data was switched on; under Cycles it came out lower.

**Hand test 16, cost side — what tile size and the acceleration-structure options cost.** Same 640 by 480 scene, one run each, shared machine:

```
base      3.31 s wall,  987% cpu, 438308 kb peak
tile 64   4.45 s wall,  861% cpu, 504600 kb peak
tile 8192 2.66 s wall, 1214% cpu, 441760 kb peak
tiling off 2.81 s wall, 1149% cpu, 438412 kb peak
compact BVH 2.57 s wall, 1273% cpu, 444736 kb peak
spatial splits 3.09 s wall, 1055% cpu, 445596 kb peak
```

**Observed.** Recorded as measured; a single run each on a busy machine is not enough to separate small differences from noise.

**Hand test 31 — the texture cache.** Done to a copy of the 640 by 480 Cycles scene with three 1024 by 1024 PNG images written next to it and wired into three materials, 52 kB of image files in all. First a render with nothing changed: `2.52 s wall, 539568 kb peak-rss`. Then the two settings were asked for by name:

```
HAS-use_texture_cache False HAS-use_auto_generate_texture_cache False
```

**Observed.** Neither setting exists in this version, so the rest of the test cannot be carried out here. Hand test 15 above establishes the same by a different route.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 4.5.12 at `/opt/blender/4.5/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/4.5/` or `/root/render-demands/prefs-card-cuda/4.5/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 27, cost side — what the card costs and what the processor costs.** One file rendered three times, changing only which device did the work: once with Device CPU and the Cycles backend preference at None, once with Device GPU Compute and the backend CUDA, once with Device GPU Compute and the backend OptiX. In the two card runs only the card was ticked and the processor was left unticked. The file: Cycles, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, OpenEXR at full float with no compression. Timed by the operating system:

```
processor   : Cycles reported Time: 00:01.45, 1701% of a processor, peak memory 441336 kb
card, CUDA  : Cycles reported Time: 00:00.68, 92% of a processor, peak memory 571616 kb
card, OptiX : Cycles reported Time: 00:00.78, 78% of a processor, peak memory 770136 kb
```

**Observed.** Choosing the card rather than the processor changes what the render demands in three ways at once, all in the same direction in this version. It takes less time. It stops asking for the whole processor — about fifteen hundred per cent of one core drops to about one hundred per cent, which is to say the render stops being a processor job. And it asks for **more** ordinary memory, not less, because the card's own working memory is held in addition. This is one small scene on one busy machine; the shape is what is being recorded, not the seconds.

**Hand test 28, memory side — what the saved backend preference takes off the card even when the processor does the work.** Through every run of hand test 28 this Blender was watched for how much card memory it held, by asking the driver which processes held work on the card and how much each held — a reading about this process alone, so the other topic's Blender could not confuse it. On a heavier version of the same file, 1024 samples:

```
backend preference None, render on the processor : never held any card memory at all
backend preference CUDA, render on the processor : held card memory, peak 248 MiB
render on the card                               : held card memory, peak 1728 MiB
```

**Observed.** A saved backend preference costs card memory on its own. With the preference naming CUDA, this Blender took about 248 MB of the card for the whole length of a render that ran entirely on the processor; with the preference at None it took none. That is a cost the preference exacts even in a run where it makes no difference at all to which device renders.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/4.5/`.

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
libc++abi: terminating
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
EXIT=134
```

This version printed the whole list and then **stopped with `libc++abi: terminating` and exit code 134**, which is the operating system ending it rather than a clean quit. It printed the same list, and aborted the same way, without `-b` as well. On the rented Linux machine the same command printed nothing at all and never returned. So the command misbehaves in this version on both operating systems, but differently: on the Mac it answers first and then falls over; on Linux it hangs.

**Observed.** This build offers three engines and names EEVEE `BLENDER_EEVEE_NEXT`, the same name the rented Linux machine got from it.

**Hand test 15 — which settings exist on this operating system.** The same sweep as on Linux: every property of `scene.render`, `scene.cycles`, `scene.eevee`, `scene.display` and the Cycles preferences, read out of a running Blender and compared across all four researched versions on this machine. This version's dump holds **285** properties.

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

**Hand test 19 — what a Cycles Time Limit produces.** The 640 by 480, 64-sample scene rendered once with no limit and once with `time_limit` set to 0.3 seconds. Both files came out **4923677 bytes**, and the two pictures are not the same picture:

```
no limit vs a 0.3-second limit : IDENTICAL-PIXELS False MAX-ABS-DIFF 3.6875343322753906 NUM-DIFFERING-CHANNEL-VALUES 921600 of 1228800
```

**Observed.** A time limit writes a complete file of the usual size holding a picture cut short, not a blank one and not a missing one. How long each render took was **not measured**, because the machine was not idle; the Linux run carries the times.

**Hand test 31 — the texture cache.** A copy of the same scene with three 1024 by 1024 PNG images written beside it and wired into the three materials.

```
HAS-use_texture_cache False HAS-use_auto_generate_texture_cache False
```

**Observed.** Neither setting exists in this version on macOS either, so the rest of the test cannot be carried out here. The same file rendered twice with nothing changed gave `IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`, and no folder appeared beside the images.

**Three tests were not run on this machine at all, and the reason is the same for all three.** Hand test 3 (whether `-t` changes how much of the processor a Cycles render actually uses), hand test 23 (whether the thread count changes anything under EEVEE or Workbench) and hand test 24 (whether Persistent Data changes the time or the memory of an animation) have no evidence except a time or a memory figure. The Mac was shared with other topics' work throughout, so a figure taken here would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. **What they leave open on macOS is left open and is not guessed at.**

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 4.5.12 at `/opt/blender/4.5/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/4.5/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — which picture comes out when the file asks for the graphics card and the settings folder names no backend. This is hand test 7 rebuilt, because hand test 7 could not have failed.** That test searched Blender's printed output for complaints about a missing card and found none; "Blender did not complain" and "Blender did not use the card" are different things, and hand test 26 in this same document is why no search of output can tell them apart. **What changed is the evidence: the question is now settled by which picture came out.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render.

Five renders, none of them with `--factory-startup`, so the saved preferences were read every time: `blender -b <file> -o <folder>/<tag>_ -F OPEN_EXR -x 1 -f 1 --`. (a) the file saying CPU, no argument — the **processor reference**; (b) the same again, unchanged; (c) the file saying CPU with `-- --cycles-device CUDA` — the **card reference**; (d) the same again, unchanged; (e) **the test itself**, the file saying GPU Compute with the backend preference naming nothing.

The two controls first, and the size of the gap this has to judge against:

```
processor run 1 vs run 2 : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0                  DIFFERING 0 of 1228800
card run 1 vs run 2      : IDENTICAL-PIXELS False MAX-ABS-DIFF 2.86102294921875e-06    DIFFERING 473990 of 1228800
processor vs card        : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.07276105880737305     DIFFERING 608159 of 1228800
```

and then the test:

```
(e) file GPU Compute, backend preference NONE, no argument
    against the processor reference : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0              DIFFERING 0 of 1228800
    against the card reference      : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.07276105880737305  DIFFERING 608159 of 1228800
```

**Observed.** A file asking for GPU Compute, in a settings folder naming no backend, renders on the **processor** — not approximately but to the very same numbers — and misses the card by the whole device gap, which is some twenty thousand times the difference between two runs on the same device. The test could have come out the other way and did not. A second reading says the same without any reference to the pictures: what each run held on the card, this Blender alone.

```
              Cycles reported   % of a processor   card memory held by this run
processor        Time: 00:01.86      1448%          none at all
card (CUDA)      Time: 00:01.44      53%            1710 MiB
file GPU, no backend named   Time: 00:02.52   1099%   none at all
```

The wall-clock times were 0:02.31, 0:02.10 and 0:03.03; they are a busy machine's times and carry nothing on their own. **The card memory carries it: the run never touched the card.** The five renders were made twice, ten minutes apart, and both passes gave pixel-identical pictures (`IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`), so this is not one moment's accident. **Still owed on the Creator's Mac**, where the question is sharper because the graphics there work perfectly.

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
--cycles-device CUDA   exit 0, image written : matched the CARD reference, MAX-ABS-DIFF 3.337860107421875e-06; against the processor 0.07276105880737305
--cycles-device OPTIX  exit 0, image written : matched NEITHER — against the processor 0.07276058197021484, against the CUDA card 0.06922316551208496
--cycles-device HIP    exit 1, no image written
--cycles-device ONEAPI exit 1, no image written
--cycles-device METAL  exit 1, no image written
```

OptiX being a third picture is not a fault in the test: hand test 27 already found the two backends on the same card disagreeing with each other. What matters is that it is not the processor's picture. What the three refused runs printed, word for word:

```
Error: Found no Cycles device of the specified type
```

and `HIP` printed nothing extra in this version.

Card memory, this Blender alone: `CPU`, `HIP`, `ONEAPI` and `METAL` held **none at all**; `CUDA` held 1710 MiB and `OPTIX` 1710 MiB. (The card was sampled five times a second and these renders last about two seconds, so a peak can be missed — the reading that carries weight is nothing against something.)

**Observed.** Naming a backend this build accepts but has no hardware for does **not** quietly fall back to the processor: the render stops, writes no image, and says `Found no Cycles device of the specified type`. So a program that writes `HIP` or `ONEAPI` into this setting on this machine gets no error when it writes it, and finds out at render time. **Still owed on the Creator's Mac** for the device-listing half and the render half.

**Hand test 35 — the property sweep written out exactly, so its totals can be checked.** The citation check sent hand test 15 back over its numbers, not its finding: three sittings had reported three different totals and the procedure as written did not say whether each block's own `rna_type` entry is counted. The sweep was run again with that stated — `blender -b --factory-startup <file> --python <script>`, walking `bl_rna.properties` of five blocks named exactly (`scene.render`, `scene.cycles`, `scene.eevee`, `scene.display`, and the Cycles add-on's preferences), counting each block twice, once including `rna_type` and once excluding it. This version: `scene.render` 109 / 108, `scene.cycles` 106 / 105, `scene.eevee` 56 / 55, `scene.display` 10 / 9, the Cycles preferences 9 / 8 — **290 counting `rna_type`, 285 not counting it**.

**Observed.** The three totals on the record are now told apart. Counting `rna_type` gives 290, 276, 278 and 286 across the four versions, which is the citation check's figure for 4.5 exactly. Not counting it gives 285, 271, 273 and 281, which is the Mac sitting's figures exactly. The first Linux sitting's 286, 272, 274 and 282 are **neither** — one more than the not-counting figure in every version — so **they cannot be reproduced from any stated rule and should be read as decoration**. This version's earlier figure was 286. **The finding itself reproduced exactly**: the set of property names not present in all four versions came back as the same twenty-eight, on the same versions, `scene.eevee.fast_gi_thickness_far` among them — present in 4.5, 5.0 and 5.1 and absent from 5.2 — so the count and the written-out list now agree.



## 2. For each of them, what does it change about what the render produces?

**LABEL: documented** where the source says so; **not in the sources** for a large and important group, set out below.

The sources split into three kinds.

### 2a. Settings the sources describe as changing cost only, with no mention of the image changing

- **Threads / Threads Mode.** The manual describes it entirely as how many cores are used, and gives as its reason "if you want to use your computer while rendering". Nothing is said about the image.
- **Persistent Data.** Described only as keeping data in memory "for faster re-renders and animation renders at the cost of extra memory usage".
- **Use Tiling / Tile Size.** "Render high resolution images in tiles to reduce memory usage." Nothing about the image. The manual adds a speed note that cuts the other way: "In some cases changing the Tile Size can result in increased performance. For example when a small object renders slowly compared to other objects, using a small Tiles Size can lead to an increase in performance." — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html, read 2026-07-30
- **Use Spatial Splits, Use Curves BVH, Use Compact BVH, BVH Time Steps.** All four are described purely as build time, render speed and RAM. The manual adds for Curves BVH: "The bounding boxes are not axis aligned allowing a spatially closer fit to the curve geometry. Disabling this option will reduce memory, at the cost of increasing curve render time." — same page.
- **Cycles Device (CPU / GPU Compute), and the Preferences device choices.** Described as where the work runs. See the "not covered" note below.
- **Lock Interface.** "in favor of giving more memory to the renderer".
- **Sample Subset.** The manual says the point is to reconstruct one render from parts: "Multiple subset renders can be combined into a full one by running the following in the Python Console… Run the 1024-sample render on each machine, then combine the results as described above to get the equivalent of a single 2048-sample render." It also warns that the total must still be set correctly: "it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets – otherwise, the subsets will have incompatible noise and combining them will give a worse result." — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30
- **Kernel Optimization.** Described only as rendering speed and background CPU usage.
- **Denoise Use GPU.** Described as speed and GPU memory; nothing is said about the denoised result differing.

### 2b. Settings the sources describe as changing the image, and how

- **Resolution X, Y and Percentage.** Change the size of the image produced: "The number of pixels horizontally and vertically in the image" and "reduce or increase the size of the rendered image relative to the Resolution values".
- **Render Region / Crop to Render Region.** Limits rendering to a rectangle; with crop off, the rest is transparent: "Crops the rendered image to the size of the render region, instead of rendering a transparent background around it." — Blender manual, https://docs.blender.org/manual/en/4.5/render/output/properties/format.html, read 2026-07-30
- **Render (Max) Samples.** "A higher number results in a cleaner image at the cost of a longer render time."
- **Time Limit.** Stops early: "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet." (So a time-limited render is a partially-sampled one.)
- **Noise Threshold, Min Samples.** "lower values meaning better quality but longer render times".
- **Denoising, its quality and its passes.** "Denoising uses a specialized algorithm to get a less noisy image without requiring more samples." Quality: "High: Produces the highest quality at the cost of time. … Fast: Sacrifices quality for speed". Passes: "Generally, the more passes the denoiser has access to, the better the result. It is recommended to use at least Albedo as None can blur out details, especially at lower sample counts." — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30
- **Scrambling Distance.** "at the cost of potential artifacts" and "Not compatible with the Blue-Noise sampling pattern".
- **Max Bounces and the per-type bounces.** "A value of 0 bounces results in direct lighting only."
- **Min Light Bounces / Min Transparent Bounces.** "This will still converge to the same image, but renders faster while possibly being noisier."
- **Light Tree.** "This can significantly reduce noise, at the cost of a somewhat longer render time per sample", but with a stated blind spot: "Certain lighting properties are not accounted for in the light tree. This includes custom falloff, ray visibility, and complex shader node setups including textures. This can result in an increase in noise in some scenes that make use of these features."
- **Light Threshold.** "Probabilistically terminates light samples when the light contribution is below the threshold. This avoids wasting time on lights that contribute little to the image. Zero disables the test."
- **Volume Step Rate Render.** "Increase to reduce render time, at the cost of losing detail."
- **Volume Max Steps.** "Maximum number of steps through the volume before giving up" — so exceeding it stops the walk through the volume.
- **Fast GI Approximation.** "This provides fast alternative to full global illumination, for interactive viewport rendering or final renders with reduced quality."
- **Simplify: Max Subdivision, Child Particles, Texture Limit.** Ceilings on subdivision level, a subset of hairs and particles ("Show only a subset of all child hairs and particles"), and textures "automatically scales textures down".
- **Camera Cull / Distance Cull.** Objects are removed from the render.
- **High Quality Normals.** "can improve visual quality for dense meshes with high frequency textures at the cost of memory".
- **Compositor Precision.** "The precision of compositor intermediate result."
- **Compositor Denoise Final Quality.** "High: Produces the highest quality output at the cost of long processing times. … Fast: Produces an output quickly at a noticeable cost of quality."
- **Use Compositing / Use Sequencer.** Turn whole post-render stages on or off, so the file written differs.
- **EEVEE Samples, Shadow Rays, Shadow Steps.** "Higher values reduces the noise caused by random shadow sampling" and "Higher step count results in softer shadows but have a higher cost."
- **EEVEE Shadow Pool / Light Probes Volume Pool.** Size limits with a stated failure mode: "Increasing the size might fix the Shadow buffer full error."
- **EEVEE Raytracing Resolution and Fast GI Resolution.** "Lower options will be faster and use less memory but will produce blurrier results."
- **EEVEE Raytracing Precision.** "Higher values increase precision of the screen space ray-tracing but lower the maximum trace distance."
- **EEVEE Volumes Resolution and Steps.** The manual's wording here reads oddly but I quote it as written: "Resolution / Controls the quality of the volumetric effects. Lower resolution increases video memory usage and quality." and "Steps / Number of steps to compute volumetric effects. Higher count increases video memory usage and quality." — Blender manual, https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/volumes.html, read 2026-07-30
- **EEVEE Light Threshold.** "Minimum light intensity for a light to contribute to the lighting. Used to compute the distance at which to cut-off lights influence. Lower values improve performance."
- **Workbench Anti-Aliasing.** "With this option selected no anti-aliasing will be applied" vs. Multisample rendering the scene several times.
- **Adaptive Subdivision Offscreen Scale.** "Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory."
- **Render Texture Limit.** "Limit texture size used by final rendering."
- **Which engine.** Changing engine changes which set of the above even exists; the manual documents the three engines as separate chapters with different settings.

### 2c. Where the sources are silent, and it matters

**LABEL for this part: not in the sources.**

For the settings in 2a, the sources say nothing about whether the image comes out the same. They describe the cost and stop. They do not say that Threads, Tile Size, Use Tiling, the four BVH options, Persistent Data, or the choice of CPU against GPU leave the rendered pixels unchanged — and they do not say the opposite either. This is the question a render farm would most want settled, and it is not settled here.

Where I looked for it: I searched the whole manual and Cycles' `properties.py` for `deterministic`, `reproducib`, `identical result`, `same image`, `bit-identical`, and `noise pattern`. The only hits that bear on rendering at all are the light-paths line "This will still converge to the same image" (about minimum bounces, not about threads or devices) and the Seed description "Seed value for integrator to get different noise patterns". I read the Cycles Performance page, the Cycles Sampling page, the GPU Rendering page and the Cycles release notes in full, and none of them addresses it.

**Not covered.** Whether the settings in 2a leave the output identical. How large any of the quality changes in 2b actually are. Whether two of these settings interact — for example whether tiling changes what adaptive sampling decides.

**Outside the interface.** Nothing in the sources says the effect on the image differs when Blender is driven from outside its window. The viewport-only members of these families (Viewport Samples, Viewport Texture Limit, Pixel Size, Viewport anti-aliasing, preview denoising) have no bearing on a final render either way.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Operating system.** The sources do not say the effect on the image differs by operating system, with one exception that is really about hardware: "Path guiding is only available when rendering on a CPU" and "Path Guiding is not supported on any GPU", plus "Shadow caustics are not supported with HIP" and "Open Shading Language is only supported for OptiX" — Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html and https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30. Since which device you can have depends on the operating system, these limits land differently on different systems.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/4.5/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 16 — whether thread count, tile size, Use Tiling, the three acceleration-structure options, BVH Time Steps, Persistent Data or High Quality Normals change the picture.** Done to a scene built from a script and saved: Cycles, Device CPU, 640 by 480 at 100 per cent, 64 samples, Seed 7, Use Animated Seed off, Denoise off, Adaptive Sampling off, Max Bounces 4, stamping off, saved as OpenEXR at full float with no compression. Twelve renders of frame 1, each changing exactly one thing — nothing; `-t 1`; `-t 8`; Tile Size 64; Tile Size 8192; Use Tiling off; Use Spatial Splits on; Use Compact BVH on; Use Curves BVH off; BVH Time Steps 4; Persistent Data on; High Quality Normals on — then each compared against the base **by pixels**, by loading both images and subtracting them channel by channel. What came back, for every one of the eleven pairs:

```
IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800
```

Two control renders were made to show the comparison can tell a difference at all — without them no result from it would mean anything:

```
seed 7 → 8: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.2438785433769226 NUM-DIFFERING-CHANNEL-VALUES 862914 of 1228800
64 samples → 63: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.2541635036468506 NUM-DIFFERING-CHANNEL-VALUES 863070 of 1228800
```

**Observed.** None of those eleven changes altered a single channel value, on this scene, on this machine, in this version.

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench as well as Cycles.** Done to a scene with a Subdivision Surface modifier at render level 4. Six renders: each of the three engines with Simplify off, and each with `use_simplify` on and `simplify_subdivision_render` set to 0, compared by pixels:

```
CYCLES              : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.10663598775863647 NUM-DIFFERING-CHANNEL-VALUES 26331 of 76800
BLENDER_EEVEE_NEXT  : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.105712890625      NUM-DIFFERING-CHANNEL-VALUES 3570 of 76800
BLENDER_WORKBENCH   : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.0909423828125     NUM-DIFFERING-CHANNEL-VALUES 3398 of 76800
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

**Hand test 19 — what a Time Limit does to the picture.** Same scene rendered with Time Limit 0 (`Time: 00:01.40`) and with Time Limit 0.3 seconds (`Time: 00:00.40`). Both files 4923638 bytes; compared by pixels, `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.35095858573913574`.

**Observed.** A time limit changes the picture as well as the cost.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 4.5.12 at `/opt/blender/4.5/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/4.5/` or `/root/render-demands/prefs-card-cuda/4.5/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 27 — whether the same file renders to the same picture on the processor and on the card.** One file, three renders, changing only the device: Device CPU with the backend preference at None; Device GPU Compute with the backend CUDA; Device GPU Compute with the backend OptiX. The file was built to leave nothing else free to move: Cycles, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, no path guiding, no Open Shading Language, four bounces, OpenEXR at full float with no compression. The three images were then compared **by pixels** — both loaded and subtracted channel by channel — and never by comparing the saved files. What came back:

```
processor vs card (CUDA)  : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.042065441608428955  DIFFERING 694851 of 1228800  MEAN-ABS-DIFF 0.00014677665603812784
processor vs card (OptiX) : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.0502660870552063  DIFFERING 694651 of 1228800  MEAN-ABS-DIFF 0.0001593937777215615
card CUDA vs card OptiX   : IDENTICAL-PIXELS False  MAX-ABS-DIFF 0.03436154127120972  DIFFERING 672129 of 1228800  MEAN-ABS-DIFF 0.0001258109405171126
```

The control, which is what makes those numbers mean anything: each of the three renders was run a second time, unchanged, and compared with its first run.

```
processor, run 1 vs run 2  : IDENTICAL-PIXELS True   MAX-ABS-DIFF 0.0
card CUDA, run 1 vs run 2  : IDENTICAL-PIXELS False  MAX-ABS-DIFF 5.960464477539062e-07  DIFFERING 618182 of 1228800
card OptiX, run 1 vs run 2 : IDENTICAL-PIXELS False  MAX-ABS-DIFF 7.152557373046875e-07  DIFFERING 616158 of 1228800
```

**Observed.** **The device changes the picture.** Which device did the work is not only a matter of what the render costs; the image that comes out is a different image. Two things say how much of a difference it is. It is small in size — the largest single channel value moves by about 0.0420 on a nought-to-one scale, and the average move is about a ten-thousandth — but it is spread over more than half of the image. And it is far above the noise of the machine: the processor repeats its own render exactly, and the card repeats its own render to within about a millionth, which is about the finest step a number of this kind can take, while the difference between processor and card is some tens of thousands of times larger than that. The two card backends do not agree with each other either. So a frame rendered on the processor and a frame rendered on the card are not interchangeable, and nor are frames from two different backends on the same card.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/4.5/`.

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
base vs seed8 (seed 7 -> 8):    IDENTICAL-PIXELS False MAX-ABS-DIFF 0.6313424110412598 NUM-DIFFERING-CHANNEL-VALUES 714775 of 1228800
base vs s63 (64 samples -> 63): IDENTICAL-PIXELS False MAX-ABS-DIFF 0.6271419525146484 NUM-DIFFERING-CHANNEL-VALUES 921599 of 1228800
```

**Observed.** Thread count, tile size, Use Tiling, the three acceleration-structure options, BVH Time Steps, Persistent Data and High Quality Normals all left the picture untouched, on a machine with a different processor from the one the same eleven were shown clean on. The rented Linux machine gave the same eleven identical results. **The two machines agree.**

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench.** A scene with a Subdivision Surface modifier at render level 4, rendered under each of the three engines with Simplify off, then again with Simplify on and Max Subdivision (render) at 0, each pair compared by pixels:

```
CYCLES              simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.4128134250640869 NUM-DIFFERING-CHANNEL-VALUES 37701 of 76800
BLENDER_EEVEE_NEXT  simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.3798828125 NUM-DIFFERING-CHANNEL-VALUES 10298 of 76800
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
processor vs card (Metal) : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.03450274467468262 NUM-DIFFERING-CHANNEL-VALUES 636416 of 1228800 MEAN 8.436348790989238e-07
```

**And the control, without which none of that means anything** — each of the two renders repeated, unchanged, and compared with its own first run:

```
processor run 1 vs run 2   : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0
card Metal run 1 vs run 2  : IDENTICAL-PIXELS False MAX-ABS-DIFF 4.291534423828125e-06 NUM-DIFFERING-CHANNEL-VALUES 417415 of 1228800
```

**Observed.** **The device changes the picture on this machine too.** More than half of the channel values differ, the largest by about three to five hundredths on a nought-to-one scale. The processor repeats its own render exactly; Apple's graphics do not, but only by about a millionth, which is some tens of thousands of times smaller than the gap between the two devices. The rented Linux machine found the same thing with an NVIDIA card through CUDA and through OptiX. **Two different makes of graphics processor, two operating systems, the same answer: choosing the device changes the image, not only what the image costs.** What it costs was not measured here; the Linux machine carries that.

**Hand test 20 — a start-up script setting Samples.** `--python-expr "import bpy; bpy.context.scene.cycles.samples=7"` against the 64-sample file:

```
SAMPLES-SET 7
the 64-sample base vs the 7-sample render : IDENTICAL-PIXELS False MAX-ABS-DIFF 2.5408496856689453 NUM-DIFFERING-CHANNEL-VALUES 921600 of 1228800
```

**Observed.** The assignment reaches the render and changes the picture. The render time was not recorded here.

**Hand test 31 — the texture cache and the picture.**

```
HAS-use_texture_cache False HAS-use_auto_generate_texture_cache False
```

**Observed.** Neither setting exists in this version on macOS either, so the rest of the test cannot be carried out here. The same file rendered twice with nothing changed gave `IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0`, and no folder appeared beside the images.

### Answered by hand test — the tests the citation check sent back, 2026-08-07

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 4.5.12 at `/opt/blender/4.5/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/4.5/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — what the device does to the picture, and what a file asking for a card it has not been given produces.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render.

Two renders of it, changing only the device, plus a repeat of each: the file saying CPU with no argument, and the same file with `-- --cycles-device CUDA`. Compared on the pixels, never on the saved files:

```
processor run 1 vs run 2 : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0                  DIFFERING 0 of 1228800
card run 1 vs run 2      : IDENTICAL-PIXELS False MAX-ABS-DIFF 2.86102294921875e-06    DIFFERING 473990 of 1228800
processor vs card        : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.07276105880737305     DIFFERING 608159 of 1228800
```

**Observed.** The device changes the picture, on a second scene and in a second sitting: about half the channel values differ, the largest by some hundredths on a nought-to-one scale, where the processor repeats its own render exactly and the card repeats its own to about three millionths. This is hand test 27's result standing up on a scene built for a different question.

**And the one that answers what the file's own Device produces.** The same scene saved with Device **GPU Compute**, rendered with the backend preference naming nothing:

```
against the processor reference : IDENTICAL-PIXELS True  MAX-ABS-DIFF 0.0              DIFFERING 0 of 1228800
against the card reference      : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.07276105880737305  DIFFERING 608159 of 1228800
```

**Observed.** It produces the **processor's** picture, to the very last number. So a file whose Device says GPU Compute does not, on its own, produce the graphics card's output; what it produces depends on whether a backend has been named somewhere else, and this is settled here by which picture came out rather than by what Blender printed, which names no device at all.

**One more thing about the picture, from hand test 34.** The same file rendered with `-- --cycles-device OPTIX` matched **neither** reference — against the processor 0.07276058197021484, against the CUDA card 0.06922316551208496. Three devices, three pictures. **Still owed on the Creator's Mac.**



## 3. Which of them can be settled at the moment a render is started from outside Blender, and which only by changing the file?

**LABEL: documented** for the flags; **inferred** for the general lever at the end.

### 3a. Settled by a command-line flag, directly

The manual's Command Line Arguments page for 4.5 (it names itself "Blender 4.5 LTS" at the top) lists these, and only these, that bear on demand:

> "-E, --engine <engine> / Specify the render engine. Use -E help to list available engines."
> "-t, --threads <threads> / Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count."
> "-S, --scene <name> / Set the active scene <name> for rendering."
> — Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30

Under a separate heading:

> "Cycles Render Options / Cycles add-on options must be specified following a double dash. / --cycles-device <device> / Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. / Append +CPU to a GPU device to render on both CPU and GPU. / Example: / blender -b file.blend -f 20 -- --cycles-device OPTIX"
> "--cycles-print-stats / Log statistics about render memory and time usage."
> — same page

Cycles' own files say plainly what `--cycles-device` overrides, which the manual does not:

> ```
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)
> ```
> — Cycles' own files, `engine.py`, lines 16–20, read 2026-07-30 (**observed**)

Two more flags touch the machine rather than the render itself:

> "--gpu-backend / Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."
> "--gpu-compilation-subprocesses / Override the Max Compilation Subprocesses setting (OpenGL only)."
> — Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30

And these decide how much work there is by deciding what gets rendered: `-a/--render-anim`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, plus `-b/--background` ("Run in background (often used for UI-less rendering)") — same page.

The manual's own worked example puts three of them together:

> "blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a / -E CYCLES / Use the "Cycles Render" engine. … -t 2 / Use only two threads."
> — Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30

Order matters, and the manual is emphatic about it:

> "Arguments are executed in the order they are given. eg: blender --background test.blend --render-frame 1 --render-output "/tmp" …will not render to /tmp because --render-frame 1 renders before the output path is set. blender --background --render-output /tmp test.blend --render-frame 1 …will not render to /tmp because loading the blend-file overwrites the render output that was set."
> — Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30

### 3b. Everything else on the list has no flag

I read the whole Command Line Arguments page. There is no flag for samples, tile size, tiling on or off, persistent data, resolution, resolution percentage, render region, simplify, the BVH options, time limit, denoising, bounces, or any EEVEE or Workbench setting. The flags for demand are the short list in 3a.

### 3c. The general lever, and what it does and does not do

**LABEL: inferred.** The command line can run Python, and Python can set any of these properties.

The documented pieces are: "-P, --python <filepath> / Run the given Python script file." and "--python-expr <expression> / Run the given expression as a Python script." (Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30); "Arguments are executed in the order they are given" (same page); and the properties themselves being writable properties on the scene, per the API page for `bpy.types.RenderSettings` and Cycles' `properties.py`. The step I am taking: if a script runs after the file loads and before the render argument, it can set those properties for that run. The sources do not spell this out for render settings, so this is inference, not documentation.

The important consequence, and it is the honest answer to the question: **for the scene settings in question 1, nothing needs to be settled "only by changing the file"**, because a script at launch can set them. What is true is narrower — apart from the flags in 3a, nothing can be settled *by a flag alone*.

Two limits on how far that reaches, and neither is settled here. The Preferences-held settings in 3d are not scene properties, and no source I read says a launch script can change them. And the API note in 3e says outright that `--threads` has no equivalent in Blender's Python API, which pulls directly against the word "everything".

### 3d. The settings that are not in the file at all

The Preferences-held ones (Cycles compute device type, which devices are ticked, Distribute Memory Across Devices, MetalRT, HIP RT, Embree on GPU, Kernel Optimization, shader compilation method and count) cannot be settled by changing the .blend, because they are not in it. They live in a separate file:

> "./config/userpref.blend / User preferences."
> — Blender manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html, read 2026-07-30

And they still apply to a command-line render:

> "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them"
> — Blender manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html, read 2026-07-30

For the Cycles device, `--cycles-device` is the documented override (3a). For the others I found no flag.

### 3e. One documented oddity about --threads

The API documentation, in the page about using Blender as a Python module, says:

> "Functionality controlled by command line arguments (shown by calling blender --help aren't accessible). Typically this isn't such a limitation although there are some command line arguments that don't have equivalents in Blender's Python API (--threads and --log for example)."
> — Blender Python API, https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html, read 2026-07-30

This is a statement that `--threads` has no Python equivalent, even though `RenderSettings.threads` exists as a property. I record it as quoted and do not try to reconcile the two; I do not know from the sources whether the command-line thread count and the scene's thread count are the same thing.

**Not covered.** Whether `-t/--threads` overrides the scene's own Threads Mode and Threads, or interacts with them some other way. The manual does not say, and the API note above pulls the other way. Also not covered: whether `--cycles-device` survives a file that has never had Cycles selected as its engine, and what happens if `-E` and the file disagree.

**Outside the interface.** This whole answer is about being outside the interface, so there is nothing to contrast. The one thing worth carrying: `-b/--background` is what the manual pairs with all of these, and "The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards." (Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30).

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Operating system.** The flags themselves are not marked as operating-system specific, except that `--cycles-device`'s valid values are only usable where that technology is supported (see question 1's operating system note), `--gpu-backend`'s `metal` value likewise, `--gpu-compilation-subprocesses` is marked "(OpenGL only)" and the underlying preference is documented as "not available on macOS", and `-con, --start-console` is "(Windows only)". Also documented: `TEMP` "Store temporary files here (MS-Windows)" and `TMPDIR` "Store temporary files here (UNIX Systems)" — same page.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/4.5/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

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
    wall 0:13.32
(b) the same with the script before -t 3
    SET-BY-SCRIPT FIXED 3
    FINAL FIXED 3
    wall 0:14.51
(c) the script alone, no -t
    SET-BY-SCRIPT FIXED 5
    FINAL FIXED 5
    wall 0:08.80
```

**Observed.** The script assigned 5 and printed back 3. Order in the command line made no difference. With no `-t` the same script got 5.

**Hand test 5 — whether `-E` overrides the file's engine, whether its place matters, and whether the file on disk changes.** Done to a file saved with EEVEE as its engine:

```
(a) no -E                                   ENGINE BLENDER_EEVEE_NEXT
(b) -E CYCLES after the file name           ENGINE CYCLES
(c) -E CYCLES before the file name          ENGINE BLENDER_EEVEE_NEXT
(d) -E CYCLES, then save a copy             BEFORE-SAVE CYCLES ; the copy reads COPY-ENGINE CYCLES
(e) the original file, read again           ENGINE BLENDER_EEVEE_NEXT
(f) -E BOGUS_ENGINE                         Error: engine not found 'BOGUS_ENGINE'
```

**Observed.** The argument takes effect only after the file name. It does not change the file on disk, but a session that saves afterwards writes the overridden engine into the new file. An unknown name is refused by name.

**Hand test 1 — what engines the build offers.** Done by running the documented listing:

```
$ blender -b -E help
(no output at all; still running when killed after forty seconds)
EXIT=124
Tried three ways — `blender -b -E help`, `blender --background --engine help`, and with a file named as well. All three behaved the same.
Background mode itself works in this build: `blender -b --factory-startup --python-expr "print('ALIVE')"` printed ALIVE and quit at once.
```

Without `-b`, on this machine, all four versions fail before reaching the list: `Failed to initialize display for back-end(s): ['WAYLAND', 'X11'] ... ERROR Unable to initialize GHOST, exiting!`

**Observed.**

**Hand test 6 — whether `--cycles-device` changes the device stored in the scene.** Done to a file saved with Cycles and Device CPU. Read back before a render and after one:

```
(a) no override                                          DEVICE CPU
(b) -- --cycles-device OPTIX, read before any render     DEVICE CPU
(c) -f 1 first, then read, with --cycles-device OPTIX
CUDA cuInit: Unknown error
Error: Found no Cycles device of the specified type
DEVICE CPU
(d) -f 1 first, then read, with --cycles-device CPU      DEVICE CPU
(e) -- --cycles-device NOSUCHDEVICE
Error: NOSUCHDEVICE is not a valid Cycles device.
```

**Observed.** The stored value never moved, at either moment. The machine's card was unreachable, so what the override does to a render that can use a card is not settled here.

**Hand test 8 — whether a render started from outside reads the saved preferences.** Done in a settings folder of its own. The Cycles compute backend was set to `CUDA` and `bpy.ops.wm.save_userpref()` called, producing `config/userpref.blend` of 177783 bytes. Then read back:

```
plain background run (no --factory-startup)   READ-PREF CUDA
the same with --factory-startup               READ-PREF NONE
```

**Observed.** A background run does read the saved preferences; `--factory-startup` stops it.

**Hand test 20 — whether a start-up script can set a setting that has no argument of its own.** Done to the 640 by 480 scene, whose Samples is 64: `blender -b --factory-startup med.blend --python-expr "import bpy; bpy.context.scene.cycles.samples=7; print('SAMPLES-SET', bpy.context.scene.cycles.samples)" -o <folder>/s7_ -F OPEN_EXR -x 1 -f 1`. What came back: `SAMPLES-SET 7`, `Time: 00:00.26` against 00:01.40 for the unchanged render, and the picture differs from the 64-sample one — `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.6403772830963135`.

**Observed.** The assignment reaches the render.

**Hand test 21 — whether "Render Single Layer" is honoured from the command line.** Done to a file with two view layers, `ViewLayer` and `Second`, saved once with the setting off and once on, each rendered with `-f 1`. In both runs Blender named both view layers in its own progress lines (`Scene, Second` and `Scene, ViewLayer`).

**Observed.** The setting made no difference to a render started from the command line.

**One thing learned by getting a command wrong, recorded because it bears on every command in this answer.** An earlier run gave the arguments as `-f 1 -o <folder>/r_ -F OPEN_EXR -x 1`, with the render before the output path, and the image went to `/tmp/0001.exr` instead. `-o`, `-F` and `-x` have to come before `-f`.

### Answered by hand test — the card tests, 2026-08-07

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 4.5.12 at `/opt/blender/4.5/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/4.5/` or `/root/render-demands/prefs-card-cuda/4.5/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

```
TYPE CUDA  DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'CUDA', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
TYPE OPTIX DEVICES [('NVIDIA GeForce RTX 4080 SUPER', 'OPTIX', True), ('AMD EPYC 7B12 64-Core Processor', 'CPU', False)]
```

**Two things to hold on to while reading the numbers below.** The machine turned out to be **shared with another topic's tests** while these ran — a second Blender was rendering on the same card — so every time here is one busy machine's time, on one card of 16 GB, and none of it is a general claim about how fast a card is. The picture comparisons are not touched by that: they compare the pictures, pixel by pixel, and never the saved files, because Blender writes the time of day into every image it saves, so comparing files reads a clock as often as it reads a render.

The full list of tests, and everything else this topic settled by hand, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 28 — what `--cycles-device` does to a render that really can use the card.** This is the test the earlier sitting could not run at all. Six timed runs, none of them using `--factory-startup`, so the settings folder's saved preferences were read every time. Two files, both Cycles at 640 by 480 with 1024 samples: one saved with Device **CPU**, one saved with Device **GPU Compute**. Two settings folders: one with the Cycles backend preference saved as **None**, one with it saved as **CUDA**. Each run was `blender -b <file> -o <folder>/x_ -F OPEN_EXR -x 1 -f 1 -- --cycles-print-stats`, with the device argument added after `--cycles-print-stats` where one was given. Alongside the timing, this Blender alone was watched for whether it held work on the card.

```
file CPU,  preference None, no argument            : Time 00:22.51, wall 23.05 s, 2142% of a processor — never held any card memory at all
file CPU,  preference None, --cycles-device OPTIX  : Time 00:02.03, wall 2.54 s, 117% of a processor — held card memory, peak 1728 MiB
file CPU,  preference None, --cycles-device CUDA   : Time 00:02.21, wall 2.71 s, 128% of a processor — held card memory, peak 1714 MiB
file CPU,  preference CUDA, no argument            : Time 00:22.87, wall 23.43 s, 2109% of a processor — held card memory, peak 248 MiB
file GPU,  preference CUDA, no argument            : Time 00:02.52, wall 3.18 s, 128% of a processor — held card memory, peak 1714 MiB
file GPU,  preference CUDA, --cycles-device CPU    : Time 00:21.71, wall 22.78 s, 2146% of a processor — held card memory, peak 248 MiB
```

Two more runs were added, because this is the exact pair the 4.5 audit asked for — the file already asking for the card, with the preferences naming no backend at all:

```
file GPU,  preference None, no argument            : Time 00:24.33, wall 24.94 s, 2254% of a processor — never held any card memory at all
file GPU,  preference None, --cycles-device OPTIX  : Time 00:03.30, wall 4.05 s, 83% of a processor — held card memory, peak 1726 MiB
```

**Observed.** **The argument wins, in both directions.** A file that says CPU, with preferences that name no backend at all, renders on the card the moment `--cycles-device` names one — about seven times faster, the processor left almost idle, and the card holding well over a gigabyte. And a file that says GPU Compute, with CUDA saved in the preferences, is dragged back onto the processor by `--cycles-device CPU`. So neither the file's own Device setting nor the saved backend preference can stop the argument. What the two of them do decide is what happens when no argument is given: the file's Device says whether the card is used at all, and the preference says which backend. A file saying GPU Compute with the preference at None renders on the processor and never touches the card — which is the earlier sitting's result, confirmed here with a working card rather than an unreachable one — and a file saying CPU renders on the processor however the preference is set.

**A correction to hand test 26, which was run with no reachable card.** That test found that nothing Blender printed named the device that rendered. With a working card that holds for CUDA — a card render through CUDA printed no word about the card anywhere — but not quite for OptiX, whose renders name it in their ordinary progress lines while the acceleration structure is being built:

```
Fra:1 Mem:10.93M (Peak 38.01M) | Time:00:00.40 | Mem:1.55M, Peak:6.16M | Scene, ViewLayer | Updating Scene BVH | Building OptiX acceleration structure
```

Those lines say OptiX built the structure. They are not a statement of which device rendered, and no line anywhere says that. Anything reading the device off Blender's output still cannot settle it; the timing, and what the process holds on the card, remain the evidence.

**Hand test 6, run again now the card works — whether `--cycles-device` changes the device stored in the scene.** The earlier sitting could only show this on a machine where the named device was absent, which left the obvious objection open. Repeated here against a file saved with Cycles and Device CPU, with the card present and working:

```
(a) no argument                                            : DEVICE CPU
(b) -- --cycles-device OPTIX, read before any render       : DEVICE CPU
(c) -- --cycles-device OPTIX, read after a render          : DEVICE CPU   (and the render ran on the card)
(d) -- --cycles-device CPU, read after a render            : DEVICE CPU
(e) -- --cycles-device NOSUCHDEVICE                        : Error: NOSUCHDEVICE is not a valid Cycles device.
(f) preference CUDA, -- --cycles-device OPTIX, after render: DEVICE CPU   and the preference still reads CUDA
```

**Observed.** The stored value never moves, before a render or after one, whether or not the named device exists and whether or not it is used. The argument changes what does the work for that one run and writes nothing down — not into the scene, not into the saved preferences. An unknown device name is still refused by name. One thing changed from the earlier sitting: with no reachable card, asking for OptiX also produced `CUDA cuInit: Unknown error` and `Error: Found no Cycles device of the specified type`; with the card working, both lines are gone and the render simply runs on the card.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/4.5/`.

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
(a) no -E                                              ENGINE BLENDER_EEVEE_NEXT
(b) -E CYCLES after the file name                      ENGINE CYCLES
(c) -E CYCLES before the file name                     ENGINE BLENDER_EEVEE_NEXT
(d) -E CYCLES, then save a copy, then read the copy    BEFORE-SAVE CYCLES / COPY-ENGINE CYCLES
(e) read the original file again                       ENGINE BLENDER_EEVEE_NEXT
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

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 4.5.12 at `/opt/blender/4.5/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/4.5/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

**Every picture comparison below compares the pictures, pixel by pixel — both images loaded and subtracted channel by channel — and never the saved files.**

**Hand test 33 — what the file's stored Device settles on its own when a render is started from outside.** **The scene these were done to**, built for them from a script: Cycles, Device **CPU**, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, OpenEXR at full float with no compression, stamping off — a plane, a sphere and a torus lit by one area light. It was saved twice, once with Device CPU and once with Device **GPU Compute**, nothing else differing, and what each file and each settings folder actually held was printed rather than assumed before any render. Rendered from the command line with `blender -b <file> -o <folder>/<tag>_ -F OPEN_EXR -x 1 -f 1 --`, no `--factory-startup`, so the saved preferences were read:

```
file CPU,  backend preference NONE, no argument            : the processor's picture (the reference)
file CPU,  backend preference NONE, -- --cycles-device CUDA : the card's picture      (the reference)
file GPU,  backend preference NONE, no argument            : IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 against the PROCESSOR
                                                              IDENTICAL-PIXELS False MAX-ABS-DIFF 0.07276105880737305 against the CARD
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
Error: Found no Cycles device of the specified type
```

and `HIP` printed nothing extra in this version.

**Observed.** The argument does not quietly fall back. A backend with no hardware behind it stops the render, writes no image, and returns a non-zero exit code — which is a thing anything driving Blender from outside can actually see, unlike the device that rendered. An unknown name is a different case again and was refused by name in the earlier sitting. **Still owed on the Creator's Mac.**



## 4. Do any of them belong to a particular render engine rather than to every engine?

**LABEL: documented** and **observed** for where each setting is kept; **inferred** for the groupings in 4c, as noted there. Yes. The split is clear and it runs three ways.

### 4a. The manual keeps them apart

There are three separate Performance pages, one per engine, and they do not hold the same things:

- Cycles: https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html — presets, Threads, Memory (tiling), Acceleration Structure, Final Render, Viewport, Compositor, Denoise Nodes.
- EEVEE: https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/performance.html — High Quality Normals, Memory (Shadow Pool, Light Probes Volume Pool), Viewport, Compositor, Denoise Nodes.
- Workbench: https://docs.blender.org/manual/en/4.5/render/workbench/performance.html — High Quality Normals, and nothing else. The page is thirteen lines long.

All read 2026-07-30.

The manual also names the divide directly when discussing view layers: "This section covers only the Render Layer settings appropriate for the Cycles renderer. For the engine-independent settings, see this section." — Blender manual, https://docs.blender.org/manual/en/4.5/render/layers/introduction.html, read 2026-07-30.

### 4b. Cycles' own files mark their panels as Cycles-only

**observed.** Every Cycles panel derives from a base class that refuses to draw under another engine:

> ```
> class CyclesButtonsPanel(Panel):
> ...
>     COMPAT_ENGINES = {'CYCLES'}
> ...
>     @classmethod
>     def poll(cls, context):
>         return context.engine in cls.COMPAT_ENGINES
> ```
> — Cycles' own files, `ui.py`, lines 64–68, read 2026-07-30

And Cycles registers and unregisters itself against other engines' panels:

> `panel.COMPAT_ENGINES.add('CYCLES')` … `panel.COMPAT_ENGINES.remove('CYCLES')`
> — Cycles' own files, `ui.py`, lines 2631 and 2645, read 2026-07-30

### 4c. Which is which

**LABEL for the groupings below: inferred.** What the sources establish is where each setting is *kept* — on `scene.cycles`, on `scene.eevee`, on `scene.display`, or on Blender's own `bpy.types.RenderSettings`. The step from where a setting is kept to which engines it actually affects is mine, and no source I read states it. Read each heading as "kept here", not as "takes effect there". Section 4b is the one part with direct evidence, and it is about which panels Cycles draws, not about which engine a setting reaches.

**Cycles only** — everything defined in Cycles' `properties.py` and reached as `scene.cycles`: Device, Use Tiling, Tile Size, the four BVH options, Samples, Time Limit, adaptive sampling, Sample Subset, denoising and its device and quality, bounce ceilings, Light Threshold, Light Tree, Scrambling Distance, path guiding, volume step rate and max steps, Fast GI, Render Texture Limit, camera and distance culling, adaptive-subdivision dicing, feature set and OSL, and the debug CPU/BVH flags. The command line reflects the same split: the `--cycles-device` and `--cycles-print-stats` flags sit under a heading "Cycles Render Options" and the manual says "Cycles add-on options must be specified following a double dash" — Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30.

**EEVEE only** — reached as `scene.eevee`: Shadow Pool, Light Probes Volume Pool, render Samples, shadow Rays / Steps / Resolution, volumetric shadows and their steps, ray tracing method / resolution / precision / thickness and its denoising, Fast GI's EEVEE form, volumes Resolution / Steps / Distribution / Max Depth, Overscan, Light Threshold, motion blur steps.

**Workbench only** — reached as `scene.display`: the anti-aliasing choice, with separate viewport and render values; "The setting for viewport rendering and final rendering is saved per scene." — Blender manual, https://docs.blender.org/manual/en/4.5/render/workbench/sampling.html, read 2026-07-30.

**Kept on Blender's own render settings, not on any engine's** — `bpy.types.RenderSettings`: Threads Mode and Threads, Persistent Data, resolution and percentage, render region, Simplify and culling, High Quality Normals, Lock Interface, the compositor device / precision / denoise settings, Use Compositing, Use Sequencer, Use Render Cache, Motion Blur, and the engine choice itself. The strongest evidence that Threads and Persistent Data are Blender's own rather than Cycles', despite appearing on the Cycles page, is that Cycles reads them off `scene.render`, quoted in question 1.

**Which of these names carry a quote, and which do not.** Everything quoted somewhere above is established as far as its quote goes. These I have named from the sources' own headings without quoting anything for them, so nothing written down here establishes them: EEVEE's ray-tracing thickness and its denoising, EEVEE's Fast GI, EEVEE's volume Distribution and Max Depth, Overscan, EEVEE's motion blur steps, Cycles' feature set, and Use Render Cache and Motion Blur on the render settings. Each needs to be quoted from a source or dropped from the lists.

One thing worth naming: the Cycles Performance page is where the manual documents Threads and Persistent Data, and no other page documents them. I searched the whole manual for "Persistent Data" — one hit, on that page. I searched for "Threads Mode", "number of threads" and "CPU cores" — the render-related hits are all on that page. So a reader using EEVEE will not find those two settings written up anywhere, even though the property is kept on Blender's own render settings rather than on Cycles'.

**Not covered.** What Threads and Persistent Data do when the engine is EEVEE or Workbench. The property exists for them; the manual documents it only under Cycles; no source I read says what it does there. Also not covered: whether a third-party engine sees these settings.

**Outside the interface.** The split is a matter of where the setting is stored, not of how Blender is driven, so it holds either way. What does change is that `-E/--engine` can pick the engine at launch, so the same file can be rendered under a different engine's settings than the one it was saved with. The sources do not say what becomes of the unused engine's settings in that case.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Operating system.** The Cycles / EEVEE / Workbench split is not tied to any operating system. Which Cycles devices are available is, as set out in question 1.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/4.5/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

**Hand test 22 — whether EEVEE and Workbench render at all on a machine with no display server.** Done to the base scene: `blender -b --factory-startup scene.blend -E BLENDER_EEVEE_NEXT -o <folder>/e_ -F OPEN_EXR -x 1 -f 1`, then the same with `-E BLENDER_WORKBENCH`. **Both rendered and wrote an image, exit code 0.** EEVEE took `Time: 00:27.92`; Workbench about 1.2 seconds. Workbench printed no such line.

**Observed.** This machine had neither a display server nor a usable graphics card, and both engines still produced their image.

**Hand test 23 — whether the thread count changes anything under EEVEE or Workbench.** Done to the base scene with EEVEE Render Samples 16. Twelve runs: each engine, `-t 1` and `-t 24`, three times each, timed by the operating system. Twenty-four logical processors, machine shared with other work:

```
EEVEE     -t 1  : 6.83 s wall, 721% cpu, 1641636 kb / 7.09 s, 699%, 1642188 kb / 6.76 s, 744%, 1637500 kb
EEVEE     -t 24 : 6.72 s wall, 695% cpu, 1779000 kb / 6.84 s, 700%, 1779032 kb / 6.76 s, 645%, 1787792 kb
WORKBENCH -t 1  : 1.51 s wall,  94% cpu,  552552 kb / 1.30 s,  94%,  526588 kb / 1.27 s,  94%,  522216 kb
WORKBENCH -t 24 : 1.35 s wall, 101% cpu,  661684 kb / 1.38 s, 112%,  710392 kb / 1.37 s, 108%,  701868 kb
```

**Observed.** The two sets of times overlap in both engines, and the processor percentages stay put. Under Cycles the same argument changed the time by a factor of fifteen (hand test 3). **This is "no effect seen", not "the engine ignores it"** — a render spending its time elsewhere would look the same.

**Hand test 17 — whether Simplify's Render column reaches EEVEE and Workbench.** Six renders of a scene with a Subdivision Surface modifier at render level 4, each engine with Simplify off and with `simplify_subdivision_render` at 0, compared by pixels:

```
CYCLES              : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.10663598775863647 NUM-DIFFERING-CHANNEL-VALUES 26331 of 76800
BLENDER_EEVEE_NEXT  : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.105712890625      NUM-DIFFERING-CHANNEL-VALUES 3570 of 76800
BLENDER_WORKBENCH   : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.0909423828125     NUM-DIFFERING-CHANNEL-VALUES 3398 of 76800
```

**Observed.** All three engines changed, so that setting is not Cycles-only in effect.

**Hand test 13 — whether Cycles' own values survive the engine being switched away and back.** Done to a file with Tile Size 256, Samples 11 and Use Tiling off. Opened, printed, engine set to EEVEE and saved; reopened, printed, engine set back to Cycles and saved; reopened and printed again:

```
STEP1 engine CYCLES tile_size 256 samples 11 use_auto_tile False
STEP2 engine BLENDER_EEVEE_NEXT tile_size 256 samples 11 use_auto_tile False
STEP3 engine CYCLES tile_size 256 samples 11 use_auto_tile False
```

**Observed.** Nothing was dropped or reset.

### Answered by hand test — the Creator's Mac, 2026-08-07

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/4.5/`.

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
BLENDER_EEVEE_NEXT Performance -> High Quality Normals drawn straight in it, then Memory, Viewport and Compositor
BLENDER_WORKBENCH  Performance -> High Quality Normals drawn straight in it, then Compositor and nothing else
```

**Observed.** The three engines do not show the same Performance panel. **Threads, Acceleration Structure and Final Render appear under Cycles and under neither of the other two.** Memory and Viewport appear under Cycles and EEVEE but not under Workbench. Compositor appears under all three. High Quality Normals is drawn straight into the Performance panel under EEVEE and Workbench, and is not in Cycles' Performance panel at all.

**One trap is worth recording, because it would have produced a wrong answer.** Asked on its own, the Acceleration Structure panel's own test for whether it should be shown returns **true under all three engines**. It is not drawn under EEVEE or Workbench all the same, because its parent — Cycles' Performance panel — is not shown, and Blender does not draw a child of a panel it is not drawing. The screenshots are what settle it; the panel's own answer, taken alone, does not. This version also draws a **Feature Set** row above Device that the other three do not, which is the interface side of `cycles.feature_set` existing only here.

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
    ENGINE BLENDER_EEVEE_NEXT
(c) the same file with the Cycles add-on on
    TILE-SIZE 2048 SAMPLES 64 DEVICE CPU     ENGINE CYCLES
(d) the same file with the engine set to EEVEE, Cycles add-on on
    TILE-SIZE 2048 SAMPLES 64 DEVICE CPU     ENGINE BLENDER_EEVEE_NEXT
```

One line appeared here that the Linux run of the same test did not record, and it is kept as it came:

```
Error: Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
```


**Observed.** Cycles' settings cannot be read at all with the add-on off, and with the add-on off the same file that reads back as `CYCLES` on line (c) reads back as the EEVEE name on line (b) — the engine stored in the file is not readable either. Cycles' values **are** readable while EEVEE is the engine. The same on the Linux machine.

**Hand test 13 — whether Cycles' own values survive the engine being switched away and back.**

```
STEP1 engine CYCLES tile_size 256 samples 11 use_auto_tile False
STEP2 engine BLENDER_EEVEE_NEXT tile_size 256 samples 11 use_auto_tile False
STEP3 engine CYCLES tile_size 256 samples 11 use_auto_tile False
```

**Observed.** Nothing was dropped or reset. Same as on Linux.

**Hand test 17 — whether Simplify's Render column reaches all three engines.**

```
CYCLES              simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.4128134250640869 NUM-DIFFERING-CHANNEL-VALUES 37701 of 76800
BLENDER_EEVEE_NEXT  simplify off vs on : IDENTICAL-PIXELS False MAX-ABS-DIFF 0.3798828125 NUM-DIFFERING-CHANNEL-VALUES 10298 of 76800
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

**The machine, in full.** The rented Linux machine: **Ubuntu 22.04.5 LTS** on the 6.8.0-59 kernel, one **NVIDIA GeForce RTX 4080 SUPER** with 16376 MiB, the graphics driver reading **580.173.02** on both halves, an **AMD EPYC 7B12 64-Core Processor** offering twenty-four logical processors, 39 GB of memory, and **no display server at all**. Blender 4.5.12 at `/opt/blender/4.5/blender`. Every run was pointed at a settings folder of its own, made before use, at `/root/again-render-demands/prefs/4.5/none/`, with the Cycles compute backend set to `NONE` and saved — never the machine's own; every working folder was under `/root/again-render-demands/`, outside the project. **The machine was checked before the work began and was idle** — nothing held work on the card, the load average was 0.17, no other Blender was running — **and it did not stay that way**: within three minutes four other topics' Blenders were running on it and the load average reached 27.80. **So every time below is a busy machine's time, and nothing here rests on a time.** Card memory was read only for this test's own Blender, by matching each process the driver named against that run's own output path in the process's own command line.

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

**Hand test 35 — which of the settings named in one version's findings exist in the other three, with the sweep written out exactly.** The citation check sent hand test 15 back over its totals, which three sittings had reported three ways. The sweep was run again with the rule stated: five blocks named exactly (`scene.render`, `scene.cycles`, `scene.eevee`, `scene.display`, and the Cycles add-on's preferences), each counted twice, once including its own `rna_type` entry and once excluding it. This version: `scene.render` 109 / 108, `scene.cycles` 106 / 105, `scene.eevee` 56 / 55, `scene.display` 10 / 9, the Cycles preferences 9 / 8 — **290 counting `rna_type`, 285 not counting it**. Counting it gives 290, 276, 278 and 286 across the four versions, which is the citation check's figure; not counting it gives 285, 271, 273 and 281, which is the Mac's; and the first Linux sitting's 286, 272, 274 and 282 are neither and **cannot be reproduced**. **The finding is untouched by that**: the set of names not present in all four versions came back as the same twenty-eight, on the same versions, including `scene.eevee.fast_gi_thickness_far` — in 4.5, 5.0 and 5.1, absent from 5.2. **Still owed on the Creator's Mac**, as a repeat under the stated rule.



## 5. Can their values be read out of a file?

**LABEL: inferred** for the main claim; **documented** for each piece it rests on.

### 5a. The scene-level ones: yes

The pieces, all documented:

> "Each blend-file contains a database. This database contains all scenes, objects, meshes, textures, etc. that are in the file."
> — Blender manual, https://docs.blender.org/manual/en/4.5/files/introduction.html, read 2026-07-30

> "The base unit for any Blender project is the data-block. Examples of data-blocks include: meshes, objects, materials, textures, node trees, scenes, texts, brushes, and even Workspaces. … They are the primary contents of the blend-file."
> — Blender manual, https://docs.blender.org/manual/en/4.5/files/data_blocks.html, read 2026-07-30

> "cycles / Cycles render settings / Type: CyclesRenderSettings, (readonly)"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.Scene.html, read 2026-07-30

> "# Print all scene names in a list. / print(bpy.data.scenes.keys())"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.data.html, read 2026-07-30

The step I am taking, which makes this inferred and not documented: the render settings in question 1 are properties of the Scene data-block (via its `render`, `cycles`, `eevee` and `display` members), scenes are among the data-blocks a blend-file holds, and `bpy.data.scenes` reaches loaded scenes by name, therefore opening the file in Blender and reading those properties in Python gives their values. No page I read states that in one sentence, and none gives a worked example of reading a render setting out of a file.

Combined with question 3's `-P` / `--python-expr` and `-b`, that reading can be done without a person at the interface. That too is inference.

### 5b. Reading without fully loading the file: only partly

`bpy.data.libraries.load` can open a blend-file and report what is in it, but the documentation describes what it gives as names, not settings:

> "Returns a context manager which exposes 2 library objects on entering. Each object has attributes matching bpy.data which are lists of strings to be linked."
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html, read 2026-07-30

The example loads a scene by name — `data_to.scenes = ["Scene"]` — so a scene can be brought in without opening the file as the main file. The API page about Blender as a Python module puts the limitation plainly:

> "In some cases the library API may be an alternative to starting separate processes, although this API operates on reading and writing ID data-blocks and isn't a complete substitute for loading .blend files"
> — Blender Python API, https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html, read 2026-07-30

### 5c. Reading without the Blender application: documented as possible

> "Blender supports being built as a Python module, allowing import bpy to be added to any Python script, providing access to Blender's features."
> "Note / Blender as a Python Module isn't provided on Blender's official download page. A pre-compiled bpy module is available via PIP."
> "For the most part using Blender as a Python module is equivalent to running a script in background-mode (passing the command-line arguments --background or -b), however there are some differences to be aware of."
> — Blender Python API, https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html, read 2026-07-30

With one difference that bears directly on this question:

> "The users startup and preferences are ignored to prevent your local configuration from impacting scripts behavior. The Python module behaves as if --factory-startup was passed as a command line argument."
> — same page

I found nothing anywhere in the sources describing the blend-file's own layout, or any way to read a setting out of one without Blender's code doing the reading. Where I looked: the whole `files/` chapter of the manual (introduction, data_blocks, blend/index, blend/open_save, blend/compatibility, blend/packed_data), the API pages `bpy.data`, `bpy.types.BlendData`, `bpy.types.BlendDataLibraries`, `info_advanced_blender_as_bpy`, and searches across the manual and API text for `libraries.load`, `open_mainfile`, `load_mainfile`.

### 5d. Two settings that cannot be read out of the file

**Preferences.** Everything in section 1e is stored in `./config/userpref.blend`, not in the scene file. Reading the scene file will not tell you which compute device Cycles will use, whether MetalRT is on, or how kernels will be optimised.

**Anything set by a flag at launch.** `-t`, `-E`, `--cycles-device` and the rest are not in the file at all, by definition.

### 5e. One awkward thing about the Cycles settings

**LABEL: documented**, with an inferred reading flagged.

The API documentation names `Scene.cycles` and gives its type as `CyclesRenderSettings`, but there is no page for `CyclesRenderSettings` in the 4.5 API documentation — I checked the file list of all 1,851 pages in `api_4.5` and searched for `CyclesRenderSettings`, `CyclesPreferences` and `CyclesCurveRenderSettings`; none exists. Likewise the render engine enum lists only one engine:

> "engine / Engine to use for rendering / Type: enum in ['BLENDER_EEVEE_NEXT'], default 'BLENDER_EEVEE_NEXT'"
> — Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30

My reading — and this is **inferred**, the step being that Cycles is an add-on and add-on-registered classes and enum entries only exist when the add-on is registered — is that the API documentation was generated without Cycles and Workbench registered, which is why neither their settings class nor their engine identifiers appear. The practical consequence for question 5: the names and permitted values of every Cycles setting have to come from Cycles' own `properties.py`, not from the API documentation. The engine identifier `CYCLES` is **observed** there:

> ```
> class CyclesRender(bpy.types.RenderEngine):
>     bl_idname = 'CYCLES'
>     bl_label = "Cycles"
> ```
> — Cycles' own files, `__init__.py`, lines 42–44, read 2026-07-30

**Not covered.** Whether reading a scene's settings requires loading the whole file (and therefore paying its memory cost), or whether the library API can reach a scene's render settings on its own — the documentation says the library API reads data-blocks, but does not say what of a scene data-block's contents come across. Also not covered: whether the values read out of a file are the values that will actually be used, since flags and Preferences can override them (see 3d and 5d), and since Cycles ships a version-update handler (`version_update.py`, registered as `bpy.app.handlers.version_update`, observed in `__init__.py` line 156) whose effect on settings read from an older file I did not investigate.

**Outside the interface.** Reading them requires running Blender's code, in the interface, in background mode, or as the Python module. Background mode is documented as the usual way ("Run in background (often used for UI-less rendering)"). One documented difference for the Python module: it ignores the user's preferences, "behaves as if --factory-startup was passed", so a reader built that way will not see the Preferences-held settings the same way a normal Blender run would.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Operating system.** Nothing in the sources ties reading a blend-file to an operating system. The one operating-system note I found nearby is a deprecation, not a restriction on reading: "Blendfiles saved by Blender from a Big Endian platform will trigger a warning when opened." — Blender release notes, https://developer.blender.org/docs/release_notes/4.5/core/, read 2026-07-30.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

---


### Answered by hand test — 2026-08-06

**The machine, in full.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, 118 GB of free disk, and **no display server at all**. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`. Every run below was pointed at a settings folder of its own, made before use, at `/root/render-demands/prefs/4.5/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Two limits on reading any of it: the machine was **shared with other topics' tests** while these ran, so every time and every memory figure is from a busy machine; and **its graphics card could not be reached at all**, because its loaded driver was 580.95.05 and its driver library 580.173.02 and the two did not match, so nothing below settles anything about rendering on a graphics card. The full list, including the tests that had to be left for a machine with a working card, is `Audits/Blender Research/What a render demands/Hand tests.md`.

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
    ENGINE BLENDER_EEVEE_NEXT
(c) the same file with the add-on on
    TILE-SIZE 2048 SAMPLES 64 DEVICE CPU
    ENGINE CYCLES
(d) a file whose engine is EEVEE, add-on on
    TILE-SIZE 2048 SAMPLES 64 DEVICE CPU
    ENGINE BLENDER_EEVEE_NEXT
```

**Observed.** Two things. Cycles' settings cannot be read at all with the add-on off. And with the add-on off, the same file that reads back as `CYCLES` on line (c) reads back as `BLENDER_EEVEE_NEXT` on line (b) — so the engine stored in the file is not readable either. With the add-on on, Cycles' values read back whichever engine the file names.

**Hand test 14 — what each version reads out of a file saved by 4.5 with Use Tiling off.** Done to one file, saved by Blender 4.5.12 with Use Tiling off, Tile Size 256 and Samples 11, opened unchanged by each of the four Blenders. This version read:

```
OLDFILE use_auto_tile False tile_size 256 samples 11
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

**The machine, in full.** The same rented Linux machine as the tests above, with its graphics card now repaired: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with **16 GB** of card memory, graphics driver **580.173.02** — the loaded driver and the driver library now name the same version, where on 2026-08-06 they did not and nothing on the machine could start work on the card — an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, and **no display server at all**. Blender 4.5.12 at `/opt/blender/4.5/blender`. Every run below was pointed at a settings folder of its own, made before use, under `/root/render-demands/prefs-card-none/4.5/` or `/root/render-demands/prefs-card-cuda/4.5/` — never the machine's real one — and every working folder was under `/root/render-demands/`. Asked for its devices, Cycles now lists the card under both of this machine's backends:

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

**The machine, in full.** The Creator's own Mac: a MacBook Pro with an Apple **M1 Pro** chip — eight logical processors, and one built-in graphics processor that Cycles names `Apple M1 Pro (GPU - 14 cores)` — 16 GB of memory, **macOS 26.5.2** (build 25F84), and **no second graphics card**. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. Every run below was pointed at a settings folder of its own, made before use, under a temporary folder outside the project — never the Creator's real one — and every working folder was outside the project too. The files the tests were done to were built for them and live in `/Users/artist/Documents/Smoozy hand tests/What a render demands/4.5/`.

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


## Where I looked

**Files searched.** The whole of `manual_4.5`, `api_4.5` (1,851 pages), `release_notes_4.5` (19 files) and `cycles_4.5`, using text search across all of them at once before opening anything.

**Search terms used across the manual and API:** `performance`, `Performance`, `memory`, `memory usage`, `memory limit`, `RAM limit`, `max memory`, `memory cap`, `peak memory`, `threads`, `Threads Mode`, `number of threads`, `CPU cores`, `tile`, `tile size`, `device`, `persistent data`, `Persistent Data`, `sample`, `samples`, `Sample Override`, `Render Region`, `Lock Interface`, `Memory & Limits`, `Memory Cache Limit`, `memory_cache_limit`, `userpref.blend`, `Save Preferences`, `saved in the blend-file`, `stored in the blend`, `contains all`, `background mode`, `background-mode`, `priority`, `process priority`, `affinity`, `nice value`, `deterministic`, `reproducib`, `identical result`, `same image`, `bit-identical`, `noise pattern`, `cost`, `expensive`, `slower`, `faster`, `CYCLES_` (environment variables), `libraries.load`, `open_mainfile`, `load_mainfile`, `as a Python module`, `CyclesRenderSettings`, `CyclesPreferences`.

**Pages read in full:** the three Performance pages (Cycles, EEVEE, Workbench); Cycles Sampling, Simplify, Subdivision, Volumes, Light Paths (partial), GPU Rendering; EEVEE Sampling, Raytracing, Volumes; Workbench Sampling; Output Format; Rendering Animations (partial); Command Line Arguments; Rendering From The Command Line; Preferences → System; Preferences → Introduction; Blender Directory Layout (partial); Scripting Security (partial); Layers & Passes Introduction (partial); Files Introduction; Data-Blocks (partial); 3D View Regions (partial). API pages: `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE`, `bpy.types.SceneDisplay`, `bpy.types.RaytraceEEVEE`, `bpy.types.Scene` (partial), `bpy.types.ViewLayer` (partial), `bpy.types.BlendDataLibraries`, `bpy.data`, `info_advanced_blender_as_bpy`, `bpy.ops.render` (partial). Release notes: `cycles.md`, `rendering.md`, `core.md`, `eevee.md` (partial), plus searches over all nineteen. Cycles' files: `properties.py`, `ui.py` (relevant panels), `presets.py`, `engine.py` (partial), `__init__.py`.

**Picture opened:** `_images/editors_preferences_section_system.png`.

**Ruled out and why:** `cycles_4.5/source/` (262 files) and `cycles_4.5/shader/` — I listed them and they are OSL shader node files and kernel maths headers (`node_*.oso`, `math_*.h`, `types_*.h`), holding no settings, names, defaults or ranges. `cycles_4.5/osl.py`, `camera.py`, `operators.py`, `version_update.py` — checked for the settings in question and found nothing relevant beyond what is quoted. The 5.0, 5.1 and 5.2 folders in `Blender Sources/` — not opened at all; the question is about 4.5.

**Where a search came back empty, and the terms used:** no setting limiting a render's memory (`memory limit`, `RAM limit`, `max memory`, `memory cap`); no setting for process priority or core pinning (`priority`, `process priority`, `affinity`, `nice value`); no statement that any cost-only setting leaves the image unchanged (`deterministic`, `reproducib`, `identical result`, `same image`, `bit-identical`); no description of the blend-file format or of reading one without Blender.

---

## Things a person with Blender open could settle by hand in a few minutes

This list has been pruned three times. Everything it held that a rented Linux machine could settle was run on 2026-08-06 and is written into the answers above, under the question each one answers. The two items that were left waiting for a working graphics card were run on 2026-08-07, once that machine's card had been repaired. The one list covering every version, with the full procedure for each test and the results, is `Audits/Blender Research/What a render demands/Hand tests.md`. **A third sitting, on the Creator's own Mac on 2026-08-07, ran every item that was still owed there, including the two that need a window — which no machine on this project but that one has.**

**Nothing on this list is still open.**

**Run and answered above:** the thread count and the picture; tiling and the picture; `-t` against the file's Threads Mode; `-t 0`; Threads and Persistent Data under EEVEE; `blender -E help`; reading a render setting out of a file without a person present; what a time limit produces; **whether the processor and the graphics card render the same pixels** (they do not); and **whether `--cycles-device` beats both the Preferences and the scene** (it does, in both directions).

**All of that has now been run, on the Creator's own Mac on 2026-08-07, and is recorded above under the question each one answers.** Apple's own graphics render a different picture from the processor, exactly as the NVIDIA card does; `--cycles-device METAL` beats both the file and the saved preferences, in both directions; the three engines really do show different Performance panels; and the Acceleration Structure controls really do change with the device. **Nothing on this subject is still owed on either machine.**

Three tests were deliberately **not** repeated on the Mac, and the reason is written here rather than left as a silence: the ones whose only possible evidence is a time or a memory figure — whether `-t` changes how much of the processor a Cycles render uses, whether the thread count changes anything under EEVEE or Workbench, and whether Persistent Data changes the time or the memory of an animation. The Mac was shared with other topics' work throughout, so a figure taken there would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. What they leave open on macOS is left open and is not guessed at.

---

2026-08-06 — Opus 5 (Claude Code) — What a render demands, Blender 4.5

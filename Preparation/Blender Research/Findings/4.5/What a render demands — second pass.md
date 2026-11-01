# What a render demands — Blender 4.5 — second pass

**Sources consulted, and when.** Blender's 4.5 manual (plain-text copy), Blender's 4.5 Python API documentation (plain-text copy), Blender's 4.5 release notes, and Cycles' own files as shipped with an installed Blender — all four taken on **2026-07-30**, which is the date read for every quotation from them. Also Blender's requirements page, taken on **2026-08-01**, which is the date read for the one quotation from it. Two pictures from the manual's `_images` folder were opened and are named where used.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

---

## 1. What settings, if any, change how much a render demands of the computer it runs on?

**LABEL: documented** (for the list below, each item is documented as affecting speed, memory or which device is used), with two items marked **observed** where the claim comes from Cycles' own files rather than from a page.

The sources gather most of these under one heading. The manual's Cycles Performance page opens:

> "Properties that affect render speeds or memory consumption.
> There are several presets available to help choose between different trade offs:"
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html (read 2026-07-30)

and names the three presets:

> "Default:
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
> — same page, same date

**Observed** — which settings those three presets actually write is visible in Cycles' own preset file:

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
> — `Blender Sources/cycles_4.5/presets.py`, lines 102–111, inside `class AddPresetPerformance` (read 2026-07-30)

### The list

**A. Settings that live on the scene's general render settings (not on any one engine's block)**

- **Threads Mode** and **Threads**. "Method to determine the maximum number of CPU cores to use while rendering." / "The maximum number of CPU cores to use simultaneously while rendering." Auto-Detect "Automatically chooses the amount of threads to match the number of logical processors on your computer"; Fixed "Manually choose the maximum number threads to use for rendering. This can be useful for example, if you want to use your computer while rendering, you can set the property to a thread count lower the amount of logical processors on your computer." (Cycles Performance page, address above, read 2026-07-30.) The API gives the range: `threads` is "int in [1, 1024], default 1"; `threads_mode` is "enum in ['AUTO', 'FIXED'], default 'AUTO'" — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html (read 2026-07-30).
- **Persistent Data**. "Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage while performing other tasks in Blender." And: "When using multiple View Layers, only data from a single view layer is preserved to keep memory usage within bounds; however, objects shared between view layers are preserved." (Cycles Performance page.) API: `use_persistent_data`, "Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage", "boolean, default False".
- **Resolution X, Y** and **Percentage**. "The number of pixels horizontally and vertically in the image." / "Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image." — https://docs.blender.org/manual/en/4.5/render/output/properties/format.html (read 2026-07-30). API: `resolution_x` "int in [4, 65536], default 1920", `resolution_y` "int in [4, 65536], default 1080", `resolution_percentage` "int in [1, 32767], default 100". The picture `_images/render_output_properties_format_panel.png`, referenced from that page as "Format panel", shows these three fields with the values 1920 px, 1080 px and 100% (documented, from the picture).
- **Render Region** and **Crop to Render Region**. "Renders just a portion of the view instead of the entire frame." / "Crops the rendered image to the size of the render region, instead of rendering a transparent background around it." (Format page.) API: `use_border`, "Render a user-defined render region, within the frame size", "boolean, default False".
- **Simplify** (the whole panel), whose Render column carries **Max Subdivision** ("Maximum number of subdivision by the Subdivision Surface modifiers"), **Child Particles** ("Show only a subset of all child hairs and particles") and **Texture Limit** ("Automatically scales textures down so that they are no larger than the values chosen. This can help reduce computer memory resources when rendering large scenes with huge textures") — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/simplify.html (read 2026-07-30). The same page's Culling section adds **Camera Cull** ("Automatically culls objects based on the camera frustum defined by the Margin") and **Distance Cull** ("Automatically culls objects based on their distance from the active camera"). API: `use_simplify`, "Enable simplification of scene for quicker preview renders", "boolean, default False"; `simplify_subdivision_render`, "Global maximum subdivision level during rendering", "int in [0, 32767], default 0"; `simplify_child_particles_render`, "Global child particles percentage during rendering", "float in [0, 1], default 0.0".
- **High Quality Normals**. API: `use_high_quality_normals`, "Use high quality tangent space at the cost of lower performance", "boolean, default False". The EEVEE and Workbench Performance pages both describe it as "Uses higher precision normals and tangents which can improve visual quality for dense meshes with high frequency textures at the cost of memory." — https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/performance.html and https://docs.blender.org/manual/en/4.5/render/workbench/performance.html (read 2026-07-30).
- **Compositor Device**, **Precision**, **Denoising Device**, **Preview Quality**, **Final Quality**. "The device used for compositing." (CPU / GPU); "The precision of compositor intermediate result" (Auto: "Use full precision for final renders, half precision otherwise"); "The device to use to process Denoise nodes in the compositor"; and the three qualities: "High: Produces the highest quality output at the cost of long processing times. Balanced: Balanced between performance and quality, typically processing in half the time as High, while retaining most of the quality. Fast: Produces an output quickly at a noticeable cost of quality." (Both the Cycles and the EEVEE Performance pages carry this identical block.) API: `compositor_device` "enum in ['CPU', 'GPU'], default 'CPU'"; `compositor_denoise_final_quality` "enum in ['HIGH', 'BALANCED', 'FAST'], default 'HIGH'".
- **Save render cache**. API: `use_render_cache`, "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)", "boolean, default False".
- **Per-view-layer sample override**. "View layer samples to override the scene samples. Controlled by the layer samples in the Sampling panel." — https://docs.blender.org/manual/en/4.5/render/layers/introduction.html (read 2026-07-30). The scene-side switch is **Layer Samples**, "Use: Allow view layers to override the scene-level sample count. Bounded: Allow the overrides as long as they don't exceed the scene-level sample count. Ignore: Ignore the overrides." — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html (read 2026-07-30).

**B. Settings on the Cycles block**

- **Device** (CPU or GPU Compute). **Observed**: `Blender Sources/cycles_4.5/properties.py`, lines 30–35:
  > ```
  > enum_devices = (
  >     ('CPU', "CPU", "Use CPU for rendering"),
  >     ('GPU', "GPU Compute",
  >         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
  > )
  > ```
  and lines 378–383:
  > ```
  >     device: EnumProperty(
  >         name="Device",
  >         description="Device to use for rendering",
  >         items=enum_devices,
  >         default='CPU',
  >     )
  > ```
  The manual ties this to the machine: "GPU rendering makes it possible to use your graphics card for rendering, instead of the CPU. This can speed up rendering because modern GPUs are designed to do quite a lot of number crunching. On the other hand, they also have some limitations in rendering complex scenes, due to more limited memory, and issues with interactivity when using the same graphics card for display and rendering." — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html (read 2026-07-30).
- **Use Tiling** and **Tile Size**. "Render high resolution images in tiles to reduce memory usage. Tiles are cached to disk while rendering to save memory." / "This value is used to control the size of the tile used for rendering; decreasing the size reduces memory usage." plus the note "In some cases changing the Tile Size can result in increased performance. For example when a small object renders slowly compared to other objects, using a small Tiles Size can lead to an increase in performance." (Cycles Performance page.) **Observed** defaults and range, `properties.py` lines 1051–1061:
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
- **Use Spatial Splits**, **Use Curves BVH**, **BVH Time Steps**, **Use Compact BVH**. "Spatial splits improve the rendering performance in scenes with a mix of large and small polygons. The downsides are longer BVH build times and slightly increased memory usage." / "Use a special type of BVH for rendering curves… Disabling this option will reduce memory, at the cost of increasing curve render time." / "Split BVH primitives by this number of time steps to speed up render time at the expense of memory." / "Use a more compact BVH structure, which can reduce RAM usage but render slower." (Cycles Performance page.) **Observed**, `properties.py` lines 920–940, gives their defaults: `debug_use_spatial_splits` default False; `debug_use_hair_bvh` default True; `debug_use_compact_bvh` default False; `debug_bvh_time_steps` default 0, "min=0, max=16".
- **Samples**, **Time Limit**, **Adaptive Sampling** (Noise Threshold, Min Samples). "The number of paths to trace per pixel in the final render. A higher number results in a cleaner image at the cost of a longer render time." / "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit." with the note "The time limit does not include pre-render processing time, only render time." / "cutting short the sampling process in areas that have become less noisy than the specified threshold value" and "Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times." (Cycles Sampling page.) **Observed**, `properties.py` lines 482–487 and 517–525:
  > ```
  >     samples: IntProperty(
  >         name="Samples",
  >         description="Number of samples to render for each pixel",
  >         min=1, max=(1 << 24),
  >         default=4096,
  >     )
  > ```
  > ```
  >     time_limit: FloatProperty(
  >         name="Time Limit",
  >         description="Limit the render time (excluding synchronization time). "
  >         "Zero disables the limit",
  >         min=0.0,
  >         default=0.0,
  >         step=100.0,
  >         unit='TIME_ABSOLUTE',
  >     )
  > ```
- **Sample Subset** (Offset, Length). "Only render a subset of the samples… A typical use case is to distribute the rendering of a single frame over multiple machines." (Cycles Sampling page.)
- **Denoise (Render)**, **Denoiser**, **Passes**, **Prefilter**, **Quality**, **Use GPU**. "Denoising uses a specialized algorithm to get a less noisy image without requiring more samples." Automatic "Uses GPU accelerated denoising if supported, for best performance." Quality: "High: Produces the highest quality at the cost of time… Fast: Sacrifices quality for speed". Use GPU: "Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled." (Cycles Sampling page.)
- **Light Tree** and **Light Threshold**. "This can significantly reduce noise, at the cost of a somewhat longer render time per sample." / "Probabilistically terminates light samples when the light contribution is below the threshold. This avoids wasting time on lights that contribute little to the image." (Cycles Sampling page.)
- **Scrambling Distance** (Automatic, Viewport, Multiplier). "A technique that reduces the randomness between pixels in an attempt to improve GPU rendering performance, at the cost of potential artifacts." / "Values below one will reduce the distance, having the potential to improve GPU rendering performance but increase the visibility of artifacts." (Cycles Sampling page.)
- **Min Light Bounces** and **Min Transparent Bounces**. "Setting this higher gives less noise, but may also increase render time considerably." / "Setting this higher reduces noise in the first bounces, but can also be less efficient for more complex geometry like hair and volumes." (Cycles Sampling page.)
- **Max Bounces** (Total, Diffuse, Glossy, Transmission, Volume, Transparent). "Maximum number of light bounces. For best quality, this should be set to the maximum. However, in practice, it may be good to set it to lower values for faster rendering." — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/light_paths.html (read 2026-07-30).
- **Reflective / Refractive Caustics**, **Filter Glossy**, **Clamping**. "While in principle path tracing supports rendering of caustics with a sufficient number of samples, in practice it may be inefficient to the point that there is just too much noise. This option can be unchecked, to disable reflective caustics." / "this will blur glossy reflections after blurry bounces, to reduce noise at the cost of accuracy." / "It reduces noise at the cost of accuracy." (Light Paths page.)
- **Fast GI Approximation** (Method, AO Factor, AO Distance, Render Bounces). "Approximate diffuse indirect light with background tinted ambient occlusion. This provides fast alternative to full global illumination (GI), for interactive viewport rendering or final renders with reduced quality." (Light Paths page.)
- **Volume Step Rate Render** and **Max Steps**. "Render time can be reduced by increasing the step size, at the cost of potentially losing some volume detail." / "Global multiplier on the step size for all volumes in renders. Increase to reduce render time, at the cost of losing detail." / "Maximum number of steps through the volume before giving up, to protect from extremely long render times with big objects or small step sizes." — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/volumes.html (read 2026-07-30).
- **Render Texture Limit** (Cycles' own copy of the Simplify texture limit). **Observed**, `properties.py` lines 1016–1021, with the choices at lines 185–194: `OFF` "No texture size limit" through `8192`.
- **Dicing Rate Render**, **Offscreen Scale**, **Max Subdivisions**, **Dicing Camera**, but only under the experimental feature set: "Note — These settings are only available if Experimental Feature Set is turned on." / "Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory." — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/subdivision.html (read 2026-07-30).
- **Path Guiding** (Training Samples, Surface, Volume). "Path guiding is only available when rendering on a CPU." / "Higher values can lead to a minor increases in guiding quality but with increased render times." (Cycles Sampling page.)

**C. Settings on the EEVEE block**

- **Render Samples**. "The number of samples to use in the final render." — https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html (read 2026-07-30). API: `taa_render_samples`, "Number of samples per pixel for rendering", "int in [1, inf], default 64".
- **Shadow Pool** and **Light Probes Volume Pool**. "A bigger pool size allows for more shadows in the scene but might not fit into GPU memory and decreases performance. Increasing the size might fix the Shadow buffer full error." / "A bigger pool size allows for more irradiance grids in the scene but might not fit into GPU memory and decreases performance." (EEVEE Performance page.) API: `shadow_pool_size` "enum in ['16', '32', '64', '128', '256', '512', '1024'], default '512'"; `gi_irradiance_pool_size` same choices, "default '16'".
- **Shadow Rays**, **Steps**, **Resolution**, **Volumetric Shadows**. "Number of rays to trace for each light. Higher values reduces the noise caused by random shadow sampling." / "Higher step count results in softer shadows but have a higher cost." / "Resolution percentage of shadow maps." / "This is a very computationally expensive option and has limitations." (EEVEE Sampling page.) API: `shadow_ray_count` "int in [1, 4], default 1"; `shadow_step_count` "int in [1, 16], default 6"; `shadow_resolution_scale` "float in [0, 1], default 1.0".
- **Light Threshold**. "Minimum light intensity for a light to contribute to the lighting. Used to compute the distance at which to cut-off lights influence. Lower values improve performance." (EEVEE Sampling page.)
- **Raytracing** — the master switch, **Method**, **Resolution**, **Precision**, and the Fast GI block. "When disabled, it is replaced by a faster pipeline that uses pre-filtered light-probes." / "Light Probe: … This option has the lowest tracing cost but relies on manually placed light-probes." / "Resolution at which the ray-tracing is performed. Lower options will be faster and use less memory but will produce blurrier results." / "Increased precision also increases performance cost." — https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/raytracing.html (read 2026-07-30). API: `use_raytracing`, "Enable the ray-tracing module", "boolean, default False".
- **Volumes Resolution** and **Steps**. "To achieve this, EEVEE uses several 3D textures which have a high video memory usage. The texture dimensions can be tweaked using the Resolution and Steps parameters." — https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/volumes.html (read 2026-07-30).
- **Overscan**. "Percentage of the render size to add to the internal render buffer. This will have a serious impact on performance but can fix render glitches around the perimeter of the rendered image." — https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/film.html (read 2026-07-30).

**D. Settings on the Workbench block**

- **Anti-Aliasing method for final rendering**. "The quality of the renders can be adjusted by changing the Anti-Aliasing method." / "Multisample — The scene will be rendered multiple times with a slight offset." with the choices "5, 8, 11, 16, 32" — https://docs.blender.org/manual/en/4.5/render/workbench/sampling.html (read 2026-07-30). API: `render_aa`, "Method of anti-aliasing when rendering final image", "enum in ['OFF', 'FXAA', '5', '8', '11', '16', '32'], default '8'".

**E. Settings that live in the preferences rather than in the scene**

- **Cycles Render Device** (None / CUDA / OptiX / HIP / oneAPI / Metal). "Changes the computing device the Cycles render engine uses to render images." — https://docs.blender.org/manual/en/4.5/editors/preferences/system.html (read 2026-07-30). The picture `_images/editors_preferences_section_system.png`, referenced from that page as "Preferences System section", shows a "Cycles Render Devices" panel heading, collapsed, above a separate "Memory & Limits" panel (documented, from the picture — the device choices themselves are not visible in it).
- **Distribute Memory Across Devices**. "Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes." **Observed**, `properties.py` lines 1594–1598, `peer_memory`, default False.
- **MetalRT**, **HIP RT**, **Embree on GPU**. "MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases." / "Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above." / "Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance." (Preferences System page.) **Observed** defaults, `properties.py` lines 1600–1622: `metalrt` default `'AUTO'`; `use_hiprt` default False; `use_oneapirt` default True.
- **Kernel Optimization**. **Observed**, `properties.py` lines 1624–1637:
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
- **Display Graphics Backend** (OpenGL or Vulkan). "Vulkan may offer improved performance and better support for modern GPU features, but compatibility may vary depending on the system and drivers." (Preferences System page.)
- **Shader Compilation Method** and **Threads / Subprocesses**. "Subprocess: Uses multiple separate subprocesses to compile shaders in parallel. This can be faster, especially on systems with high core counts, but consumes significantly more RAM." / "The number of shader compilation threads or subprocesses to use. The maximum value is limited by the number of logical CPU cores on the system. Increasing the number can reduce shader compilation time at the cost of higher memory usage. A value of 0 lets Blender automatically choose a suitable number based on the system configuration." (Preferences System page.) The 4.5 release notes record its arrival: "Shader compilation is now multi-threaded by default on platform that benefits from it. A new user preference setting was added to choose between multi-threaded and subprocess compilation." — https://developer.blender.org/docs/release_notes/4.5/eevee/ (read 2026-07-30).
- **GPU Subdivision**. "Under certain circumstances, the GPU will be used to subdivide a mesh with a Subdivision Surface modifier. This typically results in increased subdivision performance." — https://docs.blender.org/manual/en/4.5/editors/preferences/viewport.html (read 2026-07-30).

**Not covered.** This is the list of settings the sources *describe* in terms of speed, memory or device. It is not a list of everything that changes the demand — the sources say plainly that scene content does too ("One way to reduce memory usage is by using smaller resolution textures. For example, 8k, 4k, 2k, and 1k image textures take up respectively 256MB, 64MB, 16MB and 4MB of memory", GPU Rendering page), and I have deliberately not tried to extend the list to every material, modifier and object setting that has a cost. Nothing here says by how much any setting changes the demand; the wording is comparative ("faster", "less memory") and never numeric, apart from the texture example just quoted. I have also not covered the Grease Pencil engine, or per-object settings, or the sequencer.

**Outside the interface.** All of section A–D live on scene data in the file, and the manual's command-line render page describes rendering with "all the settings saved in the blend-file" (https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30), so they apply. Two documented differences: the viewport-only members of these panels (Viewport Samples, Pixel Size, Viewport Denoise, viewport Simplify column, Viewport BVH Type, Preview Quality for the compositor's denoise nodes) are described as applying to the 3D Viewport, which does not exist in background mode; and **Render Single Layer** is explicitly "ignored when rendering from the command line" (https://docs.blender.org/manual/en/4.5/render/layers/view_layer.html, read 2026-07-30) — that one is a documented case of an interface-only setting sitting next to the sample-override setting listed above. Section E lives in the preferences, and the sources do not say whether preferences are read when Blender is started with `-b`; the only thing said about skipping saved state is `--factory-startup`, "Skip reading the startup.blend in the users home directory" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30), which mentions the startup file and not the preferences.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

**Operating system.** Several items are documented as not holding everywhere. Cycles' GPU technologies are per-system: "CUDA is supported on Windows and Linux"; "OptiX is supported on Windows and Linux"; "HIP is supported on Windows and Linux"; "oneAPI is a computation library that is supported on Windows and Linux"; "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." (GPU Rendering page.) MetalRT is a Metal-only setting, HIP RT a HIP-only one, Embree on GPU "Only supported with oneAPI rendering devices" (Preferences System page). Shader Compilation Method: "This option is not available on macOS and requires using OpenGL backend" (Preferences System page). GPU Subdivision: "This feature is not supported on Qualcomm GPUs on Windows" (Preferences Viewport page). EEVEE: "Headless rendering is not supported on headless Windows systems" (https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html, read 2026-07-30). For the rest — threads, tiling, samples, resolution, simplify — the sources say nothing about any operating-system difference.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

**Where I looked.** Searched the whole 4.5 manual for `performance` (129 files matched; read every one under `render/`, plus `editors/preferences/system.txt`, `editors/preferences/viewport.txt`, `advanced/command_line/arguments.txt`, `getting_started/configuration/hardware.txt`), then for `memory` across `render/` and `scene_layout/`. Read in full: the Cycles, EEVEE and Workbench Performance pages; Cycles Sampling, Simplify, Light Paths, Volumes, Subdivision, GPU Rendering; EEVEE Sampling, Raytracing, Volumes, Clamping, Film, Limitations; Workbench Sampling; Output Format; View Layers introduction and View Layer; Preferences System. In the API documentation, read `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE`, `bpy.types.SceneDisplay`, `bpy.types.Scene`, `bpy.types.BlendData`, `bpy.types.BlendDataLibraries`, `bpy.data`, `info_api_reference`, `info_tips_and_tricks`, `bpy.app`. In Cycles' files, listed every scene-level property in `properties.py` (a grep for property declarations returned 160 of them) and read the blocks named above, plus all of `presets.py` and `engine.py`, and the Performance panel classes in `ui.py`. In the release notes, searched `rendering.md`, `cycles.md`, `eevee.md`, `core.md`, `python_api.md` for `performance`, `memory`, `thread`, `tile`, `device`, `persistent`. Terms searched beyond the obvious ones: `headless`, `expensive`, `VRAM`, `RAM`, `out of memory`, `cache`.

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

**LABEL: documented**, for the settings where the sources say; **not in the sources** for a number of them, listed at the end.

I take "what the render produces" to mean the image (or images) that come out. The question is ambiguous in one respect: it could also mean the files written. I answer the image reading, which is what the sources address; on the files-written reading the sources say nothing about any of these settings changing the output files, except Sample Subset, which is designed to produce partial images that are later merged.

**Changes nothing about the image (the sources describe them as pure speed or memory trade-offs):**

- Threads Mode / Threads — described only as "the maximum number of CPU cores to use". No effect on the image is stated.
- Persistent Data — "Keep render data in memory after rendering for faster re-renders". No effect on the image is stated.
- Use Tiling / Tile Size — "to reduce memory usage". No effect on the image is stated.
- Use Spatial Splits, Use Curves BVH, BVH Time Steps, Use Compact BVH — all four are stated as build-time / render-time / memory trade-offs only.
- Cycles Device (CPU vs GPU) — the sources describe speed and memory differences, and list features unavailable on GPU ("Path Guiding is not supported on any GPU"; "Open Shading Language is only supported for OptiX"; "Shadow caustics are not supported with HIP"). So it changes the image only by making certain features unavailable — that part is documented; a general statement that CPU and GPU otherwise match is not in the sources.
- Distribute Memory Across Devices, Kernel Optimization, MetalRT, HIP RT, Embree on GPU, Shader Compilation Method — speed/memory only.
- Compositor Device / Precision — "The precision of compositor intermediate result", with Auto "Use full precision for final renders, half precision otherwise". For final renders both settings say full precision, so no image change is stated for final renders.

**Changes the image, and the sources say how:**

- **Samples** — "A higher number results in a cleaner image at the cost of a longer render time."
- **Time Limit** — "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet." (**Inferred**, one step: if it stops early, fewer samples are taken than asked for, and by the Samples sentence just quoted the image is correspondingly less clean. The step a reader has to supply is the link between "sample count not reached" and "noisier image".)
- **Noise Threshold / Min Samples** — "lower values meaning better quality but longer render times."
- **Resolution X, Y, Percentage** — "The number of pixels horizontally and vertically in the image"; Percentage "reduce or increase the size of the rendered image".
- **Render Region / Crop to Render Region** — "Renders just a portion of the view instead of the entire frame"; crop "Crops the rendered image to the size of the render region, instead of rendering a transparent background around it."
- **Simplify Max Subdivision** — "Maximum number of subdivision by the Subdivision Surface modifiers." **Child Particles** — "Show only a subset of all child hairs and particles." **Texture Limit** — "Automatically scales textures down so that they are no larger than the values chosen."
- **Camera Cull / Distance Cull** — objects are culled, i.e. removed from the render.
- **Denoising** — "a specialized algorithm to get a less noisy image without requiring more samples", and per quality "High: Produces the highest quality at the cost of time… Fast: Sacrifices quality for speed". Passes: "It is recommended to use at least Albedo as None can blur out details, especially at lower sample counts."
- **Denoise Use GPU** — no image change stated, only "significantly faster than on CPU, but requires additional GPU memory".
- **Light Tree** — "This can significantly reduce noise… Certain lighting properties are not accounted for in the light tree… This can result in an increase in noise in some scenes that make use of these features."
- **Light Threshold** — "Probabilistically terminates light samples when the light contribution is below the threshold."
- **Scrambling Distance** — "at the cost of potential artifacts" and "increase the visibility of artifacts".
- **Min Light Bounces** — "Setting this higher gives less noise". **Min Transparent Bounces** — "Setting this higher reduces noise in the first bounces".
- **Max Bounces** — "For best quality, this should be set to the maximum… A value of 0 bounces results in direct lighting only."
- **Caustics off, Filter Glossy, Clamping** — "to reduce noise at the cost of accuracy"; clamping "It reduces noise at the cost of accuracy" and "as you clamp out such values, other bright lights/reflections will be dimmed as well".
- **Fast GI Approximation** — "final renders with reduced quality"; it replaces or adds ambient occlusion in place of full global illumination.
- **Volume Step Rate Render** — "Increase to reduce render time, at the cost of losing detail." **Max Steps** — "Maximum number of steps through the volume before giving up".
- **Sample Subset** — "Only render a subset of the samples", with the warning "it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets – otherwise, the subsets will have incompatible noise and combining them will give a worse result."
- **Path Guiding** — "helps reduce noise in scenes where finding a path to a light source is difficult".
- **Dicing Rate Render / Offscreen Scale / Max Subdivisions** — "Size of Micropolygons in pixels"; "Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory"; "Stop subdividing when this level is reached even if the dicing rate would produce finer Tessellation."
- **High Quality Normals** — "can improve visual quality for dense meshes with high frequency textures at the cost of memory."
- **EEVEE Render Samples** — the page's own framing: "TAA is sample based so the more samples the more aliasing is reduced at the cost of performance."
- **EEVEE Shadow Pool / Light Probes Volume Pool** — "A bigger pool size allows for more shadows in the scene"; "allows for more irradiance grids in the scene". **Inferred**, one step: if the pool is too small for the scene's shadows, some are not accommodated — the page mentions a "Shadow buffer full error" that increasing the size "might fix". The sources do not say what the image looks like when the pool is full.
- **EEVEE Shadow Rays / Steps / Resolution** — "Higher values reduces the noise caused by random shadow sampling"; "Higher step count results in softer shadows"; "Resolution percentage of shadow maps".
- **EEVEE Light Threshold** — "Minimum light intensity for a light to contribute to the lighting."
- **EEVEE Raytracing off** — "it is replaced by a faster pipeline that uses pre-filtered light-probes. This fallback mode offers a more visually stable and optimized alternative when visual fidelity is not the primary goal." Resolution: "Lower options will be faster and use less memory but will produce blurrier results."
- **EEVEE Volumes Resolution / Steps** — "Controls the quality of the volumetric effects."
- **EEVEE Overscan** — "can fix render glitches around the perimeter of the rendered image."
- **Workbench Anti-Aliasing** — "The quality of the renders can be adjusted by changing the Anti-Aliasing method"; "With this option selected no anti-aliasing will be applied."
- **Compositor Denoise Node quality (Final Quality)** — "High: Produces the highest quality output at the cost of long processing times… Fast: Produces an output quickly at a noticeable cost of quality."
- **Per-view-layer sample override / Layer Samples** — changes how many samples a layer gets, hence, by the Samples sentence, how clean it is (**inferred**, same one step as Time Limit).

**Not in the sources, for these:** what Save Render Cache changes about the image; what GPU Subdivision changes about a final render's image (the page speaks of subdividing a mesh and of performance, not of the rendered result); what the Display Graphics Backend choice changes about a rendered image; what EEVEE Volumes Distribution and Max Depth change beyond "Will create artifacts if it is exceeded"; and, for every setting above, the *size* of the change.

**Not covered.** No source here gives a numeric relationship for anything — not samples to noise, not tile size to memory, not resolution to time. Nothing here says which of these changes are visible at all in a given scene. Nothing settles whether two machines with different thread counts produce byte-identical images.

**Outside the interface.** These are descriptions of what the settings do to the image, and the sources attach no interface condition to any of them, except that the viewport-only variants (Viewport Samples, Preview Denoiser, viewport Simplify, Pixel Size) describe the 3D Viewport, which does not exist in background mode.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Operating system.** The sources state no operating-system difference in what any of these does to the image. The device-availability limits quoted in question 1 apply, and where a feature is unavailable on a platform's device, its image effect is unavailable with it — that much follows from the quoted limitation lines, but the sources do not spell it out.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Where I looked.** The same pages as question 1, read for the phrases describing the result rather than the cost: `quality`, `noise`, `noisier`, `blur`, `detail`, `artifact`, `accuracy`, `aliasing`. Where a page describes a setting purely in terms of time or memory I have said so rather than filling the gap.

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

**LABEL: documented** for the arguments that exist; **inferred** for the wider claim about Python; **not in the sources** for whether preferences are honoured in background mode.

### Settled by a dedicated command-line argument

From the command-line arguments page, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (read 2026-07-30), whose header line is "Blender 4.5 LTS":

- **Engine** — "-E, --engine <engine> — Specify the render engine. Use -E help to list available engines."
- **Threads** — "-t, --threads <threads> — Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count." The render page's worked example spells it out: "-t 2 — Use only two threads." (https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30.)
- **Cycles device** — "--cycles-device <device> — Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. Append +CPU to a GPU device to render on both CPU and GPU." The page notes the placement rule: "Cycles add-on options must be specified following a double dash", with the example `blender -b file.blend -f 20 -- --cycles-device OPTIX`.
- **Display graphics backend** — "--gpu-backend — Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."
- **Shader compilation subprocess count** — "--gpu-compilation-subprocesses — Override the Max Compilation Subprocesses setting (OpenGL only)."
- **Cycles statistics** — "--cycles-print-stats — Log statistics about render memory and time usage." (Not a demand setting; it reports the demand.)

**Observed** — Cycles' own file is more specific than the manual about what `--cycles-device` overrides:

> ```
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)
> ```
> — `Blender Sources/cycles_4.5/engine.py`, lines 16–20, in `_configure_argument_parser` (read 2026-07-30)

and the same file shows the parsing only happens when a double dash is present:

> ```
>     argv = sys.argv
>     if "--" not in argv:
>         return
> ```
> — `Blender Sources/cycles_4.5/engine.py`, lines 27–29, in `_parse_command_line` (read 2026-07-30)

### No dedicated argument exists

Every other setting in question 1 has no argument on that page. That includes Samples, Time Limit, Noise Threshold, Tile Size and Use Tiling, Persistent Data, Resolution X/Y and Percentage, Render Region, all four BVH options, Simplify and its Texture Limit, the culling options, denoising, Max Bounces, Clamping, Volume Step Rate, the Cycles scene Device enum (CPU vs GPU Compute — as opposed to `--cycles-device`, which is the technology), all EEVEE settings, and the Workbench anti-aliasing method. The manual presents its list as complete: "See Command Line Arguments for a full list of arguments" (command-line render page). I read the arguments page in full and searched it for `sample`, `resolution`, `memory`, `tile` and `persistent` — the only hits are unrelated (`-c <cache_memory>` belongs to the animation player, `--debug-memory` is a debugging switch).

### The general route, which is not "changing the file"

**LABEL: inferred.** The arguments page documents "-P, --python <filepath> — Run the given Python script file" and "--python-expr <expression> — Run the given expression as a Python script", and it documents that arguments run in order: "Arguments are executed in the order they are given." The API documentation documents that these settings are reachable from Python — "bpy.context.scene.cycles.samples" is given as a worked example of accessing "the number of samples for the Cycles render engine" (https://docs.blender.org/api/4.5/info_api_reference.html, read 2026-07-30) — and the tips page documents the shape `blender myscene.blend --background --python myscript.py` (https://docs.blender.org/api/4.5/info_tips_and_tricks.html, read 2026-07-30). Putting those together: any of these settings can be set at the moment a render starts, by a script run after the file is loaded and before the render argument, without the file on disk being changed. **The step the reader must supply** is that a value set by such a script is in force for a render triggered later in the same command — no source states that in so many words. That is why this is inferred, not documented.

The ordering trap is documented, and it applies to this route as much as to output paths: "blender --background --render-output /tmp test.blend --render-frame 1 …will not render to /tmp because loading the blend-file overwrites the render output that was set." (arguments page). And the render page's blunt version: "Always position -f or -a as the last arguments."

### So: which only by changing the file?

**LABEL: not in the sources**, strictly. Given the Python route above, the sources name no setting that can *only* be changed by editing the file. What the sources do settle is the narrower thing: which settings have a dedicated argument (the six listed above) and which do not (everything else).

**Not covered.** Whether preferences-level settings (Cycles Render Device, MetalRT, HIP RT, Embree on GPU, Kernel Optimization, Distribute Memory Across Devices, GPU Subdivision, Shader Compilation Method) are read at all when Blender is started with `-b`, and whether a script can change them for the current run. `--cycles-device` is documented (observed, from `engine.py`) to override the user preferences for the Cycles device specifically; nothing covers the others. Also not covered: whether `-t` overrides the file's Threads Mode or is overridden by it, and what happens if both are given. Also not covered: what `-c help` lists, and whether any of its commands touch render settings.

**Outside the interface.** This whole question is about outside the interface, so the answer is native to that setting. The one thing worth carrying: `-b` is described as "Run in background (often used for UI-less rendering)", and the command-line render page adds "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Operating system.** `--gpu-compilation-subprocesses` is marked "(OpenGL only)", and the preference it overrides is documented as "not available on macOS and requires using OpenGL backend". `--cycles-device`'s valid values are only meaningful where the technology is supported, per the per-platform list quoted in question 1. `-con, --start-console` is "(Windows only)" but is not a demand setting. The remaining arguments carry no platform note.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Where I looked.** Read `advanced/command_line/arguments.txt` and `advanced/command_line/render.txt` in full; searched the arguments page for `sample`, `resolution`, `memory`, `tile`, `persistent`, `render-`. Searched the whole manual for `CYCLES_[A-Z_]*` (two hits, both `CYCLES_CUDA_EXTRA_CFLAGS` on the GPU Rendering page, which is about compiling a CUDA kernel, not about demand) and for `background mode`, `command line`, `command-line`, `headless` across `render/`, `advanced/` and `editors/preferences/`. Searched Cycles' own `.py` files for `CYCLES_[A-Z_]*` (only class-name prefixes) and for `add_argument` / `argv` (only `engine.py`). Searched the API documentation for `background`. Read `info_tips_and_tricks`, `info_api_reference`, `bpy.app`.

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

**LABEL: documented**, with one part **observed**.

Yes. The split falls three ways.

**Cycles only.** Everything reached through the scene's `cycles` block. **Observed** — Cycles is an add-on that registers its own settings onto the scene:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```
> — `Blender Sources/cycles_4.5/properties.py`, lines 1100–1106, inside `class CyclesRenderSettings` (read 2026-07-30)

and its panels are declared to appear only for Cycles:

> ```
> class CyclesButtonsPanel:
>     bl_space_type = "PROPERTIES"
>     bl_region_type = "WINDOW"
>     bl_context = "render"
>     COMPAT_ENGINES = {'CYCLES'}
>
>     @classmethod
>     def poll(cls, context):
>         return context.engine in cls.COMPAT_ENGINES
> ```
> — `Blender Sources/cycles_4.5/ui.py`, lines 60–68 (read 2026-07-30)

So: Device (CPU/GPU Compute), Use Tiling, Tile Size, the four BVH options, Samples, Time Limit, adaptive sampling, Sample Subset, denoising and its device, Light Tree, Light Threshold, Scrambling Distance, min bounces, Max Bounces, caustics, Filter Glossy, Clamping, Fast GI, volume step rate and max steps, Render Texture Limit, dicing/subdivision, Path Guiding — all Cycles.

**EEVEE only.** Everything on the scene's `eevee` block: Render Samples, Shadow Pool, Light Probes Volume Pool, shadow rays/steps/resolution, volumetric shadows, EEVEE Light Threshold, Raytracing and its resolution/precision/Fast GI, Volumes resolution/steps, Overscan. The API records the block as "eevee — EEVEE settings for the scene — Type: SceneEEVEE" (https://docs.blender.org/api/4.5/bpy.types.Scene.html, read 2026-07-30). One EEVEE limitation is itself an engine-level fact about demand: "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient. CPU power is still needed to handle high scene complexity as the geometry must be prepared by the CPU before rendering each frame." and "There is currently no support for multiple GPU systems." (https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html, read 2026-07-30).

**Workbench only.** The final-render Anti-Aliasing method, on the scene's `display` block — "display — Scene display settings for 3D viewport — Type: SceneDisplay" (https://docs.blender.org/api/4.5/bpy.types.Scene.html), with `render_aa` documented as "Method of anti-aliasing when rendering final image".

**Not tied to an engine in the data, but exposed by only one engine's panel.** Threads Mode, Threads, Persistent Data and High Quality Normals live on the scene's general render settings, which the API documents without any engine qualification (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html). But the manual documents Threads and Persistent Data **only** on the Cycles Performance page — I searched the whole 4.5 manual for `Persistent Data` and `persistent_data` (one hit, that page) and for `Threads` under `render/` (four hits, all on the Cycles Performance page and the Cycles render-settings index). **Observed**, Cycles' UI file draws them from the general render settings inside Cycles-only panels:

> ```
> class CYCLES_RENDER_PT_performance_threads(CyclesButtonsPanel, Panel):
>     bl_label = "Threads"
>     bl_parent_id = "CYCLES_RENDER_PT_performance"
>
>     def draw(self, context):
>         layout = self.layout
>         layout.use_property_split = True
>         layout.use_property_decorate = False
>
>         scene = context.scene
>         rd = scene.render
>
>         col = layout.column()
>
>         col.prop(rd, "threads_mode")
> ```
> — `Blender Sources/cycles_4.5/ui.py`, lines 829–843 (read 2026-07-30)

> ```
> class CYCLES_RENDER_PT_performance_final_render(CyclesButtonsPanel, Panel):
>     bl_label = "Final Render"
>     bl_parent_id = "CYCLES_RENDER_PT_performance"
> …
>         col.prop(rd, "use_persistent_data", text="Persistent Data")
> ```
> — `Blender Sources/cycles_4.5/ui.py`, lines 917–931 (read 2026-07-30)

Note the important limit on that observation: this shows *where the setting is shown*, not *which engines act on it*. Whether EEVEE or Workbench honour `threads` or `use_persistent_data` is **not in the sources**.

**Belongs to every engine.** Resolution X/Y and Percentage, Render Region and Crop, Simplify (whose page is under the Cycles section of the manual but whose properties the API documents on the general render settings without engine qualification), Save Render Cache, and the Compositor device/precision/denoise-quality block — the last of these appears identically on the Cycles Performance page and the EEVEE Performance page, which is the sources' own way of saying it is not Cycles-specific. The View Layer page says the same about its own panel: "This section covers only the Render Layer settings appropriate for the Cycles renderer. For the engine-independent settings, see this section." (https://docs.blender.org/manual/en/4.5/render/layers/introduction.html, read 2026-07-30.)

**Preferences.** Cycles Render Device, Distribute Memory Across Devices, MetalRT, HIP RT, Embree on GPU and Kernel Optimization are Cycles-specific — the manual's Preferences System page calls the first "the computing device the Cycles render engine uses", and the rest are declared in Cycles' own `CyclesPreferences` class (`properties.py`, line 1555). Display Graphics Backend, Shader Compilation Method and GPU Subdivision are not attached to any engine on that page.

**Not covered.** Whether a general setting that only Cycles displays is actually used by the other engines. Whether Simplify's Render column applies to EEVEE and Workbench renders — its page sits in the Cycles section and does not say. The Grease Pencil engine is not covered at all here. Third-party engines are not covered.

**Outside the interface.** The engine in force at render time can be chosen from outside: "-E, --engine <engine> — Specify the render engine." (arguments page). Which engine is chosen decides which of the three blocks above matters. The panel-visibility distinction drawn above is an interface fact and has no meaning in background mode — the underlying properties still exist and are still readable and writable there.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Operating system.** EEVEE's engine-level limits are partly per-platform: "Headless rendering is not supported on headless Windows systems." Cycles' device technologies are per-platform, as quoted in question 1. Workbench: the sources state no platform difference.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Where I looked.** `properties.py` (register blocks for `Scene.cycles`, and `CyclesPreferences`), `ui.py` (`COMPAT_ENGINES`, the Performance panel classes), `presets.py`. In the manual: the Cycles, EEVEE and Workbench Performance pages side by side; EEVEE Limitations; View Layers introduction; Preferences System. In the API: `bpy.types.Scene` (for the `cycles`, `eevee`, `display`, `render` pointers — note `cycles` is absent from the API documentation, as Cycles is an add-on: a search of the whole 4.5 API folder for `scene.cycles` returns exactly one file, the reference-intro page, and there is no `bpy.types.CyclesRenderSettings` page), `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE`, `bpy.types.SceneDisplay`.

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

**LABEL: documented** for reading them from a file that Blender has open; **inferred** for reading them out of an arbitrary file on disk; **not in the sources** for reading them without Blender.

### From a file Blender has open — documented

The API reference states the general access shape and gives one of these very settings as its worked example:

> "# Access the number of samples for the Cycles render engine.
> bpy.context.scene.cycles.samples"
> — Blender 4.5 Python API documentation, https://docs.blender.org/api/4.5/info_api_reference.html (read 2026-07-30)

The data root is documented as the file itself:

> "Main data structure representing a .blend file and all its data-blocks"
> — https://docs.blender.org/api/4.5/bpy.types.BlendData.html (read 2026-07-30)

with a scene collection on it — "scenes — Scene data-blocks — Type: BlendDataScenes bpy_prop_collection of Scene, (readonly)" (same page) — and the render settings hanging off each scene: "render — Type: RenderSettings, (readonly, never None)" and "eevee — EEVEE settings for the scene — Type: SceneEEVEE, (readonly)" and "display — Scene display settings for 3D viewport — Type: SceneDisplay, (readonly)" (https://docs.blender.org/api/4.5/bpy.types.Scene.html, read 2026-07-30). The `(readonly)` marking there is on the pointer — you cannot swap the settings block for another — while the individual settings under it are documented with plain types and defaults and no readonly marking, e.g. "threads — Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) — Type: int in [1, 1024], default 1" (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30). Every setting listed in question 1 is documented this way, on `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE` or `bpy.types.SceneDisplay`, except the Cycles ones, which are not in the API documentation at all (Cycles is an add-on) but are declared in its own file — for example, **observed**, `properties.py` lines 482–487 for `samples`, quoted under question 1.

The API also documents where the open file came from: "filepath — Path to the .blend file — Type: string, default "", (readonly, never None)".

And the reading can be done with no interface: "blender myscene.blend --background --python myscript.py" (https://docs.blender.org/api/4.5/info_tips_and_tricks.html, read 2026-07-30), with "bpy.app.background — Boolean, True when blender is running without a user interface (started with -b)" (https://docs.blender.org/api/4.5/bpy.app.html, read 2026-07-30).

### From another file, without opening it as the current file — inferred

`bpy.data.libraries.load` is documented as taking "filepath (str | bytes) – The path to a blend file" and returning "a context manager which exposes 2 library objects on entering. Each object has attributes matching bpy.data which are lists of strings to be linked", with this example:

> "# Load a single scene we know the name of.
> with bpy.data.libraries.load(filepath) as (data_from, data_to):
> data_to.scenes = ["Scene"]"
> — https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html (read 2026-07-30)

**The step I am taking:** the docs say a named scene can be loaded from another blend file, and separately that a Scene carries `render`, `eevee` and `display`; from those two the settings of a scene in another file are reachable. No source says "you can read another file's render settings this way", and nothing there tells you the scene names in advance beyond the note that the `data_from` object holds "lists of strings". Hence **inferred**, not documented.

### Without Blender at all — not in the sources

Nothing in the manual, the API documentation, the release notes or Cycles' files describes the layout of a .blend file, or any tool for reading values out of one without running Blender. The nearest statement is about the file's compression, not its contents: "It is not unique to Blender so files can be compressed/decompressed with external tools." (https://docs.blender.org/manual/en/4.5/files/blend/open_save.html, read 2026-07-30).

**Not covered.** Whether preferences-level settings (Cycles Render Device, MetalRT, Kernel Optimization, Shader Compilation Method, GPU Subdivision) can be read out of a .blend at all — they are preferences, not scene data, and the sources do not say where they are stored or how to read them from a given file. Whether reading a setting from a file requires the add-on that owns it to be enabled — the Cycles settings only exist on the scene because Cycles registers them (`properties.py` register block quoted in question 4), so reading `scene.cycles` from a file with Cycles disabled is not covered. Whether loading a file to read it has side effects (script execution, dependency-graph building) is not covered here, though the arguments page does mention "--disable-depsgraph-on-file-load — Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or -c options)". Nothing covers how fast or how memory-hungry reading a file this way is.

**Outside the interface.** This holds outside the interface, and the sources address that case directly: `--background --python` is the documented shape, and `bpy.app.background` exists to tell a script it is in that state. Nothing here needs a window.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Operating system.** The sources state no operating-system difference for reading values this way. The only nearby platform note is about file paths and encodings, which the API documentation treats on its own page and which I did not need for this answer.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Where I looked.** API documentation: `bpy.data`, `bpy.types.BlendData`, `bpy.types.BlendDataLibraries`, `bpy.types.Scene`, `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE`, `bpy.types.SceneDisplay`, `info_api_reference`, `info_tips_and_tricks`, `bpy.app`, `bpy.ops.wm` (for `open_mainfile`). Searched the whole 4.5 API folder for `blend_render_info` (no hits), `libraries.load`, `scene.cycles` (one hit). Searched the manual under `files/` for `file format`, `binary`, `DNA`, `without opening`, `external tool`, `third-party`, `inspect the file`; read `files/blend/open_save.txt` and listed the whole `files/blend/` folder (`compatibility`, `index`, `open_save`, `packed_data`, `previews`, `rename` — none describes the format). Searched the release notes for anything on file reading. Ruled out the requirements page as irrelevant to this question: it names 4.5 only in a hardware-support list ("Blender 4.5 LTS supports NVIDIA GeForce 400 and newer. AMD GCN 1st gen and newer. Intel Broadwell architecture and newer.", https://www.blender.org/download/requirements/, read 2026-08-01) and says nothing about settings or files.

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


## Things a person with Blender open could settle in a few minutes

This list has been pruned three times. Everything it held that a rented Linux machine could settle was run on 2026-08-06 and is written into the answers above, under the question each one answers. The item that was left waiting for a working graphics card was run on 2026-08-07, once that machine's card had been repaired. The one list covering every version, with the full procedure for each test and the results, is `Audits/Blender Research/What a render demands/Hand tests.md`. **A third sitting, on the Creator's own Mac on 2026-08-07, ran every item that was still owed there, including the two that need a window — which no machine on this project but that one has.**

**Nothing on this list is still open.**

**Run and answered above:** `-t` against the file's Threads setting; whether preferences are honoured with `-b`; EEVEE and the Threads setting; Persistent Data under EEVEE; Simplify's Render column across the engines; Tile Size against memory and against the picture; `bpy.data.libraries.load`; `scene.cycles` with the add-on off; what the Performance presets write; and **whether processor and card renders of the same file match** — they do not, and the difference is far beyond the run-to-run wobble of either device.

**All of that has now been run, on the Creator's own Mac on 2026-08-07, and is recorded above under the question each one answers.** Apple's own graphics render a different picture from the processor, exactly as the NVIDIA card does; `--cycles-device METAL` beats both the file and the saved preferences, in both directions; the three engines really do show different Performance panels; and the Acceleration Structure controls really do change with the device. **Nothing on this subject is still owed on either machine.**

Three tests were deliberately **not** repeated on the Mac, and the reason is written here rather than left as a silence: the ones whose only possible evidence is a time or a memory figure — whether `-t` changes how much of the processor a Cycles render uses, whether the thread count changes anything under EEVEE or Workbench, and whether Persistent Data changes the time or the memory of an animation. The Mac was shared with other topics' work throughout, so a figure taken there would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. What they leave open on macOS is left open and is not guessed at.

---

2026-08-06 — Opus 5 (Claude Code) — What a render demands, Blender 4.5

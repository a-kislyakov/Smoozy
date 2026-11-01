# What a render demands — findings for Blender 5.3

These findings record what Blender 5.3's documentation and source files establish about the computer resources a render demands, how those demands can be controlled, and what those controls change about the result.

- **Sources consulted, and dates.** The Blender 5.3 manual (plain-text copy, with its pictures), the Blender 5.3 Python API documentation and the Blender 5.3 release notes, all as taken on 2026-10-11, used as the date read. Cycles' own files for 5.3 (the add-on's Python files and the kernel source files that come with it), as taken on 2026-10-16, used as the date read. I also read the folder's own note, "About this folder.md", which sits inside `Blender Sources/`. I did not use the requirements page.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Two things to declare beside that line, so it does not have to be taken on trust:

1. When this session started, the tool I run in showed me some text without my asking for it: the project's `CLAUDE.md` signpost, an index of the user's saved notes, and the project's git status with recent commit titles. I did not open, search or follow any of it. Nothing below comes from it.
2. To look at one manual picture, I made a PNG copy of it in my own scratch folder, which is outside the project. When I listed that folder I saw the names of files I had not made. By their names they are other agents' briefs, drafts and logs. I did not open any of them. My PNG copy is still in that folder because I was not allowed to delete it.

**How to read this file.**

- Each setting in question 1 gets a code (A1, B1, C1 and so on). Questions 2 to 4 use the same codes.
- Manual, API and release-note quotes give the web address of the page. Quotes from Cycles' own files give the file name and line numbers.
- Release notes are quoted only where they belong to an answer.
- Labels: **documented**, **inferred** (the step is written out), **observed** (read straight out of a Cycles file), **not in the sources**.
- Read dates: everything from the manual, API and release notes was read on 2026-10-11. Everything from Cycles' own files was read on 2026-10-16. The date is also given beside each quote.

---

## 1. What settings, if any, change how much a render demands of the computer it runs on?

**Answer.** Many settings do. The sources describe them in terms of which processor or graphics card does the work, how many processor cores it uses, how much memory it takes, and how fast it runs. The list below groups them that way.

**How I read the question.** The question is ambiguous.

- My reading: "demand" means the hardware a render takes up and how hard it works that hardware. That covers which chip does the work, how many cores, how much memory, and speed settings that exist for that purpose. Groups A to D cover this reading.
- The other reading: anything that changes the total amount of work, such as samples or image size. Almost every quality setting fits that reading. Group E lists only those settings whose descriptions in the sources name a cost in memory or time. It is what my searches found, not a complete list of everything that makes a render take longer.

### Group A — which processor or card does the work

| Code | Setting (as shown) | Where it is kept / its name | What the source says (exact words) | Source |
|---|---|---|---|---|
| A1 | Render engine | Scene: `scene.render.engine`. Launch option `-E` | "Being a rasterization engine, EEVEE only uses the power of the GPU to render." … "CPU power is still needed to handle high scene complexity as the geometry must be prepared by the CPU before rendering each frame." | https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html (read 2026-10-11) |
| A2 | Device (Cycles): CPU / GPU Compute | Scene: `scene.cycles.device`. Values `CPU`, `GPU`. Default `CPU` | "Specifies the compute device used for rendering in Cycles. The choice of device can significantly affect render performance, GPU Compute is often faster ." | https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/index.html (read 2026-10-11). Names and default: Cycles file `properties.py`, lines 32–36 and 430–436 (read 2026-10-16) |
| A3 | Cycles Render Devices (Preferences ‣ System): None / CUDA / OptiX / HIP / oneAPI / Metal, plus a "use" tick for each device found, including the CPU | User preferences, not the scene: Cycles add-on preferences `compute_device_type` and `devices` | "Changes the Computing Device the Cycles render engine uses to render images." "None: When set to None or when the only option is None: the CPU will be used as the computing device for Cycles." | https://docs.blender.org/manual/en/5.3/editors/preferences/system.html (read 2026-10-11). Names: Cycles file `properties.py`, lines 1680–1717 (read 2026-10-16) |
| A4 | `--cycles-device` launch option | Command line only | "Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL." "Append +CPU to a GPU device to render on both CPU and GPU." | https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html (read 2026-10-11) |
| A5 | Denoise ‣ Use GPU (Cycles) | Scene: `scene.cycles.denoising_use_gpu`, default off. Viewport version `preview_denoising_use_gpu`, default on | "Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled." | https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html (read 2026-10-11). Defaults: Cycles file `properties.py`, lines 480–484 and 521–525 (read 2026-10-16) |
| A6 | Compositor ‣ Device; Denoise Nodes ‣ Denoising Device | Scene: `scene.render.compositor_device` (CPU/GPU, default `GPU`); `scene.render.compositor_denoise_device` (AUTO/CPU/GPU, default `AUTO`) | "The device used for compositing." "GPU: Use the GPU to process the denoise node if available, otherwise fallback to CPU." | https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html (read 2026-10-11); https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html (read 2026-10-11) |
| A7 | Per-card switches: HIP RT, Embree on GPU, MetalRT (Off/On/Auto), Kernel Optimization (Off / Intersection only / Full) | User preferences: `use_hiprt`, `use_oneapirt`, `metalrt`, `kernel_optimization_level` | Kernel Optimization: "Kernels can be optimized based on scene content... This can result in additional CPU usage for a brief time (tens of seconds)". HIP RT: "Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above." Embree on GPU: "Hardware acceleration for ray-tracing and denoising is supported." MetalRT: "uses less memory for scenes which use curves extensively, and can give better performance in specific cases". (Further details under question 7.) *(Source corrected by citation check: HIP RT quote is from editors/preferences/system.html rather than gpu_rendering.html)* | Cycles file `properties.py`, lines 1725–1760 (read 2026-10-16); https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html (read 2026-10-11); https://docs.blender.org/manual/en/5.3/editors/preferences/system.html (read 2026-10-11) |

### Group B — how many processor cores

| Code | Setting | Where it is kept / its name | What the source says | Source |
|---|---|---|---|---|
| B1 | Threads Mode (Auto-Detect / Fixed) and Threads | Scene: `scene.render.threads_mode` (default `AUTO`), `scene.render.threads` (1 to 1024) | "Method to determine the maximum number of CPU cores to use while rendering." Auto-Detect: "Automatically chooses the amount of threads to match the number of logical processors on your computer." Fixed: "Manually choose the maximum number threads to use for rendering." Threads: "The maximum number of CPU cores to use simultaneously while rendering." | https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html (read 2026-10-11); API: "Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) (in [1, 1024], default 1)", https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html (read 2026-10-11) |
| B2 | `-t` / `--threads` launch option | Command line only | "Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count." | https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html (read 2026-10-11) |
| B3 | `--qos` launch option | Command line only | "Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only)." "high: Always makes use of performance cores." "eco: Schedules Blender threads exclusively to efficiency cores." | same page (read 2026-10-11) |

### Group C — memory

| Code | Setting | Where it is kept / its name | What the source says | Source |
|---|---|---|---|---|
| C1 | Tile Size (Cycles) | Scene: `scene.cycles.tile_size`, default 2048, range 8 to 8192 | "Render high resolution images in tiles of this size, to reduce memory usage of render passes. Tiles are cached to disk while rendering to save memory." "When running out of memory, lowering to 1024 or 512 pixels helps reduce memory usage, typically at a small performance cost." | https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html (read 2026-10-11). Range and default: Cycles file `properties.py`, lines 1133–1139 (read 2026-10-16) |
| C2 | Texture Cache (with Auto Generate) | Scene: `scene.render.use_texture_cache` (default on), `scene.render.use_auto_generate_texture_cache` (default off). Folder: preference "Texture Cache" path | "Reduce memory usage and startup time on scenes with many image textures, at the cost of a small rendering performance impact and increased disk space usage." API: "Load texture tiles at appropriate resolution on demand to reduce memory usage. This avoids loading all textures into memory, at the cost of extra disk space and some performance (default True)" | manual page as C1; https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html (read 2026-10-11) |
| C3 | Acceleration Structure: Use Spatial Splits, Use Curves BVH, BVH Time Steps, Use Compact BVH (Cycles) | Scene: `scene.cycles.debug_use_spatial_splits` (off), `debug_use_hair_bvh` (on), `debug_bvh_time_steps` (0, range 0–16), `debug_use_compact_bvh` (off) | Spatial splits: "The downsides are longer BVH build times and slightly increased memory usage." Curves BVH: "Disabling this option will reduce memory, at the cost of increasing curve render time." Time steps: "Split BVH primitives by this number of time steps to speed up render time at the expense of memory." Compact: "Use a more compact BVH structure, which can reduce RAM usage but render slower." | manual page as C1; defaults: Cycles file `properties.py`, lines 981–1001 (read 2026-10-16) |
| C4 | Final Render ‣ Persistent Data | Scene: `scene.render.use_persistent_data` | "Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage while performing other tasks in Blender." | manual page as C1 |
| C5 | Render menu ‣ Lock Interface | Scene: `scene.render.use_lock_interface` (default off) | "Lock interface during rendering in favor of giving more memory to the renderer." | https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html (read 2026-10-11) |
| C6 | Distribute Memory Across Devices | User preferences: `peer_memory` | "Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes." "Currently only NVLink on NVIDIA GPUs is supported." | https://docs.blender.org/manual/en/5.3/editors/preferences/system.html (read 2026-10-11) |
| C7 | Simplify: Max Subdivision, Child Particles, Texture Resolution, Texture Size Limit, Camera Cull, Distance Cull | Scene; details under question 6 | Texture Resolution: "Scale down image textures to reduce memory usage." Texture Size Limit: "This can help reduce computer memory resources when rendering large scenes with huge textures." | https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/simplify.html (read 2026-10-11) |
| C8 | Compositor ‣ Precision | Scene: `scene.render.compositor_precision` (AUTO/FULL, default AUTO) | "The precision of compositor intermediate result." "Auto: Use full precision for final renders, half precision otherwise." | manual page as C1 |
| C9 | EEVEE: Shadow Pool; Light Probes Volume Pool | Scene: `scene.eevee.shadow_pool_size` (default '512'), `scene.eevee.gi_irradiance_pool_size` (default '16') | Shadow Pool: "A bigger pool size allows for more shadows in the scene but might not fit into GPU memory and decreases performance." Volume Pool: "A bigger pool size allows for more irradiance grids in the scene but might not fit into GPU memory and decreases performance." | https://docs.blender.org/manual/en/5.3/render/eevee/render_settings/performance.html (read 2026-10-11); https://docs.blender.org/api/5.3/bpy.types.SceneEEVEE.html (read 2026-10-11) |
| C10 | High Quality Normals (EEVEE, Workbench) | Scene: `scene.render.use_high_quality_normals` | "Uses higher precision normals and tangents which can improve visual quality for dense meshes with high frequency textures at the cost of memory." | EEVEE page as C9; https://docs.blender.org/manual/en/5.3/render/workbench/performance.html (read 2026-10-11) |
| C11 | Performance presets: Default / Faster Render / Lower Memory (Cycles) | A preset is not a stored setting. It sets other values | "Faster Render: Uses settings to render faster at the cost of higher memory consumption." "Lower Memory: Uses settings to decrease memory consumption at the cost of slower renders." Observed: a performance preset records exactly `render.threads_mode`, `render.use_persistent_data`, `cycles.debug_use_spatial_splits`, `cycles.debug_use_compact_bvh`, `cycles.debug_use_hair_bvh`, `cycles.debug_bvh_time_steps`, `cycles.tile_size`. | manual page as C1; Cycles file `presets.py`, lines 91–112 (read 2026-10-16) |

### Group D — speed settings for which the sources give no memory cost

| Code | Setting | Name | What the source says | Source |
|---|---|---|---|---|
| D1 | Scrambling Distance (Automatic, Viewport, Multiplier), Cycles | `scene.cycles.scrambling_distance` (default 1.0), `auto_scrambling_distance`, `preview_scrambling_distance` | "A technique that reduces the randomness between pixels in an attempt to improve GPU rendering performance, at the cost of potential artifacts. Not compatible with the Blue-Noise sampling pattern." | https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html (read 2026-10-11) |
| D2 | Anisotropic Filtering (EEVEE, Workbench) | `scene.render.anisotropic_filter` (default 2×) | "Increasing the setting may have a small impact on performance." | EEVEE and Workbench pages as C9, C10 |
| D3 | Denoise Nodes ‣ Final Quality (High / Balanced / Fast) | `scene.render.compositor_denoise_final_quality` | Balanced: "typically processing in half the time as High, while retaining most of the quality." Fast: "Produces an output quickly at a noticeable cost of quality." | manual page as C1 |

### Group E — settings with another main purpose whose descriptions also name a memory or time cost

These are quality or content settings. I list them only because their descriptions say they cost memory or time.

| Setting | Exact words | Source |
|---|---|---|
| Cycles Render Samples | "A higher number results in a cleaner image at the cost of a longer render time." | https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html |
| Cycles Time Limit | "Stops rendering if the time exceeds the limit, even if the desired sample count hasn’t been reached yet." | same |
| Cycles Noise Threshold | "lower values meaning better quality but longer render times." | same |
| Cycles Sample Subset | "A typical use case is to distribute the rendering of a single frame over multiple machines." | same |
| Cycles Light Tree | "This can significantly reduce noise, at the cost of a somewhat longer render time per sample." | same |
| Cycles Offscreen Scale (adaptive subdivision) | "Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory." | https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/subdivision.html |
| Cycles object Motion Blur Steps | "Controls accuracy of deformation motion blur, more steps uses more memory." | https://docs.blender.org/manual/en/5.3/render/cycles/object_settings/object_data.html |
| Cycles world Map Resolution | "conversely will take up more memory and render slightly slower." | https://docs.blender.org/manual/en/5.3/render/cycles/world_settings.html |
| Material Displacement method | "Displacement Only: … this method is also the most memory intensive." "Bump Only: … This is a less memory intensive alternative to actual displacement, but only an approximation." | https://docs.blender.org/manual/en/5.3/render/cycles/material_settings.html |
| Denoising data passes | "These passes increase memory usage and render time." | https://docs.blender.org/manual/en/5.3/render/layers/passes.html |
| Output Color Depth | "Yet a higher bit depth will increase memory usage exponentially." | https://docs.blender.org/manual/en/5.3/render/output/properties/output.html |
| Freestyle | "Highly memory demanding: All mesh objects in a view layer are loaded at once." | https://docs.blender.org/manual/en/5.3/render/freestyle/introduction.html |
| Subdivision Surface modifier levels | "Higher levels of subdivisions results in more vertices, which means higher memory consumption (both system RAM, and video memory for display)." | https://docs.blender.org/manual/en/5.3/modeling/modifiers/generate/subdivision_surface.html |
| EEVEE Volumes Resolution and Steps | "Lower resolution increases video memory usage and quality." "Higher count increases video memory usage and quality." | https://docs.blender.org/manual/en/5.3/render/eevee/render_settings/volumes.html |

All Group E quotes were read on 2026-10-11.

**Label.**

- **Documented:** what each setting does to processor, card, memory or speed. Each item is quoted above.
- **Observed:** the names, defaults and ranges taken from Cycles' own files.
- **Inferred:** that the engine choice (A1) is a lever on demand. The step: EEVEE uses only the graphics card, and Cycles can use either the processor or the card (A2). So choosing the engine also chooses the hardware.
- **Inferred:** the grouping, and the claim that groups A to D are complete. The step: I searched the whole manual, the API pages and the Cycles files for the terms listed below and kept every setting whose description talks about cores, threads, memory, device or performance. A setting described in other words could have been missed.

**Not covered.**

- The sources never measure any of these. They give no numbers such as "this saves X MB". *(Measured by hand on the Creator's Mac, 2026-10-17, for thread count, tile size, the acceleration-structure options, Persistent Data, the texture cache, and the processor against Apple's graphics: entries 8, 20 and 26 below and entry 11 under question 2. A busy machine's figures, recorded as they came.)*
- I found nothing on how the settings combine. For example, nothing says whether Threads affects a GPU render. *(Tested by hand, entry 8 (c) below: on Apple's graphics, `-t 1` left the render time and the processor share where they were in eleven of twelve runs.)*
- The contents of the three Performance presets are not in the sources. Only the list of values a preset records is. *(Read out of a running Blender, entry 6 below.)*
- Viewport-only settings were set aside: Viewport Pixel Size, viewport samples, viewport Simplify values, and the Video Sequencer memory cache. *(Tested by hand: none of seven Viewport settings changed a render started from outside — entry 24, under question 6.)*

**Where I looked.**

- Whole-manual searches for: threads, thread count, cpu cores, logical processors, processor count, memory, lock interface, simplify, large scene, complex scene, heavy scene, scene size, tile/tiles/tiling.
- A whole-API search for "threads", plus the full property lists of `RenderSettings`, `SceneEEVEE`, `PreferencesSystem` and `Scene`.
- A full read of the Cycles files `properties.py`, `ui.py` (the Performance, Simplify, Sampling and Debug panels), `presets.py`, `engine.py`, `__init__.py`, `maketx.py` and the relevant parts of `version_update.py`.
- A search of all 5.3 release notes for: texture, cache, memory, thread, simplify, performance, GPU, device, OptiX, CUDA, Metal, HIP, oneAPI.
- I ruled out the Freestyle line-style pages and the sculpt and geometry-node performance pages. They are about editing or one effect, not the demand a render makes.

**Outside the interface.**

*[Brought up to date by the hand tests of 2026-10-17, recorded under each question below. Every one of them drove Blender from a terminal with no window, except the window photographs of entries 39 to 42; so for every setting tested, what is recorded is what it does when Blender is driven from outside. Lock Interface, set from a start-up script in a render with no window, left the picture unchanged to every channel value (entry 7, under question 2); whether it changes memory there was not measured.]*

- Most of these settings are stored data, not interface controls. The API says "any setting that can be changed via a button can also be changed with Python" (https://docs.blender.org/api/5.3/info_quickstart.html, read 2026-10-11).
- Three of them are command-line options in the first place: B2, B3 and A4. The command line can also choose the engine for A1.
- Lock Interface (C5) is described only in terms of the interface. The sources do not say whether it does anything when Blender has no interface.
- Some panels are hidden in some setups. For example, Acceleration Structure is hidden for OptiX. Hiding a panel does not remove the stored setting (inferred from the fact that the panel only decides what is drawn: Cycles file `ui.py`, lines 949–951).

**Operating system.**

*[Brought up to date by the hand tests of 2026-10-17. Every 5.3 test was run on one machine, the Creator's Mac (macOS 26.6.2 on Apple silicon), so every observed result in this document holds for macOS on Apple silicon and is untested on Linux and Windows — not answered there, and not to be read as answered. Nothing could be run on Linux or Windows in this sitting: no such machine was available. Two operating-system facts were observed on the Mac: the Cycles backend preference offers and accepts only `NONE` and `METAL` and starts at `METAL` (question 7, entry 3), and `--gpu-backend` takes only `metal` (question 7, entries 2 and 15). `--qos` and EEVEE on headless Windows were not tested.]*

- `--qos` is "Windows only".
- The card methods behind A3 and A4 depend on the operating system. CUDA, OptiX, HIP and oneAPI run on Windows and Linux. Metal runs on Apple Silicon Macs (question 7).
- EEVEE: "Headless rendering is not supported on headless Windows systems." (https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html, read 2026-10-11).
- For the other settings, the sources do not name any operating-system difference.

### Tested by hand on the Creator's Mac — 2026-10-17

**Label: observed.** Run by Claude Opus 5.5 in Claude Code from a terminal. **The machine:** the Creator's Mac, a MacBook Pro (MacBookPro18,3) with an Apple M1 Pro chip — eight logical processors and Apple's own 14-core graphics, reached through Metal — 16 GB of memory, macOS 26.6.2 (build 25G83), and no second graphics card. **Blender 5.3.0 Beta**, hash `5d8151335b6b`, built 2026-10-16, at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. Every run pointed Blender's settings folder and its temporary folder at a work folder of the test's own, so the Creator's own Blender settings were neither read nor written. Pictures were compared by their pixels — both images loaded and subtracted channel by channel — never as saved files, because Blender writes the time of day into a saved image. The test files were built by script for these runs; the full procedure, every command, and the scripts word for word are in `Audits/Blender Research/What a render demands/Hand tests — 5.3.md`, under the entry number given with each result. **Nothing here was run on Linux, on Windows, or on any NVIDIA, AMD or Intel graphics**, so where a result is about a machine, it is about macOS on Apple silicon and nothing else.

**Which render engines this build offers (entry 1).** In the terminal, `blender -b -E help` printed, word for word:

```
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
EXIT=0
```

Without `-b` it printed the same three names (without the version line) and exited 0.

**Which settings exist, counted by a stated rule, and their factory values (entry 5).** Every property of five named blocks — `scene.render`, `scene.cycles`, `scene.eevee`, `scene.display` and the Cycles add-on's preferences — read out of a running Blender with no window and `--factory-startup`, each block counted once including its own `rna_type` entry and once without it:

```
COUNT scene.render 103 / 102
COUNT scene.cycles 113 / 112
COUNT scene.eevee 55 / 54
COUNT scene.display 10 / 9
COUNT cycles preferences 9 / 8
TOTAL counting rna_type 290, not counting it 285
```

The names the earlier versions recorded as present in some versions and not others, looked up one by one in 5.3:

```
RECORDED-NAME scene.render.use_texture_cache PRESENT
RECORDED-NAME scene.render.use_auto_generate_texture_cache PRESENT
RECORDED-NAME scene.cycles.texture_resolution PRESENT
RECORDED-NAME scene.cycles.texture_resolution_render PRESENT
RECORDED-NAME scene.cycles.debug_use_texture_cache_eviction PRESENT
RECORDED-NAME scene.cycles.debug_texture_cache_preserve_unused PRESENT
RECORDED-NAME scene.render.anisotropic_filter PRESENT
RECORDED-NAME scene.render.save_output PRESENT
RECORDED-NAME scene.cycles.use_pixel_jitter PRESENT
RECORDED-NAME scene.eevee.direct_light_intensity PRESENT
RECORDED-NAME scene.eevee.indirect_light_intensity PRESENT
RECORDED-NAME scene.eevee.fast_gi_thickness_far ABSENT
RECORDED-NAME scene.cycles.feature_set ABSENT
RECORDED-NAME scene.eevee.gtao_distance ABSENT
RECORDED-NAME scene.eevee.gtao_quality ABSENT
RECORDED-NAME scene.eevee.use_gtao ABSENT
RENDER-BAKE-NAMES ['scene.render.bake']
```

and the factory value of the settings this document names with a default:

```
FACTORY-VALUE scene.render.compositor_device 'GPU'
FACTORY-VALUE scene.render.compositor_denoise_device 'AUTO'
FACTORY-VALUE scene.render.compositor_precision 'AUTO'
FACTORY-VALUE scene.render.use_texture_cache True
FACTORY-VALUE scene.render.use_auto_generate_texture_cache False
FACTORY-VALUE scene.cycles.denoising_use_gpu False
FACTORY-VALUE scene.cycles.preview_denoising_use_gpu True
FACTORY-VALUE scene.cycles.tile_size 2048
FACTORY-VALUE scene.cycles.device 'CPU'
FACTORY-VALUE scene.render.use_lock_interface False
FACTORY-VALUE scene.render.threads_mode 'AUTO'
FACTORY-VALUE scene.eevee.shadow_pool_size '512'
FACTORY-VALUE scene.eevee.gi_irradiance_pool_size '16'
FACTORY-VALUE cycles preferences.use_hiprt True
FACTORY-VALUE cycles preferences.metalrt 'AUTO'
FACTORY-VALUE cycles preferences.kernel_optimization_level 'FULL'
CYCLES-PERFORMANCE-PANEL-CLASSES ['CYCLES_RENDER_PT_performance', 'CYCLES_RENDER_PT_performance_acceleration_structure', 'CYCLES_RENDER_PT_performance_compositor', 'CYCLES_RENDER_PT_performance_compositor_denoise_settings', 'CYCLES_RENDER_PT_performance_final_render', 'CYCLES_RENDER_PT_performance_memory', 'CYCLES_RENDER_PT_performance_texture_cache', 'CYCLES_RENDER_PT_performance_threads', 'CYCLES_RENDER_PT_performance_viewport']
```

Every factory value printed is the default this document gives for that setting. The 5.3 totals are four above the 5.2 record's 286 and 281 under the same rule; the full list of 285 names is kept in the hand-test list, so that the comparison can read off which four.

**What each Cycles Performance preset writes (entry 6).** The preset files Blender ships were found through `bpy.utils.preset_paths('cycles/performance')`, every property of `scene.render` and `scene.cycles` recorded, each preset applied with `bpy.ops.script.execute_preset(filepath=<the file>, menu_idname="CYCLES_PT_performance_presets")` — the operator's arguments printed from the running Blender rather than guessed — and what changed listed. The scene started at factory values, so a preset writing a value the scene already held shows as no change.

```
PRESET-FILES ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/presets/cycles/performance/Default.py', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/presets/cycles/performance/Faster_Render.py', '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/presets/cycles/performance/Lower_Memory.py']
OPERATOR-ARGS ['rna_type', 'filepath', 'menu_idname']
---- applying Default.py APPLIED-OK NOTHING CHANGED
---- applying Faster_Render.py APPLIED-OK 
     CHANGED cycles.debug_bvh_time_steps         0 -> 2
     CHANGED cycles.debug_use_spatial_splits     False -> True
     CHANGED render.use_persistent_data          False -> True
---- applying Lower_Memory.py APPLIED-OK 
     CHANGED cycles.debug_bvh_time_steps         2 -> 0
     CHANGED cycles.debug_use_compact_bvh        False -> True
     CHANGED cycles.debug_use_spatial_splits     True -> False
     CHANGED cycles.tile_size                    2048 -> 512
     CHANGED render.use_persistent_data          True -> False
```

No property outside the seven a preset records changed.

**What thread count, tile size and the acceleration-structure options cost in time and memory (entry 8).** Each render was run under macOS's own timer, `/usr/bin/time -l`, which gives the wall time, the processor time (shown below as a share of one processor) and the peak memory of the Blender process; Cycles' own `Time:` line is given beside it. **The Mac was not idle, and these are a busy machine's figures**: the load average read 6.32 before the first block and 11.03 after it, the Creator's own Blender (at about 2 per cent) was open throughout, and **another agent's Blender started and ran at about 52 per cent of a processor during block (b) and the first three Metal pairs of block (c)**, then ended. One run of each in (b), as the earlier versions did; three of each in (a) and (c).

(a) The 640 by 480 file of entry 7, as saved and with one change each, three times over:

```
base rep1: EXIT 0 | wall 4.24 s | cpu 622% | peak 263749632 bytes | Cycles Time: 00:03.45
tile64 rep1: EXIT 0 | wall 4.38 s | cpu 604% | peak 271138816 bytes | Cycles Time: 00:03.57
tile8192 rep1: EXIT 0 | wall 4.29 s | cpu 613% | peak 264372224 bytes | Cycles Time: 00:03.49
notile rep1: EXIT 0 | wall 4.31 s | cpu 606% | peak 264159232 bytes | Cycles Time: 00:03.50
compact rep1: EXIT 0 | wall 4.30 s | cpu 610% | peak 263618560 bytes | Cycles Time: 00:03.49
spatial rep1: EXIT 0 | wall 4.33 s | cpu 606% | peak 263618560 bytes | Cycles Time: 00:03.52
base rep2: EXIT 0 | wall 4.30 s | cpu 608% | peak 263258112 bytes | Cycles Time: 00:03.49
tile64 rep2: EXIT 0 | wall 4.41 s | cpu 601% | peak 270483456 bytes | Cycles Time: 00:03.60
tile8192 rep2: EXIT 0 | wall 4.31 s | cpu 610% | peak 263487488 bytes | Cycles Time: 00:03.50
notile rep2: EXIT 0 | wall 4.93 s | cpu 536% | peak 264290304 bytes | Cycles Time: 00:04.11
compact rep2: EXIT 0 | wall 5.07 s | cpu 523% | peak 264962048 bytes | Cycles Time: 00:04.24
spatial rep2: EXIT 0 | wall 5.02 s | cpu 529% | peak 264077312 bytes | Cycles Time: 00:04.20
base rep3: EXIT 0 | wall 4.99 s | cpu 530% | peak 265175040 bytes | Cycles Time: 00:04.17
tile64 rep3: EXIT 0 | wall 4.40 s | cpu 602% | peak 270303232 bytes | Cycles Time: 00:03.58
tile8192 rep3: EXIT 0 | wall 4.31 s | cpu 607% | peak 264175616 bytes | Cycles Time: 00:03.49
notile rep3: EXIT 0 | wall 4.45 s | cpu 591% | peak 263962624 bytes | Cycles Time: 00:03.64
compact rep3: EXIT 0 | wall 4.36 s | cpu 599% | peak 263503872 bytes | Cycles Time: 00:03.55
spatial rep3: EXIT 0 | wall 4.29 s | cpu 612% | peak 264323072 bytes | Cycles Time: 00:03.48
```

Tile Size 64 came out about 7 MB higher in peak memory in all three repeats (270.3 to 271.1 MB against 263.3 to 265.2 MB for the rest). Every other difference in time is smaller than the spread between repeats of the same setting.

(b) Whether `-t` changes how much of the processor a Cycles render uses: the 320 by 240, 256-sample file, and a copy of it saved with Threads Mode Fixed and Threads 1, one run each:

```
heavy no -t: EXIT 0 | wall 4.65 s | cpu 568% | peak 248840192 bytes | Cycles Time: 00:03.86
heavy -t 1: EXIT 0 | wall 22.54 s | cpu 99% | peak 248250368 bytes | Cycles Time: 00:21.72
heavy -t 2: EXIT 0 | wall 11.74 s | cpu 191% | peak 249610240 bytes | Cycles Time: 00:10.94
heavy -t 0: EXIT 0 | wall 4.27 s | cpu 616% | peak 248659968 bytes | Cycles Time: 00:03.47
heavy_fixed1 no -t: EXIT 0 | wall 22.45 s | cpu 99% | peak 248627200 bytes | Cycles Time: 00:21.63
heavy_fixed1 -t 8: EXIT 0 | wall 4.89 s | cpu 542% | peak 250134528 bytes | Cycles Time: 00:04.08
```

On this Mac `-t 1` took the processor share to 99 per cent and the render to about five and a half times as long; `-t 2` to 191 per cent and about half of that; `-t 0` matched the unrestricted run. Fixed 1 saved in the file did the same as `-t 1`, and `-t 8` on that file undid it. Peak memory did not move with the thread count.

(c) Whether the thread count reaches a render on Apple's graphics — the 5.3 sources say nothing either way. The 640 by 480 file rendered with `-- --cycles-device METAL`, with no `-t` and with `-t 1`. The first three pairs ran while the other agent's Blender was running:

```
metal no -t rep1: EXIT 0 | wall 2.38 s | cpu 52% | peak 589447168 bytes | Cycles Time: 00:01.55
metal -t 1 rep1: EXIT 0 | wall 2.40 s | cpu 48% | peak 581713920 bytes | Cycles Time: 00:01.54
metal no -t rep2: EXIT 0 | wall 2.62 s | cpu 47% | peak 589398016 bytes | Cycles Time: 00:01.72
metal -t 1 rep2: EXIT 0 | wall 2.48 s | cpu 49% | peak 576962560 bytes | Cycles Time: 00:01.57
metal no -t rep3: EXIT 0 | wall 2.49 s | cpu 49% | peak 587415552 bytes | Cycles Time: 00:01.57
metal -t 1 rep3: EXIT 0 | wall 11.47 s | cpu 10% | peak 651902976 bytes | Cycles Time: 00:10.59
```

The pairs were run three times more once that Blender had ended, with only the Creator's own Blender open:

```
metal no -t rep4: EXIT 0 | wall 2.32 s | cpu 48% | peak 592609280 bytes | Cycles Time: 00:01.52
metal -t 1 rep4: EXIT 0 | wall 2.37 s | cpu 48% | peak 583909376 bytes | Cycles Time: 00:01.55
metal no -t rep5: EXIT 0 | wall 2.44 s | cpu 48% | peak 584712192 bytes | Cycles Time: 00:01.59
metal -t 1 rep5: EXIT 0 | wall 2.43 s | cpu 48% | peak 583335936 bytes | Cycles Time: 00:01.55
metal no -t rep6: EXIT 0 | wall 2.40 s | cpu 49% | peak 588300288 bytes | Cycles Time: 00:01.56
metal -t 1 rep6: EXIT 0 | wall 2.40 s | cpu 48% | peak 588759040 bytes | Cycles Time: 00:01.56
```

Eleven of the twelve Metal renders took between 00:01.52 and 00:01.72 by Cycles' own count, with or without `-t 1`, and used about half of one processor either way. One, a `-t 1` run made while the other Blender was running, took 00:10.59; it is recorded as it came and is not explained. Against the processor renders of (a), the same file on Apple's graphics took less than half the time, used about a twelfth of the processor, and had a peak memory more than twice as high.

**Whether Persistent Data changes the time or the memory of a ten-frame animation (entry 26; C4).** The same 640 by 480 file, frames 1 to 10 of an unchanging scene, rendered with `-a` under EEVEE and under Cycles on the processor, with a start-up script setting Persistent Data off or on and printing it back, one run each under macOS's own timer. The last figure is the last frame's own `Time:` line; every run saved ten frames:

```
BLENDER_EEVEE persistent False: EXIT 0 | wall 7.24 s | cpu 34% | peak 406487040 bytes | Cycles Time: 00:00.62 PERSISTENT False frames saved: 10
BLENDER_EEVEE persistent True: EXIT 0 | wall 7.26 s | cpu 36% | peak 404455424 bytes | Cycles Time: 00:00.62 PERSISTENT True frames saved: 10
CYCLES persistent False: EXIT 0 | wall 36.40 s | cpu 700% | peak 264355840 bytes | Cycles Time: 00:03.48 PERSISTENT False frames saved: 10
CYCLES persistent True: EXIT 0 | wall 36.71 s | cpu 691% | peak 263979008 bytes | Cycles Time: 00:03.51 PERSISTENT True frames saved: 10
```

The load average was 11.57 before these four runs and 17.90 after, with only the Creator's own Blender open beside them, so other work on the Mac was heavy. On this scene, neither engine's wall time or peak memory moved by more than about one per cent between Persistent Data off and on. A single run of each on a busy machine cannot separate a small effect from noise.

**Written in full under other questions, and bearing on this one.** The texture cache's cost (C2): peak memory 290.3 MB with it off, 291.1 MB as saved, 344.6 MB with Auto Generate on, on a scene with three small textures (entry 20, question 2). The processor against Apple's graphics on the same file (A2, A3): Cycles' own times 00:04.06 and 00:04.54 on the processor at 493 to 545 per cent of a processor, 00:01.56 and 00:01.52 on Metal at about 48 per cent, peak memory about 263 MB against about 590 MB (entry 11, question 2). Apple's graphics against the processor on three scene sizes, and MetalRT and Kernel Optimization's times and memory (entries 35 to 37, question 7). The thread count under EEVEE and Workbench (entry 25, question 4).

---

## 2. For each of them, what does it change about what the render produces?

**Answer, by code.**

| Code | What the sources say it changes in the result | Label |
|---|---|---|
| A1 Engine | It changes which renderer makes the image. The sources document separate feature sets for each engine. | inferred: a different renderer produces a different image; the sources do not compare the outputs |
| A2, A3, A4 Device / Render Devices / `--cycles-device` | On the processor, Cycles is "compatible with all Cycles features". Some features do not work on the card: Path Guiding ("not supported on any GPU"), Open Shading Language (CPU or OptiX only), and the Render Time pass ("Supported only on CPU render devices"). A scene that uses these features can come out differently. The sources do not say whether processor and card give the same pixels for a scene that uses none of them. | documented (feature limits); inferred (that the image differs when using features unsupported on GPU); not in the sources (whether the pixels are identical for supported features) |
| A4 "+CPU" (card and processor together) | Nothing said about the image. | not in the sources |
| A5 Denoise on GPU | Nothing said about the image. Only speed and memory are mentioned. | not in the sources |
| A6 Compositor device / denoise device | Nothing said about the image. | not in the sources |
| A7 Per-card switches | HIP RT: "Shadow Caustics are not supported when using HIP without hardware ray-tracing." For Embree on GPU, MetalRT and Kernel Optimization, the sources describe speed, memory, or CPU usage, but state no difference in the rendered image. | documented (HIP RT shadow caustics limit); not in the sources (whether Embree, MetalRT or Kernel Optimization change the image) |
| B1, B2, B3 Threads / `-t` / `--qos` | Nothing said about the image. They are described only in terms of cores. | not in the sources |
| C1 Tile Size | Nothing said about the image, with one exception: with Path Guiding on, the interface warns "May work poorly with render tiling" when the image is bigger than one tile. | observed (the warning); inferred (that a guided render may come out noisier when it is split into tiles). The step: the warning says guiding "may work poorly", and guiding exists to reduce noise |
| C2 Texture Cache | Nothing said about the image. The API wording "Load texture tiles at appropriate resolution on demand" means textures may be read at less than full size. The sources do not say whether that can be seen. | inferred (that textures may be read at lower resolution); not in the sources (whether any difference is visible in the rendered image) |
| C3 Acceleration Structure options | Described only by speed and memory. | not in the sources |
| C4 Persistent Data, C5 Lock Interface, C6 Distribute Memory | Described only by speed and memory. | not in the sources |
| C7 Simplify | It lowers subdivision, shows "only a subset of all child hairs and particles", scales textures down, and culls objects. Culled objects become invisible to some rays. | documented (what is reduced); inferred (that the image shows the reduced geometry, hair and textures — this follows directly) |
| C8 Compositor Precision | Auto gives "full precision for final renders". So for final renders, Auto and Full should give the same result. Half precision is used only outside final renders. | inferred from the quoted wording |
| C9 EEVEE pools | A bigger pool "allows for more shadows" or more irradiance grids. A bigger shadow pool "might fix the Shadow buffer full error". | inferred: a pool that is too small can leave shadows or light-probe data short in the image. The sources do not describe what that looks like |
| C10 High Quality Normals | "can improve visual quality for dense meshes with high frequency textures" | documented |
| C11 Presets | Whatever the values they set (C1, C3, C4, B1) change. | inferred |
| D1 Scrambling Distance | "at the cost of potential artifacts". The Viewport option "may cause flickering". | documented |
| D2 Anisotropic Filtering | "Higher values improve the sharpness of textures viewed at a grazing angle" | documented |
| D3 Denoise node quality | Fast: "a noticeable cost of quality". Balanced: "retaining most of the quality". | documented |
| Group E | Each is first a quality or content setting. The quotes in question 1 say what each one changes: noise, detail, accuracy, or which objects and passes are rendered. | documented (as quoted) |

**Evidence not already quoted in question 1.**

- "Path Guiding is not supported on any GPU." "Open Shading Language is only supported for OptiX, with some limitations listed in the documentation." "Shadow Caustics are not supported when using HIP without hardware ray-tracing." — https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html (read 2026-10-11).
- "CPU: Render using the system’s central processing unit. This mode is compatible with all Cycles features." — https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/index.html (read 2026-10-11).
- Render Time pass: `description="Reports time per pixel in milliseconds. Supported only on CPU render devices"`. In `engine.py` the pass is offered only when `scene.cycles.device == 'CPU'`. — Cycles file `properties.py`, lines 1581–1586; Cycles file `engine.py`, lines 229–232 (read 2026-10-16).
- Path guiding is built only into processor kernels. Lines 163–168 of the Cycles file `source/kernel/features.h` read: `#ifndef __KERNEL_GPU__` / `#  if defined(WITH_PATH_GUIDING)` / `#    define __PATH_GUIDING__` (read 2026-10-16). The Path Guiding panel is shown only `use_cpu(context) and engine.with_path_guiding()` — Cycles file `ui.py`, lines 377–380 (read 2026-10-16).
- Tile warning: `layout.label(text="May work poorly with render tiling", icon='STATUS_INFO')`. It is shown when `render_area > tile_area`. The effective tile size is the set value rounded up to a multiple of 128 and capped at 8192, with values under 128 kept as they are. The comment on the code says the calculation "matches TileManager::compute_render_tile_size and Session::get_effective_tile_size" — Cycles file `ui.py`, lines 404–420 (read 2026-10-16).
- Culling: "Ignore and this way make objects invisible to rays outside of the camera frustum." Distance cull "can be used to avoid culling nearby objects that are outside the camera frustum, but still visible in reflections." — https://docs.blender.org/manual/en/5.3/render/cycles/object_settings/object_data.html (read 2026-10-11).

**Label.** Mixed, as marked in each row.

**Not covered.**

- The sources never say whether a Cycles render is pixel-for-pixel the same with different thread counts, tile sizes, devices, or with persistent data on or off. That is the main gap here (hand tests 1 and 2). *(Tested by hand on the Creator's Mac, 2026-10-17, below: thread count, tile size, Use Tiling, the acceleration-structure options, Persistent Data, High Quality Normals and Lock Interface left the picture identical to every channel value (entry 7); the processor and Apple's graphics did not (entry 11). The NVIDIA half of hand test 2 could not be run: no NVIDIA card in this sitting. Whether denoising on the graphics changes the picture stayed open — entry 18.)*

**Where I looked.**

- Searched the manual's render and command-line pages and the Cycles Python files for: deterministic, reproducible, identical, same result, same image, noise pattern, tile/tiles/tiling.
- The only reproducibility statement found is about path guiding: "Makes path guiding deterministic which means renderings will be reproducible with the same pixel values every time" (Cycles file `properties.py`, lines 722–728, read 2026-10-16). It says nothing about threads, tiles or devices.

**Outside the interface.** *[Brought up to date by the hand tests of 2026-10-17 below: every render in them was started from a terminal with no window, so what they show each setting doing to the picture is what it does when Blender is driven from outside.]* Nothing here depends on the interface, except the tile warning. That warning is an interface label. Whether the condition it warns about also applies to a render with no interface is not stated. The inferred answer is that it does, because the comment ties the calculation to the renderer's own tile sizing.

**Operating system.** *[Brought up to date by the hand tests of 2026-10-17 below: they were run on macOS on Apple silicon only, so what they show holds there and is untested on Linux and Windows — no machine running either was available to this sitting.]* The feature limits differ by card method, so they follow the operating-system split under question 7. Nothing else in this answer is tied to an operating system in the sources.

### Tested by hand on the Creator's Mac — 2026-10-17

**Label: observed.** Run by Claude Opus 5.5 in Claude Code from a terminal, on the Creator's Mac — MacBook Pro (MacBookPro18,3), Apple M1 Pro, Apple's own 14-core graphics through Metal, 16 GB, macOS 26.6.2 (build 25G83), no second graphics card — in Blender 5.3.0 Beta, hash `5d8151335b6b`. The machine and the rules every run kept are written out under question 1's hand-test section; the full procedures are in `Audits/Blender Research/What a render demands/Hand tests — 5.3.md`, by entry number. Every picture comparison below loads both images and subtracts them channel by channel; it never compares the saved files.

**Whether the cost settings change the picture (entry 7).** The file: Cycles, Device CPU, 640 by 480 at 100 per cent, 64 samples, Seed 7, animated seed off, denoising off, adaptive sampling off, four bounces, path guiding off, Open Shading Language off, stamping off, OpenEXR at full float with no compression; a plane, a sphere and a torus lit by one area light. Frame 1 rendered from the terminal with `blender -b --factory-startup <file> -o <folder>/<tag>_ -F OPEN_EXR -x 1 -f 1`, once as saved and once for each change, each change made on the same line — `-t 1`, `-t 8`, a copy of the file saved with Threads Mode Fixed and Threads 1, or a `--python-expr` that set one value and printed it back (`SET 64`, `SET 256`, `SET 8192`, `SET False`, …). Each was compared with the render as saved:

```
base_vs_t1: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_t8: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_fixed1: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_tile64: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_tile256: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_tile8192: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_notile: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_spatial: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_compact: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_nohair: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_bvhsteps: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_persist: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_hqn: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
base_vs_lock: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
```

Two control renders, which are what show the comparison can see a difference at all — Seed 7 changed to 8, and 64 samples to 63:

```
base_vs_seed8: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.3951958417892456 NUM-DIFFERING-CHANNEL-VALUES 719547 of 1228800 MEAN-ABS-DIFF 0.02188808946215204 SIZE (640, 480)
base_vs_s63: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.4484149217605591 NUM-DIFFERING-CHANNEL-VALUES 921600 of 1228800 MEAN-ABS-DIFF 0.02262888852598053 SIZE (640, 480)
```

On this scene, on this Mac, in 5.3: one thread, eight threads, Fixed 1 saved in the file, Tile Size 64, 256 and 8192, Use Tiling off, Spatial Splits on, Compact BVH on, Curves BVH off, BVH Time Steps 4, Persistent Data on, High Quality Normals on and Lock Interface on each left every channel value of the picture unchanged, while both controls changed it. Cycles' own `Time:` lines for these renders were 00:03.49 as saved, 00:21.48 with `-t 1` and 00:21.61 with Fixed 1 in the file, and between 00:03.47 and 00:04.10 for every other render; the Mac was not idle (entry 8 says what else was running), so those times are recorded and not leaned on.

**Whether the processor and Apple's graphics render the same picture, and what "+CPU" renders (entry 11).** Three settings folders of the test's own were made first, each by a script that set the Cycles backend preference and saved it: one with the backend `NONE`; one with `METAL` and only the graphics device ticked (`[('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`); and one with `METAL` and no device ticked. Then, with no `--factory-startup`, so the saved preferences were read: the file above with Device CPU in the `NONE` folder, twice; the same file saved with Device GPU Compute in the `METAL` folder, twice; and the CPU file in the `NONE` folder with `-- --cycles-device METAL+CPU`, once. Compared by pixels:

```
cpu1_vs_cpu2: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
metal1_vs_metal2: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.1920928955078125e-06 NUM-DIFFERING-CHANNEL-VALUES 219425 of 1228800 MEAN-ABS-DIFF 2.5088718302868075e-08 SIZE (640, 480)
cpu1_vs_metal1: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168 NUM-DIFFERING-CHANNEL-VALUES 693524 of 1228800 MEAN-ABS-DIFF 3.928096756832626e-06 SIZE (640, 480)
both_vs_cpu1: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168 NUM-DIFFERING-CHANNEL-VALUES 655175 of 1228800 MEAN-ABS-DIFF 3.335290505977658e-06 SIZE (640, 480)
both_vs_metal1: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.01979970932006836 NUM-DIFFERING-CHANNEL-VALUES 609627 of 1228800 MEAN-ABS-DIFF 8.675749882058881e-07 SIZE (640, 480)
```

The processor repeated its own picture exactly. Apple's graphics repeated theirs to within about a millionth, in about a sixth of the channel values. The processor against Apple's graphics differed in more than half of the channel values, the largest by about 0.028 on a nought-to-one scale — some twenty thousand times the graphics' own repeat. The "+CPU" render matched neither: it differed from the processor's picture and from the graphics' picture, by more than the graphics' own repeat in each case. The same sitting's timer, on a Mac where another agent's Blender was running at about 80 per cent of a processor when these began:

```
cpu1: EXIT 0 | wall 4.86 s | cpu 545% | peak 263208960 bytes | Cycles Time: 00:04.06
metal1: EXIT 0 | wall 2.38 s | cpu 48% | peak 591380480 bytes | Cycles Time: 00:01.56
cpu2: EXIT 0 | wall 5.39 s | cpu 493% | peak 263782400 bytes | Cycles Time: 00:04.54
metal2: EXIT 0 | wall 2.34 s | cpu 49% | peak 586858496 bytes | Cycles Time: 00:01.52
both: EXIT 0 | wall 2.77 s | cpu 359% | peak 536838144 bytes | Cycles Time: 00:01.95
```

What Blender printed for the "+CPU" run names no device; every line holding the words device, CPU or Metal was an ordinary progress line such as `Updating Scene BVH | Copying BVH to device`.

**What a Cycles Time Limit produces (entry 16; Group E).** The same file rendered once as saved and once with `--python-expr "import bpy; c=bpy.context.scene.cycles; c.time_limit=0.3; print('TIME-LIMIT', c.time_limit)"`:

```
nolimit EXIT=0 Time: 00:03.46
limit EXIT=0 TIME-LIMIT 0.30000001192092896 Time: 00:00.34
-rw-r--r--@ 1 artist     wheel  4923809 Oct 17 18:21 limit_0001.exr
-rw-r--r--@ 1 artist     wheel  4923809 Oct 17 18:21 nolimit_0001.exr
nolimit_vs_limit: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.18979150056839 NUM-DIFFERING-CHANNEL-VALUES 921600 of 1228800 MEAN-ABS-DIFF 0.058494475829328395 SIZE (640, 480)
```

The limited render stopped at 00:00.34 by Cycles' own count, against 00:03.46, and wrote a file of the same size holding a different picture — every colour channel value differs.

**Whether denoising on the graphics rather than the processor changes the picture (entry 18; A5).** The same file with denoising switched on from a start-up script — `c.use_denoising=True; c.denoiser='OPENIMAGEDENOISE'; c.denoising_use_gpu=<False|True>` — two renders of each, with `--log "cycles" --log-level info` so Cycles would say where it denoised. First rendered on the processor (the file's own Device), then again with `-- --cycles-device METAL`. Every one of the eight runs printed, among its log lines:

```
cycles           | Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
                            | Denoising on: Apple M1 Pro (CPU) (8 threads) [CPU]
```

— the four with Use GPU off and the four with it on alike, `DENOISE True OPENIMAGEDENOISE True` printed by the script in the latter. The pictures:

```
render on the processor:
cpuA_vs_cpuB: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
gpuA_vs_gpuB: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
cpuA_vs_gpuA: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
render on Apple's graphics:
mcpuA_vs_mcpuB: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.4557113647460938e-05 NUM-DIFFERING-CHANNEL-VALUES 728980 of 1228800 MEAN-ABS-DIFF 7.237465888465522e-07 SIZE (640, 480)
mgpuA_vs_mgpuB: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.3365020751953125e-05 NUM-DIFFERING-CHANNEL-VALUES 728024 of 1228800 MEAN-ABS-DIFF 7.246472614497179e-07 SIZE (640, 480)
mcpuA_vs_mgpuA: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.574920654296875e-05 NUM-DIFFERING-CHANNEL-VALUES 728137 of 1228800 MEAN-ABS-DIFF 7.239028824794029e-07 SIZE (640, 480)
```

**This does not settle A5.** In 5.3 on this Mac, a final render started from the terminal reported denoising on the processor whether Use GPU was on or off, and whether the render itself ran on the processor or on Apple's graphics; with the denoiser never reported on the graphics, the run has no graphics-denoised picture to compare. What it does record: switching Use GPU on changed nothing in the picture beyond what two identical runs differ by.

**Whether the compositor's device, the Denoise node's device, and compositor Precision change the picture (entry 19; A6, C8).** The same file given a compositor by a start-up script — a new compositing node group, Render Layers into a Blur node of size 20 (or into a Denoise node, with Samples lowered to 8), into the group's output — then rendered on the processor. The script printed what it set, for example `COMP blur compositor_device CPU denoise_device AUTO precision AUTO nodes ['CompositorNodeRLayers', 'NodeGroupOutput', 'CompositorNodeBlur'] samples 64` and `COMP denoise compositor_device GPU denoise_device CPU precision AUTO nodes ['CompositorNodeRLayers', 'NodeGroupOutput', 'CompositorNodeDenoise'] samples 8`; Blender also printed `DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0` for the line that attached the group. Two renders of each, and the comparisons:

```
control_unblurred_vs_blurcpuA: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.1947470605373383 NUM-DIFFERING-CHANNEL-VALUES 1228800 of 1228800 MEAN-ABS-DIFF 0.0379734456679762 SIZE (640, 480)
blurcpuA_vs_blurcpuB: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
blurgpuA_vs_blurgpuB: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
blurcpuA_vs_blurgpuA: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.9073486328125e-06 NUM-DIFFERING-CHANNEL-VALUES 1070316 of 1228800 MEAN-ABS-DIFF 1.2737891665892676e-07 SIZE (640, 480)
control_raw8_vs_dencpuA: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4486746788024902 NUM-DIFFERING-CHANNEL-VALUES 921599 of 1228800 MEAN-ABS-DIFF 0.05004338425812572 SIZE (640, 480)
control_raw8_vs_dengpuA: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4505568742752075 NUM-DIFFERING-CHANNEL-VALUES 921600 of 1228800 MEAN-ABS-DIFF 0.05005289855807708 SIZE (640, 480)
dencpuA_vs_dencpuB: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
dengpuA_vs_dengpuB: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
dencpuA_vs_dengpuA: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.018398284912109375 NUM-DIFFERING-CHANNEL-VALUES 921593 of 1228800 MEAN-ABS-DIFF 0.0008870878585366882 SIZE (640, 480)
blurcpuAUTO_vs_blurcpuFULL: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
```

The two `control` lines show the compositor ran: the blurred picture differs from the unblurred render of entry 7, and both denoised pictures differ from a plain 8-sample render with no compositor. Each device repeated its own compositor result exactly. **The Blur node on the processor and on Apple's graphics differed**, by at most about two millionths, in most channel values. **The Denoise node on the processor and on Apple's graphics differed by up to about 0.018.** Precision Auto and Full gave the same picture in a final render.

**The texture cache: whether it exists, what it writes beside the images, what it costs, and whether it changes the picture (entry 20; C2).** A copy of the same file with three 1024 by 1024 PNG images written beside it and wired into the three objects' materials. As saved, the file holds this version's factory values — the script printed `CACHE-AS-SAVED True False` (Texture Cache on, Auto Generate off). Four renders, in this order, each under macOS's own timer, with the folder listed after each:

```
-- folder listing (before):
-rw-r--r--@ 131056 tex.blend
-rw-r--r--@ 213596 tex0.png
-rw-r--r--@ 14357 tex1.png
-rw-r--r--@ 35034 tex2.png
392K	data/tex
off: EXIT 0 | wall 4.28 s | cpu 608% | peak 290275328 bytes | Cycles Time: 00:03.50
HAS-use_texture_cache True HAS-use_auto_generate_texture_cache True
CACHE-SET False False
asis: EXIT 0 | wall 4.30 s | cpu 606% | peak 291078144 bytes | Cycles Time: 00:03.50
CACHE-AS-SAVED True False
auto: EXIT 0 | wall 4.60 s | cpu 607% | peak 344604672 bytes | Cycles Time: 00:03.81
CACHE-SET True True
-- folder listing (after auto):
drwxr-xr-x@ 160 blender_tx
-rw-r--r--@ 131056 tex.blend
-rw-r--r--@ 213596 tex0.png
-rw-r--r--@ 14357 tex1.png
-rw-r--r--@ 35034 tex2.png
1.0M	data/tex
-- inside blender_tx:
-rw-r--r--@ 535019 tex0.png.0-926BDA8569F1F2504902F06ABA51E46D.tx
-rw-r--r--@ 2566 tex1.png.0-926BDA8569F1F2504902F06ABA51E46D.tx
-rw-r--r--@ 101259 tex2.png.0-926BDA8569F1F2504902F06ABA51E46D.tx
asis2: EXIT 0 | wall 4.48 s | cpu 615% | peak 273203200 bytes | Cycles Time: 00:03.68
CACHE-AS-SAVED True False
```

The folder was unchanged after `off` and after `asis`. The image lines Blender printed: `auto` printed `Generating tx cache | tex0.png` and `Generating tx cache | tex2.png`, then `Loading tex0.png.0-926BDA8569F1F2504902F06ABA51E46D.tx` and `Loading tex1.png.0-926BDA8569F1F2504902F06ABA51E46D.tx`; `asis2`, run after the `.tx` files existed, printed the same two `Loading ….tx` lines; `off` and `asis` printed `Loading tex0.png` and `Loading tex1.png`. The pictures:

```
off_vs_asis: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
off_vs_auto: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.12330907583236694 NUM-DIFFERING-CHANNEL-VALUES 685007 of 1228800 MEAN-ABS-DIFF 0.0005446679808669212 SIZE (640, 480)
asis_vs_auto: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.12330907583236694 NUM-DIFFERING-CHANNEL-VALUES 685007 of 1228800 MEAN-ABS-DIFF 0.0005446679808669212 SIZE (640, 480)
auto_vs_asis2: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.0011849403381347656 NUM-DIFFERING-CHANNEL-VALUES 476356 of 1228800 MEAN-ABS-DIFF 2.5308716897024887e-07 SIZE (640, 480)
off_vs_asis2: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.12330907583236694 NUM-DIFFERING-CHANNEL-VALUES 687500 of 1228800 MEAN-ABS-DIFF 0.0005446796622744365 SIZE (640, 480)
```

On this Mac, in 5.3: with the file as saved and no `.tx` files beside the images, the picture was identical to the cache switched off. With Auto Generate on, Blender wrote a `blender_tx` folder of `.tx` files beside the images, the folder grew from 392 kB to 1.0 MB, the picture changed (by up to about 0.12), and the peak memory of that run was higher (344.6 MB against 290.3 MB). Once the `.tx` files existed, the file as saved — Auto Generate off — loaded them and gave a picture close to, but not identical with, the Auto Generate run, with a peak memory of 273.2 MB. On a scene whose textures are this small, the cache did not lower peak memory in any run against the cache switched off. Whether it does on a scene whose textures press on memory is not tested here: such a scene would press the Mac's 16 GB, which this sitting was not allowed to do. One more reading, from question 6's entry 24: with the `.tx` files in use, two unchanged renders of this file on the processor were not identical (`tex_asis_run1_vs_run2: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.00044193118810653687`), where with the cache off every processor render in this document repeated itself exactly.

---

## 3. Which of them can be settled at the moment a render is started from outside Blender, and which only by changing the file?

**Answer.** There are three kinds.

1. **Settled at launch by a dedicated option, with the file unchanged.**
   - A1, the engine: `-E`.
   - A2, A3 and A4, the Cycles device and card method: `--cycles-device`. It must come after `--`, and "+CPU" can be added. (This overrides the compute device type, but does not choose which individual GPU cards are ticked; individual card selection is a user preference under Kind 3.)
   - B2, the thread count: `-t` / `--threads`.
   - B3, the core scheduling on Windows: `--qos`.
2. **Kept in the scene inside the file, with no dedicated launch option.** These are A5, A6, B1, C1–C5, C7–C10, D1–D3 and Group E. They can still be changed at launch without changing the file on disk: a Python script given at launch with `-P` or `--python-expr`, placed after the file name, can set them in memory before the render starts. Without such a script, the only way to change them is to change the file.
3. **Not in the file at all, because they are user preferences.** These are A3 (the card method and which devices are ticked), A7 and C6. Changing the file cannot change them. At launch, `--cycles-device` overrides the card method. The other preference values could be set by a launch script that writes to the preferences in memory.

**Evidence.**

- `-E`: "Specify the render engine." `-t`: "Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count." `--cycles-device`: "Cycles add-on options must be specified following a double dash." — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html (read 2026-10-11).
- The Cycles file's own help text for the option: `help="Set the device to use for Cycles, overriding user preferences and the scene setting."`. It is read only from arguments after `--`: `if "--" not in argv: return`. — Cycles file `engine.py`, lines 16–20 and 27–29 (read 2026-10-16).
- `-t 2` "Use only two threads." `-a` "Render the whole animation using all the settings saved in the blend-file." — https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html (read 2026-10-11).
- `--python-expr`: "Run the given expression as a Python script." `-P`: "Run the given Python script file." "Arguments are executed in the order they are given." Its example: "…will not render to /tmp because loading the blend-file overwrites the render output that was set." — arguments page as above (read 2026-10-11).
- "any setting that can be changed via a button can also be changed with Python." — https://docs.blender.org/api/5.3/info_quickstart.html (read 2026-10-11).
- The thread count has no Python equivalent: "there are some command line arguments that don’t have equivalents in Blender’s Python API (--threads and --log for example)." — https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html (read 2026-10-11). The scene's own Threads Mode and Threads (B1) are ordinary settings and do have Python names. What has no Python equivalent is the launch override.
- Preferences live in a separate file: "./config/userpref.blend — User preferences." — https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html (read 2026-10-11). The Cycles device settings are add-on preferences: `class CyclesPreferences(bpy.types.AddonPreferences)`, and the comment `# Properties saved in preferences` sits on the per-device settings — Cycles file `properties.py`, lines 1673 and 1680 (read 2026-10-16).

**Label.**

- Kind 1: **documented**. The override of the scene and preferences by `--cycles-device` is **documented** in `engine.py`'s help string and **inferred** from its argument parser logic; actual override behaviour during a render has not been tested in a running Blender. *(**Observed** 2026-10-17 on the Creator's Mac, entry 12 below: by which picture came out, `--cycles-device METAL` and `--cycles-device CPU` each beat both the file's Device and the saved backend preference, in both directions, and wrote nothing back to either. `-E`, `-t` and the rest are observed too: entries 9 and 10.)*
- Kind 2: **inferred**. The step: the API says any setting can be changed with Python. The command line runs a given Python expression, in order. Loading the file replaces what came before it. So a script placed after the file name can change the loaded scene in memory before a render option that follows it. Nothing writes the file back unless the script saves it. *(**Observed** 2026-10-17, entries 17, 21 and 9 below and many entries under question 2: a start-up script's setting reached the render; a file's own Resolution Percentage was honoured and its Render Single Layer was not (entry 22); and where `-t` is on the line it outranks a script's thread count.)*
- Kind 3: **inferred** that preferences are outside the scene file. The step: they are add-on preferences, and the manual lists a separate user-preferences file. That a launch script can set them is also inferred. *(**Observed** 2026-10-17, entries 12 and 14 below and question 7's entries 4, 36 and 37: the backend preference came from the settings folder, not the file, and was set aside by `--factory-startup`; and all six Cycles preference values could be set by a start-up script, MetalRT and Kernel Optimization then changing what the render did.)*

**Not covered.**

- No source says how `-t` interacts with the scene's Threads Mode. For example, it is not said whether `-t 4` overrides a file set to Fixed 16 (hand test 3). *(Tested by hand, entry 9 below and question 1's entry 8: `-t` with a number above 0 sets the scene's own Threads value for the run, in a file saved Fixed, and the renderer follows it; `-t 0` leaves the file's value.)*
- No source says whether a launch script must run before Cycles starts for a preference change to take effect. *(Tested by hand only in one form: a `--python-expr` placed after the file name and before `-f 1` set MetalRT and Kernel Optimization and the render followed — question 7, entries 36 and 37. A script placed elsewhere was not tried.)*
- Changing the display Backend "requires restarting Blender". The sources say nothing similar for the Cycles card preferences.

**Where I looked.**

- The whole command-line arguments page and the command-line rendering page.
- The Cycles `engine.py` argument parser.
- An API search for "--threads" and "cycles-device".
- The preferences introduction and directory-layout pages.
- Searched the manual for "userpref.blend".

**Outside the interface.** *[Brought up to date by the hand tests of 2026-10-17 below, every one run from a terminal with no window.]* This whole question is about running without the interface. One more point: with Blender loaded as a Python module, "Functionality controlled by command line arguments (shown by calling blender --help) isn’t accessible." (same API page, read 2026-10-11). So `-t` and `--cycles-device` are not available that way.

**Operating system.**

*[Brought up to date by the hand tests of 2026-10-17 below. Observed on macOS on Apple silicon only; Linux and Windows untested, no machine running either being available to this sitting. On this Mac the factory value of the Cycles backend preference is `METAL` (entry 14), and `--cycles-device` rendered with `CPU`, `METAL` and `METAL+CPU`, and stopped with `ERROR Found no Cycles device of the specified type` and no image for `CUDA`, `OPTIX`, `HIP` and `ONEAPI` (question 7, entry 13). `--qos` was not tested: it is Windows only.]*

- `--qos` is Windows only.
- Which `--cycles-device` values work depends on the operating system and the card (question 7).
- The other options carry no operating-system note.

### Tested by hand on the Creator's Mac — 2026-10-17

**Label: observed.** Run by Claude Opus 5.5 in Claude Code from a terminal, on the Creator's Mac — MacBook Pro (MacBookPro18,3), Apple M1 Pro, Apple's own 14-core graphics through Metal, 16 GB, macOS 26.6.2 (build 25G83), no second graphics card — in Blender 5.3.0 Beta, hash `5d8151335b6b`. The machine and the rules every run kept are written out under question 1's hand-test section; the full procedures are in `Audits/Blender Research/What a render demands/Hand tests — 5.3.md`, by entry number. Every run here was started from the terminal with no window, so **every result in this section is a result for Blender driven from outside its own interface.**

**What `-t` does to the scene's stored thread setting, and whether its place on the line matters (entry 9).** A file saved with Threads Mode Fixed and Threads 1, read back with `--python-expr "import bpy; r=bpy.context.scene.render; print('THREADS', r.threads_mode, r.threads)"`:

```
(a) no -t                                     THREADS FIXED 1
(b) -t 8 after the file name                  THREADS FIXED 8
(c) -t 8 before the file name                 THREADS FIXED 8
(d) -t 0 after the file name                  THREADS FIXED 1
(e) --factory-startup, no file (Auto-Detect)  THREADS AUTO 8
```

`-t 8` changed the scene's own stored Threads value to 8 on either side of the file name, the Mode staying Fixed; `-t 0` left it at 1; and with no file, under Auto-Detect, the value read back as 8, which is this Mac's count of logical processors. Entry 8 shows the renderer following the same numbers: Fixed 1 in the file renders at 99 per cent of one processor, and `-t 8` on that file at 542 per cent.

**Which wins when `-t` and a start-up script both set the thread count (entry 9, continued).** The 320 by 240, 256-sample file, rendered with a script that sets Fixed 5 and prints back what it just set (`SET-BY-SCRIPT`), and a second that prints the value before the render (`FINAL`):

```
(f) -t 3, then the two scripts:  SET-BY-SCRIPT FIXED 3 / FINAL FIXED 3 — EXIT 0 | wall 8.34 s | cpu 276% | Cycles Time: 00:07.54
(g) the two scripts, then -t 3:  SET-BY-SCRIPT FIXED 3 / FINAL FIXED 3 — EXIT 0 | wall 8.31 s | cpu 276% | Cycles Time: 00:07.49
(h) the two scripts, no -t:      SET-BY-SCRIPT FIXED 5 / FINAL FIXED 5 — EXIT 0 | wall 5.50 s | cpu 432% | Cycles Time: 00:04.69
```

The script assigned 5 and printed back 3 whenever `-t 3` was on the line, before or after it; with no `-t` it got 5. The processor share followed the printed number. This answers the open point above about how `-t` meets a file's Threads Mode: on this Mac, in 5.3, `-t` with a number above 0 sets the scene's Threads value for that run and holds it against a script, and `-t 0` leaves the file's own setting in force.

**Whether `-E` overrides the file's engine, whether its place matters, and whether the file on disk changes (entry 10).** A file saved with EEVEE as its engine, read back with `--python-expr "import bpy; print('ENGINE', bpy.context.scene.render.engine)"`:

```
(a) no -E                                               ENGINE BLENDER_EEVEE
(b) -E CYCLES after the file name                       ENGINE CYCLES
(c) -E CYCLES before the file name                      ENGINE BLENDER_EEVEE
(d) -E CYCLES, then save a copy, then read the copy     BEFORE-SAVE CYCLES / COPY-ENGINE CYCLES
(e) the original file read again                        ENGINE BLENDER_EEVEE
```

and (f), `-E BOGUS_ENGINE` after the file name, printed word for word:

```
00:00.328  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/What a render demands/data/eevee.blend"

Error: engine not found 'BOGUS_ENGINE'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
EXIT=1
```

The `--python-expr` after it never ran: no `ENGINE` line was printed. So `-E` took effect only after the file name; it did not change the file on disk; a session that saved afterwards wrote the overridden engine into the new file; and an unknown name was refused by name and ended the run with exit status 1.

**Which picture comes out for each mix of the file's Device, the saved backend preference and `--cycles-device`, and what is written down afterwards (entry 12).** This is the test the label under Kind 1 above says had not been run. Two reference pictures from question 2's entry 11 — the file rendered on the processor, and on Apple's graphics, which differ by about 0.028 while the processor repeats itself exactly and the graphics to about a millionth — settle which device did each render below. Three settings folders of the test's own: backend `NONE`; backend `METAL` with only the graphics ticked; backend `METAL` with no device ticked. No `--factory-startup`, so each run read its folder's saved preferences. Each render was `blender -b <file> -o <folder>/<tag>_ -F OPEN_EXR -x 1 -f 1 --python-expr "<print the scene's Device and the backend preference>" -- [--cycles-device X] --cycles-print-stats`, so the `DEVICE ... PREF ...` line was printed **after** the render:

```
(a) folder=none file=base_cpu.blend args=[] EXIT=0 DEVICE CPU PREF NONE
(b) folder=none file=base_cpu.blend args=[--cycles-device METAL] EXIT=0 DEVICE CPU PREF NONE
(c) folder=metal file=base_gpu.blend args=[] EXIT=0 DEVICE GPU PREF METAL
(d) folder=metal file=base_gpu.blend args=[--cycles-device CPU] EXIT=0 DEVICE GPU PREF METAL
(e) folder=none file=base_gpu.blend args=[] EXIT=0 DEVICE GPU PREF NONE
(f) folder=none file=base_gpu.blend args=[--cycles-device METAL] EXIT=0 DEVICE GPU PREF NONE
(g) folder=metal_unticked file=base_gpu.blend args=[] EXIT=0 DEVICE GPU PREF METAL
(h) folder=metal file=base_cpu.blend args=[] EXIT=0 DEVICE CPU PREF METAL
```

and read before any render, with `-- --cycles-device METAL` on the line: `DEVICE CPU PREF NONE`. Each picture against the two references:

```
a_vs_CPUREF: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
a_vs_METALREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168 NUM-DIFFERING-CHANNEL-VALUES 693524 of 1228800 MEAN-ABS-DIFF 3.928096756832626e-06 SIZE (640, 480)
b_vs_CPUREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.027767658233642578 NUM-DIFFERING-CHANNEL-VALUES 693826 of 1228800 MEAN-ABS-DIFF 3.928380330459428e-06 SIZE (640, 480)
b_vs_METALREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.1920928955078125e-06 NUM-DIFFERING-CHANNEL-VALUES 215039 of 1228800 MEAN-ABS-DIFF 2.4421487220630904e-08 SIZE (640, 480)
c_vs_CPUREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776801586151123 NUM-DIFFERING-CHANNEL-VALUES 693707 of 1228800 MEAN-ABS-DIFF 3.928333864375115e-06 SIZE (640, 480)
c_vs_METALREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.430511474609375e-06 NUM-DIFFERING-CHANNEL-VALUES 215921 of 1228800 MEAN-ABS-DIFF 2.457637265251833e-08 SIZE (640, 480)
d_vs_CPUREF: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
d_vs_METALREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168 NUM-DIFFERING-CHANNEL-VALUES 693524 of 1228800 MEAN-ABS-DIFF 3.928096756832626e-06 SIZE (640, 480)
e_vs_CPUREF: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
e_vs_METALREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168 NUM-DIFFERING-CHANNEL-VALUES 693524 of 1228800 MEAN-ABS-DIFF 3.928096756832626e-06 SIZE (640, 480)
f_vs_CPUREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168 NUM-DIFFERING-CHANNEL-VALUES 693732 of 1228800 MEAN-ABS-DIFF 3.928271544282325e-06 SIZE (640, 480)
f_vs_METALREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.430511474609375e-06 NUM-DIFFERING-CHANNEL-VALUES 213429 of 1228800 MEAN-ABS-DIFF 2.4429977353671954e-08 SIZE (640, 480)
g_vs_CPUREF: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
g_vs_METALREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168 NUM-DIFFERING-CHANNEL-VALUES 693524 of 1228800 MEAN-ABS-DIFF 3.928096756832626e-06 SIZE (640, 480)
h_vs_CPUREF: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
h_vs_METALREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168 NUM-DIFFERING-CHANNEL-VALUES 693524 of 1228800 MEAN-ABS-DIFF 3.928096756832626e-06 SIZE (640, 480)
```

So, by which picture came out: (a) processor; (b) Apple's graphics — the argument beat a file saying CPU and a folder naming no backend; (c) Apple's graphics; (d) the processor, to the last number — the argument beat a file saying GPU Compute and a folder naming Metal; (e) **the processor, to the last number** — a file saying GPU Compute, in a folder naming no backend, did not reach the graphics; (f) Apple's graphics; (g) **the processor, to the last number** — Metal named in the folder but no device ticked; (h) the processor — a file saying CPU, whatever the folder names. The scene's stored Device and the saved backend preference read back unchanged after every run, including the four where the work went somewhere else than they say.

What Blender printed, searched for the words Metal, GPU, device, CPU, fall, warn and error: in every run only ordinary progress lines (`Updating Scene BVH | Copying BVH to device`, `Updating Device | Writing constant memory`, and so on) and the statistics line `device_update`. **No line named the device that rendered.** One line of `--cycles-print-stats`' output appeared in exactly the three runs the pictures put on Apple's graphics — (b), (c) and (f) — and in none of the five on the processor:

```
Profiling information not available (only works with CPU rendering)
```

**With Cycles' own log switched on, a line does name the device.** Those runs used Blender's ordinary output. Renders made later in this sitting with `--log "cycles" --log-level info` on the line (question 2's entry 18 and question 7's entry 36) each printed a line naming where the path tracing ran, for example:

```
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
```

— the first from a render on the processor, the second from renders with `-- --cycles-device METAL`. So the eight crossings (a) to (h) were rendered once more with `--log "cycles" --log-level info` added (and without `--cycles-print-stats`), and the line set beside which picture came out, each new picture compared with the processor reference:

```
(a) folder=none file=base_cpu.blend args=[] EXIT=0 Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
(b) folder=none file=base_cpu.blend args=[--cycles-device METAL] EXIT=0 Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
(c) folder=metal file=base_gpu.blend args=[] EXIT=0 Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
(d) folder=metal file=base_gpu.blend args=[--cycles-device CPU] EXIT=0 Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
(e) folder=none file=base_gpu.blend args=[] EXIT=0 Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
(f) folder=none file=base_gpu.blend args=[--cycles-device METAL] EXIT=0 Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
(g) folder=metal_unticked file=base_gpu.blend args=[] EXIT=0 Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
(h) folder=metal file=base_cpu.blend args=[] EXIT=0 Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
alog_vs_CPUREF: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0
blog_vs_CPUREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168
clog_vs_CPUREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776777744293213
dlog_vs_CPUREF: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0
elog_vs_CPUREF: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0
flog_vs_CPUREF: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168
glog_vs_CPUREF: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0
hlog_vs_CPUREF: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0
```

(the comparison lines cut after their largest difference). In all eight, the device the log line named was the device the picture says rendered, including (e) and (g), where the file's Device says GPU Compute and the render ran on the processor.

**Whether a render started from outside reads the saved preferences, and what factory settings hold (entry 14).** Three settings folders: a brand-new empty one, the one saved with backend `NONE`, and the one saved with `METAL`. Each read with `--python-expr "import bpy; print('READ-PREF', bpy.context.preferences.addons['cycles'].preferences.compute_device_type)"` in a plain background run and again with `--factory-startup`:

```
== folder fresh: config holds []
  plain background run: READ-PREF METAL
  with --factory-startup: READ-PREF METAL
== folder none: config holds [userpref.blend ]
  plain background run: READ-PREF NONE
  with --factory-startup: READ-PREF METAL
== folder metal: config holds [userpref.blend ]
  plain background run: READ-PREF METAL
  with --factory-startup: READ-PREF METAL
after: fresh config holds []
```

A plain background run read the saved `NONE`; `--factory-startup` set it aside and gave `METAL`, which is also what a folder with no saved preferences gives. On this Mac, the factory value of the Cycles backend preference is `METAL`. Neither kind of run wrote a preferences file into the empty folder.

**Whether a start-up script can set a setting that has no launch argument of its own (entry 17; Kind 2).** The 64-sample file rendered with `--python-expr "import bpy; bpy.context.scene.cycles.samples=7; print('SAMPLES-SET', bpy.context.scene.cycles.samples)"` placed after the file name and before `-f 1`:

```
s7 EXIT=0 SAMPLES-SET 7 Time: 00:00.39
base64_vs_s7: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.3136330246925354 NUM-DIFFERING-CHANNEL-VALUES 921600 of 1228800 MEAN-ABS-DIFF 0.059723226384175515 SIZE (640, 480)
```

The value set by the script reached the render: 00:00.39 against 00:03.46 to 00:04.17 for the file as saved, and a different picture. The Time Limit of entry 16 (question 2) was set the same way and also reached the render.

**Whether a scene's Resolution Percentage reaches a render started from the command line (entry 21).** A file saved at 160 by 120 with Resolution Percentage 37, rendered with `blender -b --factory-startup pct.blend -o <folder>/r_ -F PNG -x 1 -f 1`, and the written image then loaded in Blender and its size printed:

```
00:00.420  render           | Saved: 'renders/e21/r_0001.png'
IMAGE r_0001.png SIZE (59, 44)
```

The percentage stored in the file was applied.

**Whether Render Single Layer is honoured from the command line (entry 22).** A file with two view layers, `ViewLayer` and `Second`, saved once with Render Single Layer off and once with it on, each rendered with `-f 1`. The lines naming a view layer, word for word:

```
== off
00:00.326  render           | Start rendering: Scene, ViewLayer
00:00.386  render           | Start rendering: Scene, Second
== on
00:00.315  render           | Start rendering: Scene, ViewLayer
00:00.374  render           | Start rendering: Scene, Second
```

Both runs exited 0 and both rendered both view layers: the setting stored in the file made no difference to a render started from the command line.

---

## 4. Do any of them belong to a particular render engine rather than to every engine?

**Answer.** Yes, many do.

| Code | Engine | Why |
|---|---|---|
| A1 | — | This is the engine choice itself. Documented (launch option and property descriptions). |
| A2, A3, A4, A5, A7, C1, C3, C6, D1; Simplify Texture Resolution, Texture Size Limit and Culling (part of C7); Group E items marked Cycles | **Cycles only** | They are stored in Cycles' own settings, in Cycles' preferences, or are Cycles command-line options. Cycles' panels are drawn only when the engine is Cycles. Observed (settings storage and panel conditions in Cycles files); documented (Cycles-only command-line options and manual pages). |
| C9, EEVEE Volumes | **EEVEE only** | They are EEVEE scene settings, on EEVEE's pages. Documented (EEVEE manual pages); observed (EEVEE property storage). |
| C10, D2 | **EEVEE and Workbench** | They are documented on the EEVEE and Workbench Performance pages, not on Cycles'. They are stored in the general render settings. Documented (EEVEE and Workbench manual pages); observed (general render settings storage). |
| A6, C8, D3 | **Not engine-specific** | These are compositor settings. They appear on both the Cycles and EEVEE Performance pages. Documented (manual placement on both Cycles and EEVEE pages); observed (general render settings storage). |
| B1 Threads Mode / Threads | Stored in the general render settings. The interface shows them only in Cycles' Performance panel, and the manual documents them only on the Cycles page. | Whether EEVEE or Workbench use them: **not in the sources** |
| C4 Persistent Data | Same situation as B1: general setting, shown and documented only for Cycles. | Whether other engines use it: **not in the sources** |
| C2 Texture Cache | General render setting. Documented only for Cycles. The preference path says "Used by the Cycles Texture Cache." | inferred: Cycles only |
| C7 Max Subdivision, Child Particles, and the Simplify switch | General render settings. The manual documents Simplify only under Cycles. | Observed (general render settings storage); whether other engines honour these limits: **not in the sources** (see below) |
| B2 `-t` | "for rendering and other operations". Not tied to an engine. | documented |
| B3 `--qos` | Applies to "Blender threads". Not tied to an engine. | inferred |
| C5 Lock Interface | In the general Render menu. No engine named. | documented (no engine named) |

**Evidence.**

- Cycles registers its own settings group on the scene: `bpy.types.Scene.cycles = PointerProperty(... type=cls,)`. Its panels use `COMPAT_ENGINES = {'CYCLES'}` with `return context.engine in cls.COMPAT_ENGINES`. — Cycles file `properties.py`, lines 1189–1195; Cycles file `ui.py`, lines 63–71 (read 2026-10-16).
- The Threads panel draws `rd.threads_mode` and `rd.threads`, where `rd = scene.render`. The Final Render panel draws `rd.use_persistent_data`. — Cycles file `ui.py`, lines 873–890 and 994–1008 (read 2026-10-16).
- The EEVEE Performance page lists High Quality Normals, Anisotropic Filtering, Shadow Pool, Light Probes Volume Pool, Viewport Pixel Size, and the Compositor and Denoise Nodes settings. It has no Threads, Tile Size or Persistent Data. — https://docs.blender.org/manual/en/5.3/render/eevee/render_settings/performance.html (read 2026-10-11).
- "Texture Cache — Directory to store .tx texture cache files, relative to the source image file or as an absolute path. Used by the Cycles Texture Cache." — https://docs.blender.org/manual/en/5.3/editors/preferences/file_paths.html (read 2026-10-11).
- Simplify for other engines. Cycles leaves out a general panel named `'RENDER_PT_simplify'` and draws its own Simplify panel instead — Cycles file `ui.py`, lines 2561–2576 (the exclusion list) and 2344–2412 (its own panel) (read 2026-10-16). This implies that a general Simplify panel exists for other engines (**inferred** from the name being excluded). Its contents are not in my sources, and the manual has no Simplify page for EEVEE or Workbench.
- "Denoising and Compositing have their own device selection." — https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/index.html (read 2026-10-11).

**Label.** Documented or observed for storage and placement. Inferred where marked.

**Not covered.**

- Whether EEVEE or Workbench honour Threads, Persistent Data, Max Subdivision or Child Particles when rendering. *(Tested by hand on the Creator's Mac, 2026-10-17, below: Max Subdivision and Child Particles changed EEVEE's and Workbench's pictures (entry 23). For Threads and Persistent Data only short renders were timed (entries 25 and 26); EEVEE's three `-t 8` renders were each about 0.05 seconds shorter than the three `-t 1` renders, Workbench's did not move, and Persistent Data moved neither engine by more than about one per cent in one run each.)*
- What the general Simplify panel shows for those engines. *(Photographed, entry 40 below.)*

**Where I looked.**

- The EEVEE and Workbench Performance pages and the EEVEE limitations page.
- Searched the whole manual for "simplify". Only the Cycles Simplify page and unrelated tools came up.
- The API `RenderSettings`, `SceneEEVEE` and `Scene` pages.
- The Cycles `ui.py` panel classes and their `COMPAT_ENGINES`.

**Outside the interface.** *[Brought up to date by the hand tests of 2026-10-17 below: the renders were started from a terminal with no window, and the panels were photographed in Blender's own window.]* Engine membership is a property of where the setting is stored. It does not depend on the interface. The interface only hides panels that belong to another engine.

**Operating system.** *[Brought up to date by the hand tests of 2026-10-17 below: observed on macOS on Apple silicon only, where EEVEE and Workbench both rendered in a background run; Linux and Windows untested, and EEVEE on a headless Windows machine not tried — no such machine was available to this sitting.]* Not engine-dependent in the sources, with one exception: EEVEE's note that "Headless rendering is not supported on headless Windows systems."

### Tested by hand on the Creator's Mac — 2026-10-17

**Label: observed.** Run by Claude Opus 5.5 in Claude Code from a terminal, on the Creator's Mac — MacBook Pro (MacBookPro18,3), Apple M1 Pro, Apple's own 14-core graphics through Metal, 16 GB, macOS 26.6.2 (build 25G83), no second graphics card — in Blender 5.3.0 Beta, hash `5d8151335b6b`. The machine and the rules every run kept are written out under question 1's hand-test section; the full procedures are in `Audits/Blender Research/What a render demands/Hand tests — 5.3.md`, by entry number.

**Whether Simplify's Render limits reach EEVEE and Workbench as well as Cycles (entry 23).** Two files at 160 by 120: a cube with a Subdivision Surface modifier at render level 4 and a sun; and a plane carrying 30 hairs with interpolated children (100 per parent at render) and a sun. Each rendered from the terminal under each engine, given with `-E` after the file name — once as saved (Simplify off), and once with a start-up script switching Simplify on and setting the Render limit to nothing (`SIMPLIFY True 0` for Max Subdivision, `SIMPLIFY True 0.0` for Child Particles). All twelve renders exited 0. Each pair compared by pixels:

```
subdivision_CYCLES_off_vs_on: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.2541359253227711 NUM-DIFFERING-CHANNEL-VALUES 25515 of 76800 MEAN-ABS-DIFF 0.015583620941761182 SIZE (160, 120)
childparticles_CYCLES_off_vs_on: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.28261405788362026 NUM-DIFFERING-CHANNEL-VALUES 29463 of 76800 MEAN-ABS-DIFF 0.03557576904022426 SIZE (160, 120)
subdivision_BLENDER_EEVEE_off_vs_on: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.250274658203125 NUM-DIFFERING-CHANNEL-VALUES 25912 of 76800 MEAN-ABS-DIFF 0.01546609918276469 SIZE (160, 120)
childparticles_BLENDER_EEVEE_off_vs_on: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.24591064453125 NUM-DIFFERING-CHANNEL-VALUES 31154 of 76800 MEAN-ABS-DIFF 0.03425968289375305 SIZE (160, 120)
subdivision_BLENDER_WORKBENCH_off_vs_on: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.31744384765625 NUM-DIFFERING-CHANNEL-VALUES 25663 of 76800 MEAN-ABS-DIFF 0.037816125551859536 SIZE (160, 120)
childparticles_BLENDER_WORKBENCH_off_vs_on: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.269287109375 NUM-DIFFERING-CHANNEL-VALUES 30560 of 76800 MEAN-ABS-DIFF 0.04284907778104146 SIZE (160, 120)
```

Under all three engines, both Render limits changed the picture. EEVEE and Workbench both rendered in a background run on this Mac.

**Whether the thread count changes anything under EEVEE or Workbench (entry 25; B1).** The 640 by 480 file of question 2 rendered under each engine with `-E` after the file name, EEVEE's render samples set to 16 by a start-up script, with `-t 1` and with `-t 8`, three times each, under macOS's own timer. The last figure on each line is Blender's own `Time:` line for the render (the helper that prints it labels it "Cycles" whatever the engine):

```
BLENDER_EEVEE -t 1 rep1: EXIT 0 | wall 1.18 s | cpu 83% | peak 367558656 bytes | Cycles Time: 00:00.39
BLENDER_EEVEE -t 1 rep2: EXIT 0 | wall 1.19 s | cpu 82% | peak 370180096 bytes | Cycles Time: 00:00.39
BLENDER_EEVEE -t 1 rep3: EXIT 0 | wall 1.17 s | cpu 83% | peak 369934336 bytes | Cycles Time: 00:00.38
BLENDER_EEVEE -t 8 rep1: EXIT 0 | wall 1.14 s | cpu 86% | peak 397885440 bytes | Cycles Time: 00:00.34
BLENDER_EEVEE -t 8 rep2: EXIT 0 | wall 1.12 s | cpu 86% | peak 397852672 bytes | Cycles Time: 00:00.33
BLENDER_EEVEE -t 8 rep3: EXIT 0 | wall 1.14 s | cpu 86% | peak 397492224 bytes | Cycles Time: 00:00.34
BLENDER_WORKBENCH -t 1 rep1: EXIT 0 | wall 0.91 s | cpu 84% | peak 324763648 bytes | Cycles Time: 00:00.11
BLENDER_WORKBENCH -t 1 rep2: EXIT 0 | wall 0.89 s | cpu 86% | peak 326696960 bytes | Cycles Time: 00:00.10
BLENDER_WORKBENCH -t 1 rep3: EXIT 0 | wall 0.90 s | cpu 85% | peak 325009408 bytes | Cycles Time: 00:00.11
BLENDER_WORKBENCH -t 8 rep1: EXIT 0 | wall 0.91 s | cpu 85% | peak 329678848 bytes | Cycles Time: 00:00.11
BLENDER_WORKBENCH -t 8 rep2: EXIT 0 | wall 0.90 s | cpu 85% | peak 329957376 bytes | Cycles Time: 00:00.11
BLENDER_WORKBENCH -t 8 rep3: EXIT 0 | wall 0.90 s | cpu 86% | peak 329613312 bytes | Cycles Time: 00:00.11
```

The load average was 6.68 before and 7.32 after; only the Creator's own Blender was open beside these runs. Under EEVEE, the three `-t 8` renders were each about 0.05 seconds shorter by Blender's own count, and each about 28 MB higher in peak memory, than the three `-t 1` renders. Under Workbench the times did not move and peak memory was about 4 MB higher with `-t 8`. Every process used under one processor's worth either way. These renders take well under a second, so a difference in how a longer render spends its time would not show here.

**Persistent Data under EEVEE (entry 26, written in full under question 1).** A ten-frame EEVEE animation with Persistent Data off and on: `wall 7.24 s ... peak 406487040 bytes` and `wall 7.26 s ... peak 404455424 bytes`, one run each on a busy Mac.

**Whether Cycles' own values survive the engine being switched away and back (entry 27).** A file saved with Tile Size 256, Samples 11 and Use Tiling off: opened and printed, engine set to EEVEE and saved; reopened and printed, engine set back to Cycles and saved; reopened and printed — each step a separate Blender started from the terminal:

```
STEP CYCLES tile_size 256 samples 11 use_auto_tile False
STEP BLENDER_EEVEE tile_size 256 samples 11 use_auto_tile False
STEP CYCLES tile_size 256 samples 11 use_auto_tile False
```

Nothing was dropped or reset, and Cycles' values read back while EEVEE was the engine.

**Which Performance sub-panels each engine shows (entry 39).** This needs Blender's window. Blender 5.3 was started from the terminal with its window, factory settings, a settings folder of the test's own, and a script that set the interface scale to 0.7, made the Performance panels and their sub-panels start open (by removing their "starts closed" flag and registering them again before the Render tab was first drawn; every other Render-tab panel was made to start closed, and the order panels appear in may differ from Blender's usual one), maximised the Properties editor on the Render tab, switched the engine through Cycles, EEVEE and Workbench, took Blender's own screenshot of the editor with `bpy.ops.screen.screenshot_area`, and quit. **Each picture names its engine in its own Render Engine field**, and that is how each is identified here: the pictures came out one drawing behind the script's steps, so the file names the script gave them do not match the engine they show (the hand-test list records this, and two earlier attempts). What the three pictures show under Performance, read off them:

- **Render Engine: Cycles** — Compositor (Device CPU / GPU, Precision), with Denoise Nodes under it (Denoising Device Auto / CPU / GPU, Preview Quality, Final Quality); Threads (Threads Mode Auto-Detect, Threads 8, greyed); Memory (Tile Size 2048); Texture Cache, with its own tick (ticked) and Auto Generate, Generate and Clear; Acceleration Structure (Use Spatial Splits, Use Compact BVH); Final Render (Persistent Data); Viewport (Pixel Size Automatic).
- **Render Engine: EEVEE** — High Quality Normals and Anisotropic Filtering (2×) drawn straight in the Performance panel; then Compositor with Denoise Nodes as above; Memory (Shadow Pool 512 MB, Light Probes Volume Pool 16 MB); Viewport (Pixel Size Automatic).
- **Render Engine: Workbench** — High Quality Normals and Anisotropic Filtering drawn straight in it; Compositor with Denoise Nodes; nothing else.

Threads, Texture Cache, Acceleration Structure and Final Render (Persistent Data) appear under Cycles and under neither other engine. Every picture also shows a closed Simplify panel with its own tick, under all three engines. The script also asked each panel class whether it would draw; the Performance-family answers:

```
== CYCLES: CYCLES_RENDER_PT_performance;CYCLES_RENDER_PT_performance_compositor (under CYCLES_RENDER_PT_performance);CYCLES_RENDER_PT_performance_compositor_denoise_settings (under CYCLES_RENDER_PT_performance_compositor);CYCLES_RENDER_PT_performance_threads (under CYCLES_RENDER_PT_performance);CYCLES_RENDER_PT_performance_memory (under CYCLES_RENDER_PT_performance);CYCLES_RENDER_PT_performance_texture_cache (under CYCLES_RENDER_PT_performance);CYCLES_RENDER_PT_performance_acceleration_structure (under CYCLES_RENDER_PT_performance);CYCLES_RENDER_PT_performance_final_render (under CYCLES_RENDER_PT_performance);CYCLES_RENDER_PT_performance_viewport (under CYCLES_RENDER_PT_performance)
== BLENDER_EEVEE: RENDER_PT_eevee_performance;RENDER_PT_eevee_performance_compositor (under RENDER_PT_eevee_performance);RENDER_PT_eevee_performance_compositor_denoise_settings (under RENDER_PT_eevee_performance_compositor);RENDER_PT_eevee_performance_memory (under RENDER_PT_eevee_performance);RENDER_PT_eevee_performance_viewport (under RENDER_PT_eevee_performance);CYCLES_RENDER_PT_performance_acceleration_structure (under CYCLES_RENDER_PT_performance)
== BLENDER_WORKBENCH: RENDER_PT_eevee_performance;RENDER_PT_eevee_performance_compositor (under RENDER_PT_eevee_performance);RENDER_PT_eevee_performance_compositor_denoise_settings (under RENDER_PT_eevee_performance_compositor);CYCLES_RENDER_PT_performance_acceleration_structure (under CYCLES_RENDER_PT_performance)
```

Asked on its own, Cycles' Acceleration Structure panel answers that it would draw under EEVEE and Workbench too; its parent, Cycles' Performance panel, does not, and neither EEVEE's nor Workbench's picture shows it. The pictures settle what is drawn; a panel's own answer taken alone does not.

**What the Simplify panel shows under each engine (entry 40).** The same window procedure as entry 39, with the Simplify panel and its sub-panels made to start open instead of Performance. Read off the three pictures, each identified by its own Render Engine field; in all three the Simplify header's tick is clear and the fields under it are drawn dimmed, at factory values:

- **Render Engine: Cycles** — Viewport: Max Subdivision 6, Child Particles 1.000, Normals (tick), Volume Resolution 1.000, Texture Resolution 1.00, Texture Size Limit No Limit. Render: Max Subdivision 6, Child Particles 1.000, Texture Resolution 1.00, Texture Size Limit No Limit. Culling: Camera Culling 0.10 and Distance Culling 50 m, each with a tick. Grease Pencil, with its own tick: Playback Only, Fill, Modifiers, Shader Effects, Layers Tinting, Antialiasing.
- **Render Engine: EEVEE** — Viewport: Max Subdivision 6, Max Child Particles 1.000, Volume Resolution 1.000, Normals. Render: Max Subdivision 6, Max Child Particles 1.000. Grease Pencil, as under Cycles.
- **Render Engine: Workbench** — the same as EEVEE.

So a Simplify panel exists for EEVEE and Workbench, holding the same switch, Max Subdivision and child-particle limits under the label "Max Child Particles", and no texture limits and no culling. The same script's panel answers named it `RENDER_PT_simplify`, with `RENDER_PT_simplify_viewport`, `RENDER_PT_simplify_render` and `RENDER_PT_simplify_greasepencil` under it, for EEVEE and Workbench; and `CYCLES_RENDER_PT_simplify` with `CYCLES_RENDER_PT_simplify_viewport`, `CYCLES_RENDER_PT_simplify_render`, `CYCLES_RENDER_PT_simplify_culling` and `CYCLES_VIEW3D_PT_simplify_greasepencil` under it for Cycles. Entry 23 above shows the two Render limits this panel holds changing EEVEE's and Workbench's pictures.

---

## 5. Can their values be read out of a file?

**Answer.** It depends on the kind of setting.

- **Settings kept in the scene: yes.** That is A1, A2, A5, A6, B1, C1–C5, C7–C10, D1–D3 and Group E. Open the file in Blender, including in background mode with no interface, and read them through Python: `scene.render.*`, `scene.cycles.*`, `scene.eevee.*`. The Cycles settings are saved in the file even though Cycles defines them as an add-on.
- **Preference settings: not from the scene file.** That is A3, A7 and C6. They live in the user-preferences file of the machine running Blender, not in the `.blend` being rendered. On a running Blender they can be read from that Blender's preferences.
- **Launch-only options: no.** That is A4, B2 and B3. Nothing in the sources says they are written anywhere.
- **Without running Blender at all: the sources do not cover reading a `.blend`'s settings.** The only related fact is that compressed `.blend` files use Zstandard, "It is not unique to Blender so files can be compressed/decompressed with external tools." Blender can also be used as a Python module (`bpy`), but that is still Blender.

**Evidence.**

- "Accessing data from the currently loaded blend-file is done with the module bpy.data." — https://docs.blender.org/api/5.3/info_quickstart.html (read 2026-10-11).
- `-b`: "Run in background (often used for UI-less rendering)." — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html (read 2026-10-11).
- About add-on settings groups such as Cycles': "The values assigned to Blender data are saved to disk but the class definitions are not, this means whenever you load Blender the class needs to be registered too." — https://docs.blender.org/api/5.3/bpy.types.PropertyGroup.html (read 2026-10-11).
- The scene page gives the Cycles group as "Cycles render settings (readonly)" with type "CyclesRenderSettings | None". — https://docs.blender.org/api/5.3/bpy.types.Scene.html (read 2026-10-11).
- Preferences file: "./config/userpref.blend — User preferences." — https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html (read 2026-10-11). The Cycles device settings are `bpy.types.AddonPreferences` — Cycles file `properties.py`, line 1680 (read 2026-10-16).
- "It is not unique to Blender so files can be compressed/decompressed with external tools." — https://docs.blender.org/manual/en/5.3/files/blend/open_save.html (read 2026-10-11).
- `bpy.data.libraries.load(...)` exposes, for another file, "lists of strings representing linkable data-blocks". That is names only, not their settings. — https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html (read 2026-10-11).
- Unset values. Cycles' own version-update code treats a value as possibly "not set" in an older file and fills it in: `if not cscene.is_property_set("samples"): cscene.samples = 10`, under the comment `# Several default changes for 2.77`. It also reads raw stored values with `cscene.get("ao_bounces", 0)`. — Cycles file `version_update.py`, lines 152–158 and 213 (read 2026-10-16).

**Label.**

- Scene settings readable through Python in background mode: **inferred**. The step: background mode runs without an interface, `bpy.data` reads the loaded file, and the API pages name each setting. *(**Observed** 2026-10-17 on the Creator's Mac: entries 29 to 31 below and under question 6.)*
- Cycles values saved in the file: **documented** for add-on settings groups in general. That Cycles' settings are such a group is **observed**. *(Their values read back after save and reopen: entries 29 and 31.)*
- Preferences not in the scene file: **inferred** (step given under question 3). *(**Observed**: question 3, entries 12 and 14.)*
- Launch options not stored: **not in the sources**. No statement either way. *(Tested by hand for two of them, entries 9, 10 and 12 under question 3: `-t` and `-E` change the scene in the running Blender, and a session that saves afterwards writes `-E`'s engine into the new file; `--cycles-device` changes neither the scene's Device nor the saved preference.)*
- Unset values: **inferred**. A Cycles setting that was never changed may not be recorded as "set" in the file. Reading it through Python would then give the default of the Blender version doing the reading. The step: Cycles' own update code checks `is_property_set` and fills in old defaults for exactly that case. *(Tested by hand, question 6, entry 32: an untouched scene listed none of the four Cycles texture values as stored and `scene.cycles.get('texture_limit_render')` returned `None`, while reading the property by name gave `OFF`.)*

**Not covered.**

- Whether `scene.cycles` reads as "None" when the Cycles add-on is not enabled. The API type allows "None" but does not say when. Also not covered: whether the raw values can then still be read with `scene.get('cycles')` (hand test 5). *(Tested by hand, entry 28 below: with the add-on off, `scene.cycles` raised `AttributeError 'Scene' object has no attribute 'cycles'` rather than reading `None`; `scene.get('cycles')` returned `None` with the add-on off and on alike.)*
- Any way to read the values without Blender. *(Not tested: it needs a program other than Blender, and this sitting could run Blender and nothing else.)*

**Where I looked.**

- The API pages `info_quickstart`, `info_overview`, `bpy.props`, `PropertyGroup`, `AddonPreferences`, `BlendDataLibraries`, `bpy_struct` (for `get` and `is_property_set`), `Scene` and `info_advanced_blender_as_bpy`.
- The manual pages on opening, saving, compatibility and directory layout.
- Searched the manual for "userpref.blend".
- Searched the manual's blend-file pages for "file format", "compress" and "SDNA".

**Outside the interface.** *[Brought up to date by the hand tests of 2026-10-17 below: every read was made from a terminal with no window, and read back the values saved, with two conditions found — the Cycles add-on must be on in the reading Blender, and a value read after a launch option may be the option's rather than the file's (`-t`).]* Yes, this is how a reader with no interface would do it: `blender -b file.blend --python-expr …` and print the values. The values are the same inside and outside the interface. The sources describe no difference.

**Operating system.** *[Brought up to date by the hand tests of 2026-10-17 below: observed on macOS on Apple silicon only; a file written on one system and read on another was not tried, and Linux and Windows are untested — no machine running either was available to this sitting.]* No difference is stated. Which preferences file is read depends on the user's configuration folder, which differs by system. The directory-layout page covers that and does not change the answer.

### Tested by hand on the Creator's Mac — 2026-10-17

**Label: observed.** Run by Claude Opus 5.5 in Claude Code from a terminal, on the Creator's Mac — MacBook Pro (MacBookPro18,3), Apple M1 Pro, Apple's own 14-core graphics through Metal, 16 GB, macOS 26.6.2 (build 25G83), no second graphics card — in Blender 5.3.0 Beta, hash `5d8151335b6b`. The machine and the rules every run kept are written out under question 1's hand-test section; the full procedures are in `Audits/Blender Research/What a render demands/Hand tests — 5.3.md`, by entry number. Every read here was made by a Blender started from the terminal with no window, so **every result in this section is a result for reading a file from outside Blender's interface.**

**Whether a file's Cycles settings can be read with the Cycles add-on off, while EEVEE is the engine, or under factory startup (entry 28).** A script that prints the enabled add-ons, whether the scene type and the scene have `cycles`, three Cycles values, the engine, the scene's `keys()` and what `scene.get('cycles')` returns. (a) In a settings folder of the test's own, the Cycles add-on switched off with `bpy.ops.preferences.addon_disable(module="cycles")`, the preferences saved, and the file (Cycles, Device CPU) read at once in the same run; (b) the same file read by a fresh Blender against that folder; (c) the same file with the add-on on; (d) a file whose engine is EEVEE, add-on on:

```
== (a)
DISABLED-AND-SAVED
AFTER-ADDONS ['bl_pkg', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
AFTER-TYPE-HAS-CYCLES False
AFTER-SCENE-HAS-CYCLES False
AFTER-READ-FAILED AttributeError 'Scene' object has no attribute 'cycles'
AFTER-ENGINE BLENDER_EEVEE
AFTER-SCENE-KEYS []
AFTER-SCENE-GET-CYCLES NoneType None
== (b)
ADDONS ['bl_pkg', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
TYPE-HAS-CYCLES False
SCENE-HAS-CYCLES False
READ-FAILED AttributeError 'Scene' object has no attribute 'cycles'
ENGINE BLENDER_EEVEE
SCENE-KEYS []
SCENE-GET-CYCLES NoneType None
== (c)
ADDONS ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
TYPE-HAS-CYCLES True
SCENE-HAS-CYCLES True
SCENE-CYCLES <bpy_struct, CyclesRenderSettings("") at 0xac79d9520>
TILE-SIZE 2048 SAMPLES 64 DEVICE CPU
ENGINE CYCLES
SCENE-KEYS []
SCENE-GET-CYCLES NoneType None
== (d)
ADDONS ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
TYPE-HAS-CYCLES True
SCENE-HAS-CYCLES True
SCENE-CYCLES <bpy_struct, CyclesRenderSettings("") at 0x89f62cc20>
TILE-SIZE 2048 SAMPLES 64 DEVICE CPU
ENGINE BLENDER_EEVEE
SCENE-KEYS []
SCENE-GET-CYCLES NoneType None
```

and (e), the Cycles file read under `--factory-startup` with `print(hasattr(bpy.context.scene, 'cycles'), bpy.context.scene.cycles)`:

```
True <bpy_struct, CyclesRenderSettings("") at 0xb8d790e60>
```

With the add-on off, `scene.cycles` did not exist at all — an `AttributeError`, not `None` — and the same file that reads `CYCLES` as its engine in (c) read `BLENDER_EEVEE` in (a) and (b). `scene.get('cycles')` returned `None`, and `scene.keys()` was empty, **with the add-on off and with it on**, so in 5.3 neither gives a raw route to the Cycles values. With the add-on on, Cycles' values read back whichever engine the file names, and under `--factory-startup` `scene.cycles` was present.

**Whether nineteen settings survive being saved into a file and read back (entry 29).** A file written by a script with nineteen settings at unusual values, read by a fresh Blender with `--factory-startup`:

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

Every value came back as written.

**Whether another file's render settings can be read without making it the current file (entry 30).** A fresh Blender with no file of its own, a script using `bpy.data.libraries.load` on the file above, trying to reach a render setting inside the block, then taking the scene across (`dst.scenes = src.scenes`) and reading it after the block:

```
ENTRY 'Scene' str
FAILED AttributeError 'str' object has no attribute 'render'
LOADED-SCENE 'Scene.001'
  render.resolution_x 160 threads_mode FIXED threads 3
  cycles.samples 11 tile_size 256 device CPU
  eevee.taa_render_samples 23
CURRENT-FILE-SCENES ['Scene', 'Scene.001']
```

Inside the block the other file's scene is a name and nothing else. After the block its settings read back at the values saved, and the scene sits in the current session beside the session's own.

**What a value read out of a running Blender is worth (entries 9 and 12, written in full under question 3).** Two launch options behave in opposite ways when the scene is read back in the same run. `-t 8` on a file saved with Threads 1 made `scene.render.threads` read `8`. `--cycles-device` never moved `scene.cycles.device`, and the saved backend preference never moved either, before a render or after one — including four runs the pictures show were rendered somewhere other than what those two values say. So the stored Device, read back, tells what the file says, not what rendered.

---

## 6. What, if anything, does a saved scene record about limits on subdivision, child particles, and texture size, including any switch governing those limits, and what can be read back when the file is opened from outside Blender's interface?

**Answer.** The scene's render settings hold the Simplify switch and the subdivision and child-particle limits. Cycles' own scene settings hold the texture limits. Each limit comes in a viewport version and a final-render version.

| What | Name in the scene | Kind, range, default (as the sources give them) | Shown as |
|---|---|---|---|
| **Switch** | `scene.render.use_simplify` | on/off; "Enable simplification of scene for quicker preview renders (default False)" | the tick in the Simplify panel header |
| Subdivision limit, render | `scene.render.simplify_subdivision_render` | whole number; "Global maximum subdivision level during rendering (in [0, 32767], default 0)" | Render ‣ Max Subdivision |
| Subdivision limit, viewport | `scene.render.simplify_subdivision` | "Global maximum subdivision level (in [0, 32767], default 6)" | Viewport ‣ Max Subdivision |
| Child particles, render | `scene.render.simplify_child_particles_render` | fraction; "Global child particles percentage during rendering (in [0, 1], default 0.0)" | Render ‣ Child Particles |
| Child particles, viewport | `scene.render.simplify_child_particles` | "Global child particles percentage (in [0, 1], default 1.0)" | Viewport ‣ Child Particles |
| Texture size limit, render (Cycles) | `scene.cycles.texture_limit_render` | choice of `OFF` ("No Limit"), `128`, `256`, `512`, `1024`, `2048`, `4096`, `8192`; default `OFF` | Render ‣ Texture Size Limit |
| Texture size limit, viewport (Cycles) | `scene.cycles.texture_limit` | same choices; default `OFF` | Viewport ‣ Texture Size Limit |
| Texture scale, render (Cycles) | `scene.cycles.texture_resolution_render` | number 0.00001 to 1.0, default 1.0, "Scale factor for texture resolution used by final rendering" | Render ‣ Texture Resolution |
| Texture scale, viewport (Cycles) | `scene.cycles.texture_resolution` | same range, default 1.0 | Viewport ‣ Texture Resolution |

What the limits mean, in the manual's words:

- Max Subdivision: "Maximum number of subdivision by the Subdivision Surface modifiers."
- Child Particles: "Show only a subset of all child hairs and particles."
- Texture Size Limit: "Automatically scales textures down so that they are no larger than the values chosen."

The amounts being limited are stored elsewhere, on each object's modifier and on the particle settings:

- Subdivision Surface "render_levels": "Number of subdivisions to perform when rendering (in [0, 11], default 2)".
- Particle "rendered_child_count": "Number of children per parent for rendering (in [0, 100000], default 100)".

**The switch.** In Cycles' Simplify panel, the switch dims the Viewport, Render and Culling sub-panels: `layout.active = rd.use_simplify`. That covers the subdivision, child-particle and texture limits alike.

**What can be read back from outside the interface.** All of the scene values above, by opening the file in background mode and reading them through Python (see question 5).

- The four general values and the switch are built into Blender's render settings. They are always there.
- The four texture values are Cycles' settings. They are saved in the file. Reading them by name needs Cycles registered in the Blender doing the reading.
- A texture value that was never changed may not be recorded as set. Reading it then gives the reading Blender's default (question 5).

**Not part of the saved scene.** There is a separate texture-size limit for the viewport display in Preferences ‣ Viewport ‣ Textures ‣ Limit Size (`gl_texture_limit`, "Limit the texture size to save graphics memory (default 'CLAMP_OFF')"). It is a user preference, not a scene value. The manual describes it for "textured display".

**Evidence.**

- The API entries quoted in the table — https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html (read 2026-10-11).
- The Cycles texture settings: Cycles file `properties.py`, lines 183–192 (`enum_texture_limit`) and 1070–1098 (`texture_resolution`, `texture_resolution_render`, `texture_limit`, `texture_limit_render`) (read 2026-10-16).
- Panel layout and switch: Cycles file `ui.py`, lines 2350–2352 (`self.layout.prop(rd, "use_simplify", text="")`), lines 2373 and 2402 (`layout.active = rd.use_simplify`), lines 2376–2384 (viewport fields) and 2406–2412 (render fields) (read 2026-10-16).
- Manual meanings — https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/simplify.html (read 2026-10-11).
- Modifier and particle amounts — https://docs.blender.org/api/5.3/bpy.types.SubsurfModifier.html and https://docs.blender.org/api/5.3/bpy.types.ParticleSettings.html (read 2026-10-11).
- Saved with the file — the `PropertyGroup` note quoted under question 5.
- Preference limit — https://docs.blender.org/api/5.3/bpy.types.PreferencesSystem.html; https://docs.blender.org/manual/en/5.3/editors/preferences/viewport.html (read 2026-10-11).

**Label.**

- **Documented and observed:** the names, kinds, ranges and defaults in the table, and the panel layout.
- **Inferred:** that the general values are saved with the scene. The step: the API calls render settings "Rendering settings for a Scene data-block", and data-block data is what a `.blend` holds. *(**Observed** 2026-10-17 on the Creator's Mac, entry 31 below: all nine values read back after save and reopen, scene by scene.)*
- **Inferred, and possibly wrong:** that the switch also governs the texture limits during a render. The step: the switch dims those fields in the panel. But dimming only changes what the panel draws. The sources never say the renderer ignores the texture limit when Simplify is off (hand test 4). *(**Observed** 2026-10-17, entry 33 below: in a render started from a terminal, a stored render Texture Size Limit of 128, and a stored render Texture Resolution of 0.25, each changed the picture only with the switch on — as Max Subdivision and Child Particles did.)*
- **Ambiguity:** the "default" values in the table are the API's stated defaults. A new scene made from the user's startup file may hold different values. The sources do not say which. *(Observed, entry 32 below: the factory startup scene read render Max Subdivision 6 and render Child Particles 1.0, against the API's stated 0 and 0.0.)*

**Not covered.**

- Whether "Max Subdivision" also caps Cycles adaptive subdivision. That has its own "Max Subdivisions" setting (`scene.cycles.max_subdivisions`, 0–16, default 12). *(Tested by hand, entry 34 below: render Max Subdivision 0 left an adaptive-subdivision picture unchanged to every channel value.)*
- Whether the limits apply to other modifiers, such as Multiresolution. *(Tested by hand, entry 34 below: render Max Subdivision 0 changed a Multiresolution picture.)*
- What happens to child particles when the fraction is 0.0, the stated render default. *(Tested by hand, entry 33 below: with the switch on and render Child Particles 0.0, the picture of 30 hairs with interpolated children changed against the switch off; question 4's entry 23 shows the same under EEVEE and Workbench. What the changed picture shows was not read.)*

**Where I looked.**

- The Cycles Simplify manual page and the Cycles `ui.py` Simplify panels.
- The API `RenderSettings` page, every entry beginning "simplify".
- Searched the API and manual for "texture_limit", "texture size", "texture limit", "texture_resolution", "max_texture".
- Searched the manual for "Max Subdivision" and "child particles".
- The Subdivision Surface modifier page and the particle Children page.

**Outside the interface.** *[Brought up to date by the hand tests of 2026-10-17 below: every value read back in a Blender started from a terminal with no window, and in renders started the same way the switch governed all four limits — the open question above is answered by test on this Mac.]* The values are stored data and read the same way with or without the interface. The dimming effect of the switch is an interface effect only. What the renderer does with the switch is the open question above.

**Operating system.** *[Brought up to date by the hand tests of 2026-10-17 below: observed on macOS on Apple silicon only. A file written on one system and read on another was not tried, and Linux and Windows are untested — no machine running either was available to this sitting.]* No difference is stated.

### Tested by hand on the Creator's Mac — 2026-10-17

**Label: observed.** Run by Claude Opus 5.5 in Claude Code from a terminal, on the Creator's Mac — MacBook Pro (MacBookPro18,3), Apple M1 Pro, Apple's own 14-core graphics through Metal, 16 GB, macOS 26.6.2 (build 25G83), no second graphics card — in Blender 5.3.0 Beta, hash `5d8151335b6b`. The machine and the rules every run kept are written out under question 1's hand-test section; the full procedures are in `Audits/Blender Research/What a render demands/Hand tests — 5.3.md`, by entry number. Every run was started from the terminal with no window.

**Whether the Viewport-labelled settings touch a render started from outside (entry 24).** (a) The 640 by 480 Cycles file of question 2 (no textures, no subdivision) rendered with seven Viewport settings pushed to extremes by a start-up script and every Render counterpart left alone, compared with the file rendered as saved:

```
vp EXIT=0 VIEWPORT-SET 1 100.0 128 0.009999999776482582 0 8 1
base_vs_viewport_settings: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
```

(the seven: Cycles viewport samples 1, viewport dicing rate 100, viewport Texture Size Limit 128, viewport Texture Resolution 0.01, viewport Max Subdivision 0, Preview Pixel Size 8, EEVEE viewport samples 1). That file has nothing for the subdivision and texture limits to act on, so two more runs were made where they could. (b) The cube with Subdivision Surface at render level 4, Simplify on in both renders, render Max Subdivision 6 in both, viewport Max Subdivision 6 in one and 0 in the other:

```
sub vp6 EXIT=0 SIMPLIFY True viewport-max-subdiv 6 render-max-subdiv 6
sub vp0 EXIT=0 SIMPLIFY True viewport-max-subdiv 0 render-max-subdiv 6
viewport_maxsubdiv_6_vs_0: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 76800 MEAN-ABS-DIFF 0.0 SIZE (160, 120)
```

(c) The textured file of question 2's entry 20, Simplify on, viewport Texture Size Limit 128 and viewport Texture Resolution 0.01 (render values left at `OFF` and 1.0), against the file as saved; and, as the control that shows a texture limit can be seen in this picture, the **render** Texture Size Limit at 128 instead:

```
tex vp EXIT=0 VIEWPORT-TEX-SET simplify True texture_limit 128 texture_resolution 0.009999999776482582 render OFF 1.0
tex render-limit control EXIT=0 RENDER-TEX-SET simplify True texture_limit_render 128
tex_asis_vs_viewport_texture_settings: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.00030685216188430786 NUM-DIFFERING-CHANNEL-VALUES 418876 of 1228800 MEAN-ABS-DIFF 1.6233184496764844e-07 SIZE (640, 480)
control_tex_asis_vs_render_texture_limit: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.1034887433052063 NUM-DIFFERING-CHANNEL-VALUES 678060 of 1228800 MEAN-ABS-DIFF 0.046567861232633574 SIZE (640, 480)
```

The file as saved has the texture cache on, and by this point its `.tx` files existed beside the images (entry 20). Two runs of it as saved, unchanged, and the viewport texture settings again with the cache switched off:

```
tex_asis_run1_vs_run2: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.00044193118810653687 NUM-DIFFERING-CHANNEL-VALUES 298263 of 1228800 MEAN-ABS-DIFF 1.244104193117342e-07 SIZE (640, 480)
tex_nocache_vs_viewport_texture_settings_nocache: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 1228800 MEAN-ABS-DIFF 0.0 SIZE (640, 480)
```

So the 0.0003 difference in (c) is smaller than two unchanged runs of the same file differ by while the cache's `.tx` files are in use (0.0004); with the cache off, the viewport texture settings left every channel value unchanged; and the render Texture Size Limit changed the picture by up to 2.1. No Viewport-labelled setting reached a render started from outside.

**Whether the Simplify values survive save and reopen, scene by scene, with the switch off, and with another engine active (entry 31).** A script wrote four files and printed each scene's nine values before saving: `fact134-enabled.blend` with two scenes, `FACT134_A` (switch on; viewport/render Max Subdivision 3/5, Child Particles 0.37/0.42, Texture Size Limit 1024/2048, Texture Resolution 0.5/0.25) and `FACT134_B` (switch on; 1/7, 0.22/0.81, 256/4096, 0.75/0.125); `fact134-disabled.blend`, A's values with the switch off; `fact134-other-engine.blend`, A's values with the switch on and EEVEE active; and `fact134-defaults.blend` (entry 32). Each was then read by a separate Blender started from the terminal with `--factory-startup`, by a script printing every scene's nine values as JSON, the Cycles values Python lists as stored (`cycles_items`), and `scene.cycles.get('texture_limit_render')`:

```
FACT134_READBACK_BEGIN
version 5.3.0 Beta
build_platform b'Darwin'
operating_system macOS-26.6.2-arm64-arm-64bit-Mach-O
background True
loaded_path /private/tmp/claude-501/smoozy-hand-tests-5.3/What a render demands/data/fact134/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "texture_resolution": 0.5, "texture_resolution_render": 0.25, "use_simplify": true}
cycles_items FACT134_A [('sampling_pattern', 5), ('texture_limit', 4), ('texture_limit_render', 5), ('texture_resolution', 0.5), ('texture_resolution_render', 0.25)]
cycles_get texture_limit_render FACT134_A 5
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "texture_resolution": 0.75, "texture_resolution_render": 0.125, "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6), ('texture_resolution', 0.75), ('texture_resolution_render', 0.125)]
cycles_get texture_limit_render FACT134_B 6
FACT134_READBACK_END
```

The disabled and other-engine files, the lines that differ from the header above:

```
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_DISABLED", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "texture_resolution": 0.5, "texture_resolution_render": 0.25, "use_simplify": false}
cycles_items FACT134_DISABLED [('sampling_pattern', 5), ('texture_limit', 4), ('texture_limit_render', 5), ('texture_resolution', 0.5), ('texture_resolution_render', 0.25)]
{"cycles_present": true, "engine": "BLENDER_EEVEE", "scene": "FACT134_OTHER_ENGINE", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "texture_resolution": 0.5, "texture_resolution_render": 0.25, "use_simplify": true}
cycles_items FACT134_OTHER_ENGINE [('sampling_pattern', 5), ('texture_limit', 4), ('texture_limit_render', 5), ('texture_resolution', 0.5), ('texture_resolution_render', 0.25)]
```

Every value read back as printed before saving, each scene keeping its own, the switch's value kept apart from its limits, and Cycles' texture values readable while EEVEE was the engine. All four reads exited 0.

**Whether a value never touched is recorded in the file, against the same value set on purpose (entry 32).** `fact134-defaults.blend` holds `FACT134_UNTOUCHED`, the factory scene with nothing set, and `FACT134_EXPLICIT_DEFAULTS`, a full copy of it with the nine values set to the API's stated defaults. Read the same way:

```
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_EXPLICIT_DEFAULTS", "simplify_child_particles": 1.0, "simplify_child_particles_render": 0.0, "simplify_subdivision": 6, "simplify_subdivision_render": 0, "texture_limit": "OFF", "texture_limit_render": "OFF", "texture_resolution": 1.0, "texture_resolution_render": 1.0, "use_simplify": false}
cycles_items FACT134_EXPLICIT_DEFAULTS [('sampling_pattern', 5), ('texture_limit', 0), ('texture_limit_render', 0), ('texture_resolution', 1.0), ('texture_resolution_render', 1.0)]
cycles_get texture_limit_render FACT134_EXPLICIT_DEFAULTS 0
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_UNTOUCHED", "simplify_child_particles": 1.0, "simplify_child_particles_render": 1.0, "simplify_subdivision": 6, "simplify_subdivision_render": 6, "texture_limit": "OFF", "texture_limit_render": "OFF", "texture_resolution": 1.0, "texture_resolution_render": 1.0, "use_simplify": false}
cycles_items FACT134_UNTOUCHED [('sampling_pattern', 5)]
cycles_get texture_limit_render FACT134_UNTOUCHED None
```

The untouched scene lists none of the four Cycles texture values as stored, and `get('texture_limit_render')` returns `None` for it, while reading the property by name still gives `OFF`. The scene where they were set lists all four and returns `0`. The untouched factory scene also reads render Max Subdivision 6 and render Child Particles 1.0, where the API's stated defaults are 0 and 0.0.

**Whether the Simplify switch governs each limit in a render started from outside (entry 33).** This is the test the label above marks "inferred, and possibly wrong" for the texture limits. A script builds one case at 128 by 128 in Cycles on the processor (16 samples, seed fixed, denoising and adaptive sampling off, PNG), sets one Render limit, renders twice with the Simplify switch off and once with it on, and compares the pictures by pixels. The cases: a cube with Subdivision Surface at render level 4, against render Max Subdivision 0; a plane with 30 hairs and interpolated children, against render Child Particles 0.0; a plane seen straight on carrying a generated 512 by 512 colour grid (closest-pixel sampling), against render Texture Size Limit 128 alone; and the same plane against render Texture Resolution 0.25 alone. **The first run of the script stopped at its first picture** with `TypeError: incorrect sequence item type: d`, raised while reading the rendered pixels into a buffer of the wrong number type; the script was corrected to read them as Blender hands them over and run again, and both runs are in the hand-test list. The second run printed:

```
CASE subdivision LIMIT simplify_subdivision_render = 0
pixel_diff {"case": "subdivision", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "subdivision", "changed_channels": 21004, "comparison": "off-versus-on", "max_abs_diff": 0.3450980484485626, "mean_abs_diff": 0.03432467660945804}
CASE child_particles LIMIT simplify_child_particles_render = 0.0
pixel_diff {"case": "child_particles", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "child_particles", "changed_channels": 18470, "comparison": "off-versus-on", "max_abs_diff": 0.4627451151609421, "mean_abs_diff": 0.04217876565485312}
CASE texture_limit LIMIT texture_limit_render = 128
pixel_diff {"case": "texture_limit", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "texture_limit", "changed_channels": 39513, "comparison": "off-versus-on", "max_abs_diff": 0.30980393290519714, "mean_abs_diff": 0.01787013674498894}
CASE texture_resolution LIMIT texture_resolution_render = 0.25
pixel_diff {"case": "texture_resolution", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "texture_resolution", "changed_channels": 39513, "comparison": "off-versus-on", "max_abs_diff": 0.30980393290519714, "mean_abs_diff": 0.01787013674498894}
```

In every case two renders with the switch off were identical and the render with it on was not: with each limit stored in the scene, the picture changed only when the switch was on. That holds for the Texture Size Limit and the Texture Resolution as much as for Max Subdivision and Child Particles. (The two texture cases came out with the same figures; 128 is a quarter of 512.)

**Whether Max Subdivision also limits Multiresolution and adaptive subdivision (entry 34).** The same script, with a cube carrying a Multiresolution modifier subdivided three times (render level 3), and a cube with Subdivision Surface at render level 4 and adaptive subdivision switched on (`SUBSURF render_levels 4 adaptive True`), each against render Max Subdivision 0:

```
MULTIRES total_levels 3 render_levels 3
pixel_diff {"case": "multires", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "multires", "changed_channels": 21036, "comparison": "off-versus-on", "max_abs_diff": 0.35686275362968445, "mean_abs_diff": 0.034378231947584936}
SUBSURF render_levels 4 adaptive True
pixel_diff {"case": "adaptive", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "adaptive", "changed_channels": 0, "comparison": "off-versus-on", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
```

and two controls, which show the adaptive case really was rendered differently from plain subdivision, and did not end up as the limited plain cube:

```
control_subdivision_off_vs_adaptive_off: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.027450978755950928 NUM-DIFFERING-CHANNEL-VALUES 4031 of 65536 MEAN-ABS-DIFF 0.00036214194210515416 SIZE (128, 128)
control_subdivision_on_vs_adaptive_on: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.34117648005485535 NUM-DIFFERING-CHANNEL-VALUES 20923 of 65536 MEAN-ABS-DIFF 0.034301938058433734 SIZE (128, 128)
```

Render Max Subdivision 0 changed the Multiresolution picture. It left the adaptive-subdivision picture unchanged to every channel value.

**Written in full under question 4, and bearing on this one.** With the switch on, render Max Subdivision 0 and render Child Particles 0.0 changed the picture under EEVEE and Workbench as well as Cycles (entry 23); and the Simplify panel those two engines show holds the switch, Max Subdivision and Max Child Particles, with no texture limits (entry 40).

---

## 7. Where a render can be told which of a graphics card's rendering methods to use, what methods are there, what does choosing one rather than another change about the render, and does anything establish whether that changes with the size of the scene?

**How I read the question.** It is ambiguous.

- My reading: Cycles' card methods (CUDA, OptiX, HIP, oneAPI, Metal) and their hardware ray-tracing switches. That is the main answer below.
- The other reading: the graphics system Blender draws with (OpenGL, Vulkan, Metal), which EEVEE and Workbench also render through. That is answered briefly at the end.

**Where a render is told.**

1. In user preferences (Preferences ‣ System ‣ Cycles Render Devices), together with the scene's Device set to GPU Compute.
2. At launch, with `--cycles-device`.

**The methods.**

- **In preferences:** None (meaning the processor), CUDA, OptiX, HIP, oneAPI and Metal. Only the methods this computer supports are listed. The manual's own screenshot of the System section shows only "None", "CUDA" and "HIP" (picture `editors_preferences_section_system.webp` on https://docs.blender.org/manual/en/5.3/editors/preferences/system.html, read 2026-10-11).
- **At launch:** CPU, CUDA, OPTIX, HIP, ONEAPI, METAL, each optionally with "+CPU".
- **Switches within a method:**
  - HIP RT, for HIP, default on.
  - Embree on GPU, for oneAPI, default on.
  - MetalRT for Metal: Off / On / Auto, default Auto.
  - Kernel Optimization for Metal: Off / Intersection only / Full, default Full.

  The MetalRT and Kernel Optimization switches are shown only on Apple Silicon. Each hardware ray-tracing switch is dimmed if any chosen card lacks hardware ray tracing: `# Any device without RT support will disable it for all.`

**What choosing one changes.**

| Method | Speed or memory, as stated | Effect on the result, as stated |
|---|---|---|
| CUDA | — | Path Guiding not available (no GPU method has it). Open Shading Language not available. |
| OptiX | "takes advantage of hardware ray-tracing acceleration in RTX graphics cards, for improved performance". The Acceleration Structure settings are not offered. | Open Shading Language is available only here among card methods, "with some limitations". Under OSL on OptiX, "The trace function is not functional", so "the Ambient Occlusion and Bevel nodes do not work". Custom cameras: "not supported with GPU rendering unless using the OptiX backend". The OptiX denoiser is offered only when OptiX is the chosen method. |
| HIP | HIP RT: "Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above." | "Shadow Caustics are not supported when using HIP without hardware ray-tracing." Under shadow caustics: "GPU rendering using Radeon RX 6000 series HIP devices on Windows is not supported." |
| oneAPI | "Hardware acceleration for ray-tracing and denoising is supported." Embree on GPU gives "better overall performance". | Nothing beyond the general card limits. |
| Metal | MetalRT "uses less memory for scenes which use curves extensively, and can give better performance in specific cases". Off "uses BVH2 layout". Auto picks "the fastest intersection method". Kernel Optimization trades rendering speed against brief extra processor use (question 1, A7). | Nothing beyond the general card limits. |
| All card methods | Each method uses its own ray-tracing acceleration structure (observed). | Path Guiding "not supported on any GPU". Render Time pass only on the processor. |
| CUDA, OptiX, HIP, Metal (oneAPI not named) | "if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact" | — |
| NVIDIA (NVLink) | Distribute Memory Across Devices shares memory "at a small performance cost". | — |

**Does anything establish whether the difference changes with the size of the scene? No.** The sources come close in four places, but none of them settles it:

1. "What renders faster? This varies depending on the hardware used. Different technologies also have different compute times depending on the scene tested." It says the result depends on the scene. It does not mention size.
2. The fall-back to system memory "if the GPU memory is full" is a size-related behaviour that differs by method. It is stated for CUDA, OptiX, HIP and Metal. oneAPI is not named, and the sources do not say what oneAPI does.
3. MetalRT's memory saving is tied to "scenes which use curves extensively", which is content, not size. Auto picks "the fastest intersection method" without saying on what basis. Kernel Optimization is "based on scene content".
4. The Metal kernel file has an "extended limits" variant (`#  if defined(__METALRT_EXTENDED_LIMITS__)`). Nothing in my sources says what selects it or which limits it extends.

**Evidence.**

- Preference options and requirements — https://docs.blender.org/manual/en/5.3/editors/preferences/system.html and https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html (read 2026-10-11). Quotes in the table come from these two pages, except as noted below.
- The OSL limits: "OSL is not supported with GPU rendering unless using the OptiX backend." "The trace function is not functional. As a result of this, the Ambient Occlusion and Bevel nodes do not work." — https://docs.blender.org/manual/en/5.3/render/cycles/osl/index.html (read 2026-10-11).
- Custom cameras: "Custom cameras are not supported with GPU rendering unless using the OptiX backend." — https://docs.blender.org/manual/en/5.3/render/cycles/osl/camera.html (read 2026-10-11).
- HIP on Windows under shadow caustics — https://docs.blender.org/manual/en/5.3/render/cycles/object_settings/object_data.html (read 2026-10-11).
- Acceleration Structure: "The Acceleration Structure panel will not be present when using an Optix compute device." — https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html (read 2026-10-11). The panel's own test is `return not use_optix(context) or use_multi_device(context)`, so it does show for OptiX when the processor is used as well — Cycles file `ui.py`, lines 949–951 (read 2026-10-16).
- The OptiX denoiser test is `if compute_device_type == 'OPTIX':` — Cycles file `properties.py`, lines 1918–1936. The manual: "Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences." — https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html (read 2026-10-11).
- The method list is built from what is detected: `if has_cuda: list.append(('CUDA', ...` and so on. The default is Metal on Apple Silicon and None elsewhere: `if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'): return 5`. — Cycles file `properties.py`, lines 1683–1708 (read 2026-10-16).
- The switches, their defaults and the conditions for showing them: Cycles file `properties.py`, lines 1725–1760 and 2137–2165 (read 2026-10-16). A device that is not optimised is labelled `" (Unoptimized Performance)"` (lines 2047–2048). The file does not say what that means for the render.
- Per-method acceleration structures. Cycles file `source/kernel/bvh/bvh.h`, lines 19–31 (read 2026-10-16), under the comment `/* Device specific acceleration structures for ray tracing. */`:
  - `__EMBREE__` → `kernel/device/cpu/bvh.h`
  - `__KERNEL_METALRT__` → `kernel/device/metal/bvh.h`
  - `__KERNEL_OPTIX__` → `kernel/device/optix/bvh.h`
  - `__KERNEL_HIPRT__` → `kernel/device/hiprt/bvh.h`
  - otherwise `__BVH2__`
- Extended limits: Cycles file `source/kernel/device/metal/compat.h`, lines 335–339 (read 2026-10-16).
- Release notes, 5.3, Cycles: "Various GPU render optimizations" and "Support for NVIDIA RTX Spark devices running Windows 11, including unified memory optimizations." — https://developer.blender.org/docs/release_notes/5.3/cycles/ (read 2026-10-11).

**Label.**

- **Documented:** the method list, requirements, speed claims and feature limits.
- **Observed:** the switches, defaults, display conditions, per-method acceleration structures, and the OptiX-only denoiser test.
- **Inferred:** that these feature limits change the image when a scene uses those features (step under question 2).
- **Not in the sources:** whether the speed difference between methods changes with scene size. Also not in the sources: what Auto in MetalRT decides on, and whether oneAPI falls back to system memory. *(Tested by hand on the Creator's Mac, 2026-10-17, below, for what this Mac has: the processor against Apple's graphics on three scene sizes, and MetalRT Off, On and Auto on the same three (entries 35 and 36) — Auto built and reported as Off on all three, and Cycles' own log said `Hardware Ray-Tracing: Off` for it. CUDA against OptiX, and oneAPI, could not be run: no NVIDIA or Intel card in this sitting.)*
- **Drawing system (other reading):** **documented** (backends, default Vulkan on Windows and Linux, launch option, benchmark frames-per-second table); **not in the sources** (whether backend changes final render or depends on scene size).

**Not covered.**

- Whether two methods give identical pixels for the same scene. *(Tested by hand, entries 11, 35 and 36 below and under question 2: the processor and Apple's graphics do not, nor do MetalRT Off and On; Auto and Off sat at the noise floor. NVIDIA, AMD and Intel methods could not be run.)*
- What the speed claims amount to. No figures are given. *(Measured for Metal on this Mac, entries 35 to 37 below, on a busy machine.)*
- What "(Unoptimized Performance)" means. *(Not settled: no device on this Mac carried that label in Preferences, entry 42 below, so nothing here could show what it means for a render.)*

**The other reading (the drawing system).**

- The manual's Backend offers OpenGL and Vulkan: "Selects the graphics API used for drawing the interface and rendering display content." "Changing the backend requires restarting Blender for the change to take effect." — https://docs.blender.org/manual/en/5.3/editors/preferences/system.html (read 2026-10-11).
- The API lists `OPENGL`, `METAL` and `VULKAN`, default `VULKAN` — https://docs.blender.org/api/5.3/bpy.types.PreferencesSystem.html (read 2026-10-11).
- At launch: `--gpu-backend` "Force to use a specific GPU backend. Valid options: vulkan, metal, opengl." — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html (read 2026-10-11).
- Release notes: "Vulkan backend is now default on Windows (not ARM) and Linux." They also give a table of frames-per-second for seven named test scenes under OpenGL and Vulkan, on one benchmark system. The gap between the two backends differs from scene to scene. The table does not give the scenes' sizes, and does not say the figures are final renders. On Workbench hardware-ray-traced shadows (Metal and Vulkan): "The visual results are the same as the stencil shadows implementation". — https://developer.blender.org/docs/release_notes/5.3/eevee/ (read 2026-10-11).
- Nothing in the sources says whether the backend changes an EEVEE final render, or whether that depends on scene size. *(Not testable on this sitting's machine: the Mac build takes only `metal` for `--gpu-backend` (entries 2 and 15 below), so no two backends could be compared; a Linux or Windows machine would be needed, and none was available.)*

**Where I looked.**

- The whole GPU Rendering, Preferences System, OSL and OSL camera pages.
- Searched the whole manual for: OptiX, HIP, oneAPI, MetalRT, HIP RT, Embree, CUDA, hardware ray.
- Searched the manual, the 5.3 release notes and the Cycles Python files for: large scene, larger scene, big scene, scene size, size of the scene, complex scene, heavy scene, small scene, depending on the scene, scene content, scene complexity.
- Searched the Cycles kernel device folders for: metalrt, extended limits, memory, limit, not supported, unsupported, scene size.
- I ruled out the kernel launch-size files (`config.h`, `block_sizes.h`, `hiprt/globals.h`). They are about how work is split across the card. They say nothing about how the methods compare.

**Outside the interface.**

*[Brought up to date by the hand tests of 2026-10-17 below, run from a terminal with no window except the window photographs of entries 41 and 42. Observed on this Mac: `--cycles-device` beat both the preference and the scene's Device, in both directions (entry 12); all six Cycles preference values could be set by a start-up script with no window (entry 4), and MetalRT and Kernel Optimization set that way changed what the render built and printed (entries 36 and 37); a method this Mac has no hardware for ended the run with exit status 1 and no image (entry 13). With `--log "cycles" --log-level info`, Cycles printed which device it path-traced on and whether hardware ray tracing was on (entry 36).]*

- The method can be chosen without the interface by `--cycles-device`, which overrides both the preference and the scene setting.
- HIP RT, Embree on GPU, MetalRT and Kernel Optimization have no launch option in the sources. They are preferences, so a launch script would be needed (inferred).
- The MetalRT and Kernel Optimization rows are drawn only on Apple Silicon. That is an interface condition. The preference values exist regardless (inferred).

**Operating system.**

*[Brought up to date by the hand tests of 2026-10-17 below. Observed on macOS on Apple silicon (an M1 Pro) only: the preference offers `None` and `Metal`, starts at `Metal`, and refuses the other four by name (entry 3); `--gpu-backend` takes `metal` and refuses `opengl` and `vulkan` by name, the render still finishing (entry 15). Linux and Windows untested, and no NVIDIA, AMD or Intel card — none was available to this sitting; what those systems and cards do is not answered here.]*

- CUDA, OptiX, HIP and oneAPI: "supported on Windows and Linux".
- Metal: Apple Silicon Macs, "macOS 13.0 or newer is required to support all features".
- AMD card denoising: "discrete Radeon RX 6000 and newer GPUs on Linux, and discrete Radeon RX 7000 and newer GPUs on Windows".
- HIP with Radeon RX 6000 on Windows: not supported for shadow caustics.
- Vulkan is the default drawing backend "on Windows (not ARM) and Linux".
- Driver minimums differ between the two sources. The manual says OptiX needs "a driver version of at least 575". Cycles' own file sets `optix_minimum_driver_version = "580"` and `cuda_minimum_driver_version = "580"` (Cycles file `properties.py`, lines 1958–1959, read 2026-10-16). The two sources disagree. I report both.

### Tested by hand on the Creator's Mac — 2026-10-17

**Label: observed.** Run by Claude Opus 5.5 in Claude Code from a terminal, on the Creator's Mac — MacBook Pro (MacBookPro18,3), Apple M1 Pro, Apple's own 14-core graphics through Metal, 16 GB, macOS 26.6.2 (build 25G83), no second graphics card — in Blender 5.3.0 Beta, hash `5d8151335b6b`. The machine and the rules every run kept are written out under question 1's hand-test section; the full procedures are in `Audits/Blender Research/What a render demands/Hand tests — 5.3.md`, by entry number. **This Mac answers the Metal half of this question and nothing else**: nothing here bears on CUDA, OptiX, HIP or oneAPI in use, and nothing was run on Linux or Windows.

**What Blender's own help says, on this build (entry 2).** `blender --help`, exit 0, prints for the three options this question touches:

```
63:--cycles-device <device>
64-	Set the device used for rendering.
65-	Valid options are: 'CPU' 'CUDA' 'OPTIX' 'HIP' 'ONEAPI' 'METAL'.
71:--cycles-print-stats
72-	Log statistics about render memory and time usage.
357:--gpu-backend 
358-	Force to use a specific GPU backend. Valid options: 'metal'.
```

The help printed by this macOS build names only `metal` for `--gpu-backend`, where the manual page quoted above names "vulkan, metal, opengl".

**Which methods the preference offers, which it accepts, and what each has behind it (entry 3).** Blender started with no window and `--factory-startup`, asked through the Cycles add-on's own preference object:

```
get_device_types(context) = [('NONE', 'None', 'Do not use compute device', 0), ('METAL', 'Metal', 'Use Metal for GPU acceleration', 5)]
get_devices_for_type CUDA = []
get_devices_for_type OPTIX = []
get_devices_for_type HIP = []
get_devices_for_type METAL = [('Apple M1 Pro (GPU - 14 cores)', 'METAL', True), ('Apple M1 Pro', 'CPU', False)]
get_devices_for_type ONEAPI = []
compute_device_type = 'METAL'
```

Then each name assigned to `compute_device_type` in a fresh Blender of its own, and the device lists read with `get_devices_for_type` (which takes the method as its argument) and from the plain `devices` collection:

```
TYPE NONE ACCEPTED
TYPE NONE DEVICES-FOR-TYPE []
TYPE NONE P-DEVICES [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]
TYPE CUDA COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "CUDA" not found in ('NONE', 'METAL')
TYPE OPTIX COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "OPTIX" not found in ('NONE', 'METAL')
TYPE HIP COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "HIP" not found in ('NONE', 'METAL')
TYPE ONEAPI COULD-NOT-SET TypeError bpy_struct: item.attr = val: enum "ONEAPI" not found in ('NONE', 'METAL')
TYPE METAL ACCEPTED
TYPE METAL DEVICES-FOR-TYPE [('Apple M1 Pro (GPU - 14 cores)', 'METAL', True), ('Apple M1 Pro', 'CPU', False)]
TYPE METAL P-DEVICES [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]
```

On this Mac the preference offers and accepts two values, `NONE` and `METAL`, starts at `METAL`, and refuses the other four by name. With `NONE` chosen, `get_devices_for_type` holds nothing while the plain `devices` collection still lists both of this Mac's devices.

**What the preference object holds, and whether each value can be set with no window (entry 4).** The same run listed the object's properties and their factory values:

```
PROPERTIES ['bl_idname', 'compute_device_type', 'devices', 'kernel_optimization_level', 'metalrt', 'peer_memory', 'use_hiprt', 'use_oneapirt']
FACTORY peer_memory = False
FACTORY metalrt = 'AUTO'
FACTORY use_hiprt = True
FACTORY use_oneapirt = True
FACTORY kernel_optimization_level = 'FULL'
```

Then each of the six settable values was assigned from a script in a background run with no window, and read back:

```
Q7SET compute_device_type: asked='NONE' before='METAL' after='NONE' TOOK
Q7SET metalrt: asked='OFF' before='AUTO' after='OFF' TOOK
Q7SET kernel_optimization_level: asked='INTERSECT' before='FULL' after='INTERSECT' TOOK
Q7SET use_hiprt: asked=False before=True after=False TOOK
Q7SET use_oneapirt: asked=False before=True after=False TOOK
Q7SET peer_memory: asked=True before=False after=True TOOK
```

All six took. Whether a value set this way changes what a render does is shown for MetalRT and Kernel Optimization further down (entries 36 and 37); `use_hiprt`, `use_oneapirt` and `peer_memory` were only set and read back, since nothing on this Mac can use them.

**Where a render is told, and which telling wins (entry 12, written in full under question 3).** On this Mac, settled by which picture came out: `--cycles-device METAL` put a render on Apple's graphics whatever the file's Device and the saved backend said, and `--cycles-device CPU` put it on the processor the same way. With no argument, it took the file saying GPU Compute **and** a saved backend of Metal **with the graphics device ticked** to reach the graphics; a file saying GPU Compute with the backend at None, or with Metal chosen and no device ticked, rendered the processor's picture to the last number. Blender's ordinary output named no device; `--cycles-print-stats` printed `Profiling information not available (only works with CPU rendering)` in the graphics runs only; and with `--log "cycles" --log-level info` on the line, Cycles printed a `Path tracing on:` line naming the device, which matched the picture in all eight crossings.

**Whether the method changes the picture (entry 11, written in full under question 2).** The processor against Apple's graphics, same file: `IDENTICAL-PIXELS False MAX-ABS-DIFF 0.02776789665222168`, where the processor repeats itself exactly and the graphics to `MAX-ABS-DIFF 1.1920928955078125e-06`. Entry 35 below repeats this on three scene sizes.

**What a render does when `--cycles-device` names a method this Mac has no hardware for, or a name that is not a method (entry 13).** The file saying CPU, in a settings folder naming no backend, `-- --cycles-device <name>`:

```
== CUDA EXIT=1
00:00.333  reports          | ERROR Found no Cycles device of the specified type
== OPTIX EXIT=1
00:00.329  reports          | ERROR Found no Cycles device of the specified type
== HIP EXIT=1
00:00.328  reports          | ERROR Found no Cycles device of the specified type
== ONEAPI EXIT=1
00:00.325  reports          | ERROR Found no Cycles device of the specified type
== BANANA EXIT=1
00:00.314  cycles           | ERROR BANANA is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

No image was written by any of the five (the output folder was empty afterwards). Nothing fell back to the processor or to Apple's graphics.

**The other reading: which drawing backends `--gpu-backend` accepts on this Mac, and whether a refused one stops a background render (entry 15).** A small Cycles file rendered with no window and `--gpu-backend <name>`:

```
== metal EXIT=0
00:00.405  render           | Saved: 'renders/e15/metal_0001.png'
== opengl EXIT=0
Error: Unrecognized GPU backend for '--gpu-backend', expected one of [metal].
00:00.421  render           | Saved: 'renders/e15/opengl_0001.png'
== vulkan EXIT=0
Error: Unrecognized GPU backend for '--gpu-backend', expected one of [metal].
00:00.421  render           | Saved: 'renders/e15/vulkan_0001.png'
```

On this Mac `metal` was taken; `opengl` and `vulkan` were refused by name, and each render still finished, saved its frame and exited 0. Since this build takes only one backend, nothing here can say whether the backend changes a final render.

**Three scene sizes: whether the method changes the picture, how far two runs of one method differ, and how the time gap moves with size (entry 35).** Three files built by script, each Cycles, 640 by 360, 64 samples, adaptive sampling off, denoising off, Seed 0 with animated seed off, eight bounces, stamping off, OpenEXR at full float with no compression, Device CPU, a floor and a sun: **small**, one icosphere (322 triangles with the floor); **heavy mesh**, eight icospheres at subdivision 9 (10,485,762 triangles); **heavy curves**, 300,000 hair curves of eight points each. Each rendered from the terminal with `--factory-startup` and `-- --cycles-device CPU` or `-- --cycles-device METAL` (MetalRT at its factory `AUTO`), two renders in a row, five times over: sixty renders, all exit 0. Each same-method pair compared, and the first processor render against the first Metal render:

```
small_cpu_rep1_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
small_metal_rep1_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4901161193847656e-07 NUM-DIFFERING-CHANNEL-VALUES 138767 of 921600 MEAN-ABS-DIFF 4.505744881397631e-09 SIZE (640, 360)
small_cpu_rep2_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
small_metal_rep2_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.1920928955078125e-07 NUM-DIFFERING-CHANNEL-VALUES 132895 of 921600 MEAN-ABS-DIFF 4.282540733887193e-09 SIZE (640, 360)
small_cpu_rep3_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
small_metal_rep3_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.1920928955078125e-07 NUM-DIFFERING-CHANNEL-VALUES 135986 of 921600 MEAN-ABS-DIFF 4.37866826056658e-09 SIZE (640, 360)
small_cpu_rep4_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
small_metal_rep4_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4901161193847656e-07 NUM-DIFFERING-CHANNEL-VALUES 134519 of 921600 MEAN-ABS-DIFF 4.354167483850486e-09 SIZE (640, 360)
small_cpu_rep5_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
small_metal_rep5_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4901161193847656e-07 NUM-DIFFERING-CHANNEL-VALUES 135157 of 921600 MEAN-ABS-DIFF 4.372852041948742e-09 SIZE (640, 360)
small_cpu1a_vs_metal1a: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.0038593411445617676 NUM-DIFFERING-CHANNEL-VALUES 493098 of 921600 MEAN-ABS-DIFF 2.773585664625797e-07 SIZE (640, 360)
heavymesh_cpu_rep1_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
heavymesh_metal_rep1_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 163094 of 921600 MEAN-ABS-DIFF 3.672420360898185e-09 SIZE (640, 360)
heavymesh_cpu_rep2_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
heavymesh_metal_rep2_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 162358 of 921600 MEAN-ABS-DIFF 3.6661387841579403e-09 SIZE (640, 360)
heavymesh_cpu_rep3_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
heavymesh_metal_rep3_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 162909 of 921600 MEAN-ABS-DIFF 3.6534664913132374e-09 SIZE (640, 360)
heavymesh_cpu_rep4_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
heavymesh_metal_rep4_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 164647 of 921600 MEAN-ABS-DIFF 3.7116216036439355e-09 SIZE (640, 360)
heavymesh_cpu_rep5_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
heavymesh_metal_rep5_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 160683 of 921600 MEAN-ABS-DIFF 3.6047509234372733e-09 SIZE (640, 360)
heavymesh_cpu1a_vs_metal1a: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.006285488605499268 NUM-DIFFERING-CHANNEL-VALUES 575873 of 921600 MEAN-ABS-DIFF 1.6505083880878778e-06 SIZE (640, 360)
heavycurves_cpu_rep1_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
heavycurves_metal_rep1_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.009777963161468506 NUM-DIFFERING-CHANNEL-VALUES 304592 of 921600 MEAN-ABS-DIFF 4.6760416138972185e-05 SIZE (640, 360)
heavycurves_cpu_rep2_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
heavycurves_metal_rep2_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 8.940696716308594e-08 NUM-DIFFERING-CHANNEL-VALUES 156503 of 921600 MEAN-ABS-DIFF 1.5167339976566534e-09 SIZE (640, 360)
heavycurves_cpu_rep3_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
heavycurves_metal_rep3_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.1920928955078125e-07 NUM-DIFFERING-CHANNEL-VALUES 156087 of 921600 MEAN-ABS-DIFF 1.5279622150248744e-09 SIZE (640, 360)
heavycurves_cpu_rep4_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
heavycurves_metal_rep4_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.1920928955078125e-07 NUM-DIFFERING-CHANNEL-VALUES 159388 of 921600 MEAN-ABS-DIFF 1.5624906761028494e-09 SIZE (640, 360)
heavycurves_cpu_rep5_a_vs_b: IDENTICAL-PIXELS True MAX-ABS-DIFF 0.0 NUM-DIFFERING-CHANNEL-VALUES 0 of 921600 MEAN-ABS-DIFF 0.0 SIZE (640, 360)
heavycurves_metal_rep5_a_vs_b: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4901161193847656e-07 NUM-DIFFERING-CHANNEL-VALUES 156071 of 921600 MEAN-ABS-DIFF 1.5299954409581713e-09 SIZE (640, 360)
heavycurves_cpu1a_vs_metal1a: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.020525701344013214 NUM-DIFFERING-CHANNEL-VALUES 560198 of 921600 MEAN-ABS-DIFF 0.0004659477568950226 SIZE (640, 360)
```

**The processor: all fifteen pairs identical to the pixel.** **Apple's graphics: fourteen of fifteen pairs between 8.9 × 10⁻⁸ and 2.1 × 10⁻⁷; one — the first pair on the curve scene — at 0.0098.** The processor against Apple's graphics: 0.0039 on the small scene, 0.0063 on the heavy mesh, 0.0205 on the curves. On the small and heavy-mesh scenes that is four to five orders of magnitude above every same-method pair; on the curve scene it is about twice the one excursion and five orders above the other four pairs.

Each render's exit status and Cycles' own last `Time:` line, as the loop printed them:

```
small cpu 1a EXIT=0 Time: 00:02.06
small cpu 1b EXIT=0 Time: 00:02.14
small metal 1a EXIT=0 Time: 00:01.19
small metal 1b EXIT=0 Time: 00:01.17
small cpu 2a EXIT=0 Time: 00:02.70
small cpu 2b EXIT=0 Time: 00:02.18
small metal 2a EXIT=0 Time: 00:01.13
small metal 2b EXIT=0 Time: 00:01.14
small cpu 3a EXIT=0 Time: 00:02.59
small cpu 3b EXIT=0 Time: 00:02.18
small metal 3a EXIT=0 Time: 00:01.11
small metal 3b EXIT=0 Time: 00:01.15
small cpu 4a EXIT=0 Time: 00:02.58
small cpu 4b EXIT=0 Time: 00:02.18
small metal 4a EXIT=0 Time: 00:01.12
small metal 4b EXIT=0 Time: 00:01.13
small cpu 5a EXIT=0 Time: 00:02.60
small cpu 5b EXIT=0 Time: 00:02.16
small metal 5a EXIT=0 Time: 00:01.13
small metal 5b EXIT=0 Time: 00:01.25
heavymesh cpu 1a EXIT=0 Time: 00:08.51
heavymesh cpu 1b EXIT=0 Time: 00:07.79
heavymesh metal 1a EXIT=0 Time: 00:05.85
heavymesh metal 1b EXIT=0 Time: 00:05.82
heavymesh cpu 2a EXIT=0 Time: 00:08.23
heavymesh cpu 2b EXIT=0 Time: 00:07.99
heavymesh metal 2a EXIT=0 Time: 00:05.58
heavymesh metal 2b EXIT=0 Time: 00:05.61
heavymesh cpu 3a EXIT=0 Time: 00:08.91
heavymesh cpu 3b EXIT=0 Time: 00:07.74
heavymesh metal 3a EXIT=0 Time: 00:04.77
heavymesh metal 3b EXIT=0 Time: 00:05.35
heavymesh cpu 4a EXIT=0 Time: 00:07.09
heavymesh cpu 4b EXIT=0 Time: 00:06.70
heavymesh metal 4a EXIT=0 Time: 00:05.38
heavymesh metal 4b EXIT=0 Time: 00:05.63
heavymesh cpu 5a EXIT=0 Time: 00:08.01
heavymesh cpu 5b EXIT=0 Time: 00:06.81
heavymesh metal 5a EXIT=0 Time: 00:04.60
heavymesh metal 5b EXIT=0 Time: 00:05.40
heavycurves cpu 1a EXIT=0 Time: 00:09.50
heavycurves cpu 1b EXIT=0 Time: 00:09.32
heavycurves metal 1a EXIT=0 Time: 00:05.24
heavycurves metal 1b EXIT=0 Time: 00:05.32
heavycurves cpu 2a EXIT=0 Time: 00:10.42
heavycurves cpu 2b EXIT=0 Time: 00:09.00
heavycurves metal 2a EXIT=0 Time: 00:04.82
heavycurves metal 2b EXIT=0 Time: 00:05.18
heavycurves cpu 3a EXIT=0 Time: 00:11.17
heavycurves cpu 3b EXIT=0 Time: 00:10.52
heavycurves metal 3a EXIT=0 Time: 00:04.97
heavycurves metal 3b EXIT=0 Time: 00:05.10
heavycurves cpu 4a EXIT=0 Time: 00:09.57
heavycurves cpu 4b EXIT=0 Time: 00:09.23
heavycurves metal 4a EXIT=0 Time: 00:04.99
heavycurves metal 4b EXIT=0 Time: 00:05.58
heavycurves cpu 5a EXIT=0 Time: 00:11.45
heavycurves cpu 5b EXIT=0 Time: 00:10.40
heavycurves metal 5a EXIT=0 Time: 00:04.75
heavycurves metal 5b EXIT=0 Time: 00:04.97
```

The load average was 2.88 when these began and 18.02 when they ended, with only the Creator's own Blender open beside them, so other work on the Mac rose during the run. On these figures, Apple's graphics took about half the processor's time on the small and the curve scenes, and about two-thirds of it on the heavy mesh; the gap did not simply grow or shrink with the number of triangles.

**MetalRT Off, On and Auto: what each builds, what memory Cycles reports, how long it takes, and whether Auto ever differs from Off (entry 36).** The same three scenes on Apple's graphics (`-- --cycles-device METAL`, `--factory-startup`), MetalRT set by a start-up script that printed what it got — Off five times, Auto five times, On twice, per scene: thirty-six renders, all exit 0, and every one printed the setting it was given (`METALRT-SET OFF`, `METALRT-SET AUTO`, `METALRT-SET ON`).

**What Blender printed while building.** Searched for the build lines, every Off and every Auto run on every scene printed exactly these two and no others:

```
render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Packing BVH nodes
render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Packing BVH triangles and strands
```

and every On run printed `Building TLAS` and `Building mesh BLAS`, with `Building hair BLAS` as well on the curve scene, for example:

```
render           | Fra: 1 | Mem: 1M | Updating Geometry BVH Icosphere.006 10/11 | Building mesh BLAS | 1310720 tris | Icosphere
render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building TLAS      |      11 instances
```

One more run of each setting on the heavy-mesh scene with Cycles' own log switched on (`--log "cycles" --log-level info`, the option this Blender's help lists) printed, for Off and for Auto alike:

```
cycles           | Using BVH2 layout.
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                            |                    Hardware Ray-Tracing: Off
```

and for On:

```
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                            |                    Hardware Ray-Tracing: On
```

**On all three scenes, Auto built and reported the same thing as Off.**

**The memory Cycles reported.** The `Mem:` figure on each render's `Finished` line, which was also the highest it printed in that run:

```
small_OFF1-5  Mem: 1373M (all five)    small_AUTO1-5  Mem: 1373M (all five)    small_ON1-2  Mem: 1375M (both)
heavymesh_OFF1-5  Mem: 1977M (all five)    heavymesh_AUTO1-5  Mem: 1977M (all five)    heavymesh_ON1-2  Mem: 2435M (both)
heavycurves_OFF1-5  Mem: 1718M (all five)    heavycurves_AUTO1-5  Mem: 1718M (all five)    heavycurves_ON1-2  Mem: 1670M (both)
```

On the 322-triangle scene On reported 2 MB more than Off; on the heavy mesh, 458 MB more; on the curve scene, 48 MB less.

**How long each took, Cycles' own `Time:` line, in order.** Small: Off 00:01.17, 00:01.33, 00:01.28, 00:01.30, 00:01.42; Auto 00:01.27, 00:01.27, 00:01.35, 00:01.33, 00:01.47; On **03:58.14**, 00:00.98. Heavy mesh: Off 00:05.32, 00:05.57, 00:05.57, 00:05.08, 00:05.04; Auto 00:05.29, 00:05.09, 00:05.15, 00:05.29, 00:05.39; On 00:03.41, 00:02.84. Heavy curves: Off 00:05.37, 00:05.29, 00:05.34, 00:05.25, 00:05.39; Auto 00:05.44, 00:05.45, 00:05.45, 00:05.70, 00:05.77; On 00:04.81, 00:04.37. The first On render of the sitting — on the small scene — spent almost four minutes before its first sample, printing lines from `Loading render kernels (may take a few minutes the first time)` through `79 / 80 render kernels loaded (may take a few minutes the first time)`; no later On render did. The load average was 7.02 when these began and 16.53 when they ended.

**The pictures.** Every Off render was compared with every Auto render of the same scene — twenty-five comparisons per scene, seventy-five in all — and none was identical to the pixel; the largest and smallest per scene, uncut (all seventy-five lines are kept in the hand-test list):

```
small_OFF5_vs_AUTO3: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 134811 of 921600 MEAN-ABS-DIFF 4.351027706030032e-09 SIZE (640, 360)
small_OFF1_vs_AUTO4: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.1920928955078125e-07 NUM-DIFFERING-CHANNEL-VALUES 136161 of 921600 MEAN-ABS-DIFF 4.408792746188636e-09 SIZE (640, 360)
heavymesh_OFF5_vs_AUTO5: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.384185791015625e-07 NUM-DIFFERING-CHANNEL-VALUES 159946 of 921600 MEAN-ABS-DIFF 3.6012685692791518e-09 SIZE (640, 360)
heavymesh_OFF1_vs_AUTO5: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 162418 of 921600 MEAN-ABS-DIFF 3.658591999232562e-09 SIZE (640, 360)
heavycurves_OFF5_vs_AUTO1: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4901161193847656e-07 NUM-DIFFERING-CHANNEL-VALUES 153875 of 921600 MEAN-ABS-DIFF 1.4933943425098228e-09 SIZE (640, 360)
heavycurves_OFF1_vs_AUTO1: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.1920928955078125e-07 NUM-DIFFERING-CHANNEL-VALUES 154559 of 921600 MEAN-ABS-DIFF 1.5176738088484854e-09 SIZE (640, 360)
```

and the On renders, against each other and against Off, with an Off pair beside them:

```
small_ON1_vs_ON2: IDENTICAL-PIXELS False MAX-ABS-DIFF 3.129243850708008e-07 NUM-DIFFERING-CHANNEL-VALUES 137464 of 921600 MEAN-ABS-DIFF 4.427606654644478e-09 SIZE (640, 360)
small_OFF1_vs_ON1: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.00466609001159668 NUM-DIFFERING-CHANNEL-VALUES 191808 of 921600 MEAN-ABS-DIFF 5.4014070782512944e-08 SIZE (640, 360)
small_OFF1_vs_OFF2: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4901161193847656e-07 NUM-DIFFERING-CHANNEL-VALUES 132520 of 921600 MEAN-ABS-DIFF 4.272031017333373e-09 SIZE (640, 360)
heavymesh_ON1_vs_ON2: IDENTICAL-PIXELS False MAX-ABS-DIFF 4.827976226806641e-06 NUM-DIFFERING-CHANNEL-VALUES 163498 of 921600 MEAN-ABS-DIFF 3.7284007703419775e-09 SIZE (640, 360)
heavymesh_OFF1_vs_ON1: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.00880482792854309 NUM-DIFFERING-CHANNEL-VALUES 257308 of 921600 MEAN-ABS-DIFF 1.2432203387409875e-06 SIZE (640, 360)
heavymesh_OFF1_vs_OFF2: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 162218 of 921600 MEAN-ABS-DIFF 3.6346965417679813e-09 SIZE (640, 360)
heavycurves_ON1_vs_ON2: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.1920928955078125e-07 NUM-DIFFERING-CHANNEL-VALUES 153407 of 921600 MEAN-ABS-DIFF 1.499671371776559e-09 SIZE (640, 360)
heavycurves_OFF1_vs_ON1: IDENTICAL-PIXELS False MAX-ABS-DIFF 0.01476270705461502 NUM-DIFFERING-CHANNEL-VALUES 554575 of 921600 MEAN-ABS-DIFF 0.000449442494991672 SIZE (640, 360)
heavycurves_OFF1_vs_OFF2: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4901161193847656e-07 NUM-DIFFERING-CHANNEL-VALUES 154420 of 921600 MEAN-ABS-DIFF 1.5224769514235151e-09 SIZE (640, 360)
```

Every Off-against-Auto comparison sat between 1.2 × 10⁻⁷ and 2.4 × 10⁻⁷, the same range as two Off renders of the same scene. Off against On differed by 0.0047, 0.0088 and 0.0148 on the three scenes — four to five orders of magnitude above the same-setting pairs.

**Kernel Optimization: time and picture (entry 37).** The heavy-mesh scene on Apple's graphics, Kernel Optimization set by a start-up script to Off, Intersection only and Full, three renders each; each run's exit status, the setting it printed, Cycles' last `Time:` line, and any line about optimizing kernels:

```
OFF1 EXIT=0 KERNELOPT-SET OFF Time: 00:04.80 
OFF2 EXIT=0 KERNELOPT-SET OFF Time: 00:04.72 
OFF3 EXIT=0 KERNELOPT-SET OFF Time: 00:04.65 
INTERSECT1 EXIT=0 KERNELOPT-SET INTERSECT Time: 00:04.60 
INTERSECT2 EXIT=0 KERNELOPT-SET INTERSECT Time: 00:05.17 
INTERSECT3 EXIT=0 KERNELOPT-SET INTERSECT Time: 00:05.32 
FULL1 EXIT=0 KERNELOPT-SET FULL Time: 00:05.39 (1 kernels to optimize) (Using optimized kernels) 
FULL2 EXIT=0 KERNELOPT-SET FULL Time: 00:05.75 (1 kernels to optimize) (Using optimized kernels) 
FULL3 EXIT=0 KERNELOPT-SET FULL Time: 00:05.54 (1 kernels to optimize) (Using optimized kernels) 
```

The load average was 5.88 when these began and 7.36 when they ended. Every pair of the nine pictures compared:

```
OFF1_vs_OFF2: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 171413 of 921600 MEAN-ABS-DIFF 3.881087751425285e-09 SIZE (640, 360)
OFF1_vs_OFF3: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 168845 of 921600 MEAN-ABS-DIFF 3.837232927455463e-09 SIZE (640, 360)
OFF1_vs_INTERSECT1: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 166948 of 921600 MEAN-ABS-DIFF 3.76053726035429e-09 SIZE (640, 360)
OFF1_vs_INTERSECT2: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 165310 of 921600 MEAN-ABS-DIFF 3.727095140170099e-09 SIZE (640, 360)
OFF1_vs_INTERSECT3: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 168741 of 921600 MEAN-ABS-DIFF 3.798398514239428e-09 SIZE (640, 360)
OFF1_vs_FULL1: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 163511 of 921600 MEAN-ABS-DIFF 3.671083403686579e-09 SIZE (640, 360)
OFF1_vs_FULL2: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 166832 of 921600 MEAN-ABS-DIFF 3.790601112996228e-09 SIZE (640, 360)
OFF1_vs_FULL3: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 165038 of 921600 MEAN-ABS-DIFF 3.7062828697445285e-09 SIZE (640, 360)
OFF2_vs_OFF3: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 173472 of 921600 MEAN-ABS-DIFF 3.9263401655868314e-09 SIZE (640, 360)
OFF2_vs_INTERSECT1: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 172494 of 921600 MEAN-ABS-DIFF 3.870124298070247e-09 SIZE (640, 360)
OFF2_vs_INTERSECT2: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.384185791015625e-07 NUM-DIFFERING-CHANNEL-VALUES 169919 of 921600 MEAN-ABS-DIFF 3.826081511862059e-09 SIZE (640, 360)
OFF2_vs_INTERSECT3: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 172980 of 921600 MEAN-ABS-DIFF 3.8824792783189975e-09 SIZE (640, 360)
OFF2_vs_FULL1: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 168392 of 921600 MEAN-ABS-DIFF 3.763962013181299e-09 SIZE (640, 360)
OFF2_vs_FULL2: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 170903 of 921600 MEAN-ABS-DIFF 3.872468773301484e-09 SIZE (640, 360)
OFF2_vs_FULL3: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 169936 of 921600 MEAN-ABS-DIFF 3.805267220337151e-09 SIZE (640, 360)
OFF3_vs_INTERSECT1: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.384185791015625e-07 NUM-DIFFERING-CHANNEL-VALUES 168861 of 921600 MEAN-ABS-DIFF 3.806856814965916e-09 SIZE (640, 360)
OFF3_vs_INTERSECT2: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4901161193847656e-07 NUM-DIFFERING-CHANNEL-VALUES 167046 of 921600 MEAN-ABS-DIFF 3.77923748197241e-09 SIZE (640, 360)
OFF3_vs_INTERSECT3: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 170067 of 921600 MEAN-ABS-DIFF 3.839839135050877e-09 SIZE (640, 360)
OFF3_vs_FULL1: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 164938 of 921600 MEAN-ABS-DIFF 3.7055148519963647e-09 SIZE (640, 360)
OFF3_vs_FULL2: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 167901 of 921600 MEAN-ABS-DIFF 3.827180989901535e-09 SIZE (640, 360)
OFF3_vs_FULL3: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 166758 of 921600 MEAN-ABS-DIFF 3.750156894158054e-09 SIZE (640, 360)
INTERSECT1_vs_INTERSECT2: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 165756 of 921600 MEAN-ABS-DIFF 3.7154061121529794e-09 SIZE (640, 360)
INTERSECT1_vs_INTERSECT3: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4901161193847656e-07 NUM-DIFFERING-CHANNEL-VALUES 168465 of 921600 MEAN-ABS-DIFF 3.765651652227259e-09 SIZE (640, 360)
INTERSECT1_vs_FULL1: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 163423 of 921600 MEAN-ABS-DIFF 3.6515444258434903e-09 SIZE (640, 360)
INTERSECT1_vs_FULL2: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.682209014892578e-07 NUM-DIFFERING-CHANNEL-VALUES 166468 of 921600 MEAN-ABS-DIFF 3.771989819748948e-09 SIZE (640, 360)
INTERSECT1_vs_FULL3: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 166106 of 921600 MEAN-ABS-DIFF 3.7079088441613646e-09 SIZE (640, 360)
INTERSECT2_vs_INTERSECT3: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 166650 of 921600 MEAN-ABS-DIFF 3.731413218904183e-09 SIZE (640, 360)
INTERSECT2_vs_FULL1: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 161666 of 921600 MEAN-ABS-DIFF 3.610355431899532e-09 SIZE (640, 360)
INTERSECT2_vs_FULL2: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 164976 of 921600 MEAN-ABS-DIFF 3.732583435420464e-09 SIZE (640, 360)
INTERSECT2_vs_FULL3: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.4901161193847656e-07 NUM-DIFFERING-CHANNEL-VALUES 164307 of 921600 MEAN-ABS-DIFF 3.6740766518050805e-09 SIZE (640, 360)
INTERSECT3_vs_FULL1: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 164480 of 921600 MEAN-ABS-DIFF 3.6597581735501687e-09 SIZE (640, 360)
INTERSECT3_vs_FULL2: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 167667 of 921600 MEAN-ABS-DIFF 3.78553724860669e-09 SIZE (640, 360)
INTERSECT3_vs_FULL3: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.086162567138672e-07 NUM-DIFFERING-CHANNEL-VALUES 166582 of 921600 MEAN-ABS-DIFF 3.72911017620936e-09 SIZE (640, 360)
FULL1_vs_FULL2: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 162612 of 921600 MEAN-ABS-DIFF 3.6618722434569565e-09 SIZE (640, 360)
FULL1_vs_FULL3: IDENTICAL-PIXELS False MAX-ABS-DIFF 1.7881393432617188e-07 NUM-DIFFERING-CHANNEL-VALUES 161469 of 921600 MEAN-ABS-DIFF 3.5970252712205465e-09 SIZE (640, 360)
FULL2_vs_FULL3: IDENTICAL-PIXELS False MAX-ABS-DIFF 2.384185791015625e-07 NUM-DIFFERING-CHANNEL-VALUES 164836 of 921600 MEAN-ABS-DIFF 3.7183296323443454e-09 SIZE (640, 360)
```

On this scene, every pair — same setting or different — sat between 1.5 × 10⁻⁷ and 2.7 × 10⁻⁷, the range two runs of one method fall in under entry 35. Only the Full runs printed `(1 kernels to optimize)` and `(Using optimized kernels)`. By Cycles' own count, the three Full renders were the slowest here (00:05.39 to 00:05.75) and the Off renders among the fastest (00:04.65 to 00:04.80).

**Whether the Acceleration Structure panel would be drawn, asked with no window (entry 38).** The panel class asked directly for every crossing this Mac can make of the backend preference, the scene's Device, and whether the processor is ticked alongside the graphics:

```
PANEL-CLASS <class 'cycles.ui.CYCLES_RENDER_PT_performance_acceleration_structure'>
pref=NONE  scene_device=CPU cpu_device_ticked=False -> Acceleration Structure panel poll = True
pref=NONE  scene_device=CPU cpu_device_ticked=True  -> Acceleration Structure panel poll = True
pref=NONE  scene_device=GPU cpu_device_ticked=False -> Acceleration Structure panel poll = True
pref=NONE  scene_device=GPU cpu_device_ticked=True  -> Acceleration Structure panel poll = True
pref=METAL scene_device=CPU cpu_device_ticked=False -> Acceleration Structure panel poll = True
pref=METAL scene_device=CPU cpu_device_ticked=True  -> Acceleration Structure panel poll = True
pref=METAL scene_device=GPU cpu_device_ticked=False -> Acceleration Structure panel poll = True
pref=METAL scene_device=GPU cpu_device_ticked=True  -> Acceleration Structure panel poll = True
```

Under `NONE` and `METAL` the panel answers that it would be drawn in every crossing. OptiX cannot be chosen on this Mac, so the one crossing the manual and the panel's own test single out is not asked here. Which controls it then shows is entry 41, below.

**Which Acceleration Structure controls appear for each device and backend (entry 41).** Blender's window, opened from the terminal with factory settings and a settings folder of the test's own, Cycles as the engine, the Performance panels made to start open (as in question 4's entry 39), the Properties editor maximised on the Render tab; a script moved the scene's Device and the backend preference through CPU with None, CPU with Metal (graphics ticked, processor not), GPU Compute with None, and GPU Compute with Metal, taking Blender's own screenshot after each. As in entry 39, the pictures lag the script's steps, so they are identified by what they show; the four CPU pictures, with None and with Metal, came out byte-for-byte the same file. What the three distinct pictures show at the top of the tab and under Acceleration Structure:

- **Device CPU (with the backend at None and at Metal alike)** — the Device field reads CPU and is drawn normally, with Open Shading Language under it; Acceleration Structure holds **Use Spatial Splits** and **Use Compact BVH**.
- **Device GPU Compute, backend None** — the Device field reads GPU Compute and is **drawn dimmed**, with Open Shading Language under it; Acceleration Structure holds **Use Spatial Splits** and **Use Compact BVH**.
- **Device GPU Compute, backend Metal** — the Device field reads GPU Compute and is drawn normally, with **no Open Shading Language** tick under it; Acceleration Structure holds **Use Spatial Splits**, **BVH Time Steps** (0) and **Use Curves BVH** (ticked).

The script's own question to the panel after each change printed `POLL acceleration structure True` four times. On this Mac the controls change with the device: Use Compact BVH is offered for the processor, BVH Time Steps and Use Curves BVH for Metal; and a file set to GPU Compute with no backend named keeps the processor's controls and dims its Device field — the same case entry 12 found rendering on the processor.

**What Preferences ▸ System shows for Cycles on this Mac (entry 42).** Blender's window opened from the terminal with factory settings and a settings folder of the test's own; a script opened the Preferences window on its System section (583 by 382 points), set the backend to Metal and then to None, and took Blender's own screenshot of that window after each. Read off the two pictures, from the top of the section down:

- **Metal chosen** — *Cycles Render Devices*: a row of two buttons, **None** and **Metal**, Metal highlighted; below it two tick-boxes, **Apple M1 Pro (GPU - 14 cores)**, ticked, and **Apple M1 Pro**, not ticked; then **Kernel Optimization: Full** and **MetalRT: Auto**, both drawn like the rows around them, neither dimmed. Then *Network*, then *Memory & Limits*, then *Video Sequencer*.
- **None chosen** — *Cycles Render Devices*: the same two buttons, None highlighted, and nothing else in the panel — no device tick-boxes, no Kernel Optimization, no MetalRT. Then *Network*, *Memory & Limits*, *Video Sequencer*, and the start of *Sound*.

No device carried an "(Unoptimized Performance)" label. No drawing-backend row appears anywhere in the part of the section these pictures show. The script printed, for each state, `SET backend METAL devices [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)] metalrt AUTO kernel_optimization_level FULL` and the same with `NONE`.

---

## Tests for a running Blender

*(Pruned 2026-10-17 by the hand tests run on the Creator's Mac that day. The one list, with the full procedure for each test, is `Audits/Blender Research/What a render demands/Hand tests — 5.3.md`; every result is written above under the question it answers. Only what is genuinely still open is left here, each with the reason it was not settled.)*

**Run and answered above:** 1 (thread count and tile size and the pixels — question 2, entry 7); the Apple-graphics half of 2 (question 2, entry 11; question 7, entry 35); 3 (`-t` and the scene's Threads setting — question 3, entry 9; question 1, entry 8); 4 (the Simplify switch and the texture limit — question 6, entry 33); 5 (untouched Cycles values, and `scene.cycles` under factory startup — question 6, entry 32; question 5, entry 28); 6 (what MetalRT Auto builds on this Mac — question 7, entry 36).

**Still open:**

1. **Whether the processor and an NVIDIA card (CUDA, OptiX) give the same pixels, and how CUDA and OptiX compare as the scene grows.** Needs an NVIDIA card; none was available to the 5.3 hand tests, which had the Creator's Mac alone.
2. **Whether oneAPI falls back to system memory, and what any card does when its own memory fills.** Needs an Intel Arc card, or a card with memory of its own; the Mac's graphics share its memory, and pressing the Mac's 16 GB was not allowed.
3. **What MetalRT Auto picks on Apple hardware with hardware ray tracing, or on other kinds of scene.** On the M1 Pro it built as Off on all three scenes tried; no other Apple machine was available.
4. **Whether denoising on the graphics changes the picture (A5).** On the Mac every final render logged `Denoising on: Apple M1 Pro (CPU)` whether Use GPU was on or off (question 2, entry 18), so no graphics-denoised picture existed to compare.
5. **Whether the texture cache lowers peak memory on a scene whose textures press on memory.** Not run, for the reason in item 2.
6. **Anything on Linux or Windows**: `--qos`, EEVEE on headless Windows, which drawing backends those builds take and whether the backend changes an EEVEE final render, EEVEE and Workbench with no display server, and reading on one system a file written on another. No machine running either system was available.
7. **HIP and HIP RT, Embree on GPU, Distribute Memory Across Devices, and two cards of different ray-tracing generations in one machine.** No such hardware was available.
8. **Reading a file's settings without running Blender.** Needs a program other than Blender; the hand tests could run Blender and nothing else.
9. **What 5.3 reads out of a file saved by Blender 4.5 with Use Tiling off.** Needs a file written by Blender 4.5; no 4.5 was among the hand tests' Blenders.

---

2026-10-16 — Claude Opus 5.5 (Claude Code) — What a render demands, Blender 5.3

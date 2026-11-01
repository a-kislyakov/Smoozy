# What a render demands — Blender 5.2

**Sources consulted, and when.** Blender's manual for 5.2, Blender's Python API documentation for 5.2, Blender's 5.2 release notes, and Cycles' own files from 5.2 — all read on **2026-07-30** (the snapshot date given to me). Blender's requirements page, read on **2026-08-01**.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

**One note on the requirements page before anything else.** It never names Blender 5.2. Under "Requirements for Older Versions" it names 4.5, 3.6, 3.4, 2.92, 2.90, 2.83, 2.79 and 2.76, and one line reads: "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." That phrase covers 5.2 by arithmetic but does not name it in so many words, so I flag it wherever I lean on it rather than treating it as a plain statement about 5.2. Source: <https://www.blender.org/download/requirements/>, read 2026-08-01.

---

## 1. What settings, if any, change how much a render demands of the computer it runs on?

**Label: documented** (with the parts marked **observed** taken from Cycles' own files).

There are many. I drew the line at settings whose own documentation names an effect on **which hardware does the work**, **how many processor cores are used**, or **how much memory is used** — that is, the demand a render makes rather than the picture it makes. A fifth group at the end is the large family of quality settings whose documented cost is time; I list it briefly because "how hard it works the computer" plainly covers it, but I have not tried to enumerate it exhaustively.

Blender itself gathers most of the first four groups into one panel. The manual says so:

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

Manual, Cycles Performance panel — <https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html>, read 2026-07-30.

Cycles' own files say exactly which settings those presets carry, which is as close to an authoritative list of "the demand settings" as the sources get:

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

**Observed** in `Blender Sources/cycles_5.2/presets.py`, class `AddPresetPerformance`, lines 102–110, read 2026-07-30.

### Group A — which hardware does the work

| Setting | Where it lives | What the source says |
|---|---|---|
| **Device** (`scene.cycles.device`, CPU / GPU Compute) | Cycles scene setting | **Observed**, `cycles_5.2/properties.py`, class `CyclesRenderSettings`, line 387: `device: EnumProperty(name="Device", description="Device to use for rendering", items=enum_devices, default='CPU', …)`; and lines 30–34: `('CPU', "CPU", "Use CPU for rendering")`, `('GPU', "GPU Compute", "Use GPU compute device for rendering, configured in the system tab in the user preferences")` |
| **Cycles Render Devices** — compute device type, and which individual devices are ticked | User preference, not the scene | Manual: "Changes the Computing Device the Cycles render engine uses to render images. Cycles can use either the CPU or certain GPUs to render images". <https://docs.blender.org/manual/en/5.2/editors/preferences/system.html> |
| **Distribute Memory Across Devices** (`peer_memory`) | Cycles preference | Manual: "Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes." Same page. **Observed**, `properties.py` line 1664: `description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it"` |
| **MetalRT**, **HIP RT**, **Embree on GPU** | Cycles preferences | Manual: "MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases." / "HIP RT … Speeds up rendering by enabling AMD hardware ray tracing". / "Embree on GPU … Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance." Same page |
| **Kernel Optimization** | Cycles preference | **Observed**, `properties.py` line 1694: "Optimized kernels are requested at the start of a render… This can result in additional CPU usage for a brief time (tens of seconds)", with options "Off … Slowest rendering, no extra background CPU usage", "Intersection only … negligible extra background CPU usage", "Full … Fastest rendering, may result in extra background CPU usage" |
| **Compositor Device** (`compositor_device`) and **Denoising Device** for Denoise nodes | Scene render settings | Manual: "The device used for compositing. CPU: Use the CPU for compositing. GPU: Use the GPU for compositing." and "The device to use to process Denoise nodes in the compositor." Cycles and EEVEE Performance pages |
| **Use GPU** for Cycles denoising | Cycles sampling | Manual: "Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled." <https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html> |
| **Render engine** (`scene.render.engine`) | Scene render settings | API: "Engine to use for rendering (default 'BLENDER_EEVEE')". <https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html>. The manual states EEVEE "only uses the power of the GPU to render. There is no plan to support CPU (software) rendering", so the choice of engine decides which hardware is used at all. <https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html> |

### Group B — how much of the processor is used

| Setting | What the source says |
|---|---|
| **Threads Mode** (`render.threads_mode`) | Manual: "Method to determine the maximum number of CPU cores to use while rendering. Auto-Detect: Automatically chooses the amount of threads to match the number of logical processors on your computer. Fixed: Manually choose the maximum number threads to use for rendering. This can be useful for example, if you want to use your computer while rendering, you can set the property to a thread count lower the amount of logical processors on your computer." Cycles Performance page |
| **Threads** (`render.threads`) | Manual: "The maximum number of CPU cores to use simultaneously while rendering." Same page. API: "Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems) (in [1, 1024], default 1)" |
| **`-t, --threads` on the command line** | Manual: "Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count." <https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html> |
| **`--qos <level>`** | Manual: "Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only). default: Uses the default behavior of the OS. high: Always makes use of performance cores. eco: Schedules Blender threads exclusively to efficiency cores." Same page |
| **Shader Compilation Method / Threads / Subprocesses** (preference; `--gpu-compilation-subprocesses` on the command line) | Manual: "Subprocess: Uses multiple separate subprocesses to compile shaders in parallel. This can be faster, especially on systems with high core counts, but consumes significantly more RAM." and "Increasing the number can reduce shader compilation time at the cost of higher memory usage. A value of 0 lets Blender automatically choose a suitable number based on the system configuration." <https://docs.blender.org/manual/en/5.2/editors/preferences/system.html> |
| **`--debug-depsgraph-no-threads`** | Manual: "Switch dependency graph to a single threaded evaluation." Command line arguments page |

### Group C — how much memory is used (Cycles)

| Setting | What the source says |
|---|---|
| **Tile Size** (`cycles.tile_size`) | Manual: "Render high resolution images in tiles of this size, to reduce memory usage of render passes. Tiles are cached to disk while rendering to save memory. Lowering the tile size also reduces texture cache memory usage, as only the image textures needed by each tile will be loaded into memory. Tile size has an impact on performance, and the default 2048 pixels provides a good balance for most scenes. When running out of memory, lowering to 1024 or 512 pixels helps reduce memory usage, typically at a small performance cost." Cycles Performance page. **Observed**, `properties.py` line 1084: `tile_size: IntProperty(name="Tile Size", default=2048, … min=8, max=8192)` |
| **Texture Cache** (`render.use_texture_cache`) | Manual: "Reduce memory usage and startup time on scenes with many image textures, at the cost of a small rendering performance impact and increased disk space usage." Cycles Performance page. API: "Load texture tiles at appropriate resolution on demand to reduce memory usage. This avoids loading all textures into memory, at the cost of extra disk space and some performance (default True)" |
| **Auto Generate** texture cache (`render.use_auto_generate_texture_cache`) | API: "Automatically create tx files from image files when rendering, if the files do not exist or are outdated. The path to store the texture cache files is configured in the preferences (default False)" |
| **Use Spatial Splits** | Manual: "Spatial splits improve the rendering performance in scenes with a mix of large and small polygons. The downsides are longer BVH build times and slightly increased memory usage." **Observed**, `properties.py` line 932: `description="Use BVH spatial splits: longer builder time, faster render", default=False` |
| **Use Curves BVH** | Manual: "Use a special type of BVH for rendering curves… Disabling this option will reduce memory, at the cost of increasing curve render time." **Observed**, `properties.py` line 937: `description="Use special type BVH optimized for curves (uses more ram but renders faster)", default=True` |
| **BVH Time Steps** | Manual: "Split BVH primitives by this number of time steps to speed up render time at the expense of memory." **Observed**, `properties.py` line 947: `default=0, min=0, max=16` |
| **Use Compact BVH** | Manual: "Use a more compact BVH structure, which can reduce RAM usage but render slower." **Observed**, `properties.py` line 942: `description="Use compact BVH structure (uses less ram but renders slower)", default=False` |
| **Persistent Data** (`render.use_persistent_data`) | Manual: "Keep render data in memory after rendering for faster re-renders and animation renders at the cost of extra memory usage while performing other tasks in Blender." Cycles Performance page |
| **Simplify → Max Subdivision, Child Particles, Texture Resolution, Texture Size Limit, Camera Cull, Distance Cull** | Manual: "Texture Resolution — Scale down image textures to reduce memory usage." and "Texture Size Limit — Automatically scales textures down so that they are no larger than the values chosen. This can help reduce computer memory resources when rendering large scenes with huge textures." <https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/simplify.html> |
| **Offscreen Scale** (adaptive subdivision) | Manual: "Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory." <https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/subdivision.html> |
| **Lock Interface** (`render.use_lock_interface`) | Manual: "Lock interface during rendering in favor of giving more memory to the renderer." <https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html> |
| **Denoising Data passes** | Manual: "These passes increase memory usage and render time. Enable them only when denoising in the Compositor is required." <https://docs.blender.org/manual/en/5.2/render/layers/passes.html> |
| **Color Depth** of the output format | Manual: "Yet a higher bit depth will increase memory usage exponentially." <https://docs.blender.org/manual/en/5.2/render/output/properties/output.html> *Address corrected by the citation check, 2026-08-07: cited to the Format page, which does not carry these words; they are on the Output page.* |

### Group D — how much memory is used (EEVEE)

| Setting | What the source says |
|---|---|
| **Shadow Pool** (`eevee.shadow_pool_size`) | Manual: "A bigger pool size allows for more shadows in the scene but might not fit into GPU memory and decreases performance. Increasing the size might fix the Shadow buffer full error." <https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/performance.html> |
| **Light Probes Volume Pool** (`eevee.gi_irradiance_pool_size`) | Manual: "A bigger pool size allows for more irradiance grids in the scene but might not fit into GPU memory and decreases performance." Same page |
| **High Quality Normals** (`render.use_high_quality_normals`) | Manual: "Uses higher precision normals and tangents which can improve visual quality for dense meshes with high frequency textures at the cost of memory." Same page |
| **Anisotropic Filtering** (`render.anisotropic_filter`) | Manual: "Increasing the setting may have a small impact on performance." Same page. *The quoted words name performance, not memory. It sits in this group only because Blender puts it on the EEVEE Performance page; nothing quoted here ties it to memory.* |
| **Volumes → Resolution and Steps** | Manual: "To achieve this, EEVEE uses several 3D textures which have a high video memory usage. The texture dimensions can be tweaked using the Resolution and Steps parameters." <https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/volumes.html>. API, `volumetric_samples`: "Number of steps to compute volumetric effects. Higher step count increase VRAM usage and quality. (in [1, 256], default 64)" |
| **Volume Max Depth** (`volumetric_ray_depth`) | API: "Maximum surface intersection count used by the accurate volume intersection method… Higher count increases VRAM usage. (in [1, 16], default 16)" <https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html> |
| **Raytracing Resolution**, **Fast GI Resolution** | Manual: "Resolution at which the ray-tracing is performed. Lower options will be faster and use less memory but will produce blurrier results." <https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/raytracing.html> |

### Group E — settings whose documented cost is time rather than hardware

These change how long the computer works, not what it must have. I am not listing them exhaustively; a representative set, all from the Cycles Sampling page: **Render (Max) Samples** ("A higher number results in a cleaner image at the cost of a longer render time"), **Time Limit** ("Stops rendering if the time exceeds the limit"), **Noise Threshold** for adaptive sampling ("lower values meaning better quality but longer render times"), **Light Tree** ("at the cost of a somewhat longer render time per sample"), **Min Light Bounces** ("may also increase render time considerably"), **Denoising quality** ("Produces the highest quality at the cost of time"), **Scrambling Distance** ("in an attempt to improve GPU rendering performance"), and **Sample Subset** ("A typical use case is to distribute the rendering of a single frame over multiple machines"). Also **Resolution Percentage** on the output format page, and the Cycles Volumes **Step Rate Render** ("Increase to reduce render time, at the cost of losing detail").

**Where I looked.** I searched the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` for: "memory usage", "memory consumption", "GPU memory", "VRAM", "CPU cores", "logical processors", "threads", "performance", "tile", "texture cache", "persistent", "device", "core". I read in full: the Cycles, EEVEE and Workbench Performance pages; the Cycles Simplify, Sampling, Subdivision and Volumes pages; the EEVEE Sampling, Raytracing, Volumes and Limitations pages; the GPU Rendering page; the Preferences → System page; the Command Line Arguments and Rendering From The Command Line pages; the output Format page; the passes page. In the API: `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE`, `bpy.types.PreferencesSystem`, `bpy.types.PreferencesFilePaths`. In Cycles' files: `properties.py` (all 2063 lines scanned by search, the relevant blocks read), `ui.py` performance panels, `presets.py`, `engine.py`, `__init__.py`, `maketx.py`. I also searched all three documentation sets for "process priority", "affinity", "nice level", "RAM limit", "cap memory", "memory budget", "hyperthread", "NUMA" and found nothing — there is no setting that caps a render's total memory or sets its operating-system priority, other than `--qos` on Windows.

**Not covered.** This does not settle how large any of these effects are on a given scene, nor how they interact. It does not settle the boundary case of Group E — I chose "the documentation names processor, memory or hardware" as the line, and a wider reading of the question would pull in most of the render settings in Blender. It also says nothing about disk or network demand beyond the texture cache's "increased disk space usage".

**Outside the interface.** Almost all of these are scene or preference values that a background render reads the same way; the manual's own command-line page shows `-t 2` ("Use only two threads") changing the demand of a background animation render. Three qualifications the sources do state. First, several of the settings above are labelled Viewport — Pixel Size, viewport samples, viewport texture limit and resolution, Viewport BVH Type — and a background render performs no viewport rendering, so they do not apply; that last step is **inferred** from their names and descriptions, the sources do not spell it out. Second, Lock Interface is about the interface being locked while rendering, which has no meaning with no interface open; the sources do not say what it does in background mode. Third, the manual explicitly flags one render setting as interface-only, which shows Blender does make this distinction: `use_single_layer` — "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line."

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

**Operating system.** The memory and threading settings are not marked as platform-specific anywhere I read. The hardware ones are. Manual, GPU Rendering page: "CUDA is supported on Windows and Linux"; "OptiX is supported on Windows and Linux"; "HIP is supported on Windows and Linux"; "oneAPI is a computation library that is supported on Windows and Linux"; "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." `--qos` is documented "(Windows only)". Shader Compilation Method: "This option is not available on macOS and requires using OpenGL backend." EEVEE: "Headless rendering is not supported on headless Windows systems."

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by the three tests the citation check sent back to the hand tests — 33, 34 and 35 — all three run on the rented Linux machine and none of them on the Mac, whose halves are still owed. They move this line in one place: which Cycles backends a build will even accept is an operating-system fact, five names taken here against two on the Mac, and it is now settled by asking a running Blender on both. Everything else they found came out the same in all four versions. That machine was idle when the work began and was not for a minute afterwards — four other topics' Blenders appeared on it and the load average reached 27.80 — so nothing in those three tests rests on a time.*

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

**Label: documented for some, not in the sources for others.** I read "what the render produces" as the image or image sequence that comes out.

**The sources name no change to the picture — the trade they describe is time or memory only.** For Threads Mode, Threads, `-t`, `--qos`, Tile Size, Persistent Data, Use Spatial Splits, Use Curves BVH, Use Compact BVH, BVH Time Steps, Distribute Memory Across Devices, and Kernel Optimization, every description I found frames the trade purely as speed against memory. Examples, quoted above in full: "longer builder time, faster render"; "uses less ram but renders slower"; "to speed up render time at the expense of memory"; "Keep render data in memory after rendering for faster re-renders"; "Slowest rendering, no extra background CPU usage". **None of these say the image is identical, and I could find no statement anywhere in the manual or API documentation that a render is reproducible across thread counts, tile sizes or acceleration structures** — I searched all three documentation sets for "deterministic", "identical result", "same result", "slight difference", "differ between" and "noise pattern" and found nothing on the subject. So: the documented cost is not to the picture, and whether the picture is bit-for-bit the same is **not in the sources**.

**They say the picture changes, and how.**

- **Simplify → Texture Size Limit**: "Automatically scales textures down so that they are no larger than the values chosen." **Texture Resolution**: "Scale down image textures to reduce memory usage."
- **Simplify → Max Subdivision**: "Maximum number of subdivision by the Subdivision Surface modifiers." **Child Particles**: "Show only a subset of all child hairs and particles."
- **Simplify → Camera Cull / Distance Cull**: "Automatically culls objects based on the camera frustum defined by the Margin." / "Automatically culls objects based on their distance from the active camera."
- **Offscreen Scale**: "Lower values provide higher quality reflections and shadows for off screen objects, while higher values use less memory."
- **EEVEE Raytracing Resolution** and **Fast GI Resolution**: "Lower options will be faster and use less memory but will produce blurrier results."
- **EEVEE Shadow Pool**: "Increasing the size might fix the Shadow buffer full error" — so too small a pool produces an error condition rather than a correct image.
- **EEVEE High Quality Normals**: "Uses higher precision normals and tangents which can improve visual quality for dense meshes with high frequency textures at the cost of memory."
- **EEVEE Anisotropic Filtering**: "Higher values improve the sharpness of textures viewed at a grazing angle, such as floors, roads, and other surfaces that extend into the distance."
- **EEVEE Volumes Resolution / Steps**: "Number of steps to compute volumetric effects. Higher count increases video memory usage and quality." <https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/volumes.html> *Corrected by the citation check, 2026-08-07: this line used to read "Higher count increases VRAM usage and quality", which is in none of the sources — it welded together the API's `volumetric_samples` ("Higher step count increase VRAM usage and quality") and its `volumetric_ray_depth` ("Higher count increases VRAM usage"), which are two different settings. The manual's own sentence for this setting is quoted instead. Both API sentences are quoted correctly in question 1's Group D table.*
- **Compositor Precision**: "Auto: Use full precision for final renders, half precision otherwise." Denoise node **Final Quality**: "High: Produces the highest quality output at the cost of long processing times… Fast: Produces an output quickly at a noticeable cost of quality."
- **Color Depth**: "A higher bit depth will allow more possible colors, reducing banding, and increasing precision." <https://docs.blender.org/manual/en/5.2/render/output/properties/output.html> *Address corrected by the citation check, 2026-08-07: this was cited to the Format page, which does not carry these words. They are on the Output page.*
- **Group E generally**: samples, noise threshold, time limit and denoiser settings all change noise or detail, by their own descriptions.

**Texture Cache is the one with a documented side effect on output.** The release note says: "The texture cache is also used for Open Shading Language, which previously used OpenImageIO. This means improved performance and GPU support. However some more advanced options for `texture()` and `gettextureinfo()` are no longer available." <https://developer.blender.org/docs/release_notes/5.2/cycles/>, read 2026-07-30. That is a change in what OSL shaders can do, which can change what they render. For non-OSL scenes the manual frames the cost as "a small rendering performance impact and increased disk space usage" and does not mention the image.

**Device (CPU versus GPU) — partly documented, and the rest not in the sources.** The manual does carry one such statement, on the Cycles Curves page, under Linear 3D Curves: "This is an optimization to take advantage of hardware accelerated curve intersection on GPUs. Results will not look exactly the same on different CPU and GPU devices, so this is not suitable for multi device rendering or render farms with mixed hardware." — Blender manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/hair.html, read 2026-07-30. It is narrow — it is about one curve shape option, not about rendering in general — but it is a statement, in these sources, that the same scene does not come out the same on the two devices, and it names render farms with mixed hardware in so many words. *Corrected by the citation check, 2026-08-07: this entry used to read "not in the sources — I found no statement about whether a scene rendered on CPU and on GPU produces the same image". That sentence was in the 5.2 snapshot the whole time; the 5.0 findings and the 5.1 second-pass findings both quote it.* Beyond that one option, I found no statement about whether a scene rendered on CPU and on GPU produces the same image. The GPU Rendering page discusses speed, memory and failure modes, and lists two feature differences: "Path Guiding is not supported on any GPU" and "Open Shading Language is only supported for OptiX, with some limitations listed in the documentation." Those two are documented output-relevant differences. Beyond them, nothing.

**Render engine — the picture is different by definition,** and the sources name concrete consequences: EEVEE's "render output subsequently also only has the precision of half-precision (16-bit) floating point numbers" and "The Combined and Light data passes clamp negative output values to zero." <https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html>

**Where I looked.** Every page listed under question 1, read for output claims specifically, plus the searches for determinism language named above. I also read the 5.2 release notes for Cycles, EEVEE, rendering, compositor and Python API in full or in the relevant part.

**Not covered.** Where the sources are silent I have said so rather than reasoned it out; silence here is not evidence that the image is unchanged. Nothing here settles how much the picture changes, only whether the documentation says it does.

**Named in question 1 and never reached here.** The question asks for each of them, and this answer does not cover all of them. These are named in question 1 and say nothing about output anywhere above: MetalRT, HIP RT and Embree on GPU; the Cycles Render Devices preference and the individual device tick boxes; Shader Compilation Method, Shader Compilation Threads and Subprocesses (and `--gpu-compilation-subprocesses`); `--debug-depsgraph-no-threads`; Lock Interface; the Denoising Data passes; Compositor Device itself, as against Compositor Precision; Auto Generate for the texture cache; and Use GPU for Cycles denoising. Whether the sources say anything about what any of these does to the image is unestablished, and no search for it is recorded. Sent back for research.

**The other reading of the question.** I read "what the render produces" as the image or image sequence. It can also be read as everything a render writes to disk — the image files themselves, their names and count, and side files such as the texture cache. That reading is not answered here.

**Outside the interface.** The output-changing settings above are scene values, and I found no statement that any of them behave differently in a background render. The viewport-labelled variants (viewport texture limit, viewport texture resolution, viewport samples, Viewport BVH Type, Pixel Size) do not touch the final image at all, by their own descriptions.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

**Operating system.** Nothing in the descriptions of what these settings change to the image is marked platform-specific. The platform limits are on availability of devices and of headless EEVEE, quoted in question 1.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand test 33, the rebuild of hand test 7, run on the rented Linux machine in all four versions and settled by which picture came out rather than by what Blender printed. It adds one thing to this line and takes nothing away: a file whose Device says GPU Compute, in a settings folder naming no backend, produces the **processor's** picture to the very last number and misses the card's by the whole device gap. What that says about macOS is untested rather than answered — the Mac half is still owed, and it is sharper there, because the graphics on that machine work perfectly.*

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

**Label: documented, with one inferred step that I mark.**

### Settable directly on the command line

All quotes from the Command Line Arguments page, <https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html>, read 2026-07-30, unless noted.

| Argument | What the source says |
|---|---|
| `-t, --threads <threads>` | "Use amount of <threads> for rendering and other operations [1-1024], 0 to use the systems processor count." |
| `-E, --engine <engine>` | "Specify the render engine. Use -E help to list available engines." |
| `--cycles-device <device>` | "Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. Append +CPU to a GPU device to render on both CPU and GPU." Must follow a double dash: "Cycles add-on options must be specified following a double dash." Example given: `blender -b file.blend -f 20 -- --cycles-device OPTIX` |
| `--qos <level>` | "Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only)." |
| `--gpu-backend` | "Force to use a specific GPU backend. Valid options: vulkan, metal, opengl." |
| `--gpu-compilation-subprocesses` | "Override the Max Compilation Subprocesses setting (OpenGL only)." |
| `--debug-depsgraph-no-threads` | "Switch dependency graph to a single threaded evaluation." |
| `--cycles-print-stats` | "Log statistics about render memory and time usage." (a report, not a demand setting, but it is how demand is measured from outside) |
| `-b, --background` | "Run in background (often used for UI-less rendering). The audio device is disabled in background-mode by default" |
| `--disable-depsgraph-on-file-load` | "Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode" |

`--cycles-device` deserves a note, because Cycles' own files state what it overrides more plainly than the manual does. **Observed** in `Blender Sources/cycles_5.2/engine.py`, lines 16–20:

> ```
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)
> ```

and lines 38–41 show what the add-on does with it: `if not _cycles.set_device_override(args.cycles_device): sys.exit(1)`. That it is applied as an override and **not** written into the scene is **inferred** — the help string says "overriding user preferences and the scene setting", and a reader must supply the step that a call named this way leaves the stored value alone. The quoted lines do not show what becomes of `scene.cycles.device`. Hand test 2 below is aimed at exactly this.

There is also a command that prepares a render's demand ahead of time rather than at the moment of the render — generating the texture cache. Manual: "blender scene.blend --command maketx" and "blender --command maketx image.png --colorspace sRGB".

### Settable from outside, but only by running a script at start-up

Everything else in question 1 that lives in the scene can be set this way. The mechanism is documented: `--python-expr <expression>` — "Run the given expression as a Python script. The expression may be a complete multi-line script; you are limited only by the platform's maximum argument length." Also `-P, --python <filepath>` — "Run the given Python script file." Order matters: "Arguments are executed in the order they are given" and the page works through three examples showing a value being overwritten by a later file load.

That any of these settings can be reached that way is **inferred**: the sources document the argument, and separately document each setting as a readable and writable property on `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE` or (in Cycles' own files) `bpy.types.Scene.cycles`. The step a reader must supply is that a Python expression run before the render can assign those properties. I did not find a page that says this in so many words.

### Not settable from outside at render time

- **Threads, the command-line way round.** The API documentation says: "there are some command line arguments that don't have equivalents in Blender's Python API (--threads and --log for example)." <https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html>, read 2026-07-30. Those words carry that `--threads` has no Python equivalent. That `-t` and `render.threads` are therefore two separate controls rather than two ways to reach one is **inferred** — the step a reader must supply is that having no Python equivalent means the argument does not write the scene property. Note also that the scene property itself *is* reachable from outside, by the start-up-script route above; what is unestablished is what `-t` does to it. Hand test 1 below is aimed at exactly this.
- **The preferences.** Cycles Render Devices, Distribute Memory Across Devices, MetalRT, HIP RT, Embree on GPU, Kernel Optimization, Shader Compilation Method and the texture cache directory are all user preferences, stored in a different file: "./config/userpref.blend — User preferences." <https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html>. Cycles' own files confirm the device list is preference data — `properties.py` line 1618 carries the comment `# Properties saved in preferences` directly above the `CyclesDeviceSettings` fields, and `CyclesPreferences` at line 1625 is declared `class CyclesPreferences(bpy.types.AddonPreferences)`. Only the compute device itself has a command-line override, `--cycles-device`. For the rest, no argument exists.
- **No environment variables.** I searched the manual, the API documentation and Cycles' files for environment variables that set threads, device or memory. The manual lists its environment variables in full at the foot of the arguments page — they cover user and system directories, the splash, OpenColorIO, the 3D-mouse socket and the temporary directory. The only Cycles one anywhere is `CYCLES_CUDA_EXTRA_CFLAGS`, and it is a compiler flag: "This is done by setting the CYCLES_CUDA_EXTRA_CFLAGS environment variable when starting Blender."

**Where I looked.** The full Command Line Arguments page and the Rendering From The Command Line page; `extension_arguments.txt` for other `--command` verbs; `cycles_5.2/engine.py`, `__init__.py` and `maketx.py` for the add-on's own argument parsing; `api_5.2/info_advanced_blender_as_bpy.txt`; the directory layout page; searches across all sources for `CYCLES_`, `BLENDER_THREADS`, `OMP_NUM`, "environ", "getenv".

**Not covered.** This does not settle precedence when two of these are given at once — for example `-t` together with a `--python-expr` that sets `render.threads`, or `--cycles-device` on a scene whose device is CPU. The Cycles help string says the argument overrides "user preferences and the scene setting", which covers that one case; the others are unstated.

**Outside the interface.** This question is entirely about outside the interface, and the sources are written for it: "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)." <https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html>

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

**Operating system.** `--qos` is "(Windows only)". `--gpu-compilation-subprocesses` is "(OpenGL only)", and the preference it overrides is "not available on macOS". `--gpu-backend` accepts "vulkan, metal, opengl", and Metal is an Apple technology by the GPU Rendering page. The values `--cycles-device` accepts are constrained by platform in the same way — CUDA, OptiX, HIP and oneAPI on Windows and Linux, Metal on Apple Silicon. `-t`, `-E`, `--python-expr` and `--background` carry no platform note.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 33 and 34, run on the rented Linux machine in all four versions. Two things are added. A file whose Device says GPU Compute does not reach the card from the command line unless a backend is named somewhere else — settled by which picture came out, replacing the half of hand test 7 that searched Blender's output for complaints and so could not have failed. And `--cycles-device` naming a backend the build accepts but has no hardware for does not quietly fall back: the render stops, writes no image, and returns a non-zero exit code. Both Mac halves are still owed, so what these say about macOS is untested rather than answered.*

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

**Label: documented for where each is presented; inferred for what "belongs to" means in the data.** Two different things can be meant, and they do not line up, so I answer both.

### By where Blender puts them

**Cycles only.** Tile Size, Texture Cache and Auto Generate, the whole Acceleration Structure panel (Use Spatial Splits, Use Curves BVH, BVH Time Steps, Use Compact BVH), Persistent Data, the Threads panel, Device, Scrambling Distance, Path Guiding, Sample Subset, adaptive subdivision Offscreen Scale, and the Cycles Render Devices preference with everything under it. All of these are on the Cycles Performance, Sampling or Subdivision pages, and none appear on the EEVEE or Workbench pages. Cycles' own files make the panel ownership plain: **observed**, `cycles_5.2/ui.py` — every panel class in that file inherits `CyclesButtonsPanel`, whose gate at lines 60–68 is `COMPAT_ENGINES = {'CYCLES'}` and `def poll(cls, context): return context.engine in cls.COMPAT_ENGINES`. `--cycles-device` and `--cycles-print-stats` are likewise Cycles' own, parsed by the add-on itself in `engine.py`.

**EEVEE only.** Shadow Pool, Light Probes Volume Pool, the ray-tracing Resolution and Fast GI Resolution, volumetric Resolution / Steps / Max Depth, shadow Rays / Steps / Resolution, and TAA samples. The EEVEE Performance page carries the two pools; the EEVEE Raytracing, Volumes and Sampling pages carry the rest.

**EEVEE and Workbench, but not Cycles.** High Quality Normals and Anisotropic Filtering appear on both the EEVEE Performance page and the Workbench Performance page, and on neither Cycles page. The Workbench page holds nothing else at all — its entire content is those two settings.

**Shared by Cycles and EEVEE.** The Compositor group — Device, Precision, and the Denoise Nodes device and Preview/Final Quality — is word-for-word the same on the Cycles Performance page and the EEVEE Performance page. In Cycles' files this is visible as reuse rather than duplication: **observed**, `ui.py` line 14 imports `CompositorPerformanceButtonsPanel` and `CompositorDenoisePerformanceButtonsPanel` from `bl_ui.properties_render`, and lines 842–850 subclass them.

**Engine-independent.** Simplify, Resolution Percentage, Color Depth and Lock Interface are documented outside any engine's own pages, and `--threads`, `--qos`, `--gpu-backend` and `--gpu-compilation-subprocesses` are general Blender arguments.

### By where the value is stored

This is where the two readings part company, and it matters for anyone reading a file. Several settings that Blender only ever shows inside a Cycles panel are not Cycles data at all — they are ordinary scene render settings. Threads Mode, Threads, Persistent Data, Texture Cache, Auto Generate, Compositor Device and High Quality Normals are all properties of `bpy.types.RenderSettings`, documented with no mention of any engine: for instance "Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage (default False)". <https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html> Cycles' user interface code reads them off `scene.render`, not off `scene.cycles` — **observed**, `ui.py` lines 863–870 (`rd = scene.render`, `col.prop(rd, "threads_mode")`, `sub.prop(rd, "threads")`) and lines 893–894 (`rd = context.scene.render`, `self.layout.prop(rd, "use_texture_cache", text="")`).

Genuinely Cycles-owned values live on a separate structure the add-on attaches to the scene — **observed**, `properties.py` lines 1140–1147:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

That Cycles is an add-on and not part of Blender proper is stated in its own file: `__init__.py` lines 7–16 carry `bl_info = {"name": "Cycles Render Engine", … "support": 'OFFICIAL', "category": "Render"}`. EEVEE's values sit on `Scene.eevee`, documented as `bpy.types.SceneEEVEE`, "Scene display settings for 3D viewport".

**Not covered.** The sources do not say what happens to a Cycles-only value when the scene's engine is EEVEE, or the reverse — whether it is ignored, kept, or dropped. Nor do they say whether an engine-independent property such as `render.threads` is actually honoured by EEVEE; the property is engine-neutral, the panel that exposes it is Cycles-only, and EEVEE's own page says it "only uses the power of the GPU to render", but no page connects those. I have not tried to bridge that gap.

**No record of the search.** Unlike the other four answers, this one records no list of pages searched or terms tried. Several of its claims are negative ones — that of the Cycles-only settings "none appear on the EEVEE or Workbench pages", that High Quality Normals and Anisotropic Filtering appear "on neither Cycles page", that the Workbench page "holds nothing else at all" — and a negative claim with no record of looking cannot be repeated or checked. Sent back for research.

**Outside the interface.** The panel gate quoted above (`poll` on `context.engine`) governs what is drawn, so in a background render with no panels it is not in play; the underlying values are all present either way. That last step is **inferred** — the sources describe `poll` as a panel classmethod and do not discuss background mode. What is documented is that the engine itself is selectable at launch, `-E, --engine <engine>`, and that `-E help` lists what is available.

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

**Operating system.** No engine ownership above is platform-dependent. The one platform line about an engine is EEVEE's: "Headless rendering is not supported on headless Windows systems."

*Brought up to date by the hand tests recorded at the end of this section, and this line needed it more than most: the tests that bear hardest on it were run last, on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**, because they need a window and the rented Linux machine has no display server at all. **Both of the questions this line raises are now answered rather than open.** Driven from outside its own window, the engine split holds exactly as it does inside it — Simplify's Render column reaches all three engines, Cycles' values survive an engine switch and are readable while EEVEE is the engine, and EEVEE and Workbench both render in a background run on macOS and on Linux alike. And inside the window, the three engines really do show different Performance panels, and the Acceleration Structure controls really do change with the device. On the operating system half: which Cycles backends exist **is** tied to the operating system, and this was settled by asking a running Blender rather than left to the documentation — the Mac offers `NONE` and `METAL` and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` by name, where the Linux machine offers `NONE`, `CUDA` and `OPTIX`. *Disagreement recorded by the citation check, 2026-08-07, not resolved.* That check asked the Linux machine the same question that was asked of the Mac — which values the backend setting will actually take — in a fresh Blender for each value, and got five rather than three: `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI` all accepted, and only `METAL` refused, with `TypeError bpy_struct: item.attr = val: enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. The two runs may be answering two different questions — which values the setting takes, against which backends have a card behind them — and this line does not say which it means. Both results stand as written; neither has been overwritten by the other. Sent back to the hand tests to be run again with the question stated. Everything else under this question came out the same on both machines. The results below say which machine each came from. Where they are silent, this line still stands.* *Brought up to date again on 2026-08-07 by hand tests 34 and 35, run on the rented Linux machine in all four versions. Hand test 34 asks the two questions separately at last — which values the backend setting will take, and which backends have a device behind them — and this machine takes five names and refuses `METAL` by name, while having devices behind two. **The disagreement about the number three is recorded again and is still not resolved**: two runs of the acceptance question have now come out as five, both are written out in the hand tests word for word, the sentence above is left exactly as it stood, and nobody has chosen between them, because the sentence may mean the narrower question. Hand test 35 leaves the set of names that differ between versions exactly as it was — the same twenty-eight — and settles that of the three property totals on the record only two can be reproduced. The Mac halves of both are still owed.*

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

**Label: inferred for the main answer, documented for each step it rests on.** I read "a file" as the blend-file the render will use. The other reading — a settings file that holds these values outside any scene — I answer at the end.

**The main answer: yes for the scene-side settings, by opening the file in Blender and reading the properties; and no for the preference-side ones, because they are not in that file at all.**

Each step is documented:

1. The values are properties of data-blocks that live in the blend-file. `bpy.types.RenderSettings` is described as "Rendering settings for a Scene data-block", and `Scene.render` and `Scene.eevee` are listed as its references. <https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html> and <https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html>, read 2026-07-30.
2. Cycles' values are attached by the add-on as a property group on the Scene, quoted under question 4. Properties defined this way are saved with the file: "This data is saved with the blend-file and copied with objects". <https://docs.blender.org/api/5.2/info_quickstart.html>, read 2026-07-30.
3. A file can be opened without the interface: "-b, --background — Run in background (often used for UI-less rendering)" and a script can be run in that session: "-P, --python <filepath> — Run the given Python script file" / "--python-expr <expression>". Also `bpy.ops.wm.open_mainfile(*, filepath='', …)`. <https://docs.blender.org/api/5.2/bpy.ops.wm.html>

The inferred step is the join: that running such a script against such a file lets you print those properties. The sources document each piece and never put them together on one page, so I mark the whole as inferred rather than documented.

**What you cannot do is read them without loading the file.** The one documented way to look inside a blend-file short of opening it is `bpy.data.libraries.load()`, and it hands back names only: "The input contains the data-blocks available in the loaded blend-file library and should be treated as read-only… Attributes matching bpy.data, which are lists of strings representing linkable data-blocks." A new addition in 5.2 extends that to library paths and nothing more: "Some limited library info is now exposed to the `bpy.data.libraries.load()` input context manager, giving in particular access to the blend-file paths of the libraries used by the loaded one." <https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html> and <https://developer.blender.org/docs/release_notes/5.2/python_api/>. Render settings are not data-block names, so this route does not reach them. I searched all three documentation sets for a published blend-file format, a stand-alone reader, or any "read without opening" facility — terms "blendfile", "file format specification", "DNA struct", "without opening Blender" — and found nothing.

**Two catches worth naming.**

- **Cycles' values need Cycles registered to read them by name.** `scene.cycles` only exists because the add-on's `register()` creates it (quoted in question 4), and `unregister()` immediately below it does `del bpy.types.Scene.cycles`. **Observed**, `properties.py` lines 1140–1153. That the attribute is therefore absent when the add-on is not loaded is **inferred** — the file shows what `register()` and `unregister()` do, not what state a Blender that ran neither of them is in. Hand test 5 below is aimed at this.
- **The preference-side settings are in a different file.** Compute device type, the per-device tick boxes, Distribute Memory Across Devices, MetalRT, HIP RT, Embree on GPU, Kernel Optimization and Shader Compilation Method are user preferences — "./config/userpref.blend — User preferences" — and Cycles' file marks them so directly: `# Properties saved in preferences`, `properties.py` line 1618. Reading a scene file will not tell you which device a render will actually use.

**One 5.2 addition aimed squarely at reading a file from outside.** The release notes record: "Three new options for `bpy.data.file_path_foreach` so render farms and other scripts can find all relevant image files for rendering, including the new Cycles texture cache… `EXPAND_TOKENS`: visit each UDIM tile path. `EXPAND_SEQUENCES`: visit each image and volume sequence file path. `EXPAND_CACHES`: visit cache files, like texture cache files associated with images." <https://developer.blender.org/docs/release_notes/5.2/python_api/>, read 2026-07-30. That reads file paths, not settings values, but it is the sources' own example of inspecting a blend-file for what a render will need.

**The other reading of the question.** These values can also be written to and read from a file that is not a blend-file: Blender's presets. Cycles defines a performance preset that stores exactly the seven values quoted in question 1, and the manual describes three shipped presets — Default, Faster Render, Lower Memory. **Observed**, `presets.py` lines 91–110, where `AddPresetPerformance` declares `preset_defines = ["render = bpy.context.scene.render", "cycles = bpy.context.scene.cycles"]` and the `preset_values` list. The Cycles panel that offers them is `CYCLES_PT_performance_presets` with `preset_subdir = "cycles/performance"` (`ui.py` lines 54–57, class declaration through `preset_add_operator`). I did not find a page describing the preset file format, so how such a file is read outside Blender is **not in the sources**.

**Where I looked.** `api_5.2/bpy.types.RenderSettings.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.BlendDataLibraries.txt`, `bpy.props.txt`, `info_quickstart.txt`, `info_overview.txt`, `info_advanced_blender_as_bpy.txt`, `bpy.ops.wm.txt`, `bpy.ops.render.txt`; the 5.2 Python API release notes in full; the manual's directory layout page and command-line pages; `cycles_5.2/properties.py`, `presets.py`, `ui.py`, `__init__.py`. Searches across all sources for "saved in the blend", "stored in the blend", "written to the blend", "custom propert", "blendfile", "open_mainfile", "libraries.load".

**Not covered.** This does not settle whether a value read back matches what the render will actually use — `render.threads` read from a file says nothing about `-t` given on the command line, and `cycles.device` says nothing about `--cycles-device`. It does not settle what `render.threads` reads back as when Threads Mode is Auto-Detect; the API gives its default as 1 and its range as [1, 1024] and says nothing further. And it does not settle how a file written by an older Blender reports these values on load; Cycles ships a `version_update.py`, which I noted but did not read into.

**Outside the interface.** This whole answer is about reading from outside; the reading route documented — background mode plus a script — has no interface at all. One caveat the API documentation gives about the related case of using Blender as a Python module: "Functionality controlled by command line arguments (shown by calling blender --help) isn't accessible."

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

**Operating system.** Nothing in this answer is marked platform-specific. The blend-file, the preferences file and the Python API are described identically for all platforms in everything I read.

*Brought up to date by the hand tests recorded at the end of this section. Three sittings have now touched this line: a rented Linux machine on 2026-08-06, when that machine's graphics card could not be reached at all; the same machine on 2026-08-07, once the card had been repaired; and **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card — on 2026-08-07**. Every test that could be run on both machines was, so the two standing questions this line raises are now answered rather than open: **the settings behave the same when Blender is driven from outside its own window, and, for everything settled by which picture came out or by what a setting read back as, they behave the same on Linux and on macOS**, with the differences named below and nowhere else. *Narrowed by the citation check, 2026-08-07: this line used to claim sameness across the two operating systems outright. Three tests were never run on the Mac at all — hand test 3, hand test 23 and hand test 24, whose only evidence is a time or a memory figure — so for those, macOS is untested and not merely silent.* The results say which machine each came from. Where they are silent, this line still stands.*

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

This list has been pruned three times. Everything it held that a rented Linux machine could settle was run on 2026-08-06 and is written into the answers above, under the question each one answers. The one list covering every version, with the full procedure for each test and the results, is `Audits/Blender Research/What a render demands/Hand tests.md`. **A third sitting, on the Creator's own Mac on 2026-08-07, ran every item that was still owed there, including the two that need a window — which no machine on this project but that one has.** Only genuinely open items are left here.

**Still open, and only on a scene large enough to press on memory.**

8. **Does the Texture Cache change a non-OSL render on a scene whose textures far exceed the machine's memory?** The small-scene half of this has been run and is recorded under question 1 above; it changed the picture and raised peak memory slightly. The large-scene case is a separate question and needs a scene whose textures do not fit in memory.
   *Negative result:* peak memory is unchanged on such a scene.

**Run and answered above:** 1 (`-t` and `render.threads`), 2 (`--cycles-device` and `scene.cycles.device`), 3 (thread count and the picture), 4 (EEVEE and `-t`), 5 (`scene.cycles` with the add-on off), 6 (`render.threads` under Auto-Detect) and **7 (which panels the Performance section shows under each engine)** — which was run in a window on the Creator's Mac on 2026-08-07 and is recorded under question 4 above. The three engines do **not** show the same sub-panels: Threads, Acceleration Structure and Final Render appear under Cycles and under neither of the other two.

**All of that has now been run, on the Creator's own Mac on 2026-08-07, and is recorded above under the question each one answers.** Apple's own graphics render a different picture from the processor, exactly as the NVIDIA card does; `--cycles-device METAL` beats both the file and the saved preferences, in both directions; the three engines really do show different Performance panels; and the Acceleration Structure controls really do change with the device. **Nothing on this subject is still owed on either machine.**

Three tests were deliberately **not** repeated on the Mac, and the reason is written here rather than left as a silence: the ones whose only possible evidence is a time or a memory figure — whether `-t` changes how much of the processor a Cycles render uses, whether the thread count changes anything under EEVEE or Workbench, and whether Persistent Data changes the time or the memory of an animation. The Mac was shared with other topics' work throughout, so a figure taken there would be a busy machine's figure with nothing to compare it against. All three were run on the rented Linux machine and are recorded above. What they leave open on macOS is left open and is not guessed at.

---

2026-08-06 — Opus 5 (Claude Code) — What a render demands, Blender 5.2

Sources consulted: Blender 5.3 Manual (snapshot 2026-10-11), Blender 5.3 Python API Documentation (snapshot 2026-10-11), Blender 5.3 Release Notes (snapshot 2026-10-11), and Cycles source files for Blender 5.3 (snapshot 2026-10-16).
I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

# Settings

### 1. Which of a render's settings can be settled at the moment a render is started from outside Blender, and which can only be changed by changing the file? Cover at least a frame range, an interval to skip frames by, a list of separate frames, a sample count, whether a view layer renders at a given frame, a render engine, and a resolution percentage.

**Where looked:**
- Searched `manual_5.3/advanced/command_line/arguments.txt` and `manual_5.3/advanced/command_line/render.txt` for terms `frame-start`, `frame-end`, `frame-jump`, `render-frame`, `engine`, `sample`, `resolution`, `layer`, `view_layer`.
- Searched `cycles_5.3/engine.py` for command-line arguments registered by Cycles (`--cycles-device`, `--cycles-print-stats`).
- Searched `api_5.3` for `RenderSettings.resolution_percentage`, `ViewLayer.use`, `CyclesRenderSettings.samples`, and Python command-line switches (`--python-expr`, `--python`, `-P`).
- Ruled out general command-line arguments for setting sample count, view layer enablement per frame, or resolution percentage directly: none exist in Blender's CLI argument list.

**Finding:**
Settings that can be settled directly at launch via dedicated command-line arguments from outside Blender:
1. **Frame range:** Settled via `-s, --frame-start <frame>` and `-e, --frame-end <frame>`, or using range syntax `-f <start>..<end>`.
2. **Interval to skip frames by:** Settled via `-j, --frame-jump <frames>`.
3. **List of separate frames:** Settled via `-f, --render-frame <frame>` passing a comma-separated list of frame numbers (e.g. `-f 1,5,10`).
4. **Render engine:** Settled via `-E, --engine <engine>`.
5. Other settings with dedicated CLI arguments: background mode (`-b`), active scene (`-S <name>`), animation render (`-a`), output path and file template (`-o <path>`), output format (`-F <format>`), file extension appending (`-x <bool>`), thread count (`-t <threads>`), Cycles compute device override (`-- --cycles-device <device>`), and Cycles statistics logging (`-- --cycles-print-stats`).

Settings that have no dedicated command-line arguments:
1. **Sample count:** There is no command-line flag for sample count. In the file, it is stored in engine-specific settings (`scene.cycles.samples`, `scene.eevee.taa_render_samples`, or `view_layer.samples`).
2. **Whether a view layer renders at a given frame:** There is no command-line flag. In the file, `bpy.types.ViewLayer.use` ("Use for Rendering") is a static boolean per view layer. The manual notes that the single layer toggle (`Render Single Layer`) is ignored when rendering from the command line. Blender provides no native CLI switch or setting to selectively toggle a view layer conditionally on specific frames of an animation.
3. **Resolution percentage:** There is no command-line flag for resolution percentage. In the file, it is stored in `scene.render.resolution_percentage`.

However, Blender provides Python evaluation options at launch:
- `-P, --python <filepath>`: "Run the given Python script file."
- `--python-expr <expression>`: "Run the given expression as a Python script. The expression may be a complete multi-line script; you are limited only by the platform’s maximum argument length."
Because command-line arguments are executed sequentially in the order given, passing a Python expression after the blend-file loads and before the render command (e.g. `blender -b file.blend --python-expr "bpy.context.scene.render.resolution_percentage = 50; bpy.context.scene.cycles.samples = 128" -a`) allows settling sample count and resolution percentage in memory at launch without modifying the file on disk. Without executing Python at launch, sample count, resolution percentage, and view layer rendering can only be changed by modifying the file.

LABEL: **documented** (for CLI arguments and properties), **inferred** (for using `--python-expr` to alter in-memory settings at launch rather than changing the file).
*Inference step:* Blender executes CLI arguments in the order given and `--python-expr` runs Python against the loaded scene before `-a` or `-f` triggers rendering, thereby allowing in-memory modification without saving to the blend-file.

**Not covered:**
Whether external process wrappers or third-party render farm managers provide their own configuration files or argument translation layers on top of Blender.

**Sources:**
- `https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html`, read 2026-10-11:
  > "-s, --frame-start <frame>  
  > Set start to frame <frame>, supports +/- for relative frames too."
  > "-e, --frame-end <frame>  
  > Set end to frame <frame>, supports +/- for relative frames too."
  > "-j, --frame-jump <frames>  
  > Set number of frames to step forward after each rendered frame."
  > "-f, --render-frame <frame>  
  > Render frame <frame> and save it. +<frame> start frame relative, -<frame> end frame relative. A comma separated list of frames can also be used (no spaces). A range of frames can be expressed using .. separator between the first and last frames (inclusive)."
  > "-E, --engine <engine>  
  > Specify the render engine. Use -E help to list available engines."
  > "--python-expr <expression>  
  > Run the given expression as a Python script. The expression may be a complete multi-line script; you are limited only by the platform’s maximum argument length."
  > "Argument Order  
  > Arguments are executed in the order they are given."
- `https://docs.blender.org/manual/en/5.3/render/layers/view_layer.html`, read 2026-10-11:
  > "Render Single Layer  
  > Only render the active view layer.  
  > Note: This option is ignored when rendering from the command line."
- `https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html`, read 2026-10-11:
  > "resolution_percentage  
  > Percentage scale for render resolution (in [1, 32767], default 100)  
  > Type: int"
- `https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html`, read 2026-10-11:
  > "use  
  > Enable or disable rendering of this View Layer (default True)  
  > Type: bool"
- `Blender Sources/cycles_5.3/engine.py`, lines 16–20, read 2026-10-16:
  ```python
  parser.add_argument("--cycles-device",
                      help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                           "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                           "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                      default=None)
  ```

**Outside the interface.** This entire answer specifically addresses driving Blender from the command line outside its interface.  
**Operating system.** Holds on Linux, macOS, and Windows. Command-line syntax and argument processing are identical across operating systems, subject only to OS command-line string length limits for `--python-expr`.

**Observed in Blender 5.3 — hand tests of 2026-10-17.** Run from a terminal on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — by Claude Opus 5.5; written up in full, with the machine's own output, under question 1 of `Preparation/Blender Research/Findings/5.3/Settings.md` (entries 1 to 6, 10, 14 to 16, 60 and 61 of `Audits/Blender Research/Settings/Hand tests — 5.3.md`). On the two standing lines above: **outside the interface**, a Python line after the file settled the sample count, the resolution percentage and a view layer's switch for that run and left the file unchanged, and a view layer's switch keyed per frame was honoured by an animation render with no window; **operating system**, every run was on macOS, and no Linux or Windows machine was available to this sitting, so "holds on Linux, macOS, and Windows" is observed for macOS alone.

---

### 2. Can a render be set to stop once the picture is clean enough, and can it be set to stop after a length of time rather than after a count?

**Where looked:**
- Searched `manual_5.3/render/cycles/render_settings/sampling.txt` for `Time Limit`, `Adaptive Sampling`, `Noise Threshold`, and `clean`.
- Searched `manual_5.3/render/eevee/render_settings/sampling.txt` for `Time Limit` and `Noise Threshold`.
- Searched `cycles_5.3/properties.py` for `time_limit`, `adaptive_threshold`, and `use_adaptive_sampling`.
- Searched `api_5.3/bpy.types.SceneEEVEE.txt` for `time_limit`.
- Ruled out adaptive threshold stopping in EEVEE: EEVEE's sampling documentation does not describe adaptive sampling stopping.

**Finding:**
1. **Stopping once the picture is clean enough:**
   - In Cycles: Yes. Cycles supports Adaptive Sampling via the Noise Threshold setting. Cycles cuts short sampling in regions that reach the noise threshold, and when all pixels reach the specified threshold, sampling terminates early before reaching the maximum sample count.
   - In EEVEE: The sources do not document a feature to stop rendering once a noise cleanliness threshold is reached.
2. **Stopping after a length of time rather than after a count:**
   - Yes, in both Cycles and EEVEE. Both engines have a dedicated `Time Limit` setting under `Render ‣ Sampling`. Setting `Time Limit` stops rendering once the specified elapsed render time is reached, even if the desired sample count has not been reached. A value of zero disables the limit. Pre-render processing / synchronization time is excluded from the limit.

LABEL: **documented** (for Time Limit in Cycles and EEVEE, and Adaptive Sampling / Noise Threshold in Cycles), **inferred** (that Cycles stops the entire render early once all pixels satisfy the noise threshold).
*Inference step:* The manual states that adaptive sampling cuts short sampling in areas less noisy than the threshold; when all areas meet this criterion, path tracing has no remaining pixels to refine and stops.

**Not covered:**
Whether the Workbench engine provides time limits (it does not document a sampling panel or time limit).

**Sources:**
- `https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html`, read 2026-10-11:
  > "Time Limit  
  > Stops rendering if the time exceeds the limit, even if the desired sample count hasn’t been reached yet. Setting this value to zero disables the limit.  
  > Note: The time limit does not include pre-render processing time, only render time."
  > "Adaptive Sampling  
  > If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling, cutting short the sampling process in areas that have become less noisy than the specified threshold value – in other words, not wasting time further refining areas that already look good enough."
  > "Noise Threshold  
  > The threshold to decide whether to continue sampling a pixel. Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times. Setting this to 0 makes Cycles guess an automatic value based on the total sample count."
- `https://docs.blender.org/manual/en/5.3/render/eevee/render_settings/sampling.html`, read 2026-10-11:
  > "Time Limit  
  > Stops rendering if the time exceeds the limit, even if the desired sample count hasn’t been reached yet. Setting this value to zero disables the limit.  
  > Note: The time limit does not include pre-render processing time, only render time."
- `Blender Sources/cycles_5.3/properties.py`, lines 568–576, 620–633, read 2026-10-16:
  ```python
  time_limit: FloatProperty(
      name="Time Limit",
      description="Limit the render time (excluding synchronization time). "
      "Zero disables the limit",
      min=0.0,
      default=0.0,
      step=100.0,
      unit='TIME_ABSOLUTE',
  )
  use_adaptive_sampling: BoolProperty(
      name="Use Adaptive Sampling",
      description="Automatically reduce the number of samples per pixel based on estimated noise level",
      default=True,
  )
  adaptive_threshold: FloatProperty(
      name="Adaptive Sampling Threshold",
      description="Noise level step to stop sampling at, lower values reduce noise at the cost of render time. Zero for automatic setting based on number of AA samples",
      min=0.0,
      max=1.0,
      soft_min=0.001,
      default=0.01,
      precision=4,
  )
  ```
- `https://docs.blender.org/api/5.3/bpy.types.SceneEEVEE.html`, read 2026-10-11:
  > "time_limit  
  > Time limit for rendering in seconds (in [0, inf], default 0.0)  
  > Type: float"

**Outside the interface.** Holds outside the interface. Renders executed via command-line flags (e.g. `-f` or `-a`) respect both the noise threshold and the time limit saved in the file or injected via Python.  
**Operating system.** Holds on all operating systems supported by Blender.

**Observed in Blender 5.3 — hand tests of 2026-10-17.** Run on the Creator's Mac (MacBook Pro, Apple M1 Pro with its own 14-core graphics, 16 GB, macOS 26.6.2) by Claude Opus 5.5; written up in full under question 2 of `Preparation/Blender Research/Findings/5.3/Settings.md` (entries 18 to 23, 63 and 67). On the two standing lines: **outside the interface**, renders started from a terminal with no window did respect the noise threshold (a whole frame ended at 48 of 2048), Cycles' time limit (spent on each frame, the cut-short frame still written) and EEVEE's time limit (the render stopped at 275 of 4096 after about two seconds and wrote its picture), none printing a word about it; **operating system**, observed on macOS alone — no Linux or Windows machine was available to this sitting. On the Workbench point this answer leaves out: Workbench's settings block and its Sampling panel were found to hold no time limit (entries 20 and 63).

---

### 3. Which of a render's settings belong to a particular render engine rather than to every engine?

**Where looked:**
- Searched `manual_5.3/render/render_engines.txt` and `manual_5.3/render/introduction.txt` for engine comparisons and setting ownership.
- Searched `api_5.3/bpy.types.Scene.txt` for properties on `Scene` (`render`, `cycles`, `eevee`, `display`).
- Searched `api_5.3/bpy.types.RenderSettings.txt` for common render settings.
- Searched `api_5.3/bpy.types.SceneEEVEE.txt` for EEVEE-specific properties.
- Searched `cycles_5.3/properties.py` for `CyclesRenderSettings` properties.
- Searched `manual_5.3/render/workbench/` for Workbench display properties.

**Finding:**
Blender divides render settings into common scene render settings (`scene.render` / `bpy.types.RenderSettings`, Output properties, and Color Management) shared across engines, and engine-specific settings attached as separate property groups on the Scene or View Layer.

**Settings belonging to EVERY engine (`scene.render` / `bpy.types.RenderSettings` and Output properties):**
- Resolution and framing: `resolution_x`, `resolution_y`, `resolution_percentage`, `pixel_aspect_x`, `pixel_aspect_y`.
- Frame range and timing: `frame_start`, `frame_end`, `frame_step`, `fps`, `fps_base`.
- File output format and encoding: `filepath`, `image_settings` (file format, color depth, compression/quality).
- Pipeline toggles: `use_compositing`, `use_sequencer`.
- Color management: display device, view transform, look, exposure, and gamma (`scene.view_settings`, `scene.display_settings`).
- Render stamping: `use_stamp`, stamp metadata fields.
- CPU render threading: `threads_mode`, `threads`.
- Compositor denoiser settings: `compositor_denoise_device`, `compositor_denoise_final_quality`, `compositor_denoise_preview_quality`.

**Settings belonging to a PARTICULAR render engine:**
1. **Cycles only (`scene.cycles` / `CyclesRenderSettings` and `view_layer.cycles`):**
   - Compute device: `device` ('CPU' vs 'GPU').
   - Feature set: `feature_set` ('SUPPORTED' vs 'EXPERIMENTAL').
   - Path tracing integrator sampling: `samples`, `preview_samples`, `use_adaptive_sampling`, `adaptive_threshold`, `adaptive_min_samples`, `sampling_pattern`, `scrambling_distance`, `sample_subset_length`.
   - Light paths / Bounces: `max_bounces`, `diffuse_bounces`, `glossy_bounces`, `transmission_bounces`, `volume_bounces`, `transparent_max_bounces`.
   - Light clamping & caustics: `sample_clamp_direct`, `sample_clamp_indirect`, `caustics_reflective`, `caustics_refractive`, `blur_glossy`.
   - Shading system: `shading_system` (Open Shading Language toggle).
   - Cycles Denoising: `use_denoising`, `denoiser` (OpenImageDenoise, OptiX), `denoising_prefilter`, `denoising_quality`, `denoising_input_passes`, `denoising_use_gpu`, `use_preview_denoising`, `preview_denoiser`, `preview_denoising_upscale_quality`.
   - Path Guiding: `use_guiding`, `guiding_training_samples`, `guiding_directional_sampling_type`.
   - Volumes: `volume_step_rate`, `volume_max_steps`, `volume_bounces`.
   - Performance / BVH: `use_spatial_splits`, `bvh_time_steps`, `use_compact_bvh`, `tile_size`.
   - Curves & hair geometry: `cycles_curves.shape`, `cycles_curves.subdivisions`.
   - Cycles View Layer passes: Shadow Catcher passes, Lightgroup passes, and Cycles Denoising Data passes (`denoising_store_passes`, `denoising_store_motion_passes`).
2. **EEVEE only (`scene.eevee` / `bpy.types.SceneEEVEE` and `view_layer.eevee`):**
   - Temporal Anti-Aliasing (TAA) sampling: `taa_samples`, `taa_render_samples`.
   - Raytracing: `use_raytracing`, `ray_tracing_method`, `ray_tracing_resolution_scale`, `ray_max_roughness`, `screen_trace_quality`, `screen_trace_thickness`.
   - Fast GI Approximation: `use_fast_gi`, `fast_gi_method`, `fast_gi_resolution`, `fast_gi_ray_distance`, `fast_gi_bias`.
   - EEVEE Volumetrics: `volumetric_tile_size`, `volumetric_samples`, `volumetric_sample_distribution`, `volumetric_light_clamp`, `volumetric_shadow_samples`.
   - EEVEE Shadows: `shadow_ray_count`, `shadow_step_count`, `use_shadow_jittering`.
   - Light evaluation thresholds and clamping: `clamp_surface_direct`, `clamp_surface_indirect`, `clamp_volume_direct`, `clamp_volume_indirect`, `light_threshold`.
   - Depth of field bokeh properties: `bokeh_max_size`, `bokeh_threshold`, `bokeh_neighbor_max`, `bokeh_overblur`.
   - EEVEE View Layer settings: `ViewLayerEEVEE.ambient_occlusion_distance`, `ViewLayerEEVEE.denoising_store_passes`.
3. **Workbench only (`scene.display` / `bpy.types.SceneDisplay`):**
   - Shading lighting style: `shading.light` (Studio, MatCap, Flat).
   - Shading color mode: `shading.color_type` (Material, Single, Object, Random, Vertex, Texture).
   - Display options: `shading.show_backface_culling`, `shading.show_xray`, `shading.show_shadows`, `shading.show_cavity`, `shading.show_object_outline`.
   - Anti-aliasing method: `render_aa` (Off, FXAA, 5, 8, 11, 16, 32 samples).

LABEL: **documented** (common and engine-specific properties), **inferred** (attributing `scene.display` to Workbench). *(Changed by citation check: corrected `RenderDisplay` to `SceneDisplay`, and added inferred label for Workbench display properties.)*

**Not covered:**
Settings belonging exclusively to uninstalled third-party render add-ons (such as Octane, Radeon ProRender, or RenderMan).

**Sources:**
- `https://docs.blender.org/manual/en/5.3/render/render_engines.html`, read 2026-10-11:
  > "Blender provides the following render engines: EEVEE, Cycles, Workbench... Each renderer has its own render settings to control render quality and performance. Switching render engines may change available settings, materials, and rendering behavior. Some features are engine-specific."
- `https://docs.blender.org/manual/en/5.3/render/introduction.html`, read 2026-10-11:
  > "Each renderer has its own render settings to control render quality and performance."
- `https://docs.blender.org/api/5.3/bpy.types.Scene.html`, read 2026-10-11:
  > "cycles  
  > Cycles render settings (readonly)  
  > Type: CyclesRenderSettings"
  > "eevee  
  > EEVEE settings for the scene (readonly)  
  > Type: SceneEEVEE"
  > "display  
  > Scene display settings for 3D viewport (readonly)  
  > Type: RenderDisplay"
  > "render  
  > Render settings for the scene (readonly)  
  > Type: RenderSettings"
- `https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html`, read 2026-10-11.
- `https://docs.blender.org/api/5.3/bpy.types.SceneEEVEE.html`, read 2026-10-11.
- `Blender Sources/cycles_5.3/properties.py`, lines 427–550, read 2026-10-16.

**Outside the interface.** Holds outside the interface. The same property separation exists in the underlying data model accessed via Python and CLI.  
**Operating system.** Holds on all operating systems supported by Blender.

**Observed in Blender 5.3 — hand tests of 2026-10-17.** Run on the Creator's Mac (MacBook Pro, Apple M1 Pro with its own 14-core graphics, 16 GB, macOS 26.6.2) by Claude Opus 5.5; written up in full under question 3 of `Preparation/Blender Research/Findings/5.3/Settings.md` (entries 11, 20, 29, 45). On the two standing lines: **outside the interface**, from a terminal with no window all three engines' settings were reached in one run whatever the engine, and EEVEE's block holds 54 properties and the display block 9; **operating system**, observed on macOS alone — no Linux or Windows machine was available to this sitting.

---

### 5. Can a sample count change from frame to frame?

**Where looked:**
- Searched `manual_5.3/animation/keyframes/introduction.txt` and `editing.txt` for keyframing properties.
- Searched `api_5.3/bpy.types.bpy_struct.txt` for `keyframe_insert` and animation of RNA properties.
- Searched `cycles_5.3/properties.py` for definition of `samples: IntProperty`.
- Searched `api_5.3/bpy.types.SceneEEVEE.txt` for `taa_render_samples`.
- Searched `manual_5.3/advanced/command_line/arguments.txt` for per-frame rendering and script execution.
- Ruled out any restriction preventing animation of integer properties on Scene: Blender RNA allows inserting keyframes on `cycles.samples` and `eevee.taa_render_samples`.

**Finding:**
Yes. A sample count can change from frame to frame through multiple mechanisms:
1. **Keyframing / Animation Curves (F-Curves):**
   In Blender, scene RNA properties can be keyframed across time. The user can hover over the sample count field and press `I` or right-click to choose "Insert Keyframe", or call `scene.keyframe_insert(data_path="cycles.samples", frame=...)` via Python. When rendering an animation, the dependency graph evaluates animated scene properties on each frame prior to rendering that frame.
2. **Animation Drivers:**
   The sample count property can have a driver attached (`scene.driver_add("cycles.samples")`) driven by an expression (such as the current frame variable `#frame`).
3. **Application Handlers (Python Scripts):**
   A script can register a frame-change pre-render handler (`bpy.app.handlers.frame_change_pre`) that assigns a new sample count value to `scene.cycles.samples` or `scene.eevee.taa_render_samples` as the frame changes during rendering.
4. **Command-Line Invocation per Frame:**
   When rendering from outside Blender frame-by-frame, independent commands can supply different sample counts to different frames via `--python-expr "bpy.context.scene.cycles.samples = <N>"` before `-f <frame>`.
5. **Adaptive Sampling Variation:**
   Additionally, in Cycles, when Adaptive Sampling is enabled, the actual number of samples executed dynamically adapts to noise per pixel and per frame, meaning the effective sample count naturally varies from frame to frame even if the maximum sample count setting is unchanged.

LABEL: **documented** (for keyframe insertion, drivers, and handler capabilities across RNA properties), **observed** (that sample counts are standard RNA `IntProperty` attributes), **inferred** (that animating or driving `cycles.samples` / `eevee.taa_render_samples` alters the active sample count frame-to-frame during animation rendering).
*Inference step:* Because sample counts are RNA properties on `Scene` and `Scene` properties evaluate per frame via F-Curves/drivers during animation rendering, keyframing them causes the engine to render each frame with the evaluated value for that frame.

**Not covered:**
Whether external renderers or specific third-party render delegates cache the sample count at the start of an animation sequence rather than re-evaluating it per frame.

**Sources:**
- `https://docs.blender.org/manual/en/5.3/animation/keyframes/introduction.html`, read 2026-10-11:
  > "A Keyframe is simply a marker of time which stores the value of a property. For example, a Keyframe might define that the horizontal position of a cube is at 3 m on frame 1... depending on the chosen interpolation method"
- `https://docs.blender.org/manual/en/5.3/animation/keyframes/editing.html`, read 2026-10-11:
  > "Hovering over a property and pressing I or with the context menu by RMB a property and choose Insert Keyframe from the menu."
- `https://docs.blender.org/api/5.3/bpy.types.bpy_struct.html`, read 2026-10-11:
  > "keyframe_insert(data_path, *, index=-1, frame=None, group='', options=set(), keytype='KEYFRAME')  
  > Insert a keyframe on the property given, adding fcurves and animation data when necessary."
- `Blender Sources/cycles_5.3/properties.py`, lines 533–537, read 2026-10-16:
  ```python
  samples: IntProperty(
      name="Samples",
      description="Number of samples to render for each pixel",
      min=1, max=(1 << 24),
      default=4096,
  )
  ```
- `https://docs.blender.org/api/5.3/bpy.types.SceneEEVEE.html`, read 2026-10-11:
  > "taa_render_samples  
  > Number of samples per pixel for rendering (in [1, inf], default 64)  
  > Type: int"

**Outside the interface.** Holds outside the interface. Animations rendered via `-a` or frame-by-frame via `-f` evaluate animation curves, drivers, and frame-change handlers.  
**Operating system.** Holds on all operating systems supported by Blender.

**Observed in Blender 5.3 — hand tests of 2026-10-17.** Run on the Creator's Mac (MacBook Pro, Apple M1 Pro with its own 14-core graphics, 16 GB, macOS 26.6.2) by Claude Opus 5.5; written up in full under question 5 of `Preparation/Blender Research/Findings/5.3/Settings.md` (entries 14, 17, 61, 62). On the two standing lines: **outside the interface**, an animation render with no window used keyed counts frame by frame in Cycles, in EEVEE and for a view layer's own count; drivers and frame-change handlers were not tried. Workbench's `render_aa` refuses a keyframe (`property "render_aa" not animatable`). **Operating system**, observed on macOS alone — no Linux or Windows machine was available to this sitting.

---

### 9. Does a file record whether it renders on the computer's graphics cards or on its processor?

**Where looked:**
- Searched `cycles_5.3/properties.py` for `device` definition on `CyclesRenderSettings` and `enum_devices`.
- Searched `manual_5.3/render/cycles/gpu_rendering.txt` for GPU device configuration and preferences.
- Searched `api_5.3/bpy.types.SceneEEVEE.txt` and `manual_5.3/render/eevee/` for CPU vs GPU settings.
- Ruled out CPU execution options in EEVEE and Workbench: both engines are exclusively GPU-based.

**Finding:**
Yes, partially:
1. **Cycles:**
   - The `.blend` file records whether a scene targets the processor (`'CPU'`) or graphics cards (`'GPU'`). This is stored in the scene property `scene.cycles.device` (`CyclesRenderSettings.device`). Because `Scene` is saved in the `.blend` file, this choice is preserved in the file.
   - However, the file does **not** record *which* graphics cards are used, which GPU compute technology backend is used (CUDA, OptiX, HIP, oneAPI, or Metal), or whether the CPU is paired with the GPU for hybrid rendering (`+CPU`). Those hardware selections are stored in Blender's machine-specific User Preferences (`userpref.blend`), not in the scene or the `.blend` file. Furthermore, the command-line flag `--cycles-device <device>` overrides both the file setting and preferences at launch.
2. **EEVEE and Workbench:**
   - A `.blend` file does **not** record whether it renders on graphics cards or processor for EEVEE or Workbench. These engines are GPU rasterizers that execute on the system's display GPU hardware; they do not support CPU rendering and have no scene setting to choose between graphics card and processor.

LABEL: **documented** (in manual and API), **observed** (in Cycles source files).

**Not covered:**
Whether third-party render engine add-ons store their own proprietary hardware device assignments inside custom ID properties in the `.blend` file.

**Sources:**
- `https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html`, read 2026-10-11:
  > "To enable GPU rendering:  
  > 1. Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.  
  > 2. Enable the available devices for the chosen backend.  
  > 3. Configure each scene to use the GPU Compute Device."  
  > "Can multiple GPUs be used for rendering?  
  > Yes, go to Preferences ‣ System ‣ Compute Device Panel, and configure it as you desire."
- `Blender Sources/cycles_5.3/properties.py`, lines 32–36, 430–436, read 2026-10-16:
  ```python
  enum_devices = (
      ('CPU', "CPU", "Use CPU for rendering"),
      ('GPU', "GPU Compute",
          "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
  )

  device: EnumProperty(
      name="Device",
      description="Device to use for rendering",
      items=enum_devices,
      default='CPU',
      update=update_render_passes,
  )
  ```
- `Blender Sources/cycles_5.3/engine.py`, lines 16–20, read 2026-10-16:
  ```python
  parser.add_argument("--cycles-device",
                      help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                           "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                           "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                      default=None)
  ```

**Outside the interface.** Holds outside the interface. When launched from the command line without `--cycles-device`, Cycles reads `scene.cycles.device` from the blend-file and uses the compute devices configured in the host computer's preferences.  
**Operating system.** Holds on all operating systems supported by Blender (supported GPU backends vary by OS: Metal on macOS; CUDA, OptiX, HIP, and oneAPI on Windows and Linux).

**Observed in Blender 5.3 — hand tests of 2026-10-17.** Run on the Creator's Mac (MacBook Pro, Apple M1 Pro with its own 14-core graphics, 16 GB, macOS 26.6.2) by Claude Opus 5.5; written up in full under question 9 of `Preparation/Blender Research/Findings/5.3/Settings.md` (entries 1, 20, 30 to 32, 56, 65). On the two standing lines: **outside the interface**, from a terminal with no window the scene's device choice came back from the file per scene; `--cycles-device METAL` beat the file; a file asking for the graphics card rendered under every set of preferences with no warning, its statistics marking a render off the processor only when Metal and its devices were switched on; **operating system**, this Mac offers `NONE` and `METAL` only and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` with exit status 1; no Linux or Windows machine and no other make of card was available to this sitting. EEVEE's and Workbench's settings hold no device choice, and Workbench's Render tab has none either (entries 20, 65).

---

### 10. What does Blender state about opening a file that was saved by a different version of Blender?

**Where looked:**
- Searched `manual_5.3/files/blend/compatibility.txt` for forward compatibility, backward compatibility, and data loss.
- Searched `manual_5.3/files/blend/open_save.txt` for version differences and file compression changes.
- Searched `release_notes_5.3/core.md` for `.blend` file handling changes in 5.3.

**Finding:**
Blender states the following regarding opening blend-files saved by different versions:
1. **General compatibility statement:**
   Blender can open blend-files saved with both older versions (backward compatibility) and newer versions (forward compatibility), but with limitations.
2. **Backward Compatibility (opening older files in newer Blender):**
   - Opening older files and converting them for the current version is usually straightforward and expected to give very good, usable results.
   - For major architectural feature changes, backward compatibility is guaranteed for a limited time, which will "never be less than a full major release cycle (i.e. two years at least)."
   - When encountering issues opening much older files, Blender recommends using intermediary releases to perform conversions in smaller steps.
3. **Forward Compatibility (opening newer files in older Blender):**
   - Forward compatibility is inherently harder to ensure, and loss of features/data should always be expected when opening a blend-file saved with a newer version.
   - A warning is shown in the UI when editing a file saved with a more recent version. Saving or overwriting such a file triggers a confirmation popup warning that data loss could become permanent.
4. **Complete Incompatibility across major release cycles:**
   - When Blender switches to a new major version release (e.g. 3.x to 4.0), major changes can make blend-files fully incompatible with older versions. Older versions will fail to open (or append/link from) the newer file, displaying an error message stating the minimum version required.
   - In such cases, the last LTS release of the previous cycle is kept compatible with the newer format to serve as a converter (for example, Blender 3.6 LTS can open 4.x files and resave them for compatibility with earlier 3.x versions).
5. **File Compression Changes:**
   Prior to Blender 3.0, blend-file compression used Gzip. To open files in versions prior to 3.0, files saved in newer versions must first be saved without compression.
6. **Blender 5.3 developments:**
   In Blender 5.3 release notes, core changes include "Initial support for keeping .blend files the same when opening and saving a .blend file".

LABEL: **documented**

**Not covered:**
Exact binary sub-version compatibility tables for every point release of Blender.

**Sources:**
- `https://docs.blender.org/manual/en/5.3/files/blend/compatibility.html`, read 2026-10-11:
  > "Blender can open blend-files saved with both older versions of the software (backward compatibility), and newer ones (forward compatibility). This comes with some limitations though."  
  > "Tip: When having issues with opening much older (or newer) blend-files, it can help to use a few intermediary Blender releases to perform conversions by smaller steps."  
  > "Backward Compatibility  
  > Opening older files and converting them for the current version of Blender is usually straight-forward. It is expected to give very good and usable results.  
  > There can be major feature changes, for which the backward compatibility will only be ensured for a limited amount of time. For example the changes to the animation system that happened during the Blender 2.5x project. This will never be less than a full major release cycle (i.e. two years at least)."  
  > "Forward Compatibility  
  > Loss of Data  
  > Forward compatibility is inherently harder to ensure, and loss of feature should always be expected when opening a blend-file saved with a more recent version of Blender.  
  > A warning is shown in the UI when editing a more recent blend-file. Trying to overwrite it (with a simple ‘Save’ operation) will also show a confirmation popup, as this could make that loss of data permanent.  
  > Complete Incompatibility  
  > When Blender switches to a new major version release (e.g. from 3.x to 4.0), there can also be major changes that will make the blend-file fully incompatible with older versions of Blender.  
  > In such cases, older Blender will fail opening (or appending/linking from) the newer blend-file, with a message stating which minimal version is needed to open it.  
  > In such cases, the last LTS release of the previous release cycle will be kept compatible with the newer file format version, and will be usable as converter between both versions.  
  > For example, Blender 3.6 LTS can open files from Blender 4.x, and will perform the necessary conversion such that when re-saved from 3.6, the files become compatible with all 3.x Blender versions."
- `https://docs.blender.org/manual/en/5.3/files/blend/open_save.html`, read 2026-10-11:
  > "Changed in version 3.0: Prior to this version, the compression algorithm used was Gzip. This means to open newer blend-files in versions prior to 3.0, blend-files must first be saved without compression in a newer version of Blender"
- `https://developer.blender.org/docs/release_notes/5.3/core/`, read 2026-10-11:
  > ".blend Files: Initial support for keeping .blend files the same when opening and saving a .blend file (blender/blender@cadb3fe5c542dcdebab19b4a9369a0581d09a66c)."

**Outside the interface.** Holds outside the interface. Background mode loading (`-b`) performs the same file version conversion and reports incompatibility or conversion errors to the terminal stderr/stdout. However, interactive warning dialogues and overwrite confirmation popups are suppressed in background mode.  
**Operating system.** Holds on all operating systems supported by Blender. Blend-files are cross-platform binary files.

**Observed in Blender 5.3 — hand tests of 2026-10-17.** Run on the Creator's Mac (MacBook Pro, Apple M1 Pro with its own 14-core graphics, 16 GB, macOS 26.6.2) by Claude Opus 5.5, with Blender 5.2.2 as the other version; written up in full under question 10 of `Preparation/Blender Research/Findings/5.3/Settings.md` (entry 41). On the two standing lines: **outside the interface**, a 5.3 file opened by 5.2.2 with no window printed `WARNING File written by newer Blender binary (503.29), expect loss of data!` and rendered, exit status 0, and a 5.2.2 file opened by 5.3 printed nothing about versions and rendered; a newer-than-5.3 file could not be tried; **operating system**, observed on macOS alone — no Linux or Windows machine was available to this sitting.

---

### 11. Can a file rely on scripts stored inside it to render correctly, and what decides whether those scripts run when a render is started from outside Blender?

**Where looked:**
- Searched `manual_5.3/advanced/scripting/security.txt` for auto execution, security risks, command-line arguments, and scripts in blend-files.
- Searched `manual_5.3/advanced/command_line/arguments.txt` for `-y`, `--enable-autoexec`, `-Y`, `--disable-autoexec`.
- Searched `manual_5.3/render/cycles/osl/index.txt` for internal OSL scripts.
- Ruled out automatic execution of unapproved Python scripts: automatic Python execution is disabled by default for security.

**Finding:**
1. **Can a file rely on scripts stored inside it to render correctly?**
   Yes. Blend-files can store and rely on internal scripts for rendering in several ways:
   - **Registered Text-Blocks:** Python scripts stored as internal text data-blocks with the `Register` toggle enabled, which execute when the file loads.
   - **Animation Drivers:** Python expressions embedded in drivers to calculate animated property values (e.g. bone transforms, procedural motion, rig parameters).
   - **Freestyle line styling scripts:** Freestyle NPR rendering uses Python scripts to define line style modules.
   - **Internal Open Shading Language (OSL) scripts:** Cycles materials can contain Script Nodes whose mode is set to "Internal", compiling and running OSL shader code stored directly inside Blender internal text data-blocks.
2. **What decides whether those scripts run when a render is started from outside Blender?**
   - **For Python scripts and drivers:**
     - **Command-line flags take precedence:**
       - Passing `-y` or `--enable-autoexec` explicitly enables automatic script and driver execution.
       - Passing `-Y` or `--disable-autoexec` explicitly disables automatic script execution (this is Blender's default startup mode).
     - **User Preferences fallback:**
       If neither `-y` nor `-Y` is specified on the command line, Blender falls back to the host machine's User Preferences setting: `Auto Run Python Scripts` (and any excluded directories configured therein).
   - **For Freestyle scripts:**
     Freestyle scripts execute as part of the Freestyle rendering pipeline even when general Python auto-execution is disabled.
   - **For internal OSL scripts:**
     OSL scripts do not require Python auto-execution. They run automatically when `Open Shading Language` is checked in the scene's Render Properties and rendering runs on a supported device (CPU or OptiX).

LABEL: **documented**

**Not covered:**
Whether external third-party Python modules imported by internal scripts are installed in the system Python environment.

**Sources:**
- `https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html`, read 2026-10-11:
  > "The ability to include Python scripts within blend-files is valuable for advanced tasks such as rigging and automation. However, it poses a security risk since Python does not restrict what a script can do. Therefore, you should only run scripts from sources you know and trust. Automatic execution is disabled by default; however, some blend-files need this to function properly."  
  > "Scripts in Blend-Files  
  > Auto Execution  
  > Here are the different ways blend-files may automatically run scripts.  
  > Registered Text-Blocks: A text data-block can have its Register option enabled which means it will load on start.  
  > Animation Drivers: Python expressions can be used to Drive values and are often used in more advanced rigs and animations.  
  > Manual Execution: ... Rendering with Freestyle, because Freestyle uses scripts to control line styles."  
  > "Command Line  
  > You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them:  
  > Enable with -y or --enable-autoexec  
  > Disable with -Y or --disable-autoexec  
  > Example: To render an animation in background mode, allowing drivers and other scripts to run:  
  > blender --background --enable-autoexec my_movie.blend --render-anim  
  > Note: These command-line arguments can be used to start a regular Blender instance and will still override the Preferences."
- `https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html`, read 2026-10-11:
  > "-y, --enable-autoexec  
  > Enable automatic Python script execution."  
  > "-Y, --disable-autoexec  
  > Disable automatic Python script execution (Python-drivers & startup scripts), (default)."
- `https://docs.blender.org/manual/en/5.3/render/cycles/osl/index.html`, read 2026-10-11:
  > "Unlike node-based materials, OSL shaders are authored as text scripts using Blender’s internal Text Editor or loaded from external .osl or .oso files... Set the Mode to Internal to use a Blender text data-block... create a new text data-block in the Text Editor, then write or paste your OSL code there. Blender will compile the OSL source file automatically."

**Outside the interface.** Holds outside the interface. When rendering from outside Blender without UI, passing `-y` / `--enable-autoexec` is specifically required if files rely on drivers or registered Python blocks, because background mode does not display an interactive "Allow Execution" prompt.  
**Operating system.** Holds on all operating systems supported by Blender.

**Observed in Blender 5.3 — hand tests of 2026-10-17.** Run on the Creator's Mac (MacBook Pro, Apple M1 Pro with its own 14-core graphics, 16 GB, macOS 26.6.2) by Claude Opus 5.5; written up in full under question 11 of `Preparation/Blender Research/Findings/5.3/Settings.md` (entries 42 to 44). On the two standing lines: **outside the interface**, with neither switch the preference decided — with Auto Run on in the preferences a driver needing scripts ran with no switch at all — `-y` and `-Y` overrode it either way, a text block named by `--python-text` ran even under `-Y`, and an internal Open Shading Language script coloured the render under `-Y` as well; **operating system**, observed on macOS alone — no Linux or Windows machine was available to this sitting.

---

### 13. What decides the list of render engines a file can choose between, and can that list grow on a given computer?

**Where looked:**
- Searched `manual_5.3/render/render_engines.txt` and `manual_5.3/render/introduction.txt` for engine availability and add-ons.
- Searched `api_5.3/bpy.types.RenderEngine.txt` for custom render engine definition and registration.
- Searched `api_5.3/bpy.types.RenderSettings.txt` for `engine` and `has_multiple_engines`.
- Ruled out the file as the decider of available engines: the blend-file only stores the identifier string of the selected engine (`scene.render.engine`).

**Finding:**
1. **What decides the list of render engines a file can choose between?**
   The list of render engines is determined entirely by the Blender software installation and its active runtime environment on the computer, **not** by the file itself. The list consists of:
   - **Engines included with Blender:**
     1. EEVEE (`BLENDER_EEVEE`) and Workbench (`BLENDER_WORKBENCH`): built directly into the Blender binary.
     2. Cycles (`CYCLES`): bundled as an official add-on enabled by default (registered as a `RenderEngine` subclass). *(Changed by citation check: narrowed binary compilation claim to built-in engines; Cycles is an official add-on).*
   - **Registered third-party engines:** Renderers provided as add-ons, extensions, or Python scripts that are enabled and registered in that Blender instance.
   The blend-file simply stores an engine identifier string in `scene.render.engine` specifying which engine the scene prefers. If a file specifies an engine not present on the host computer, Blender cannot use that engine.
2. **Can that list grow on a given computer?**
   Yes. The list of engines can grow on any computer by:
   - Installing and enabling third-party render add-ons or extensions.
   - Registering a custom Python render engine subclass: In the Python API, any script or add-on can subclass `bpy.types.RenderEngine` (defining `bl_idname`, `bl_label`, `render()`, etc.) and register it via `bpy.utils.register_class(CustomRenderEngine)`. Once registered, the custom engine appears in the engine selection list in the user interface, the Python API, and the command line (`blender -E <name>`).

LABEL: **documented**

**Not covered:**
System-level GPU or driver requirements that cause an installed render engine to fail at initialization on unsupported hardware.

**Sources:**
- `https://docs.blender.org/manual/en/5.3/render/render_engines.html`, read 2026-10-11:
  > "Blender provides the following render engines: EEVEE, Cycles, Workbench... Each engine uses a different rendering approach and is optimized for specific tasks. More renderers from third-party developers are available as add-ons. Each renderer has its own render settings to control render quality and performance."
- `https://docs.blender.org/manual/en/5.3/render/introduction.html`, read 2026-10-11:
  > "Blender includes three render engines with different strengths: EEVEE is a physically based realtime renderer. Cycles is a physically based path tracer. Workbench is designed for layout, modeling and previews. More renderers from third-party developers are available as add-ons."
- `https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html`, read 2026-10-11:
  > "class CustomRenderEngine(bpy.types.RenderEngine):  
  > bl_idname = "CUSTOM"  
  > bl_label = "Custom"  
  > ...  
  > def register():  
  >     bpy.utils.register_class(CustomRenderEngine)"
- `https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html`, read 2026-10-11:
  > "engine  
  > Engine to use for rendering (default 'BLENDER_EEVEE')  
  > Type: string"  
  > "has_multiple_engines  
  > More than one rendering engine is available (default False, readonly)  
  > Type: bool"

**Outside the interface.** Holds outside the interface. Running `blender -E help` lists all available engines currently registered in background mode.  
**Operating system.** Holds on all operating systems supported by Blender.

**Observed in Blender 5.3 — hand tests of 2026-10-17.** Run on the Creator's Mac (MacBook Pro, Apple M1 Pro with its own 14-core graphics, 16 GB, macOS 26.6.2) by Claude Opus 5.5; written up in full under question 13 of `Preparation/Blender Research/Findings/5.3/Settings.md` (entries 2, 13, 45, 46, 64). On the two standing lines: **outside the interface**, `-E help` printed `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES` (no `BLENDER_EEVEE_NEXT`), grew by an engine a launch script registered and shrank when the Cycles add-on was off; a file naming an engine not present was reported and rendered on EEVEE; **operating system**, observed on macOS alone — no Linux or Windows machine was available to this sitting.

---

### 19. What settings does a scene or a view layer carry that govern denoising, and what does each of them do?

**Where looked:**
- Searched `manual_5.3/render/cycles/render_settings/sampling.txt` for denoising controls.
- Searched `manual_5.3/render/layers/passes.txt` for Denoising Data passes in Cycles and EEVEE.
- Searched `api_5.3/bpy.types.RenderSettings.txt` for compositor denoising settings (`compositor_denoise_device`, `compositor_denoise_final_quality`, `compositor_denoise_preview_quality`).
- Searched `api_5.3/bpy.types.ViewLayerEEVEE.txt` for `denoising_store_passes` and `denoising_pass_use_albedo_roughness_weighting`.
- Searched `cycles_5.3/properties.py` for `CyclesRenderSettings` and `CyclesViewLayerSettings` denoising properties.
- Searched `release_notes_5.3/cycles.md` for DLSS denoiser additions in 5.3.

**Finding:**
Denoising settings are carried across three levels: the Scene common render settings (`scene.render`), the Scene Cycles settings (`scene.cycles`), and the View Layer settings (`view_layer.cycles` and `view_layer.eevee`).

#### 1. Scene Common Render Settings (`scene.render` / `bpy.types.RenderSettings`):
Govern denoising performed in the Compositor via Denoise nodes across all engines:
- `compositor_denoise_device`: Selects the device used to process denoise nodes in the compositor (`'AUTO'`, `'CPU'`, or `'GPU'`). Auto uses the same device as the compositor; GPU falls back to CPU if unavailable.
- `compositor_denoise_final_quality`: The quality used by denoise nodes during compositing of final renders when the node's quality option is set to Follow Scene (`'HIGH'`, `'BALANCED'`, or `'FAST'`).
- `compositor_denoise_preview_quality`: The quality used by denoise nodes during viewport and interactive compositing when the node's quality is set to Follow Scene (`'HIGH'`, `'BALANCED'`, or `'FAST'`).

#### 2. Scene Cycles Settings (`scene.cycles` / `CyclesRenderSettings`):
Govern automatic denoising during and after Cycles rendering:
- **Final Render Denoising:**
  - `use_denoising`: Boolean. Enables or disables denoising for the final rendered image.
  - `denoiser`: Enum. Chooses the denoiser algorithm:
    - `OPENIMAGEDENOISE`: Intel Open Image Denoise AI denoiser (CPU or GPU).
    - `OPTIX`: NVIDIA OptiX AI denoiser (requires supported NVIDIA GPU).
  - `denoising_prefilter`: Enum (for OpenImageDenoise). Controls prefiltering of guiding passes before denoising:
    - `NONE`: No prefiltering applied. Fastest, preserves detail, assumes passes are clean.
    - `FAST`: Faster prefiltering, produces a blurrier result.
    - `ACCURATE`: Prefilters guiding passes to reduce noise before color denoising. Most detailed result.
  - `denoising_quality`: Enum (for OpenImageDenoise). Chooses overall quality: `HIGH`, `BALANCED`, or `FAST`.
  - `denoising_input_passes`: Enum. Controls which auxiliary passes guide the denoiser:
    - `RGB` ("None"): Color data only.
    - `RGB_ALBEDO` ("Albedo"): Color and albedo passes.
    - `RGB_ALBEDO_NORMAL` ("Albedo and Normal"): Color, albedo, and normal passes.
  - `denoising_use_gpu`: Boolean. Executes OpenImageDenoise on the GPU rather than CPU.
- **3D Viewport Denoising:**
  - `use_preview_denoising`: Boolean. Enables denoising after each viewport preview update.
  - `preview_denoiser`: Enum. Chooses viewport denoiser: `AUTO`, `OPTIX`, `OPENIMAGEDENOISE`, or `DLSS`.
  - `preview_denoising_prefilter`: Enum. Prefilter mode for viewport OIDN (`NONE`, `FAST`, `ACCURATE`).
  - `preview_denoising_quality`: Enum. Quality mode for viewport OIDN (`HIGH`, `BALANCED`, `FAST`).
  - `preview_denoising_input_passes`: Enum. Input passes for viewport denoiser (`RGB`, `RGB_ALBEDO`, `RGB_ALBEDO_NORMAL`).
  - `preview_denoising_start_sample`: Integer. Sample count at which viewport denoising starts.
  - `preview_denoising_use_gpu`: Boolean. Performs viewport denoising on GPU.
  - `preview_denoising_upscale_quality`: Enum. Upscaling factor and denoising quality when using NVIDIA DLSS in the viewport (new in 5.3).

#### 3. View Layer Settings:
- **Cycles View Layer Settings (`view_layer.cycles` / `CyclesViewLayerSettings`):**
  - `use_denoising`: Boolean (default True). Enables or disables denoising for this specific view layer in final renders.
  - `denoising_store_passes`: Boolean (default False). Outputs Denoising Data auxiliary passes (Noisy Image, Denoising Albedo, Denoising Normal, Denoising Roughness, Denoising Depth) for compositing with the Denoise node.
  - `denoising_store_motion_passes`: Boolean (default False). Stores motion vectors for temporal denoisers.
  - `denoising_pass_follow_reflections`: Boolean (default True). Directs feature passes to follow reflections.
  - `denoising_pass_use_albedo_roughness_weighting`: Boolean (default True). Uses roughness-based weighting of the albedo pass for denoising feature passes.
- **EEVEE View Layer Settings (`view_layer.eevee` / `bpy.types.ViewLayerEEVEE`):**
  - `denoising_store_passes`: Boolean (default False). Outputs denoising feature passes for EEVEE for compositing.
  - `denoising_pass_use_albedo_roughness_weighting`: Boolean (default True). Uses roughness-based weighting of the albedo pass.

LABEL: **documented** (in manual and API), **observed** (in Cycles source files).

**Not covered:**
Internal algorithmic weights of the OpenImageDenoise and OptiX AI neural networks.

**Sources:**
- `https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html`, read 2026-10-11:
  > "Denoising  
  > Denoising uses a specialized algorithm to get a less noisy image without requiring more samples...  
  > Denoise (Viewport): Whether to perform denoising for the 3D Viewport in the Rendered shading mode.  
  > Denoise (Render): Whether to perform denoising for the final rendered image.  
  > Denoiser: The algorithm to use. Automatic... OpenImageDenoise... OptiX...  
  > Passes: Controls which Render Passes the denoiser should use as input... None... Albedo... Albedo + Normal...  
  > Prefilter OpenImageDenoise: Controls whether prefiltering is applied to the Passes before denoising... None... Fast... Accurate...  
  > Quality OpenImageDenoise: Overall denoising quality... High... Balanced... Fast...  
  > Start Sample: Sample at which to start denoising in the 3D Viewport.  
  > Use GPU: Perform denoising on the GPU."
- `https://docs.blender.org/manual/en/5.3/render/layers/passes.html`, read 2026-10-11:
  > "Denoising Data Cycles, EEVEE  
  > Includes auxiliary passes used for denoising, along with the combined image before denoising. These passes can be used with the Denoise Node as an alternative to automatic denoising.  
  > Noisy Image – The combined render result before any denoising is applied.  
  > Denoising Albedo – The diffuse base color of surfaces without lighting or shading.  
  > Denoising Specular Albedo – The specular base color...  
  > Denoising Normal – Surface normals used to preserve edges and fine detail during denoising.  
  > Denoising Roughness – Surface roughness values used to improve denoising of glossy reflections.  
  > Denoising Depth – Distance from the camera, used to help preserve depth-based detail."
- `https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html`, read 2026-10-11:
  > "compositor_denoise_device: The device to use to process the denoise nodes in the compositor (default 'AUTO')"  
  > "compositor_denoise_final_quality: The quality used by denoise nodes during the compositing of final renders if the nodes’ quality option is set to Follow Scene (default 'HIGH')"  
  > "compositor_denoise_preview_quality: The quality used by denoise nodes during viewport and interactive compositing if the nodes’ quality option is set to Follow Scene (default 'BALANCED')"
- `https://docs.blender.org/api/5.3/bpy.types.ViewLayerEEVEE.html`, read 2026-10-11:
  > "denoising_store_passes: Deliver denoising passes (default False)"  
  > "denoising_pass_use_albedo_roughness_weighting: Use roughness-based weighting of the albedo for the denoising feature passes (default True)"
- `Blender Sources/cycles_5.3/properties.py`, lines 448–529, 1625–1654, read 2026-10-16.
- `https://developer.blender.org/docs/release_notes/5.3/cycles/`, read 2026-10-11:
  > "Denoising: On NVIDIA GPUs, a new DLSS denoiser is available for the viewport. It provides a cleaner image faster through upscaling and reusing samples from previous frames."

**Outside the interface.** Holds outside the interface. All scene and view layer denoising flags and pass generation settings are executed during background renders (`-b -f` or `-b -a`). Viewport denoising settings are ignored during background rendering.  
**Operating system.** Holds across all operating systems. Hardware support for GPU denoising depends on platform drivers: OptiX and DLSS require NVIDIA GPUs on Windows/Linux; OpenImageDenoise GPU acceleration runs on supported NVIDIA, AMD, Intel, and Apple Silicon GPUs.

**Observed in Blender 5.3 — hand tests of 2026-10-17.** Run on the Creator's Mac (MacBook Pro, Apple M1 Pro with its own 14-core graphics, 16 GB, macOS 26.6.2) by Claude Opus 5.5; written up in full under question 19 of `Preparation/Blender Research/Findings/5.3/Settings.md` (entries 55 to 59, 66). On the two standing lines: **outside the interface**, in renders with no window Cycles denoised only with both the scene's and the view layer's switch on, the compositor's denoise quality changed the render under both engines, and a file whose stored denoiser this Mac lacks still rendered denoised; whether the viewport denoising settings reach a background render was not tested; **operating system**, on this Mac the final render offers OpenImageDenoise alone and the viewport `AUTO` and OpenImageDenoise — no OptiX and no DLSS; what other cards and Linux or Windows offer was not available to this sitting.

---

## Things a Person Sitting at a Computer with Blender Open Could Settle by Hand

*(Pruned on 2026-10-17. All five items that stood here were run on the Creator's Mac, in Blender 5.3's own window driven from the terminal where a window was needed, and are written up as observed in `Preparation/Blender Research/Findings/5.3/Settings.md`: keying the sample count in the panel (question 5, entry 61); the Time Limit cutting a render short, from a terminal and in the render window (question 2, entries 18, 21 and 67); where the compute-device choice is kept — the file per scene, the preferences per machine, factory preferences holding Metal with no device switched on (question 9, entries 30 and 32); the Denoising Data passes on a Render Layers node (question 19, entry 57); and an engine registered by a script appearing as "Custom" in the Render Engine menu (question 13, entry 46). The whole record is `Audits/Blender Research/Settings/Hand tests — 5.3.md`. Nothing on this list is still open.)*

---

2026-10-16 — Gemini 3.8 Flash High — Settings, Blender 5.3

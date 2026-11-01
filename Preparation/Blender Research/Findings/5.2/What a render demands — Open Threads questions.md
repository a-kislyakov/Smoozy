# What a render demands — Open Threads questions (Blender 5.2)

**Sources consulted, and when.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including one image from its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and the Cycles add-on's own files (`Blender Sources/cycles_5.2`). All were taken on **2026-07-30**, and that is the date read for every finding below. Where a citation below is shortened to "(Manual, GPU Rendering)", it means that page at https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

No helper agents were used. Everything below was found by me, directly, in the two places I was allowed to read.

Scope: **question 7 only**, as instructed.

---

## 7. Where a render can be told which of a graphics card's rendering methods to use, what methods are there, what does choosing one rather than another change about the render, and does anything establish whether that changes with the size of the scene?

### A note on the question's wording, before the answer

The question can be read two ways, and I want to be plain about which one I answered.

**The reading I answered.** "A graphics card's rendering methods" = the technologies Cycles can use to render on a graphics card — CUDA, OptiX, HIP, oneAPI, Metal — and, inside those, the ray-tracing methods MetalRT, HIP RT and Embree on GPU. I chose this reading because the manual itself uses exactly this framing: the page that covers it is headed "Rendering Technologies" and opens "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer and operating system." — Manual, "GPU Rendering", https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30.

**The other reading, and what it would have given.** "A graphics card's rendering methods" could mean the graphics API Blender uses to draw with — OpenGL or Vulkan (and Metal on the command line). That is a real, separate setting in the same preferences panel, and I quote it at the end of this answer so the other reading is not left empty. The manual scopes it to drawing rather than to rendering an image, which is why I did not treat it as the main reading.

I did not rewrite the question. Both readings are answered; the first at length, the second briefly.

---

### 7a. Where the choice is made

**LABEL: documented** (and **observed** for the exact property names and defaults)

There are two separate places, and they do different jobs.

**1. In the preferences — which technology.** This is where the method itself is chosen.

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

— Manual, "GPU Rendering", https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30

> "Cycles Render Devices
>
> Changes the Computing Device the Cycles render engine uses to render images. Cycles can use either the CPU or certain GPUs to render images, for more information see the GPU Rendering page."

— Manual, "System" preferences, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30

**2. In the scene — whether to use it at all.** The scene only says CPU or GPU Compute; it does not say which technology.

> "Device
>
> Specifies the compute device used for rendering in Cycles. The choice of device can significantly affect render performance, GPU Compute is often faster .
>
> CPU:
>
> Render using the system's central processing unit. This mode is compatible with all Cycles features. […]
>
> GPU Compute:
>
> Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices. Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."

— Manual, "Render Settings", https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/index.html — read 2026-07-30

**The picture confirms the shape of the preference.** I opened `manual_5.2/_images/editors_preferences_section_system.webp`, the image the System preferences page shows as `[image: _images/editors_preferences_section_system.webp]` under the caption "Preferences System section." It shows a panel headed **Cycles Render Devices** containing a single row of three buttons — **None**, **CUDA**, **HIP** — with None highlighted and the other two not. Below it a panel headed **Display Graphics** with a single dropdown labelled **Backend**, set to **OpenGL**. What is visible: the technology choice is one row of buttons; three buttons appear, named None, CUDA and HIP; one of them is highlighted and the other two are not. (Documented — it is part of the manual.) **Inferred**, one step: that only one of the buttons can be active at a time. The step a reader has to accept is that a still picture of one highlighted button shows the rule rather than one moment; the picture shows a state, not what states are possible. What I do *not* take from it: which options appear on any other machine, or what machine it was taken on.

**In the add-on's own file**, the property is:

```
    compute_device_type: EnumProperty(
        name="Compute Device Type",
        description="Device to use for computation (rendering with Cycles)",
        default=CyclesPreferences.default_device(),
        items=CyclesPreferences.get_device_types,
    )
```

— `cycles_5.2/properties.py`, lines 1653–1659, inside `class CyclesPreferences(bpy.types.AddonPreferences)` — read 2026-07-30 — **observed**

The list of options is built when the panel is drawn, from what the running build reports:

```
    def get_device_types(self, context):
        import _cycles
        has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()

        list = [('NONE', "None", n_("Do not use compute device"), 0)]
        if has_cuda:
            list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))
        if has_optix:
            list.append(('OPTIX', "OptiX", n_("Use OptiX for GPU acceleration"), 3))
        if has_hip:
            list.append(('HIP', "HIP", n_("Use HIP for GPU acceleration"), 4))
        if has_metal:
            list.append(('METAL', "Metal", n_("Use Metal for GPU acceleration"), 5))
        if has_oneapi:
            list.append(('ONEAPI', "oneAPI", n_("Use oneAPI for GPU acceleration"), 6))

        return list
```

— `cycles_5.2/properties.py`, lines 1637–1652 — read 2026-07-30 — **observed**

The default is not the same everywhere:

```
    @staticmethod
    def default_device():
        import platform
        # Default to selecting the Metal compute device on Apple Silicon GPUs
        # (drivers are tightly integrated with macOS so pose no stability risk)
        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
            return 5
        return 0
```

— `cycles_5.2/properties.py`, lines 1628–1635 — read 2026-07-30 — **observed**. Value 5 is `METAL` and value 0 is `NONE` in the list above.

The scene-side property, in the same file:

```
enum_devices = (
    ('CPU', "CPU", "Use CPU for rendering"),
    ('GPU', "GPU Compute",
        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
)
```

— `cycles_5.2/properties.py`, lines 30–34 — read 2026-07-30 — **observed**

```
    device: EnumProperty(
        name="Device",
        description="Device to use for rendering",
        items=enum_devices,
        default='CPU',
        update=update_render_passes,
    )
```

— `cycles_5.2/properties.py`, lines 387–393 — read 2026-07-30 — **observed**

**3. From the command line — which technology, overriding both of the above.**

> "--cycles-device <device>
>
> Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU.
>
> Example:
>
> blender -b file.blend -f 20 -- --cycles-device OPTIX"

— Manual, "Command Line Arguments", https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. The page places this under a heading that says "Cycles add-on options must be specified following a double dash."

The add-on's own text for that switch says more than the manual does:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

— `cycles_5.2/engine.py`, lines 16–20 — read 2026-07-30 — **observed**

**Not covered (7a):** the sources do not say what happens if a technology named on the command line is not available on the machine, beyond the add-on file showing that a failed `set_device_override` ends in `sys.exit(1)` (`cycles_5.2/engine.py`, lines 38–41 — **observed**). Nothing says whether the preference is written back to disk, or whether an unsaved preference survives to the next launch.

---

### 7b. What methods there are

**LABEL: documented** (names and hardware), **observed** (exact values and defaults)

**Top layer — the technology.** Five, plus "None".

> "CUDA – NVIDIA
>
> CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher."

> "OptiX – NVIDIA
>
> OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."

> "HIP – AMD
>
> HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."

> "oneAPI – Intel
>
> oneAPI is a computation library that is supported on Windows and Linux and requires an Intel® Arc™ graphics card with the Xe HPG architecture."

> "Metal – Apple (macOS)
>
> Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

— all five: Manual, "GPU Rendering", https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30

The same five as stored values, with their numbers:

```
enum_device_type = (
    ('CPU', "CPU", "CPU", 0),
    ('CUDA', "CUDA", "CUDA", 1),
    ('OPTIX', "OptiX", "OptiX", 3),
    ('HIP', "HIP", "HIP", 4),
    ('METAL', "Metal", "Metal", 5),
    ('ONEAPI', "oneAPI", "oneAPI", 6)
)
```

— `cycles_5.2/properties.py`, lines 172–179 — read 2026-07-30 — **observed**. Note there is no 2; the numbering has a gap.

**Second layer — the ray-tracing method inside a technology.** Three settings, one per vendor, each written into the panel code inside a test on its own technology.

> "Embree on GPU
>
> Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance.
>
> Only supported with oneAPI rendering devices.
>
> HIP RT
>
> Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above.
>
> This feature is only available when using a HIP render device.
>
> MetalRT
>
> MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases.
>
> Off:
>
> Disable MetalRT (uses BVH2 layout for intersection queries).
>
> On:
>
> Enable MetalRT for intersection queries.
>
> Auto:
>
> Automatically pick the fastest intersection method."

— Manual, "System" preferences, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30

Their exact kinds and defaults:

```
    metalrt: EnumProperty(
        name="MetalRT",
        description="MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better "
                    "performance in specific cases",
        default='AUTO',
        items=(
            ('OFF', "Off", "Disable MetalRT (uses BVH2 layout for intersection queries)"),
            ('ON', "On", "Enable MetalRT for intersection queries"),
            ('AUTO', "Auto", "Automatically pick the fastest intersection method"),
        ),
    )

    use_hiprt: BoolProperty(
        name="HIP RT",
        description="HIP RT enables AMD hardware ray tracing on RDNA2 and above",
        default=True,
    )

    use_oneapirt: BoolProperty(
        name="Embree on GPU",
        description="Embree on GPU enables the use of hardware ray tracing on Intel GPUs, providing better overall performance",
        default=True,
    )
```

— `cycles_5.2/properties.py`, lines 1670–1692 — read 2026-07-30 — **observed**. So: MetalRT is a three-way choice defaulting to Auto; HIP RT and Embree on GPU are on/off switches, both defaulting to on.

There is no equivalent switch for CUDA or OptiX in that class. I read the whole `CyclesPreferences` class (lines 1625–1860) and the whole of its `draw_impl` (lines 1949–2005) and found only these three.

**Each of the three is written inside a test on the technology chosen** — from the add-on's own drawing code:

```
        if compute_device_type == 'METAL':
            import platform

            # MetalRT only works on Apple Silicon.
            if (platform.machine() == 'arm64'):
                col = layout.column()
                col.use_property_split = True
                col.prop(self, "kernel_optimization_level")
                row = col.row()
                row.active = has_hardware_rt
                row.prop(self, "metalrt")

        if compute_device_type == 'HIP':
            row = layout.row()
            row.active = has_hardware_rt
            row.prop(self, "use_hiprt")

        elif compute_device_type == 'ONEAPI' and _cycles.with_embree_gpu:
            row = layout.row()
            row.active = has_hardware_rt
            row.prop(self, "use_oneapirt")
```

— `cycles_5.2/properties.py`, lines 1986–2004 — read 2026-07-30 — **observed** for the code as quoted.

**Inferred from those lines:** each of the three is therefore *shown* only when its own technology is chosen. The steps a reader must supply are that this function is what draws the panel and that a block not reached draws nothing; the file states neither. **LABEL: inferred.** *(Citation check, 2026-08-30: this passage was labelled observed and read as a statement about what a person sees. What the file carries is where the properties are written. Both 5.0's and 5.1's first passes label the same step inferred. Changed by this check.)*

Just above that, in the same function, is a rule about mixed hardware:

```
        # Any device without RT support will disable it for all.
        has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt
```

— `cycles_5.2/properties.py`, lines 1977–1978 — read 2026-07-30 — **observed**. What the file says: that variable is computed this way, and it is used to set `row.active` on all three ray-tracing rows; the comment beside it reads "Any device without RT support will disable it for all." The comment does not say what "it" is, what "disable" does, or anything about the row. What actually happens to the render in that case is a step past the file, so I do not claim it.

**One more setting sits alongside them, for Metal only.**

```
    kernel_optimization_level: EnumProperty(
        name="Kernel Optimization",
        description="Kernels can be optimized based on scene content. Optimized kernels are requested at the start of a render. "
                    "If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set "
                    "is available in the cache. This can result in additional CPU usage for a brief time (tens of seconds)",
        default='FULL',
        items=(
            ('OFF', "Off", "Disable kernel optimization. Slowest rendering, no extra background CPU usage"),
            ('INTERSECT', "Intersection only", "Optimize only intersection kernels. Faster rendering, negligible extra background CPU usage"),
            ('FULL', "Full", "Optimize all kernels. Fastest rendering, may result in extra background CPU usage"),
        ),
    )
```

— `cycles_5.2/properties.py`, lines 1694–1706 — read 2026-07-30 — **observed**. I searched the whole 5.2 manual for "Kernel Optimization" and found nothing; this setting is in the add-on's file but not in the manual.

**Not covered (7b):** the internal list of intersection layouts in Cycles' kernel headers (`BVH_LAYOUT_BVH2`, `BVH_LAYOUT_EMBREE`, `BVH_LAYOUT_OPTIX`, `BVH_LAYOUT_METAL`, `BVH_LAYOUT_HIPRT`, `BVH_LAYOUT_EMBREEGPU` and their "MULTI" variants — `cycles_5.2/source/kernel/types.h`, lines 1293–1317 — **observed**) is longer than the list of settings a person can set. Nothing in the sources I was given maps those layouts onto the three switches above, so I do not attempt it.

---

### 7c. What choosing one rather than another changes about the render

**LABEL: documented**, with the pieces below. Grouped by what changes.

**(i) Which features work at all.**

> "Limitations
>
> Path Guiding is not supported on any GPU.
>
> Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

— Manual, "GPU Rendering", https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30

> "OSL is not supported with GPU rendering unless using the OptiX backend.
>
> Some OSL features are not available when using the OptiX backend. Examples include:
>
> Texture lookups require OSL to be able to determine a constant image file path for each
>
> texture call.
>
> Some noise functions are not available. Examples include Cell, Simplex, and Gabor.
>
> The trace function is not functional."

— Manual, "Open Shading Language", https://docs.blender.org/manual/en/5.2/render/cycles/osl/index.html — read 2026-07-30. (The line break in the middle of the first bullet is in the source text; I have quoted it as it stands.)

> "Custom cameras are not supported with GPU rendering unless using the OptiX backend."

— Manual, "OSL Custom Camera", https://docs.blender.org/manual/en/5.2/render/cycles/osl/camera.html — read 2026-07-30

> "OSL is supported only when rendering with the CPU or with the OptiX GPU Compute backend."

— Manual, "Render Settings", https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/index.html — read 2026-07-30

**(ii) Which settings are even shown.**

> "The Acceleration Structure panel will not be present when using an Optix compute device."

— Manual, "Performance", https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html — read 2026-07-30. The panel it refers to holds Use Spatial Splits, Use Curves BVH, BVH Time Steps and Use Compact BVH, each of which that same page describes as trading render speed against memory.

**(iii) What happens when the scene will not fit.**

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

— Manual, "GPU Rendering", https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30

Note what that sentence names: CUDA, OptiX, HIP and Metal. oneAPI is not in the list. I record that as what the sentence says. I do **not** claim oneAPI lacks the behaviour — the page simply does not mention it, and that is a different thing.

**(iv) Whether several cards can pool their memory.**

> "Would multiple GPUs increase available memory?
>
> Typically, no, each GPU can only access its own memory.
>
> The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."

— Manual, "GPU Rendering", same address — read 2026-07-30

> "Distribute Memory Across Devices
>
> Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes. Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol.
>
> Currently only NVLink on NVIDIA GPUs is supported."

— Manual, "System" preferences, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30

The matching property:

```
    peer_memory: BoolProperty(
        name="Distribute memory across devices",
        description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it",
        default=False,
    )
```

— `cycles_5.2/properties.py`, lines 1664–1668 — read 2026-07-30 — **observed**. Default off.

**(v) Which denoising is available, and where it runs.**

> "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards." (OptiX section)

> "GPU accelerated denoising is available on discrete Radeon RX 6000 and newer GPUs on Linux, and discrete Radeon RX 7000 and newer GPUs on Windows." (HIP section)

> "GPU accelerated denoising is available on all supported GPUs." (oneAPI section)

> "GPU accelerated ray-tracing and denoising is available on Apple Silicon." (Metal section)

— all: Manual, "GPU Rendering", https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30. The CUDA section contains no such sentence.

> "OptiX:
>
> Uses NVIDIA's OptiX AI denoiser. Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.
>
> Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences."

— Manual, "Sampling", https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30

**(vi) What the ray-tracing switches change.** The three quotes in 7b are the whole of what the sources say: MetalRT "uses less memory for scenes which use curves extensively, and can give better performance in specific cases", with Off described as using "BVH2 layout for intersection queries"; HIP RT "Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above"; Embree on GPU "Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance". And for OptiX generally: "OptiX takes advantage of hardware ray-tracing acceleration in RTX graphics cards, for improved performance." — Manual, "GPU Rendering", https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30.

**(vii) A general warning the manual attaches to the whole choice.**

> "GPU rendering may have limitations compared to CPU rendering, depending on the backend."

— Manual, "Render Settings", https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/index.html — read 2026-07-30

**Not covered (7c):** nothing in the sources says whether the finished image is *identical* between two technologies, or how it differs if it is not. Every statement I found is about what is supported, how fast it is, or how much memory it uses. There is one hint in the other direction, in the fixed-bug list rather than the documentation:

> " * HIP RT: Transparent curves render darker than expected"
> " * Cycles HIP RT + Subsurface Scattering + Motion Blur + Object Animation = Dark, Oversaturated, Partially Transparent Render"
> " * MetalRT Light Linking Not Updating Until Light Is Toggled"

— Release notes, "Fixes", https://developer.blender.org/docs/release_notes/5.2/bugfixes/ — read 2026-07-30, lines 336–339 of `release_notes_5.2/bugfixes.md`. These are listed as fixed in 5.2. Quoting them is reporting what the release notes state. They show that the ray-tracing method *has* been able to change the picture, and that these particular cases were fixed; they establish nothing about the general case.

Also not covered: the amount of any speed or memory difference. No source gives a number. The manual explicitly declines to:

> "What renders faster?
>
> This varies depending on the hardware used. Different technologies also have different compute times depending on the scene tested. For the most up to date information on the performance of different devices, browse the Blender Open Data resource."

— Manual, "GPU Rendering", same address — read 2026-07-30

---

### 7d. Does anything establish whether that changes with the size of the scene?

**LABEL: documented, partly — and "not in the sources" for the strong form of the question.**

This sub-question also has two readings. I answer both.

**Reading one: does the *effect* of the choice change with the size of the scene?** Partly documented. Three sentences bear on it directly, and they are the only three I found.

> "Different technologies also have different compute times depending on the scene tested."

— Manual, "GPU Rendering", https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30. This says the ranking between technologies is not fixed; it varies by scene. It says nothing about *size* specifically, and gives no rule.

> "MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases."

— Manual, "System" preferences, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30. This ties one method's advantage to a property of scene content — heavy use of curves — not to size as such.

> "GPU rendering makes it possible to use your graphics card for rendering, instead of the CPU. This can speed up rendering because modern GPUs are designed to do quite a lot of number crunching. On the other hand, they also have some limitations in rendering complex scenes, due to more limited memory, and issues with interactivity when using the same graphics card for display and rendering."

— Manual, "GPU Rendering", same address — read 2026-07-30. This is GPU versus CPU, not one technology versus another.

Adding to those, the memory-fallback sentence quoted in 7c(iii) is itself a scene-size-dependent behaviour that is stated for four technologies and not for the fifth — it only comes into play when the scene no longer fits.

**Reading two: does the choice itself get changed automatically as the scene grows?** **Not in the sources.** One setting sounds as if it might:

> "Auto:
>
> Automatically pick the fastest intersection method."

— Manual, "System" preferences, same address — read 2026-07-30 — and the identical text in `cycles_5.2/properties.py`, line 1678. The sources say the pick is automatic. They nowhere say what it is based on, when it is decided, or whether the scene is looked at. I searched for that and found nothing (searches listed below). Anything I said about the basis for that pick would be invention.

One further setting mentions scene content, though it is a compilation option rather than a method choice, and it is only for Metal:

> "Kernels can be optimized based on scene content."

— `cycles_5.2/properties.py`, line 1696 (the `kernel_optimization_level` description) — read 2026-07-30 — **observed**

And in the add-on's hidden debug panel:

```
    adaptive_compile_description = "Compile the Cycles GPU kernel with only the feature set required for the current scene"
```

— `cycles_5.2/properties.py`, line 1107, used for `debug_use_cuda_adaptive_compile`, `debug_use_hip_adaptive_compile` and `debug_use_metal_adaptive_compile` (lines 1109–1127, all `default=False`) — read 2026-07-30 — **observed**. "Feature set required for the current scene" is scene *content*, not scene size. These sit in `CYCLES_RENDER_PT_debug` (`cycles_5.2/ui.py`, lines 2278–2315), a panel the file gates behind a debug class.

**Not covered (7d):** no source states a threshold, a measurement, or a rule of any kind. Nothing says at what point a scene is "large" for these purposes. Nothing establishes whether the Auto setting for MetalRT re-decides per scene, per render, or per session. Nothing compares how two technologies scale as a scene grows.

---

### 7e. The other reading — the graphics API

Included briefly so the second reading of the question is not left blank.

> "Backend
>
> Selects the graphics API used for drawing the interface and rendering display content.
>
> Changing the backend requires restarting Blender for the change to take effect.
>
> OpenGL:
>
> Uses the OpenGL backend. This is the traditional backend, compatible with a wide range of systems.
>
> Vulkan:
>
> Uses the Vulkan backend. Vulkan may offer improved performance and better support for modern GPU features, but compatibility may vary depending on the system and drivers."

— Manual, "System" preferences, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30

> "--gpu-backend
>
> Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."

— Manual, "Command Line Arguments", https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30

The screenshot in 7a shows this as a dropdown in a panel headed "Display Graphics", set to OpenGL.

**LABEL: documented.** **Not covered:** the manual scopes this to "drawing the interface and rendering display content". It does not say whether or how it affects a rendered image, and I found no page in the 5.2 manual that connects it to EEVEE's or Cycles' output. Note also that the command-line list includes `metal`, which the preferences page does not mention as an option, and the preferences page has a companion "Device Vulkan" setting for which GPU draws the interface. Anything further would be guesswork.

---


---

## Tested by hand — what a running Blender showed

**Label: observed.** Run on 2026-08-30 by Claude Opus 5 in Claude Code, on the two computers described below, in **Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.1**. The same tests were run in all four versions, so that a gap in one is not read as a difference in Blender. Blender 5.2's own figures are given here in full; where the four versions agreed, that is said.

**Why these were run at all.** The Cycles files supplied to this research hold the render kernel and its helpers and no device-implementation folder, so the part of Cycles that picks a rendering method, builds the structure it uses, and decides what "Auto" means is in no file that could be read. Only a running Blender settles it.

### The two computers

**The Creator's Mac.** A MacBook Pro with an Apple M1 Pro chip, macOS 26.6.2 build 25G83, Darwin 25.6.0, arm64. Apple's own graphics with fourteen graphics cores; **no second card and no NVIDIA card**. Blender 4.5.12 (build hash `84afd5f785f7`), 5.0.1 (`a3db93c5b259`), 5.1.2 (`ec6e62d40fa9`) and 5.2.1 (`9e2066aef7ef`). Every run used either `--factory-startup` or a scratch configuration folder set with `BLENDER_USER_CONFIG`, so nothing of the Creator's own Blender settings was read or written. **Another agent was running Blender on this same computer at the time**, so no time measured on the Mac is a clean one; times from it are given as three repeats and nothing here rests on a single Mac time. What that contention does not touch is what the machine printed, the memory Cycles reported, and whether two images match.

**A rented Linux machine.** Vast.ai instance `49253937`, machine `135829`, in Austria, rented and destroyed on 2026-08-30. **A container, not a whole machine**, established by reading rather than assumed: `/.dockerenv` present, `systemd-detect-virt` says `docker`, process 1 is the provider's launch script. Ubuntu 22.04.4 LTS, kernel `5.15.0-190-generic`, glibc 2.35. An AMD EPYC 3151 4-Core Processor with 8 visible threads against a quota of 7.68 cores; a memory ceiling of 32,161,923,072 bytes; one **NVIDIA GeForce RTX 3080 with 10,240 MiB on driver 595.71.05**, CUDA 13.2; a 60 GiB overlay root. **It was chosen to be uncontended and it was**: the provider reported the whole processor and the whole card, and `nvidia-smi` before anything ran showed 0% card use and 1 MiB of 10,240 MiB in use. The four Blenders were installed from Blender's own download server as the official Linux x64 archives, and **all four build hashes matched the Mac's exactly**, which matters here because this question asks which method was used. As rented it had no display server; one was installed for the panel photograph.

**Neither machine speaks for the other.** The Mac answers the Metal half and nothing else; the Linux machine answers the CUDA and OptiX half and nothing else.

### The scenes

Four `.blend` files, all built by Blender 4.5.12 on each machine from one script so that all four versions could open the same file. Every scene: Cycles, 640 by 360, 64 samples, **adaptive sampling off, denoising off, seed 0 with animated seed off, stamping off**, eight bounces, saved as 32-bit OpenEXR with no compression, scene device set to `CPU`.

| Scene | What is in it | Counted size |
|---|---|---|
| small | one icosphere, a floor, a sun | 322 triangles |
| heavy mesh | eight icospheres at subdivision 9, a floor, a sun | 10,485,762 triangles |
| heavy curves | 300,000 hair curves of eight points each, a floor, a sun | 300,000 curves, 2,400,000 points |
| huge mesh | twenty-four icospheres at subdivision 10, a floor, a sun | 125,829,122 triangles |

The two ordinary heavy scenes are **32,564 times** the small one in triangles. That gap is the point: two scenes that both fit comfortably would show the same thing under either method whatever Blender did.

---

### 1. What methods this Blender offers, and what the list is built from

Blender 5.2 was started with no window and `--factory-startup` on each machine, and asked what the Cycles preference offers.

On the **rented Linux machine** it returned:

```text
get_device_types(context) = [('NONE', 'None', 'Do not use compute device', 0), ('CUDA', 'CUDA', 'Use CUDA for GPU acceleration', 1), ('OPTIX', 'OptiX', 'Use OptiX for GPU acceleration', 3), ('HIP', 'HIP', 'Use HIP for GPU acceleration', 4), ('ONEAPI', 'oneAPI', 'Use oneAPI for GPU acceleration', 6)]
get_devices_for_type CUDA = [('NVIDIA GeForce RTX 3080', 'CUDA', True), ('AMD EPYC 3151 4-Core Processor', 'CPU', False)]
get_devices_for_type OPTIX = [('NVIDIA GeForce RTX 3080', 'OPTIX', True), ('AMD EPYC 3151 4-Core Processor', 'CPU', False)]
get_devices_for_type HIP = []
get_devices_for_type METAL = []
get_devices_for_type ONEAPI = []
compute_device_type = 'NONE'
```

On the **Creator's Mac** it returned:

```text
get_device_types(context) = [('NONE', 'None', 'Do not use compute device', 0), ('METAL', 'Metal', 'Use Metal for GPU acceleration', 5)]
get_devices_for_type METAL = [('Apple M1 Pro (GPU - 14 cores)', 'METAL', True), ('Apple M1 Pro', 'CPU', False)]
compute_device_type = 'METAL'
```

Every other device type returned `[]` on the Mac. All four versions returned exactly these two lists on their respective machines.

**Two things follow, and the second is the one no reading could have given.** The list is not fixed — the two computers offer different lists, and the starting value differs (`NONE` on Linux, `METAL` on the Mac). But **the list is not built from the cards that are present either**: the Linux machine has one NVIDIA card, no AMD card and no Intel graphics, and it still offers HIP and oneAPI. What is empty in that case is the device list underneath the choice, not the choice itself.

### 2. The hardware ray-tracing switches, what they hold, and whether they can be set with no window

On both machines, in Blender 5.2, the Cycles preference object carries exactly eight properties:

```text
['bl_idname', 'compute_device_type', 'devices', 'kernel_optimization_level', 'metalrt', 'peer_memory', 'use_hiprt', 'use_oneapirt']
```

At factory settings, on both machines: `peer_memory = False`, `metalrt = 'AUTO'`, `use_hiprt = True`, `use_oneapirt = True`, `kernel_optimization_level = 'FULL'`.

*(Across the versions, `use_hiprt` starts `False` in 4.5.12 and 5.0.1 and `True` in 5.1.2 and 5.2.1 — the same on both machines, so that is a difference between Blender versions and not between computers. The other five start the same in all four.)*

All six settable values were assigned from a script in a background run with no window and read back:

```text
Q7SET compute_device_type: asked='NONE' before='METAL' after='NONE' TOOK
Q7SET metalrt: asked='OFF' before='AUTO' after='OFF' TOOK
Q7SET kernel_optimization_level: asked='INTERSECT' before='FULL' after='INTERSECT' TOOK
Q7SET use_hiprt: asked='true' before=True after=True TOOK
Q7SET use_oneapirt: asked='false' before=True after=False TOOK
Q7SET peer_memory: asked='true' before=False after=True TOOK
```

Findings 6 and 7 below show that setting `metalrt` and `kernel_optimization_level` this way changes what the render does, so the assignment is not merely accepted and then ignored.

### 3. Naming the device on the command line outranks the stored preference and the scene's own setting

Four crossings were run in Blender 5.2 on both machines, each with a scratch configuration folder holding a saved preference and a `.blend` saved with the scene's device one way or the other. Which method actually ran was read from two things Blender prints: `Building OptiX acceleration structure`, which only an OptiX render prints, and `Profiling information not available (only works with CPU rendering)`, which only a card render prints.

On the rented Linux machine: with the stored technology at `NONE` and the scene's device at `CPU`, adding `-- --cycles-device OPTIX` produced the OptiX lines, the card-only line, and a render in under a second against about six seconds for the processor. With the stored technology at `OPTIX` and the scene's device at `GPU`, adding `-- --cycles-device CPU` produced neither line and about six seconds. The Mac gave the same shape with `METAL` in place of `OPTIX`.

**So the command-line option beats both the stored preference and the scene's own setting, in both directions, on both operating systems.** All four versions behaved the same way.

### 4. What happens when the named method is one the machine cannot do, or is not a method at all

In Blender 5.2, on the rented Linux machine (no Apple, no AMD, no Intel graphics), `-- --cycles-device METAL`, `HIP` and `ONEAPI` were each asked for; on the Mac (no NVIDIA, no AMD, no Intel graphics), `CUDA`, `OPTIX` and `HIP`. On both, `-- --cycles-device BANANA` was also asked for.

**Every one of them ended with exit status 1 and wrote nothing to disk.** For a real method the machine has no hardware for, Blender 5.2 printed:

```text
ERROR Found no Cycles device of the specified type
```

For the nonsense name it printed:

```text
ERROR BANANA is not a valid Cycles device.
```

*(4.5.12 prints these as `Error:` lines; 5.0.1, 5.1.2 and 5.2.1 print them as `ERROR` lines through their own message system. The words after that are identical.)* Nothing fell back to the processor and nothing fell back to the card the machine did have. On the Linux machine, asking for `HIP` also printed `WARNING HIPEW initialization failed: Error opening HIP dynamic library` in 5.0.1, 5.1.2 and 5.2.1 before the error.

### 5. The choice of method changes the picture, and a control shows that is not run-to-run variation

Each scene was rendered with each method and the **pixels** compared, not the files — Blender writes clocks into a saved image, so a byte comparison reads a clock. The images are 32-bit OpenEXR at 640 by 360, which is 230,400 pixels.

**The control first, because without it this finding carries nothing.** The same scene was rendered **twice with the same method** and the two compared. On the processor, in Blender 5.2 on both machines, the two runs are **identical to the pixel** — not one channel differs. On a card they are not identical, but the largest difference between two runs of the same method is 4.17 × 10⁻⁷ to 8.34 × 10⁻⁷ on the rented Linux machine and 2.98 × 10⁻⁷ to 4.77 × 10⁻⁷ on the Mac.

Against that floor, in Blender 5.2:

| pair | small scene | heavy mesh | heavy curves |
|---|---|---|---|
| processor against CUDA (Linux) | 0.0121 | 0.0185 | 0.1212 |
| processor against OptiX (Linux) | 0.0102 | 0.0185 | 0.1177 |
| CUDA against OptiX (Linux) | 0.0121 | 0.0133 | 0.0431 |
| processor against Metal (Mac) | 0.0103 | 0.0185 | 0.1199 |
| MetalRT off against on (Mac) | 0.0103 | 0.0205 | 0.1425 |
| MetalRT off against auto (Mac) | 4.77 × 10⁻⁷ | 5.30 × 10⁻⁵ | 3.58 × 10⁻⁷ |

Figures are the largest difference in any colour channel; between 224,257 and 230,015 of the 230,400 pixels differ on the mesh scenes and about 149,000 to 162,500 on the curve scene. **Every pair of different methods in the first five rows differs by three to five orders of magnitude more than two runs of the same method do.** The last row, MetalRT off against auto, is at the noise floor on two of the three scenes; its heavy-mesh figure of 5.30 × 10⁻⁵ is two orders of magnitude above that floor and is not at it.

*(Citation check, 2026-08-30, repeated on the Creator's Mac in 5.2.1: the processor half of the control reproduced exactly — two CPU renders identical to the pixel on all three scenes — and the Mac card control came back at 4.77 × 10⁻⁷, 5.96 × 10⁻⁷ and 3.58 × 10⁻⁷, the same order as recorded. The first five rows reproduced to four decimal places: processor against Metal 0.010348, 0.018478 and 0.119888; MetalRT off against on 0.010348, 0.020503 and 0.142475. **The last row did not match on the heavy mesh scene**: off against auto came back at 4.77 × 10⁻⁷, **4.77 × 10⁻⁷** and 3.58 × 10⁻⁷ against the 4.77 × 10⁻⁷, 5.30 × 10⁻⁵ and 3.58 × 10⁻⁷ recorded. Both runs stand; neither has been chosen between, and the original figures above are untouched. The disagreement is on the hand-test list to be run again. The summary sentence was narrowed by this check because 5.30 × 10⁻⁵ is two orders of magnitude above the noise floor the same paragraph states.)*


*(Run again on 2026-08-30, and the control was measured properly this time: see **The noise floor measured, and three rows run again** below. Two runs of the same method were compared 120 times over. **On the processor all sixty pairs are identical to the pixel. On Metal the floor is 10⁻⁷ most of the time and not all of the time** — two card pairs in sixty came back at 5.31 × 10⁻⁵ and 0.0182. The rows above stay comfortably above the floor on the small and heavy mesh scenes and are only about six to eight times the worst case on the curve scene. Nothing above was changed.)*

### 6. What the choice changes depends on the size of the scene, and it changes direction

On the rented Linux machine, Blender 5.2, three repeats of every measurement with warm kernel caches. The three repeats agreed to within a few hundredths of a second throughout.

| scene | processor | CUDA | OptiX | card memory: CUDA / OptiX |
|---|---|---|---|---|
| small, 322 triangles | 00:05.96 / 00:06.00 / 00:06.09 | 00:00.68 / 00:00.66 / 00:00.68 | 00:00.71 / 00:00.71 / 00:00.72 | 1539 / 1553 MiB |
| heavy mesh, 10,485,762 triangles | 00:11.17 / 00:10.98 / 00:11.01 | 00:08.41 / 00:08.48 / 00:08.40 | 00:02.03 / 00:01.96 / 00:01.97 | 2163 / 2103 MiB |
| heavy curves, 300,000 curves | 00:15.11 / 00:14.86 / 00:14.98 | 00:06.67 / 00:06.68 / 00:06.66 | 00:01.39 / 00:01.34 / 00:01.39 | 1885 / 1659 MiB |

**On the 322-triangle scene CUDA is marginally ahead of OptiX. On the 10.5-million-triangle scene OptiX is about four times faster.** Same card, same driver, same file, same settings, same version. The card's own memory moves the same way: on the small scene OptiX takes slightly more than CUDA, on the heavy mesh scene rather less, and on the curve scene least of all. All four versions showed the same shape.

**A negative result was available and did not happen.** Had the ratio of one method's time to the other's been the same at both sizes, within the spread of the three repeats, the answer would have been that the effect does not depend on the size of the scene.

### 7. MetalRT, and what "Auto" does

The Creator's Mac only. MetalRT is a preference rather than a scene setting, so it was set from a script at start-up before the frame was rendered, and the script printed what it asked for and what the value became, so a run where the setting did not apply cannot be mistaken for one where it made no difference.

**What Blender prints while it builds separates the three settings cleanly, and identically in all four versions.** With MetalRT `ON` it prints `Building TLAS` and `Building mesh BLAS`, and on the curve scene `Building hair BLAS` as well. With `OFF` and with `AUTO` it prints `Packing BVH nodes` and `Packing BVH triangles and strands`. **On all three scenes, in all four versions, Auto built the same structure as Off.**

Cycles' own reported peak memory in Blender 5.2, which the shared Mac does not affect:

| scene | MetalRT Off | MetalRT On | MetalRT Auto |
|---|---|---|---|
| small | 1412 M | 1414 M | 1412 M |
| heavy mesh | 2017 M | 2474 M | 2017 M |
| heavy curves | 1742 M | 1709 M | 1742 M |

**At 322 triangles the switch changes nothing measurable. On the mesh scene it costs about 455 MB more. On the curve scene it saves about 34 MB.** That is the same direction for curves in all four versions and the opposite direction on a mesh scene of comparable weight, and it appears only once the scene is large enough for it to.

Times in Blender 5.2, fastest of three repeats on a shared Mac: on the heavy mesh scene, Off 00:04.98, On 00:03.32, Auto 00:04.80; on the curve scene, Off 00:05.60, On 00:05.88, Auto 00:05.78; on the small scene, Off 00:01.74, On 00:01.80, Auto 00:01.28.

### 8. Kernel Optimization

The Creator's Mac only, the heavy mesh scene, Blender 5.2, set from a script and confirmed to have taken. Off 00:05.57, Intersection only 00:05.36, Full 00:04.84. Off was the slowest in all four versions. On the pictures: FULL against OFF reached 0.0125, above the noise; INTERSECT against OFF stayed at 7.15 × 10⁻⁷.


*(Run again on 2026-08-30, three renders of each setting and every pair of the nine compared: see **The noise floor measured, and three rows run again** below. Nothing above was changed.)*

### 9. A scene the machine cannot hold, and where the choice decides whether the render happens at all

The huge mesh scene — **125,829,122 triangles**, a 9.3 GB file — on the rented Linux machine, in Blender 5.2:

```text
OptiX: exit status 0, 29.96 seconds of wall time, card memory peaked at 7825 MiB of 10,240, host memory peaked at 23,502,557,184 bytes, render time 00:15.29, frame written
CUDA:  exit status 137, 100.44 seconds of wall time, card memory peaked at 861 MiB of 10,240, host memory peaked at 32,916,774,912 bytes, no time line was reached, no frame written
```

**The OptiX run finished and wrote its frame. The CUDA run was killed by the system for want of host memory** — status 137, the last line printed being `Updating Scene BVH | Building BVH` at somewhere between 45% and 76%, with the container's memory at its ceiling of about 32.9 GB. The same happened in all four versions: every OptiX run completed, every CUDA run was killed.

**What this does not say.** The condition reached was the host's memory ceiling, not the card's. OptiX peaked at between 7,765 and 8,353 MiB of the card's 10,240 MiB and CUDA never got past 1,775 MiB before it died, so **nothing here bears on what happens when a card's own memory fills**, and the documented fallback to system memory was not exercised.

### 10. The Acceleration Structure panel

Blender 5.2 was started with no window on both machines and the panel itself was asked, for every crossing of the technology, the scene's device setting, and whether the processor is ticked in the device list, whether it would be drawn. The panel class `CYCLES_RENDER_PT_performance_acceleration_structure` exists in Blender 5.2 on both machines.

On the rented Linux machine the answer was `True` under every crossing except one:

```text
pref=OPTIX  scene_device=GPU cpu_device_ticked=False -> Acceleration Structure panel poll = False
pref=OPTIX  scene_device=GPU cpu_device_ticked=True  -> Acceleration Structure panel poll = True
```

So the panel goes away only when OptiX is chosen **and** the scene is set to the card **and** the processor is not ticked alongside it. Under `NONE`, `CUDA`, `HIP` and `ONEAPI` it is always present. On the Mac, under `NONE` and `METAL`, it is always present.

**The four versions answered identically.** That matters here: three of the four versions' documents quote a manual line about this panel and one does not, and this test shows that silence to be a gap in the documentation rather than a difference in Blender. One version difference showed up beside it and is not about this panel: 5.2.1 carries a `CYCLES_RENDER_PT_performance_texture_cache` panel that 4.5.12, 5.0.1 and 5.1.2 do not.

### 11. What travels with the file, and what a stored preference alone does

In Blender 5.2, on both machines: a `.blend` saved with the scene's own device set to `GPU`, opened with a configuration whose technology was set one way and then another, read back:

```text
OPENED WITH TICKED-OPTIX CONFIG: scene.cycles.device = GPU | preference = OPTIX
OPENED WITH NONE CONFIG: scene.cycles.device = GPU | preference = NONE
```

**The scene's own device setting travels in the file; the technology does not** — it belongs to the configuration folder. The Mac gave the same with `METAL`.

**Both parts are needed for a card render.** A saved preference naming the technology **with the card ticked in its device list**, plus the scene set to `GPU`, put the render on the card with no `--cycles-device` at all — under a second on Linux with the OptiX lines present, and the card-only line present on the Mac. The same preference with the scene set to `CPU` rendered on the processor. And a preference where the technology was set but **no device was ticked** did not put the render on the card at all: with the scene set to `GPU` it still rendered on the processor, in all four versions.

### 12. What the Preferences panel actually shows

The rented Linux machine only, with an X server (`Xvfb` at 1600 by 1000) and Mesa's software drawing (`llvmpipe (LLVM 15.0.7, 256 bits)`, OpenGL 4.5) installed for the purpose, and the screen photographed. **The Mac's own panel could not be photographed** — the only Apple machine this project has is the Creator's, who runs nothing, and a rented Linux machine cannot show Metal.

With OptiX chosen, the row under **Cycles Render Devices** reads **None, CUDA, OptiX, HIP, oneAPI** with OptiX highlighted, and below it two tick-boxes: `NVIDIA GeForce RTX 3080`, ticked, and `AMD EPYC 3151 4-Core Processor`, unticked. **No MetalRT, no HIP RT, no Embree on GPU, no Kernel Optimization and no Distribute Memory Across Devices control appears anywhere on that page.** With None chosen, the same five buttons appear with None highlighted and **the tick-box list below is gone entirely**. Below the Cycles panel sit **Display Graphics** with **Backend: OpenGL**, then Operating System Settings, Network, Memory & Limits and Video Sequencer. The photographs are kept at `Audits/Blender Research/What a render demands/Question 7 test procedure/`.

### 13. The drawing backend, and which values each operating system accepts

Blender 5.2, both machines, `--gpu-backend` given each of its three documented values on a background render.

On the **rented Linux machine**, `opengl` and `vulkan` were accepted and `metal` printed:

```text
Error: Unrecognized GPU backend for '--gpu-backend', expected one of [opengl, vulkan].
```

On the **Mac** it is the other way round: `metal` was accepted and both `opengl` and `vulkan` printed:

```text
Error: Unrecognized GPU backend for '--gpu-backend', expected one of [metal].
```

**In every one of those cases the render still finished and the frame was still saved, with exit status 0.** So a rejected backend name is reported and then ignored by a render with no window. All four versions behaved the same way on each machine.

### 14. `--cycles-print-stats` exists

`blender --help` in Blender 5.2, on both machines, lists under **Cycles Render Options**:

```text
--cycles-print-stats
	Log statistics about render memory and time usage.
```

Every run above that used it printed a `Render statistics:` block with per-object geometry memory, and on a card render the line `Profiling information not available (only works with CPU rendering)`.

### The noise floor measured, and three rows run again — 2026-08-30

**Label: observed.** Run on 2026-08-30 by Claude Opus 5 in Claude Code, **on the Creator's Mac and on no other computer**, in Blender 5.2.1 alongside the other three researched versions. These are entries 26 to 29 of `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md`, sent there by that topic's citation check on the same day because its own repeats of the tests above came back different in four places.

**What this can and cannot settle.** The Creator's Mac has **Apple's own graphics and no NVIDIA card**, and the rented Austrian machine behind the CUDA and OptiX figures above was destroyed on the day those tests were run. **So everything in this section settles macOS and Apple's own graphics, and says nothing whatever about an NVIDIA card.** Nothing was rented for it. Other agents had been running Blender on the same Mac all day, so **no time in this section is a clean one**; all four tests are about pixels and exit statuses rather than speed. Every one of about 900 Blender launches ran with a scratch configuration folder, so nothing of the Creator's own settings was read or written.

**Why it was worth running at all.** Finding 5 above rests entirely on a control: two runs of the *same* method should come back far closer together than two runs of *different* methods. Nobody had measured how wide that same-method spread really is — one pair had been run, not a spread — and **an answer whose noise floor is unknown cannot be weighed.**

#### The floor itself, over sixty pairs on the card and sixty on the processor

**What was done.** Each of the small, heavy mesh and heavy curve scenes was rendered **twice with the same method** — once on the processor, once on Metal — and the two compared by their pixels, in all four versions. **The whole set was repeated five times over**: 240 renders and 120 pairs, of which 15 card pairs and 15 processor pairs are Blender 5.2's.

**What a negative result would have looked like:** every Metal pair at 10⁻⁷ every time, which would have confirmed the floor exactly as recorded above.

**The processor. All sixty pairs across the four versions are identical to the pixel** — not one channel of 921,600 differs — and Blender 5.2's fifteen are among them; every one printed `VERDICT: IDENTICAL to the pixel`.

**The card, in Blender 5.2, the five repeats in order:**

| scene | two runs of Metal, repeats 1 to 5 |
|---|---|
| small | 3.58 × 10⁻⁷, 3.58 × 10⁻⁷, 4.77 × 10⁻⁷, 4.77 × 10⁻⁷, 4.77 × 10⁻⁷ |
| heavy mesh | 4.77 × 10⁻⁷ on all five |
| heavy curves | 3.58 × 10⁻⁷, 2.98 × 10⁻⁷, 2.98 × 10⁻⁷, 2.98 × 10⁻⁷, 2.98 × 10⁻⁷ |

**Not one of this version's fifteen card pairs left the 10⁻⁷ range.** Both excursions in the set of sixty were in other versions.

**Across all four versions, fifty-eight of the sixty card pairs sit between 2.98 × 10⁻⁷ and 5.96 × 10⁻⁷.** The two that do not are 5.310773849487305 × 10⁻⁵, in Blender 5.0.1 on the heavy mesh scene, and 0.018208861351013184, in Blender 4.5.12 on the curve scene. **Both were the first repeat of the set.** So **the floor on this machine is not one number**: it is 10⁻⁷ most of the time, and about one card pair in thirty came back four to five orders of magnitude higher.

#### What that makes of the differences recorded above

Only the Mac rows of finding 5 are touched here. The Linux rows are outside what this machine can say anything about.

| row, as finding 5 records it for Blender 5.2 | small | heavy mesh | heavy curves |
|---|---|---|---|
| worst two-runs-of-Metal figure measured here, this version | 4.77 × 10⁻⁷ | 4.77 × 10⁻⁷ | 3.58 × 10⁻⁷ |
| worst two-runs-of-Metal figure measured here, all four versions | 4.77 × 10⁻⁷ | 5.31 × 10⁻⁵ | **0.0182** |
| processor against Metal | 0.0103 | 0.0185 | 0.1199 |
| MetalRT off against on | 0.0103 | 0.0205 | 0.1425 |

**On the small scene and the heavy mesh scene those recorded differences sit comfortably above the floor** — from about 350 times it up to tens of thousands of times it. **On the curve scene they do not sit comfortably above it.** The worst same-method figure this machine produced on that scene, in any version, is 0.0182, and the recorded curve-scene differences of 0.1199 and 0.1425 are only about six to eight times that. They are still above it, and nineteen of the twenty curve-scene pairs were at 10⁻⁷ — but *orders of magnitude* is the wrong description of the curve-scene rows, and *a factor of six to eight against the worst case* is the right one.

**Both runs stand.** Finding 5's own control figures above are untouched, this run's are here, and neither has been chosen over the other.

#### MetalRT Auto against MetalRT Off, three hundred comparisons

**What was done.** Each of the three scenes was rendered **five times with MetalRT off and five times with MetalRT auto**, in all four versions, and **every off was compared against every auto** — 25 comparisons per scene per version, 300 in all, of which 75 are Blender 5.2's.

**What a negative result would have looked like:** every pair at the noise floor every time. That is what came back, and it could have gone the other way — the same procedure produced figures of 0.012 and 0.018 twice before, once in the record above and once in the citation check's repeat.

**Every one of the 120 renders exited 0 and printed the setting it had been given**, sixty reading off and sixty reading auto. **And what Blender printed while it built was the same line in all 120**, in every version and on every scene:

```text
Packing BVH nodes; Packing BVH triangles and strands
```

So **auto built the same structure as off in all 120 runs**, which is what finding 7 above already says.

**The pixels, in Blender 5.2, as the range over the twenty-five comparisons in each cell:**

| scene | MetalRT off against auto, twenty-five comparisons |
|---|---|
| small | 3.58 × 10⁻⁷ to 7.15 × 10⁻⁷ |
| heavy mesh | 4.77 × 10⁻⁷ to 5.96 × 10⁻⁷ |
| heavy curves | 2.38 × 10⁻⁷ to 3.58 × 10⁻⁷ |

Across all four versions **not one of the 300 comparisons is above 7.15 × 10⁻⁷**. **This version's recorded figure for that row is 5.30 × 10⁻⁵ on the heavy mesh scene. It did not come back in twenty-five tries.**

**Both runs stand.** Finding 5's off-against-auto row for Blender 5.2 — 4.77 × 10⁻⁷, 5.30 × 10⁻⁵ and 3.58 × 10⁻⁷ — is untouched, and so is the citation check's note beside it.

**One plain thing about the numbers, offered as arithmetic and not as an answer.** The figures recorded for that row which are above the floor are the same size as the two same-method excursions measured above, and two of them fall on the same scene in the same version as an excursion. **Nothing run here establishes what that means**, and this section does not say which reading is the right one.

#### Kernel Optimization, thirty-six renders and a hundred and forty-four comparisons

**What was done.** The heavy mesh scene rendered with Kernel Optimization off, intersection only, and full — **three times each**, in all four versions — and **every pair of the nine images in a version compared**, which is 36 pairs a version. The nine include three same-setting pairs, so the floor for this scene is measured inside the same test.

**What a negative result would have looked like:** all pairs at the noise floor, which is what Blender 5.1's own record says and what the other three versions' records deny.

**All 36 renders exited 0 and every one printed the setting it had been given. In Blender 5.2:**

| pair | range over the nine, or the three |
|---|---|
| off against full | 0.0124985 |
| off against intersection only | 4.77 × 10⁻⁷ to 5.96 × 10⁻⁷ |
| full against intersection only | 0.0124985 |
| two runs of the same setting | 4.77 × 10⁻⁷ to 5.96 × 10⁻⁷ |

**That agrees with this version's own record**, which has Full against Off at 0.0125 and Intersection only against Off at the floor.

**Across the four versions the split is clean and it is the same split the records above have**: 4.5.12, 5.0.1 and 5.2.1 gave 0.0125 for off against full in all nine pairs each, and 5.1.2 gave the noise floor in all nine.

**Times, on a machine that was not quiet**, as Cycles' own `Time:` line for the three runs of each setting in Blender 5.2: off 00:05.72, 00:05.63, 00:05.76; intersection only 00:05.27, 00:05.24, 00:05.21; full 00:04.74, 00:04.70, 00:04.71. **Off was the slowest in all four versions**, which is the shape both earlier runs recorded and the one part of this test nobody disputes.

**Both runs stand.** Finding 8's figures above are untouched, and so is the citation check's note where it left one.

#### The 4.5.12 crossing that failed twice, run ten times

**What was done.** Blender **4.5.12** wrote its own copy of the small scene with the scene's device on the processor, and no newer Blender ever opened it; 4.5.12 then saved a scratch preference folder with the technology on `NONE`, which was read back to prove it had taken. The crossing "stored technology `NONE`, scene device processor, `-- --cycles-device METAL`" was then run **ten times**, with five minutes left between the first and the second, because both recorded failures had been the first run after a gap.

**What a negative result would have looked like:** ten clean runs, which would say the failure is rarer than one in six or has some cause this set-up does not reproduce.

**All ten exited 0, all ten wrote their frame, and all ten printed a `Saved:` line and a `Time:` line.** Wall times ran from 1.70 to 2.04 seconds and Cycles' own times from 00:01.20 to 00:01.55.

**Both runs stand.** The record above of two failures in six runs with `Abort trap: 6` and exit status 134 is untouched. What is now added is that the same crossing, set up the same way on the same Mac, has been run seventeen times across three agents and has failed only in the two already recorded. **This is a Blender 4.5.12 matter and it is written into every version's document because the section it belongs to is.**

**The whole output of all four tests, word for word, with the scripts that produced it, is at `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions — raw output/Mac 2026-08-30/`.**

---

### What could not be run, and why

- **Anything about HIP in use, and the HIP RT switch in use.** HIP is AMD's, and this project has reached Apple's own graphics and NVIDIA cards and no card of any other make. `use_hiprt` can be read and set from a script — finding 2 does that — but no render here can use HIP.
- **Anything about oneAPI, the Embree-on-GPU switch, and what oneAPI does when a card's memory fills.** oneAPI needs an Intel Arc card, which this project has never had.
- **Whether a hardware ray-tracing control greys out when a card that cannot do it is enabled beside one that can.** This needs two cards of different generations in one machine, one with ray-tracing hardware and one without. Every card on every multi-card machine this project has rented could do hardware ray tracing.
- **Distribute Memory Across Devices, and anything about NVLink.** The manual ties that switch to cards joined by NVLink. **This project has never had NVLink-joined cards.** The switch can be read and set from a script; what it does to a render cannot be asked.
- **How the MetalRT and Kernel Optimization controls are drawn, and whether they are greyed.** The only Apple machine is the Creator's, and the Creator runs nothing. What those two settings *do* is answered in findings 7 and 8; what is missing is only how they look.
- **Whether Metal is offered on a Mac that is not Apple silicon.** This project has one Mac and it has an M1 Pro chip.
- **Anything about a rendering method on Windows.** This project's route to a Windows computer is a Kamatera rental, and those machines have two general-purpose cores and **no graphics card at all**. Four card-free Windows tests — what the row of methods holds there, what a render does when told to use a method the machine has no hardware for, whether the switches can be set from a script, and whether the Acceleration Structure panel behaves the same — are written out ready to run in `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md` and wait on the next Windows batch.

### One thing that happened twice and is recorded rather than explained

In **Blender 4.5.12 on the Creator's Mac only**, the crossing "stored technology `NONE`, scene device `CPU`, `-- --cycles-device METAL`" ended twice with `Abort trap: 6` and exit status 134, having printed as far as `Loading render kernels (may take a few minutes the first time)` and written no file. It was run six times in that version; the other four completed in about 1.7 seconds. Both failures were the first run of that crossing after a gap. The same crossing never failed in 5.0.1, 5.1.2 or 5.2.1, and the neighbouring crossing with the scene's device on `GPU` never failed in any version. **The second failure is not a clean observation**: the scene file it opened had been re-saved by 5.2.1 a moment before, and Blender said so — `Warning: File written by newer Blender binary (502.44), expect loss of data!`. The first was on a file 4.5.12 had written itself.


*(Run again on 2026-08-30, ten times, and it did not fail once: see **The noise floor measured, and three rows run again** below. Nothing above was changed.)*

**The full list, every command run, and the raw output are at `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md`, with the scripts beside it.**

### Outside the interface

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Four things that were open here are settled by test, in all four versions and on both computers. **`--cycles-device` does outrank both the stored preference and the scene's own setting**, in both directions. **A stored preference is applied by a Blender started with no window**, but only puts the render on the card when the technology's own device is ticked in the preference *and* the scene is set to the card; a preference naming the technology with no device ticked renders on the processor. **All six Cycles preference settings can be read and set from a script with no window** — the technology, MetalRT, HIP RT, Embree on GPU, Kernel Optimization and Distribute Memory Across Devices — and MetalRT and Kernel Optimization were then shown to change the render, so the assignment is not merely accepted. **A method the machine cannot do, or a name that is not a method, ends the run** with exit status 1 and no file written; nothing falls back. What is unchanged: there is still no command-line option for the three hardware ray-tracing switches.]*

**LABEL: documented for the command-line switch; observed for the rest; not in the sources for the gaps.**

**The technology can be set from outside, and the manual's own example does exactly that in background mode:** `blender -b file.blend -f 20 -- --cycles-device OPTIX` (Manual, Command Line Arguments). The add-on's help text says this "overrid[es] user preferences and the scene setting" (`cycles_5.2/engine.py`, line 17) — **observed**. That is the clearest point in this whole answer: the choice of technology does not have to be made by a person clicking.

**The scene's CPU/GPU Compute setting sits on the scene's own Cycles settings.** The `device` property quoted above is defined on `CyclesRenderSettings` (`cycles_5.2/properties.py`, line 387) — **observed**. **Inferred**, one step: because it is a property of the scene rather than of the add-on's preferences, it is saved in the blend-file and travels with it. The step a reader has to accept is that a scene property is written into the file when the file is saved; the files I read show which class the property is on, and never say where such a property is stored. Test 3 in the list at the end is the direct check.

**The technology and the three ray-tracing switches are not stored in the file.** They are properties on `class CyclesPreferences(bpy.types.AddonPreferences)` (`cycles_5.2/properties.py`, line 1625) — **observed** — and the manual calls the technology "the Cycles Render Devices user preference" (Manual, GPU Rendering) — **documented**. **Inferred**, one step: because they live on the add-on's preferences rather than on the scene, they belong to the machine and user account Blender is running as, not to the blend-file, so a file carried to another machine does not carry them. The step a reader has to accept is that add-on preferences are stored with user preferences rather than in the file; the sources I was given show the class they are on but do not state where that class is saved.

**They can be reached from Python.** The add-on reads the technology this way:

```
def get_device_type(context):
    return context.preferences.addons[__package__].preferences.compute_device_type
```

— `cycles_5.2/ui.py`, lines 96–97 — read 2026-07-30 — **observed**. **Inferred**, one step: `metalrt`, `use_hiprt`, `use_oneapirt`, `peer_memory` and `kernel_optimization_level` are defined on the same class, so the same path reaches them under their own names. The step is that all properties of one class are reachable through the same object; the file shows the class and the path, but never spells out those five.

**Where it differs from inside the interface, and where the sources go quiet:**

- There is a command-line switch for the technology and none for MetalRT, HIP RT, Embree on GPU, Distribute Memory Across Devices or Kernel Optimization. I searched the whole of the 5.2 command-line arguments page for each of those names and for "metalrt", "hiprt", "embree", "peer", "optimization" and found nothing. **Not in the sources** that they can be set by a launch argument.
- Which technologies appear as options comes from a call to `_cycles.get_device_types()` (`cycles_5.2/properties.py`, line 1639, quoted in 7a) — **observed**. **Inferred**, one step: the same file launched on two machines may therefore face different options, whether driven by a person or not. The step a reader has to accept is that the call answers differently on different machines; the file shows the list is built from that call and never says what the call returns, nor whether anything calls it when no preferences panel is drawn.
- The three ray-tracing rows are drawn with `row.active = has_hardware_rt` (`cycles_5.2/properties.py`, lines 1993, 1998, 2003) — **observed**. Greying out a row is an interface effect; whether the stored value still takes effect in a render started from outside is a step past what the file says, and I do not take it.
- The add-on's device list refresh carries this comment: "# Refresh device list. This does not happen automatically on Blender startup due to unstable drivers that can cause crashes." (`cycles_5.2/properties.py`, lines 1763–1764) — **observed**. What that means for a render started from outside the interface, where no preferences panel is ever drawn, is **not in the sources**.
- The manual's warnings about the interface being unresponsive during rendering, and the Windows display-driver timeout, are framed around a person watching a window. **Not in the sources** whether they apply the same way with no window.
- **For the second reading of the question — the graphics API of 7e — there is a launch argument, `--gpu-backend`,** quoted in 7e from the command-line arguments page. So that setting too can be given from outside. **Not in the sources** what it does when Blender is started with no window at all, since the manual scopes the setting to drawing the interface.

---

### Operating system

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Tested rather than inferred: the Creator's Mac offers only `None` and `Metal` and starts at `Metal`; a rented Linux machine with one NVIDIA card offers `None`, `CUDA`, `OptiX`, `HIP` and `oneAPI` and starts at `None`. So the list of methods does differ by operating system — **but it is built from what that build supports rather than from the cards present**, since the Linux machine offered HIP and oneAPI with no AMD and no Intel graphics in it. `--gpu-backend` accepts `opengl` and `vulkan` on Linux and only `metal` on macOS, naming the rejected value in an error and then finishing the render anyway. **Windows is not settled**: this project's route to a Windows computer is a rental with two general-purpose cores and no graphics card at all, so four card-free Windows tests are written out ready to run and wait on the next Windows batch.]*

**LABEL: documented.**

This is one of the places where the answer genuinely does not hold everywhere — the available methods are decided by the operating system.

- **CUDA:** "supported on Windows and Linux" — not macOS.
- **OptiX:** "supported on Windows and Linux" — not macOS.
- **HIP:** "supported on Windows and Linux". Its minimum drivers differ by system: "Windows: Radeon Software 24.9.1 or Radeon PRO Software 24.Q4 / Linux: Radeon Software 24.30 or ROCm HIP Runtime 6.3".
- **oneAPI:** "supported on Windows and Linux", again with different minimum drivers per system.
- **Metal:** "supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

— all: Manual, "GPU Rendering", https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30

So on macOS the only graphics-card technology documented is Metal; on Windows and Linux, Metal is not offered and the other four may be.

Further differences the sources state:

- **HIP denoising differs between Windows and Linux:** "GPU accelerated denoising is available on discrete Radeon RX 6000 and newer GPUs on Linux, and discrete Radeon RX 7000 and newer GPUs on Windows." (Manual, GPU Rendering.)
- **MetalRT and Kernel Optimization are Apple-Silicon-only**, and the add-on file checks the machine before showing them: `# MetalRT only works on Apple Silicon.` followed by `if (platform.machine() == 'arm64'):` — `cycles_5.2/properties.py`, lines 1987–1988 — **observed**. The default technology is also chosen by platform: `if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'): return 5` (Metal) — lines 1631–1634 — **observed**.
- **The debug panel shows different per-technology options by platform:** `is_macos = platform.system() == 'Darwin'`, then Metal's adaptive compile on macOS, and CUDA/OptiX/HIP options otherwise — `cycles_5.2/ui.py`, lines 2294–2311 — **observed**.
- **The "Unsupported GNU version" error and its two workarounds are Linux-only**: "On Linux, depending on your GCC version you might get this error." (Manual, GPU Rendering.)
- **The display-driver timeout problem is Windows-only**: "If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations." (Manual, GPU Rendering.)

**For the second reading — the graphics API of 7e — the sources say almost nothing about the operating system.** The preferences page names only OpenGL and Vulkan; the command-line list of valid options also names `metal`, which is Apple's own and appears nowhere on the preferences page. Nothing I found says which of the three is available on which system, or which is used when none is named. My search for `vulkan` across the whole of the 5.2 material is in the table below, and turned up nothing that says it. **Not in the sources.**

**Not in the sources:** whether the same technology behaves identically on Windows and Linux beyond the driver minimums and the HIP denoising line above. The manual lists requirements per system but nowhere compares results between systems.

---

### The record of where I looked

Everything below was run against the 5.2 folders only.

**Searches across the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2`** (case-insensitive), and what each turned up:

| Term searched | Outcome |
|---|---|
| `optix` | 11 manual files, 1 release-notes file. All read. |
| `metalrt`, `hiprt`, `hip_rt`, `hip rt`, `embree on gpu`, `embree_gpu`, `oneapirt` | Manual: only the System preferences page. Cycles files: `properties.py` (the settings), plus kernel headers. Release notes: three fixed-bug lines. |
| `compute_device_type` | API docs: nothing except `PreferencesSystem.legacy_compute_device_type`, which reads in full "For backwards compatibility only (in [-inf, inf], default 0, readonly)" — `api_5.2/bpy.types.PreferencesSystem.txt`, lines 205–213. Ruled out: it is marked backwards-compatibility-only and read-only. Cycles files: `properties.py` and `ui.py`. |
| `optix`, `oneapi`, `HIP` across `api_5.2` | **No matches at all.** This is the basis for saying the API documentation does not cover the technology choice. It is consistent with Cycles being an add-on: its preference class is registered by the add-on and does not appear in the bundled API reference. |
| `GPU Compute` | Four manual hits, all read. |
| `backend` | 24 manual hits, all read or classified. |
| `vulkan` | Command-line arguments, System preferences, colour-management system configuration, one Linux troubleshooting note, and a run of fixed-bug lines. |
| `scene dependent`, `scene-dependent`, `depending on the scene`, `scene complexity`, `complex scenes`, `large scenes`, `size of the scene`, `scene size` | 19 hits across manual and release notes. Every one checked; the ones that bear on this question are quoted in 7d. The rest were ruled out as unrelated: video-sequencer and compositor "Scene Size" fields, viewport overlays, EEVEE light-probe volumes, the Simplify page, fluid caching, the Array modifier, collections, and a 3D-view clipping tip. |
| `hardware ray`, `ray-tracing acceleration`, `intersection method`, `raytracing method`, `ray tracing method` | Led to the System preferences and GPU Rendering pages already covered, plus EEVEE's own Method setting and the Bevel modifier's intersection options. |
| `cycles-device`, `cycles_device`, `argv` in the Cycles Python files | `engine.py` only. |

**Manual pages opened in full or in the relevant part:**

- `render/cycles/gpu_rendering.txt` — read end to end. This is the central page.
- `editors/preferences/system.txt` — read end to end.
- `render/cycles/render_settings/index.txt` — Device and OSL sections.
- `render/cycles/render_settings/performance.txt` — Acceleration Structure, Final Render, Viewport, Compositor sections.
- `render/cycles/render_settings/sampling.txt` — Denoising section, including the denoiser list and Use GPU.
- `render/cycles/osl/index.txt` and `render/cycles/osl/camera.txt` — the OptiX limitation notes.
- `render/shader_nodes/input/bevel.txt` and `render/shader_nodes/input/ao.txt` — both carry the line "Open Shading Language is active while using the OptiX rendering backend." Noted, not used further; they are about two specific nodes.
- `advanced/command_line/arguments.txt` — the Cycles Render Options section and the GPU Options section.
- `render/eevee/render_settings/raytracing.txt` — read the Method section. **Ruled out** as the answer: its two options are "Light Probe" ("Use light-probe spheres and planes to find scene intersection") and "Screen-Trace" ("Trace ray against the screen depth buffer. Fallback to light-probes if ray exits the view"). Those are lighting algorithms inside EEVEE, not a choice between a graphics card's rendering technologies.

**Image opened:** `manual_5.2/_images/editors_preferences_section_system.webp`, described in 7a. It is the only image referenced by either of the two central pages — I checked, and `render/cycles/gpu_rendering.txt` contains no `[image: …]` line at all, and `render/cycles/render_settings/performance.txt` contains none either.

**Release notes read in full:** `cycles.md` and `rendering.md` for 5.2. Neither mentions a change to which GPU technologies exist or how they are chosen; `cycles.md` covers a texture cache, raycast attributes and shader nodes. `bugfixes.md` searched for the terms above. `index.md`, `core.md`, `eevee.md` and the rest were not opened; I searched all of `release_notes_5.2` for the terms in the table above, which covers them.

**Cycles files read:** `engine.py` (all 60 lines); `properties.py` — lines 30–60 (device and BVH enums), 172–200 (device type enum), 380–400 (scene device), 925–960 (BVH debug properties), 1090–1135 (debug properties), 1600–1720 (`CyclesDeviceSettings` and the start of `CyclesPreferences`), 1740–1840 (device-list helpers), 1940–2005 (`draw_impl`); `ui.py` — lines 90–190 (device helpers), 2270–2320 (the debug panel); `source/kernel/types.h` lines 1290–1320 (the BVH layout enum). Listed the whole `cycles_5.2` folder to confirm what is present: `__init__.py`, `camera.py`, `engine.py`, `maketx.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`, plus `license/`, `shader/` and `source/`. **`source/` contains only `kernel/` and `util/`** — there is no device implementation directory, so the code that would decide what MetalRT's "Auto" actually picks, or how a technology is chosen at run time, is not in what I was given. That is the concrete reason 7d's second reading is "not in the sources" rather than answerable.

**Ruled out, with reasons:**

- `debug_bvh_layout` (`cycles_5.2/properties.py`, lines 1101–1105, offering only `BVH2` and `EMBREE`, default `EMBREE`) — **observed** — is drawn in the debug panel under the heading "CPU" (`cycles_5.2/ui.py`, lines 2286–2292). It is a CPU-side structure choice, not a graphics card's rendering method.
- `PreferencesSystem.legacy_compute_device_type` in the API docs — marked "For backwards compatibility only" and read-only.
- The compositor's own Device and Denoising Device settings (`render/cycles/render_settings/performance.txt`, Compositor section) — a separate device choice for compositing, which the Render Settings page itself flags as separate: "Denoising and Compositing have their own device selection."
- The texture-cache material in `release_notes_5.2/cycles.md` — its "Memory savings and performance are highly scene dependent" line is about the texture cache, a different feature, not about the choice between graphics-card methods.

**I did not use the web, and did not need to.** Every claim above comes from a file inside `Blender Sources/`.

---

## Things a running Blender could still settle

*(Pruned 2026-08-30 by the hand tests. Everything the earlier list held has either been run — the results are above, under **Tested by hand — what a running Blender showed** — or is recorded there as unable to be run, with its reason. What follows is what is genuinely still open.)*

1. **What happens when the card's own memory fills.** The scene of 125,829,122 triangles reached the host machine's memory ceiling and not the card's, so the documented fallback to system memory was never exercised. Settling it needs a machine whose card is small against what its host can build, which is the opposite of the usual want.
2. **What MetalRT's "Auto" weighs.** On the Creator's Mac it built the same structure as Off on all three scenes, including one of 300,000 curves — in the tests above, and again in sixty more runs on 2026-08-30, in all four versions, without one exception. Whether it ever picks the other way — on other Apple hardware, or on some other kind of scene — is still not settled.
3. **Whether the four card-free Windows tests give the same answers.** They are written out ready to run in `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md` and wait on the next Windows rental.
4. **Everything needing hardware nobody here has**: an AMD card, an Intel Arc card, two cards of different ray-tracing generations in one machine, NVLink-joined cards, a Mac that is not Apple silicon, or an Apple machine an agent may open a window on. Each is named above with its reason.

---

2026-08-29 — Claude Opus 5 (Claude Code) — What a render demands, Blender 5.2

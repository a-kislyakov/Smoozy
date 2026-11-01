# What a render demands — Open Threads questions (Blender 5.0)

Sources consulted: the Blender 5.0 manual, the Blender 5.0 Python API documentation, the Blender 5.0 release notes, and Cycles' own files, all in `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is the date read carried by every citation below; the reading and the writing were done on 2026-08-29.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

This is a first pass. It is not a re-answer of an earlier findings document, and no earlier findings document covers this question.

Only question 7 was assigned. It is answered below.

---

## 7. Where a render can be told which of a graphics card's rendering methods to use, what methods are there, what does choosing one rather than another change about the render, and does anything establish whether that changes with the size of the scene?

### Note on the wording

"A graphics card's rendering methods" has more than one reading in Blender 5.0. I answer the reading I believe is meant — the named compute backends Cycles can drive a graphics card through — and I say plainly what the other readings are and cover them, because they are close enough that a reader could mean either:

- **Reading A (answered as the main answer):** the compute backend Cycles uses — CUDA, OptiX, HIP, oneAPI, Metal.
- **Reading B (covered in part 3):** the ray-tracing method *within* a backend — MetalRT, HIP RT, Embree on GPU.
- **Reading C (covered in part 4):** the graphics API Blender draws and displays through — OpenGL, Vulkan, Metal.

---

### Part 1 — Where the choice is made, and what the methods are

**LABEL: documented**

The methods are named in the manual, in two places.

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal
> in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

The same page heads the list "Rendering Technologies" and says:

> "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer
> and operating system."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

The place where the method is picked is the System section of Preferences:

> "Cycles Render Devices
>
> Changes the Computing Device
> the Cycles render engine uses to render images."

with the options listed one by one:

> "None:
>
> When set to None or when the only option is None:
> the CPU will be used as the computing device for Cycles.
>
> CUDA:
>
> If the system has a compatible NVIDIA CUDA device, it will be available as an option for rendering with Cycles.
>
> OptiX:
>
> If the system has a compatible NVIDIA OptiX device, it will be available as an option for rendering with Cycles.
>
> HIP:
>
> If the system has a compatible AMD HIP device, it will be available as an option for rendering with Cycles.
>
> oneAPI:
>
> If the system has a compatible Intel oneAPI device, it will be available as an option for rendering with Cycles.
>
> Metal:
>
> If the system has a compatible Apple Metal device, it will be available as an option for rendering with Cycles."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30.

**The picture on that page.** The manual shows `[image: _images/editors_preferences_section_system.webp]`, captioned "Preferences System section." I opened it. It shows a panel headed "Cycles Render Devices" with a single row of three side-by-side buttons — "None", "CUDA", "HIP" — with "None" highlighted. Below it a panel "Display Graphics" with a single dropdown labelled "Backend" reading "OpenGL". This is **documented**, being part of the manual. What is visible: the choice is one-of-several, presented as a row of buttons, and only three of the six named options appear in that picture — None, CUDA and HIP. What is not visible: why only three appear, and which appear anywhere else. (That the list shown depends on the machine is a separate finding, from Cycles' own files below, not something this picture shows.)

**Where the choice does not live.** The per-scene render setting is a separate, coarser choice — CPU or GPU, not which backend:

> "Device
>
> Specifies the compute device used for rendering in Cycles.
> The choice of device can significantly affect render performance, GPU Compute is often faster .
>
> CPU:
>
> Render using the system's central processing unit. This mode is compatible with all Cycles features.
> The number of CPU threads used can be changed in the
> Performance Properties.
>
> GPU Compute:
>
> Render using one or more graphics processing units (GPUs) through the compute backend
> and devices selected in the Cycles Render Devices.
> Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html — read 2026-07-30.

**Confirmation in Cycles' own files (observed).** The backend choice is declared as a preferences property, and Cycles' files carry a separate two-value list, CPU and GPU Compute. The property that uses that second list is not quoted here; that it is the scene's own Device setting comes from the manual quote above, not from this file:

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
File `Blender Sources/cycles_5.0/properties.py`, lines 172–179.

```
enum_devices = (
    ('CPU', "CPU", "Use CPU for rendering"),
    ('GPU', "GPU Compute",
        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
)
```
File `Blender Sources/cycles_5.0/properties.py`, lines 30–33.

```
class CyclesPreferences(bpy.types.AddonPreferences):
```
…
```
    compute_device_type: EnumProperty(
        name="Compute Device Type",
        description="Device to use for computation (rendering with Cycles)",
        default=CyclesPreferences.default_device(),
        items=CyclesPreferences.get_device_types,
    )
```
File `Blender Sources/cycles_5.0/properties.py`, line 1569 and lines 1599–1604.

The list offered is built at the moment it is shown, from what the running build of Blender reports: *(Citation check, 2026-08-30: **this was tested and the wording is wrong.** Finding 1 under "Tested by hand" records a machine with one NVIDIA card, no AMD card and no Intel graphics still offering HIP and oneAPI. The list is built from what that build of Blender supports, not from what the machine holds; what is empty is the device list underneath each choice. Corrected by this check.)*

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
File `Blender Sources/cycles_5.0/properties.py`, lines 1581–1597.

And the default is machine-dependent:

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
File `Blender Sources/cycles_5.0/properties.py`, lines 1572–1579.

Read 2026-07-30.

**Told from outside Blender.** The method can be named on the command line:

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU.
>
> Example:
>
> blender -b file.blend -f 20 -- --cycles-device OPTIX"

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

Cycles' own files state what that option does to the two other places the choice could come from (**observed**):

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```
File `Blender Sources/cycles_5.0/engine.py`, lines 16–20. The parsing only happens after a bare `--` on the command line:

```
    argv = sys.argv
    if "--" not in argv:
        return
```
File `Blender Sources/cycles_5.0/engine.py`, lines 27–29. Read 2026-07-30.

**Not covered by Part 1:** whether the chosen backend is stored anywhere inside a `.blend` file (see the "Outside the interface" line below); what happens when a named backend is not present on the machine; how "+CPU" hybrid rendering behaves beyond the one sentence quoted.

---

### Part 2 — What choosing one rather than another changes about the render

The sources say four different kinds of thing here. I separate them because they have different strengths.

#### 2a. Which hardware can be used at all — documented

> "CUDA is supported on Windows and Linux and requires a
> NVIDIA graphics cards with compute capability 5.0 and higher."

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher
> and a driver version of at least 535."

> "HIP is supported on Windows and Linux and requires a
> AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."

> "oneAPI is a computation library that is supported on Windows and Linux and requires a
> Intel® Arc™ graphics card with the Xe HPG architecture.
> Hardware acceleration for ray-tracing and denoising is supported."

> "Metal is supported on Apple computers with Apple Silicon.
> macOS 13.0 or newer is required to support all features."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

#### 2b. Speed — documented, but not quantified

> "OptiX takes advantage of hardware ray-tracing acceleration in RTX graphics cards, for improved performance."

> "What renders faster?
>
> This varies depending on the hardware used. Different technologies also have different compute times
> depending on the scene tested. For the most up to date information on the performance of different devices,
> browse the Blender Open Data resource."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

That second quote is as close as the sources come to a general statement about which method is faster, and it declines to give one.

#### 2c. Which features work — documented

> "Limitations
>
> Path Guiding is not supported on any GPU.
>
> Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

The same restriction is stated on the render settings page:

> "OSL is supported only when rendering with the CPU or with the
> OptiX GPU Compute backend."

and, generally:

> "GPU rendering may have limitations compared to CPU rendering, depending on the backend."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html — read 2026-07-30.

Denoising availability also differs by method:

> "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes
> all NVIDIA RTX cards." (CUDA/OptiX section)

> "GPU accelerated denoising is available on discrete Radeon RX 6000 and Radeon RX 7000 GPUs." (HIP section)

> "GPU accelerated denoising is available on all supported GPUs." (oneAPI section)

> "GPU accelerated denoising is available on Apple Silicon." (Metal section)

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

And one denoiser is named after a method:

> "OptiX:
>
> Uses NVIDIA's OptiX AI denoiser.
> Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.
>
> Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30.

#### 2d. What happens when the scene does not fit in the card's memory — documented, and this is the one that names methods unequally

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
> try to use system memory. This has a performance impact, but will usually still result in a faster render
> than using CPU rendering."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

Four of the five methods are named. oneAPI is not. **Whether oneAPI therefore lacks that fallback is inferred, not documented** — the step is: the sentence enumerates four backends by name where five exist, so the fifth is not covered by the sentence. The sources nowhere say outright that oneAPI cannot fall back to system memory, and an omission is not a denial.

#### 2e. What the render *produces* — not in the sources

Nothing I found says the chosen method changes the pixels. The sources describe the choice in terms of supported hardware, speed, feature availability and memory behaviour, never in terms of a different image. I searched the manual and API documentation for `optix`, `cuda`, `HIP`, `oneapi`, `Metal`, `hardware ray`, `rasteriz`, `BVH`, and read the whole of the GPU Rendering page, the System preferences page, the Cycles render settings index and sampling pages. None states that output differs by backend, and none states that output is identical either.

**Not covered by Part 2:** any figure for how much faster one method is than another; whether the image produced is bit-for-bit the same across methods; how the "+CPU" hybrid mode divides work.

---

### Part 3 — Reading B: the ray-tracing method within a backend

Three further switches sit beside the five backends. The manual ties two of them to one backend in so many words — Embree on GPU to oneAPI, HIP RT to HIP. For MetalRT it says no such thing in the passage quoted below; that it belongs to Metal is not documented there.

**LABEL: documented**

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
> MetalRT for ray tracing uses less memory for scenes which use curves extensively,
> and can give better performance in specific cases.
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

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30.

The AMD section of the GPU page adds:

> "Hardware ray-tracing support is available with the most recent drivers.
> This can be enabled in the preferences, and is supported on Radeon RX 6000 and newer."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

**LABEL: observed** — the same three switches, with their defaults, in Cycles' own files:

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
        default=False,
    )

    use_oneapirt: BoolProperty(
        name="Embree on GPU",
        description="Embree on GPU enables the use of hardware ray tracing on Intel GPUs, providing better overall performance",
        default=True,
    )
```
File `Blender Sources/cycles_5.0/properties.py`, lines 1614–1636. Read 2026-07-30.

Defaults, exactly: MetalRT `AUTO`, HIP RT off, Embree on GPU on.

In the same file, each of those three properties is written inside a test on which backend is chosen, and the MetalRT one inside a further test on the processor:

```
        if compute_device_type == 'METAL':
            import platform

            # MetalRT only works on Apple Silicon.
            if (platform.machine() == 'arm64'):
```
…
```
        if compute_device_type == 'HIP':
            row = layout.row()
            row.active = has_hardware_rt
            row.prop(self, "use_hiprt")

        elif compute_device_type == 'ONEAPI' and _cycles.with_embree_gpu:
```
File `Blender Sources/cycles_5.0/properties.py`, lines 1915–1935. The same block sets:

```
        # Any device without RT support will disable it for all.
        has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt
```
File `Blender Sources/cycles_5.0/properties.py`, line 1909. Read 2026-07-30.

**What those file lines carry, and what they do not.** Read as text, they show that the three properties are written inside those tests. **That each switch therefore only appears on screen for its own backend, and MetalRT only on Apple Silicon, is inferred, not observed** — the step a reader must supply is that this block is what puts those controls on the screen and is the only place they are put there. Nothing quoted here says that, and nothing here was run. Whether the controls actually appear that way is on the hand-test list below, item 5.

**LABEL: observed** — Cycles' own files include a different ray-tracing header depending on which of four names is defined:

```
/* Device specific acceleration structures for ray tracing. */

#if defined(__EMBREE__)
#  include "kernel/device/cpu/bvh.h"
#  define __BVH2__
#elif defined(__METALRT__)
#  include "kernel/device/metal/bvh.h"
#elif defined(__KERNEL_OPTIX__)
#  include "kernel/device/optix/bvh.h"
#elif defined(__HIPRT__)
#  include "kernel/device/hiprt/bvh.h"
#else
#  define __BVH2__
#endif
```
File `Blender Sources/cycles_5.0/source/kernel/bvh/bvh.h`, lines 17–30, with the fallback described a few lines later:

```
/* BVH2
 *
 * Bounding volume hierarchy for ray tracing, when no native acceleration
 * structure is available for the device.
```
File `Blender Sources/cycles_5.0/source/kernel/bvh/bvh.h`, lines 58–61. Read 2026-07-30.

What the file text carries: four names are tested in turn, each pulls in a different ray-tracing header, and where none of them is defined a name meaning Cycles' own BVH2 is set instead. The comment beside BVH2 says in the file's own words that it is used "when no native acceleration structure is available for the device".

**Reading that as a statement about a render is inferred, not observed.** The step: the four names would have to line up with the four methods a render can be told to use, and a header being included would have to mean the running render uses what is in it. Neither is stated in the file, and nothing here was run. So this establishes that Cycles' files hold more than one ray-tracing arrangement and choose between them by name; it does not establish what any render does with them. What it does not show at all: how fast any of them is, or what a render looks like as a result.

There is also a compilation switch (**observed**). Nothing quoted in this document ties it to any one backend — not the words below, and not the drawing block above, which is quoted only in part:

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
File `Blender Sources/cycles_5.0/properties.py`, lines 1638–1651. Read 2026-07-30. I did not find this one described in the manual (searched the System preferences page and the GPU Rendering page for "Kernel", "optimization", "optimized").

And backend-specific debug switches (**observed**):

```
    adaptive_compile_description = "Compile the Cycles GPU kernel with only the feature set required for the current scene"

    debug_use_cuda_adaptive_compile: BoolProperty(
        name="Adaptive Compile",
        description=adaptive_compile_description,
        default=False)

    debug_use_optix_debug: BoolProperty(
        name="OptiX Module Debug",
        description="Load OptiX module in debug mode: lower logging verbosity level, enable validations, and lower optimization level",
        default=False)

    debug_use_hip_adaptive_compile: BoolProperty(
        name="Adaptive Compile",
        description=adaptive_compile_description,
        default=False)

    debug_use_metal_adaptive_compile: BoolProperty(
        name="Adaptive Compile",
        description=adaptive_compile_description,
        default=False)
```
File `Blender Sources/cycles_5.0/properties.py`, lines 1079–1099. These sit on the scene's Cycles settings under a comment "# Various fine-tuning debug flags" (line 1064). Note there is an "Adaptive Compile" for CUDA, HIP and Metal, and none for oneAPI.

There is one more, and unlike the three above it is not written inside a test on any backend (**observed**):

```
enum_bvh_layouts = (
    ('BVH2', "BVH2", "", 1),
    ('EMBREE', "Embree", "", 4),
)
```
File `Blender Sources/cycles_5.0/properties.py`, lines 36–39, used by:
```
    debug_bvh_layout: EnumProperty(
        name="BVH Layout",
        items=enum_bvh_layouts,
        default='EMBREE',
    )
```
File `Blender Sources/cycles_5.0/properties.py`, lines 1073–1077. It carries no description in the file, and I found no manual page for it.

**Not covered by Part 3:** whether these debug switches are reachable without a debug build; what "Auto" actually picks for MetalRT and on what basis; how much memory MetalRT saves on curve-heavy scenes.

---

### Part 4 — Reading C: the graphics API for drawing and display

If the question means the graphics API rather than the Cycles backend, there is a separate and differently-named choice.

**LABEL: documented**

> "Display Graphics
>
> Settings that control how Blender draws its user interface and other display graphics.
> These options can influence performance and compatibility.
>
> Backend
>
> Selects the graphics API used for drawing the interface and rendering display content.
>
> Changing the backend requires restarting Blender for the change to take effect.
>
> OpenGL:
>
> Uses the OpenGL backend.
> This is the traditional backend, compatible with a wide range of systems.
>
> Vulkan:
>
> Uses the Vulkan backend. Vulkan may offer improved performance and better support for modern GPU features,
> but compatibility may vary depending on the system and drivers."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30. (The quoted passage runs on past Vulkan; those two are what the page spells out under Backend.)

The API documentation gives the full set of three:

> "gpu_backend
>
> GPU backend to use (requires restarting Blender for changes to take effect)
>
> OPENGL
> OpenGL – Use OpenGL backend.
>
> METAL
> Metal – Use Metal backend.
>
> VULKAN
> Vulkan – Use Vulkan backend.
>
> Type:
>
> enum in ['OPENGL', 'METAL', 'VULKAN'], default 'OPENGL'"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.PreferencesSystem.html — read 2026-07-30.

It can be named from outside:

> "GPU Options
>
> --gpu-backend
>
> Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

The connection between this backend and a *render* is documented only for EEVEE's reliance on the GPU in general:

> "Being a rasterization engine, EEVEE only uses the power of
> the GPU to render.
> There is no plan to support CPU (software) rendering
> as it would be very inefficient. CPU power is still needed to handle high scene complexity
> as the geometry must be prepared by the CPU before rendering each frame."

Source: manual, https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html — read 2026-07-30.

Concluding from those two that an EEVEE render is performed through whichever Display Graphics backend is selected would be **inference**, and I flag it as such rather than asserting it: the step would be "EEVEE renders on the GPU" plus "the Backend setting is the graphics API used for drawing and rendering display content" therefore "EEVEE renders through that API". The sources never join those two sentences, and "display content" is not obviously the same as a final render.

One release note bears on the backends' effect (**documented**):

> "Simple materials should now be 4 time faster to compile on NVidia OpenGL driver.
> This brings improvement on all backends."

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/eevee/ — read 2026-07-30.

**Not covered by Part 4:** whether the Display Graphics backend has any effect on a Cycles render at all; whether it changes an EEVEE render's output.

---

### Part 5 — Does anything establish whether the choice changes with the size of the scene?

This is the part of the question the sources answer least well. I searched the whole manual for "large scenes", "larger scenes", "scene size", "scene complexity", "complex scenes", "heavy scene", "depending on the scene", and "scene content", and read every hit that touched rendering. Here is everything the sources say that bears on it.

**5a. That performance varies with the scene — documented, but with no rule**

> "This varies depending on the hardware used. Different technologies also have different compute times
> depending on the scene tested."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

This establishes that the ranking between methods is scene-dependent. It does not say the dependence is on *size*, and it gives no rule; it points at an external resource instead.

**5b. What the sources say about memory and complex scenes — documented**

> "On the other hand, they also have some limitations in rendering complex scenes, due to more limited memory,
> and issues with interactivity when using the same graphics card for display and rendering."

> "There may be multiple causes,
> but the most common one is that there is not enough memory on your graphics card."

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
> try to use system memory."

Source, all three: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

What those three sentences say, on their face: complex scenes are named as a limitation of graphics cards, limited memory is named as the reason, running out of card memory is named as the most common cause of trouble, and four of the five methods are named as falling back to system memory when the card fills.

**Reading them together as "below the card's memory the choice is about speed and features, and above it the methods differ" is inferred, not documented.** The step a reader must supply is that the two halves are exhaustive — that nothing else about the methods changes with scene size. The sources nowhere say that.

**5c. That one sub-choice depends on scene content, not size — documented**

> "MetalRT for ray tracing uses less memory for scenes which use curves extensively,
> and can give better performance in specific cases."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30.

"Curves extensively" is a statement about what is in the scene, not how big it is.

**5d. That a multi-card option is aimed at large scenes — documented**

> "Distribute Memory Across Devices
>
> Allocates resources across multiple GPUs rather than duplicating data,
> effectively freeing up space for larger scenes. Note that in order for this option to be available,
> the GPUs must be connected together with a high bandwidth communication protocol.
>
> Currently only NVLink on NVIDIA GPUs is supported."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30.

The same, from the GPU page:

> "The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost.
> This can be enabled with Distributed Memory Across Devices
> in the preferences."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

Matching file text (**observed**):

```
    peer_memory: BoolProperty(
        name="Distribute memory across devices",
        description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it",
        default=False,
    )
```
File `Blender Sources/cycles_5.0/properties.py`, lines 1608–1612. Read 2026-07-30.

This is size-dependent and method-dependent (NVIDIA only), so it is a real instance of the effect the question asks about — but it is about how many cards, not which method, once you are on NVIDIA.

**5e. That kernels are tailored to scene content — observed**

The `kernel_optimization_level` description quoted in Part 3 opens "Kernels can be optimized based on scene content", and the adaptive-compile description is "Compile the Cycles GPU kernel with only the feature set required for the current scene". Both say the compiled kernel depends on the scene, and neither says anything about size. The adaptive-compile switches are named one per backend — CUDA, HIP and Metal; nothing quoted here ties `kernel_optimization_level` to any backend.

**5f. The verdict**

**LABEL: not in the sources** — for a general statement that the *choice between methods* changes with the size of the scene.

Where I looked, in full:
- Read end to end: manual GPU Rendering page; manual Preferences → System page; manual Cycles Render Settings index page; the Cycles Render Options and GPU Options sections of the command-line arguments page.
- Read the relevant sections of: manual Cycles Sampling page (Denoiser, Use GPU); manual Cycles Performance page (Viewport, Compositor Device); manual EEVEE limitations page; manual EEVEE introduction page.
- Searched the whole 5.0 manual, case-insensitively, for: `optix`, `cuda`, `HIP`, `oneapi`, `Metal`, `hardware ray`, `BVH`, `rasteriz`, `larger scenes`, `large scenes`, `scene complexity`, `complex scenes`, `depending on the scene`, `scene size`, `heavy scene`, `scene content`, `GPU Compute`.
- Searched the whole 5.0 API documentation for: `optix`, `oneapi`, `compute_device_type`, `metalrt`, `backend`. The Cycles backend enum does not appear there at all; the only hits were `legacy_compute_device_type` ("For backwards compatibility only", read-only integer) and `gpu_backend` on `bpy.types.PreferencesSystem`, plus `gpu.platform.backend_type_get()`.
- Searched all 5.0 release notes for `optix`, `cuda`, `hip`, `oneapi`, `metal`, `vulkan`, `backend`, `ray trac`. Read the GPU section of the Cycles notes in full.
- Searched `cycles_5.0/properties.py`, `engine.py`, `ui.py` for every device and ray-tracing term above; read the whole `CyclesPreferences` class and the debug-flags block. Read `source/kernel/bvh/bvh.h`.

Ruled out and why: the sculpting "Rebuild BVH" pages and the BioVision Motion Capture import/export pages are unrelated uses of the letters BVH. The Video Sequencer and Compositor "Scene Size" hits are a different setting entirely. The version-history page mentions the backends but is a history of releases, and I did not draw on it.

What the sources do establish, and it is worth stating plainly: **the sources say the ranking varies by scene but decline to give a rule, and the one place they tie method to scale is memory** — that four named methods spill to system memory when the card fills, and that a fifth is not named there.

**Not covered by Part 5:** at what scene size any of this begins to matter; whether the amount of geometry, the number of textures, or something else is the thing that matters; whether the OptiX/HIP RT/Embree hardware ray-tracing advantage grows or shrinks with scene size.

---


---

## Tested by hand — what a running Blender showed

**Label: observed.** Run on 2026-08-30 by Claude Opus 5 in Claude Code, on the two computers described below, in **Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.1**. The same tests were run in all four versions, so that a gap in one is not read as a difference in Blender. Blender 5.0's own figures are given here in full; where the four versions agreed, that is said.

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

Blender 5.0 was started with no window and `--factory-startup` on each machine, and asked what the Cycles preference offers.

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

On both machines, in Blender 5.0, the Cycles preference object carries exactly eight properties:

```text
['bl_idname', 'compute_device_type', 'devices', 'kernel_optimization_level', 'metalrt', 'peer_memory', 'use_hiprt', 'use_oneapirt']
```

At factory settings, on both machines: `peer_memory = False`, `metalrt = 'AUTO'`, `use_hiprt = False`, `use_oneapirt = True`, `kernel_optimization_level = 'FULL'`.

*(Across the versions, `use_hiprt` starts `False` in 4.5.12 and 5.0.1 and `True` in 5.1.2 and 5.2.1 — the same on both machines, so that is a difference between Blender versions and not between computers. The other five start the same in all four.)*

All six settable values were assigned from a script in a background run with no window and read back:

```text
Q7SET compute_device_type: asked='NONE' before='METAL' after='NONE' TOOK
Q7SET metalrt: asked='OFF' before='AUTO' after='OFF' TOOK
Q7SET kernel_optimization_level: asked='INTERSECT' before='FULL' after='INTERSECT' TOOK
Q7SET use_hiprt: asked='true' before=False after=True TOOK
Q7SET use_oneapirt: asked='false' before=True after=False TOOK
Q7SET peer_memory: asked='true' before=False after=True TOOK
```

Findings 6 and 7 below show that setting `metalrt` and `kernel_optimization_level` this way changes what the render does, so the assignment is not merely accepted and then ignored.

### 3. Naming the device on the command line outranks the stored preference and the scene's own setting

Four crossings were run in Blender 5.0 on both machines, each with a scratch configuration folder holding a saved preference and a `.blend` saved with the scene's device one way or the other. Which method actually ran was read from two things Blender prints: `Building OptiX acceleration structure`, which only an OptiX render prints, and `Profiling information not available (only works with CPU rendering)`, which only a card render prints.

On the rented Linux machine: with the stored technology at `NONE` and the scene's device at `CPU`, adding `-- --cycles-device OPTIX` produced the OptiX lines, the card-only line, and a render in under a second against about six seconds for the processor. With the stored technology at `OPTIX` and the scene's device at `GPU`, adding `-- --cycles-device CPU` produced neither line and about six seconds. The Mac gave the same shape with `METAL` in place of `OPTIX`.

**So the command-line option beats both the stored preference and the scene's own setting, in both directions, on both operating systems.** All four versions behaved the same way.

### 4. What happens when the named method is one the machine cannot do, or is not a method at all

In Blender 5.0, on the rented Linux machine (no Apple, no AMD, no Intel graphics), `-- --cycles-device METAL`, `HIP` and `ONEAPI` were each asked for; on the Mac (no NVIDIA, no AMD, no Intel graphics), `CUDA`, `OPTIX` and `HIP`. On both, `-- --cycles-device BANANA` was also asked for.

**Every one of them ended with exit status 1 and wrote nothing to disk.** For a real method the machine has no hardware for, Blender 5.0 printed:

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

**The control first, because without it this finding carries nothing.** The same scene was rendered **twice with the same method** and the two compared. On the processor, in Blender 5.0 on both machines, the two runs are **identical to the pixel** — not one channel differs. On a card they are not identical, but the largest difference between two runs of the same method is 4.17 × 10⁻⁷ to 8.34 × 10⁻⁷ on the rented Linux machine and 3.58 × 10⁻⁷ to 4.77 × 10⁻⁷ on the Mac.

Against that floor, in Blender 5.0:

| pair | small scene | heavy mesh | heavy curves |
|---|---|---|---|
| processor against CUDA (Linux) | 0.0121 | 0.0185 | 0.1221 |
| processor against OptiX (Linux) | 0.0102 | 0.0185 | 0.1209 |
| CUDA against OptiX (Linux) | 0.0121 | 0.0133 | 0.0467 |
| processor against Metal (Mac) | 0.0103 | 0.0185 | 0.1136 |
| MetalRT off against on (Mac) | 0.0103 | 0.0205 | 0.1425 |
| MetalRT off against auto (Mac) | 5.96 × 10⁻⁷ | 5.30 × 10⁻⁵ | 2.98 × 10⁻⁷ |

Figures are the largest difference in any colour channel; between 224,257 and 230,015 of the 230,400 pixels differ on the mesh scenes and about 149,000 to 162,500 on the curve scene. **Every pair of different methods in the first five rows differs by three to five orders of magnitude more than two runs of the same method do.** The last row, MetalRT off against auto, is at the noise floor on two of the three scenes; its heavy-mesh figure of 5.30 × 10⁻⁵ is two orders of magnitude above that floor and is not at it.

*(Citation check, 2026-08-30, repeated on the Creator's Mac in 5.0.1: the processor half reproduced exactly — two CPU renders identical to the pixel on all three scenes. **The Mac card control did not.** Two runs of Metal on the same scene came back at **1.999 × 10⁻³** on the small scene and **5.877 × 10⁻⁵** on the heavy mesh scene, with only the curve scene at 2.98 × 10⁻⁷ — four orders of magnitude above the floor stated above, and on the small scene only about five times below the 0.0103 recorded for the processor against Metal. The off-against-auto row came back at 5.96 × 10⁻⁷, **4.77 × 10⁻⁷** and 3.58 × 10⁻⁷ against the 5.96 × 10⁻⁷, 5.30 × 10⁻⁵ and 2.98 × 10⁻⁷ recorded. Both runs stand; neither has been chosen between, and the original figures above are untouched. The disagreement is on the hand-test list to be run again. The summary sentence was narrowed by this check because 5.30 × 10⁻⁵ in the last row is two orders of magnitude above the noise floor the same paragraph states.)*


*(Run again on 2026-08-30, and the control was measured properly this time: see **The noise floor measured, and three rows run again** below. Two runs of the same method were compared 120 times over. **On the processor all sixty pairs are identical to the pixel. On Metal the floor is 10⁻⁷ most of the time and not all of the time** — two card pairs in sixty came back at 5.31 × 10⁻⁵ and 0.0182. The rows above stay comfortably above the floor on the small and heavy mesh scenes and are only about six to eight times the worst case on the curve scene. Nothing above was changed.)*

### 6. What the choice changes depends on the size of the scene, and it changes direction

On the rented Linux machine, Blender 5.0, three repeats of every measurement with warm kernel caches. The three repeats agreed to within a few hundredths of a second throughout.

| scene | processor | CUDA | OptiX | card memory: CUDA / OptiX |
|---|---|---|---|---|
| small, 322 triangles | 00:06.30 / 00:06.28 / 00:06.32 | 00:00.59 / 00:00.60 / 00:00.60 | 00:00.61 / 00:00.66 / 00:00.67 | 1587 / 1601 MiB |
| heavy mesh, 10,485,762 triangles | 00:11.08 / 00:11.14 / 00:11.04 | 00:06.21 / 00:06.23 / 00:06.23 | 00:01.50 / 00:01.54 / 00:01.54 | 2259 / 2191 MiB |
| heavy curves, 300,000 curves | 00:14.92 / 00:14.99 / 00:14.92 | 00:04.43 / 00:04.38 / 00:04.40 | 00:01.20 / 00:01.20 / 00:01.23 | 1933 / 1707 MiB |

**On the 322-triangle scene CUDA is marginally ahead of OptiX. On the 10.5-million-triangle scene OptiX is about four times faster.** Same card, same driver, same file, same settings, same version. The card's own memory moves the same way: on the small scene OptiX takes slightly more than CUDA, on the heavy mesh scene rather less, and on the curve scene least of all. All four versions showed the same shape.

**A negative result was available and did not happen.** Had the ratio of one method's time to the other's been the same at both sizes, within the spread of the three repeats, the answer would have been that the effect does not depend on the size of the scene.

### 7. MetalRT, and what "Auto" does

The Creator's Mac only. MetalRT is a preference rather than a scene setting, so it was set from a script at start-up before the frame was rendered, and the script printed what it asked for and what the value became, so a run where the setting did not apply cannot be mistaken for one where it made no difference.

**What Blender prints while it builds separates the three settings cleanly, and identically in all four versions.** With MetalRT `ON` it prints `Building TLAS` and `Building mesh BLAS`, and on the curve scene `Building hair BLAS` as well. With `OFF` and with `AUTO` it prints `Packing BVH nodes` and `Packing BVH triangles and strands`. **On all three scenes, in all four versions, Auto built the same structure as Off.**

Cycles' own reported peak memory in Blender 5.0, which the shared Mac does not affect:

| scene | MetalRT Off | MetalRT On | MetalRT Auto |
|---|---|---|---|
| small | 1388 M | 1388 M | 1388 M |
| heavy mesh | 2033 M | 2488 M | 2033 M |
| heavy curves | 1718 M | 1683 M | 1718 M |

**At 322 triangles the switch changes nothing measurable. On the mesh scene it costs about 455 MB more. On the curve scene it saves about 34 MB.** That is the same direction for curves in all four versions and the opposite direction on a mesh scene of comparable weight, and it appears only once the scene is large enough for it to.

Times in Blender 5.0, fastest of three repeats on a shared Mac: on the heavy mesh scene, Off 00:05.14, On 00:03.55, Auto 00:05.16; on the curve scene, Off 00:05.96, On 00:06.20, Auto 00:06.08; on the small scene, Off 00:01.84, On 00:01.92, Auto 00:01.40.

### 8. Kernel Optimization

The Creator's Mac only, the heavy mesh scene, Blender 5.0, set from a script and confirmed to have taken. Off 00:05.76, Intersection only 00:05.23, Full 00:04.93. Off was the slowest in all four versions. On the pictures: FULL against OFF reached 0.0125, above the noise; INTERSECT against OFF stayed at 4.77 × 10⁻⁷.


*(Run again on 2026-08-30, three renders of each setting and every pair of the nine compared: see **The noise floor measured, and three rows run again** below. Nothing above was changed.)*

### 9. A scene the machine cannot hold, and where the choice decides whether the render happens at all

The huge mesh scene — **125,829,122 triangles**, a 9.3 GB file — on the rented Linux machine, in Blender 5.0:

```text
OptiX: exit status 0, 28.44 seconds of wall time, card memory peaked at 8353 MiB of 10,240, host memory peaked at 27,088,814,080 bytes, render time 00:19.22, frame written
CUDA:  exit status 137, 122.18 seconds of wall time, card memory peaked at 1643 MiB of 10,240, host memory peaked at 32,912,109,568 bytes, no time line was reached, no frame written
```

**The OptiX run finished and wrote its frame. The CUDA run was killed by the system for want of host memory** — status 137, the last line printed being `Updating Scene BVH | Building BVH` at somewhere between 45% and 76%, with the container's memory at its ceiling of about 32.9 GB. The same happened in all four versions: every OptiX run completed, every CUDA run was killed.

**What this does not say.** The condition reached was the host's memory ceiling, not the card's. OptiX peaked at between 7,765 and 8,353 MiB of the card's 10,240 MiB and CUDA never got past 1,775 MiB before it died, so **nothing here bears on what happens when a card's own memory fills**, and the documented fallback to system memory was not exercised.

### 10. The Acceleration Structure panel

Blender 5.0 was started with no window on both machines and the panel itself was asked, for every crossing of the technology, the scene's device setting, and whether the processor is ticked in the device list, whether it would be drawn. The panel class `CYCLES_RENDER_PT_performance_acceleration_structure` exists in Blender 5.0 on both machines.

On the rented Linux machine the answer was `True` under every crossing except one:

```text
pref=OPTIX  scene_device=GPU cpu_device_ticked=False -> Acceleration Structure panel poll = False
pref=OPTIX  scene_device=GPU cpu_device_ticked=True  -> Acceleration Structure panel poll = True
```

So the panel goes away only when OptiX is chosen **and** the scene is set to the card **and** the processor is not ticked alongside it. Under `NONE`, `CUDA`, `HIP` and `ONEAPI` it is always present. On the Mac, under `NONE` and `METAL`, it is always present.

**The four versions answered identically.** That matters here: three of the four versions' documents quote a manual line about this panel and one does not, and this test shows that silence to be a gap in the documentation rather than a difference in Blender. One version difference showed up beside it and is not about this panel: 5.2.1 carries a `CYCLES_RENDER_PT_performance_texture_cache` panel that this version does not.

### 11. What travels with the file, and what a stored preference alone does

In Blender 5.0, on both machines: a `.blend` saved with the scene's own device set to `GPU`, opened with a configuration whose technology was set one way and then another, read back:

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

Blender 5.0, both machines, `--gpu-backend` given each of its three documented values on a background render.

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

`blender --help` in Blender 5.0, on both machines, lists under **Cycles Render Options**:

```text
--cycles-print-stats
	Log statistics about render memory and time usage.
```

Every run above that used it printed a `Render statistics:` block with per-object geometry memory, and on a card render the line `Profiling information not available (only works with CPU rendering)`.

### The noise floor measured, and three rows run again — 2026-08-30

**Label: observed.** Run on 2026-08-30 by Claude Opus 5 in Claude Code, **on the Creator's Mac and on no other computer**, in Blender 5.0.1 alongside the other three researched versions. These are entries 26 to 29 of `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md`, sent there by that topic's citation check on the same day because its own repeats of the tests above came back different in four places.

**What this can and cannot settle.** The Creator's Mac has **Apple's own graphics and no NVIDIA card**, and the rented Austrian machine behind the CUDA and OptiX figures above was destroyed on the day those tests were run. **So everything in this section settles macOS and Apple's own graphics, and says nothing whatever about an NVIDIA card.** Nothing was rented for it. Other agents had been running Blender on the same Mac all day, so **no time in this section is a clean one**; all four tests are about pixels and exit statuses rather than speed. Every one of about 900 Blender launches ran with a scratch configuration folder, so nothing of the Creator's own settings was read or written.

**Why it was worth running at all.** Finding 5 above rests entirely on a control: two runs of the *same* method should come back far closer together than two runs of *different* methods. Nobody had measured how wide that same-method spread really is — one pair had been run, not a spread — and **an answer whose noise floor is unknown cannot be weighed.**

#### The floor itself, over sixty pairs on the card and sixty on the processor

**What was done.** Each of the small, heavy mesh and heavy curve scenes was rendered **twice with the same method** — once on the processor, once on Metal — and the two compared by their pixels, in all four versions. **The whole set was repeated five times over**: 240 renders and 120 pairs, of which 15 card pairs and 15 processor pairs are Blender 5.0's.

**What a negative result would have looked like:** every Metal pair at 10⁻⁷ every time, which would have confirmed the floor exactly as recorded above.

**The processor. All sixty pairs across the four versions are identical to the pixel** — not one channel of 921,600 differs — and Blender 5.0's fifteen are among them; every one printed `VERDICT: IDENTICAL to the pixel`.

**The card, in Blender 5.0, the five repeats in order:**

| scene | two runs of Metal, repeats 1 to 5 |
|---|---|
| small | 3.58 × 10⁻⁷, 4.77 × 10⁻⁷, 4.77 × 10⁻⁷, 4.77 × 10⁻⁷, 4.77 × 10⁻⁷ |
| heavy mesh | **5.310773849487305 × 10⁻⁵**, 5.96 × 10⁻⁷, 4.77 × 10⁻⁷, 4.77 × 10⁻⁷, 4.77 × 10⁻⁷ |
| heavy curves | 2.98 × 10⁻⁷, 2.98 × 10⁻⁷, 3.58 × 10⁻⁷, 2.98 × 10⁻⁷, 2.98 × 10⁻⁷ |

**This version produced one of the two excursions in the whole set of sixty**, on the heavy mesh scene, on the first of the five repeats, with 161,771 of the 230,400 pixels differing. The other four repeats of that same scene sat at 4.77 × 10⁻⁷ to 5.96 × 10⁻⁷.

**Across all four versions, fifty-eight of the sixty card pairs sit between 2.98 × 10⁻⁷ and 5.96 × 10⁻⁷.** The two that do not are 5.310773849487305 × 10⁻⁵, in Blender 5.0.1 on the heavy mesh scene, and 0.018208861351013184, in Blender 4.5.12 on the curve scene. **Both were the first repeat of the set.** So **the floor on this machine is not one number**: it is 10⁻⁷ most of the time, and about one card pair in thirty came back four to five orders of magnitude higher.

#### What that makes of the differences recorded above

Only the Mac rows of finding 5 are touched here. The Linux rows are outside what this machine can say anything about.

| row, as finding 5 records it for Blender 5.0 | small | heavy mesh | heavy curves |
|---|---|---|---|
| worst two-runs-of-Metal figure measured here, this version | 4.77 × 10⁻⁷ | **5.31 × 10⁻⁵** | 3.58 × 10⁻⁷ |
| worst two-runs-of-Metal figure measured here, all four versions | 4.77 × 10⁻⁷ | 5.31 × 10⁻⁵ | **0.0182** |
| processor against Metal | 0.0103 | 0.0185 | 0.1136 |
| MetalRT off against on | 0.0103 | 0.0205 | 0.1425 |

**On the small scene and the heavy mesh scene those recorded differences sit comfortably above the floor** — from about 350 times it up to tens of thousands of times it. **On the curve scene they do not sit comfortably above it.** The worst same-method figure this machine produced on that scene, in any version, is 0.0182, and the recorded curve-scene differences of 0.1136 and 0.1425 are only about six to eight times that. They are still above it, and nineteen of the twenty curve-scene pairs were at 10⁻⁷ — but *orders of magnitude* is the wrong description of the curve-scene rows, and *a factor of six to eight against the worst case* is the right one.

**Both runs stand.** Finding 5's own control figures above are untouched, this run's are here, and neither has been chosen over the other.

#### MetalRT Auto against MetalRT Off, three hundred comparisons

**What was done.** Each of the three scenes was rendered **five times with MetalRT off and five times with MetalRT auto**, in all four versions, and **every off was compared against every auto** — 25 comparisons per scene per version, 300 in all, of which 75 are Blender 5.0's.

**What a negative result would have looked like:** every pair at the noise floor every time. That is what came back, and it could have gone the other way — the same procedure produced figures of 0.012 and 0.018 twice before, once in the record above and once in the citation check's repeat.

**Every one of the 120 renders exited 0 and printed the setting it had been given**, sixty reading off and sixty reading auto. **And what Blender printed while it built was the same line in all 120**, in every version and on every scene:

```text
Packing BVH nodes; Packing BVH triangles and strands
```

So **auto built the same structure as off in all 120 runs**, which is what finding 7 above already says.

**The pixels, in Blender 5.0, as the range over the twenty-five comparisons in each cell:**

| scene | MetalRT off against auto, twenty-five comparisons |
|---|---|
| small | 3.58 × 10⁻⁷ to 5.96 × 10⁻⁷ |
| heavy mesh | 4.77 × 10⁻⁷ to 5.96 × 10⁻⁷ |
| heavy curves | 2.98 × 10⁻⁷ to 3.58 × 10⁻⁷ |

Across all four versions **not one of the 300 comparisons is above 7.15 × 10⁻⁷**. **This version's recorded figure for that row is 5.30 × 10⁻⁵ on the heavy mesh scene. It did not come back in twenty-five tries.**

**Both runs stand.** Finding 5's off-against-auto row for Blender 5.0 — 5.96 × 10⁻⁷, 5.30 × 10⁻⁵ and 2.98 × 10⁻⁷ — is untouched, and so is the citation check's note beside it.

**One plain thing about the numbers, offered as arithmetic and not as an answer.** The figures recorded for that row which are above the floor are the same size as the two same-method excursions measured above, and two of them fall on the same scene in the same version as an excursion. **Nothing run here establishes what that means**, and this section does not say which reading is the right one.

#### Kernel Optimization, thirty-six renders and a hundred and forty-four comparisons

**What was done.** The heavy mesh scene rendered with Kernel Optimization off, intersection only, and full — **three times each**, in all four versions — and **every pair of the nine images in a version compared**, which is 36 pairs a version. The nine include three same-setting pairs, so the floor for this scene is measured inside the same test.

**What a negative result would have looked like:** all pairs at the noise floor, which is what Blender 5.1's own record says and what the other three versions' records deny.

**All 36 renders exited 0 and every one printed the setting it had been given. In Blender 5.0:**

| pair | range over the nine, or the three |
|---|---|
| off against full | 0.0124985 |
| off against intersection only | 4.77 × 10⁻⁷ |
| full against intersection only | 0.0124985 |
| two runs of the same setting | 4.77 × 10⁻⁷ to 5.96 × 10⁻⁷ |

**That agrees with this version's own record**, which has Full against Off at 0.0125 and Intersection only against Off at the floor.

**Across the four versions the split is clean and it is the same split the records above have**: 4.5.12, 5.0.1 and 5.2.1 gave 0.0125 for off against full in all nine pairs each, and 5.1.2 gave the noise floor in all nine.

**Times, on a machine that was not quiet**, as Cycles' own `Time:` line for the three runs of each setting in Blender 5.0: off 00:05.82, 00:05.67, 00:05.68; intersection only 00:05.37, 00:05.29, 00:05.25; full 00:04.94, 00:05.05, 00:04.94. **Off was the slowest in all four versions**, which is the shape both earlier runs recorded and the one part of this test nobody disputes.

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

**LABEL: three parts, each labelled where it sits below — documented, observed, and inferred.**

Documented: the method can be set on a run started from outside Blender's window, and the manual's own example is a background run — "blender -b file.blend -f 20 -- --cycles-device OPTIX" (manual, command-line arguments page, read 2026-07-30). The `--gpu-backend` option is likewise a command-line option.

Observed: the Cycles add-on only reads `--cycles-device` when a bare `--` appears in the command line (`engine.py`, lines 27–29), and its own help text says it overrides "user preferences and the scene setting" (`engine.py`, lines 17–19).

Inferred, with the step written out: the backend choice lives on `CyclesPreferences`, which the file declares as `class CyclesPreferences(bpy.types.AddonPreferences)` (`properties.py`, line 1569), and the manual calls it "the Cycles Render Devices user preference". Preferences belong to the installation, not to a `.blend`. So a headless run that does not pass `--cycles-device` uses whatever that machine's preferences hold, and the same file on two machines can render through different methods. The step a reader must supply is that add-on preferences are stored outside the `.blend`; the sources I read do not state that in so many words.

One documented difference the other way: the Display Graphics backend "requires restarting Blender for the change to take effect" (manual, System preferences page) — so for that one, setting it at launch is the only way, whereas `--cycles-device` is read at add-on start.

Documented, and specific to a headless run: for EEVEE, not Cycles —

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

Source: manual, https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html — read 2026-07-30.

The sources do not say whether the MetalRT / HIP RT / Embree-on-GPU sub-choices can be set from the command line. I found no command-line option for them, and they have no `--cycles-*` argument in `engine.py`. They are preferences properties, so a script could set them, but nothing I read says that.

### Operating system

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Tested rather than inferred: the Creator's Mac offers only `None` and `Metal` and starts at `Metal`; a rented Linux machine with one NVIDIA card offers `None`, `CUDA`, `OptiX`, `HIP` and `oneAPI` and starts at `None`. So the list of methods does differ by operating system — **but it is built from what that build supports rather than from the cards present**, since the Linux machine offered HIP and oneAPI with no AMD and no Intel graphics in it. `--gpu-backend` accepts `opengl` and `vulkan` on Linux and only `metal` on macOS, naming the rejected value in an error and then finishing the render anyway. **Windows is not settled**: this project's route to a Windows computer is a rental with two general-purpose cores and no graphics card at all, so four card-free Windows tests are written out ready to run and wait on the next Windows batch.]*

**LABEL: documented.** This is one of the few places the sources are explicit, and the answer is no — the available methods differ by operating system.

> "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer
> and operating system."

CUDA, OptiX, HIP and oneAPI are each stated as "supported on Windows and Linux". Metal is "supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." Minimum driver versions are stated separately for Windows and Linux for both HIP and oneAPI.

Source, all: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

Observed: the default backend is chosen by operating system and processor —

```
        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
            return 5
        return 0
```
File `Blender Sources/cycles_5.0/properties.py`, lines 1576–1578 — and the MetalRT switch is drawn only when `platform.machine() == 'arm64'` (`properties.py`, line 1918, with the comment "# MetalRT only works on Apple Silicon."). Read 2026-07-30.

Other OS-specific items I found: the CUDA compilation errors section of the GPU page is Linux-specific ("On Linux, depending on your GCC version you might get this error"), and the display-driver timeout section is Windows-specific ("Windows has a limit on the time the GPU can do render computations"). The command-line documentation for `--gpu-backend` lists "vulkan, metal, opengl" without saying which apply on which system.

Not covered: whether `--cycles-device METAL` behaves any differently on an Intel Mac, and whether Metal is listed but inert on non-Apple-Silicon Macs.

---

## Things a running Blender could still settle

*(Pruned 2026-08-30 by the hand tests. Everything the earlier list held has either been run — the results are above, under **Tested by hand — what a running Blender showed** — or is recorded there as unable to be run, with its reason. What follows is what is genuinely still open.)*

1. **What happens when the card's own memory fills.** The scene of 125,829,122 triangles reached the host machine's memory ceiling and not the card's, so the documented fallback to system memory was never exercised. Settling it needs a machine whose card is small against what its host can build, which is the opposite of the usual want.
2. **What MetalRT's "Auto" weighs.** On the Creator's Mac it built the same structure as Off on all three scenes, including one of 300,000 curves — in the tests above, and again in sixty more runs on 2026-08-30, in all four versions, without one exception. Whether it ever picks the other way — on other Apple hardware, or on some other kind of scene — is still not settled.
3. **Whether the four card-free Windows tests give the same answers.** They are written out ready to run in `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md` and wait on the next Windows rental.
4. **Everything needing hardware nobody here has**: an AMD card, an Intel Arc card, two cards of different ray-tracing generations in one machine, NVLink-joined cards, a Mac that is not Apple silicon, or an Apple machine an agent may open a window on. Each is named above with its reason.

---

2026-08-29 — Claude Opus 5 (Claude Code) — What a render demands, question 7, Blender 5.0

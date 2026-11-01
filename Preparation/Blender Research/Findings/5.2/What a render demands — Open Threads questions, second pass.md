# What a render demands — Open Threads questions, second pass

**Sources consulted, and when.** Blender 5.2 manual (plain-text snapshot), Blender 5.2 Python API reference (plain-text snapshot), Blender 5.2 release notes (plain files), and the Cycles add-on's own files as supplied — all inside `Blender Sources/`, all taken on 2026-07-30, all read by me on 2026-08-30. One manual image was opened: `_images/editors_preferences_section_system.webp`.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: question 7 only, as instructed. Questions 1–6 are not answered here.

---

## 7. Where a render can be told which of a graphics card's rendering methods to use, what methods are there, what does choosing one rather than another change about the render, and does anything establish whether that changes with the size of the scene?

### The question was ambiguous, and how I read it

"A graphics card's rendering methods" can be read three ways. I answer the first, which I believe is meant, and name the other two so nothing is silently chosen.

1. **The reading I answer.** The named technologies Cycles uses to drive a graphics card — CUDA, OptiX, HIP, oneAPI, Metal — plus the separate switches that turn a card's built-in ray-tracing hardware on or off within one of those technologies (MetalRT, HIP RT, Embree on GPU). These are the manual's own "Rendering Technologies", and they are literally methods of using a graphics card.
2. **Second reading, reported briefly at the end.** The graphics interface Blender draws and rasterises through — OpenGL, Vulkan, Metal. This is a card's rendering method in the plain sense, but the manual attaches it to drawing the interface, not to Cycles.
3. **Third reading, reported briefly at the end.** EEVEE's Raytracing "Method" (Light Probe versus Screen-Trace). This is a rendering method, but nothing in the sources ties it to a particular graphics card or its capabilities.

---

### 7a. Where the choice is made

**LABEL: documented**

The choice does not live in the scene. It lives in Blender's preferences, in the System section.

> "Cycles Render Devices
>
> Changes the Computing Device
> the Cycles render engine uses to render images.
> Cycles can use either the CPU or certain GPUs to render images,
> for more information see the GPU Rendering page."

Source: manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30.

The GPU Rendering page states the three steps, and this is where the split between preference and scene is set out:

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal
> in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30.

So there are two separate places. The *method* is a preference; whether a given scene uses it at all is a scene setting:

> "Device
>
> Specifies the compute device used for rendering in Cycles.
> The choice of device can significantly affect render performance, GPU Compute is often faster ."
>
> "GPU Compute:
>
> Render using one or more graphics processing units (GPUs) through the compute backend
> and devices selected in the Cycles Render Devices."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/index.html — read 2026-07-30.

**Observed, from Cycles' own files.** The preference is an add-on preference, not a scene property. In `Blender Sources/cycles_5.2/properties.py`, line 1625 onwards:

```
class CyclesPreferences(bpy.types.AddonPreferences):
    bl_idname = __package__
```

and at line 1655:

```
    compute_device_type: EnumProperty(
        name="Compute Device Type",
        description="Device to use for computation (rendering with Cycles)",
        default=CyclesPreferences.default_device(),
        items=CyclesPreferences.get_device_types,
    )
```

The scene-side setting is a separate property, at line 30 and line 387 of the same file:

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
        update=update_render_passes,
    )
```

Source: Cycles' own files, `Blender Sources/cycles_5.2/properties.py`, lines 30–34, 387–393, 1625–1659 — read 2026-07-30.

**Documented, from a picture.** The preferences screenshot shows "Cycles Render Devices" as a single row of mutually exclusive buttons. On the machine the screenshot was taken, the row reads: None, CUDA, HIP. Directly below sits a separate "Display Graphics" panel with a "Backend" dropdown set to "OpenGL". This confirms that the Cycles method choice and the display backend choice are two different controls in the same preferences section.

Image: `_images/editors_preferences_section_system.webp`, shown on https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30.

**There is a third place, and it wins over both.** From outside Blender's window, on the command line:

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

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

**Observed** that this overrides both other places. In `Blender Sources/cycles_5.2/engine.py`, lines 16–20:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

Source: Cycles' own files, `Blender Sources/cycles_5.2/engine.py`, lines 16–20 — read 2026-07-30.

**Not covered.** The sources do not say whether the hardware-ray-tracing switches (MetalRT, HIP RT, Embree on GPU) can be set from the command line — there is no command-line option for them in the arguments page, and `engine.py` parses only `--cycles-device` and `--cycles-print-stats`. They can presumably still be changed by script, since they are add-on preferences, but nothing I read states that outright. The sources also do not say what happens if `--cycles-device` names a technology the machine has no card for; `engine.py` shows only that Blender exits with a failure code if the override is refused (`sys.exit(1)`, line 41), not what makes it refuse.

---

### 7b. What methods there are

**LABEL: documented**

Five technologies, plus "None" meaning the CPU.

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

Source: manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30.

Each is tied to a maker and to operating systems:

> "CUDA is supported on Windows and Linux and requires a
> NVIDIA graphics cards with compute capability 5.0 and higher."

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher
> and a driver version of at least 535."

> "HIP is supported on Windows and Linux and requires a
> AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."

> "oneAPI is a computation library that is supported on Windows and Linux and requires an
> Intel® Arc™ graphics card
> with the Xe HPG architecture."

> "Metal for GPU acceleration is supported on Apple computers with Apple Silicon.
> macOS 13.0 or newer is required to support all features."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30.

**Observed**, the same five in Cycles' own files, with the wording shown in the interface. `Blender Sources/cycles_5.2/properties.py`, lines 1637–1653:

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

Two things this shows that the manual does not. First, the list offered is built at the moment the menu is drawn, from what that build of Blender reports — so what is on offer is not fixed. *(Citation check, 2026-08-30: this read "from what the build and the machine report". Finding 1 under "Tested by hand" shows a machine with one NVIDIA card and no AMD or Intel hardware still offering HIP and oneAPI, so the machine's own cards are not what the row is built from; what is empty is the device list underneath each choice. Narrowed by this check.)* Second, the stored numbers are not consecutive: None is 0, CUDA 1, OptiX 3, HIP 4, Metal 5, oneAPI 6. Number 2 is absent.

The default is also **observed**, at lines 1628–1635:

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

So on Apple Silicon the factory setting is Metal; everywhere else it is None, meaning the CPU.

Source: Cycles' own files, `Blender Sources/cycles_5.2/properties.py`, lines 1628–1653 — read 2026-07-30.

**Within a technology, there is a second, separate method choice: whether the card's ray-tracing hardware is used.** There are three such switches, one per maker, each written into the panel code inside a test on its own technology.

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

Source: manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30.

**Observed**, with their kinds, defaults and ranges, from `Blender Sources/cycles_5.2/properties.py`, lines 1670–1705:

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

So MetalRT has three states and defaults to Auto; HIP RT and Embree on GPU are simple on/off and both default to on. Kernel Optimization is a fourth, Metal-only setting with three states, defaulting to Full — it is not in the manual page I read, so this is the only description of it I have.

**Observed**, the tests each switch is written inside, from `Blender Sources/cycles_5.2/properties.py`, lines 1984–2004:

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

And, ten lines above, a rule about mixed hardware that is nowhere in the manual — lines 1976–1977:

```
        # Any device without RT support will disable it for all.
        has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt
```

**Inferred** from the block above: each of the three is *shown* only when its own technology is chosen, the steps being that this function is what draws the panel and that a block not reached draws nothing. *(Citation check, 2026-08-30: the heading of this section read "each only appears for its own technology" and the block was labelled observed; what the file carries is where the properties are written, and 5.0's and 5.1's first passes label the same step inferred. Changed by this check.)*

**Inferred** from those last lines: if several cards are enabled and even one of them lacks ray-tracing hardware, the hardware ray-tracing switch is greyed out for the whole set. The step I took: the code sets `has_hardware_rt` to true only when every enabled device reports support, and then uses that value to set `row.active`, which is what makes a control live or greyed. I did not see this stated in words anywhere, and I did not run it.

Source: Cycles' own files, `Blender Sources/cycles_5.2/properties.py`, lines 1670–1705 and 1959–2004 — read 2026-07-30.

**Not covered.** Nothing I read defines what "Auto" weighs when it picks an intersection method for MetalRT. The rule is not in the manual, and the part of Cycles that would contain it is not among the files supplied — the supplied Cycles source is only the render kernel and its helpers (`source/kernel/…`, `source/util/…`), not the device layer that builds acceleration structures and chooses between them. Nor do the sources give a way to read back which way Auto went.

---

### 7c. What choosing one rather than another changes about the render

This splits into three kinds of change: what the render costs, what the render is allowed to contain, and what controls disappear.

#### What it costs — speed and memory

**LABEL: documented**

> "OptiX takes advantage of hardware ray-tracing acceleration in RTX graphics cards, for improved performance."

> "Hardware ray-tracing support is available with the most recent drivers.
> This can be enabled in the preferences, and is supported on Radeon RX 6000 and newer."

> "oneAPI is a computation library that is supported on Windows and Linux and requires an
> Intel® Arc™ graphics card
> with the Xe HPG architecture.
> Hardware acceleration for ray-tracing and denoising is supported."

> "GPU accelerated ray-tracing and denoising is available on Apple Silicon."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30.

The manual declines to rank them:

> "This varies depending on the hardware used. Different technologies also have different compute times
> depending on the scene tested. For the most up to date information on the performance of different devices,
> browse the Blender Open Data resource."

Same page, same date.

Memory behaviour differs between the technologies, and the difference is spelt out by omission:

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
> try to use system memory. This has a performance impact, but will usually still result in a faster render
> than using CPU rendering."

Same page, same date. oneAPI is not in that list of four. **Inferred**, and only weakly: a scene that overflows card memory may fail under oneAPI where it would survive under the other four. The step I took is reading a list of four named technologies as exhaustive when a fifth exists and is not named. That is an argument from silence, and the page nowhere says oneAPI lacks the fallback. Treat this as a thing to test, not a fact.

Denoising acceleration also differs by technology:

> "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes
> all NVIDIA RTX cards."

> "GPU accelerated denoising is available on discrete Radeon RX 6000 and newer GPUs on Linux, and discrete Radeon RX 7000 and newer GPUs on Windows."

> "GPU accelerated denoising is available on all supported GPUs." (oneAPI)

Same page, same date. Note that the AMD line splits by operating system as well as by card.

And the OptiX denoiser is only offered at all on NVIDIA:

> "OptiX:
>
> Uses NVIDIA’s OptiX AI denoiser.
> Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.
>
> Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30. Since the denoiser is a step that changes the pixels, this is a case where the method chosen can change the picture and not merely the clock.

#### What the render is allowed to contain

**LABEL: documented**

This is the sharpest set of differences, and they are differences in the image, not in the speed.

> "Path Guiding is not supported on any GPU.
>
> Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30.

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
> The trace function is not functional.
> As a result of this, the Ambient Occlusion
> and Bevel nodes do not work."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/osl/index.html — read 2026-07-30.

The two node pages confirm the same from the other side. Both list, among the conditions under which the node is unsupported:

> "Open Shading Language is active while using the OptiX rendering backend."

Sources: manual, https://docs.blender.org/manual/en/5.2/render/shader_nodes/input/ao.html and https://docs.blender.org/manual/en/5.2/render/shader_nodes/input/bevel.html — read 2026-07-30.

Custom cameras follow the same rule:

> "Custom cameras are not supported with GPU rendering unless using the OptiX backend."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/osl/camera.html — read 2026-07-30.

And one maker-and-operating-system combination is called out for caustics:

> "GPU rendering using Radeon RX 6000 series HIP devices on Windows is not supported."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/object_settings/object_data.html — read 2026-07-30.

Cycles' own files corroborate the CPU-versus-GPU split and the OptiX special case. In `Blender Sources/cycles_5.2/source/kernel/features.h`, under the comment `/* Device specific features */`:

```
#ifdef WITH_OSL
#  define __OSL__
#  ifdef __KERNEL_OPTIX__
/* Kernels with OSL support are built separately in OptiX and don't need SVM. */
#    undef __SVM__
#  endif
#endif
#ifndef __KERNEL_GPU__
#  if defined(WITH_PATH_GUIDING)
#    define __PATH_GUIDING__
#  endif
#  define __VOLUME_RECORD_ALL__
#endif /* !__KERNEL_GPU__ */
```

**Observed.** Path guiding and `__VOLUME_RECORD_ALL__` are switched on only when the target is not a graphics card. Source: Cycles' own files, `Blender Sources/cycles_5.2/source/kernel/features.h` — read 2026-07-30.

There is also a documented operating-system carve-out inside Metal, in the same file:

```
/* MNEE caused "Compute function exceeds available temporary registers" in macOS < 13 due to a bug
 * in spill buffer allocation sizing. */
#if defined(__KERNEL_METAL__) && (__KERNEL_METAL_MACOS__ < 13)
#  undef __MNEE__
#endif
```

**Observed**, same file, same date. MNEE is the caustics feature; on macOS before 13 it is compiled out of the Metal kernel.

#### What the choice does to the acceleration structure and its controls

**LABEL: documented**

> "Note
>
> The Acceleration Structure panel will not be present when using an Optix compute device."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html — read 2026-07-30. The controls that disappear with it are, from the same page:

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

**Observed**, the same rule in Cycles' own files, with one detail the manual leaves out. `Blender Sources/cycles_5.2/ui.py`, lines 925–931:

```
class CYCLES_RENDER_PT_performance_acceleration_structure(CyclesButtonsPanel, Panel):
    bl_label = "Acceleration Structure"
    bl_parent_id = "CYCLES_RENDER_PT_performance"

    @classmethod
    def poll(cls, context):
        return not use_optix(context) or use_multi_device(context)
```

The detail: the panel comes back if OptiX is combined with the CPU. Source: Cycles' own files, `Blender Sources/cycles_5.2/ui.py`, lines 925–931 — read 2026-07-30.

**Observed**, that the choice replaces the whole intersection path in the render kernel. `Blender Sources/cycles_5.2/source/kernel/bvh/bvh.h`, lines 18–31:

```
/* Device specific acceleration structures for ray tracing. */

#if defined(__EMBREE__)
#  include "kernel/device/cpu/bvh.h"
#  define __BVH2__
#elif defined(__KERNEL_METALRT__)
#  include "kernel/device/metal/bvh.h"
#elif defined(__KERNEL_OPTIX__)
#  include "kernel/device/optix/bvh.h"
#elif defined(__KERNEL_HIPRT__)
#  include "kernel/device/hiprt/bvh.h"
#else
#  define __BVH2__
#endif
```

and `Blender Sources/cycles_5.2/source/kernel/types.h`, lines 1385–1393:

```
  /* Device specific BVH. */
#ifdef __KERNEL_OPTIX__
  OptixTraversableHandle device_bvh;
#elif defined __KERNEL_METALRT__
  metalrt_as_type device_bvh;
#elif defined(__KERNEL_HIPRT__)
  void *device_bvh;
#else
```

**Inferred** from those two: turning MetalRT, HIP RT or OptiX on does not merely accelerate the same search; it swaps in a different body of code for finding where a ray hits, and swaps the acceleration structure the card holds. The step I took: reading a compile-time branch that selects between different implementation files as meaning different code runs. I did not run anything. Whether the different code returns exactly the same hits is a further step I do not take — see the "Not covered" note below.

Source folders read: `Blender Sources/cycles_5.2/source/kernel/device/` contains `cuda`, `gpu`, `hip`, `hiprt`, `metal`, `optix` — six sets of device-specific kernel files, and `kernel/bvh/bvh.h` picks among them.

**Not covered, and this is the important gap.** Nothing in any of the sources says whether two methods produce the *same pixels* for the same scene, seed and sample count. The manual talks about speed, memory and which features are available; it never says the images match, and it never says they differ. I searched the whole manual for "identical", "deterministic", "same result" and "differences between" (see the record below) and found nothing on the point. So: the sources establish that the method changes what can be rendered and how long it takes; they do not establish whether it changes the picture when everything is supported on both sides.

---

### 7d. Does anything establish whether this changes with the size of the scene?

**LABEL: documented, for the fact that the effect is scene-dependent. Not in the sources, for any rule tying it to size.**

Four statements establish that the effect of the choice is not fixed and moves with the scene:

> "This varies depending on the hardware used. Different technologies also have different compute times
> depending on the scene tested."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30.

> "MetalRT for ray tracing uses less memory for scenes which use curves extensively,
> and can give better performance in specific cases."

Source: manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30. This is scene *content*, not scene size: the named condition is heavy use of curves.

> "GPU rendering makes it possible to use your
> graphics card for rendering, instead of the CPU. This can speed up rendering
> because modern GPUs are designed to do quite a lot of number crunching.
> On the other hand, they also have some limitations in rendering complex scenes, due to more limited memory,
> and issues with interactivity when using the same graphics card for display and rendering."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30. This is about card versus CPU, not about one method versus another.

**Observed**, that the kernel itself is tailored to the scene. `Blender Sources/cycles_5.2/source/kernel/features.h`, near the top:

```
/* Kernel Feature Flags
 *
 * These are set from the host side depending on which features are used by the scene,
 * either for runtime checks or specialization.
```

and further down, the heading `/* Scene-based selective features compilation. */` above a block that switches off motion blur, hair, point clouds, volumes, subsurface, shadow catcher, denoising, ambient occlusion, MNEE, path guiding, principled hair, light linking and shadow linking when the scene does not use them.

The Metal-only Kernel Optimization setting says the same in words, **observed** at `Blender Sources/cycles_5.2/properties.py`, line 1696: *(citation check, 2026-08-30: this cited line 1706, which is blank; the description is at line 1696, where this version's first pass cites it. Corrected by this check.)*

```
        description="Kernels can be optimized based on scene content. Optimized kernels are requested at the start of a render. "
```

All of that is scene *content*. On scene *size* specifically, I found nothing that ties the method choice to it. The closest anything comes is memory, and memory pressure is where a big scene bites:

> "Distribute Memory Across Devices
>
> Allocates resources across multiple GPUs rather than duplicating data,
> effectively freeing up space for larger scenes. Note that in order for this option to be available,
> the GPUs must be connected together with a high bandwidth communication protocol.
>
> Currently only NVLink on NVIDIA GPUs is supported."

Source: manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30. That is a technology-specific way of coping with a large scene, available only to NVIDIA cards with NVLink — so in that narrow sense the method chosen does change what a large scene can do. But it is a separate switch, not a property of picking CUDA over HIP.

**Not covered.** No source I read gives a threshold, a curve, a rule of thumb, or even a qualitative statement of the form "the gap widens as the scene grows". The manual explicitly hands that question to an outside resource ("browse the Blender Open Data resource") which I did not and may not visit. So: **not in the sources**, for any relationship between scene size and the effect of the method choice.

---

### 7e. Two other readings of the question, reported briefly

**Second reading — the graphics interface Blender draws through.** **LABEL: documented.**

> "Backend
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

Source: manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30.

From outside the interface:

> "--gpu-backend
>
> Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

It can be read back by script:

> "gpu.platform.backend_type_get()
>
> Get active GPU backend.
>
> Returns:
>
> Backend type (‘OPENGL’, ‘VULKAN’, ‘METAL’, ‘NONE’, ‘UNKNOWN’)."

Source: Python API, https://docs.blender.org/api/5.2/gpu.platform.html — read 2026-07-30.

The manual's own wording attaches this to "drawing the interface and rendering display content", so I do not claim it governs a final EEVEE frame; nothing I read says it does, and nothing says it does not. Two side-effects of the choice are documented, though: two preferences options "not available on macOS and requires using OpenGL backend" (System page, lines 219 and 241), and the auto vsync mode, where "auto is only supported by the OpenGL backend" — that one on the Command Line Arguments page under `--gpu-vsync`, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, not on the System page. *(Citation check, 2026-08-30: the vsync sentence was attributed to the System preferences page, which carries no vsync sentence at all; it is at `advanced/command_line/arguments.txt` line 469. Address corrected by this check.)*

**Third reading — EEVEE's tracing method.** **LABEL: documented.**

> "Method
>
> Determine the tracing method used to find scene-ray intersections and indirect lighting.
>
> Light Probe:
>
> Use light-probe spheres and planes to find scene intersection.
> This option has the lowest tracing cost but relies on manually placed light-probes.
>
> Screen-Trace:
>
> Trace ray against the screen depth buffer. Fallback to light-probes if ray exits the view."

Source: manual, https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/raytracing.html — read 2026-07-30.

This changes the picture directly, but it is a scene setting and nothing in the sources ties it to any property of a graphics card. I report it only because "rendering method" is its literal name. EEVEE offers no equivalent of Cycles' technology choice: I searched the whole EEVEE section of the manual for "device" and found only compositing and denoising device settings, and the EEVEE limitations page states plainly that "There is currently no support for multiple GPU systems" and "Being a rasterization engine, EEVEE only uses the power of the GPU to render." (https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html — read 2026-07-30).

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

**LABEL: documented, with one gap.**

The Cycles technology choice is fully settable from outside Blender's window, and it is the *only* one of the settings in this answer for which that is documented. `--cycles-device` takes CPU, CUDA, OPTIX, HIP, ONEAPI or METAL, with `+CPU` appended for a mixed render, and the example given is a background render: `blender -b file.blend -f 20 -- --cycles-device OPTIX`. Cycles' own files state it "overrid[es] user preferences and the scene setting". So a machine driven by a script does not depend on what the preferences file says.

Where it differs from inside: the option names only the technology, not which individual cards within it to enable. Enabling and disabling particular cards is described only as a preferences action ("Enable the available devices for the chosen backend", and "Yes, go to Preferences ‣ System ‣ Compute Device Panel, and configure it as you desire"), and there is no command-line option for it in the arguments page. Likewise there is no command-line option for MetalRT, HIP RT, Embree on GPU or Kernel Optimization.

Those settings live in the preferences file rather than the scene file — "./config/userpref.blend — User preferences." (manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30). **Inferred**: a scene file carried to another machine does not carry the method choice with it; the machine's own preferences decide, unless `--cycles-device` overrides them. The step I took: combining "the choice is a preference", "preferences are stored in userpref.blend in the user's config directory" and "the scene setting is a separate property" into a statement about what travels with a file. Each part is quoted above; the conclusion is mine.

Two documented differences that bite specifically outside the interface: EEVEE's "Headless rendering is not supported on headless Windows systems." (https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html — read 2026-07-30); and, for the display backend, the release notes record "Added `gpu.init()` to initialize gpu backend when running Blender in background mode `--background`." (release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30), which is a documented statement that the graphics backend needs an explicit step when there is no window.

**Not covered.** Nothing states whether the hardware ray-tracing switches take effect in a background render at all, nor whether Kernel Optimization's warm-up cost ("additional CPU usage for a brief time (tens of seconds)") applies to each background invocation separately.

### Operating system

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Tested rather than inferred: the Creator's Mac offers only `None` and `Metal` and starts at `Metal`; a rented Linux machine with one NVIDIA card offers `None`, `CUDA`, `OptiX`, `HIP` and `oneAPI` and starts at `None`. So the list of methods does differ by operating system — **but it is built from what that build supports rather than from the cards present**, since the Linux machine offered HIP and oneAPI with no AMD and no Intel graphics in it. `--gpu-backend` accepts `opengl` and `vulkan` on Linux and only `metal` on macOS, naming the rejected value in an error and then finishing the render anyway. **Windows is not settled**: this project's route to a Windows computer is a rental with two general-purpose cores and no graphics card at all, so four card-free Windows tests are written out ready to run and wait on the next Windows batch.]*

**LABEL: documented.** This answer does not hold the same on every operating system — the method choice is more operating-system-bound than anything else in this topic.

- CUDA, OptiX, HIP and oneAPI: "supported on Windows and Linux". None of the four is stated to work on macOS.
- Metal: "supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." Cycles' own files further restrict MetalRT and Kernel Optimization to Apple Silicon specifically (`if (platform.machine() == 'arm64')`, `properties.py` line 1987), and the factory default is Metal only on Darwin/arm64 (`properties.py` lines 1628–1635).
- AMD denoising acceleration splits by operating system: "discrete Radeon RX 6000 and newer GPUs on Linux, and discrete Radeon RX 7000 and newer GPUs on Windows."
- Caustics: "GPU rendering using Radeon RX 6000 series HIP devices on Windows is not supported."
- Minimum drivers are given per operating system for HIP (Windows: Radeon Software 24.9.1 or Radeon PRO Software 24.Q4; Linux: Radeon Software 24.30 or ROCm HIP Runtime 6.3) and for oneAPI (Windows: Intel Graphics Driver XX.X.101.5518; Linux: intel-level-zero-gpu 1.3.27642).
- MNEE is compiled out of the Metal kernel on macOS before 13 (`features.h`).
- Two Linux/Windows-only failure modes are documented for CUDA — "Unsupported GNU version" on Linux with an incompatible GCC, and the NVIDIA display-driver timeout on Windows.
- The debug panel in Cycles' own files shows a different set of controls on macOS than elsewhere (`ui.py` lines 2296–2310: Metal on Darwin; CUDA, OptiX and HIP otherwise).

All manual quotes in this section: https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html and https://docs.blender.org/manual/en/5.2/render/cycles/object_settings/object_data.html — read 2026-07-30. File quotes: `Blender Sources/cycles_5.2/properties.py`, `ui.py`, `source/kernel/features.h` — read 2026-07-30.

---

## Record of where I looked

Everything below was run against the folders `Blender Sources/manual_5.2`, `Blender Sources/api_5.2`, `Blender Sources/release_notes_5.2` and `Blender Sources/cycles_5.2`, searching whole folders at once rather than opening files by guess. All searches were case-insensitive unless noted.

**Terms searched across the whole manual.** optix; cuda; HIP (whole word); oneapi; metalrt; "rendering method"; "render method"; "intersection method"; "rendering backend"; "compute backend"; "Rendering Technologies"; "hardware ray"; bvh; intersection; Vulkan; OpenGL; "graphics API"; "large scene"; "complex scene"; "scene size"; "scene complexity"; "depending on the scene"; "scene content"; "scene dependent"; "scene-dependent"; identical; deterministic; "same result"; "differences between"; gpu-backend; cycles-device; device (within the EEVEE section only).

**Manual pages opened in full or in part, and why.**
- `render/cycles/gpu_rendering.txt` — the page the whole answer rests on. Read in full.
- `editors/preferences/system.txt` — read lines 1–115 in full, for the Cycles Render Devices list and the three hardware ray-tracing switches; also lines 219 and 241 found by the OpenGL search.
- `advanced/command_line/arguments.txt` — read the Cycles Render Options block and the GPU Options block.
- `render/cycles/render_settings/index.txt` — the scene Device setting and the OSL limitation.
- `render/cycles/render_settings/performance.txt` — read lines 1–120 for the Acceleration Structure panel note, tile size, and the compositing device.
- `render/cycles/render_settings/sampling.txt` — the Denoiser choice and the "Use GPU" denoising option.
- `render/cycles/osl/index.txt` and `render/cycles/osl/camera.txt` — the OptiX-only limitations.
- `render/shader_nodes/input/ao.txt` and `render/shader_nodes/input/bevel.txt` — corroboration of the OSL/OptiX node limitation.
- `render/cycles/object_settings/object_data.txt` — the HIP caustics carve-out.
- `render/eevee/limitations/limitations.txt` — read the CPU Rendering, Multiple GPU Support and Headless Rendering sections.
- `render/eevee/render_settings/raytracing.txt` — read lines 1–60 for EEVEE's tracing Method.
- `editors/preferences/introduction.txt` — read lines 1–60, to establish where preferences live and how they are saved.
- `advanced/blender_directory_layout.txt` — read lines 100–130, for `config/userpref.blend`.
- `render/cycles/simplify.txt`, `troubleshooting/gpu/*` — reached by the searches, skimmed, ruled out as not about the method choice.

**Image opened.** `_images/editors_preferences_section_system.webp`, referenced from the System preferences page. Converted to PNG in a scratch folder to view; the original file was not modified. What it shows is reported in 7a.

**Python API.** Searched the whole API folder for: metalrt; use_hiprt; use_oneapirt; kernel_optimization; CyclesPreferences; optix; cuda; hiprt; oneapi; compute_device_type; Metal. **Result: the first nine return nothing at all.** The Cycles add-on's preferences are not in the API reference. `compute_device_type` appears in only three files, and in `bpy.types.PreferencesSystem.txt` only as `legacy_compute_device_type` — "For backwards compatibility only (in [-inf, inf], default 0, readonly)". Pages opened: `bpy.types.PreferencesSystem.txt`, `bpy.types.Addon.txt` (for the `preferences` attribute), `gpu.platform.txt`. Ruled out: the API documents the display GPU module and the generic add-on-preferences mechanism, but documents none of the Cycles device settings by name.

**Release notes.** Searched all twenty-one files for: OptiX; CUDA; HIP; oneAPI; Metal; Vulkan; backend; "ray trac"; device. Read `cycles.md` and `rendering.md` in full. **Result: 5.2's release notes record no change to device selection, to the technologies on offer, or to the hardware ray-tracing switches.** The only mentions of the technologies are in `bugfixes.md`, as fixed bugs — for example "MetalRT Light Linking Not Updating Until Light Is Toggled", "HIP RT: Transparent curves render darker than expected", "Cycles Crash rendering camera-only BSSRDF with oneAPI HW-RT", "Denoising albedo for rough metals is different with OptiX OSL". These are statements that something was wrong and is now fixed; I quote them only as evidence that the release notes say nothing new about the choice itself. `python_api.md` line 15 records the `gpu.init()` addition quoted above. `sequencer.md` line 15 mentions a compositor execution device, which is a different setting.

**Cycles' own files.** Searched all of `cycles_5.2` for: compute_device_type; metalrt; hiprt; oneapirt; Embree; METALRT; EXTENDED_LIMITS; enum_bvh_layouts; enum_devices; cycles-device; debug_bvh_layout. Files opened: `properties.py` (lines 30–44, 387–393, 1085–1140, 1625–1720, 1940–2010), `engine.py` (lines 1–60), `ui.py` (lines 925–975 and 2265–2320), `source/kernel/features.h` (in full), `source/kernel/bvh/bvh.h` (lines 1–60), `source/kernel/types.h` (lines 1370–1400), `source/kernel/device/metal/bvh.h` (lines 1–50). Directory listing taken of `source/kernel/device/`, which contains `cuda`, `gpu`, `hip`, `hiprt`, `metal`, `optix`.

**What I ruled out, and why.**
- *The device layer of Cycles.* The part of Cycles that would contain the MetalRT "Auto" decision rule, the acceleration-structure build, and the memory fallback behaviour is not among the files supplied — only `source/kernel/` and `source/util/` are present, and those are the render kernel, not the host-side device code. I did not go looking for it elsewhere.
- *The Blender Open Data resource.* Named by the manual as the place to compare technologies. It is on the web and outside my sources. Not visited.
- *The compositing and denoising device settings.* Both exist and both name CPU and GPU, but neither offers a choice among a card's technologies, so they are not an answer to this question.
- *EEVEE.* Searched for a device or technology choice and found none; the engine has no equivalent setting.
- *Nothing was searched or opened outside `Blender Sources/` and the question sheet.*

---

## Things a running Blender could still settle

*(Pruned 2026-08-30 by the hand tests. Everything the earlier list held has either been run — the results are above, under **Tested by hand — what a running Blender showed** — or is recorded there as unable to be run, with its reason. What follows is what is genuinely still open.)*

1. **What happens when the card's own memory fills.** The scene of 125,829,122 triangles reached the host machine's memory ceiling and not the card's, so the documented fallback to system memory was never exercised. Settling it needs a machine whose card is small against what its host can build, which is the opposite of the usual want.
2. **What MetalRT's "Auto" weighs.** On the Creator's Mac it built the same structure as Off on all three scenes, including one of 300,000 curves — in the tests above, and again in sixty more runs on 2026-08-30, in all four versions, without one exception. Whether it ever picks the other way — on other Apple hardware, or on some other kind of scene — is still not settled.
3. **Whether the four card-free Windows tests give the same answers.** They are written out ready to run in `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md` and wait on the next Windows rental.
4. **Everything needing hardware nobody here has**: an AMD card, an Intel Arc card, two cards of different ray-tracing generations in one machine, NVLink-joined cards, a Mac that is not Apple silicon, or an Apple machine an agent may open a window on. Each is named above with its reason.

---

2026-08-30 — Claude Opus 5 (Claude Code) — What a render demands, question 7, Blender 5.2

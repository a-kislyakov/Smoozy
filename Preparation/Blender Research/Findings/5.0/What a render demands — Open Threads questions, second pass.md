# What a render demands — Open Threads questions, second pass

**Blender 5.0 — question 7 only**

Sources consulted, all read on **2026-07-30** (the snapshot date given to me):

- The Blender 5.0 manual, as plain text, plus one picture from its `_images` folder
- The Blender 5.0 Python API documentation, as plain text
- The Blender 5.0 release notes
- Cycles' own files as supplied for 5.0 (the add-on's Python files and the kernel source)

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

I worked alone. I started no helper agents, so there is no helper's work I need to vouch for.

---

## 7. Where a render can be told which of a graphics card's rendering methods to use, what methods are there, what does choosing one rather than another change about the render, and does anything establish whether that changes with the size of the scene?

### A note on the reading I took

The question is ambiguous and I want to say so before answering.

The reading I answer is: **Cycles' choice of which technology it uses to compute a render on the graphics card** — CUDA, OptiX, HIP, oneAPI, Metal. The manual calls these "Rendering Technologies" and puts them under a setting called "Cycles Render Devices". That matches "which of a graphics card's rendering methods to use" closely, and it is about rendering.

The other reading is the setting Blender calls **"Backend"** under "Display Graphics" — OpenGL, Vulkan, Metal. That one is about how Blender *draws*, not how Cycles renders. I cover it briefly at the end of this answer, marked separately, because a reader could reasonably have meant it.

There is also a third thing that sits inside the first: within some of these technologies there is a further switch for whether the graphics card's dedicated ray-tracing hardware is used. I treat that as part of the first reading, because the manual documents it in the same panel.

---

### 7a. Where the choice is made

**LABEL: documented**

There are two places, and one of them overrides the other.

**Place one — the user preferences.** The manual's instructions for turning on graphics-card rendering are:

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

The preferences page describes the same setting:

> "Cycles Render Devices
>
> Changes the Computing Device the Cycles render engine uses to render images."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30.

I also opened the picture on that page, `_images/editors_preferences_section_system.webp`, referenced in the text as `[image: _images/editors_preferences_section_system.webp]` with the caption "Preferences System section." On that screenshot the "Cycles Render Devices" panel is a single row of three side-by-side buttons reading **None**, **CUDA**, **HIP**, with **None** highlighted. Below it is a "Display Graphics" panel with a "Backend" dropdown reading "OpenGL". What is visible: the choice is presented as one row where exactly one entry is picked, and on that particular machine only None, CUDA and HIP were offered. **LABEL for the picture: documented** (it is part of the manual).

Note the wording carefully: this is a **user preference**, not a scene setting. The scene setting is a separate, coarser one — the scene picks only between CPU and "GPU Compute":

> "GPU Compute:
>
> Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html — read 2026-07-30.

**Inferred step:** because the technology is chosen in the preferences and the scene chooses only CPU versus GPU Compute, the choice of technology is not carried in the scene. The sources do not state this in one sentence; I am joining the two statements above. The manual does say preferences live in a separate file, `./config/userpref.blend`, described simply as "User preferences" (manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html, read 2026-07-30). **LABEL for this step: inferred.**

**Place two — the command line, which overrides the preferences.** The manual documents:

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU."

and gives the example

> "blender -b file.blend -f 20 -- --cycles-device OPTIX"

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

Cycles' own files say plainly that this overrides both other places. **LABEL: observed.** In `engine.py`, in `_configure_argument_parser()`, the help text for that argument reads:

> `"Set the device to use for Cycles, overriding user preferences and the scene setting."`
> `"Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."`
> `"Additionally, you can append '+CPU' to any GPU type for hybrid rendering."`

File: `Blender Sources/cycles_5.0/engine.py`, lines 16–20 (inside `_configure_argument_parser`). Read 2026-07-30.

The same file shows when it is applied — during Cycles' `init()`, via `_parse_command_line()`, and that a bad value stops Blender:

> `    if args.cycles_device:`
> `        import _cycles`
> `        if not _cycles.set_device_override(args.cycles_device):`
> `            sys.exit(1)`

File: `Blender Sources/cycles_5.0/engine.py`, lines 38–41. Read 2026-07-30.

**Place three — from Python.** Cycles' own interface code reads the setting through the add-on preferences:

> `def get_device_type(context):`
> `    return context.preferences.addons[__package__].preferences.compute_device_type`

File: `Blender Sources/cycles_5.0/ui.py`, lines 96–97. Read 2026-07-30. **LABEL: observed.** That the same path can be written as well as read is not stated in these files; I did not find it stated, so I do not claim it.

I found **no environment variable** for this. See the record of looking below.

**Not covered:** I did not find, and so cannot say, whether `--cycles-device` also has to be accompanied by the scene being set to GPU Compute, or whether it forces that too. The manual's step list and the override wording pull in different directions and neither settles it.

---

### 7b. What methods there are

**LABEL: documented, and confirmed observed in Cycles' files**

The manual lists five, each tied to a maker and to particular operating systems. Its section is headed "Rendering Technologies":

> "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer and operating system."

- **CUDA – NVIDIA.** "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher."
- **OptiX – NVIDIA.** "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."
- **HIP – AMD.** "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."
- **oneAPI – Intel.** "oneAPI is a computation library that is supported on Windows and Linux and requires a Intel® Arc™ graphics card with the Xe HPG architecture."
- **Metal – Apple (macOS).** "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

The preferences page adds a sixth entry to the list of things you can pick, "None":

> "None:
>
> When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30.

Cycles' own file gives the same set as a fixed list, with names and numbers. **LABEL: observed.**

> `enum_device_type = (`
> `    ('CPU', "CPU", "CPU", 0),`
> `    ('CUDA', "CUDA", "CUDA", 1),`
> `    ('OPTIX', "OptiX", "OptiX", 3),`
> `    ('HIP', "HIP", "HIP", 4),`
> `    ('METAL', "Metal", "Metal", 5),`
> `    ('ONEAPI', "oneAPI", "oneAPI", 6)`
> `)`

File: `Blender Sources/cycles_5.0/properties.py`, lines 172–179. Read 2026-07-30. (Note the gap: there is no 2.)

The same file shows that the list offered is built at the moment the menu is drawn, from what the running build of Blender reports, always with "None" first: *(Citation check, 2026-08-30: **this was tested and the wording is wrong.** Finding 1 under "Tested by hand" records a machine with one NVIDIA card, no AMD card and no Intel graphics still offering HIP and oneAPI. The list is built from what that build of Blender supports, not from what the machine holds; what is empty is the device list underneath each choice. Corrected by this check.)*

> `    def get_device_types(self, context):`
> `        import _cycles`
> `        has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()`
> `        list = [('NONE', "None", n_("Do not use compute device"), 0)]`
> `        if has_cuda:`
> `            list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))`

File: `Blender Sources/cycles_5.0/properties.py`, lines 1581–1591 (further `if` branches follow for OptiX, HIP, Metal and oneAPI). Read 2026-07-30. This matches the picture, where only None, CUDA and HIP appeared.

The setting itself, with its default: **LABEL: observed.**

> `    compute_device_type: EnumProperty(`
> `        name="Compute Device Type",`
> `        description="Device to use for computation (rendering with Cycles)",`
> `        default=CyclesPreferences.default_device(),`
> `        items=CyclesPreferences.get_device_types,`
> `    )`

File: `Blender Sources/cycles_5.0/properties.py`, lines 1599–1604. Read 2026-07-30.

That default is not fixed. It depends on the machine:

> `    @staticmethod`
> `    def default_device():`
> `        import platform`
> `        # Default to selecting the Metal compute device on Apple Silicon GPUs`
> `        # (drivers are tightly integrated with macOS so pose no stability risk)`
> `        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):`
> `            return 5`
> `        return 0`

File: `Blender Sources/cycles_5.0/properties.py`, lines 1572–1579. Read 2026-07-30. Read against the numbers in `enum_device_type` above, 5 is Metal and 0 is CPU. **Inferred step:** on Apple Silicon the default is Metal, and elsewhere it is the CPU/None entry. That step is mine; the file states only the numbers.

**The further switch inside three of these methods.** Within HIP, oneAPI and Metal there is a second choice about whether the card's dedicated ray-tracing hardware is used. **LABEL: documented.**

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

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30.

Cycles' own files show these as three settings with their kinds and defaults. **LABEL: observed.** `metalrt` is a three-way choice defaulting to `'AUTO'` with items `('OFF', "Off", ...)`, `('ON', "On", ...)`, `('AUTO', "Auto", ...)`; `use_hiprt` is a true/false defaulting to `False` with the description `"HIP RT enables AMD hardware ray tracing on RDNA2 and above"`; `use_oneapirt` is a true/false named `"Embree on GPU"` defaulting to **`True`**. File: `Blender Sources/cycles_5.0/properties.py`, lines 1614–1636. Read 2026-07-30. The difference in defaults — AMD's off, Intel's on — is what the file says.

For OptiX there is no such switch in the sources I have. The manual instead states the hardware ray tracing as part of what OptiX is: "OptiX takes advantage of hardware ray-tracing acceleration in RTX graphics cards, for improved performance." (same manual page as above, read 2026-07-30).

Metal alone has one more setting attached to it, "Kernel Optimization", with three levels. **LABEL: observed.** Its description reads in part:

> `"Kernels can be optimized based on scene content. Optimized kernels are requested at the start of a render. "`
> `"If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set "`
> `"is available in the cache. This can result in additional CPU usage for a brief time (tens of seconds)"`

with items `('OFF', "Off", "Disable kernel optimization. Slowest rendering, no extra background CPU usage")`, `('INTERSECT', "Intersection only", "Optimize only intersection kernels. Faster rendering, negligible extra background CPU usage")` and `('FULL', "Full", "Optimize all kernels. Fastest rendering, may result in extra background CPU usage")`, default `'FULL'`. File: `Blender Sources/cycles_5.0/properties.py`, lines 1638–1651. Read 2026-07-30.

The add-on's own preferences file writes this and the MetalRT switch inside a test on Metal and a further test on the processor kind, and sets a row's `active` flag from a value gathered above:

> `        if compute_device_type == 'METAL':`
> `            import platform`
> `            # MetalRT only works on Apple Silicon.`
> `            if (platform.machine() == 'arm64'):`

and, a few lines on, `row.active = has_hardware_rt` before each of `metalrt`, `use_hiprt` and `use_oneapirt`. Above them:

> `        # Any device without RT support will disable it for all.`
> `        has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt`

File: `Blender Sources/cycles_5.0/properties.py`, lines 1901–1935 (the comment at 1907, the `has_hardware_rt` line at 1908, the Metal block from 1915). Read 2026-07-30. **Inferred step, the first:** that the two settings are therefore *shown* only on Apple Silicon. The step a reader must supply is that this block is what puts the controls on the screen and that a block not reached draws nothing; the file states neither. **Inferred step, the second:** when several cards are enabled together and any one of them lacks ray-tracing hardware, the hardware-ray-tracing switch is inactive for all of them. The comment says so; the surrounding code only greys out the control. I am reading the comment as the statement. *(Citation check, 2026-08-30: this passage called `properties.py` "Cycles' interface file" — the interface file is `ui.py` — gave the comment and the `has_hardware_rt` line as 1904–1905, where they are at 1907 and 1908, and read "only drawn on Apple Silicon" as observed. The first pass of this version labels that same step inferred. All three corrected by this check.)*

**Not covered:** the sources do not tell me whether the hardware-ray-tracing switches and the kernel-optimization level can be set from the command line. `--cycles-device` takes only the six device names. I found no argument for the others.

---

### 7c. What choosing one rather than another changes about the render

This splits into things the sources state outright and things they do not.

**What is documented as changing.**

**1. Which features work at all.** This is the clearest difference, and it changes the picture, not just the speed.

> "Limitations
>
> Path Guiding is not supported on any GPU.
>
> Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30. **LABEL: documented.**

The OSL page repeats it and lists what is lost even on OptiX:

> "OSL is not supported with GPU rendering unless using the OptiX backend.
>
> Some OSL features are not available when using the OptiX backend. Examples include:
>
> Memory usage reductions offered by features like on-demand texture loading and mip-mapping are not available.
>
> Texture lookups require OSL to be able to determine a constant image file path for each texture call.
>
> Some noise functions are not available. Examples include Cell, Simplex, and Gabor.
>
> The trace function is not functional. As a result of this, the Ambient Occlusion and Bevel nodes do not work."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/osl/index.html — read 2026-07-30. **LABEL: documented.**

Two shading-node pages state the same consequence from the other end. The Ambient Occlusion node "will not produce a valid result when… Open Shading Language is active while using the OptiX rendering backend" (manual, https://docs.blender.org/manual/en/5.0/render/shader_nodes/input/ao.html), and the Bevel node page carries the same line (manual, https://docs.blender.org/manual/en/5.0/render/shader_nodes/input/bevel.html). Both read 2026-07-30. **LABEL: documented.** These are differences in what comes out of the render, not in how fast it arrives.

Custom cameras are the same story: "Custom cameras are not supported with GPU rendering unless using the OptiX backend." Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/osl/camera.html — read 2026-07-30.

The Render Settings page states the general shape of it without naming methods:

> "GPU rendering may have limitations compared to CPU rendering, depending on the backend."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html — read 2026-07-30.

**2. Which denoising is available.** The denoiser choice is tied to the method:

> "OptiX:
>
> Uses NVIDIA's OptiX AI denoiser.
>
> Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.
>
> Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30. **LABEL: documented.**

And which cards can denoise on the graphics card at all differs by method: "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards"; "GPU accelerated denoising is available on discrete Radeon RX 6000 and Radeon RX 7000 GPUs"; for oneAPI, "GPU accelerated denoising is available on all supported GPUs"; for Metal, "GPU accelerated denoising is available on Apple Silicon." Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

**3. What happens when the card runs out of memory.** This one names four of the five methods and leaves one out:

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30. **LABEL: documented for the four named.** oneAPI is absent from that list. **Inferred step, and I want to flag how weak it is:** the sentence names four of five methods, which reads as deliberate, but the manual nowhere says oneAPI lacks the fallback. Absence from a list is not a statement. Treat "oneAPI does not fall back to system memory" as unproven.

**4. Sharing memory between several cards.** Only one method can do it:

> "Would multiple GPUs increase available memory?
>
> Typically, no, each GPU can only access its own memory.
>
> The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30. The preferences page adds: "Currently only NVLink on NVIDIA GPUs is supported." Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30. **LABEL: documented.**

**5. Whether the CPU can be used alongside.** The Render Settings page says "Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously" (manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html, read 2026-07-30), and the command line offers "+CPU" appended to any GPU type (manual, arguments page, read 2026-07-30). The sources do not say which backends this works for. **LABEL: documented that it varies; not in the sources which ones.**

**6. Which structure is used to trace rays, and which of the scene's own acceleration settings still apply.** The Cycles kernel source shows the choice of intersection structure is made per method at build time. **LABEL: observed.**

> `/* Device specific acceleration structures for ray tracing. */`
> `#if defined(__EMBREE__)`
> `#  include "kernel/device/cpu/bvh.h"`
> `#  define __BVH2__`
> `#elif defined(__METALRT__)`
> `#  include "kernel/device/metal/bvh.h"`
> `#elif defined(__KERNEL_OPTIX__)`
> `#  include "kernel/device/optix/bvh.h"`
> `#elif defined(__HIPRT__)`
> `#  include "kernel/device/hiprt/bvh.h"`
> `#else`
> `#  define __BVH2__`
> `#endif`

File: `Blender Sources/cycles_5.0/source/kernel/bvh/bvh.h`, lines 17–30. Read 2026-07-30. A few lines further the fallback is described:

> `/* BVH2`
> ` *`
> ` * Bounding volume hierarchy for ray tracing, when no native acceleration`
> ` * structure is available for the device.`

Same file, lines 58–61. Read 2026-07-30.

**Inferred step:** a method with hardware ray tracing available uses that hardware's own structure; otherwise Cycles falls back to its own BVH2. The file states the routing and states what BVH2 is for; joining them into that sentence is my step.

More consequential for a person setting up a render: Cycles' interface hides the scene's Acceleration Structure panel when OptiX is in use **and no second kind of device is in use alongside it**. **LABEL: observed.**

> `class CYCLES_RENDER_PT_performance_acceleration_structure(CyclesButtonsPanel, Panel):`
> `    bl_label = "Acceleration Structure"`
> `    bl_parent_id = "CYCLES_RENDER_PT_performance"`
> `    @classmethod`
> `    def poll(cls, context):`
> `        return not use_optix(context) or use_multi_device(context)`

File: `Blender Sources/cycles_5.0/ui.py`, lines 880–887. Read 2026-07-30. *(Citation check, 2026-08-30: this read "hides … entirely when OptiX is in use", which the quoted line itself does not carry — `return not use_optix(context) or use_multi_device(context)` returns true again when a multi-device is in use. The word "entirely" has been dropped and the exception written in. Hand test 10 under "Tested by hand" shows the same exception on a running Blender. Changed by this check.)*

That panel is where the manual documents Use Spatial Splits, Use Curves BVH, BVH Time Steps and Use Compact BVH — settings the manual describes as trading memory against render time, for example "Use Compact BVH — Use a more compact BVH structure, which can reduce RAM usage but render slower" (manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html, read 2026-07-30). **Inferred step, and this one matters:** the panel being hidden under OptiX suggests those settings do not apply there. But hiding a panel is not the same as ignoring a value, and the underlying properties still exist in the scene. Whether Cycles ignores them under OptiX is **not in the sources** — the host-side code that would show it is not among the Cycles files I was given (I have only the add-on's Python and the kernel source; there is no `device/` or `scene/` host code).

**What is documented as *not* being settled by the sources: which is faster.**

> "What renders faster?
>
> This varies depending on the hardware used. Different technologies also have different compute times depending on the scene tested. For the most up to date information on the performance of different devices, browse the Blender Open Data resource."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30. **LABEL: documented — documented as unanswerable from the manual.** The manual explicitly declines to rank them and points outside itself.

**Nothing in the sources says the choice of method changes the pixels**, apart from the feature limitations in point 1 and the denoiser differences in point 2. I looked for such a statement and did not find one. **LABEL for that: not in the sources.** Where I looked is recorded below. This is the honest and important gap: I cannot tell you from these sources whether the same scene rendered under CUDA and under OptiX produces identical images.

**Not covered by 7c:** anything about how much slower or faster; anything about numerical differences between the methods; whether a render started under one method and a render started under another are byte-for-byte comparable; what happens when a method is chosen that the hardware cannot support.

---

### 7d. Does anything establish whether that changes with the size of the scene?

**LABEL: documented in part, not in the sources for the rest**

Four statements in the sources tie a method's behaviour to how big or heavy the scene is. None of them is a rule, and none gives a threshold.

**1. The manual says the ranking varies by scene.** Quoted in full above: "Different technologies also have different compute times depending on the scene tested." (manual, GPU Rendering page, read 2026-07-30). This is the closest the sources come to answering the question directly, and what it establishes is only that the answer varies — not how.

**2. MetalRT's benefit is tied to what the scene contains.**

> "MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30. **LABEL: documented.** The same words appear in Cycles' own file as the setting's description, at `Blender Sources/cycles_5.0/properties.py`, lines 1616–1617 (**observed**). So the effect of turning MetalRT on is stated to depend on scene content — specifically on curves — rather than being fixed.

**3. Running out of card memory is the failure that scales with scene size, and the methods differ there.** The manual's answer to "Why does a scene that renders on the CPU not render on the GPU?" is:

> "There may be multiple causes, but the most common one is that there is not enough memory on your graphics card. Typically, the GPU can only use the amount of memory that is on the GPU… This is usually much smaller than the amount of system memory the CPU can access."

followed by the CUDA/OptiX/HIP/Metal fallback sentence quoted in 7c point 3. Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30. **LABEL: documented.** And the memory-sharing option is described in terms of scene size — Cycles' own file describes `peer_memory` as:

> `        description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it",`

File: `Blender Sources/cycles_5.0/properties.py`, line 1610. Read 2026-07-30. **LABEL: observed.**

**Inferred step:** the point at which the choice of method starts to matter for whether a render completes at all is the point at which the scene stops fitting in the card's memory, and that point depends on scene size. The manual states each half; the joining is mine.

**4. Metal's kernel optimization is stated to depend on scene content**, though this is about how the render starts rather than the choice of method: "Kernels can be optimized based on scene content" (quoted in full in 7b). File: `Blender Sources/cycles_5.0/properties.py`, line 1640. Read 2026-07-30. **LABEL: observed.**

**And one adjacent statement about heaviness that is not about the choice between methods**, but is close enough that I record it so nobody mistakes it for one:

> "On older GPU generations, graphics cards can only either render or draw the user interface. This can make Blender unresponsive while it is rendering. Heavy scenes can also make Blender unresponsive on newer GPUs, when using a lot of memory or executing expensive shaders, however this is generally less of a problem."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30. **LABEL: documented.** This is about the card being shared with the display, not about which method is chosen.

**What is not in the sources.** No threshold, no measurement, no rule of the form "above such-and-such a scene size, prefer this method". No statement that the *ordering* of the methods flips with scene size. No statement that the MetalRT "Auto" setting decides using scene size — the manual says only "Automatically pick the fastest intersection method", and the code that makes that decision is not among the Cycles files I was given. **LABEL: not in the sources.**

**Not covered by 7d:** what "size" means here — the sources speak of memory used, of scenes with curves, of a mix of large and small polygons, and of "heavy" scenes, but never define a measure. Anyone using this answer should not assume "size" means polygon count.

---

### The other reading, recorded briefly

If the question meant the **Display Graphics Backend** rather than Cycles' compute technology: **LABEL: documented.**

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

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html — read 2026-07-30.

From the command line: "--gpu-backend — Force to use a specific GPU backend. Valid options: vulkan, metal, opengl." Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

From Python it is readable and writable as a preference: `gpu_backend` — "GPU backend to use (requires restarting Blender for changes to take effect)", an enum of `OPENGL`, `METAL`, `VULKAN`. Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.PreferencesSystem.html — read 2026-07-30. It can also be read back with `gpu.platform.backend_type_get()`, which "Get actuve GPU backend" and returns one of "'OPENGL', 'VULKAN', 'METAL', 'NONE', 'UNKNOWN'". Source: API documentation, https://docs.blender.org/api/5.0/gpu.platform.html — read 2026-07-30.

Two consequences are documented. Some preferences depend on it — "This option is not available on macOS and requires using OpenGL backend" appears twice on the preferences page (manual, system page, lines around the Display Graphics section, read 2026-07-30) — and one release note says "HDR and wide gamut are supported on Windows and Linux + Wayland using the Vulkan backend." Source: release notes, https://developer.blender.org/docs/release_notes/5.0/eevee/ — read 2026-07-30.

I found **nothing** in the sources tying this backend choice to scene size. **LABEL: not in the sources.**

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

**LABEL: documented for the command line; inferred for the rest.**

The choice can be made entirely from outside Blender's window, and the manual's own example is a background render: `blender -b file.blend -f 20 -- --cycles-device OPTIX` (manual, arguments page, read 2026-07-30). Cycles' own file states this overrides both the preferences and the scene setting (`engine.py` lines 16–20, read 2026-07-30), which is exactly the case that matters when nobody is clicking.

Where it differs from inside:

- The *panel* is only in the preferences window, so without `--cycles-device` a headless run takes whatever the preferences file on that machine holds. **Inferred step:** the manual says the technology is a "user preference" and lists `./config/userpref.blend` as where user preferences live; from those two I conclude a headless run depends on that per-machine file. The sources do not say this in one place.
- The MetalRT, HIP RT, Embree-on-GPU and Kernel Optimization switches have no command-line argument that I found. `--cycles-device` accepts only the six device names. So those appear to be settable only through the preferences (and possibly through Python). **LABEL for the absence: not in the sources** — see the record of looking.
- The greying-out behaviour I quoted from `properties.py` is drawing code. **Inferred step:** it governs what a person sees, not what a headless render does. Whether the same "any device without RT support disables it for all" rule is applied during an actual render is not visible in the files I have.
- Cycles' `init()` calls `_cycles.init(path, user_path, bpy.app.background)` and then parses the command line (`engine.py`, lines 44–53, read 2026-07-30) — **observed** — so the argument is handled on the same path whether or not Blender is in background mode. That the *effect* is the same is my inference, not a statement in the file.

For the Display Graphics Backend reading, `--gpu-backend` is likewise a command-line argument (manual, arguments page, read 2026-07-30), and the preference is exposed in the Python API as `gpu_backend` (API, PreferencesSystem page, read 2026-07-30) — though "requires restarting Blender for changes to take effect", which constrains setting it from a script inside a running Blender.

**Not covered:** whether `--cycles-device` alone is enough without the scene also being set to GPU Compute; whether a bad device name is reported in a way a caller outside Blender can act on beyond the exit (`sys.exit(1)` is what the file shows).

---

### Operating system

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Tested rather than inferred: the Creator's Mac offers only `None` and `Metal` and starts at `Metal`; a rented Linux machine with one NVIDIA card offers `None`, `CUDA`, `OptiX`, `HIP` and `oneAPI` and starts at `None`. So the list of methods does differ by operating system — **but it is built from what that build supports rather than from the cards present**, since the Linux machine offered HIP and oneAPI with no AMD and no Intel graphics in it. `--gpu-backend` accepts `opengl` and `vulkan` on Linux and only `metal` on macOS, naming the rejected value in an error and then finishing the render anyway. **Windows is not settled**: this project's route to a Windows computer is a rental with two general-purpose cores and no graphics card at all, so four card-free Windows tests are written out ready to run and wait on the next Windows batch.]*

**LABEL: documented.** This does *not* hold on every operating system — the whole point of the setting is that the methods differ by system, and the manual is explicit:

> "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer and operating system."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

From the quotes in 7b:

- **CUDA** — Windows and Linux.
- **OptiX** — Windows and Linux.
- **HIP** — Windows and Linux, with different minimum drivers per system: "Windows: Radeon Software 24.9.1 or Radeon PRO Software 24.Q4 / Linux: Radeon Software 23.40 or ROCm 6.0".
- **oneAPI** — Windows and Linux, again with different minimum drivers per system.
- **Metal** — "Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

All from the same manual page, read 2026-07-30.

So on macOS the only listed method is Metal, and CUDA, OptiX, HIP and oneAPI are not listed for macOS at all. **Inferred step:** that they are unavailable there. The manual lists supported systems rather than stating exclusions.

The Metal-only extras are gated by system in code as well: the MetalRT and Kernel Optimization controls are drawn only when `platform.machine() == 'arm64'`, with the comment "MetalRT only works on Apple Silicon" (`Blender Sources/cycles_5.0/properties.py`, lines 1915–1918, read 2026-07-30) — **observed**. And Cycles' interface file notes a system-version dependency elsewhere: "The MNEE kernel doesn't compile on macOS < 13" (`Blender Sources/cycles_5.0/ui.py`, lines 196–197, read 2026-07-30) — **observed**.

For the Display Graphics Backend reading, the command line accepts "vulkan, metal, opengl" (manual, arguments page, read 2026-07-30), and the manual states of two other preferences that they are "not available on macOS and requires using OpenGL backend" (manual, system page, read 2026-07-30). **LABEL: documented.**

One release note records a change in this area for 5.0: "The minimum required Windows AMD GPU driver has increased to Adrenalin version 24.9.1 or Radeon Pro driver 24.Q4" and "The minimum supported compute capability is `sm_50` now (~GeForce 900 cards)". Source: release notes, https://developer.blender.org/docs/release_notes/5.0/cycles/ — read 2026-07-30. The version index also lists "Cycles: Increased minimum GPU driver requirements for AMD HIP on Windows" (release notes, https://developer.blender.org/docs/release_notes/5.0/, read 2026-07-30).

**Not covered:** the sources say nothing about Linux distributions or Windows versions beyond driver numbers, and nothing about systems Blender runs on other than Windows, Linux and macOS.

---

## Record of where I looked

I searched everything I was given rather than opening likely pages, as instructed. Recorded so that the absences above can be checked.

**Searches run across the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` at once** (case-insensitive, whole-tree):

`CUDA`, `OptiX`, `HIP`, `oneAPI`, `Metal`, `MetalRT`, `Compute Device`, `device type`, `hardware ray`, `ray tracing`, `RT core`, `Vulkan`, `OpenCL`, `backend`, `back-end`.

Second sweep, for scene-size dependence: `large scene`, `complex scene`, `scene size`, `depending on the scene`, `scene complexity`, `heavy scene`, `scene content`.

Third sweep, for the acceleration structures: `BVH`, `acceleration structure`, `spatial split`, `compact bvh`, `curves bvh`, `instancing`.

Fourth sweep, for other places the choice might be made: `compute backend`, `render device`, `GPU Compute`, `cycles-device`, `gpu-backend`, `--gpu`, `CYCLES_[A-Z_]*`, `environment variable`, `userpref`.

**Pages opened in full and read:**

- manual: `render/cycles/gpu_rendering.txt` (whole page, 221 lines) → https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html
- manual: `editors/preferences/system.txt` (lines 1–120, plus a grep of the rest) → https://docs.blender.org/manual/en/5.0/editors/preferences/system.html
- manual: `render/cycles/render_settings/index.txt` (lines 1–70) → https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/index.html
- manual: `render/cycles/render_settings/performance.txt` (lines 1–110) → https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html
- manual: `advanced/command_line/arguments.txt` (the Cycles Render Options block, the GPU Options block, and the Environment Variables block) → https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html
- manual: `render/cycles/osl/index.txt` (the Limitations section) → https://docs.blender.org/manual/en/5.0/render/cycles/osl/index.html
- manual: `render/cycles/osl/camera.txt` (grep) → https://docs.blender.org/manual/en/5.0/render/cycles/osl/camera.html
- manual: `render/cycles/render_settings/sampling.txt` (grep, denoiser section) → https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html
- manual: `render/shader_nodes/input/ao.txt` and `render/shader_nodes/input/bevel.txt` → https://docs.blender.org/manual/en/5.0/render/shader_nodes/input/ao.html and .../bevel.html
- manual: `advanced/blender_directory_layout.txt` (the user-directory listing) → https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html
- API: `bpy.types.PreferencesSystem.txt` (grep for backend and device entries) → https://docs.blender.org/api/5.0/bpy.types.PreferencesSystem.html
- API: `gpu.platform.txt` (whole page) → https://docs.blender.org/api/5.0/gpu.platform.html
- release notes: `cycles.md` (all headings listed, the GPU section read in full) → https://developer.blender.org/docs/release_notes/5.0/cycles/
- release notes: `rendering.md`, `core.md`, `eevee.md`, `index.md`, `python_api.md`, `pipeline_io.md`, `corrective_releases.md` (grepped for the device terms) → https://developer.blender.org/docs/release_notes/5.0/rendering/ etc.

**Picture opened:** `manual_5.0/_images/editors_preferences_section_system.webp`, referenced from the System preferences page. Converted to PNG in a scratch folder purely to view it; the file in `Blender Sources/` was not modified.

**Cycles files opened:** `properties.py` (the device enums at 30–48 and 172–179; the render settings device at 378–405; the acceleration-structure debug properties at 905–945 and 1065–1095; `CyclesDeviceSettings` and `CyclesPreferences` at 1555–1660; the device drawing and warnings at 1750–1940), `engine.py` (argument parsing and `init`, lines 11–78), `ui.py` (device helpers at 96–205, the Acceleration Structure panel at 880–935, `draw_device` at 2455–2485), `source/kernel/bvh/bvh.h` (lines 1–80), `source/kernel/device/metal/bvh.h` (lines 1–60), and a listing of `source/kernel/device/` (which contains `cuda`, `gpu`, `hip`, `hiprt`, `metal`, `optix`).

**Ruled out, and why:**

- `manual_5.0/getting_started/about/history.txt` mentions hardware ray-tracing and the various technologies. I did not use it: it is a list of what changed in past versions, and quoting it would be comparing versions, which I was told not to do. It contains nothing about the choice or its effects that the GPU Rendering page does not carry.
- `manual_5.0/troubleshooting/gpu/linux/nvidia.txt` and `.../amd.txt` matched on Vulkan and OpenCL. Read enough to confirm they are about driver installation and display problems, not about the rendering method choice.
- The many `cycles_5.0/source/util/*.h` and `source/kernel/**` files that matched CUDA, Metal, oneAPI and so on. These are compile-time switches inside maths and type definitions, present in almost every kernel file; they say nothing about the choice or its effects.
- `manual_5.0/_images/editors_vse_*.svg` matched CUDA/OptiX/Metal. These are video-sequencer diagrams; the matches are inside embedded font or path data, not content.
- Everything matching "Metal" in the shading-node pages (Principled, Metallic, Glossy, Sheen, Voronoi, and so on) — those are about metallic *materials*, not the Metal technology.
- `manual_5.0/addons/import_export/anim_bvh.txt` and the sculpt "Rebuild BVH" pages matched BVH. Different subject entirely — motion capture files and sculpting — ruled out on reading.
- `release_notes_5.0/bugfixes.md` matched most of the terms. It is a list of fixed bugs; I found nothing there stating what a choice of method changes. `corrective_releases.md` had one HIP RT crash entry, noted but not load-bearing.

**Where the sources ran out:**

- There is no host-side Cycles code in `Blender Sources/cycles_5.0` — only the add-on's Python and `source/kernel` plus `source/util`. So the code that would decide MetalRT "Auto", that would choose a BVH layout at render time, or that would honour or ignore the scene's acceleration-structure settings under each method, is not present. Any question of the form "what does Cycles actually do at render time" cannot be settled from what I was given.
- I found no environment variable that selects the rendering method. The manual's Environment Variables list contains only `BLENDER_USER_*`, `BLENDER_SYSTEM_*`, `BLENDER_CUSTOM_SPLASH`, `BLENDER_CUSTOM_SPLASH_BANNER`, `BLENDER_OCIO`, `TEMP` and `TMPDIR`. The only Cycles environment variable named anywhere is `CYCLES_CUDA_EXTRA_CFLAGS`, and the manual uses it for a compiler workaround on Linux, not for choosing a method.
- I found no way documented to set MetalRT, HIP RT, Embree on GPU or Kernel Optimization from the command line.
- I did not go to the web at any point, and did not need to.

---

## Things a running Blender could still settle

*(Pruned 2026-08-30 by the hand tests. Everything the earlier list held has either been run — the results are above, under **Tested by hand — what a running Blender showed** — or is recorded there as unable to be run, with its reason. What follows is what is genuinely still open.)*

1. **What happens when the card's own memory fills.** The scene of 125,829,122 triangles reached the host machine's memory ceiling and not the card's, so the documented fallback to system memory was never exercised. Settling it needs a machine whose card is small against what its host can build, which is the opposite of the usual want.
2. **What MetalRT's "Auto" weighs.** On the Creator's Mac it built the same structure as Off on all three scenes, including one of 300,000 curves — in the tests above, and again in sixty more runs on 2026-08-30, in all four versions, without one exception. Whether it ever picks the other way — on other Apple hardware, or on some other kind of scene — is still not settled.
3. **Whether the four card-free Windows tests give the same answers.** They are written out ready to run in `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md` and wait on the next Windows rental.
4. **Everything needing hardware nobody here has**: an AMD card, an Intel Arc card, two cards of different ray-tracing generations in one machine, NVLink-joined cards, a Mac that is not Apple silicon, or an Apple machine an agent may open a window on. Each is named above with its reason.

---

2026-08-30 — Claude Opus 5 (Claude Code) — What a render demands, Blender 5.0

# What a render demands — Open Threads questions

**Blender 4.5. Question 7 only.**

Sources consulted, all inside `Blender Sources/`, all read on **2026-07-30** (the snapshot date given for these copies): the Blender 4.5 manual (`manual_4.5`, including one image from `manual_4.5/_images`), the Blender 4.5 Python API documentation (`api_4.5`), the Blender 4.5 release notes (`release_notes_4.5`), and Cycles' own files (`cycles_4.5`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 7. Where a render can be told which of a graphics card's rendering methods to use, what methods are there, what does choosing one rather than another change about the render, and does anything establish whether that changes with the size of the scene?

### The question was ambiguous, and here is the reading I answered

"A graphics card's rendering methods" could mean three different things in Blender 4.5, and the sources treat them as three different settings in different places:

1. **The Cycles rendering technology** — CUDA, OptiX, HIP, oneAPI, Metal. The manual's own heading for this set is "Rendering Technologies", and each one is a manufacturer's way of driving a graphics card for rendering.
2. **The hardware ray-tracing switches inside that same choice** — MetalRT, HIP RT, Embree on GPU. These pick whether the card's ray-tracing hardware is used or a software structure is used instead, on a card that is already chosen.
3. **The Display Graphics backend** — OpenGL or Vulkan — which is the drawing method, not the render method, but is a "graphics API" choice on a graphics card.

**I answer reading 1 as the main answer, and reading 2 alongside it**, because the sources put both in the same panel and both are Cycles render settings. Reading 3 I answer briefly and separately at the end, so the answer holds either way. Two further things that sound like this question but are not, and why, are also listed at the end.

---

### (a) Where the choice can be made

**Three places, and they do different jobs.**

**Preferences — the technology.** This is where the technology itself is picked. It is not in the scene.

> "To enable GPU rendering, go into the Preferences ‣ System ‣ Cycles Render Devices, and select either CUDA, OptiX, HIP, oneAPI, or Metal. Next, you must configure each scene to use GPU rendering in Properties ‣ Render ‣ Device."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30

The same location is given a second time:

> "enable compatible devices in Preferences ‣ System ‣ Cycles Render Devices."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/introduction.html
- Date read: 2026-07-30

The manual's picture of the Preferences System section shows a panel headed **"Cycles Render Devices"** as the first panel on that page, above "Memory & Limits", "Video Sequencer" and "Sound". In this picture the panel is open but shows nothing inside it, and no device buttons are visible.

- Label: **documented** (it is part of the manual)
- Source: manual image `_images/editors_preferences_section_system.png`, shown on https://docs.blender.org/manual/en/4.5/editors/preferences/system.html
- Date read: 2026-07-30
- Note: this picture does not show the "Display Graphics", "Network" or "Operating System Settings" sections that the same page's text describes. I report only what is visible in it.

**The scene — only whether the card is used at all.** The scene's own setting is a two-way choice, not a technology choice:

> `enum_devices = (`
> `    ('CPU', "CPU", "Use CPU for rendering"),`
> `    ('GPU', "GPU Compute",`
> `        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),`
> `)`

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/properties.py`, lines 30–34 (the list is used at line 381 by the `device` setting defined at line 378)
- Date read: 2026-07-30

**The command line — an option that Cycles' own files describe as beating both.** Started from outside Blender, one option sets the device:

> "--cycles-device <device>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
> Append +CPU to a GPU device to render on both CPU and GPU.
> Example:
> blender -b file.blend -f 20 -- --cycles-device OPTIX"

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

Cycles' own files carry the help text written beside that option:

> `parser.add_argument("--cycles-device",`
> `                    help="Set the device to use for Cycles, overriding user preferences and the scene setting."`
> `                         "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."`
> `                         "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",`
> `                    default=None)`

- Label: **observed** — that these words sit in that file
- Source: Cycles' own files — `cycles_4.5/engine.py`, lines 16–20
- Date read: 2026-07-30
- That the option really does beat the stored preference and the scene when a render runs is **inferred**, and the step is that this sentence is the help text written beside the option rather than a record of a render. Hand test 5 below is aimed at it.

The same file shows that Cycles defines exactly **two** command-line options and no more — `--cycles-print-stats` and `--cycles-device`. Cycles therefore offers no command-line option of its own for MetalRT, HIP RT, Embree on GPU, or Kernel Optimization.

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/engine.py`, lines 13–20
- Date read: 2026-07-30
- This says only what Cycles defines. Whether Blender itself defines one elsewhere is covered by the command-line page search recorded in "Where I looked".

**The hardware ray-tracing switches — Preferences only.** Cycles' own files show the three ray-tracing switches, and three settings beside them, all sitting on the Cycles preferences object rather than on the scene:

> `class CyclesPreferences(bpy.types.AddonPreferences):`
> `    bl_idname = __package__`

(`cycles_4.5/properties.py`, lines 1555–1556), with `compute_device_type` at line 1585, `peer_memory` at 1594, `metalrt` at 1600, `use_hiprt` at 1612, `use_oneapirt` at 1618 and `kernel_optimization_level` at 1624.

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/properties.py`, lines 1555–1635
- Date read: 2026-07-30

Where preferences are kept:

> "./config/userpref.blend
> User preferences."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html
- Date read: 2026-07-30

And when they are written:

> "By default changes to preferences are saved on exit, this allows changes to the keymap and Quick Favorites menu to be stored and used between Blender Sessions.
> When disabled, a Save Preferences button is shown to manually perform the operation."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/editors/preferences/introduction.html
- Date read: 2026-07-30

---

### (b) What methods there are

**The five rendering technologies.** The manual names them under the heading "Rendering Technologies", with this opening line:

> "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer and operating system."

and then, in its own words for each:

> "CUDA – NVIDIA
> CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 3.0 and higher."

> "OptiX – NVIDIA
> OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."

> "HIP – AMD
> HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."

> "oneAPI – Intel
> oneAPI is a computation library that is supported on Windows and Linux and requires a Intel® Arc™ graphics card with the Xe HPG architecture. Hardware acceleration for ray-tracing and denoising is supported."

> "Metal – Apple (macOS)
> Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30

The Preferences page adds a sixth entry to the list — the absence of any of them:

> "None:
> When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/editors/preferences/system.html
- Date read: 2026-07-30

Cycles' own files carry the same six names, each added to the list only when a matching test passes:

> `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()`
>
> `list = [('NONE', "None", n_("Do not use compute device"), 0)]`
> `if has_cuda:`
> `    list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))`
> `if has_optix:`
> `    list.append(('OPTIX', "OptiX", n_("Use OptiX for GPU acceleration"), 3))`
> `if has_hip:`
> `    list.append(('HIP', "HIP", n_("Use HIP for GPU acceleration"), 4))`
> `if has_metal:`
> `    list.append(('METAL', "Metal", n_("Use Metal for GPU acceleration"), 5))`
> `if has_oneapi:`
> `    list.append(('ONEAPI', "oneAPI", n_("Use oneAPI for GPU acceleration"), 6))`

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/properties.py`, lines 1569–1581
- Date read: 2026-07-30
- That this means each technology is offered only on a machine that can do it is **inferred**, and the step is what `get_device_types` reports. That part of Cycles is not in the files I was given, so nothing I read says what it answers. Hand tests 1 and 2 below are aimed at what the panel actually offers.
- *(Citation check, 2026-08-30: **that inference was tested and it is wrong.** Finding 1 under "Tested by hand" records a machine with one NVIDIA card, no AMD card and no Intel graphics still offering HIP and oneAPI. The list is built from what that build of Blender supports, not from the cards present; what is empty is the device list underneath each choice. Read this paragraph with finding 1 beside it. Changed by this check.)*

The setting itself, with its description and its starting value:

> `compute_device_type: EnumProperty(`
> `    name="Compute Device Type",`
> `    description="Device to use for computation (rendering with Cycles)",`
> `    default=CyclesPreferences.default_device(),`
> `    items=CyclesPreferences.get_device_types,`
> `)`

and the starting value itself:

> `    @staticmethod`
> `    def default_device():`
> `        import platform`
> `        # Default to selecting the Metal compute device on Apple Silicon GPUs`
> `        # (drivers are tightly integrated with macOS so pose no stability risk)`
> `        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):`
> `            return 5`
> `        return 0`

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/properties.py`, lines 1585–1590 and 1558–1565
- Date read: 2026-07-30
- The number 5 is the number given to `METAL` in the list quoted above, and 0 the number given to `NONE`. Reading it as "Metal on Apple Silicon, otherwise None" is one small step past the text, so that reading is **inferred**.

**The hardware ray-tracing methods.** Three settings, one per manufacturer, described in the manual as:

> "Embree on GPU
> Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance.
> Only supported with oneAPI rendering devices."

> "HIP RT
> Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above.
> This feature is only available when using a HIP render device."

> "MetalRT
> MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases.
> Off:
> Disable MetalRT (uses BVH2 layout for intersection queries).
> On:
> Enable MetalRT for intersection queries.
> Auto:
> Automatically pick the fastest intersection method."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/editors/preferences/system.html
- Date read: 2026-07-30

Cycles' own files give the same three settings with their kinds and starting values:

> `metalrt: EnumProperty(`
> `    name="MetalRT",`
> `    description="MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better "`
> `                "performance in specific cases",`
> `    default='AUTO',`
> `    items=(`
> `        ('OFF', "Off", "Disable MetalRT (uses BVH2 layout for intersection queries)"),`
> `        ('ON', "On", "Enable MetalRT for intersection queries"),`
> `        ('AUTO', "Auto", "Automatically pick the fastest intersection method"),`
> `    ),`
> `)`
>
> `use_hiprt: BoolProperty(`
> `    name="HIP RT",`
> `    description="HIP RT enables AMD hardware ray tracing on RDNA2 and above",`
> `    default=False,`
> `)`
>
> `use_oneapirt: BoolProperty(`
> `    name="Embree on GPU",`
> `    description="Embree on GPU enables the use of hardware ray tracing on Intel GPUs, providing better overall performance",`
> `    default=True,`
> `)`

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/properties.py`, lines 1600–1622
- Date read: 2026-07-30
- So: MetalRT starts at Auto, HIP RT starts off, Embree on GPU starts on. There is no equivalent switch for CUDA or OptiX in this file.

The drawing code puts each one behind a test, and sets each one's `active` from a value whose own comment says a card without support disables it for all:

> `        # Any device without RT support will disable it for all.`
> `        has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt`
> ...
> `        if compute_device_type == 'METAL':`
> `            import platform`
> `            # MetalRT only works on Apple Silicon.`
> `            if (platform.machine() == 'arm64'):`
> `                col = layout.column()`
> `                col.use_property_split = True`
> `                col.prop(self, "kernel_optimization_level")`
> `                row = col.row()`
> `                row.active = has_hardware_rt`
> `                row.prop(self, "metalrt")`
> `        if compute_device_type == 'HIP':`
> `            import platform`
> `            row = layout.row()`
> `            row.active = has_hardware_rt`
> `            row.prop(self, "use_hiprt")`
> `        elif compute_device_type == 'ONEAPI' and _cycles.with_embree_gpu:`
> `            row = layout.row()`
> `            row.active = has_hardware_rt`
> `            row.prop(self, "use_oneapirt")`

- Label: **observed** — that these words sit in that file
- Source: Cycles' own files — `cycles_4.5/properties.py`, inside the `draw_impl` method which begins at line 1861; the `has_hardware_rt` line is 1890, the Metal block begins at 1897, `metalrt` is drawn at 1907, the HIP block at 1909 with `use_hiprt` at 1913, and the oneAPI block at 1915 with `use_oneapirt` at 1918
- Date read: 2026-07-30
- Reading this as "each switch is hidden unless its technology is chosen, and greyed out unless every card in use supports hardware ray tracing" is a step past the words, so that reading is **inferred**. The step is twofold: that a block not reached draws nothing, and that `active` set false is what a reader sees as greyed out. I keep it apart from the quote because it is the same kind of step this document labels inferred for the Debug panel's `"CPU"` heading further down. Hand test 2 below is aimed at it.

**A longer list of intersection layouts exists inside Cycles, but it is not offered as a choice.** Cycles' kernel files name fourteen layouts:

> `enum KernelBVHLayout {`
> `  BVH_LAYOUT_NONE = 0,`
>
> `  BVH_LAYOUT_BVH2 = (1 << 0),`
> `  BVH_LAYOUT_EMBREE = (1 << 1),`
> `  BVH_LAYOUT_OPTIX = (1 << 2),`
> `  BVH_LAYOUT_MULTI_OPTIX = (1 << 3),`
> `  BVH_LAYOUT_MULTI_OPTIX_EMBREE = (1 << 4),`
> `  BVH_LAYOUT_METAL = (1 << 5),`
> `  BVH_LAYOUT_MULTI_METAL = (1 << 6),`
> `  BVH_LAYOUT_MULTI_METAL_EMBREE = (1 << 7),`
> `  BVH_LAYOUT_HIPRT = (1 << 8),`
> `  BVH_LAYOUT_MULTI_HIPRT = (1 << 9),`
> `  BVH_LAYOUT_MULTI_HIPRT_EMBREE = (1 << 10),`
> `  BVH_LAYOUT_EMBREEGPU = (1 << 11),`
> `  BVH_LAYOUT_MULTI_EMBREEGPU = (1 << 12),`
> `  BVH_LAYOUT_MULTI_EMBREEGPU_EMBREE = (1 << 13),`

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/source/kernel/types.h`, lines 1386–1402
- Date read: 2026-07-30
- These are names inside Cycles. Nothing I read offers them to a user as a list to pick from. I do not claim any more about them than that they exist under these names.

---

### (c) What choosing one rather than another changes about the render

The sources describe differences of **four kinds**. I have kept them apart because they are not equally well documented.

**1. Which cards and operating systems each technology is stated to support.** Covered in (b) — CUDA, OptiX, HIP and oneAPI are stated as Windows and Linux; Metal as Apple Silicon with macOS 13.0 or newer; each with its own minimum card and driver.

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30
- Reading that as "choosing a technology therefore decides whether the render runs on that machine at all" is a step past the words, so that reading is **inferred**. The step is that a machine outside a stated support list cannot use that technology, which the page states as support rather than as what happens on an unsupported machine. Hand test 4 below is aimed at it.

**2. Which Cycles features are stated to be available.** Every quote below is about availability. That an unavailable feature changes the finished picture rather than only the speed is a step past them, so it is **inferred**, and nothing I read says what a render does instead when one of these is unavailable.

> "Limitations
> Shadow caustics are not supported with HIP."

> "Limitations
> Path Guiding is not supported on any GPU.
> Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30

> "OSL is not supported with GPU rendering unless using the OptiX backend.
> Some OSL features are not available when using the OptiX backend. Examples include:
> Memory usage reductions offered by features like on-demand texture loading and
> mip-mapping are not available."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/osl/index.html
- Date read: 2026-07-30

> "Custom cameras are not supported with GPU rendering unless using the OptiX backend."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/osl/camera.html
- Date read: 2026-07-30

> "The Script node is only available when Open Shading Language is enabled in the Cycles render settings. This feature requires rendering with the CPU or OptiX backend."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/shader_nodes/script.html
- Date read: 2026-07-30

Two shader nodes carry the same line, which is a statement about how the node behaves under one technology:

> "Open Shading Language is active while using the OptiX rendering backend."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/shader_nodes/input/bevel.html and https://docs.blender.org/manual/en/4.5/render/shader_nodes/input/ao.html
- Date read: 2026-07-30

The release notes state the same restriction for the new custom cameras:

> "it's now possible to implement custom cameras using Open Shading Language. This can be enabled independently of OSL shading, but requires hardware support and is therefore currently limited to CPU and OptiX devices."

- Label: **documented**
- Source: release notes — https://developer.blender.org/docs/release_notes/4.5/cycles/
- Date read: 2026-07-30

**3. Which denoising is available.** The technology chosen decides which denoisers can run, and one denoiser is named after one of the technologies:

> "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards." (OptiX section)
> "GPU accelerated denoising is available on discrete Radeon RX 6000 and Radeon RX 7000 GPUs." (HIP section)
> "GPU accelerated denoising is available on all supported GPUs." (oneAPI section)
> "GPU accelerated denoising is available on Apple Silicon." (Metal section)

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30

> "OptiX:
> Uses NVIDIA's OptiX AI denoiser.
> Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.
> Only available on NVIDIA GPUs when configured in the Cycles Render Device user preferences."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html
- Date read: 2026-07-30

**4. Speed and memory.** The manual states plainly that speed differs, and equally plainly that it will not say by how much:

> "OptiX takes advantage of hardware ray-tracing acceleration in RTX graphics cards, for improved performance."

> "What renders faster?
> This varies depending on the hardware used. Different technologies also have different compute times depending on the scene tested. For the most up to date information on the performance of different devices, browse the Blender Open Data resource."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30

And for behaviour when memory runs out, the list of technologies is **not** complete — oneAPI is missing from it:

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30
- That oneAPI therefore does *not* fall back to system memory is **inferred**, and the step is: the sentence lists four technologies by name where five exist, and the omitted one is oneAPI. The sentence does not say what oneAPI does instead, and I found no page that does.

**Does the choice change the finished image?** The sources say it changes which features are available (point 2 above), and four release-note lines name faults in particular methods that were corrected in this version:

> "Cycles MetalRT: Some types of motion are incorrect at low step counts"
> "MetalRT Deleting the last object in a scene leaves it visible in the viewport"
> "Cycles HIP: MNEE renders with artifacts on RDNA2"
> "OneAPI Rendering artifacts in barbershop scene"

- Label: **documented** (these are listed as fixes made in this version)
- Source: release notes — https://developer.blender.org/docs/release_notes/4.5/bugfixes/
- Date read: 2026-07-30
- These are records of past faults that were fixed. They are not a statement that the methods generally disagree, and none of them compares one method's result with another's.

Beyond that, **not in the sources**: I found no statement anywhere that says whether two technologies, given the same scene and the same number of samples, produce the same pixels. Search record for this specific point is in the "Where I looked" section below.

---

### (d) Does anything establish whether that changes with the size of the scene?

**Yes — five statements bear on this, and all of them are about memory or about content rather than about a measured size.** None of them gives a threshold, and none is a rule you could compute with.

**1. The whole reason the choice is not free, stated up front:**

> "This can speed up rendering because modern GPUs are designed to do quite a lot of number crunching. On the other hand, they also have some limitations in rendering complex scenes, due to more limited memory, and issues with interactivity when using the same graphics card for display and rendering."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30

**2. Speed differences between technologies are stated to depend on the scene:**

> "Different technologies also have different compute times depending on the scene tested."

- Label: **documented**
- Same page, same date.

**3. One of the hardware ray-tracing choices has its scene dependence written into its own description** — and it names a kind of content, not a size:

> "MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases"

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/editors/preferences/system.html and, word for word, Cycles' own files, `cycles_4.5/properties.py`, lines 1602–1603
- Date read: 2026-07-30
- This is the clearest "yes" available. It says the memory a render takes depends on which intersection method is used **and** on what the scene contains. It does not say by how much, and "extensively" is not defined anywhere I found.

**4. The setting that exists purely to make large scenes fit, and which is only offered on some hardware:**

> "Distribute Memory Across Devices
> Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes. Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol.
> Currently only NVLink on NVIDIA GPUs is supported."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/editors/preferences/system.html
- Date read: 2026-07-30

The same setting in Cycles' own files, with its wording and its starting value:

> `peer_memory: BoolProperty(`
> `    name="Distribute memory across devices",`
> `    description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it",`
> `    default=False,`
> `)`

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/properties.py`, lines 1594–1598
- Date read: 2026-07-30
- I quoted nothing showing when this switch is offered and when it is not. Hand test 2 below is aimed at that.

And the manual's plain answer on memory:

> "Would multiple GPUs increase available memory?
> Typically, no, each GPU can only access its own memory.
> The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30

**5. On Metal there is a further setting whose whole description is about scene content:**

> `kernel_optimization_level: EnumProperty(`
> `    name="Kernel Optimization",`
> `    description="Kernels can be optimized based on scene content. Optimized kernels are requested at the start of a render. "`
> `                "If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set "`
> `                "is available in the cache. This can result in additional CPU usage for a brief time (tens of seconds)",`
> `    default='FULL',`
> `    items=(`
> `        ('OFF', "Off", "Disable kernel optimization. Slowest rendering, no extra background CPU usage"),`
> `        ('INTERSECT', "Intersection only", "Optimize only intersection kernels. Faster rendering, negligible extra background CPU usage"),`
> `        ('FULL', "Full", "Optimize all kernels. Fastest rendering, may result in extra background CPU usage"),`
> `    ),`
> `)`

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/properties.py`, lines 1624–1635
- Date read: 2026-07-30
- I did not find this setting described on the Preferences page of the manual. It appears only in Cycles' own files. In the drawing code quoted in (b) it sits inside the block guarded by Metal and by `arm64`; reading that as "it is offered only when the technology is Metal on Apple Silicon" is the same step flagged there, and is **inferred**.

**What is not established.** No source I read gives a size at which any of this turns over — no vertex count, no texture budget, no memory figure — for any of the five technologies or three ray-tracing switches. The only quantities the whole subject offers are a note about texture memory that is not tied to any technology:

> "One way to reduce memory usage is by using smaller resolution textures. For example, 8k, 4k, 2k, and 1k image textures take up respectively 256MB, 64MB, 16MB and 4MB of memory."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30

So the honest summary of (d): **the sources establish that the choice's effect does change with the scene — in memory taken and in time — but they establish it only as a statement of fact, never as a number or a threshold.** For the size question in its numeric form, the answer is **not in the sources**.

---

### The other readings of the question

**Reading 3 — Display Graphics Backend.** This is a real setting, in the same Preferences section, and it is the only place in Blender 4.5 that I found using the words "graphics API":

> "Backend
> Selects the graphics API used for drawing the interface and rendering display content.
> Changing the backend requires restarting Blender for the change to take effect.
> OpenGL:
> Uses the OpenGL backend. This is the traditional backend, compatible with a wide range of systems.
> Vulkan:
> Uses the Vulkan backend. Vulkan may offer improved performance and better support for modern GPU features, but compatibility may vary depending on the system and drivers."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/editors/preferences/system.html
- Date read: 2026-07-30

From outside Blender:

> "--gpu-backend
> Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30
- Note the mismatch, which I report without explaining: the command line accepts three values including `metal`, while the Preferences page describes only two, OpenGL and Vulkan.

And under this reading, the scene-size question has a sharper documented answer than under the main reading:

> "OpenGL is still the default in Blender 4.5 LTS, users are encouraged to try Vulkan by enabling it in Preferences"
> "- Vulkan backend cannot handle huge meshes (100.000.000 vertices). Vulkan has different limits compared to OpenGL. These limits are controlled by the driver and vendor specific. Changing drivers could increase these limits."
> "- All textures should fit inside the GPU memory. Vulkan has different limits compared to OpenGL. These limits are controlled by the driver and vendor specific. Changing drivers could increase these limits."
> "- Virtual reality using Vulkan has performance issues. For virtual reality we will recommend to use OpenGL."

- Label: **documented**
- Source: release notes — https://developer.blender.org/docs/release_notes/4.5/eevee/
- Date read: 2026-07-30
- This is the only place in everything I read that puts a **number** on a scene-size limit tied to a choice of graphics method. It bears on the drawing method rather than on how a render is computed, which is what reading 3 is.
- The first of these four quotes is one sentence, wrapped across two lines in the source; it finishes "enabling it in Preferences" and links to the Preferences page. It sits under a heading of its own and does not introduce the three bullets, which sit under a later "Limitations" heading. The number claim above rests on the second quote alone, which stands on its own. *(Citation check, 2026-08-30: the quote was cut off at the line break and carried a note saying the rest of the sentence was unknown; the sentence has been completed from `release_notes_4.5/eevee.md` and the note corrected. Changed by this check.)*

**Two things that sound like this question and are not.**

*EEVEE's Raytracing Method* is a scene setting with two choices, but they are ways of tracing, not ways of using a card:

> "Method
> Determine the tracing method used to find scene-ray intersections and indirect lighting.
> Light Probe:
> Use light-probe spheres and planes to find scene intersection. This option has the lowest tracing cost but relies on manually placed light-probes.
> Screen-Trace:
> Trace ray against the screen depth buffer. Fallback to light-probes if ray exits the view."

- Label: **documented**
- Source: manual — https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/raytracing.html
- Date read: 2026-07-30

*Cycles' "BVH" debug setting* offers BVH2 or Embree and is stored in the scene, but Cycles' own drawing code places it under a heading marked CPU:

> `        col = layout.column(heading="CPU")`
> `        row = col.row(align=True)`
> `        row.prop(cscene, "debug_use_cpu_sse42", toggle=True)`
> `        row.prop(cscene, "debug_use_cpu_avx2", toggle=True)`
> `        col.prop(cscene, "debug_bvh_layout", text="BVH")`

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/ui.py`, lines 2232–2237, inside `class CYCLES_RENDER_PT_debug` with `bl_label = "Debug"` (line 2218–2219); the setting itself is at `cycles_4.5/properties.py` lines 1072–1076 with the list at lines 47–50
- Date read: 2026-07-30
- Reading this as "a processor setting, not a graphics card setting" is one step past the words, so that reading is **inferred**; the step is the heading `"CPU"` above it and the two processor settings beside it.

---


---

## Tested by hand — what a running Blender showed

**Label: observed.** Run on 2026-08-30 by Claude Opus 5 in Claude Code, on the two computers described below, in **Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.1**. The same tests were run in all four versions, so that a gap in one is not read as a difference in Blender. Blender 4.5's own figures are given here in full; where the four versions agreed, that is said.

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

Blender 4.5 was started with no window and `--factory-startup` on each machine, and asked what the Cycles preference offers.

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

On both machines, in Blender 4.5, the Cycles preference object carries exactly eight properties:

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

Four crossings were run in Blender 4.5 on both machines, each with a scratch configuration folder holding a saved preference and a `.blend` saved with the scene's device one way or the other. Which method actually ran was read from two things Blender prints: `Building OptiX acceleration structure`, which only an OptiX render prints, and `Profiling information not available (only works with CPU rendering)`, which only a card render prints.

On the rented Linux machine: with the stored technology at `NONE` and the scene's device at `CPU`, adding `-- --cycles-device OPTIX` produced the OptiX lines, the card-only line, and a render in under a second against about six seconds for the processor. With the stored technology at `OPTIX` and the scene's device at `GPU`, adding `-- --cycles-device CPU` produced neither line and about six seconds. The Mac gave the same shape with `METAL` in place of `OPTIX`.

**So the command-line option beats both the stored preference and the scene's own setting, in both directions, on both operating systems.** All four versions behaved the same way.

### 4. What happens when the named method is one the machine cannot do, or is not a method at all

In Blender 4.5, on the rented Linux machine (no Apple, no AMD, no Intel graphics), `-- --cycles-device METAL`, `HIP` and `ONEAPI` were each asked for; on the Mac (no NVIDIA, no AMD, no Intel graphics), `CUDA`, `OPTIX` and `HIP`. On both, `-- --cycles-device BANANA` was also asked for.

**Every one of them ended with exit status 1 and wrote nothing to disk.** For a real method the machine has no hardware for, Blender 4.5 printed:

```text
Error: Found no Cycles device of the specified type
```

For the nonsense name it printed:

```text
Error: BANANA is not a valid Cycles device.
```

*(4.5.12 prints these as `Error:` lines; 5.0.1, 5.1.2 and 5.2.1 print them as `ERROR` lines through their own message system. The words after that are identical.)* Nothing fell back to the processor and nothing fell back to the card the machine did have. On the Linux machine, asking for `HIP` also printed `WARNING HIPEW initialization failed: Error opening HIP dynamic library` in 5.0.1, 5.1.2 and 5.2.1 before the error.

### 5. The choice of method changes the picture, and a control shows that is not run-to-run variation

Each scene was rendered with each method and the **pixels** compared, not the files — Blender writes clocks into a saved image, so a byte comparison reads a clock. The images are 32-bit OpenEXR at 640 by 360, which is 230,400 pixels.

**The control first, because without it this finding carries nothing.** The same scene was rendered **twice with the same method** and the two compared. On the processor, in Blender 4.5 on both machines, the two runs are **identical to the pixel** — not one channel differs. On a card they are not identical, but the largest difference between two runs of the same method is 4.17 × 10⁻⁷ to 9.54 × 10⁻⁷ on the rented Linux machine and 2.98 × 10⁻⁷ to 5.96 × 10⁻⁷, except the curve scene where it reached 0.0182 on the Mac.

Against that floor, in Blender 4.5:

| pair | small scene | heavy mesh | heavy curves |
|---|---|---|---|
| processor against CUDA (Linux) | 0.0121 | 0.0185 | 0.1328 |
| processor against OptiX (Linux) | 0.0102 | 0.0185 | 0.1235 |
| CUDA against OptiX (Linux) | 0.0121 | 0.0133 | 0.0407 |
| processor against Metal (Mac) | 0.0103 | 0.0185 | 0.1134 |
| MetalRT off against on (Mac) | 0.0103 | 0.0205 | 0.1425 |
| MetalRT off against auto (Mac) | 0.0020 | 4.77 × 10⁻⁷ | 0.0182 |

Figures are the largest difference in any colour channel; between 224,257 and 230,015 of the 230,400 pixels differ on the mesh scenes and about 149,000 to 162,500 on the curve scene. **Every pair of different methods in the first five rows differs by three to five orders of magnitude more than two runs of the same method do on the processor, where two runs are identical to the pixel.** On a card the floor is not always that low: the curve-scene figure in the line above is 0.0182, which is only about two and a half times below the 0.0407 in the CUDA-against-OptiX row, so on that one scene the separation is a factor rather than orders of magnitude.

*(Citation check, 2026-08-30, repeated on the Creator's Mac in 4.5.12: the processor half reproduced exactly — two CPU renders identical to the pixel on all three scenes. The Mac card control came back at 5.96 × 10⁻⁷, 4.77 × 10⁻⁷ and 2.98 × 10⁻⁷ on small, heavy mesh and heavy curves — **the curve-scene excursion to 0.0182 recorded above did not appear**, and the off-against-auto row came back at 3.58 × 10⁻⁷, 5.96 × 10⁻⁷ and 2.98 × 10⁻⁷ against the 0.0020, 4.77 × 10⁻⁷ and 0.0182 recorded. Both runs stand; neither has been chosen between, and the original figures above are untouched. The disagreement is on the hand-test list to be run again. The summary sentence was narrowed by this check because it did not fit the 0.0182 figure the same paragraph reports.)*


*(Run again on 2026-08-30, and the control was measured properly this time: see **The noise floor measured, and three rows run again** below. Two runs of the same method were compared 120 times over. **On the processor all sixty pairs are identical to the pixel. On Metal the floor is 10⁻⁷ most of the time and not all of the time** — two card pairs in sixty came back at 5.31 × 10⁻⁵ and 0.0182. The rows above stay comfortably above the floor on the small and heavy mesh scenes and are only about six to eight times the worst case on the curve scene. Nothing above was changed.)*

### 6. What the choice changes depends on the size of the scene, and it changes direction

On the rented Linux machine, Blender 4.5, three repeats of every measurement with warm kernel caches. The three repeats agreed to within a few hundredths of a second throughout.

| scene | processor | CUDA | OptiX | card memory: CUDA / OptiX |
|---|---|---|---|---|
| small, 322 triangles | 00:06.81 / 00:06.78 / 00:06.83 | 00:00.59 / 00:00.59 / 00:00.61 | 00:00.65 / 00:00.63 / 00:00.62 | 1545 / 1559 MiB |
| heavy mesh, 10,485,762 triangles | 00:11.85 / 00:11.76 / 00:11.75 | 00:06.42 / 00:06.41 / 00:06.32 | 00:01.53 / 00:01.53 / 00:01.54 | 2217 / 2149 MiB |
| heavy curves, 300,000 curves | 00:15.46 / 00:15.42 / 00:15.42 | 00:04.44 / 00:04.40 / 00:04.40 | 00:01.23 / 00:01.24 / 00:01.21 | 1891 / 1665 MiB |

**On the 322-triangle scene CUDA is marginally ahead of OptiX. On the 10.5-million-triangle scene OptiX is about four times faster.** Same card, same driver, same file, same settings, same version. The card's own memory moves the same way: on the small scene OptiX takes slightly more than CUDA, on the heavy mesh scene rather less, and on the curve scene least of all. All four versions showed the same shape.

**A negative result was available and did not happen.** Had the ratio of one method's time to the other's been the same at both sizes, within the spread of the three repeats, the answer would have been that the effect does not depend on the size of the scene.

### 7. MetalRT, and what "Auto" does

The Creator's Mac only. MetalRT is a preference rather than a scene setting, so it was set from a script at start-up before the frame was rendered, and the script printed what it asked for and what the value became, so a run where the setting did not apply cannot be mistaken for one where it made no difference.

**What Blender prints while it builds separates the three settings cleanly, and identically in all four versions.** With MetalRT `ON` it prints `Building TLAS` and `Building mesh BLAS`, and on the curve scene `Building hair BLAS` as well. With `OFF` and with `AUTO` it prints `Packing BVH nodes` and `Packing BVH triangles and strands`. **On all three scenes, in all four versions, Auto built the same structure as Off.**

Cycles' own reported peak memory in Blender 4.5, which the shared Mac does not affect:

| scene | MetalRT Off | MetalRT On | MetalRT Auto |
|---|---|---|---|
| small | 1363.68 M | 1363.70 M | 1363.68 M |
| heavy mesh | 2008.24 M | 2463.95 M | 2008.24 M |
| heavy curves | 1693.82 M | 1658.95 M | 1693.82 M |

**At 322 triangles the switch changes nothing measurable. On the mesh scene it costs about 455 MB more. On the curve scene it saves about 34 MB.** That is the same direction for curves in all four versions and the opposite direction on a mesh scene of comparable weight, and it appears only once the scene is large enough for it to.

Times in Blender 4.5, fastest of three repeats on a shared Mac: on the heavy mesh scene, Off 00:05.46, On 00:03.53, Auto 00:05.25; on the curve scene, Off 00:05.67, On 00:05.95, Auto 00:05.60; on the small scene, Off 00:02.00, On 00:01.90, Auto 00:01.53.

### 8. Kernel Optimization

The Creator's Mac only, the heavy mesh scene, Blender 4.5, set from a script and confirmed to have taken. Off 00:06.43, Intersection only 00:05.66, Full 00:05.18. Off was the slowest in all four versions. On the pictures: FULL against OFF reached 0.0125, above the noise; INTERSECT against OFF stayed at 5.96 × 10⁻⁷.


*(Run again on 2026-08-30, three renders of each setting and every pair of the nine compared: see **The noise floor measured, and three rows run again** below. Nothing above was changed.)*

### 9. A scene the machine cannot hold, and where the choice decides whether the render happens at all

The huge mesh scene — **125,829,122 triangles**, a 9.3 GB file — on the rented Linux machine, in Blender 4.5:

```text
OptiX: exit status 0, 28.88 seconds of wall time, card memory peaked at 8311 MiB of 10,240, host memory peaked at 27,037,351,936 bytes, render time 00:20.10, frame written
CUDA:  exit status 137, 162.21 seconds of wall time, card memory peaked at 1613 MiB of 10,240, host memory peaked at 32,917,737,472 bytes, render time 00:57.03, no frame written
```

**The OptiX run finished and wrote its frame. The CUDA run was killed by the system for want of host memory** — status 137, the last line printed being `Updating Scene BVH | Building BVH` at somewhere between 45% and 76%, with the container's memory at its ceiling of about 32.9 GB. The same happened in all four versions: every OptiX run completed, every CUDA run was killed.

**What this does not say.** The condition reached was the host's memory ceiling, not the card's. OptiX peaked at between 7,765 and 8,353 MiB of the card's 10,240 MiB and CUDA never got past 1,775 MiB before it died, so **nothing here bears on what happens when a card's own memory fills**, and the documented fallback to system memory was not exercised.

### 10. The Acceleration Structure panel

Blender 4.5 was started with no window on both machines and the panel itself was asked, for every crossing of the technology, the scene's device setting, and whether the processor is ticked in the device list, whether it would be drawn. The panel class `CYCLES_RENDER_PT_performance_acceleration_structure` exists in Blender 4.5 on both machines.

On the rented Linux machine the answer was `True` under every crossing except one:

```text
pref=OPTIX  scene_device=GPU cpu_device_ticked=False -> Acceleration Structure panel poll = False
pref=OPTIX  scene_device=GPU cpu_device_ticked=True  -> Acceleration Structure panel poll = True
```

So the panel goes away only when OptiX is chosen **and** the scene is set to the card **and** the processor is not ticked alongside it. Under `NONE`, `CUDA`, `HIP` and `ONEAPI` it is always present. On the Mac, under `NONE` and `METAL`, it is always present.

**The four versions answered identically.** That matters here: three of the four versions' documents quote a manual line about this panel and one does not, and this test shows that silence to be a gap in the documentation rather than a difference in Blender. One version difference showed up beside it and is not about this panel: 5.2.1 carries a `CYCLES_RENDER_PT_performance_texture_cache` panel that this version does not.

### 11. What travels with the file, and what a stored preference alone does

In Blender 4.5, on both machines: a `.blend` saved with the scene's own device set to `GPU`, opened with a configuration whose technology was set one way and then another, read back:

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

Blender 4.5, both machines, `--gpu-backend` given each of its three documented values on a background render.

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

`blender --help` in Blender 4.5, on both machines, lists under **Cycles Render Options**:

```text
--cycles-print-stats
	Log statistics about render memory and time usage.
```

Every run above that used it printed a `Render statistics:` block with per-object geometry memory, and on a card render the line `Profiling information not available (only works with CPU rendering)`.

### The noise floor measured, and three rows run again — 2026-08-30

**Label: observed.** Run on 2026-08-30 by Claude Opus 5 in Claude Code, **on the Creator's Mac and on no other computer**, in Blender 4.5.12 alongside the other three researched versions. These are entries 26 to 29 of `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md`, sent there by that topic's citation check on the same day because its own repeats of the tests above came back different in four places.

**What this can and cannot settle.** The Creator's Mac has **Apple's own graphics and no NVIDIA card**, and the rented Austrian machine behind the CUDA and OptiX figures above was destroyed on the day those tests were run. **So everything in this section settles macOS and Apple's own graphics, and says nothing whatever about an NVIDIA card.** Nothing was rented for it. Other agents had been running Blender on the same Mac all day, so **no time in this section is a clean one**; all four tests are about pixels and exit statuses rather than speed. Every one of about 900 Blender launches ran with a scratch configuration folder, so nothing of the Creator's own settings was read or written.

**Why it was worth running at all.** Finding 5 above rests entirely on a control: two runs of the *same* method should come back far closer together than two runs of *different* methods. Nobody had measured how wide that same-method spread really is — one pair had been run, not a spread — and **an answer whose noise floor is unknown cannot be weighed.**

#### The floor itself, over sixty pairs on the card and sixty on the processor

**What was done.** Each of the small, heavy mesh and heavy curve scenes was rendered **twice with the same method** — once on the processor, once on Metal — and the two compared by their pixels, in all four versions. **The whole set was repeated five times over**: 240 renders and 120 pairs, of which 15 card pairs and 15 processor pairs are Blender 4.5's.

**What a negative result would have looked like:** every Metal pair at 10⁻⁷ every time, which would have confirmed the floor exactly as recorded above.

**The processor. All sixty pairs across the four versions are identical to the pixel** — not one channel of 921,600 differs — and Blender 4.5's fifteen are among them; every one printed `VERDICT: IDENTICAL to the pixel`.

**The card, in Blender 4.5, the five repeats in order:**

| scene | two runs of Metal, repeats 1 to 5 |
|---|---|
| small | 4.77 × 10⁻⁷, 4.77 × 10⁻⁷, 3.58 × 10⁻⁷, 3.58 × 10⁻⁷, 4.77 × 10⁻⁷ |
| heavy mesh | 4.77 × 10⁻⁷ on all five |
| heavy curves | **0.018208861351013184**, 3.58 × 10⁻⁷, 2.98 × 10⁻⁷, 3.58 × 10⁻⁷, 2.98 × 10⁻⁷ |

**This version's own excursion is the one the record above already carries** — 0.0182 on the curve scene — and it came back at the same figure to nine decimal places, on the first of the five repeats. The other four repeats of that same scene sat at 2.98 × 10⁻⁷ to 3.58 × 10⁻⁷.

**Across all four versions, fifty-eight of the sixty card pairs sit between 2.98 × 10⁻⁷ and 5.96 × 10⁻⁷.** The two that do not are 5.310773849487305 × 10⁻⁵, in Blender 5.0.1 on the heavy mesh scene, and 0.018208861351013184, in Blender 4.5.12 on the curve scene. **Both were the first repeat of the set.** So **the floor on this machine is not one number**: it is 10⁻⁷ most of the time, and about one card pair in thirty came back four to five orders of magnitude higher.

#### What that makes of the differences recorded above

Only the Mac rows of finding 5 are touched here. The Linux rows are outside what this machine can say anything about.

| row, as finding 5 records it for Blender 4.5 | small | heavy mesh | heavy curves |
|---|---|---|---|
| worst two-runs-of-Metal figure measured here, this version | 4.77 × 10⁻⁷ | 4.77 × 10⁻⁷ | **0.0182** |
| worst two-runs-of-Metal figure measured here, all four versions | 4.77 × 10⁻⁷ | 5.31 × 10⁻⁵ | **0.0182** |
| processor against Metal | 0.0103 | 0.0185 | 0.1134 |
| MetalRT off against on | 0.0103 | 0.0205 | 0.1425 |

**On the small scene and the heavy mesh scene those recorded differences sit comfortably above the floor** — from about 350 times it up to tens of thousands of times it. **On the curve scene they do not sit comfortably above it.** The worst same-method figure this machine produced on that scene, in any version, is 0.0182, and the recorded curve-scene differences of 0.1134 and 0.1425 are only about six to eight times that. They are still above it, and nineteen of the twenty curve-scene pairs were at 10⁻⁷ — but *orders of magnitude* is the wrong description of the curve-scene rows, and *a factor of six to eight against the worst case* is the right one.

**Both runs stand.** Finding 5's own control figures above are untouched, this run's are here, and neither has been chosen over the other.

#### MetalRT Auto against MetalRT Off, three hundred comparisons

**What was done.** Each of the three scenes was rendered **five times with MetalRT off and five times with MetalRT auto**, in all four versions, and **every off was compared against every auto** — 25 comparisons per scene per version, 300 in all, of which 75 are Blender 4.5's.

**What a negative result would have looked like:** every pair at the noise floor every time. That is what came back, and it could have gone the other way — the same procedure produced figures of 0.012 and 0.018 twice before, once in the record above and once in the citation check's repeat.

**Every one of the 120 renders exited 0 and printed the setting it had been given**, sixty reading off and sixty reading auto. **And what Blender printed while it built was the same line in all 120**, in every version and on every scene:

```text
Packing BVH nodes; Packing BVH triangles and strands
```

So **auto built the same structure as off in all 120 runs**, which is what finding 7 above already says.

**The pixels, in Blender 4.5, as the range over the twenty-five comparisons in each cell:**

| scene | MetalRT off against auto, twenty-five comparisons |
|---|---|
| small | 3.58 × 10⁻⁷ to 4.77 × 10⁻⁷ |
| heavy mesh | 4.77 × 10⁻⁷ to 5.96 × 10⁻⁷ |
| heavy curves | 2.98 × 10⁻⁷ to 3.58 × 10⁻⁷ |

Across all four versions **not one of the 300 comparisons is above 7.15 × 10⁻⁷**. **This version's recorded figures for that row are 0.0020 on the small scene and 0.0182 on the curve scene. Neither came back in twenty-five tries on each.**

**Both runs stand.** Finding 5's off-against-auto row for Blender 4.5 — 0.0020, 4.77 × 10⁻⁷ and 0.0182 — is untouched, and so is the citation check's note beside it.

**One plain thing about the numbers, offered as arithmetic and not as an answer.** The figures recorded for that row which are above the floor are the same size as the two same-method excursions measured above, and two of them fall on the same scene in the same version as an excursion. **Nothing run here establishes what that means**, and this section does not say which reading is the right one.

#### Kernel Optimization, thirty-six renders and a hundred and forty-four comparisons

**What was done.** The heavy mesh scene rendered with Kernel Optimization off, intersection only, and full — **three times each**, in all four versions — and **every pair of the nine images in a version compared**, which is 36 pairs a version. The nine include three same-setting pairs, so the floor for this scene is measured inside the same test.

**What a negative result would have looked like:** all pairs at the noise floor, which is what Blender 5.1's own record says and what the other three versions' records deny.

**All 36 renders exited 0 and every one printed the setting it had been given. In Blender 4.5:**

| pair | range over the nine, or the three |
|---|---|
| off against full | 0.0124985 to 0.0124986 |
| off against intersection only | 4.77 × 10⁻⁷ to 7.15 × 10⁻⁷ |
| full against intersection only | 0.0124985 to 0.0124986 |
| two runs of the same setting | 4.77 × 10⁻⁷ to 7.15 × 10⁻⁷ |

**That agrees with this version's own record**, which has Full against Off at 0.0125 and Intersection only against Off at the floor.

**Across the four versions the split is clean and it is the same split the records above have**: 4.5.12, 5.0.1 and 5.2.1 gave 0.0125 for off against full in all nine pairs each, and 5.1.2 gave the noise floor in all nine.

**Times, on a machine that was not quiet**, as Cycles' own `Time:` line for the three runs of each setting in Blender 4.5: off 00:05.42, 00:05.33, 00:05.34; intersection only 00:05.34, 00:05.30, 00:05.39; full 00:05.20, 00:05.33, 00:05.21. **Off was the slowest in all four versions**, which is the shape both earlier runs recorded and the one part of this test nobody disputes.

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

**The technology can be set from outside.** `--cycles-device` is documented with a background example (`blender -b file.blend -f 20 -- --cycles-device OPTIX`) — **documented**, the command-line page cited in (a). Cycles' own files carry help text saying it overrides "user preferences and the scene setting" — **observed**, `cycles_4.5/engine.py` lines 16–20. That it is therefore the strongest of the three places is **inferred**, and the step is that the help text describes what the option is for rather than recording a render.

Cycles reads that option from the words after `--` on the command line:

> `def _parse_command_line():`
> `    import sys`
>
> `    argv = sys.argv`
> `    if "--" not in argv:`
> `        return`

and, if the value is refused, stops Blender outright:

> `    if args.cycles_device:`
> `        import _cycles`
> `        if not _cycles.set_device_override(args.cycles_device):`
> `            sys.exit(1)`

- Label: **observed**
- Source: Cycles' own files — `cycles_4.5/engine.py`, lines 24–41
- Date read: 2026-07-30
- What "sys.exit(1)" means for a run in progress is a step past the words; that a bad value ends the run rather than falling back is **inferred**, and the step is that the exit call is reached when the override is refused.

**Cycles offers no command-line option for the hardware ray-tracing switches**, since it defines only two options and neither is for them (observed, `cycles_4.5/engine.py` lines 13–20). That they therefore cannot be set from outside at all is broader than that file, and rests on the manual's command-line page and the search recorded below finding no such option; on that ground it is **not in the sources** rather than established. They live on the Cycles preferences object (observed, `cycles_4.5/properties.py` lines 1555–1635), which is stored in `userpref.blend` in the user's configuration directory (documented). Whether a Blender started from outside its window applies those stored preferences is **inferred** from one sentence — the `--cycles-device` help says it overrides "user preferences", which only makes sense if user preferences would otherwise apply. That is a real step and I flag it as such. Nothing I read states it outright.

**Where it differs from inside:** inside, the technology is chosen in Preferences ‣ System ‣ Cycles Render Devices, and the manual says preferences are saved on exit by default — both documented above. Nothing I read says what a change there does to a render already under way, and the manual's picture of that page shows the panel with nothing inside it, so I cannot say from the sources what the choice looks like as a control. Outside, `--cycles-device` is the only route, and Cycles' help text describes it as overriding the stored preference and the scene. Beyond that, **the sources do not say** whether the settings differ in effect between the two. Hand tests 1, 5 and 6 below are aimed at this.

### Operating system

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Tested rather than inferred: the Creator's Mac offers only `None` and `Metal` and starts at `Metal`; a rented Linux machine with one NVIDIA card offers `None`, `CUDA`, `OptiX`, `HIP` and `oneAPI` and starts at `None`. So the list of methods does differ by operating system — **but it is built from what that build supports rather than from the cards present**, since the Linux machine offered HIP and oneAPI with no AMD and no Intel graphics in it. `--gpu-backend` accepts `opengl` and `vulkan` on Linux and only `metal` on macOS, naming the rejected value in an error and then finishing the render anyway. **Windows is not settled**: this project's route to a Windows computer is a rental with two general-purpose cores and no graphics card at all, so four card-free Windows tests are written out ready to run and wait on the next Windows batch.]*

**No, this does not hold on every operating system, and the sources are explicit about it.** The manual's opening line for this whole subject is that the technologies depend on the operating system: "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer and operating system."

- CUDA, OptiX, HIP, oneAPI: "supported on Windows and Linux" — each stated separately, all four with those same two words. macOS is not named for any of them. **Documented**, gpu_rendering page, quoted in full in (b).
- Metal: "supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." **Documented**, same page.
- MetalRT and Kernel Optimization: Cycles' own comment reads `# MetalRT only works on Apple Silicon.` (`cycles_4.5/properties.py`, line 1900), and the line following it is `if (platform.machine() == 'arm64'):`. **Observed** — that those words sit there. That the two settings are therefore shown only on Apple Silicon is **inferred**, the same step flagged in (b).
- The Debug panel splits by operating system too. The one line I quote is `is_macos = platform.system() == 'Darwin'` (`cycles_4.5/ui.py`, lines 2239–2255). **Observed.** That Metal settings are drawn on macOS and CUDA, OptiX and HIP settings otherwise is my summary of the surrounding lines and I did not quote them, so it is not carried by anything on this page.
- Distribute Memory Across Devices: the manual ties it to NVLink hardware and names no operating system for it. **Documented**, the Preferences page quoted in (d).
- HIP minimum drivers differ by operating system: "Windows: Radeon Software 24.6.1 or Radeon PRO Software 24.Q2 / Linux: Radeon Software 23.40 or ROCm 6.0". oneAPI likewise: "Windows: Intel Graphics Driver XX.X.101.5518 / Linux: intel-level-zero-gpu package 1.3.27642…". **Documented**, gpu_rendering page.
- One Linux distribution is called out by name in the release notes: "GPU hardware ray-tracing package from the Ubuntu repository does not work on Ubuntu 25.04 due to incompatible TBB versions… Be sure to install the package `libze-intel-gpu-raytracing`." **Documented**, https://developer.blender.org/docs/release_notes/4.5/cycles/
- For the display backend reading: "This option is not available on macOS and requires using OpenGL backend" is said of Shader Compilation Method on the Preferences page. **Documented.**

### Not covered

- No source gives any number for what any technology costs or saves — no render times, no memory figures, no thresholds. The manual points elsewhere for that ("browse the Blender Open Data resource"), which I did not and may not follow.
- Nothing states whether two technologies give identical pixels for the same scene and settings.
- Nothing states what oneAPI does when graphics-card memory runs out; it is simply left out of the list of four that fall back to system memory.
- "Scenes which use curves extensively" (MetalRT) is never defined.
- Whether the hardware ray-tracing switches can be changed at all from a run started outside Blender's window — by any route other than a command-line option — is not addressed by anything I read.
- The Kernel Optimization setting does not appear in the manual at all, only in Cycles' own files; I cannot say how it is presented or documented to users.
- The Python API documentation does not describe `compute_device_type`, or any of the other five settings, as a property. That is **not in the sources**, established by the search recorded below: the whole API tree was searched for all six names and the only hits were a different legacy setting and one change-log line.
- CUDA and OptiX have no hardware ray-tracing switch of their own in Cycles' preferences; the sources do not say what governs hardware ray tracing for them beyond OptiX's own statement that it "takes advantage of hardware ray-tracing acceleration in RTX graphics cards".

---

### Where I looked

Everything below was searched across the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` unless stated.

**Terms searched, case-insensitively:** `OptiX`, `CUDA`, `HIP`, `oneAPI`, `Metal`, `Vulkan`, `OpenGL`, `Compute Device`, `GPU Backend`, `MetalRT`, `HIP RT`, `hiprt`, `Embree`, `use_oneapirt`, `compute_device_type`, `peer_memory`, `kernel_optimization`, `hardware ray`, `BVH`, `bvh_layout`, `intersection method`, `rendering method`, `render method`, `rendering backend`, `graphics API`, `Rendering Technologies`, `Cycles Render Devices`, `Cycles Device`, `cycles-device`, `gpu-backend`, `factory-startup`, `userpref.blend`, `Auto-Save Preferences`, `large scenes`, `complex scenes`, `scene size`, `heavy scene`, `depending on the scene`, `scene content`, `scene complexity`, `number of vertices`, `huge meshes`.

**Pages opened in full or in part:**
- manual: `render/cycles/gpu_rendering.txt` (whole file, 220 lines), `editors/preferences/system.txt` (whole file), `advanced/command_line/arguments.txt` (the Cycles and GPU option blocks), `render/cycles/render_settings/sampling.txt` (denoiser section), `render/cycles/osl/index.txt` (limitations), `render/cycles/osl/camera.txt` (limitations), `render/shader_nodes/script.txt`, `render/shader_nodes/input/bevel.txt`, `render/shader_nodes/input/ao.txt`, `render/eevee/render_settings/raytracing.txt`, `advanced/blender_directory_layout.txt`, `editors/preferences/introduction.txt`.
- manual image: `_images/editors_preferences_section_system.png`, opened and looked at.
- release notes: `cycles.md` (whole file), `eevee.md` (Vulkan section), `index.md` (compatibility list), `bugfixes.md` (searched).
- Cycles' own files: `properties.py` (device lists, preferences class, debug settings, drawing code), `engine.py` (whole command-line section), `ui.py` (debug panel), `source/kernel/types.h` (intersection layout names), `version_update.py` (searched only).
- API documentation: searched whole tree for the six setting names. The only hits were `bpy.types.PreferencesSystem.txt` for a different, legacy setting, and a change-log line:

> `bpy.types.CyclesPreferences.get_devices_for_type (self, compute_device_type, device_list), was (self, compute_device_type)`

  (`api_4.5/change_log.txt`, line 111 — **observed**, no web address applies to a change-log line beyond https://docs.blender.org/api/4.5/change_log.html). I found no page in the API documentation describing `compute_device_type`, `metalrt`, `use_hiprt`, `use_oneapirt`, `peer_memory` or `kernel_optimization_level` as properties.

**Ruled out and why:**
- `addons/import_export/anim_bvh.txt` and the sculpting "Rebuild BVH" pages — the letters BVH there are a motion-capture file format and a sculpting cache, unrelated.
- `getting_started/about/history.txt` — mentions the technologies only as a record of when they were added in past versions.
- `troubleshooting/gpu/linux/nvidia.txt` — mentions Vulkan in a driver-troubleshooting context, not a render setting.
- Every EEVEE `raytracing`, `light_settings`, `light_probes` page — these are tracing techniques within EEVEE, not a card's methods; kept only the Method quote above as the ruled-out alternate reading.
- `render/cycles/render_settings/simplify.txt` and the "Denoise on GPU" description — both talk about large scenes and memory, but neither is a choice between a card's rendering methods.

**Searches that found nothing:** I found no page anywhere stating that two rendering technologies produce the same or different images for the same scene; no page giving any threshold in vertices, faces, textures or bytes for any Cycles technology; and no command-line option, environment variable or file-stored setting for MetalRT, HIP RT or Embree on GPU. The terms tried for that last point were `metalrt`, `hiprt`, `use_hiprt`, `use_oneapirt`, `embree`, `hardware ray`, `--cycles`, and a read of every argument Cycles defines in `engine.py`.

---

## Things a running Blender could still settle

*(Pruned 2026-08-30 by the hand tests. Everything the earlier list held has either been run — the results are above, under **Tested by hand — what a running Blender showed** — or is recorded there as unable to be run, with its reason. What follows is what is genuinely still open.)*

1. **What happens when the card's own memory fills.** The scene of 125,829,122 triangles reached the host machine's memory ceiling and not the card's, so the documented fallback to system memory was never exercised. Settling it needs a machine whose card is small against what its host can build, which is the opposite of the usual want.
2. **What MetalRT's "Auto" weighs.** On the Creator's Mac it built the same structure as Off on all three scenes, including one of 300,000 curves — in the tests above, and again in sixty more runs on 2026-08-30, in all four versions, without one exception. Whether it ever picks the other way — on other Apple hardware, or on some other kind of scene — is still not settled.
3. **Whether the four card-free Windows tests give the same answers.** They are written out ready to run in `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md` and wait on the next Windows rental.
4. **Everything needing hardware nobody here has**: an AMD card, an Intel Arc card, two cards of different ray-tracing generations in one machine, NVLink-joined cards, a Mac that is not Apple silicon, or an Apple machine an agent may open a window on. Each is named above with its reason.

---

2026-08-29 — Claude Opus 5 (Claude Code) — What a render demands, Blender 4.5.

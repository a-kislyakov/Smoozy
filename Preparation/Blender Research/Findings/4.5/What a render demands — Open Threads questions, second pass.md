# What a render demands — Open Threads questions, second pass (Blender 4.5)

**Sources consulted, and when.** The Blender 4.5 manual, the Blender 4.5 Python API reference, the Blender 4.5 release notes, and the Cycles add-on's own files, all as supplied in `Blender Sources/` and all read on 2026-07-30 (the snapshot date given to me). I also opened one picture from the manual, `_images/editors_preferences_section_system.png`.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Answered here: question 7 only.

---

## 7. Where a render can be told which of a graphics card's rendering methods to use, what methods are there, what does choosing one rather than another change about the render, and does anything establish whether that changes with the size of the scene?

### A note on the reading I took

The question is ambiguous, so I say plainly which reading I answered.

**The reading I took (the main answer below):** "a graphics card's rendering methods" means the different technologies Blender can use to drive a graphics card for rendering — in Cycles these are named CUDA, OptiX, HIP, oneAPI and Metal, and the manual itself calls them "Rendering Technologies". I also cover, under the same reading, the further choice available *within* some of those technologies about whether the card's hardware ray-tracing is used (HIP RT, Embree on GPU, MetalRT), because that is also a choice between methods a card offers for rendering.

**The other reading, which I answer briefly at the end:** the graphics interface Blender draws and rasterises through — OpenGL, Vulkan, Metal. That one governs the interface and the display-drawing renderers, not Cycles. I have kept it separate and short so the two cannot be confused.

---

### 7a. Where the choice is made

**LABEL: documented.**

There are three places, and one of them overrides the others.

**1. In the Preferences, as the Cycles Render Device.** The manual:

> "To enable GPU rendering, go into the Preferences ‣ System ‣ Cycles Render Devices, and select either CUDA, OptiX, HIP, oneAPI, or Metal. Next, you must configure each scene to use GPU rendering in Properties ‣ Render ‣ Device."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.

And on the Preferences page itself:

> "Cycles Render Device
>
> Changes the computing device the Cycles render engine uses to render images."

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html, read 2026-07-30.

**2. In the scene, as a plain CPU-or-GPU switch — but not which technology.** The scene setting only picks between the processor and "the GPU compute device… configured in the system tab in the user preferences". **LABEL: observed.**

> ```
> enum_devices = (
>     ('CPU', "CPU", "Use CPU for rendering"),
>     ('GPU', "GPU Compute",
>         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
> )
> ```

— Cycles' own files, `cycles_4.5/properties.py`, lines 30–34, read 2026-07-30. The setting that uses it:

> ```
>     device: EnumProperty(
>         name="Device",
>         description="Device to use for rendering",
>         items=enum_devices,
>         default='CPU',
>     )
> ```

— Cycles' own files, `cycles_4.5/properties.py`, lines 378–383, read 2026-07-30.

**3. On the command line, when Blender is started.** The manual:

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

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30. The manual also says on the same page: "Cycles add-on options must be specified following a double dash."

Cycles' own files say what this argument does to the other two places. **LABEL: observed.**

> ```
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)
> ```

— Cycles' own files, `cycles_4.5/engine.py`, lines 16–20, read 2026-07-30.

The same file shows the argument is only read after a bare `--` on the command line, and that Blender exits with a failure code if the value is not accepted. **LABEL: observed.**

> ```
>     argv = sys.argv
>     if "--" not in argv:
>         return
> ...
>     if args.cycles_device:
>         import _cycles
>         if not _cycles.set_device_override(args.cycles_device):
>             sys.exit(1)
> ```

— Cycles' own files, `cycles_4.5/engine.py`, lines 27–41, read 2026-07-30.

**Where the hardware ray-tracing switches live.** All three are settings on the Cycles add-on's preferences, alongside the device choice, and the manual documents them on the same Preferences page (quoted in 7c below). In Cycles' own files they are `metalrt`, `use_hiprt` and `use_oneapirt` on `class CyclesPreferences(bpy.types.AddonPreferences)` — `cycles_4.5/properties.py`, class opens at line 1555, properties at lines 1600–1622, read 2026-07-30. **LABEL: observed.** No command-line argument for any of them appears in the manual's argument list (see the record of looking, below).

**The picture.** I opened `_images/editors_preferences_section_system.png`, the illustration of the Preferences System section on the Preferences page. It shows an **open but empty** panel headed "Cycles Render Devices" at the top of the System section, above "Memory & Limits" — its disclosure arrow points down, the same as the plainly open "Memory & Limits" beside it, and the body below the heading is blank. The panel's contents are not visible in the picture, so it confirms only where the panel sits and what it is called. *(Citation check, 2026-08-30: this read "a collapsed panel", which the picture does not show — the arrow points down and there is an empty panel body. "Sound", at the foot of the same picture, is the one drawn collapsed. Changed by this check.)* **LABEL: documented** (it is part of the manual), read 2026-07-30.

**Not covered:** I did not find, and so cannot say, where the preference value is written to disk, nor whether the `--cycles-device` override persists into anything saved. I found no environment variable that sets the device (see the record of looking).

---

### 7b. What methods there are

**LABEL: documented**, and separately **observed** in Cycles' own files.

The manual lists five under the heading "Rendering Technologies", plus the introductory sentence:

> "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer and operating system."

and then, as headings: "CUDA – NVIDIA", "OptiX – NVIDIA", "HIP – AMD", "oneAPI – Intel", "Metal – Apple (macOS)".

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.

The Preferences page lists the same five plus "None":

> "None:
>
> When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html, read 2026-07-30.

Cycles' own files carry the same list twice. The stored values: **LABEL: observed.**

> ```
> enum_device_type = (
>     ('CPU', "CPU", "CPU", 0),
>     ('CUDA', "CUDA", "CUDA", 1),
>     ('OPTIX', "OptiX", "OptiX", 3),
>     ('HIP', "HIP", "HIP", 4),
>     ('METAL', "Metal", "Metal", 5),
>     ('ONEAPI', "oneAPI", "oneAPI", 6)
> )
> ```

— Cycles' own files, `cycles_4.5/properties.py`, lines 176–183, read 2026-07-30. (Note the gap: value 2 is not used.)

And the list actually offered in the preferences, built at the moment it is shown: **LABEL: observed.**

> ```
>     def get_device_types(self, context):
>         import _cycles
>         has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()
>
>         list = [('NONE', "None", n_("Do not use compute device"), 0)]
>         if has_cuda:
>             list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))
>         if has_optix:
>             list.append(('OPTIX', "OptiX", n_("Use OptiX for GPU acceleration"), 3))
>         if has_hip:
>             list.append(('HIP', "HIP", n_("Use HIP for GPU acceleration"), 4))
>         if has_metal:
>             list.append(('METAL', "Metal", n_("Use Metal for GPU acceleration"), 5))
>         if has_oneapi:
>             list.append(('ONEAPI', "oneAPI", n_("Use oneAPI for GPU acceleration"), 6))
>
>         return list
> ```

— Cycles' own files, `cycles_4.5/properties.py`, lines 1567–1583, read 2026-07-30.

**Inferred from those lines:** the list a person or a script sees is not fixed — it holds only the technologies the running copy of Blender reports as present, so a value that is valid on one machine may simply not be on the list on another. The step I took: the code builds the list by testing each `has_…` flag before adding the entry, so an untrue flag leaves that entry out. **LABEL: inferred.**

**The default, observed:** on Apple Silicon the compute device starts as Metal; everywhere else it starts as the first entry, which is `NONE`.

> ```
>     @staticmethod
>     def default_device():
>         import platform
>         # Default to selecting the Metal compute device on Apple Silicon GPUs
>         # (drivers are tightly integrated with macOS so pose no stability risk)
>         if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
>             return 5
>         return 0
> ```

— Cycles' own files, `cycles_4.5/properties.py`, lines 1558–1565, read 2026-07-30. Value 5 is Metal and value 0 is None, per the two lists quoted above.

**A further method choice inside three of them**, documented on the Preferences page:

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

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html, read 2026-07-30.

Their stored form and defaults, **observed**:

> ```
>     metalrt: EnumProperty(
>         name="MetalRT",
>         description="MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better "
>                     "performance in specific cases",
>         default='AUTO',
>         items=(
>             ('OFF', "Off", "Disable MetalRT (uses BVH2 layout for intersection queries)"),
>             ('ON', "On", "Enable MetalRT for intersection queries"),
>             ('AUTO', "Auto", "Automatically pick the fastest intersection method"),
>         ),
>     )
>
>     use_hiprt: BoolProperty(
>         name="HIP RT",
>         description="HIP RT enables AMD hardware ray tracing on RDNA2 and above",
>         default=False,
>     )
>
>     use_oneapirt: BoolProperty(
>         name="Embree on GPU",
>         description="Embree on GPU enables the use of hardware ray tracing on Intel GPUs, providing better overall performance",
>         default=True,
>     )
> ```

— Cycles' own files, `cycles_4.5/properties.py`, lines 1600–1622, read 2026-07-30. So MetalRT starts at Auto, HIP RT starts off, Embree on GPU starts on.

**And a fourth, Metal only:** kernel optimisation, **observed**:

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

— Cycles' own files, `cycles_4.5/properties.py`, lines 1624–1635, read 2026-07-30.

**Observed, that the three properties are written inside tests on which technology is chosen.** The code reads:

> ```
>         if compute_device_type == 'METAL':
>             import platform
>
>             # MetalRT only works on Apple Silicon.
>             if (platform.machine() == 'arm64'):
>                 col = layout.column()
>                 col.use_property_split = True
>                 col.prop(self, "kernel_optimization_level")
>                 row = col.row()
>                 row.active = has_hardware_rt
>                 row.prop(self, "metalrt")
>
>         if compute_device_type == 'HIP':
>             import platform
>             row = layout.row()
>             row.active = has_hardware_rt
>             row.prop(self, "use_hiprt")
>
>         elif compute_device_type == 'ONEAPI' and _cycles.with_embree_gpu:
>             row = layout.row()
>             row.active = has_hardware_rt
>             row.prop(self, "use_oneapirt")
> ```

— Cycles' own files, `cycles_4.5/properties.py`, lines 1898–1918, read 2026-07-30.

**Inferred from those lines:** kernel optimisation and MetalRT are shown only for Metal on Apple Silicon, HIP RT only for HIP, and Embree on GPU only for oneAPI. The step a reader must supply is twofold — that this block is what puts those controls on the screen, and that a block not reached draws nothing. Neither is stated in the file. **LABEL: inferred.** *(Citation check, 2026-08-30: this passage was labelled observed and read as a statement about what appears on screen. What the file carries is where the properties are written; what a person sees is a step past it, and this version's first pass labels that same step inferred. Changed by this check.)*

**Inferred from the same lines:** the hardware ray-tracing switch is greyed out unless every card being used supports it — the code sets `row.active = has_hardware_rt`, and just above it computes `has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt` under the comment "# Any device without RT support will disable it for all." (`cycles_4.5/properties.py`, lines 1889–1890). The step I took: `row.active` controls whether a row of the interface is live, so a false value means the switch cannot be worked. **LABEL: inferred.** Whether the *setting* is ignored in that case, as opposed to merely being unclickable, I could not establish.

**Not covered:** the sources do not give a full table of which technology each card model supports; the manual points outward for that ("see the list of NVIDIA graphics cards", "Please refer to AMD's website", "Please refer to Intel's website"). I did not follow those links, and could not, since they are outside the sources.

---

### 7c. What choosing one rather than another changes about the render

Several separate things. Each is labelled on its own.

**(i) Which cards can be used at all, and on which operating system. LABEL: documented.**

> "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 3.0 and higher."
>
> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."
>
> "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported."
>
> "oneAPI is a computation library that is supported on Windows and Linux and requires a Intel® Arc™ graphics card with the Xe HPG architecture."
>
> "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.

**(ii) Which features work, so the picture itself can differ. LABEL: documented.** These are the differences that change what is produced, not only how fast:

> "Shadow caustics are not supported with HIP."

> "Path Guiding is not supported on any GPU."

> "Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.

> "OSL is not supported with GPU rendering unless using the OptiX backend.
>
> Some OSL features are not available when using the OptiX backend. Examples include:
>
> Memory usage reductions offered by features like on-demand texture loading and
>
> mip-mapping are not available."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/osl/index.html, read 2026-07-30. *(Citation check, 2026-08-30: this address read `…/render/cycles/osl.html`, which answers to no file in the 4.5 snapshot. The passage is on `…/render/cycles/osl/index.html`, which is where this version's first pass cites it. Corrected by this check.)*

> "Custom cameras are not supported with GPU rendering unless using the OptiX backend."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/osl/camera.html, read 2026-07-30.

> "The Script node is only available when Open Shading Language is enabled in the Cycles render settings. This feature requires rendering with the CPU or OptiX backend."

— Manual, https://docs.blender.org/manual/en/4.5/render/shader_nodes/script.html, read 2026-07-30.

The release notes state the same about custom cameras as a change made in this version:

> "In addition to the built-in camera types (perspective, orthographic and panoramic), it's now possible to implement custom cameras using Open Shading Language. This can be enabled independently of OSL shading, but requires hardware support and is therefore currently limited to CPU and OptiX devices."

— Release notes, https://developer.blender.org/docs/release_notes/4.5/cycles/, read 2026-07-30.

**(iii) Whether the denoiser can be accelerated, and which denoisers are on offer. LABEL: documented.**

> "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards." (OptiX section)
>
> "GPU accelerated denoising is available on discrete Radeon RX 6000 and Radeon RX 7000 GPUs." (HIP section)
>
> "GPU accelerated denoising is available on all supported GPUs." (oneAPI section)
>
> "GPU accelerated denoising is available on Apple Silicon." (Metal section)

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.

And the denoiser choice itself is tied to the device chosen:

> "OptiX:
>
> Uses NVIDIA's OptiX AI denoiser. Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.
>
> Only available on NVIDIA GPUs when configured in the Cycles Render Device user preferences."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30.

**(iv) Speed, in a way the manual explicitly refuses to generalise. LABEL: documented.**

> "What renders faster?
>
> This varies depending on the hardware used. Different technologies also have different compute times depending on the scene tested. For the most up to date information on the performance of different devices, browse the Blender Open Data resource."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.

**(v) What happens when the card runs out of memory — and this differs between the technologies. LABEL: documented.**

> "There may be multiple causes, but the most common one is that there is not enough memory on your graphics card. Typically, the GPU can only use the amount of memory that is on the GPU… With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.

Note what that sentence names and what it does not: CUDA, OptiX, HIP and Metal are listed; oneAPI is not. I am **not** claiming oneAPI lacks the fallback — an omission from a list is not a statement. I am recording that the sources say nothing about oneAPI here. **LABEL for the omission itself: not in the sources.**

**(vi) How rays are actually traced against the scene, for the hardware ray-tracing choices. LABEL: documented**, and confirmed **observed** in Cycles' own files.

Documented, from the Preferences page quoted in full in 7b: MetalRT "Off" means "uses BVH2 layout for intersection queries", MetalRT "On" means "Enable MetalRT for intersection queries", and MetalRT "Auto" means "Automatically pick the fastest intersection method". HIP RT "Speeds up rendering by enabling AMD hardware ray tracing on RDNA2 and above". Embree on GPU "Enables the use of hardware ray tracing on Intel GPUs, providing better overall performance". And for OptiX: "OptiX takes advantage of hardware ray-tracing acceleration in RTX graphics cards, for improved performance" (Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html).

Observed, in Cycles' own files, the same fork written out — one of Embree, MetalRT, OptiX or HIP RT is used, and otherwise a fallback called BVH2:

> ```
> /* Device specific acceleration structures for ray tracing. */
>
> #if defined(__EMBREE__)
> #  include "kernel/device/cpu/bvh.h"
> #  define __BVH2__
> #elif defined(__METALRT__)
> #  include "kernel/device/metal/bvh.h"
> #elif defined(__KERNEL_OPTIX__)
> #  include "kernel/device/optix/bvh.h"
> #elif defined(__HIPRT__)
> #  include "kernel/device/hiprt/bvh.h"
> #else
> #  define __BVH2__
> #endif
> ```

— Cycles' own files, `cycles_4.5/source/kernel/bvh/bvh.h`, lines 17–30, read 2026-07-30. And the fallback describes itself:

> ```
> /* BVH2
>  *
>  * Bounding volume hierarchy for ray tracing, when no native acceleration
>  * structure is available for the device.
> ```

— Cycles' own files, `cycles_4.5/source/kernel/bvh/bvh.h`, lines 58–62, read 2026-07-30.

**Also observed:** the kernel files are organised per technology, with a separate folder for HIP RT alongside HIP. `cycles_4.5/source/kernel/device/` contains exactly these folders: `cuda`, `gpu`, `hip`, `hiprt`, `metal`, `optix`. Read 2026-07-30. There is no `oneapi` folder there; I draw no conclusion from that, because the oneAPI path in `bvh.h` is written inline (the `__KERNEL_ONEAPI__` block at lines 32–52 of the same file) rather than as a folder.

**(vii) Known faults specific to a technology, in this version. LABEL: documented.** The release notes state these as facts about 4.5:

> "There are known stability issues in Blender 4.5.0 LTS. These are being worked on together with AMD to resolve in a future release.
>
> * RDNA2 GPUs have rendering artifacts in volumes on Windows.
> * HIP RT has stability issues that can lead to crashes while editing objects in the viewport."

> "The minimum driver version required for OptiX has increased to 535."

> "When more than one Intel GPU is present and used for rendering, there may be crashes."

— Release notes, https://developer.blender.org/docs/release_notes/4.5/cycles/, read 2026-07-30.

**Not covered by any of the above:** nothing in the sources states that two technologies produce the same pixels for the same scene, nor that they differ. The feature lists tell you what will be missing; they do not tell you whether an otherwise-identical scene renders identically on CUDA and on OptiX. That question is unanswered by these sources.

---

### 7d. Does anything establish whether the choice changes with the size of the scene?

**LABEL: documented**, for four separate statements. Together they establish that the sources do tie this choice to what is in the scene — but loosely, and never with a number or a threshold.

**1. Speed differences between technologies are stated to be scene-dependent.**

> "Different technologies also have different compute times depending on the scene tested."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30. This is the closest thing to a direct answer: the sources say outright that which technology is faster is not a fixed fact but varies with the scene.

**2. MetalRT's benefit is stated in terms of what the scene contains.**

> "MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases."

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html, read 2026-07-30; the identical wording is **observed** in `cycles_4.5/properties.py`, lines 1601–1603. Note this is about *what kind of* content, not size as such.

**3. Metal's kernel optimisation is stated to depend on scene content.**

> "Kernels can be optimized based on scene content."

— Cycles' own files, `cycles_4.5/properties.py`, line 1626, read 2026-07-30. **LABEL: observed.**

**4. Scene size is stated to matter to GPU rendering generally, in memory terms.**

> "they also have some limitations in rendering complex scenes, due to more limited memory"

> "Heavy scenes can also make Blender unresponsive on newer GPUs, when using a lot of memory or executing expensive shaders, however this is generally less of a problem."

> "If you have a particularly heavy scene, Cycles can take up too much GPU time."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.

And the out-of-memory fallback quoted at 7c(v) is itself a size-dependent difference: it only bites once the scene no longer fits, and the sentence naming which technologies do it is the only place the technologies are distinguished on that point.

**What is not established. LABEL: not in the sources.** Nothing states a scene size, vertex count, memory figure or any other threshold at which one technology becomes preferable to another, and nothing states that the ranking of the technologies flips at some point. The manual's answer to "What renders faster?" is to send the reader to an outside resource rather than to state anything. There is one "Auto" that decides for itself — MetalRT's "Automatically pick the fastest intersection method" — but the sources do not say what it decides on, so I cannot say whether scene size is among its inputs.

**Not covered:** I cannot say whether the four statements above are about size, about content type, or both. "Scenes which use curves extensively" is a statement about content, not size; "complex scenes" and "heavy scenes" are not defined anywhere I found. Anyone treating these as a size rule is adding something the sources do not say.

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

**LABEL: documented, in part.**

The choice of technology can be made entirely from outside Blender's window, and this is the documented way to do it: the manual's own example is a background render.

> "blender -b file.blend -f 20 -- --cycles-device OPTIX"

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30.

Cycles' own files add that this "overrid[es] user preferences and the scene setting" (`cycles_4.5/engine.py`, line 17), so a render started from outside does not depend on what the preferences hold. **LABEL: observed.**

Two things differ from inside, and both are worth naming.

First, **the hardware ray-tracing switches have no command-line equivalent that I could find.** I searched the manual's full argument list and found only `--cycles-device` and `--cycles-print-stats` under "Cycles Render Options"; I found no argument for MetalRT, HIP RT, Embree on GPU or kernel optimisation, and no environment variable for them in the manual's "Environment Variables" section. **LABEL: not in the sources** for any command-line route to them. They are settings on the Cycles add-on's preferences (`bpy.types.AddonPreferences`, `cycles_4.5/properties.py`, line 1555, **observed**), so a script run inside Blender could reach them; the sources do not describe doing so, and I am not going to invent the path.

Second, **the preferences these settings live in are per-machine and saved outside the file being rendered.** The manual: "By default changes to preferences are saved on exit" (https://docs.blender.org/manual/en/4.5/editors/preferences/introduction.html, read 2026-07-30). **Inferred:** a headless render on a machine whose preferences have never been set will not have a technology chosen for it, and the observed default (`default_device()`, quoted at 7b) is `NONE` — meaning the CPU — everywhere except Apple Silicon. The step I took: the default is what the setting holds when nothing has changed it, and `NONE` is documented as meaning the CPU is used. **LABEL: inferred.** This is exactly the case where `--cycles-device` matters, and it is why I flag it.

**Not covered:** the sources do not say whether `--cycles-device` also affects which hardware ray-tracing path is taken, nor what happens if the named technology is unavailable on the machine — only that Cycles' own file exits with a failure code when `set_device_override` returns false, which is one step short of saying which values cause that.

### Operating system

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Tested rather than inferred: the Creator's Mac offers only `None` and `Metal` and starts at `Metal`; a rented Linux machine with one NVIDIA card offers `None`, `CUDA`, `OptiX`, `HIP` and `oneAPI` and starts at `None`. So the list of methods does differ by operating system — **but it is built from what that build supports rather than from the cards present**, since the Linux machine offered HIP and oneAPI with no AMD and no Intel graphics in it. `--gpu-backend` accepts `opengl` and `vulkan` on Linux and only `metal` on macOS, naming the rejected value in an error and then finishing the render anyway. **Windows is not settled**: this project's route to a Windows computer is a rental with two general-purpose cores and no graphics card at all, so four card-free Windows tests are written out ready to run and wait on the next Windows batch.]*

**LABEL: documented.** No, this does not hold the same on every operating system — the sources are explicit that the available methods differ by platform.

> "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer and operating system."

CUDA, OptiX, HIP and oneAPI are each stated as "supported on Windows and Linux". Metal is "supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.

**Inferred:** macOS is therefore left with Metal alone among the graphics-card methods, and Windows and Linux with the other four. The step I took: the four are each stated as Windows-and-Linux and Metal as Apple-only, and the sources list no other technologies. **LABEL: inferred.** The sources do not state this exclusion outright.

Further OS-specific detail, **documented**: minimum drivers for HIP differ by platform ("Windows: Radeon Software 24.6.1 or Radeon PRO Software 24.Q2 / Linux: Radeon Software 23.40 or ROCm 6.0"), as do oneAPI's; the release notes record a Windows-only artifact fault on RDNA2 and an Ubuntu 25.04 packaging problem for Intel hardware ray-tracing (https://developer.blender.org/docs/release_notes/4.5/cycles/, read 2026-07-30); and MetalRT and kernel optimisation are shown only on Apple Silicon (`cycles_4.5/properties.py`, lines 1898–1906, **observed**). The manual's Linux-only troubleshooting section covers CUDA compilation with GCC.

**Not covered:** the sources say nothing about Intel Macs, nor about any operating system beyond Windows, Linux and macOS.

---

### The other reading, kept separate

If "a graphics card's rendering methods" was meant to be the graphics interface Blender draws through, the answer is a different setting entirely, and it is not Cycles.

**LABEL: documented.** In Preferences ▸ System ▸ Display Graphics:

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

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html, read 2026-07-30.

From outside: "--gpu-backend / Force to use a specific GPU backend. Valid options: vulkan, metal, opengl." — Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30. And in the API: "gpu_backend / GPU backend to use (requires restarting Blender for changes to take effect) / OPENGL… METAL… VULKAN" — Python API, https://docs.blender.org/api/4.5/bpy.types.PreferencesSystem.html, read 2026-07-30.

**And here the sources do give a scene-size statement, with a number.** The release notes:

> "Vulkan backend cannot handle huge meshes (100.000.000 vertices). Vulkan has different limits compared to OpenGL. These limits are controlled by the driver and vendor specific. Changing drivers could increase these limits."
>
> "All textures should fit inside the GPU memory. Vulkan has different limits compared to OpenGL."

— Release notes, https://developer.blender.org/docs/release_notes/4.5/eevee/, read 2026-07-30. The same page states: "OpenGL is still the default in Blender 4.5 LTS, users are encouraged to try Vulkan by enabling it in Preferences", and "Virtual reality using Vulkan has performance issues. For virtual reality we will recommend to use OpenGL."

**Not covered:** I did not pursue this reading further, since I judge it not to be the one meant. If it is the one meant, this section is a starting point and not a full answer — in particular I did not establish whether the backend choice affects what EEVEE or Workbench actually produce.

---

## Record of where I looked

**How I searched.** I searched the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` at once with case-insensitive text search, rather than opening files by guess.

**Terms searched, across all four source sets:** `OptiX`, `CUDA`, `HIP`, `oneAPI`, `Metal`, `MetalRT`, `HIP RT`, `hiprt`, `Embree`, `Embree on GPU`, `compute device`, `compute_device_type`, `Cycles Render Devices`, `device_type`, `kernel_optimization`, `hardware ray`, `hardware-accelerated ray`, `ray tracing acceleration`, `ray-tracing acceleration`, `intersection method`, `BVH`, `GPU Compute`, `gpu-backend`, `gpu_backend`, `cycles-device`, `cycles_device`, `CYCLES_`, `argv`.

**Terms searched specifically to test the scene-size question:** `complex scenes`, `large scenes`, `heavy scene`, `scene content`, `depending on the scene`, `scene complexity`, `scene size`. These were run across the manual, the release notes and the Cycles Python files. Everything the sweep returned that bears on rendering technology is quoted above; the remaining hits were about unrelated things (video sequencer scene size, viewport overlays, fluid baking, drivers, clipping, the array modifier, EEVEE light probes) and I ruled them out for that reason.

**Terms searched to test feature differences:** a combined search for `not supported`, `unsupported`, `only supported` and `requires` within eighty characters of each technology name, across the whole `render` part of the manual. That returned seven lines; all seven are quoted in 7c(ii) and 7c(iii).

**Pages opened in full:**
- Manual, GPU Rendering — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html (read end to end, all 220 lines)
- Manual, Preferences ▸ System — https://docs.blender.org/manual/en/4.5/editors/preferences/system.html (first 120 lines, covering Cycles Render Device through Display Graphics)
- Manual, Command Line Arguments — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (Cycles Render Options section, GPU Options section, Environment Variables section)
- Manual, Preferences introduction — https://docs.blender.org/manual/en/4.5/editors/preferences/introduction.html (Managing Preferences section)
- Manual, Sampling — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html (denoising section)
- Manual, Open Shading Language — https://docs.blender.org/manual/en/4.5/render/cycles/osl/index.html (Limitations) *(address corrected by the citation check, 2026-08-30)*
- Manual, OSL custom cameras — https://docs.blender.org/manual/en/4.5/render/cycles/osl/camera.html (Limitations)
- Manual, Script node — https://docs.blender.org/manual/en/4.5/render/shader_nodes/script.html
- Manual, Bevel node and Ambient Occlusion node — https://docs.blender.org/manual/en/4.5/render/shader_nodes/input/bevel.html and .../ao.html (the OptiX notes)
- Release notes, Cycles — https://developer.blender.org/docs/release_notes/4.5/cycles/ (read in full)
- Release notes, EEVEE — https://developer.blender.org/docs/release_notes/4.5/eevee/ (Vulkan section)
- Release notes, bug fixes — https://developer.blender.org/docs/release_notes/4.5/bugfixes/ (the device-related lines only)
- Python API, PreferencesSystem — https://docs.blender.org/api/4.5/bpy.types.PreferencesSystem.html (`gpu_backend`, `legacy_compute_device_type`)
- Cycles' own files: `cycles_4.5/properties.py` (lines 30–34, 176–183, 378–383, 1548–1650, 1782–1920), `cycles_4.5/engine.py` (lines 1–60), `cycles_4.5/source/kernel/bvh/bvh.h` (lines 1–75), `cycles_4.5/source/kernel/device/metal/bvh.h` (opening comment), and a directory listing of `cycles_4.5/source/kernel/device/`
- Picture: `_images/editors_preferences_section_system.png`

**What I ruled out, and why.**
- **The Python API reference holds almost nothing on this.** A search of all 1,851 API files for `optix`, `metalrt`, `hiprt`, `oneapi` and `compute_device` returned only four files, and two of those are the alphabetical index. The only substantive hits are `gpu_backend` (the drawing backend, not Cycles) and `legacy_compute_device_type`, which is documented as "For backwards compatibility only" and read-only. **This is expected and I want to be plain about why I am not treating it as a gap:** Cycles is an add-on, and its settings are not part of `bpy.types`, so they do not appear in the API reference at all. The Cycles Python files are the authority instead, and that is where I went.
- **No environment variable sets the device.** The manual's Environment Variables section lists only `BLENDER_USER_*` and `BLENDER_SYSTEM_*` entries. The only Cycles-related environment variable anywhere in the manual is `CYCLES_CUDA_EXTRA_CFLAGS`, and it is about compiling a kernel on Linux with an alternate compiler, not about choosing a technology.
- **The EEVEE and Workbench parts of the manual** do not offer a choice between a card's rendering technologies; I checked and found only the shared Display Graphics backend, covered under the other reading.
- **The `_images` folder** was searched along with the text and produced hits inside two unrelated SVG diagrams of the video sequencer, which I ruled out as coincidental letter matches.

---

## Things a running Blender could still settle

*(Pruned 2026-08-30 by the hand tests. Everything the earlier list held has either been run — the results are above, under **Tested by hand — what a running Blender showed** — or is recorded there as unable to be run, with its reason. What follows is what is genuinely still open.)*

1. **What happens when the card's own memory fills.** The scene of 125,829,122 triangles reached the host machine's memory ceiling and not the card's, so the documented fallback to system memory was never exercised. Settling it needs a machine whose card is small against what its host can build, which is the opposite of the usual want.
2. **What MetalRT's "Auto" weighs.** On the Creator's Mac it built the same structure as Off on all three scenes, including one of 300,000 curves — in the tests above, and again in sixty more runs on 2026-08-30, in all four versions, without one exception. Whether it ever picks the other way — on other Apple hardware, or on some other kind of scene — is still not settled.
3. **Whether the four card-free Windows tests give the same answers.** They are written out ready to run in `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md` and wait on the next Windows rental.
4. **Everything needing hardware nobody here has**: an AMD card, an Intel Arc card, two cards of different ray-tracing generations in one machine, NVLink-joined cards, a Mac that is not Apple silicon, or an Apple machine an agent may open a window on. Each is named above with its reason.

---

2026-08-30 — Claude Opus 5 (Claude Code) — What a render demands, question 7, Blender 4.5

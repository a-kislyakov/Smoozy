# What a render demands — Open Threads questions, second pass

**Blender 5.1. Question 7 only.**

- **Sources consulted, and on what date.** The Blender 5.1 manual, the Blender 5.1 Python API documentation, the Blender 5.1 release notes, and Cycles' own files for 5.1 — all as supplied inside `Blender Sources/`, all taken on **2026-07-30**, which is the date read for everything below. I also opened one picture from the manual (`_images/editors_preferences_section_system.webp`, the screenshot on the System preferences page).
- I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

*(One point of full disclosure, so the line above is exact: to confirm the folder I was told to save into exists, I listed it. That showed me one file name and no file contents. I opened no findings document.)*

---

## Question 7

> Where a render can be told which of a graphics card's rendering methods to use, what methods are there, what does choosing one rather than another change about the render, and does anything establish whether that changes with the size of the scene?

### A note on the reading I took

The question is ambiguous, so I am saying plainly which reading I answered.

**The reading I answered:** the choice of the *compute backend* a graphics card is driven through when Cycles renders — the thing the manual calls a "rendering technology" and Cycles' own files call the "Compute Device Type". This is the only place in the sources where a render is told *which method of a graphics card* to use, as opposed to *which card* or *whether to use a card at all*.

**The other reading, which I also cover briefly at the end of this answer** (part E): the "Backend" under **Display Graphics** in the System preferences — OpenGL or Vulkan (Metal appears in the API documentation) — which is the graphics interface Blender draws through, and therefore the one EEVEE renders through.

**A third reading I considered and rejected:** EEVEE's Raytracing "Method" (Light Probe / Screen-Trace). I rejected it because the sources present it as a choice of *technique for finding intersections in the scene*, not as a method belonging to a graphics card. Quoted so you can judge the rejection yourself: "Determine the tracing method used to find scene-ray intersections and indirect lighting." — manual, https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/raytracing.html, read 2026-07-30.

---

### 7A. Where a render can be told which method to use

**LABEL: documented** (with one **observed** part, marked).

There are three places, and one of them beats the other two.

**1. In the Preferences, under System.** This is the main one.

> "Cycles Render Devices
>
> Changes the Computing Device
> the Cycles render engine uses to render images."

— manual, System preferences page, https://docs.blender.org/manual/en/5.1/editors/preferences/system.html, read 2026-07-30.

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal
> in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

— manual, GPU Rendering page, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html, read 2026-07-30.

I opened the screenshot on that preferences page (`_images/editors_preferences_section_system.webp`, shown on https://docs.blender.org/manual/en/5.1/editors/preferences/system.html, read 2026-07-30). **LABEL: documented** — what is visible in it: "Cycles Render Devices" is a single row of three buttons reading **None**, **CUDA**, **HIP**, with **None** highlighted. Below it a panel "Display Graphics" holds one dropdown, "Backend", set to "OpenGL". Only three backend buttons are shown in that picture, not five.

**2. From the command line, when Blender is started.**

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU."
>
> "blender -b file.blend -f 20 -- --cycles-device OPTIX"

— manual, Command Line Arguments page, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30. The same page notes: "Cycles add-on options must be specified following a double dash."

Cycles' own files say what this option outranks. **LABEL: observed.**

> `parser.add_argument("--cycles-device",`
> `                    help="Set the device to use for Cycles, overriding user preferences and the scene setting."`
> `                         "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."`
> `                         "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",`
> `                    default=None)`

— Cycles' own files, `cycles_5.1/engine.py`, lines 16–20, inside `_configure_argument_parser()`. Read 2026-07-30.

**3. From a script, as a setting on the Cycles add-on.** **LABEL: observed.**

The setting's own definition:

> `    compute_device_type: EnumProperty(`
> `        name="Compute Device Type",`
> `        description="Device to use for computation (rendering with Cycles)",`
> `        default=CyclesPreferences.default_device(),`
> `        items=CyclesPreferences.get_device_types,`
> `    )`

— Cycles' own files, `cycles_5.1/properties.py`, lines 1599–1603, in class `CyclesPreferences`. Read 2026-07-30.

And the path Cycles itself uses to read it:

> `def get_device_type(context):`
> `    return context.preferences.addons[__package__].preferences.compute_device_type`

— Cycles' own files, `cycles_5.1/ui.py`, lines 96–97. Read 2026-07-30.

**Note on where this setting lives.** It is a *user preference*, not a scene setting. It is not part of the scene at all — the scene only carries a CPU-or-GPU switch:

> `enum_devices = (`
> `    ('CPU', "CPU", "Use CPU for rendering"),`
> `    ('GPU', "GPU Compute",`
> `        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),`
> `)`

— Cycles' own files, `cycles_5.1/properties.py`, lines 30–34. Read 2026-07-30. That `device` setting sits on `CyclesRenderSettings` at lines 381–387 of the same file, with `default='CPU'`.

The manual says the same thing in words:

> "GPU Compute:
>
> Render using one or more graphics processing units (GPUs) through the compute backend
> and devices selected in the Cycles Render Devices."

— manual, Cycles Render Settings page, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html, read 2026-07-30.

Preferences are stored separately from scenes:

> "./config/userpref.blend
>
> User preferences."

— manual, Blender's Directory Layout page, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html, read 2026-07-30.

**Not covered.** The sources do not state whether the preferences file is read when Blender is started without its window, nor whether `--cycles-device` can be given more than once, nor what happens if it names a backend the machine does not have (Cycles' own files show only that a failure to set it exits with status 1 — `cycles_5.1/engine.py`, lines 38–41 — but not what counts as failure). The sources do not say whether the choice can be changed part-way through a render.

**Outside the interface.** The command-line route is documented specifically for this: the manual's own example is `blender -b file.blend -f 20 -- --cycles-device OPTIX`, and `-b` is background mode. So yes, the choice can be made entirely from outside Blender's window, and from outside it can be made *per launch* rather than per installation, which is a real difference from the preferences route. The script route (`context.preferences.addons[...]`) is a Python setting and needs a running Blender to set. Whether the saved preference is honoured in background mode is **not in the sources** — I searched the command-line page for "background", "factory-startup" and "userpref" and found nothing that says either way.

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Four things that were open here are settled by test, in all four versions and on both computers. **`--cycles-device` does outrank both the stored preference and the scene's own setting**, in both directions. **A stored preference is applied by a Blender started with no window**, but only puts the render on the card when the technology's own device is ticked in the preference *and* the scene is set to the card; a preference naming the technology with no device ticked renders on the processor. **All six Cycles preference settings can be read and set from a script with no window** — the technology, MetalRT, HIP RT, Embree on GPU, Kernel Optimization and Distribute Memory Across Devices — and MetalRT and Kernel Optimization were then shown to change the render, so the assignment is not merely accepted. **A method the machine cannot do, or a name that is not a method, ends the run** with exit status 1 and no file written; nothing falls back. What is unchanged: there is still no command-line option for the three hardware ray-tracing switches.]*

**Operating system.** The *place* is the same on every system. *Which methods appear there* is not — see 7B. The manual states that CUDA, OptiX, HIP and oneAPI are "supported on Windows and Linux", and Metal "is supported on Apple computers with Apple Silicon" (GPU Rendering page, same address, read 2026-07-30). Cycles' own files show one operating-system difference in the *default*:

> `    @staticmethod`
> `    def default_device():`
> `        import platform`
> `        # Default to selecting the Metal compute device on Apple Silicon GPUs`
> `        # (drivers are tightly integrated with macOS so pose no stability risk)`
> `        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):`
> `            return 5`
> `        return 0`

— Cycles' own files, `cycles_5.1/properties.py`, lines 1572–1579. **LABEL: observed** for the code. That a value of 5 means Metal and 0 means None is **inferred** — the step is that the list built at lines 1585–1595 of the same file gives `('NONE', …, 0)` and `('METAL', "Metal", …, 5)`. So on Apple Silicon the method starts out as Metal, and everywhere else it starts out as None.

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Tested rather than inferred: the Creator's Mac offers only `None` and `Metal` and starts at `Metal`; a rented Linux machine with one NVIDIA card offers `None`, `CUDA`, `OptiX`, `HIP` and `oneAPI` and starts at `None`. So the list of methods does differ by operating system — **but it is built from what that build supports rather than from the cards present**, since the Linux machine offered HIP and oneAPI with no AMD and no Intel graphics in it. `--gpu-backend` accepts `opengl` and `vulkan` on Linux and only `metal` on macOS, naming the rejected value in an error and then finishing the render anyway. **Windows is not settled**: this project's route to a Windows computer is a rental with two general-purpose cores and no graphics card at all, so four card-free Windows tests are written out ready to run and wait on the next Windows batch.]*

---

### 7B. What methods there are

**LABEL: documented, and separately observed.**

**Documented.** The manual names five, each under a heading of its own on the GPU Rendering page:

> "Rendering Technologies
>
> Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer
> and operating system."

with the headings "CUDA – NVIDIA", "OptiX – NVIDIA", "HIP – AMD", "oneAPI – Intel", "Metal – Apple (macOS)".

— manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html, read 2026-07-30.

The System preferences page lists the same five plus a sixth entry, None:

> "None:
>
> When set to None or when the only option is None:
> the CPU will be used as the computing device for Cycles."

— manual, https://docs.blender.org/manual/en/5.1/editors/preferences/system.html, read 2026-07-30.

**Observed.** Cycles' own files build the list at run time, and the descriptions there are the ones a person sees as tooltips:

> `    def get_device_types(self, context):`
> `        import _cycles`
> `        has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()`
>
> `        list = [('NONE', "None", n_("Do not use compute device"), 0)]`
> `        if has_cuda:`
> `            list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))`
> `        if has_optix:`
> `            list.append(('OPTIX', "OptiX", n_("Use OptiX for GPU acceleration"), 3))`
> `        if has_hip:`
> `            list.append(('HIP', "HIP", n_("Use HIP for GPU acceleration"), 4))`
> `        if has_metal:`
> `            list.append(('METAL', "Metal", n_("Use Metal for GPU acceleration"), 5))`
> `        if has_oneapi:`
> `            list.append(('ONEAPI', "oneAPI", n_("Use oneAPI for GPU acceleration"), 6))`
>
> `        return list`

— Cycles' own files, `cycles_5.1/properties.py`, lines 1581–1597. Read 2026-07-30.

A second, fixed list of the same names, used for tagging individual devices, sits earlier in the same file:

> `enum_device_type = (`
> `    ('CPU', "CPU", "CPU", 0),`
> `    ('CUDA', "CUDA", "CUDA", 1),`
> `    ('OPTIX', "OptiX", "OptiX", 3),`
> `    ('HIP', "HIP", "HIP", 4),`
> `    ('METAL', "Metal", "Metal", 5),`
> `    ('ONEAPI', "oneAPI", "oneAPI", 6)`
> `)`

— Cycles' own files, `cycles_5.1/properties.py`, lines 172–179. Read 2026-07-30.

The command line accepts the same six names: "Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL" (manual, Command Line Arguments page, address above, read 2026-07-30).

**Only the methods this build of Blender supports are offered.** **LABEL: inferred.** The code above only appends a backend when the corresponding flag comes back true from `_cycles.get_device_types()`; the step a reader must take is that a name not appended is a name not in the list a person can pick from. *(Citation check, 2026-08-30: **this step was tested and it does not hold.** Finding 1 under "Tested by hand" records a machine with one NVIDIA card, no AMD card and no Intel graphics still offering HIP and oneAPI. The flags report what that build of Blender supports, not what the computer holds; what is empty is the device list underneath each choice. Corrected by this check.)* The manual states the matching consequence for the surrounding settings: "If your hardware does not support some of the options described on this page, then they will either not be displayed or be corrected on startup" (manual, System preferences page, address above, read 2026-07-30). The screenshot I opened, showing only None / CUDA / HIP, is consistent with this.

**Sub-methods within a method.** Three of the five carry a further on/off choice about *how* the card traces rays. These are separate settings, not separate entries in the list of methods.

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

— manual, System preferences page, https://docs.blender.org/manual/en/5.1/editors/preferences/system.html, read 2026-07-30.

Cycles' own files give their defaults, which the manual does not. **LABEL: observed.** `metalrt` is an enum with `default='AUTO'` (`cycles_5.1/properties.py`, lines 1614–1624); `use_hiprt` is a switch with `default=True` (lines 1626–1630); `use_oneapirt` is a switch with `default=True` (lines 1632–1636).

The 5.1 release notes state a change here outright:

> "For AMD GPUs, hardware ray-tracing is now enabled by default through HIP RT."

— release notes, https://developer.blender.org/docs/release_notes/5.1/cycles/, read 2026-07-30.

Cycles' own files write each of these inside a test on which method is chosen. **LABEL: observed** for that.

> `        if compute_device_type == 'METAL':`
> `            import platform`
>
> `            # MetalRT only works on Apple Silicon.`
> `            if (platform.machine() == 'arm64'):`
> `                col = layout.column()`
> `                col.use_property_split = True`
> `                col.prop(self, "kernel_optimization_level")`
> `                row = col.row()`
> `                row.active = has_hardware_rt`
> `                row.prop(self, "metalrt")`
>
> `        if compute_device_type == 'HIP':`
> `            row = layout.row()`
> `            row.active = has_hardware_rt`
> `            row.prop(self, "use_hiprt")`
>
> `        elif compute_device_type == 'ONEAPI' and _cycles.with_embree_gpu:`
> `            row = layout.row()`
> `            row.active = has_hardware_rt`
> `            row.prop(self, "use_oneapirt")`

— Cycles' own files, `cycles_5.1/properties.py`, lines 1915–1935, in `CyclesPreferences.draw_impl`. Read 2026-07-30.

**Inferred from those lines:** each control is therefore *shown* only when its own method is chosen, and Kernel Optimization only for Metal on Apple Silicon. The steps a reader must supply are that this function is what draws the Preferences panel, so a control inside a test appears only when that test passes, and that a block not reached draws nothing. Neither is stated in the file. **LABEL: inferred.** *(Citation check, 2026-08-30: this passage was labelled observed and read as a statement about what appears in the panel, and its line range stopped two lines short of its own last quoted line. Both corrected by this check; the first pass of this version labels the same step inferred.)*

The same passage shows a settings row that only appears for Metal on Apple Silicon and nowhere else — Kernel Optimization, which the manual does not describe:

> `    kernel_optimization_level: EnumProperty(`
> `        name="Kernel Optimization",`
> `        description="Kernels can be optimized based on scene content. Optimized kernels are requested at the start of a render. "`
> `                    "If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set "`
> `                    "is available in the cache. This can result in additional CPU usage for a brief time (tens of seconds)",`
> `        default='FULL',`
> `        items=(`
> `            ('OFF', "Off", "Disable kernel optimization. Slowest rendering, no extra background CPU usage"),`
> `            ('INTERSECT', "Intersection only", "Optimize only intersection kernels. Faster rendering, negligible extra background CPU usage"),`
> `            ('FULL', "Full", "Optimize all kernels. Fastest rendering, may result in extra background CPU usage"),`
> `        ),`
> `    )`

— Cycles' own files, `cycles_5.1/properties.py`, lines 1638–1651. Read 2026-07-30.

**Not covered.** The sources do not say how a machine is tested for each method (`_cycles.get_device_types()` is a compiled call, not a file I was given). They do not settle whether MetalRT, HIP RT and Embree-on-GPU can be set from the command line — I searched the whole command-line page and found only `--cycles-device`; nothing names these three. They do not say what "Auto" actually picks for MetalRT, or what "has_hardware_rt" resolves to on any given card beyond the code at `properties.py` lines 1895–1912, which only shows that a single device lacking ray-tracing support turns the option off for all of them: "# Any device without RT support will disable it for all."

**Outside the interface.** The list of methods is the same list the command line accepts, and the command-line spelling is upper-case (`OPTIX`, `ONEAPI`) matching the internal names rather than the display names ("OptiX", "oneAPI"). MetalRT, HIP RT, Kernel Optimization and Embree on GPU have no documented command-line equivalent; the passage above shows they are only *drawn* in the preferences window, but they are ordinary settings, so whether they can be set from a script and take effect is **not in the sources**.

**Operating system.** No. Which of the five is available depends on the operating system and the card. The manual's own words: CUDA "is supported on Windows and Linux"; OptiX "is supported on Windows and Linux"; HIP "is supported on Windows and Linux"; oneAPI "is supported on Windows and Linux"; "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." (manual, GPU Rendering page, address above, read 2026-07-30). Cycles' own files add that MetalRT is gated on the processor kind, not just the operating system — the comment reads "# MetalRT only works on Apple Silicon." (`properties.py`, line 1918).

---

### 7C. What choosing one rather than another changes about the render

This splits into things the sources state plainly and things they only state about performance. I have kept them apart.

#### 7C-i. It changes which Cycles features work at all

**LABEL: documented.**

> "Limitations
>
> Path Guiding is not supported on any GPU.
>
> Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

— manual, GPU Rendering page, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html, read 2026-07-30.

> "OSL is supported only when rendering with the CPU or with the
> OptiX GPU Compute backend."

— manual, Cycles Render Settings page, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html, read 2026-07-30.

> "OSL is not supported with GPU rendering unless using the OptiX backend.
>
> Some OSL features are not available when using the OptiX backend. Examples include:
>
> Memory usage reductions offered by features like on-demand texture loading and
>
> mip-mapping are not available.
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

— manual, Open Shading Language page, https://docs.blender.org/manual/en/5.1/render/cycles/osl/index.html, read 2026-07-30.

This last one is the sharpest answer to "what does choosing one rather than another change about the render": with OptiX and OSL together, two shading nodes stop producing a valid result, and the manual repeats the warning on both of those nodes' own pages:

> "The Ambient Occlusion node will not produce a valid result when: … Open Shading Language is active while using the OptiX rendering backend."

— manual, Ambient Occlusion Node page, https://docs.blender.org/manual/en/5.1/render/shader_nodes/input/ao.html, read 2026-07-30. The same sentence appears on the Bevel Node page, https://docs.blender.org/manual/en/5.1/render/shader_nodes/input/bevel.html, read 2026-07-30.

And custom cameras:

> "Custom cameras are not supported with GPU rendering unless using the OptiX backend."

— manual, Custom Camera page, https://docs.blender.org/manual/en/5.1/render/cycles/osl/camera.html, read 2026-07-30.

The general warning:

> "GPU rendering may have limitations compared to CPU rendering, depending on the backend."

— manual, Cycles Render Settings page, address above, read 2026-07-30.

Cycles' own files enforce the OSL rule in what is even shown:

> `        if engine.with_osl() and (`
> `            use_cpu(context) or (`
> `                use_optix(context) and (`
> `                engine.osl_version()[1] >= 13 or engine.osl_version()[0] > 1))):`
> `            osl_col = layout.column()`
> `            osl_col.prop(cscene, "shading_system")`

— Cycles' own files, `cycles_5.1/ui.py`, lines 2466–2471. **LABEL: observed.** Note the extra condition the manual does not mention: OSL is only offered with OptiX when the OSL version is at least 1.13.

Path guiding likewise:

> `    @classmethod`
> `    def poll(cls, context):`
> `        from . import engine`
> `        return use_cpu(context) and engine.with_path_guiding()`

— Cycles' own files, `cycles_5.1/ui.py`, line 365, in `CYCLES_RENDER_PT_sampling_path_guiding`. **LABEL: observed.**

One more, only in the files: a feature turned off for Metal on older macOS.

> `def use_mnee(context):`
> `    # The MNEE kernel doesn't compile on macOS < 13.`
> `    if use_metal(context):`
> `        import platform`
> `        version, _, _ = platform.mac_ver()`
> `        major_version = version.split(".")[0]`
> `        if int(major_version) < 13:`
> `            return False`
> `    return True`

— Cycles' own files, `cycles_5.1/ui.py`, lines 197–205. **LABEL: observed.**

#### 7C-ii. It changes which acceleration-structure settings are offered

**LABEL: observed.**

> `class CYCLES_RENDER_PT_performance_acceleration_structure(CyclesButtonsPanel, Panel):`
> `    bl_label = "Acceleration Structure"`
> `    bl_parent_id = "CYCLES_RENDER_PT_performance"`
>
> `    @classmethod`
> `    def poll(cls, context):`
> `        return not use_optix(context) or use_multi_device(context)`

— Cycles' own files, `cycles_5.1/ui.py`, lines 880–887. Read 2026-07-30.

The settings inside that panel are the ones the manual describes as trading render speed against memory:

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

— manual, Cycles Performance page, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html, read 2026-07-30.

**LABEL: inferred** for the consequence: choosing OptiX (without a CPU alongside) removes that whole set of speed-versus-memory controls from the render. The step a reader has to take is from "the panel's `poll` returns false" to "the settings in it are not available to change"; the manual's Performance page does not mention this at all.

#### 7C-iii. It changes how rays are actually traced

**LABEL: observed.**

Cycles' kernel source picks a different intersection implementation per method:

> `/* Device specific acceleration structures for ray tracing. */`
>
> `#if defined(__EMBREE__)`
> `#  include "kernel/device/cpu/bvh.h"`
> `#  define __BVH2__`
> `#elif defined(__KERNEL_METALRT__)`
> `#  include "kernel/device/metal/bvh.h"`
> `#elif defined(__KERNEL_OPTIX__)`
> `#  include "kernel/device/optix/bvh.h"`
> `#elif defined(__KERNEL_HIPRT__)`
> `#  include "kernel/device/hiprt/bvh.h"`
> `#else`
> `#  define __BVH2__`
> `#endif`

— Cycles' own files, `cycles_5.1/source/kernel/bvh/bvh.h`, lines 18–30. Read 2026-07-30.

The matching folders exist: `cycles_5.1/source/kernel/device/` contains `cuda`, `gpu`, `hip`, `hiprt`, `metal`, `optix`.

**What this shows and what it does not.** It shows, as a fact about the files, that Metal-with-MetalRT, OptiX and HIP-with-HIP-RT each use their own intersection code, while everything else falls back to a shared `__BVH2__` path. It does **not** show what a render looks like or how long it takes as a result — that step is beyond the file, and I am not taking it.

#### 7C-iv. It changes what happens when the card runs out of memory

**LABEL: documented.** This is the single most consequential difference I found, and it is the one that also answers part D.

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
> try to use system memory. This has a performance impact, but will usually still result in a faster render
> than using CPU rendering."

— manual, GPU Rendering page, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html, read 2026-07-30.

**oneAPI is not in that list.** Naming four of the five, and only four, is what the sentence says. Whether the omission means oneAPI does not fall back, or merely that the sentence was not updated, is **not in the sources** — I searched the whole manual for "system memory", "host memory", "out of core" and "unified memory" and found no second statement either way.

#### 7C-v. It changes how many cards can pool memory, and how

**LABEL: documented.**

> "Would multiple GPUs increase available memory?
>
> Typically, no, each GPU can only access its own memory.
>
> The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost.
> This can be enabled with Distributed Memory Across Devices
> in the preferences."

— manual, GPU Rendering page, address above, read 2026-07-30.

> "Distribute Memory Across Devices
>
> Allocates resources across multiple GPUs rather than duplicating data,
> effectively freeing up space for larger scenes. Note that in order for this option to be available,
> the GPUs must be connected together with a high bandwidth communication protocol.
>
> Currently only NVLink on NVIDIA GPUs is supported."

— manual, System preferences page, address above, read 2026-07-30.

Cycles' own files give the default the manual omits: `peer_memory` is a switch with `default=False` (`cycles_5.1/properties.py`, lines 1608–1612). **LABEL: observed.**

#### 7C-vi. It changes what denoising is available

**LABEL: documented.**

> "OptiX:
>
> Uses NVIDIA's OptiX AI denoiser.
> Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.
>
> Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences."

— manual, Cycles Sampling page, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html, read 2026-07-30.

The GPU Rendering page adds per-method statements: for OptiX, "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards"; for HIP, "GPU accelerated denoising is available on discrete Radeon RX 6000 and newer"; for oneAPI, "GPU accelerated denoising is available on all supported GPUs"; for Metal, "GPU accelerated ray-tracing and denoising is available on Apple Silicon." (manual, GPU Rendering page, address above, read 2026-07-30.)

#### 7C-vii. It changes speed — but the sources refuse to say by how much

**LABEL: documented.**

> "OptiX takes advantage of hardware ray-tracing acceleration in RTX graphics cards, for improved performance."

> "What renders faster?
>
> This varies depending on the hardware used. Different technologies also have different compute times
> depending on the scene tested. For the most up to date information on the performance of different devices,
> browse the Blender Open Data resource."

— manual, GPU Rendering page, address above, read 2026-07-30.

That is as far as the manual goes. It gives no figures for any method, and points outward for them.

**Not covered (for all of 7C).** The sources give no numbers for how much any method differs from another in speed or memory on any named scene. They do not say whether a picture rendered with one method is identical to the same picture rendered with another — the only image-affecting differences I found are the OSL-plus-OptiX ones (Ambient Occlusion and Bevel nodes, "will not produce a valid result"), and those depend on OSL being switched on, not on the method alone. They do not say what happens when a scene uses a feature the chosen method does not support — whether it errors, falls back, or renders the feature away.

**Outside the interface.** The feature limits (OSL, path guiding, custom cameras) are properties of the method, so they hold wherever the render is driven from; nothing in the sources ties them to the window. The differences in *which settings are offered* (7C-ii) are drawn in panels, so from outside the interface the settings still exist and could still be set — what disappears is the place to click them. I am flagging that as the one part of this answer that is interface-shaped. Whether an OptiX render actually reads the acceleration-structure settings when they are set from a script is **not in the sources**.

**Operating system.** No, and it cannot. Since the available methods differ by operating system (7B), so do the consequences. In particular: on macOS the only GPU method is Metal, so the OptiX-only capabilities — OSL on GPU, custom cameras on GPU, the OptiX denoiser — are unavailable on macOS by way of the method not existing there. **LABEL: inferred**; the step is combining "OSL is supported only when rendering with the CPU or with the OptiX GPU Compute backend" with "OptiX is supported on Windows and Linux", both quoted above. The macOS-13 gate on MNEE (`ui.py` lines 197–205) is an operating-system difference *within* a single method.

---

### 7D. Does anything establish whether that changes with the size of the scene?

**LABEL: documented, in part. The sources establish that it does, but only loosely, and only for some methods.**

Here is everything in the sources that ties the choice of method to how big the scene is. There are five statements and no more; I searched hard for others and list the searches at the end.

**1. The strongest one — memory overflow behaviour differs by method, and only a large enough scene reaches it.**

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
> try to use system memory."

— manual, GPU Rendering page, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html, read 2026-07-30.

**LABEL: inferred** for the consequence. The step: a scene that fits in card memory never triggers this, so for such a scene the four named methods and oneAPI behave alike in this respect; a scene that does not fit does trigger it. So the difference between the methods here appears only above a size threshold. The sources do not name the threshold — it is whatever the card holds.

**2. Speed differences are explicitly scene-dependent.**

> "Different technologies also have different compute times
> depending on the scene tested."

— manual, GPU Rendering page, address above, read 2026-07-30.

**LABEL: documented.** This says the ranking between methods is not fixed across scenes. Note carefully: it says "scene tested", not "scene size". The manual does not say the variation is *by size* rather than by content.

**3. MetalRT's benefit is stated in terms of what the scene contains.**

> "MetalRT for ray tracing uses less memory for scenes which use curves extensively,
> and can give better performance in specific cases."

— manual, System preferences page, https://docs.blender.org/manual/en/5.1/editors/preferences/system.html, read 2026-07-30. Identical wording in Cycles' own files, `cycles_5.1/properties.py`, lines 1616–1617 (**observed**).

**LABEL: documented.** Again this is about scene *content* (curves), not scene size as such.

**4. Metal's Kernel Optimization is described as depending on scene content.**

> "Kernels can be optimized based on scene content."

— Cycles' own files, `cycles_5.1/properties.py`, line 1640. **LABEL: observed.**

**5. Memory pooling exists specifically for large scenes.**

> "Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it"

— Cycles' own files, `cycles_5.1/properties.py`, line 1610 (the `peer_memory` description). **LABEL: observed.** The manual's version: "effectively freeing up space for larger scenes" (System preferences page, address above).

**And two general statements about GPU rendering and scene size, which are not about the choice between methods but bear on it:**

> "On the other hand, they also have some limitations in rendering complex scenes, due to more limited memory,
> and issues with interactivity when using the same graphics card for display and rendering."

> "Heavy scenes can also make Blender unresponsive on newer GPUs,
> when using a lot of memory or executing expensive shaders, however this is generally less of a problem."

— manual, GPU Rendering page, address above, read 2026-07-30.

**What the sources do not establish.** They give no threshold, no measurement, no scene-size figure, and no statement of the form "method X is better below size N and method Y above it". Every scene-dependence statement above is qualitative. Nothing in the sources lets anyone predict, for a given scene, which method will be faster or which will fit. The manual says so itself, in the passage quoted at 7C-vii, and points outward.

**Not covered.** Whether "size of the scene" means memory footprint, polygon count, or render resolution — the sources use "large", "heavy" and "complex" interchangeably and define none of them. Whether the *right* method changes with scene size in any systematic way. Whether Blender ever switches method by itself based on scene size (I found no such statement; "Auto" for MetalRT says "Automatically pick the fastest intersection method" without saying on what basis).

**Outside the interface.** Everything in this part is about what happens during a render, not about where the render was started, so nothing here is interface-only. One caveat with a bearing on running renders from outside: the memory-overflow fallback and the "GPU can only either render or draw the user interface" problem both involve the card also driving a display; the sources do not say what changes when there is no display to draw.

**Operating system.** No. Two of the five statements are Metal-specific and therefore Apple-Silicon-only; one (NVLink pooling) is NVIDIA-only and so Windows-and-Linux-only. Only statements 1 and 2 span more than one operating system.

---

### 7E. The other reading — the display backend

Included briefly because the question could mean this, and leaving it out would be hiding a source.

**LABEL: documented.**

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

— manual, System preferences page, https://docs.blender.org/manual/en/5.1/editors/preferences/system.html, read 2026-07-30.

The API documentation lists three, not two:

> "gpu_backend
>
> GPU backend to use (requires restarting Blender for changes to take effect) (default 'OPENGL')
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
> Literal['OPENGL', 'METAL', 'VULKAN']"

— API documentation, https://docs.blender.org/api/5.1/bpy.types.PreferencesSystem.html, read 2026-07-30.

It can be set at launch:

> "--gpu-backend
>
> Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."

— manual, Command Line Arguments page, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30.

**Does this one change with scene size?** The 5.1 release notes carry measurements, per backend, on a named scene — which is the closest thing to a scene-dependent figure anywhere in my sources:

> "| Version | Backend | Scene               | Cold  | Warm |
> | 5.0.1   | OpenGL  | barbershop_interior | 44.19 | 5.68 |
> | 5.1     | OpenGL  | barbershop_interior | 30.39 | 6.51 |
> | 5.0.1   | Vulkan  | barbershop_interior | 24.87 | 5.22 |
> | 5.1     | Vulkan  | barbershop_interior | 11.67 | 5.93 |
> | 5.0.1   | Metal   | barbershop_interior | 55.50 | 1.18 |
> | 5.1     | Metal   | barbershop_interior | 37.20 | 2.72 |"

with "Cold means without shader caching. Warm means with shader caching." and a second table:

> "| Version | Backend | Texture pool size |
> | 5.0.1   | Either  | 406mb             |
> | 5.1	  | OpenGL  | 291mb             |
> | 5.1	  | Vulkan  | 245mb             |"

— release notes, https://developer.blender.org/docs/release_notes/5.1/eevee/, read 2026-07-30. The page states the hardware: "Tests have been conducted on NVIDIA RTX 6000 Ada, with NVIDIA 580.126.09 drivers on a AMD Ryzen 9 7950X 16-Core Processor. For Metal, tests have been conducted on a Apple M1 Ultra on macOS Tahoe 26.2." The texture-pool table is on "the *Mr. Elephant* scene" on an "NVIDIA RTX A6000".

**Not covered.** These figures are for material compilation time and texture pool size, not for render time, and they are single-scene. They do not establish how either quantity moves with scene size. There is only one scene per table.

**Outside the interface.** `--gpu-backend` is a launch option, so this choice can be made from outside. Whether the display backend matters at all when Blender has no window is **not in the sources** — the setting is described as being for "drawing the interface and rendering display content", and the sources do not say what that means when there is no interface. I searched for this and found nothing; see the record below.

**Operating system.** No. Metal appears only in the API documentation's list, not in the manual's, and Metal is stated elsewhere to be an Apple technology. Also: "Shader Compilation Method … This option is not available on macOS and requires using OpenGL backend." (manual, System preferences page, address above, read 2026-07-30).

---


---

## Tested by hand — what a running Blender showed

**Label: observed.** Run on 2026-08-30 by Claude Opus 5 in Claude Code, on the two computers described below, in **Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.1**. The same tests were run in all four versions, so that a gap in one is not read as a difference in Blender. Blender 5.1's own figures are given here in full; where the four versions agreed, that is said.

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

Blender 5.1 was started with no window and `--factory-startup` on each machine, and asked what the Cycles preference offers.

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

On both machines, in Blender 5.1, the Cycles preference object carries exactly eight properties:

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

Four crossings were run in Blender 5.1 on both machines, each with a scratch configuration folder holding a saved preference and a `.blend` saved with the scene's device one way or the other. Which method actually ran was read from two things Blender prints: `Building OptiX acceleration structure`, which only an OptiX render prints, and `Profiling information not available (only works with CPU rendering)`, which only a card render prints.

On the rented Linux machine: with the stored technology at `NONE` and the scene's device at `CPU`, adding `-- --cycles-device OPTIX` produced the OptiX lines, the card-only line, and a render in under a second against about six seconds for the processor. With the stored technology at `OPTIX` and the scene's device at `GPU`, adding `-- --cycles-device CPU` produced neither line and about six seconds. The Mac gave the same shape with `METAL` in place of `OPTIX`.

**So the command-line option beats both the stored preference and the scene's own setting, in both directions, on both operating systems.** All four versions behaved the same way.

### 4. What happens when the named method is one the machine cannot do, or is not a method at all

In Blender 5.1, on the rented Linux machine (no Apple, no AMD, no Intel graphics), `-- --cycles-device METAL`, `HIP` and `ONEAPI` were each asked for; on the Mac (no NVIDIA, no AMD, no Intel graphics), `CUDA`, `OPTIX` and `HIP`. On both, `-- --cycles-device BANANA` was also asked for.

**Every one of them ended with exit status 1 and wrote nothing to disk.** For a real method the machine has no hardware for, Blender 5.1 printed:

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

**The control first, because without it this finding carries nothing.** The same scene was rendered **twice with the same method** and the two compared. On the processor, in Blender 5.1 on both machines, the two runs are **identical to the pixel** — not one channel differs. On a card they are not identical, but the largest difference between two runs of the same method is 4.17 × 10⁻⁷ to 8.34 × 10⁻⁷ on the rented Linux machine and 2.98 × 10⁻⁷ to 4.77 × 10⁻⁷ on the Mac.

Against that floor, in Blender 5.1:

| pair | small scene | heavy mesh | heavy curves |
|---|---|---|---|
| processor against CUDA (Linux) | 0.0121 | 0.0185 | 0.1212 |
| processor against OptiX (Linux) | 0.0102 | 0.0185 | 0.1177 |
| CUDA against OptiX (Linux) | 0.0121 | 0.0133 | 0.0431 |
| processor against Metal (Mac) | 0.0103 | 0.0185 | 0.1199 |
| MetalRT off against on (Mac) | 0.0103 | 0.0205 | 0.1425 |
| MetalRT off against auto (Mac) | 4.77 × 10⁻⁷ | 4.77 × 10⁻⁷ | 4.77 × 10⁻⁷ |

Figures are the largest difference in any colour channel; between 224,257 and 230,015 of the 230,400 pixels differ on the mesh scenes and about 149,000 to 162,500 on the curve scene. **Every pair of different methods differs by three to five orders of magnitude more than two runs of the same method do.** The one pair that sits at the noise floor is MetalRT off against auto, which is finding 6.

*(Citation check, 2026-08-30, repeated on the Creator's Mac in 5.1.2: the processor half of the control reproduced exactly — two CPU renders identical to the pixel on all three scenes — and the Mac card control came back at 7.15 × 10⁻⁷, 4.77 × 10⁻⁷ and 3.58 × 10⁻⁷, the same order as recorded. The first five rows reproduced to four decimal places: processor against Metal 0.010348, 0.018478 and 0.119888; MetalRT off against on 0.010348, 0.020503 and 0.142475. **The last row did not.** Off against auto came back at 7.15 × 10⁻⁷ on the small scene but **0.012499 on the heavy mesh scene and 0.018828 on the curve scene** — four and a half orders of magnitude above what is recorded, and well above the floor the same run measured. What Blender printed while it built was unchanged: Auto still built the same structure as Off on all three scenes, so it is the pixel row that disagrees and not finding 6. Both runs stand; neither has been chosen between, and the original figures above are untouched. The disagreement is on the hand-test list to be run again.)*


*(Run again on 2026-08-30, and the control was measured properly this time: see **The noise floor measured, and three rows run again** below. Two runs of the same method were compared 120 times over. **On the processor all sixty pairs are identical to the pixel. On Metal the floor is 10⁻⁷ most of the time and not all of the time** — two card pairs in sixty came back at 5.31 × 10⁻⁵ and 0.0182. The rows above stay comfortably above the floor on the small and heavy mesh scenes and are only about six to eight times the worst case on the curve scene. Nothing above was changed.)*

### 6. What the choice changes depends on the size of the scene, and it changes direction

On the rented Linux machine, Blender 5.1, three repeats of every measurement with warm kernel caches. The three repeats agreed to within a few hundredths of a second throughout.

| scene | processor | CUDA | OptiX | card memory: CUDA / OptiX |
|---|---|---|---|---|
| small, 322 triangles | 00:05.98 / 00:06.09 / 00:05.97 | 00:00.64 / 00:00.68 / 00:00.62 | 00:00.70 / 00:00.67 / 00:00.68 | 1477 / 1493 MiB |
| heavy mesh, 10,485,762 triangles | 00:11.03 / 00:11.04 / 00:11.00 | 00:08.44 / 00:08.53 / 00:08.52 | 00:01.98 / 00:01.96 / 00:01.96 | 2101 / 2043 MiB |
| heavy curves, 300,000 curves | 00:14.93 / 00:14.93 / 00:15.03 | 00:06.67 / 00:06.67 / 00:06.67 | 00:01.32 / 00:01.30 / 00:01.33 | 1823 / 1599 MiB |

**On the 322-triangle scene CUDA is marginally ahead of OptiX. On the 10.5-million-triangle scene OptiX is about four times faster.** Same card, same driver, same file, same settings, same version. The card's own memory moves the same way: on the small scene OptiX takes slightly more than CUDA, on the heavy mesh scene rather less, and on the curve scene least of all. All four versions showed the same shape.

**A negative result was available and did not happen.** Had the ratio of one method's time to the other's been the same at both sizes, within the spread of the three repeats, the answer would have been that the effect does not depend on the size of the scene.

### 7. MetalRT, and what "Auto" does

The Creator's Mac only. MetalRT is a preference rather than a scene setting, so it was set from a script at start-up before the frame was rendered, and the script printed what it asked for and what the value became, so a run where the setting did not apply cannot be mistaken for one where it made no difference.

**What Blender prints while it builds separates the three settings cleanly, and identically in all four versions.** With MetalRT `ON` it prints `Building TLAS` and `Building mesh BLAS`, and on the curve scene `Building hair BLAS` as well. With `OFF` and with `AUTO` it prints `Packing BVH nodes` and `Packing BVH triangles and strands`. **On all three scenes, in all four versions, Auto built the same structure as Off.**

Cycles' own reported peak memory in Blender 5.1, which the shared Mac does not affect:

| scene | MetalRT Off | MetalRT On | MetalRT Auto |
|---|---|---|---|
| small | 1388 M | 1388 M | 1388 M |
| heavy mesh | 1993 M | 2448 M | 1993 M |
| heavy curves | 1718 M | 1683 M | 1718 M |

**At 322 triangles the switch changes nothing measurable. On the mesh scene it costs about 455 MB more. On the curve scene it saves about 34 MB.** That is the same direction for curves in all four versions and the opposite direction on a mesh scene of comparable weight, and it appears only once the scene is large enough for it to.

Times in Blender 5.1, fastest of three repeats on a shared Mac: on the heavy mesh scene, Off 00:05.75, On 00:03.39, Auto 00:05.42; on the curve scene, Off 00:05.89, On 00:06.16, Auto 00:05.85; on the small scene, Off 00:02.14, On 00:01.72, Auto 00:01.69.

### 8. Kernel Optimization

The Creator's Mac only, the heavy mesh scene, Blender 5.1, set from a script and confirmed to have taken. Off 00:05.74, Intersection only 00:05.23, Full 00:05.33. Off was the slowest in all four versions. On the pictures: all three pairs stayed at 4.77 × 10⁻⁷, at the noise floor.

*(Citation check, 2026-08-30, repeated on the Creator's Mac in 5.1.2: the times agreed in shape — Off slowest, at 00:05.86 against Intersection only 00:05.55 and Full 00:04.93 on a machine this document declares shared. **The pictures did not agree.** `FULL` against `OFF` came back at **0.012499**, which is the figure the other three versions record as above the noise, not the 4.77 × 10⁻⁷ recorded here; `INTERSECT` against `OFF` came back at 5.96 × 10⁻⁷, at the floor as recorded. Both runs stand; neither has been chosen between, and the original figures above are untouched. The disagreement is on the hand-test list to be run again.)*


*(Run again on 2026-08-30, three renders of each setting and every pair of the nine compared: see **The noise floor measured, and three rows run again** below. Nothing above was changed.)*

### 9. A scene the machine cannot hold, and where the choice decides whether the render happens at all

The huge mesh scene — **125,829,122 triangles**, a 9.3 GB file — on the rented Linux machine, in Blender 5.1:

```text
OptiX: exit status 0, 30.38 seconds of wall time, card memory peaked at 7765 MiB of 10,240, host memory peaked at 24,566,472,704 bytes, render time 00:15.87, frame written
CUDA:  exit status 137, 82.56 seconds of wall time, card memory peaked at 1775 MiB of 10,240, host memory peaked at 32,823,898,112 bytes, no time line was reached, no frame written
```

**The OptiX run finished and wrote its frame. The CUDA run was killed by the system for want of host memory** — status 137, the last line printed being `Updating Scene BVH | Building BVH` at somewhere between 45% and 76%, with the container's memory at its ceiling of about 32.9 GB. The same happened in all four versions: every OptiX run completed, every CUDA run was killed.

**What this does not say.** The condition reached was the host's memory ceiling, not the card's. OptiX peaked at between 7,765 and 8,353 MiB of the card's 10,240 MiB and CUDA never got past 1,775 MiB before it died, so **nothing here bears on what happens when a card's own memory fills**, and the documented fallback to system memory was not exercised.

### 10. The Acceleration Structure panel

Blender 5.1 was started with no window on both machines and the panel itself was asked, for every crossing of the technology, the scene's device setting, and whether the processor is ticked in the device list, whether it would be drawn. The panel class `CYCLES_RENDER_PT_performance_acceleration_structure` exists in Blender 5.1 on both machines.

On the rented Linux machine the answer was `True` under every crossing except one:

```text
pref=OPTIX  scene_device=GPU cpu_device_ticked=False -> Acceleration Structure panel poll = False
pref=OPTIX  scene_device=GPU cpu_device_ticked=True  -> Acceleration Structure panel poll = True
```

So the panel goes away only when OptiX is chosen **and** the scene is set to the card **and** the processor is not ticked alongside it. Under `NONE`, `CUDA`, `HIP` and `ONEAPI` it is always present. On the Mac, under `NONE` and `METAL`, it is always present.

**The four versions answered identically.** That matters here: three of the four versions' documents quote a manual line about this panel and one does not, and this test shows that silence to be a gap in the documentation rather than a difference in Blender. One version difference showed up beside it and is not about this panel: 5.2.1 carries a `CYCLES_RENDER_PT_performance_texture_cache` panel that this version does not.

### 11. What travels with the file, and what a stored preference alone does

In Blender 5.1, on both machines: a `.blend` saved with the scene's own device set to `GPU`, opened with a configuration whose technology was set one way and then another, read back:

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

Blender 5.1, both machines, `--gpu-backend` given each of its three documented values on a background render.

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

`blender --help` in Blender 5.1, on both machines, lists under **Cycles Render Options**:

```text
--cycles-print-stats
	Log statistics about render memory and time usage.
```

Every run above that used it printed a `Render statistics:` block with per-object geometry memory, and on a card render the line `Profiling information not available (only works with CPU rendering)`.

### The noise floor measured, and three rows run again — 2026-08-30

**Label: observed.** Run on 2026-08-30 by Claude Opus 5 in Claude Code, **on the Creator's Mac and on no other computer**, in Blender 5.1.2 alongside the other three researched versions. These are entries 26 to 29 of `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md`, sent there by that topic's citation check on the same day because its own repeats of the tests above came back different in four places.

**What this can and cannot settle.** The Creator's Mac has **Apple's own graphics and no NVIDIA card**, and the rented Austrian machine behind the CUDA and OptiX figures above was destroyed on the day those tests were run. **So everything in this section settles macOS and Apple's own graphics, and says nothing whatever about an NVIDIA card.** Nothing was rented for it. Other agents had been running Blender on the same Mac all day, so **no time in this section is a clean one**; all four tests are about pixels and exit statuses rather than speed. Every one of about 900 Blender launches ran with a scratch configuration folder, so nothing of the Creator's own settings was read or written.

**Why it was worth running at all.** Finding 5 above rests entirely on a control: two runs of the *same* method should come back far closer together than two runs of *different* methods. Nobody had measured how wide that same-method spread really is — one pair had been run, not a spread — and **an answer whose noise floor is unknown cannot be weighed.**

#### The floor itself, over sixty pairs on the card and sixty on the processor

**What was done.** Each of the small, heavy mesh and heavy curve scenes was rendered **twice with the same method** — once on the processor, once on Metal — and the two compared by their pixels, in all four versions. **The whole set was repeated five times over**: 240 renders and 120 pairs, of which 15 card pairs and 15 processor pairs are Blender 5.1's.

**What a negative result would have looked like:** every Metal pair at 10⁻⁷ every time, which would have confirmed the floor exactly as recorded above.

**The processor. All sixty pairs across the four versions are identical to the pixel** — not one channel of 921,600 differs — and Blender 5.1's fifteen are among them; every one printed `VERDICT: IDENTICAL to the pixel`.

**The card, in Blender 5.1, the five repeats in order:**

| scene | two runs of Metal, repeats 1 to 5 |
|---|---|
| small | 4.77 × 10⁻⁷ on all five |
| heavy mesh | 4.77 × 10⁻⁷ on all five |
| heavy curves | 2.98 × 10⁻⁷, 4.17 × 10⁻⁷, 3.58 × 10⁻⁷, 2.98 × 10⁻⁷, 3.58 × 10⁻⁷ |

**Not one of this version's fifteen card pairs left the 10⁻⁷ range.** Both excursions in the set of sixty were in other versions.

**Across all four versions, fifty-eight of the sixty card pairs sit between 2.98 × 10⁻⁷ and 5.96 × 10⁻⁷.** The two that do not are 5.310773849487305 × 10⁻⁵, in Blender 5.0.1 on the heavy mesh scene, and 0.018208861351013184, in Blender 4.5.12 on the curve scene. **Both were the first repeat of the set.** So **the floor on this machine is not one number**: it is 10⁻⁷ most of the time, and about one card pair in thirty came back four to five orders of magnitude higher.

#### What that makes of the differences recorded above

Only the Mac rows of finding 5 are touched here. The Linux rows are outside what this machine can say anything about.

| row, as finding 5 records it for Blender 5.1 | small | heavy mesh | heavy curves |
|---|---|---|---|
| worst two-runs-of-Metal figure measured here, this version | 4.77 × 10⁻⁷ | 4.77 × 10⁻⁷ | 4.17 × 10⁻⁷ |
| worst two-runs-of-Metal figure measured here, all four versions | 4.77 × 10⁻⁷ | 5.31 × 10⁻⁵ | **0.0182** |
| processor against Metal | 0.0103 | 0.0185 | 0.1199 |
| MetalRT off against on | 0.0103 | 0.0205 | 0.1425 |

**On the small scene and the heavy mesh scene those recorded differences sit comfortably above the floor** — from about 350 times it up to tens of thousands of times it. **On the curve scene they do not sit comfortably above it.** The worst same-method figure this machine produced on that scene, in any version, is 0.0182, and the recorded curve-scene differences of 0.1199 and 0.1425 are only about six to eight times that. They are still above it, and nineteen of the twenty curve-scene pairs were at 10⁻⁷ — but *orders of magnitude* is the wrong description of the curve-scene rows, and *a factor of six to eight against the worst case* is the right one.

**Both runs stand.** Finding 5's own control figures above are untouched, this run's are here, and neither has been chosen over the other.

#### MetalRT Auto against MetalRT Off, three hundred comparisons

**What was done.** Each of the three scenes was rendered **five times with MetalRT off and five times with MetalRT auto**, in all four versions, and **every off was compared against every auto** — 25 comparisons per scene per version, 300 in all, of which 75 are Blender 5.1's.

**What a negative result would have looked like:** every pair at the noise floor every time. That is what came back, and it could have gone the other way — the same procedure produced figures of 0.012 and 0.018 twice before, once in the record above and once in the citation check's repeat.

**Every one of the 120 renders exited 0 and printed the setting it had been given**, sixty reading off and sixty reading auto. **And what Blender printed while it built was the same line in all 120**, in every version and on every scene:

```text
Packing BVH nodes; Packing BVH triangles and strands
```

So **auto built the same structure as off in all 120 runs**, which is what finding 7 above already says.

**The pixels, in Blender 5.1, as the range over the twenty-five comparisons in each cell:**

| scene | MetalRT off against auto, twenty-five comparisons |
|---|---|
| small | 3.58 × 10⁻⁷ to 5.96 × 10⁻⁷ |
| heavy mesh | 3.58 × 10⁻⁷ to 5.96 × 10⁻⁷ |
| heavy curves | 2.98 × 10⁻⁷ to 3.58 × 10⁻⁷ |

Across all four versions **not one of the 300 comparisons is above 7.15 × 10⁻⁷**. **That is this version's own recorded answer** — 4.77 × 10⁻⁷ on all three scenes — and it is the only one of the four versions' four different recorded answers that this run matches. **It is not what the citation check got here**: its repeat in this version gave 0.012499 on the heavy mesh scene and 0.018828 on the curve scene. Neither figure came back in twenty-five tries on each of those two scenes.

**Both runs stand.** Finding 5's off-against-auto row for Blender 5.1 — 4.77 × 10⁻⁷, 4.77 × 10⁻⁷ and 4.77 × 10⁻⁷ — is untouched, and so is the citation check's note beside it.

**One plain thing about the numbers, offered as arithmetic and not as an answer.** The figures recorded for that row which are above the floor are the same size as the two same-method excursions measured above, and two of them fall on the same scene in the same version as an excursion. **Nothing run here establishes what that means**, and this section does not say which reading is the right one.

#### Kernel Optimization, thirty-six renders and a hundred and forty-four comparisons

**What was done.** The heavy mesh scene rendered with Kernel Optimization off, intersection only, and full — **three times each**, in all four versions — and **every pair of the nine images in a version compared**, which is 36 pairs a version. The nine include three same-setting pairs, so the floor for this scene is measured inside the same test.

**What a negative result would have looked like:** all pairs at the noise floor, which is what Blender 5.1's own record says and what the other three versions' records deny.

**All 36 renders exited 0 and every one printed the setting it had been given. In Blender 5.1:**

| pair | range over the nine, or the three |
|---|---|
| off against full | 4.77 × 10⁻⁷ to 5.96 × 10⁻⁷ |
| off against intersection only | 4.77 × 10⁻⁷ to 7.15 × 10⁻⁷ |
| full against intersection only | 4.77 × 10⁻⁷ to 5.96 × 10⁻⁷ |
| two runs of the same setting | 4.77 × 10⁻⁷ to 5.96 × 10⁻⁷ |

**This is the version the entry was written for, and it came back on this version's own side.** 4.5, 5.0 and 5.2 all record 0.0125 for Full against Off and all three gave 0.0125 again in this run; **this version gave the noise floor in all nine pairs, which is what its own record says.** The citation check's repeat in this version gave 0.012499 and did not come back.

**Across the four versions the split is clean and it is the same split the records above have**: 4.5.12, 5.0.1 and 5.2.1 gave 0.0125 for off against full in all nine pairs each, and 5.1.2 gave the noise floor in all nine.

**Times, on a machine that was not quiet**, as Cycles' own `Time:` line for the three runs of each setting in Blender 5.1: off 00:05.73, 00:05.59, 00:05.61; intersection only 00:05.26, 00:05.30, 00:05.29; full 00:05.33, 00:05.57, 00:05.56. **Off was the slowest in all four versions**, which is the shape both earlier runs recorded and the one part of this test nobody disputes.

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

## Record of where I looked

Everything below was run against the folders I was given. All searches were case-insensitive over whole folders, not over single files.

**Searches I ran across the whole 5.1 manual, API documentation, release notes and Cycles files:**

- `optix`, `cuda`, `oneapi`, `compute device` — across the whole manual. Eight text files matched (plus three unrelated SVG diagrams of the video sequencer, which I discarded on sight as false hits on the letters "hip"/"metal"). Those eight were: `advanced/command_line/arguments`, `editors/preferences/system`, `render/cycles/gpu_rendering`, `render/cycles/render_settings/index`, `render/cycles/osl/camera`, `render/cycles/render_settings/sampling`, `render/cycles/osl/index`, `render/shader_nodes/input/bevel`, `render/shader_nodes/input/ao`, and `getting_started/about/history`. I opened every one of them.
- `metalrt`, `hip rt`, `hiprt`, `kernel optimization`, `hardware ray`, `ray-tracing acceleration`, `bvh2` — across the whole manual. Only two text files: the System preferences page and the GPU Rendering page. Both already open.
- `optix|cuda|hip rt|oneapi|metalrt|metal|embree|hardware ray|backend` — across the whole 5.1 release notes folder. Matched `cycles.md`, `eevee.md`, `virtual_reality.md`, `bugfixes.md`. I read `cycles.md` in full and the relevant section of `eevee.md`. I ruled out `virtual_reality.md` (its Metal mention is an OpenXR graphics binding for VR, not a render method) and `bugfixes.md` (a list of fixed bug titles, none stating behaviour).
- `large scene|heavy scene|complex scene|scene content|scene size|depending on the scene|scene tested|scene complexity` — across the manual, the Cycles files and the release notes. This was the search for part D. I went through every hit. The ones that bore on the question are the five quoted in 7D. The ones I ruled out and why: video sequencer and compositor "Scene Size" settings (a node/strip input, unrelated); `3dview/navigate/local_view` and `walk_fly` and `troubleshooting/3d_view` (viewport navigation); `render/eevee/light_settings` and `light_probes/volume` and `eevee/limitations` (EEVEE, not a card method); `animation/drivers/introduction`, `modeling/modifiers/generate/array_legacy`, `scene_layout/*`, `physics/fluid/*`, `addons/import_export/scene_fbx` (nothing to do with GPU methods); `render/cycles/render_settings/simplify` (a scene setting, not a method); `render/cycles/render_settings/sampling` line 157 and the two matching lines in `properties.py` (the denoiser's Use GPU switch — related, quoted at 7C-vi, but not a choice between methods); `source/kernel/light/tree.h` line 382 (a source comment posing a question, not stating behaviour).
- `out of core|host memory|unified memory|system memory|spill` — across the manual, the Cycles Python files and the release notes. This was the second search for part D, aimed at the memory-overflow claim. The only relevant hits were the two lines already quoted from the GPU Rendering page. Everything else was the Color Spill compositor node and status-bar/interface memory readouts.
- `compute_device_type|metalrt|use_hiprt|CyclesPreferences` — across the whole 5.1 API documentation. Only three files matched, and all three matched on `legacy_compute_device_type`, a different and read-only setting ("For backwards compatibility only … readonly"). **This is how I established that the Cycles compute-device setting is not in the API documentation at all.** Cycles is an add-on, so its settings are not part of the documented `bpy.types` reference. Anything I say about that setting therefore comes from Cycles' own files, labelled observed.
- `addons[` — across the API documentation and the manual, to find how an add-on's settings are reached. Two hits: `bpy.types.AddonPreferences` and the manual's Add-ons page.
- `gpu-backend|cycles-device|gpu_backend|OPTIX|CUDA|HIP|METAL|VULKAN|oneAPI` — across the command-line arguments page. Found `--cycles-device` and `--gpu-backend`, and nothing else relevant. **This is how I established that MetalRT, HIP RT, Embree on GPU and Kernel Optimization have no command-line option.**
- `background mode|--background|factory-startup|enable-autoexec|userpref` — across the command-line arguments page, looking for whether saved preferences apply when Blender runs without its window. Nothing states either way. I then searched `userpref` across the whole manual: four files, all about where the file sits on disk or what a startup log line looks like. **This is the basis for the "not in the sources" in 7A's Outside-the-interface line.**
- `cycles-device|cycles_device|CYCLES_DEVICE` — across all Cycles files. One file, `engine.py`.
- `debug_use_|debug_bvh|use_optix_debug|hiprt|metalrt|embree` — across `cycles_5.1/properties.py`, to find every method-related setting rather than only the ones the manual names.
- `use_metal(|use_cuda(|use_hip(|use_optix(|use_oneapi(|use_cpu(|use_gpu(` — across `cycles_5.1/ui.py` and `properties.py`, to find every place the chosen method changes what Cycles offers. This produced 7C-i and 7C-ii.

**Files I opened in full or in substantial part:**

- Manual: `render/cycles/gpu_rendering.txt` (whole), `editors/preferences/system.txt` (whole), `render/cycles/render_settings/index.txt`, `render/cycles/render_settings/performance.txt`, `render/cycles/render_settings/sampling.txt` (relevant lines), `render/cycles/osl/index.txt` (limitations), `render/cycles/osl/camera.txt` (limitations), `render/shader_nodes/input/ao.txt`, `render/shader_nodes/input/bevel.txt`, `render/eevee/render_settings/raytracing.txt`, `advanced/command_line/arguments.txt` (relevant sections), `advanced/blender_directory_layout.txt`, `advanced/extensions/addons.txt`, `editors/preferences/introduction.txt` (preference-saving lines).
- Manual picture: `_images/editors_preferences_section_system.webp`.
- API documentation: `bpy.types.PreferencesSystem.txt` (relevant section).
- Release notes: `cycles.md` (whole), `eevee.md` (Performance section).
- Cycles' own files: `properties.py` (the whole `CyclesPreferences` class, lines 1558–1935; the device enums at lines 30–39 and 172–179; the scene `device` setting at 381–387; the debug settings at 914–939 and 1071–1099), `ui.py` (lines 90–205, 355–375, 880–935, 2200–2270, 2455–2475), `engine.py` (lines 1–60), `__init__.py` (lines 1–60), `source/kernel/bvh/bvh.h` (lines 1–80), and the directory listing of `source/kernel/device/`.

**What I checked and found absent:**

- Nothing in `cycles_5.1/source/` beyond `kernel/` and `util/` — there is no device-management or scene-management source in the folder I was given, so the logic behind MetalRT's "Auto", behind the memory fallback to system memory, and behind how a card is tested for a method, is not present and I could not read it.
- No page in the manual describes Kernel Optimization. It exists only in Cycles' own files.
- No page in the manual mentions that the Acceleration Structure panel is hidden under OptiX.
- No figures anywhere for the speed or memory of any Cycles method against any other.

---

## Things a running Blender could still settle

*(Pruned 2026-08-30 by the hand tests. Everything the earlier list held has either been run — the results are above, under **Tested by hand — what a running Blender showed** — or is recorded there as unable to be run, with its reason. What follows is what is genuinely still open.)*

1. **What happens when the card's own memory fills.** The scene of 125,829,122 triangles reached the host machine's memory ceiling and not the card's, so the documented fallback to system memory was never exercised. Settling it needs a machine whose card is small against what its host can build, which is the opposite of the usual want.
2. **What MetalRT's "Auto" weighs.** On the Creator's Mac it built the same structure as Off on all three scenes, including one of 300,000 curves — in the tests above, and again in sixty more runs on 2026-08-30, in all four versions, without one exception. Whether it ever picks the other way — on other Apple hardware, or on some other kind of scene — is still not settled.
3. **Whether the four card-free Windows tests give the same answers.** They are written out ready to run in `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md` and wait on the next Windows rental.
4. **Everything needing hardware nobody here has**: an AMD card, an Intel Arc card, two cards of different ray-tracing generations in one machine, NVLink-joined cards, a Mac that is not Apple silicon, or an Apple machine an agent may open a window on. Each is named above with its reason.

---

2026-08-30 — Claude Opus 5 (Claude Code) — What a render demands, question 7, Blender 5.1

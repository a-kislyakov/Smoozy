# What a render demands — Open Threads questions (Blender 5.1)

**Sources consulted, and when.** Blender 5.1 manual, Blender 5.1 Python API reference, Blender 5.1 release notes, and the Cycles add-on's own files for 5.1 — all as supplied inside `Blender Sources/`, all read on **2026-07-30** (the snapshot date given to me). I also opened one manual image, named below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Answering **question 7 only**.

---

## 7. Where a render can be told which of a graphics card's rendering methods to use, what methods are there, what does choosing one rather than another change about the render, and does anything establish whether that changes with the size of the scene?

### The reading I took, and the one I did not

The question is ambiguous. I answer the reading I believe is meant: **the choice of which of a graphics card's ways of doing the rendering work Blender should use** — the compute backend (CUDA, OptiX, HIP, oneAPI, Metal) and, within a backend, whether the card's dedicated ray-tracing hardware is used or a software traversal is used instead.

Two other readings exist, and I say plainly what they would be so nobody is misled:

- **Blender's drawing graphics API** — OpenGL versus Vulkan versus Metal. This is a graphics-card method too, and it is chosen in the same Preferences section, so I report it briefly at the end of the answer rather than leaving it out.
- **EEVEE's ray-tracing "Method"** — Light Probe versus Screen-Trace. That is a choice of rendering algorithm, not a choice between things the card offers, so I do not treat it as the question's subject. For the record it lives at `Render ‣ Raytracing ‣ Method` in the manual, `https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/raytracing.html`.

---

### 7a. Where the choice is made

There are three places, and they are not equal.

**(1) A user preference, not a scene setting.** The backend is chosen in Preferences, in the System section.

> "Cycles Render Devices
>
> Changes the Computing Device the Cycles render engine uses to render images. Cycles can use either the CPU or certain GPUs to render images, for more information see the GPU Rendering page."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/editors/preferences/system.html`
- Date read: 2026-07-30

The GPU Rendering page gives the same three-step instruction:

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`
- Date read: 2026-07-30

I opened the screenshot on the Preferences System page, `_images/editors_preferences_section_system.webp`, referenced in that page's text as `[image: _images/editors_preferences_section_system.webp]`. What is visible in it: a "Cycles Render Devices" panel whose whole content is one row of three buttons — **None**, **CUDA**, **HIP** — with None highlighted. Below it a "Display Graphics" panel with a single "Backend" dropdown reading "OpenGL". No MetalRT, HIP RT or Embree-on-GPU control appears anywhere in the picture.

- LABEL: **documented** (it is part of the manual)
- Source: manual image, on the page `https://docs.blender.org/manual/en/5.1/editors/preferences/system.html`
- Date read: 2026-07-30

Two steps a reader has to supply from that picture, and neither is visible in it. That the three buttons are a choice of one — that highlighting one clears the others — comes from the way the row is drawn, not from anything the picture states. And the absence of the other three controls is not by itself a statement about that machine: the picture has None selected, and the add-on's own code quoted under 7b puts each of those controls behind its own backend being chosen, so the selection explains the absence as well as the machine would.

- LABEL: **inferred** (both steps)
- Sources: the picture above, plus the code quoted under 7b
- Date read: 2026-07-30

What the picture shows and no more: on the machine that was screenshotted only three buttons were offered. That the offered list depends on the machine is a separate claim and I set out under 7b what carries it.

**(2) The scene picks CPU or GPU, but not which GPU method.** The scene-level control only says whether to use the backend already chosen in Preferences.

> "Device
>
> Specifies the compute device used for rendering in Cycles. The choice of device can significantly affect render performance, GPU Compute is often faster ."

> "GPU Compute:
>
> Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices. Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html`
- Date read: 2026-07-30

The Cycles files show the same, and show that only two values exist:

```
30: enum_devices = (
31:     ('CPU', "CPU", "Use CPU for rendering"),
32:     ('GPU', "GPU Compute",
33:         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
34: )
```

and

```
381:     device: EnumProperty(
382:         name="Device",
383:         description="Device to use for rendering",
384:         items=enum_devices,
385:         default='CPU',
386:         update=update_render_passes,
387:     )
```

- LABEL: **observed**
- Source: `Blender Sources/cycles_5.1/properties.py`, lines 30–34 and 381–387
- Date read: 2026-07-30

**(3) The command line, which the add-on's own help text says beats both.** The manual documents a Cycles command-line option:

> "--cycles-device <device>
>
> Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU.
>
> Example:
>
> blender -b file.blend -f 20 -- --cycles-device OPTIX"

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`
- Date read: 2026-07-30

The Cycles add-on's own help text for that option is more explicit about what it claims to beat:

```
16:     parser.add_argument("--cycles-device",
17:                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
18:                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
19:                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
20:                         default=None)
```

- LABEL: **observed**
- Source: `Blender Sources/cycles_5.1/engine.py`, lines 16–20
- Date read: 2026-07-30

What that quote establishes is that the option exists, under that name, with those valid values, and that the file's own help text describes it as overriding the preference and the scene setting. **That a render actually does override them is a step past the words**: a help string is what the file says the option is for, not a record of a render obeying it. The manual's own entry, quoted above, says only "Set the device used for rendering" and never mentions overriding. So the override itself is:

- LABEL: **inferred**. The step supplied: that the behaviour matches the help text.
- Sources: the two quotes above
- Date read: 2026-07-30

Note also that the valid-values list has **no** entry for the hardware-ray-tracing switches. `--cycles-device` names a backend and nothing finer.

---

### 7b. What the methods are

**The backends.** Five, plus "None".

> "Rendering Technologies
>
> Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer and operating system."

The page then carries one section per technology, and these are its section headings word for word: "CUDA – NVIDIA", "OptiX – NVIDIA", "HIP – AMD", "oneAPI – Intel", "Metal – Apple (macOS)".

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`
- Date read: 2026-07-30

The Preferences page lists the same set with "None" first:

> "None:
>
> When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/editors/preferences/system.html`
- Date read: 2026-07-30

The Cycles files hold a function that builds the list of backends, starting from "None" and adding each one behind its own test:

```
1581:     def get_device_types(self, context):
1582:         import _cycles
1583:         has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()
1584:
1585:         list = [('NONE', "None", n_("Do not use compute device"), 0)]
1586:         if has_cuda:
1587:             list.append(('CUDA', "CUDA", n_("Use CUDA for GPU acceleration"), 1))
1588:         if has_optix:
1589:             list.append(('OPTIX', "OptiX", n_("Use OptiX for GPU acceleration"), 3))
1590:         if has_hip:
1591:             list.append(('HIP', "HIP", n_("Use HIP for GPU acceleration"), 4))
1592:         if has_metal:
1593:             list.append(('METAL', "Metal", n_("Use Metal for GPU acceleration"), 5))
1594:         if has_oneapi:
1595:             list.append(('ONEAPI', "oneAPI", n_("Use oneAPI for GPU acceleration"), 6))
1596:
1597:         return list
```

- LABEL: **observed**
- Source: `Blender Sources/cycles_5.1/properties.py`, lines 1581–1597
- Date read: 2026-07-30

That the offered list therefore changes from machine to machine — and that this is why the screenshot showed only three buttons — is a step past the quote. The step supplied: that the six flags this function reads back are a report of what that build of Blender supports, and that this function is what fills the panel. *(Citation check, 2026-08-30: **this step was tested and it does not hold.** Finding 1 under "Tested by hand" records a machine with one NVIDIA card, no AMD card and no Intel graphics still offering HIP and oneAPI. The flags report what that build of Blender supports, not what the computer holds; what is empty is the device list underneath each choice. Corrected by this check.)*

- LABEL: **inferred**
- Sources: the quote above, plus the picture under 7a
- Date read: 2026-07-30

**Within a backend, the hardware-versus-software ray-tracing choice.** Three separate controls, one per backend family.

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

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/editors/preferences/system.html`
- Date read: 2026-07-30

The Cycles files give the same three controls with their kind, default and values:

```
1614:     metalrt: EnumProperty(
1615:         name="MetalRT",
1616:         description="MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better "
1617:                     "performance in specific cases",
1618:         default='AUTO',
1619:         items=(
1620:             ('OFF', "Off", "Disable MetalRT (uses BVH2 layout for intersection queries)"),
1621:             ('ON', "On", "Enable MetalRT for intersection queries"),
1622:             ('AUTO', "Auto", "Automatically pick the fastest intersection method"),
1623:         ),
1624:     )
1625:
1626:     use_hiprt: BoolProperty(
1627:         name="HIP RT",
1628:         description="HIP RT enables AMD hardware ray tracing on RDNA2 and above",
1629:         default=True,
1630:     )
1631:
1632:     use_oneapirt: BoolProperty(
1633:         name="Embree on GPU",
1634:         description="Embree on GPU enables the use of hardware ray tracing on Intel GPUs, providing better overall performance",
1635:         default=True,
1636:     )
```

- LABEL: **observed**
- Source: `Blender Sources/cycles_5.1/properties.py`, lines 1614–1636
- Date read: 2026-07-30

So: MetalRT is a three-way choice defaulting to **Auto**; HIP RT and Embree-on-GPU are simple on/off switches, both defaulting to **on**. There is **no** equivalent switch for OptiX or CUDA in this file. What follows from that is only that this file offers no such control on NVIDIA — **not** that choosing OptiX rather than CUDA is itself the hardware-ray-tracing choice. Nothing quoted here says what CUDA does or does not use, and I do not claim it.

A 5.1 release note confirms the HIP default:

> "For AMD GPUs, hardware ray-tracing is now enabled by default through HIP RT."

- LABEL: **documented**
- Source: release notes, `https://developer.blender.org/docs/release_notes/5.1/cycles/`
- Date read: 2026-07-30

The Cycles files also hold the code that lays out that part of the panel. Each of the three controls sits inside a test on which backend is chosen, and each is preceded by a line setting a row's `active` flag from a value gathered above it:

```
1908:         has_hardware_rt = has_enabled_hardware_rt and not has_disabled_hardware_rt
...
1915:         if compute_device_type == 'METAL':
1916:             import platform
1917:
1918:             # MetalRT only works on Apple Silicon.
1919:             if (platform.machine() == 'arm64'):
1920:                 col = layout.column()
1921:                 col.use_property_split = True
1922:                 col.prop(self, "kernel_optimization_level")
1923:                 row = col.row()
1924:                 row.active = has_hardware_rt
1925:                 row.prop(self, "metalrt")
1926:
1927:         if compute_device_type == 'HIP':
1928:             row = layout.row()
1929:             row.active = has_hardware_rt
1930:             row.prop(self, "use_hiprt")
1931:
1932:         elif compute_device_type == 'ONEAPI' and _cycles.with_embree_gpu:
1933:             row = layout.row()
1934:             row.active = has_hardware_rt
1935:             row.prop(self, "use_oneapirt")
```

There is also a comment one line above line 1908 stating the rule plainly: *(citation check, 2026-08-30: this read "two lines above"; the comment is at 1907, which is the line number this document itself gives, one line above 1908. Corrected by this check.)*

```
1907:         # Any device without RT support will disable it for all.
```

- LABEL: **observed**
- Source: `Blender Sources/cycles_5.1/properties.py`, lines 1907–1935
- Date read: 2026-07-30

Reading that as *each control is shown only when its own backend is chosen, and is greyed out unless every enabled device supports hardware ray tracing* takes two steps past the words. The steps supplied: that this function is what draws the Preferences panel, so a control inside a test appears only when that test passes; and that a row whose `active` flag is false is drawn greyed rather than hidden or ignored. The manual supports the first for two of the three in different words — "Only supported with oneAPI rendering devices" and "This feature is only available when using a HIP render device", quoted above — but says *supported*, not *shown*, and says nothing of the sort for MetalRT.

- LABEL: **inferred**
- Sources: the quote above, plus the two manual sentences quoted earlier in this section
- Date read: 2026-07-30

**A fourth setting, which the same code puts behind the Metal branch, and which is in no manual page I could find:** a "Kernel Optimization" setting.

```
1638:     kernel_optimization_level: EnumProperty(
1639:         name="Kernel Optimization",
1640:         description="Kernels can be optimized based on scene content. Optimized kernels are requested at the start of a render. "
1641:                     "If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set "
1642:                     "is available in the cache. This can result in additional CPU usage for a brief time (tens of seconds)",
1643:         default='FULL',
1644:         items=(
1645:             ('OFF', "Off", "Disable kernel optimization. Slowest rendering, no extra background CPU usage"),
1646:             ('INTERSECT', "Intersection only", "Optimize only intersection kernels. Faster rendering, negligible extra background CPU usage"),
1647:             ('FULL', "Full", "Optimize all kernels. Fastest rendering, may result in extra background CPU usage"),
1648:         ),
1649:     )
```

- LABEL: **observed**
- Source: `Blender Sources/cycles_5.1/properties.py`, lines 1638–1649
- Date read: 2026-07-30

I searched the whole 5.1 manual for "Kernel Optimization" and found no page carrying it.

**The methods are separate pieces of code.** One file in the Cycles kernel chooses which ray-traversal header to include, and the branch taken when none of the named hardware ones applies instead defines `__BVH2__`:

```
18: /* Device specific acceleration structures for ray tracing. */
19:
20: #if defined(__EMBREE__)
21: #  include "kernel/device/cpu/bvh.h"
22: #  define __BVH2__
23: #elif defined(__KERNEL_METALRT__)
24: #  include "kernel/device/metal/bvh.h"
25: #elif defined(__KERNEL_OPTIX__)
26: #  include "kernel/device/optix/bvh.h"
27: #elif defined(__KERNEL_HIPRT__)
28: #  include "kernel/device/hiprt/bvh.h"
29: #else
30: #  define __BVH2__
31: #endif
```

- LABEL: **observed**
- Source: `Blender Sources/cycles_5.1/source/kernel/bvh/bvh.h`, lines 18–31
- Date read: 2026-07-30

And the thing the kernel holds a handle to is a different type per method:

```
1319:   /* Device specific BVH. */
1320: #ifdef __KERNEL_OPTIX__
1321:   OptixTraversableHandle device_bvh;
1322: #elif defined __KERNEL_METALRT__
1323:   metalrt_as_type device_bvh;
1324: #elif defined(__KERNEL_HIPRT__)
1325:   void *device_bvh;
1326: #else
```

- LABEL: **observed**
- Source: `Blender Sources/cycles_5.1/source/kernel/types.h`, lines 1319–1326
- Date read: 2026-07-30

I take one step beyond the quotes here, and label it as such: because the file selects between including a hardware-ray-tracing traversal header and defining `__BVH2__`, **turning MetalRT, HIP RT or Embree-on-GPU off means the render traverses a software BVH2 structure instead of the card's ray-tracing units**. The step a reader must supply is that `__BVH2__` names the software path, which the manual's own wording — "Disable MetalRT (uses BVH2 layout for intersection queries)" — supports but the code does not state.

- LABEL: **inferred**
- Sources: the two code quotations above, plus the manual quote on the Preferences System page
- Date read: 2026-07-30

The backend kernel folders present in the supplied Cycles files are `cuda`, `gpu`, `hip`, `hiprt`, `metal`, `optix` (under `Blender Sources/cycles_5.1/source/kernel/device/`). — **observed**, same date.

---

### 7c. What choosing one rather than another changes about the render

The sources give several concrete differences. Nearly all are about speed, memory and which features work — **not** about the picture coming out different.

**Speed, and where it comes from.**

> "OptiX takes advantage of hardware ray-tracing acceleration in RTX graphics cards, for improved performance."

> "Hardware ray-tracing support is available with the most recent drivers. This can be enabled in the preferences, and is supported on Radeon RX 6000 and newer." (in the HIP section)

> "oneAPI is a computation library that is supported on Windows and Linux and requires an Intel® Arc™ graphics card with the Xe HPG architecture. Hardware acceleration for ray-tracing and denoising is supported."

> "GPU accelerated ray-tracing and denoising is available on Apple Silicon." (in the Metal section)

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`
- Date read: 2026-07-30

**Memory.** MetalRT's own description is the only place a method is tied to memory use:

> "MetalRT for ray tracing uses less memory for scenes which use curves extensively, and can give better performance in specific cases."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/editors/preferences/system.html`
- Date read: 2026-07-30

And separately, four of the five backends can spill to system memory when the card fills up:

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`
- Date read: 2026-07-30

oneAPI is absent from that list. I do not claim it therefore cannot spill; the source simply does not name it.

**Which features are supported.** Here the sources tie the choice to something other than speed and memory.

> "Limitations
>
> Path Guiding is not supported on any GPU.
>
> Open Shading Language is only supported for OptiX, with some limitations listed in the documentation."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`
- Date read: 2026-07-30

> "OSL is supported only when rendering with the CPU or with the OptiX GPU Compute backend."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html`
- Date read: 2026-07-30

And what still does not work even on OptiX:

> "Some OSL features are not available when using the OptiX backend. Examples include:
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
> The trace function is not functional. As a result of this, the Ambient Occlusion and Bevel nodes do not work."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/osl/index.html`
- Date read: 2026-07-30

I take one step from those last two items and label it as such: a node that "does not work" and noise functions that are "not available" are things a shader can be built on, so a scene using them would not come out the same on OptiX as on the processor. The step a reader must supply is that "does not work" and "not available" mean the shader renders differently rather than the render refusing to start. The page says neither.

- LABEL: **inferred**
- Sources: the OSL quotes above
- Date read: 2026-07-30

Held to what is actually written, the sources here state which features are supported, and no more than that.

**Which denoiser is available.** The denoiser is a separate setting, but its options depend on the backend chosen here:

> "OptiX:
>
> Uses NVIDIA's OptiX AI denoiser. Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.
>
> Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html`
- Date read: 2026-07-30

The GPU Rendering page adds the per-backend denoising acceleration lines: "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards"; "GPU accelerated denoising is available on discrete Radeon RX 6000 and newer"; "GPU accelerated denoising is available on all supported GPUs" (oneAPI section); "GPU accelerated ray-tracing and denoising is available on Apple Silicon" (Metal section). — **documented**, same page and date as above.

**What the sources do not say.** Nowhere in the 5.1 manual, API reference, release notes or the supplied Cycles files did I find a statement that a render produces a *different image* depending on which backend or which intersection method is used — no statement that they match, and no statement that they differ. The OSL limitations above are the exception, and they are stated as feature availability rather than as an image difference.

- LABEL: **not in the sources** (for the general "does the picture change" question)
- Where I looked: see the record below.

---

### 7d. Does anything establish whether this changes with the size of the scene?

**Short answer: no — nothing in these sources ties the effect of the choice to the size of the scene.** Four statements come close, and each stops short in a way worth naming.

**(i) The manual says the difference is scene-dependent, and refuses to say more.**

> "What renders faster?
>
> This varies depending on the hardware used. Different technologies also have different compute times depending on the scene tested. For the most up to date information on the performance of different devices, browse the Blender Open Data resource."

That establishes the difference between technologies varies **by scene**. It does not say the variation follows scene size, gives no numbers, and hands the question to an outside resource I am not permitted to consult.

- LABEL: **documented** (that it varies by scene); **not in the sources** (that it varies with scene *size*)
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`
- Date read: 2026-07-30

**(ii) MetalRT's benefit is tied to scene content, not scene size.** "uses less memory for scenes which use curves extensively" — a statement about what a scene contains, not how big it is. Quoted in full under 7b.

**(iii) GPUs are said to struggle with complex scenes, but this is about the card versus the CPU, not about which card method.**

> "On the other hand, they also have some limitations in rendering complex scenes, due to more limited memory, and issues with interactivity when using the same graphics card for display and rendering."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`
- Date read: 2026-07-30

**(iv) One setting in the same panel does name scene size, but it is a memory setting, not a method choice.**

> "Distribute Memory Across Devices
>
> Allocates resources across multiple GPUs rather than duplicating data, effectively freeing up space for larger scenes. Note that in order for this option to be available, the GPUs must be connected together with a high bandwidth communication protocol.
>
> Currently only NVLink on NVIDIA GPUs is supported."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/editors/preferences/system.html`
- Date read: 2026-07-30

The Cycles file's own wording for the same setting is more pointed about size:

```
1608:     peer_memory: BoolProperty(
1609:         name="Distribute memory across devices",
1610:         description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it",
1611:         default=False,
1612:     )
```

- LABEL: **observed**
- Source: `Blender Sources/cycles_5.1/properties.py`, lines 1608–1612
- Date read: 2026-07-30

But this changes how memory is spread across several cards. It is not a choice between the card's rendering methods, and it says nothing about how the method choice behaves as a scene grows.

**Two loose ends I could not close, and I say so rather than fill them in.**

- MetalRT's **Auto** setting says it will "Automatically pick the fastest intersection method". What it bases that pick on — scene size, scene content, the machine, or something else — is not stated anywhere I could find. The Cycles files supplied here contain only `source/kernel` and `source/util`; the host-side Metal device code that would make that decision is not among them. So I cannot say whether Auto's answer changes with the size of the scene.
- Metal's **Kernel Optimization** says "Kernels can be optimized based on scene content" — again content, not size, and the mechanism is not described.

- LABEL: **not in the sources**
- Where I looked: named in the record below.

---

### The other reading, reported briefly: the drawing graphics API

Chosen in the same Preferences section:

> "Display Graphics
>
> Settings that control how Blender draws its user interface and other display graphics. These options can influence performance and compatibility.
>
> Backend
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

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/editors/preferences/system.html`
- Date read: 2026-07-30

The manual page lists two options. The API reference lists three:

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

- LABEL: **documented**
- Source: Python API, `https://docs.blender.org/api/5.1/bpy.types.PreferencesSystem.html`
- Date read: 2026-07-30

And the command line offers all three:

> "--gpu-backend
>
> Force to use a specific GPU backend. Valid options: vulkan, metal, opengl."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`
- Date read: 2026-07-30

The 5.1 EEVEE release notes give per-backend numbers, and they are the only per-backend measurements anywhere in these sources. They are single-scene, so they establish nothing about scaling:

> "| 5.1     | OpenGL  | barbershop_interior | 30.39 | 6.51 |"
> "| 5.1     | Vulkan  | barbershop_interior | 11.67 | 5.93 |"
> "| 5.1     | Metal   | barbershop_interior | 37.20 | 2.72 |"

with the note "Cold means without shader caching. Warm means with shader caching." and a second table headed "Texture pool size" giving 291mb for OpenGL and 245mb for Vulkan on the *Mr. Elephant* scene.

- LABEL: **documented**
- Source: release notes, `https://developer.blender.org/docs/release_notes/5.1/eevee/`
- Date read: 2026-07-30

---

### Not covered

- Whether two backends, or hardware ray tracing on versus off, produce the **same pixels**. The sources are silent, either way.
- What MetalRT's **Auto** actually decides on, and whether that decision can change mid-project as a scene grows.
- Whether OptiX or CUDA has any equivalent hidden hardware-ray-tracing switch. The Cycles preferences file has none; whether one exists deeper in Blender's own code, which was not supplied, I cannot say.
- Any performance figure at all for Cycles backends. The manual explicitly declines to give one.
- Whether oneAPI spills to system memory when the card fills. It is missing from the list of four that do; that is absence, not denial.
- Whether the hardware-ray-tracing switches survive into a saved `.blend` file. They are preferences, and I set out below what that means, but I found no statement about what a `.blend` records of them.
- The Performance panel's own acceleration-structure options (Use Spatial Splits, Use Curves BVH, BVH Time Steps, Use Compact BVH, at `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html`). These shape the software structure rather than choose between the card's methods, so I did not treat them as part of this question. Flagging them because a different reading of "rendering methods" would pull them in.

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

### Outside the interface

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Four things that were open here are settled by test, in all four versions and on both computers. **`--cycles-device` does outrank both the stored preference and the scene's own setting**, in both directions. **A stored preference is applied by a Blender started with no window**, but only puts the render on the card when the technology's own device is ticked in the preference *and* the scene is set to the card; a preference naming the technology with no device ticked renders on the processor. **All six Cycles preference settings can be read and set from a script with no window** — the technology, MetalRT, HIP RT, Embree on GPU, Kernel Optimization and Distribute Memory Across Devices — and MetalRT and Kernel Optimization were then shown to change the render, so the assignment is not merely accepted. **A method the machine cannot do, or a name that is not a method, ends the run** with exit status 1 and no file written; nothing falls back. What is unchanged: there is still no command-line option for the three hardware ray-tracing switches.]*

This is where the answer splits, and the split matters.

**The backend can be named from outside the window, and only one way.** `--cycles-device` is a command-line option and the manual's own example is a background render: `blender -b file.blend -f 20 -- --cycles-device OPTIX`. That the option exists, and its list of valid values, is **documented** (the manual) and **observed** (the add-on file), both quoted under 7a. That it *overrides* the preference and the scene setting rests on the add-on's own help string describing it that way, so it is **inferred**, as set out under 7a — the manual's entry says only "Set the device used for rendering".

**The hardware-ray-tracing method cannot.** There is no command-line option for MetalRT, HIP RT, Embree-on-GPU or Kernel Optimization. I searched the whole command-line arguments page for each name and for "hardware", "ray", "rt" — the only GPU-related switches are `--cycles-device`, `--cycles-print-stats`, `--gpu-backend`, `--gpu-vsync`, `--gpu-compilation-subprocesses`, `--profile-gpu`. I also read the Environment Variables section of that page in full; it lists only `BLENDER_USER_*`, `BLENDER_SYSTEM_*`, `BLENDER_CUSTOM_SPLASH`, `BLENDER_CUSTOM_SPLASH_BANNER`, `BLENDER_OCIO`, `SPNAV_SOCKET`, `TEMP`, `TMPDIR` — nothing for Cycles devices. (The only Cycles environment variable named anywhere is `CYCLES_CUDA_EXTRA_CFLAGS`, and the manual gives it solely as a workaround for an "Unsupported GNU version" compiler error.)

- LABEL: **not in the sources** — no documented outside-the-window route to these three switches.

What these switches *are* is preferences, and preferences live in a separate file from the scene:

> "./config/userpref.blend
>
> User preferences."

- LABEL: **documented**
- Source: manual, `https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html`
- Date read: 2026-07-30

One step further, and I mark it as a step: since these are add-on preferences and Blender can run a Python script or expression at startup ("-P, --python <filepath> — Run the given Python script file"; "--python-expr <expression> — Run the given expression as a Python script", manual command-line arguments page), a headless run could in principle set them by script. I did not find that written down anywhere, and I did not run it.

- LABEL: **inferred**. The step supplied: that the Cycles preference class quoted above is reachable from Python at startup. The sources show the properties exist and show that `--python`/`--python-expr` exist; they never join the two.

Two further things bear on running outside the window. Rebuilding the machine's preferences matters, because the backend list is built behind per-backend tests — that is the code quoted under 7b, and reading it as *a machine with no CUDA device will not offer CUDA* is the **inferred** step recorded there, not something the manual states. And `--factory-startup` is documented only as "Skip reading the startup.blend in the users home directory" (manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`, read 2026-07-30), which says nothing about preferences either way.

**Operating system**

*[Brought up to date by the hand tests of 2026-08-30, written in full above under **Tested by hand — what a running Blender showed**. Tested rather than inferred: the Creator's Mac offers only `None` and `Metal` and starts at `Metal`; a rented Linux machine with one NVIDIA card offers `None`, `CUDA`, `OptiX`, `HIP` and `oneAPI` and starts at `None`. So the list of methods does differ by operating system — **but it is built from what that build supports rather than from the cards present**, since the Linux machine offered HIP and oneAPI with no AMD and no Intel graphics in it. `--gpu-backend` accepts `opengl` and `vulkan` on Linux and only `metal` on macOS, naming the rejected value in an error and then finishing the render anyway. **Windows is not settled**: this project's route to a Windows computer is a rental with two general-purpose cores and no graphics card at all, so four card-free Windows tests are written out ready to run and wait on the next Windows batch.]*

This answer does **not** hold the same on every operating system, and the sources say so plainly.

- The backends themselves are OS-bound: "CUDA is supported on Windows and Linux"; "OptiX is supported on Windows and Linux"; "HIP is supported on Windows and Linux"; "oneAPI is a computation library that is supported on Windows and Linux"; "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." (manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`, read 2026-07-30). So macOS offers Metal and nothing else of these five; Windows and Linux offer the other four and not Metal.
- MetalRT is narrower still than macOS. The Cycles file puts it behind a machine check with a comment above it: `# MetalRT only works on Apple Silicon.` followed by `if (platform.machine() == 'arm64'):` — **observed**, `Blender Sources/cycles_5.1/properties.py`, lines 1918–1919, read 2026-07-30. Reading that as *on an Intel Mac the MetalRT and Kernel Optimization controls are not drawn at all* is the same panel-drawing step recorded as **inferred** under 7b, and carries that label here too.
- The drawing Backend: the manual's Display Graphics section names only OpenGL and Vulkan, while the API and `--gpu-backend` also carry Metal. Neither source states which are offered on which operating system.
- The `--cycles-device` option and its device list are documented without any OS qualification.
- The GPU Rendering page carries error advice named for one operating system at a time — an "Unsupported GNU version" compiler error under Linux, and a display-driver timeout under Windows. I record only that the page is written that way; I did not quote those passages in full and I make no claim from them about the setting itself.

---

## What the record of looking holds

**Searches run across the whole 5.1 manual** (case-insensitive, whole tree): `optix`, `hardware ray`, `intersection method`, `BVH2`, `HIP RT`, `MetalRT`, `Embree`, `Kernel Optimization`, `rendering technolog`, `compute device`, `graphics API`, `cycles-device`, `gpu-backend`, `userpref`, `depending on the scene`, `scene complexity`, `complex scenes`, `larger scenes`, `large scenes`, `scene tested`, `scene size`, `scales with`, `scale with`, `the larger the scene`, `number of instances`, `triangle count`, `polygon count`, `geometry count`, `scene grows`, `heavier scenes`.

**Pages opened in the manual, in full or in the searched section:**
- `render/cycles/gpu_rendering` — the central page; read end to end.
- `editors/preferences/system` — read end to end.
- `editors/preferences/introduction` — read for how preferences are stored and reset.
- `render/cycles/render_settings/index` — Device and OSL sections.
- `render/cycles/render_settings/performance` — read end to end, to check whether the method choice lives there. It does not; it holds Threads, Tile Size, the acceleration-structure options, Persistent Data, Pixel Size and the compositor device.
- `render/cycles/render_settings/sampling` — Denoising section.
- `render/cycles/osl/index` — Limitations section.
- `render/eevee/render_settings/raytracing` — checked, to decide whether it was the question's subject. Ruled out: its Method chooses a tracing algorithm (Light Probe, Screen-Trace), not one of the card's facilities.
- `render/cycles/optimizations/index` and `.../nodes` — checked for a device-choice discussion. Ruled out: they cover noise reduction and shader-node optimization only.
- `advanced/command_line/arguments` — Cycles options, GPU options, Python options, `--factory-startup`, and the whole Environment Variables list.
- `advanced/blender_directory_layout` — the configuration file list.
- Listed every `.txt` under `render/cycles/` (26 files) to be sure no dedicated hardware-ray-tracing page existed. None does; the topic lives entirely on `gpu_rendering` and the Preferences System page.
- `getting_started/about/history` matched "optix" and "hardware ray" but is a version history narrative, not a statement about 5.1 behaviour — ruled out.

**Image opened:** `_images/editors_preferences_section_system.webp`, the screenshot on the Preferences System page. Reported under 7a.

**Searches run across the whole 5.1 Python API reference:** `gpu_backend`, `OptiX`, `compute_device`, `MetalRT`, `HIP RT`, `hiprt`, `oneapirt`, `kernel_optimization`, `peer_memory`, `Embree`. Only `bpy.types.PreferencesSystem` had anything, and only about the drawing backend. **The Cycles add-on's preferences are not in the API reference at all** — the only Cycles files there are `bpy.ops.cycles` (three operators: `denoise_animation`, `merge_images`, `use_shading_nodes` — none device-related) and `bpy.types.FModifierCycles` (an animation modifier, unrelated). This is why every finding about MetalRT, HIP RT, Embree-on-GPU and Kernel Optimization above is labelled observed from the add-on's files rather than documented from the API.

**Searches run across the whole 5.1 release notes** (all 21 subject files): `optix`, `metalrt`, `hip`, `oneapi`, `cuda`, `metal`, `ray trac`, `ray-trac`, `backend`, `hardware`. Read `cycles.md` and `eevee.md` in the relevant parts. The Cycles note about HIP RT and the EEVEE backend tables are quoted above. `bugfixes.md` mentions HIP RT and CUDA only in fixed-bug titles, which say nothing about how the choice behaves — ruled out.

**Cycles files examined:** `properties.py` (the `CyclesPreferences` class, lines roughly 1560–1940, plus `enum_devices` at 30–34, `device` at 381–387, and the debug flags at 1050–1100), `engine.py` (lines 1–60, the command-line parser), `source/kernel/bvh/bvh.h`, `source/kernel/types.h`, and the directory listing of `source/kernel/device/`. I also grepped the whole `cycles_5.1` tree for `metalrt`, `use_metalrt`, `hiprt`, `Embree`, `peer_memory`, `kernel_optimization`, `cycles-device`, `CYCLES_`.

**What I could not check, and why.** The supplied Cycles folder contains only `source/kernel` and `source/util`. The host-side device code — the part that would show how MetalRT's Auto decides, how hardware-ray-tracing support is detected per device, and what happens when a scene will not fit — is not in it. I did not go looking for it elsewhere, and I did not use the web.

---

## Things a running Blender could still settle

*(Pruned 2026-08-30 by the hand tests. Everything the earlier list held has either been run — the results are above, under **Tested by hand — what a running Blender showed** — or is recorded there as unable to be run, with its reason. What follows is what is genuinely still open.)*

1. **What happens when the card's own memory fills.** The scene of 125,829,122 triangles reached the host machine's memory ceiling and not the card's, so the documented fallback to system memory was never exercised. Settling it needs a machine whose card is small against what its host can build, which is the opposite of the usual want.
2. **What MetalRT's "Auto" weighs.** On the Creator's Mac it built the same structure as Off on all three scenes, including one of 300,000 curves — in the tests above, and again in sixty more runs on 2026-08-30, in all four versions, without one exception. Whether it ever picks the other way — on other Apple hardware, or on some other kind of scene — is still not settled.
3. **Whether the four card-free Windows tests give the same answers.** They are written out ready to run in `Audits/Blender Research/What a render demands/Hand tests — Open Threads questions.md` and wait on the next Windows rental.
4. **Everything needing hardware nobody here has**: an AMD card, an Intel Arc card, two cards of different ray-tracing generations in one machine, NVLink-joined cards, a Mac that is not Apple silicon, or an Apple machine an agent may open a window on. Each is named above with its reason.

---

2026-08-29 — Claude Opus 5 (Claude Code) — What a render demands (Open Threads, question 7), Blender 5.1

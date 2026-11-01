# Settings — Blender 4.5 — second pass (questions 4 and 9)

**Sources consulted, and the date.** Blender's manual for 4.5, Blender's Python API documentation for 4.5, Blender's release notes for 4.5, and Cycles' own files as shipped with 4.5 — all of them the copies in `Blender Sources/`, all read on **2026-07-30** (the snapshot date given to me). One picture from the manual was opened as well; it is named where it is used. I did not use the wider web at any point, and I did not need it.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

A note on one folder: `Blender Sources/requirements/` exists and holds a single file, but it is not one of the four sources I was told answers may come from, so I did not open it and nothing below rests on it.

Two disclosures, so the line above is read exactly as meant. First, one file I opened inside `Blender Sources/` was written by this project rather than by Blender — `Blender Sources/About this folder.md`, which lists what the folder holds. It makes no claim about how Blender behaves and nothing below rests on it. It carries a link onward to this project's research method; I did not follow it. Second, I did the work alone. No helper agents were started, so there is nobody else's reading to vouch for.

---

## 4. Does a file carry a separate sample count for each render engine?

**LABEL: documented, in part; inferred, in part.** Split as follows, because the two halves of the claim do not have the same footing:

- That each of the three built-in engines has its *own* sample count, under its own name, in its own settings block on the scene — **documented** (manual and API), and for Cycles also **observed** in Cycles' own files.
- That Workbench's final-render setting is saved with the scene — **documented** outright.
- That the file therefore carries all of them side by side, so switching engine does not disturb the others — **inferred**. The step is written out below.

### The short answer

Yes. Cycles, EEVEE and Workbench each carry their own, separately named count, and a scene holds all three settings blocks at once no matter which engine is currently chosen.

Two qualifications that the question does not ask about but that its answer would mislead without:

- One anti-aliasing sample count is **not** per engine. Grease Pencil's SSAA sample count is a single scene-wide value that the manual documents identically under all three engines' Render properties.
- Not every settings block has a sample count at all. The scene's Hydra block carries only an export method.

### The separate counts

**Cycles.** From Cycles' own files — **observed**:

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
    preview_samples: IntProperty(
        name="Viewport Samples",
        description="Number of samples to render in the viewport, unlimited if 0",
        min=0,
        soft_min=1,
        max=(1 << 24),
        default=1024,
    )
```

File: `Blender Sources/cycles_4.5/properties.py`, lines 482–495, inside `class CyclesRenderSettings(bpy.types.PropertyGroup):` which begins at line 376. Date read 2026-07-30.

That block is hung on the scene by Cycles itself — **observed**, same file, lines 1100–1106:

```
    @classmethod
    def register(cls):
        bpy.types.Scene.cycles = PointerProperty(
            name="Cycles Render Settings",
            description="Cycles render settings",
            type=cls,
        )
```

**EEVEE.** From the API documentation — **documented**:

> "taa_render_samples
>
> Number of samples per pixel for rendering
>
> Type:
>
> int in [1, inf], default 64"

and

> "taa_samples
>
> Number of samples, unlimited if 0
>
> Type:
>
> int in [0, inf], default 16"

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html. Date read 2026-07-30.

The manual's EEVEE page describes the same two controls in words — **documented**:

> "Samples
>
> The number of samples to use in the final render."

Source: manual, https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html. Date read 2026-07-30.

**Workbench.** Not a count but a choice of anti-aliasing method, with sample numbers among the choices — **documented**:

> "render_aa … enum in ['OFF', 'FXAA', '5', '8', '11', '16', '32'], default '8'"

and separately `viewport_aa`, same list, default `'FXAA'`. Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneDisplay.html. Date read 2026-07-30.

And the manual says where that setting is kept, which is the only outright statement of storage I found anywhere for any of the three — **documented**:

> "The setting for the 3D Viewport is a user preference to specify the anti-aliasing method that runs best on the used system. The setting for viewport rendering and final rendering is saved per scene."

Source: manual, https://docs.blender.org/manual/en/4.5/render/workbench/sampling.html. Date read 2026-07-30.

### That the three blocks coexist on one scene

**Documented.** The scene carries all of them at once, as separate pointers:

> "cycles
>
> Cycles render settings
>
> Type:
>
> CyclesRenderSettings, (readonly)"

> "display
>
> Scene display settings for 3D viewport
>
> Type:
>
> SceneDisplay, (readonly)"

> "eevee
>
> EEVEE settings for the scene
>
> Type:
>
> SceneEEVEE, (readonly)"

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Scene.html. Date read 2026-07-30.

The manual states the principle plainly too — **documented**:

> "Each renderer has its own render settings to control render quality and performance."

And, in the next paragraph, what is *not* per engine: *[Citation check, 2026-08-05: was "in the same paragraph"; on the page the two sentences are in different paragraphs.]*

> "These are shared between EEVEE and Cycles, however some features are only supported in one or the other."

(that sentence is about cameras, lights and materials). Source: manual, https://docs.blender.org/manual/en/4.5/render/introduction.html. Date read 2026-07-30.

### The inferred step — that the file carries them

The sources do not say, in one sentence, "the sample count is saved in the blend-file" for Cycles or for EEVEE. Here is the chain I used, and every link is quoted.

1. The counts sit on the scene, not somewhere else — shown above, and reinforced by the API's own description of the scene: "Scene data-block, consisting in objects and defining time and render related settings" (https://docs.blender.org/api/4.5/bpy.types.Scene.html).
2. Scenes are data-blocks, and data-blocks are what a blend-file holds. Manual, https://docs.blender.org/manual/en/4.5/files/data_blocks.html: "They are the primary contents of the blend-file." The same page's table lists "Scene" with the description "Primary store of all data displayed and animated."
3. The manual says a render driven from outside uses what the file holds. Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, on `-a`: "Render the whole animation using all the settings saved in the blend-file."
4. Cycles' own files contain code that repairs the sample count of scenes loaded from older files, which only makes sense if the value arrives from the file. `Blender Sources/cycles_4.5/version_update.py`, lines 216–223 — **observed**:

```
            if version <= (3, 0, 25):
                cscene = scene.cycles

                # Default changes.
                if not cscene.is_property_set("samples"):
                    cscene.samples = 128
                if not cscene.is_property_set("preview_samples"):
                    cscene.preview_samples = 32
```

A reader has to join those four to get to "the file carries a separate count for each engine, all at once". That joining is mine, so the label on that part is inferred, not documented.

### The one count that is not per engine

**Documented, and confirmed observed against the release notes.** Grease Pencil's super-sampling count is a single scene-wide value:

> "aa_samples
>
> Number of supersampling anti-aliasing samples per pixel for final render
>
> Type:
>
> int in [1, inf], default 8"

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneGpencil.html. Date read 2026-07-30. The scene reaches it through a single pointer, `grease_pencil_settings`, of type `SceneGpencil` (https://docs.blender.org/api/4.5/bpy.types.Scene.html) — there is no Cycles copy and no EEVEE copy of it.

Yet the manual documents the same control on all three engines' pages, in identical words:

> "SSAA Samples
>
> Number of samples used for super-sampling anti-aliasing in the final render. Higher values produce smoother lines but increase render time."

Source: manual — the identical text appears at https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/grease_pencil.html, https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/grease_pencil.html and https://docs.blender.org/manual/en/4.5/render/workbench/grease_pencil.html. Date read 2026-07-30.

It is new in this version. Release notes — **documented**:

> "The SSAA sample count can be controlled with the `Grease Pencil` > `Render` > `SSAA Samples` render setting."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/grease_pencil/. Date read 2026-07-30. The API change log lists `bpy.types.SceneGpencil.aa_samples` under "Added" for 4.4 to 4.5 — https://docs.blender.org/api/4.5/change_log.html.

### A fourth settings block with no count in it

**Documented.** The scene also carries a Hydra block, and its only documented property is an export method:

> "class bpy.types.SceneHydra(bpy_struct)
>
> Scene Hydra render engine settings
>
> export_method
>
> How to export the Blender scene to the Hydra render engine"

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneHydra.html. Date read 2026-07-30. So "a block per engine" does not imply "a sample count per engine".

### One further count, per view layer rather than per engine

**Documented.** The view layer carries its own override:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0"

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html. Date read 2026-07-30.

The manual places that control under a heading "Cycles" and says how it is honoured:

> "Layer Samples
>
> If any view layers have Sample Overrides configured, this option specifies how to use them."

with the three choices "Use", "Bounded", "Ignore". Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html. Date read 2026-07-30. And, from https://docs.blender.org/manual/en/4.5/render/layers/introduction.html, under its "Cycles" section: "Samples — View layer samples to override the scene samples. Controlled by the layer samples in the Sampling panel."

So this one lives in an engine-neutral place in the data but is documented only as a Cycles control.

### Not covered

- The sources nowhere state, for Cycles or EEVEE, that the sample count survives a save-and-reload. I inferred it; I did not read it.
- Nothing in the sources says what happens to a Cycles sample count stored in a file when the Cycles add-on is not enabled. Cycles' own `unregister` does `del bpy.types.Scene.cycles` (`Blender Sources/cycles_4.5/properties.py`, line 1110), which tells you the property is gone from a running Blender; it says nothing about the value in the file.
- I did not settle whether a third-party engine add-on gets its own sample count. The manual says "More renderers from third-party developers are available as add-ons. Each renderer has its own render settings" (https://docs.blender.org/manual/en/4.5/render/introduction.html), but that is about settings in general, not about a sample count, and no third-party engine is documented in the 4.5 manual.
- Nothing here settles what any of these counts *do* to a rendered image. Every claim above is about what settings exist and where they sit.
- The manual's EEVEE and Cycles sampling pages carry no screenshots, so no picture could be used to cross-check the panels for this question.

### Outside the interface

No difference is documented, and I found nothing that would create one. The counts are scene properties, and the manual's own instruction for a render driven from a command line is that it uses "all the settings saved in the blend-file" (https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html). The engine itself can be picked at launch — "-E, --engine <engine> — Specify the render engine. Use -E help to list available engines." (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html) — but that chooses which of the stored counts is used, not what any of them is.

**Observed, and worth stating because it is the shape of a gap:** Cycles' command-line options are the entire contents of one function, and there is no sample option among them. `Blender Sources/cycles_4.5/engine.py`, lines 8–21, defines exactly two arguments, `--cycles-print-stats` and `--cycles-device`. The manual's list agrees (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, "Cycles Render Options"). So there is no documented way to set a sample count at launch without changing the file or running a script. **Inferred step:** I am reading the absence of an option in the parser as the absence of the feature; a value set some other way (an environment variable, say) would not appear there.

### Operating system

Nothing in the sources ties any of these sample counts to an operating system. The one nearby remark that does mention the machine is about the Workbench *viewport* setting, not the render one: "The setting for the 3D Viewport is a user preference to specify the anti-aliasing method that runs best on the used system" (https://docs.blender.org/manual/en/4.5/render/workbench/sampling.html). The sources do not say whether any final-render count behaves differently by operating system. The test below was run on macOS only, so the other two stay untested. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, value for value (see the Linux block at the end of this answer), so this holds on macOS and on Linux. Windows stays untested. *(Changed by the citation check of the Linux hand tests, 2026-08-06: this line still said macOS alone while a Linux result sat below it.)*

### Observed — hand test 6 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder so the Creator's own Blender settings were untouched. Label: **observed**.

A file was saved from factory startup with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render sample count 3. Blender was closed and a fresh Blender opened the file, reading all four values back and switching the engine between readings:

```
MADE cycles=123 eevee=45 workbench=32 gpencil=3
BACK as-saved               engine=BLENDER_EEVEE_NEXT   cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to CYCLES engine=CYCLES               cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to BLENDER_WORKBENCH engine=BLENDER_WORKBENCH    cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to EEVEE  engine=BLENDER_EEVEE_NEXT   cycles=123 eevee=45 workbench=32 gpencil=3
```

**This settles the inferred step above.** The file does carry all three counts side by side; they survive a save and a reload; switching the engine disturbs none of them; and the Grease Pencil count is one shared value, reading 3 under all three engines.

### Observed — hand test 26, run by the Creator (2026-08-05)

Run by the Creator in Blender 4.5's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

With the Render Engine set to **EEVEE**, the Sampling panel shows a **Viewport ▸ Samples** of 16 and a separate **Render ▸ Samples** of 64. With the Render Engine set to **Workbench**, the whole Sampling panel is two rows, **Render — 8 Samples** and **Viewport — Single Pass Anti-Aliasing**, both menus rather than number fields. Cycles' own panel shows **Max Samples 4096** as a number field, alongside Noise Threshold, Min Samples and Time Limit.

So each engine's final-render count is its own control in its own panel, and the shape of each matches what the sources describe: a free number for Cycles and EEVEE, a fixed menu for Workbench.

### Observed on the rented Linux machine — Linux hand tests 6 and 14 (2026-08-06)

Run over SSH from a terminal on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 6 — does one file hold all three counts at once, and is the Grease Pencil count shared?** A file was saved with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render count 3, closed, and read back in a fresh Blender; then the engine was switched three times and the same four values read after each switch, in one run:

```
BACK as-loaded engine= BLENDER_EEVEE_NEXT cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-CYCLES engine= CYCLES cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_WORKBENCH engine= BLENDER_WORKBENCH cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_EEVEE_NEXT engine= BLENDER_EEVEE_NEXT cycles= 123 eevee= 45 workbench= 32 gpencil= 3
```

So one file carries three separate counts, they survive a save and a reload, switching the engine disturbs none of them, and the Grease Pencil count reads the same under every engine. **The two machines agree**, value for value.

**Linux hand test 14 — what each engine's own block actually holds.** The EEVEE block holds **55** properties on this build and the Workbench block holds **9** — the same two counts the Mac's run gave for this version. **The two machines agree.**

---

## 9. Does a file record whether it renders on the computer's graphics cards or on its processor?

**LABEL: documented and observed for the part that is recorded; documented for the part that is not; "not in the sources" for Workbench.**

### The short answer

Partly, and the split matters.

- **Cycles: yes, a two-way choice is recorded per scene** — CPU or GPU Compute. That is all it records. *Which* graphics card, and which of the GPU technologies (CUDA, OptiX, HIP, oneAPI, Metal), is **not** in the file; it is a setting of the machine, kept in the user preferences.
- **Two more CPU-or-GPU choices are recorded per scene and are not tied to any engine**: the compositor's device, and the compositor's denoise-node device.
- **Cycles also records two on/off switches for denoising on the GPU.**
- **EEVEE: no such record, and the manual says there is nothing to record** — it only renders on the GPU.
- **Workbench: not in the sources.**

### Cycles' recorded choice

The manual states the split between machine and file more plainly than anywhere else — **documented**:

> "To enable GPU rendering, go into the Preferences ‣ System ‣ Cycles Render Devices, and select either CUDA, OptiX, HIP, oneAPI, or Metal. Next, you must configure each scene to use GPU rendering in Properties ‣ Render ‣ Device."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html. Date read 2026-07-30. The same two-part instruction appears on the Cycles introduction page: "To use Cycles, select it as the Render Engine in the Render properties. For GPU accelerated rendering, enable compatible devices in Preferences ‣ System ‣ Cycles Render Devices." (https://docs.blender.org/manual/en/4.5/render/cycles/introduction.html).

"configure each scene" is the phrase that carries the answer: the choice belongs to a scene, one scene at a time.

What that per-scene choice actually is, from Cycles' own files — **observed**:

```
enum_devices = (
    ('CPU', "CPU", "Use CPU for rendering"),
    ('GPU', "GPU Compute",
        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
)
```

File: `Blender Sources/cycles_4.5/properties.py`, lines 30–34. Date read 2026-07-30.

```
    device: EnumProperty(
        name="Device",
        description="Device to use for rendering",
        items=enum_devices,
        default='CPU',
    )
```

Same file, lines 378–383, the first property of `class CyclesRenderSettings(bpy.types.PropertyGroup):` (line 376) — which is the block registered onto the scene as `bpy.types.Scene.cycles` (lines 1100–1106, quoted under question 4). Date read 2026-07-30.

So: two values, `CPU` and `GPU`, default `CPU`, stored alongside the rest of the scene's Cycles settings. Note the wording built into the option itself — the GPU option is "configured in the system tab in the user preferences", which is Cycles saying, in its own file, that the file holds only the *choice to use a GPU*, not the identity of one.

### What the file does not record

**Observed.** The machine-side settings sit in a different class entirely — an add-on preferences class, not a scene block. `Blender Sources/cycles_4.5/properties.py`, line 1555 begins `class CyclesPreferences(bpy.types.AddonPreferences):`, and inside it, lines 1585–1592:

```
    compute_device_type: EnumProperty(
        name="Compute Device Type",
        description="Device to use for computation (rendering with Cycles)",
        default=CyclesPreferences.default_device(),
        items=CyclesPreferences.get_device_types,
    )

    devices: CollectionProperty(type=CyclesDeviceSettings)
```

and the individual cards, lines 1548–1552:

```
class CyclesDeviceSettings(bpy.types.PropertyGroup):
    id: StringProperty(name="ID")
    name: StringProperty(name="Name")
    use: BoolProperty(name="Use", default=True)
    type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
```

Date read 2026-07-30.

**Documented.** Preferences are global and are kept in their own file, not in the scene file. The API calls the type "Global preferences" (https://docs.blender.org/api/4.5/bpy.types.Preferences.html), and the manual's directory layout lists, among a user's configuration files:

> "./config/userpref.blend
>
> User preferences."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html. Date read 2026-07-30.

The manual's own description of the preferences control confirms it is about the machine, not the scene:

> "Cycles Render Device
>
> Changes the computing device the Cycles render engine uses to render images. Cycles can use either the CPU or certain GPUs to render images"

and, for each entry, wording of the form "If the system has a compatible NVIDIA CUDA device, it will be available as an option for rendering with Cycles." Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html. Date read 2026-07-30.

**Documented, from a picture.** I opened `_images/editors_preferences_section_system.png`, the screenshot on that same preferences page. What is visible in it: a panel headed "Cycles Render Devices" sitting at the top of the System section of the Preferences window, above "Memory & Limits", "Video Sequencer" and "Sound". In this particular screenshot the panel's body shows no device entries. What is visible is that the device panel is a preferences panel; the emptiness of the body is not something to read anything into, since the page itself says entries appear only when the system has a compatible device.

### Two further CPU-or-GPU choices that are in the file and belong to no engine

**Documented.** Both are on `RenderSettings`, which the API describes as "Rendering settings for a Scene data-block" (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html):

> "compositor_device
>
> Set how compositing is executed
>
> Type:
>
> enum in ['CPU', 'GPU'], default 'CPU'"

> "compositor_denoise_device
>
> The device to use to process the denoise nodes in the compositor
>
> AUTO
> Auto – Use the same device used by the compositor to process the denoise node.
>
> CPU
> CPU – Use the CPU to process the denoise node.
>
> GPU
> GPU – Use the GPU to process the denoise node if available, otherwise fallback to CPU.
>
> Type:
>
> enum in ['AUTO', 'CPU', 'GPU'], default 'AUTO'"

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html. Date read 2026-07-30. The second is new in this version: the API change log lists `bpy.types.RenderSettings.compositor_denoise_device` under "Added" for 4.4 to 4.5 (https://docs.blender.org/api/4.5/change_log.html).

The manual documents both under the Render properties, and it does so on the Cycles *and* the EEVEE performance pages in identical words, which is what tells you they are not Cycles' own:

> "Device
>
> The device used for compositing.
>
> CPU:
>
> Use the CPU for compositing.
>
> GPU:
>
> Use the GPU for compositing."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html and https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/performance.html. The same control is also documented in the compositor's own sidebar at https://docs.blender.org/manual/en/4.5/compositing/sidebar.html. Date read 2026-07-30.

These enums are, by search, the only CPU-or-GPU choices anywhere in the documented API: a search of all 2,049 API files for `enum in ['CPU'` and for `enum in [` containing `'GPU'` returns `bpy.types.RenderSettings.txt` and nothing else.

### Cycles' two denoise-on-GPU switches, also in the scene

**Observed.** `Blender Sources/cycles_4.5/properties.py`, lines 435–439 and 476–480, both inside `CyclesRenderSettings`:

```
    denoising_use_gpu: BoolProperty(
        name="Denoise on GPU",
        description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
        default=False,
    )
```

```
    preview_denoising_use_gpu: BoolProperty(
        name="Denoise Preview on GPU",
        description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
        default=True,
    )
```

Date read 2026-07-30. The manual documents the first as "Use GPU — Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory." (https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html).

### EEVEE

**Documented.** There is no CPU-or-GPU choice to record, because there is no CPU option:

> "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient. CPU power is still needed to handle high scene complexity as the geometry must be prepared by the CPU before rendering each frame."

Source: manual, https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html. Date read 2026-07-30. That page also states, immediately after: "There is currently no support for multiple GPU systems."

Searching the API's EEVEE settings type for `device`, `gpu` and `cpu` returns only two hits, both about GPU memory sizing (`gi_irradiance_pool_size`, `shadow_pool_size`), neither of them a choice (https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html).

### Workbench

**Not in the sources.** Searching every Workbench page in the manual for `gpu`, `cpu` and `device` returns nothing at all, and the API's `SceneDisplay` type has no property matching any of those three. The Workbench performance page contains one setting, "High Quality Normals", and nothing about hardware (https://docs.blender.org/manual/en/4.5/render/workbench/performance.html). Where I looked is in the record below.

### Not covered

- The sources do not state that the Cycles `device` value survives a save and reload. The chain is the same as for question 4 — it is a property of a scene block, scenes are data-blocks, data-blocks are the contents of a blend-file — and the same inferred step applies. What *is* documented outright is that the setting is per scene ("configure each scene…"), which is what the question mostly turns on.
- Nothing here settles what a file does when it says GPU and the machine has no enabled device. Cycles' own interface code treats that case (`Blender Sources/cycles_4.5/ui.py`, lines 102–112, where `use_cpu` returns true when `get_device_type(context) == 'NONE'` even if the scene says GPU), but that governs which panels are drawn, not what a render does. Reading a render's behaviour out of interface code would be a step I am not willing to take.
- The number of processor cores is a separate, engine-neutral scene setting (`threads`, `threads_mode` on `RenderSettings`), but it records how much processor to use, not whether the processor is used instead of a graphics card. I have not treated it as part of the answer.
- Nothing here settles what a file records for a third-party engine add-on.

### Outside the interface

**This is where the answer changes, and the sources are explicit about it.**

**Documented.** The manual lists a Cycles command-line option:

> "--cycles-device <device>
>
> Set the device used for rendering.
>
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU.
>
> Example:
>
> blender -b file.blend -f 20 -- --cycles-device OPTIX"

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html. Date read 2026-07-30. The same page notes the double dash: "Cycles add-on options must be specified following a double dash."

**Observed**, and it says something the manual page does not — that this beats *both* the machine setting and the file's:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

File: `Blender Sources/cycles_4.5/engine.py`, lines 16–20. Date read 2026-07-30. **Inferred step, small but real:** that text is the option's own help string, so it is Cycles describing its own behaviour; I am taking the description at its word rather than watching a render obey it.

Two things follow that are worth stating separately. First, from outside the interface a caller can name a specific technology (`OPTIX`, `METAL`, and so on), which is finer than anything the file can hold — the file holds only `CPU` or `GPU`. Second, the file's record is therefore not the last word when Blender is driven from outside; it is a default that a launch argument can replace.

Also **documented**, for completeness on what a headless run reads: `-b, --background — Run in background (often used for UI-less rendering)` and, for a whole animation, "Render the whole animation using all the settings saved in the blend-file" (https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html). And `--factory-startup` is documented only as "Skip reading the startup.blend in the users home directory" — it says nothing about preferences, so I make no claim about whether a background run picks up the machine's device preferences.

### Operating system

**Documented, and it does not hold everywhere.** The recorded value itself is just `CPU` or `GPU` and carries no operating system with it. But whether `GPU` can be honoured depends on the machine, and the manual sets out which technologies exist on which systems:

> "CUDA is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 3.0 and higher."

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535."

> "HIP is supported on Windows and Linux and requires a AMD graphics card with the RDNA1 architecture or newer."

> "oneAPI is a computation library that is supported on Windows and Linux and requires a Intel® Arc™ graphics card with the Xe HPG architecture."

> "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html. Date read 2026-07-30.

So the same file, saying `GPU`, meets a different set of available technologies on each system, and `--cycles-device METAL` is meaningful on macOS in a way it is not on Windows or Linux.

One further operating-system limit, documented on EEVEE's page and relevant because it bears on whether a stored setting can be acted on at all away from a screen: "Headless rendering is not supported on headless Windows systems." (https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html).

For the compositor's device setting and the Cycles denoise-on-GPU switches, the sources say nothing about operating systems.

**Observed 2026-08-05.** On this machine, Metal is the graphics technology found. CUDA, OptiX, HIP and oneAPI could not be tried at all — the Creator has Apple's own graphics and no other card — so what a file's `GPU` choice means on any other make of card stays untested, and so does the same file opened on a second machine.

**Observed 2026-08-06.** A rented Linux machine with an NVIDIA GeForce RTX 4090 settles most of that (see the Linux block at the end of this answer). That machine offers `CUDA`, `OPTIX`, `HIP` and `ONEAPI` and not `METAL`; `--cycles-device CUDA` and `OPTIX` render on the card, and `HIP`, `ONEAPI` and `METAL` are all refused with the same message. A file asking for GPU Compute renders on the card under CUDA or OptiX, and under `NONE`, `HIP` or `ONEAPI` it falls back to the processor with nothing printed at all in this version. What an AMD or an Intel card does stays untested, and so does Windows. *(Changed by the citation check of the Linux hand tests, 2026-08-06: this line still said the other technologies could not be tried while a Linux result sat below it.)*

### Observed — hand tests 7 and 8 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders so the Creator's own Blender settings were untouched. Label: **observed**.

**Hand test 7 — does the value survive a save, and is it per scene?** A file was saved from factory startup with two scenes: `One` with the Cycles device set to `GPU` and the compositor device set to `GPU`, and `Two` left at its defaults. A fresh Blender opened it and printed:

```
SCENE One cycles_device= GPU compositor_device= GPU
SCENE Two cycles_device= CPU compositor_device= CPU
```

**This settles the inferred step above.** The file does carry the Cycles device choice through a save and a reload, and it is held one scene at a time. The compositor's device choice is per scene too.

**Hand test 8 — what `--cycles-device` beats, and the double dash.** A file was saved with Cycles on the processor, 100 by 100, 16 samples. A throwaway settings folder was prepared in which Metal was chosen; preparing it printed:

```
DEVICE: CPU Apple M1 Pro use= True
DEVICE: METAL Apple M1 Pro (GPU - 14 cores) use= True
SAVED compute_device_type= METAL
```

With the file still saying `CPU`:

```
blender -b test.blend -o out/a_ -F PNG -f 1 -- --cycles-device METAL --cycles-print-stats
```

Its statistics carried the line `Profiling information not available (only works with CPU rendering)`; the same run without the override did not carry it. So the argument does beat what the file says, and Cycles' own help text is borne out on this point.

Without the double dash:

```
blender -b test.blend -o out/b_ -F PNG -f 1 --cycles-device METAL
unknown argument, loading as file: --cycles-device
Error: Cannot read file ".../--cycles-device": No such file or directory
```

With a name no device bears:

```
blender -b test.blend -o out/c_ -F PNG -f 1 -- --cycles-device NOT_A_DEVICE
Error: NOT_A_DEVICE is not a valid Cycles device.
```

and no image was written.

**A file asking for the graphics card on a machine with none enabled.** A second throwaway settings folder was prepared with the compute device type set to `NONE`, and the scene's device forced to `GPU` at launch. The run printed no warning of any kind about the device, its statistics carried no `Profiling information not available` line, and it ended:

```
Fra:1 Mem:8.29M (Peak 9.09M) | Time:00:00.07 | Mem:0.02M, Peak:0.02M | Scene, ViewLayer | Sample 16/16
Saved: '.../out/dd_0001.png'
```

So the render ran on the processor and the image was written, with nothing said about it.

### Observed — hand test 27, run by the Creator (2026-08-05)

Run by the Creator in Blender 4.5's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

With the Render Engine set to **Workbench**, they expanded every panel of the Render Properties tab and photographed the whole of it: Simplify, Performance, Sampling, Lighting, Object Color, Options, Film, Grease Pencil, Freestyle, Color Management.

Three controls in that tab name a processor or a graphics card, and all three sit in the Performance panel:

```
Performance
  High Quality Normals        unticked
  Compositor
    Device                    CPU | GPU        — CPU selected
  Denoise Nodes
    Denoising Device          Auto | CPU | GPU — Auto selected
    Preview Quality           Balanced
    Final Quality             High
```

Those are the compositor's and the denoise nodes' devices — the two engine-neutral `RenderSettings` entries recorded above — shown here because the tab carries them whatever the engine. **No control anywhere in the tab is a Workbench device setting.** The only other control naming a device is Color Management ▸ **Display Device**, reading `sRGB`, which is a colour display device.

**This closes the "not in the sources" answer for Workbench above**, and it agrees with the property list: Workbench itself records nothing about the processor or the graphics card.

### Observed on the rented Linux machine — Linux hand tests 7, 8, 9, 10 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

This is the question the new machine was most needed for. The Creator's Mac has Apple's own graphics and could reach only `METAL` and `CPU`; this one has an NVIDIA GeForce RTX 4090.

**Linux hand test 7 — does the choice survive a save, is it per scene, and what does a brand-new scene start at?** A brand-new factory scene, read without saving anything:

```
FACTORY engine= BLENDER_EEVEE_NEXT cycles_device= CPU compositor_device= CPU
```

Then a file saved with two scenes, `One` with the Cycles device set to `GPU` and the compositor device set to `GPU`, `Two` left at its defaults, opened again in a fresh Blender:

```
SCENE One cycles_device= GPU compositor_device= GPU
SCENE Two cycles_device= CPU compositor_device= CPU
```

So the file carries the choice and holds it one scene at a time. **The two machines agree**, including on the compositor device a new scene starts at, which reads `CPU` here as it did on the Mac.

**What technologies this machine offers at all.** Setting Cycles' compute device type to `METAL` was refused, and the refusal names the whole set:

```
SETTYPE ERROR TypeError('bpy_struct: item.attr = val: enum "METAL" not found in (\'NONE\', \'CUDA\', \'OPTIX\', \'HIP\', \'ONEAPI\')')
```

So the choices are `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI`, and **`METAL` is not among them on this machine**. The devices found were:

```
DEVICE: CUDA NVIDIA GeForce RTX 4090 use= True
DEVICE: CPU AMD Ryzen Threadripper PRO 3975WX 32-Cores use= True
DEVICE: OPTIX NVIDIA GeForce RTX 4090 use= True
```

(The processor entry names 32 cores; the container is given twelve of them. That is the container's doing, not Blender's.)

**Linux hand test 9 — what each `--cycles-device` name does.** A Cycles file whose scene device is `CPU`, 100 by 100, 16 samples, rendered six times from a settings folder with no technology chosen, each run being `blender -b test.blend -o out/<name>_ -F PNG -f 1 -- --cycles-device <NAME> --cycles-print-stats`. The statistics run to well over a hundred lines; the line that carries the answer is `Profiling information not available (only works with CPU rendering)`, which Cycles prints only when the render is not on the processor. The results:

```
CUDA     exit 0   image written   'Profiling information not available (only works with CPU rendering)' present
OPTIX    exit 0   image written   'Profiling information not available (only works with CPU rendering)' present
CPU      exit 0   image written   no such line
HIP      exit 1   no image        stopped with 'Found no Cycles device of the specified type'
ONEAPI   exit 1   no image        stopped with 'Found no Cycles device of the specified type'
METAL    exit 1   no image        stopped with 'Found no Cycles device of the specified type'
```

A refused run is short. The line that carries it, word for word:

```
Error: Found no Cycles device of the specified type
```

None of the three printed anything before the error — no warning about a missing library, nothing.

So on a machine with an NVIDIA card, `CUDA` and `OPTIX` both render; and **`HIP`, `ONEAPI` and `METAL` are all refused in exactly the same words**. A name for a technology no card here can serve behaves the same as a name for a technology this operating system cannot have at all. The refusal stops the render and writes nothing.

**Linux hand test 8 — a device name that names no device, and the missing double dash.** With `-- --cycles-device NOT_A_DEVICE`:

```
Error: NOT_A_DEVICE is not a valid Cycles device.
```

exit code 1 and no image, as on the Mac. Without the double dash, `blender -b test.blend -o out/nodash_ -F PNG -f 1 --cycles-device CUDA` printed:

```
Fra: 1 … Sample 16/16
Saved: '…/out/nodash_0001.png'
unknown argument, loading as file: --cycles-device
Cannot read file "…/--cycles-device": No such file or directory
```

exit code 1. **This is fuller than the Mac's record.** The argument is indeed taken as a file name, but the frame asked for had already been rendered and saved before Blender got that far, so an image was written and the run still ended at exit code 1. The device was ignored — the statistics carried no `Profiling information not available` line, so the render was on the processor, which is what the file said.

**Linux hand test 10 — what a file's "GPU Compute" choice actually means here.** The same file, its scene device forced to `GPU` at launch, run from five settings folders in turn, each reporting its own saved technology and devices before the render:

```
preferences say NONE    -> exit 0, image written, no 'Profiling information not available' line -> rendered on the processor
preferences say CUDA    -> exit 0, image written, 'Profiling information not available' present -> rendered on the card
preferences say OPTIX   -> exit 0, image written, 'Profiling information not available' present -> rendered on the card
preferences say HIP     -> exit 0, image written, no such line                                  -> rendered on the processor
preferences say ONEAPI  -> exit 0, image written, no such line                                  -> rendered on the processor
```

So the two-way choice in the file means "use whatever this machine's preferences chose", and when that choice finds no card the render falls back to the processor, writes the image and exits 0. **The two machines agree on the silence** — the Mac found the same fallback with no graphics device enabled — and this machine shows it happening for a second reason as well, a technology chosen for a card that is not there. What Blender said about it was nothing at all — not one line, in any of the three. Anything that needs to know whether a render actually used the graphics card cannot learn it from a warning, on either machine.

**Linux hand test 25 — Workbench's own Render Properties tab, photographed on this machine.** Blender was started with a real window on the machine's display, the engine set to Workbench, the Properties editor pointed at Render, and the window photographed with Blender's own screenshot command. Blender 4.5's tab lists Simplify, Performance, Sampling, Lighting and Object Color from the top, with no Curves panel. No control in the tab is a Workbench device setting, matching the Creator's photographs of the same tab on the Mac. **The two machines agree.**

---

## Where I looked

This pass exists because an earlier one did not record this. So, at length.

### What I searched, and how

The manual (4,954 files), the API documentation (2,049 files), the release notes (19 files) and Cycles' own files (399 files) for 4.5 were searched whole, by text, before any file was opened. Counts below are numbers of files containing the term, per source, in the order manual / API / release notes / Cycles.

**For question 4 — sample counts.**

| term searched | manual | API | release notes | Cycles |
|---|---|---|---|---|
| `samples` | 75 | 56 | 1 | 25 |
| `sample count` | 6 | 0 | 1 | 5 |
| `samples per pixel` | 2 | 2 | 0 | 2 |
| `AA samples` | 4 | 0 | 1 | 1 |
| `anti-aliasing` | 37 | 5 | 2 | 2 |
| `Temporal Anti-Aliasing` | 1 | 0 | 0 | 0 |
| `taa_render_samples` | 0 | 3 | 0 | 0 |
| `aa_samples` | 0 | 6 | 0 | 0 |
| `max samples` | 0 | 0 | 0 | 3 |
| `supersampling` | 0 | 1 | 1 | 0 |
| `SSAA` | 3 | 0 | 1 | 0 |
| `SMAA` | 4 | 0 | 1 | 0 |

Also searched, across the manual: `per scene` and `per-scene` (3 files, one of them the Workbench sampling page that gave the storage sentence); `saved in the blend`, `stored in the blend`, `saved with the blend`, `stored in the file`, `saved in the file` (3 files); `switch … engine`, `change the render engine`, `changing the render engine` (no relevant hit — the only hits were about pose libraries and rigid-body physics); `own render settings`, `its own settings`, `engine-specific`, `engine specific` (the render introduction page); `engine-independent` (the view layer introduction page); `Sample Override`, `Samples Override` (the Cycles sampling page only).

**For question 9 — processor or graphics cards.**

| term searched | manual | API | release notes | Cycles |
|---|---|---|---|---|
| `device` | 19 | 30 | 5 | 248 |
| `GPU` | 45 | 52 | 6 | 89 |
| `CPU` | 32 | 6 | 3 | 36 |
| `graphics card` | 19 | 1 | 0 | 0 |
| `processor` | 4 | 6 | 1 | 4 |
| `compute device` | 1 | 0 | 0 | 1 |
| `render device` | 5 | 0 | 0 | 0 |
| `CUDA` | 3 | 0 | 0 | 39 |
| `OptiX` | 10 | 0 | 2 | 33 |
| `HIP` | 54 | 23 | 2 | 34 |
| `Metal` | 24 | 21 | 3 | 42 |
| `oneAPI` | 4 | 0 | 1 | 18 |
| `OpenCL` | 1 | 3 | 0 | 4 |
| `headless` | 2 | 0 | 0 | 0 |
| `hardware` | 39 | 1 | 1 | 2 |

(The high counts for `HIP` and `Metal` are substring noise — "ship", "relationship", "metallic" — and were discounted by reading the hits.)

Two structural searches were run rather than word searches, because they can prove absence where a word search cannot:

- Every API file searched for `enum in ['CPU'` — exactly one file, `bpy.types.RenderSettings`. And every API file for an `enum in [...]` line containing `'GPU'` — the same one file, two lines. This is what lets me say the compositor's two settings are the *only* documented CPU-or-GPU choices in the whole API.
- `Blender Sources/cycles_4.5/engine.py` read whole, to enumerate Cycles' command-line options exhaustively rather than trusting a search.

### Pages and files I opened, and why

**Manual, read in full:**
`render/introduction.txt`, `render/index.txt`, `render/cycles/introduction.txt`, `render/eevee/introduction.txt`, `render/workbench/introduction.txt` — to find where sample counts and devices are said to belong.
`render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt`, `render/workbench/sampling.txt` — the three sampling pages.
`render/cycles/render_settings/grease_pencil.txt`, `render/eevee/render_settings/grease_pencil.txt`, `render/workbench/grease_pencil.txt` — all three, to test whether the new SSAA count is per engine. They are word-for-word identical, which is what pointed me to a single shared property.
`render/cycles/gpu_rendering.txt` — the page that carries the machine-versus-scene split for question 9.
`render/cycles/render_settings/performance.txt`, `render/eevee/render_settings/performance.txt`, `render/workbench/performance.txt` — for the compositor device, and to establish that Workbench's performance page has no device setting.
`render/eevee/limitations/limitations.txt` — EEVEE's CPU-rendering statement.
`editors/preferences/system.txt`, `editors/preferences/introduction.txt` — the preferences side.
`advanced/command_line/arguments.txt` (the render-options portion), `advanced/command_line/render.txt` — for driving Blender from outside.
`advanced/blender_directory_layout.txt` (the config portion) — where preferences are kept.
`files/data_blocks.txt` (opening portion and the Scene row of its table), `files/blend/index.txt`, `scene_layout/scene/introduction.txt` — for what a blend-file holds.
`render/layers/introduction.txt`, `render/cycles/render_settings/index.txt`, `compositing/sidebar.txt` (performance portion).

**Manual pictures opened:** `_images/editors_preferences_section_system.png`. I checked for panel screenshots on the three sampling pages and on the Cycles render-settings pages first; there are none — a search of all three sampling pages and both render-settings index pages for `image:` returns nothing, and a listing of the 2,880 files in `_images` shows no Cycles or EEVEE render-settings panel screenshot at all. So no picture could be brought to bear on question 4.

**API documentation, read:** `bpy.types.Scene`, `bpy.types.RenderSettings`, `bpy.types.SceneEEVEE`, `bpy.types.SceneDisplay`, `bpy.types.SceneGpencil`, `bpy.types.SceneHydra`, `bpy.types.ViewLayer` (samples portion), `bpy.types.Preferences`, `bpy.types.ID` (opening), `bpy.props` (opening), `change_log`.

**Release notes, searched and read:** all 19 files searched for `sample` and for `device`; `grease_pencil.md` read for the SSAA passage; `cycles.md`, `eevee.md`, `rendering.md`, `compositor.md` scanned for anything about sample counts or devices. `bugfixes.md` searched for both terms — the hits are geometry-node and colour-picker fixes, nothing bearing on either question.

**Cycles' own files, read:** `properties.py` (the whole of `CyclesRenderSettings`, `CyclesRenderLayerSettings`, `CyclesDeviceSettings`, `CyclesPreferences`, and the enum tables at the head of the file), `engine.py` (whole), `__init__.py` (whole), `version_update.py` (the scene-versioning portion), `ui.py` (the device helpers and the sampling and device panels).

### What I ruled out, and why

- **`bpy.props` documentation as a source for "custom properties are saved in the blend-file".** I opened it hoping for an outright statement, since Cycles' settings are registered that way. It has none — it covers assigning properties to existing classes, operator properties and thread safety, and never mentions saving. So I did not use it, and the storage claim is carried by the data-blocks page instead, and labelled inferred.
- **The `render/cycles/render_settings/` index as a place to find the Device setting.** It has no Device entry; the Device control is documented only on the GPU Rendering page. Worth recording because it is a natural place to look and it is empty.
- **The manual's glossary, colour-management page, output-properties page and getting-started hardware page**, all of which match on `device`. Every hit is about input devices, display devices or playback devices. None bears on either question.
- **`--factory-startup`** as evidence about preferences in a headless run. Its documented text mentions only `startup.blend`, so I drew nothing from it.
- **`Blender Sources/requirements/`.** Not one of the four permitted sources. Not opened.
- **Anything outside `Blender Sources/`.** Not opened.

---

## Things a person with Blender 4.5 open could settle in a few minutes

Everything on this list has now been run and written into the answers above — items 1, 3, 4, 5 and 6 from a terminal on the Creator's Mac on 2026-08-05, item 7 by the Creator at Blender's window the same day, and every terminal test again on a rented Linux machine on 2026-08-06. The two items that were left have now been answered:

1. **Whether the same file's GPU choice means the same on a second computer.** *Answered on 2026-08-06.* The same file was read and rendered on a rented Linux machine with an NVIDIA GeForce RTX 4090. The two-way choice in the file is the same there, and what it means depends on that machine's own preferences — see question 9. What it means on an AMD or an Intel card is still untried, because the project has no such card.

2. **What `--cycles-device OPTIX` does.** *Answered on 2026-08-06* on the same machine: it renders on the card. `CUDA` does too; `HIP`, `ONEAPI` and `METAL` are all refused there with the same message. Recorded under question 9.

---

2026-08-04 — Opus 5 (Claude Code) — Settings, Blender 4.5

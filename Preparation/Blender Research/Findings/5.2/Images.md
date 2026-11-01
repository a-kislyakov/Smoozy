# Images — Blender 5.2

**Sources consulted:** Blender 5.2 manual (plain-text copy, including one picture from its `_images` folder), Blender 5.2 Python API documentation (plain-text copy), Blender 5.2 release notes (plain files), and Cycles' own files as shipped with Blender 5.2. All were read on **2026-07-30** (the snapshot date given to me); I worked through them on 2026-08-03.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

A note on one file I did open inside `Blender Sources/`: `Blender Sources/About this folder.md`. It sits at the top of that folder and describes what the folder holds. It contains a link out to a project document; I did not follow it.

---

## 1. What kinds of picture can a render produce, and how can the finished picture be told apart from the others?

**LABEL: documented** (for the kinds, and for the name of the finished one), with one **inferred** step marked below.

The sources describe passes and view layers as ways a render separates pictures. They also describe render slots as places to keep whole renders.

**Passes.** These are the different pictures of the same frame.

> "A Pass is a type of intermediate rendering information that's extracted as a separate image. Examples include the diffuse colors of the objects in the scene, the light distribution, the depth map, and the normal map."

**View layers.** A scene can be split into several, each of which renders its own set of passes:

> "View Layers allow you to separate a render into multiple layers, which can then be composited together."

**Render slots.** Somewhere to keep several whole renders side by side:

> "The render slot to view (and render to). You can create new renders without losing previous ones by selecting an empty slot before rendering."

**How the finished picture is told apart.** It is the pass named **Combined**. The manual says of it, in the Passes list:

> "Combined Cycles, EEVEE, Workbench
>
> The render output before any compositing is applied."

and in the View Layers introduction, listing what each view layer can output:

> "Combined (final image)"

The Python API says the same of the switch that turns it on:

> "use_pass_combined
>
> Deliver full combined RGBA buffer (default False, readonly)"

If compositing is switched on, the quoted passages say that the compositor's Group Output defines the image result sent to image output:

> "In the Compositor, this node defines the image result that is sent to the renderer or to image output after rendering."

> "In the Compositor, only the first image input is used as the final render result. Any additional sockets will be ignored."

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

In the Image Editor, the three dropdowns that pick between all of these are named outright:

> "Render Result: displays renders. When this item is selected, the Slot, View Layer, and Render Pass selectors become available (see below)."

**Inferred step:** the manual never says in so many words "the single file written to the output path is the finished picture and not a pass". I take that from the three statements above put together — Combined is "the render output", Group Output "defines the image result that is sent to … image output after rendering", and the Output panel "defines where rendered images and animations are saved". A reader has to join those, so this is inferred, not documented.

**LABEL: observed** (Cycles' own files) — Cycles defines settings whose names refer to passes that the manual's Passes page does not list. In `Blender Sources/cycles_5.2/properties.py`, class `CyclesRenderLayerSettings` (beginning at line 1523), the following per-view-layer switches appear, each of which calls `update_render_passes`:

> ```
>     use_pass_volume_scatter: BoolProperty(
>         name="Volume Scatter",
>         description="Contribution of paths that scattered in the volume at the primary ray",
>         default=False,
>         update=update_render_passes,
>     )
>     use_pass_volume_transmit: BoolProperty(
>         name="Volume Transmit",
>         description="Contribution of paths that transmitted through the volume at the primary ray",
>         default=False,
>         update=update_render_passes,
>     )
>     use_pass_volume_majorant: BoolProperty(
>         name="Volume Majorant",
>         description="Majorant transmittance of the volume",
>         default=False,
>         update=update_render_passes,
>     )
> ```
>
> — `Blender Sources/cycles_5.2/properties.py`, lines 1550–1567

What the quoted file says is that these settings exist under these names with these defaults; that a render actually produces those pictures is a step beyond the file, and I have not taken it.

**Sources and addresses**

- Manual, https://docs.blender.org/manual/en/5.2/render/layers/passes.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/layers/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/editors/image/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/group_output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.RenderLayer.html — read 2026-07-30
- Cycles' own files, `Blender Sources/cycles_5.2/properties.py` — read 2026-07-30

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2. Blender 5.2.0 was started from a terminal with no window (`-b --factory-startup -noaudio`) and driven by a script. The whole list of tests is at `Audits/Blender Research/Images/Hand tests.md`.

*Two view layers, one plain single-image output.* Set up: the factory startup scene at 32 by 32; the world background set to pure red; a second view layer added, called `Second`, with the cube's collection excluded from it; output PNG; frames 1 to 1. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
VIEW LAYERS: ['ViewLayer', 'Second']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.8078, 0.5451, 0.5333, 1.0) corner=(0.8549, 0.2196, 0.1255, 1.0)
```

One file appeared, not one per view layer. Its corner carries the red world and its centre carries the lit cube.

*What lands in the output file when compositing is on.* This is the step the answer above marks as inferred. Set up: the same scene at 32 by 32, PNG, one frame; a new node group of type `CompositorNodeTree` with one colour output socket, holding a `NodeGroupOutput` node and a `CompositorNodeRGB` set to pure blue linked into it, assigned to `scene.compositing_node_group`. Done: the same render call. Blender printed:

```
use_compositing: True
NODES: ['NodeGroupOutput', 'CompositorNodeRGB']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

Centre and corner are both the compositor's blue. So the single file written to the output path holds the compositor's result, not the Combined pass.

*What the finished picture is called once written.* From the multi-layer OpenEXR test recorded under question 5: with the view layer renamed to `Beauty`, that view layer's finished picture is a part of the file whose `name` attribute reads `Beauty.Combined` and whose channels are `Beauty.Combined.R`, `.G`, `.B` and `.A`.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 5.2.0 open. Looked at: the Image Editor header with the multi-layer OpenEXR from the test above loaded, its two picture-choosing menus opened in turn. Seen: the header shows the file name `frame_0001.exr` and then two dropdown menus, the first reading `Beauty` and the second reading `Combined`. Opening the first shows `Beauty` and `Second` above a greyed label reading `Layer`. Opening the second shows `Combined`, `Depth`, `Normal`, `Mist`, `Emission`, `Diffuse Color` above a greyed label reading `Pass`. So the two selectors this answer names are the `Layer` and `Pass` menus, and the finished picture is picked by choosing `Combined` in the second.

**Observed — the same tests on a second machine, 2026-08-06.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.2.0 LTS at `/opt/blender/blender-5.2.0-linux-x64/blender`, started with no window (`-b --factory-startup -noaudio`) and driven by the same scripts the Mac run used, with only the folder path changed. The whole list is at `Audits/Blender Research/Images/Hand tests — Linux.md`.

*Two view layers, one plain single-image output.* Blender printed:

```
ENGINE: BLENDER_EEVEE
VIEW LAYERS: ['ViewLayer', 'Second']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.8078, 0.5451, 0.5333, 1.0) corner=(0.8549, 0.2196, 0.1255, 1.0)
```

*Compositing switched on.* Blender printed `ROUTE: scene.compositing_node_group with NodeGroupOutput`, `NODES: ['NodeGroupOutput', 'CompositorNodeRGB']`, and `FILE frame_0001.png size=32x32 centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)`.

*The names once written.* The multi-layer OpenEXR written on this machine carries the same part name as the Mac's — the finished picture of the view layer called `Beauty` is the part named `Beauty.Combined`.

Both machines were run and both agreed, to the fourth decimal place.

**Observed — the window on the rented Linux machine, 2026-08-06.** Blender 5.2.0 LTS was opened with a window on that machine's own display, the largest area turned into an Image Editor, the multi-layer OpenEXR above loaded into it, and the window photographed with Blender's own screenshot command, `bpy.ops.screen.screenshot`. The picture shows the header holding **two** picture-choosing menus, reading `Beauty` and `Combined` — the same two the Creator photographed on the Mac — and no third menu beside them. The Image Editor's own `View` menu, which sits in the header next to `Image`, was then opened with `bpy.ops.wm.call_menu(name="IMAGE_MT_view")` and photographed. It holds, top to bottom: `Toolbar`, `Sidebar`, `Tool Settings`, `Asset Shelf`, `Adjust Last Operation`, `Update Automatically`, `Show Metadata`, `Frame All`, `Center View to Cursor`, `Zoom`, `Area`, with `Asset Shelf` and `Center View to Cursor` greyed out. Nothing in that menu chooses which picture is shown.

**Not covered.** What these pictures are called once written into a file is settled by the hand test above; the sources still give only the names as they appear in Blender's own lists. Also not covered: the exact full list of pass names, engines other than Cycles, EEVEE and Workbench, how many render slots there can be, and whether every pass on the manual page is produced by every engine.

**Outside the interface.** **Observed.** The tests above were all run with no window open, and each did what is recorded: the passes were switched on as view-layer properties, the render was started, the file was written, and the compositor's result was the thing written. So the passes, the names they carry into a file, and the compositor deciding the saved picture all hold when Blender is driven from outside its own window. What stays window-only is the *picking*: the Slot, View Layer and Render Pass selectors are documented only inside the interface, and outside the window the choice is made by what gets written to a file. What becomes of render slots in a background run was not tested. (Previously, before the test: not settled.)

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** Every test above was run on both and came back the same, and the Image Editor's `Layer` and `Pass` menus were photographed on both. [Citation check 2026-08-06: this read "the Image Editor's menus were photographed on both". The header's `Layer` and `Pass` menus were indeed photographed on both machines, but the Image Editor's own `View` menu (`IMAGE_MT_view`) was photographed on the Linux machine alone and stays untried on the Mac. Narrowed.] **Windows is still untried** — no Windows machine has run it. From the sources: nothing on the pages above ties any of this to a particular operating system.

**Where I looked.** Searched the whole manual for `Combined` (30 hits, most of them unrelated uses of the ordinary word), `Render Result` (14 hits), `pass`/`passes`; read `render/layers/passes.txt`, `render/layers/introduction.txt`, `render/layers/index.txt`, `editors/image/introduction.txt`, `editors/image/image_settings.txt`, `editors/image/editing.txt`, `render/output/properties/post_processing.txt`, `compositing/types/output/index.txt` and all five files under `compositing/types/output/`, `compositing/types/input/scene/render_layers.txt`. Searched the API folder for `Combined` and read `bpy.types.RenderResult`, `bpy.types.RenderLayer`, `bpy.types.RenderPass`, `bpy.types.ImageUser`. Grepped `cycles_5.2/properties.py` for `pass_`/`use_pass`. I checked whether 5.2 still has a "Composite" output node — the folder `compositing/types/output/` contains only `enable_output`, `file_output`, `group_output`, `viewer` and `warning`, and a manual-wide search for "Composite node" returned one hit, in `post_processing.txt`, which points at the Group Output node.

---

## 2. Do any of the pictures a render produces show whether a scene's textures loaded, and if so, which?

**LABEL: observed** (Cycles' own files), with an **inferred** step, and **not in the sources** for the documentation side.

No page in the manual or the API documentation describes a render pass, or any other picture a render produces, that reports whether a texture loaded. I searched hard for one; the record is below.

What Cycles' own files do show is a colour defined for images that are not found: opaque magenta.

> ```
> /* Color to use when images are not found. */
> #define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)
> ```
>
> — `Blender Sources/cycles_5.2/source/util/types_image.h`, lines 17–18

The same file names a state for a texture tile whose loading failed:

> ```
> /* Tile descriptor load status. Values lower than these mean the tile has been
>  * successfully loaded, and the value encodes where the tile is stored.
>  * - LOAD_NONE: Tile has never been accessed and is not in memory.
>  * - LOAD_REQUEST: Kernel requests this tile to be loaded.
>  * - LOAD_FAILED: Tile loading failed, will not try again.
>  */
> ```
>
> — `Blender Sources/cycles_5.2/source/util/types_image.h`, lines 114–119

and the image-lookup routine returns that magenta in four places, including the one that handles a failed tile load:

> ```
> ccl_device float4 kernel_image_interp(KernelGlobals kg,
>                                       ccl_private ShaderData *sd,
>                                       const int image_texture_id,
>                                       dual2 uv)
> {
>   if (image_texture_id == KERNEL_IMAGE_NONE) {
>     return IMAGE_MISSING_RGBA;
>   }
> ```
>
> — `Blender Sources/cycles_5.2/source/kernel/device/gpu/image.h`, lines 88–95

> ```
>     if (!kernel_tile_descriptor_loaded(tile_descriptor)) {
>       return (tile_descriptor == KERNEL_TILE_LOAD_FAILED) ? IMAGE_MISSING_RGBA : tex.average_color;
>     }
> ```
>
> — `Blender Sources/cycles_5.2/source/kernel/device/gpu/image.h`, lines 112–114

(The other two are at lines 124 and 167 of the same file, and one more at line 259 of `Blender Sources/cycles_5.2/source/kernel/util/image_3d.h`.)

**Inferred step, written out:** the files say that the image-lookup routine *returns* magenta with full alpha when the image is missing or its tile failed to load. Going from "the lookup returns this value" to "you can see magenta in the finished picture" requires assuming that the returned value flows on through the shader into the shaded result, which the files do not state. So: *if* a texture is used directly as a colour, the pixels that sample it would come out magenta, and that would show in Combined and in any colour-carrying pass. If the texture drives roughness, a bump, a mask or a mix factor, what you would see is not magenta but wrong shading. That is inference, not documentation.

**Which textures.** Even taken at face value, this tells you *which surfaces* are affected, not which files are missing. Nothing in the sources says a picture reports a file name.

There is one *documented* magenta-for-missing-media feature in 5.2, but it is not about scene textures — it is about strips in the Video Sequencer:

> "Show Missing Media
>
> Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent."

> "Tip
>
> Strips with missing content will be displayed as red in the timeline."

— Manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html — read 2026-07-30

I am reporting this only to be clear that it is *not* an answer to the question as asked: it concerns footage referenced by sequencer strips, not textures in a 3D scene, and the sidebar setting has a switch that turns the magenta off.

**Sources and addresses**

- Cycles' own files, `Blender Sources/cycles_5.2/source/util/types_image.h` and `Blender Sources/cycles_5.2/source/kernel/device/gpu/image.h` and `Blender Sources/cycles_5.2/source/kernel/util/image_3d.h` — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html — read 2026-07-30

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2. Blender 5.2.0 started from a terminal with no window and driven by a script. Set up: 32 by 32, one still, output OpenEXR, Cycles samples 1; a plainly green 8 by 8 PNG written to disk; the default cube given a material whose only shading is an Emission node fed by an Image Texture node pointing at that file, so the rendered colour is the texture's colour; Workbench's shading colour type set to `TEXTURE`; Metal switched on in the Cycles preferences, which reported `CYCLES DEVICES: [('Apple M1 Pro', 'CPU', True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`. Done: sixteen renders — four cases in four engines — each written to its own OpenEXR and the centre pixel read back. Blender printed:

```
--- CASE present (image slot='green.png', file exists=True, has_data=False) ---
RESULT present    CYCLES_CPU  centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    CYCLES_GPU  centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    EEVEE       centre RGBA = (0.0, 1.0, 0.0, 1.0)
RESULT present    WORKBENCH   centre RGBA = (0.0032, 0.1169, 0.0026, 0.998)
--- CASE missing (image slot='green.png', file exists=False, has_data=False) ---
RESULT missing    CYCLES_CPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    CYCLES_GPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    EEVEE       centre RGBA = (1.0, 0.0, 1.0, 1.0)
RESULT missing    WORKBENCH   centre RGBA = (0.1186, 0.0029, 0.1181, 0.998)
--- CASE emptyslot (image slot=None, file exists=False, has_data=False) ---
RESULT emptyslot  CYCLES_CPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  CYCLES_GPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  EEVEE       centre RGBA = (0.0, 0.0, 0.0, 1.0)
RESULT emptyslot  WORKBENCH   centre RGBA = (0.0955, 0.0941, 0.0951, 0.998)
--- CASE corrupt (image slot='green.png', file exists=True, has_data=False) ---
RESULT corrupt    CYCLES_CPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    CYCLES_GPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    EEVEE       centre RGBA = (1.0, 0.0, 1.0, 1.0)
RESULT corrupt    WORKBENCH   centre RGBA = (0.1186, 0.0029, 0.1181, 0.998)
```

The "corrupt" case was the same file replaced with the bytes `this is not a picture`. Cycles was run on the CPU and on the Metal graphics, and both gave the same numbers — so the CPU path this document could not read behaves like the graphics path here.

EEVEE and Workbench do substitute a colour, and it is the same magenta, for a missing file and for an unreadable one. The one case where they part is an empty image slot: Cycles still shows magenta, EEVEE shows black, and Workbench shows plain grey.

This version prints messages naming the file, and it says WARNING where the earlier versions said ERROR:

```
00:01.762  cycles           | WARNING Image file /tmp/images-hand-tests/out/t11/5.2.0_LTS/green.png does not exist.
00:01.762  cycles           | ERROR Failed to load 1 image files
00:02.062  gpu.texture      | ERROR Failed to create GPU texture from Blender image
00:02.383  cycles           | WARNING Image file  does not exist.
00:02.383  cycles           | ERROR Failed to load 1 image files
00:03.108  cycles           | WARNING Image file /tmp/images-hand-tests/out/t11/5.2.0_LTS/green.png failed to load.
00:03.108  cycles           | ERROR Failed to load 1 image files
00:03.398  image.read       | ERROR IMB_load_image_from_memory: unknown file-format (/tmp/images-hand-tests/out/t11/5.2.0_LTS/green.png)
00:03.398  gpu.texture      | ERROR Failed to create GPU texture from Blender image
```

The lines reading `Image file  does not exist.` with nothing between the words are the empty-slot case.

**Observed — the same test on a second machine and a different make of graphics card, 2026-08-06.** Run on the rented Linux machine, Blender 5.2.0 LTS, no window. The set-up is the Mac's, with one change: Apple's Metal graphics do not exist here, so the script asks Cycles what it has and then renders on this machine's card twice, once through CUDA and once through OptiX. It reported:

```
CYCLES DEVICES with CUDA: [('NVIDIA GeForce RTX 4090', 'CUDA', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', True), ('NVIDIA GeForce RTX 4090', 'OPTIX', True)]
```

Twenty renders were made — four cases in five engines. Blender printed:

```
--- CASE present (image slot='green.png', file exists=True, has_data=False) ---
RESULT present    CYCLES_CPU   centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    CYCLES_CUDA  centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    CYCLES_OPTIX centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    EEVEE        centre RGBA = (0.0, 1.0, 0.0, 1.0)
RESULT present    WORKBENCH    centre RGBA = (0.0032, 0.1171, 0.0026, 1.0)
--- CASE missing (image slot='green.png', file exists=False, has_data=False) ---
RESULT missing    CYCLES_CPU   centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    CYCLES_CUDA  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    CYCLES_OPTIX centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    EEVEE        centre RGBA = (1.0, 0.0, 1.0, 1.0)
RESULT missing    WORKBENCH    centre RGBA = (0.1188, 0.0029, 0.1183, 1.0)
--- CASE emptyslot (image slot=None, file exists=False, has_data=False) ---
RESULT emptyslot  CYCLES_CPU   centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  CYCLES_CUDA  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  CYCLES_OPTIX centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  EEVEE        centre RGBA = (0.0, 0.0, 0.0, 1.0)
RESULT emptyslot  WORKBENCH    centre RGBA = (0.0957, 0.0943, 0.0952, 1.0)
--- CASE corrupt (image slot='green.png', file exists=True, has_data=False) ---
RESULT corrupt    CYCLES_CPU   centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    CYCLES_CUDA  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    CYCLES_OPTIX centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    EEVEE        centre RGBA = (1.0, 0.0, 1.0, 1.0)
RESULT corrupt    WORKBENCH    centre RGBA = (0.1188, 0.0029, 0.1183, 1.0)
```

Both machines were run. **They agree on the colour in every case and every engine**, and the NVIDIA card gives the same answer both ways it can be used. **One number differs**: Workbench's alpha is `1.0` on this machine and `0.998` on the Mac, in all four cases. Nothing was read into that. [Citation check 2026-08-06: this read `0.9976`, which is 4.5's figure; the Mac printout above in this document reads `0.998`. Corrected to what the printout says. The checker re-ran the missing-texture script on the rented Linux machine and got `1.0` in all four cases, matching the Linux printout above.]

The wording this version prints came back exactly as on the Mac — the missing file is a `WARNING` and the summary line an `ERROR`:

```
00:04.058  cycles           | WARNING Image file /root/smoozy/Images/work/out/t11/5.2.0_LTS/green.png does not exist.
00:04.058  cycles           | ERROR Failed to load 1 image files
00:06.006  cycles           | WARNING Image file  does not exist.
00:08.759  cycles           | WARNING Image file /root/smoozy/Images/work/out/t11/5.2.0_LTS/green.png failed to load.
00:10.704  image.read       | ERROR IMB_load_image_from_memory: unknown file-format (/root/smoozy/Images/work/out/t11/5.2.0_LTS/green.png)
```

This machine adds two lines the Mac never printed:

```
00:01.073  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
00:05.663  gpu.texture      | ERROR Failed to create GPU texture from Blender image
```

The first is Cycles looking for AMD graphics that are not in this machine.

**Not covered.** How a render behaves in a case the test did not cover — a texture driving roughness or a mask rather than a colour — and which separate passes show the magenta. Also not covered: the CPU code path. The file `Blender Sources/cycles_5.2/source/kernel/image.h` (all twelve lines of it) reads:

> ```
> /* Header that can be included wherever image textures are used in the kernel,
>  * to make clangd happy. For GPU devices it's defined beforehand. */
> 
> #ifndef __KERNEL_GPU__
> #  include "kernel/device/cpu/image.h"  // IWYU pragma: export
> #endif
> ```

— and `source/kernel/device/cpu/` is not present in the folder supplied, so the sources cannot say the CPU path behaves the same. The hand test above does: Cycles on the processor and Cycles on the Metal graphics gave the same numbers in every case. Whether a *render* is what reports a missing texture at all is also answered there — a background render prints messages naming the file, alongside the magenta in the picture.

**Outside the interface.** **Observed.** The whole test above was run with no window open, and the substituted colour appeared in the written files just the same. So the substitution holds when Blender is driven from outside its own window. And a missing texture *does* produce console messages in a background render: the lines above name the missing or unreadable file by its full path. The Video Sequencer's "Show Missing Media" was not tested. (Previously, before the test: not stated either way.)

**Operating system.** **Observed on two machines and on two makes of graphics card, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro) with Apple's own Metal graphics, and Ubuntu 22.04.5 LTS with an NVIDIA GeForce RTX 4090 used both through CUDA and through OptiX.** The substituted colour is identical everywhere, and so is the wording of the warnings; what Blender prints gains two machine-specific lines on Linux, quoted above. **Windows is still untried** — no Windows machine has run it — and so is any card that is neither Apple's nor NVIDIA's. From the sources: they do not say, and the file layout alone does not establish equal behaviour across operating systems.

**Where I looked.** Manual-wide searches for `magenta` (6 hits — the sequencer one, two about green/magenta colour shift, one about anaglyph glasses, one about a loop-cut line), `pink` (16 hits, all about interface colours or anatomy), `missing file` (3 hits, all in `interface/window_system/topbar.txt` about the Report Missing Files / Find Missing Files tools, which are file-path repair tools and produce no picture), and a combined pattern for texture/image + "not found" / "fails to load" / "failed to load" / "could not be loaded" / "cannot be loaded" (no hits). Searched manual and API for `texture cache` and `texture ... load`. Read the whole Passes page looking for a pass of this kind — there is none. Searched Cycles' own files for `missing` (14 hits, of which the relevant ones are quoted above; the rest are about missing energy in the microfacet model, missing Fresnel factors, missing address-space qualifiers and missing intersections) and for `magenta` and the literal `1.0f, 0.0f, 1.0f` (two hits, both inside noise/wave texture code, unrelated). Read all of `types_image.h` and the relevant parts of `device/gpu/image.h`, `util/image_2d.h`, `util/image_3d.h`, `kernel/image.h`. Searched the 5.2 release notes for anything about missing textures — nothing.

---

## 3. Which file formats can Blender write a render out as?

**LABEL: documented.**

The manual gives the list outright:

> "This is the list of image file formats supported internally by Blender:"

and then a table whose rows are, with their channel depth, whether they carry alpha, whether they carry metadata, whether they carry DPI, and their extensions:

| Format | Channel Depth | Alpha | Metadata | DPI | Extensions |
| --- | --- | --- | --- | --- | --- |
| JPEG | 8bit | ✗ | ✓ | ✓ | .jpg .jpeg |
| OpenEXR | float 16, 32bit | ✓ | ✓ | ✓ | .exr |
| PNG | 8, 16bit | ✓ | ✓ | ✓ | .png |
| WebP | 8bit | ✓ | ✓ | ✗ | .webp |
| AVIF | 8, 10, 12bit | ✓ | ✗ | ✗ | .avif |
| BMP | 8bit | ✓ | ✗ | ✓ | .bmp |
| Cineon | 8, 10, 12, 16bit | ✓ | ✗ | ✗ | .cin |
| DPX | 8, 10, 12, 16bit | ✓ | ✗ | ✗ | .dpx |
| Iris | 8, 16bit | ✓ | ✗ | ✗ | .sgi .rgb .bw |
| JPEG 2000 | 8, 12, 16bit | ✓ | ✗ | ✗ | .jp2 .jp2 .j2c |
| Radiance HDR | float | ✓ | ✗ | ✗ | .hdr |
| Targa | 8bit | ✓ | ✗ | ✗ | .tga |
| Targa Raw | 8bit | ✓ | ✗ | ✗ | .tga |
| TIFF | 8, 16bit | ✓ | ✗ | ✓ | .tif .tiff |

(That table is my transcription of the plain-text rendering of the manual's table, which lists each column value on its own line; the values are exactly as they appear, including the duplicated ".jp2" in the JPEG 2000 row.)

The Python API's enumeration of what the file-format setting accepts adds two entries the manual's table does not have as rows of their own — the multi-layer variant of OpenEXR, and video:

> "OPEN_EXR_MULTILAYER:
>
> OpenEXR MultiLayer (.exr).
>
> Output image in multilayer OpenEXR format.
>
> FFMPEG:
>
> FFmpeg Video."

The default is PNG:

> "file_format
>
> File format to save the rendered images as (default 'PNG')"

In 5.2 the choice is split in two, by a "Media Type" setting:

> "Media Type
>
> The type of media to save.
>
> Image:
>
> Saves each input as a separate image file. Each input can use its own image format settings.
>
> Multi-Layer EXR:
>
> Saves all inputs together in a single multi-layer OpenEXR file.
>
> Video:
>
> Encodes each frame into a video container."

matching the API:

> "media_type
>
> The type of media to save (default 'IMAGE')
>
> Type: Literal['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']"

> "Blender uses FFmpeg to handle video encoding and decoding."

The manual pages named below describe the available video containers and codecs. Their full lists are not quoted here, so this finding does not claim an exact list of those choices.

**An important qualification, also documented:** not every one of these is present in every copy of Blender. The command-line page splits its list in two:

> "-F, --render-format <format>
>
> Set the render format. Valid options are: TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF.
>
> Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

**Documented** by way of a picture: the screenshot of the Output panel on the Output properties page (`_images/render_output_properties_output_panel.png`) shows a "File Format" dropdown reading "PNG", with rows for Color (BW / RGB / RGBA, RGBA selected), Color Depth (8 / 16, 8 selected) and Compression (15%), above them File Path, Overwrite, Placeholders, File Extensions and Cache Result. It does not show a "Media Type" row, although the text of the same page describes one — I report only what is visible in the picture.

**Sources and addresses**

- Manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/files/media/video_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html (text and the panel screenshot on it) — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy_types_enum_items/image_type_all_items.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html — read 2026-07-30

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, on the Blender 5.2.0 installed there, started from a terminal with no window. Set up: nothing built; a plain start with factory settings. Done: printed the build options, the output-format setting and its choices, the Media Type setting and its choices, and the FFmpeg container and codec choices. Blender printed:

```
VERSION: 5.2.0 LTS
BUILD_OPTIONS: bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=True, jack=False, pulseaudio=False, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)
FILE_FORMAT_DEFAULT: PNG
FILE_FORMAT_ITEMS: ['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
HAS_MEDIA_TYPE: True
MEDIA_TYPE_DEFAULT: IMAGE
MEDIA_TYPE_ITEMS: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']
HAS_USE_PREVIEW: True
FFMPEG_CONTAINERS: ['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']
FFMPEG_CODECS: ['NONE', 'AV1', 'H264', 'H265', 'WEBM', 'DNXHD', 'DV', 'FFV1', 'FLASH', 'HUFFYUV', 'MPEG1', 'MPEG2', 'MPEG4', 'PNG', 'PRORES', 'QTRLE', 'THEORA']
FFMPEG_AUDIO_CODECS: ['NONE', 'AAC', 'AC3', 'FLAC', 'MP2', 'MP3', 'OPUS', 'PCM', 'VORBIS']
```

This build has every optional image format compiled in. Note `codec_avi=False`, as in the documentation's own example, and `image_dds=True` even though DDS is not among the formats a render can be written as.

*Media Type comes first.* Setting `file_format` while `media_type` is `IMAGE` is refused for anything that is not a still image, with a message listing what is allowed:

```
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "NOSUCHFORMAT" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
```

So the sixteen names above are the whole set, but which of them the setting will take at any moment depends on the Media Type.

*AVIF from the command line.* This was one of the two things that did not line up. Done, in a terminal:

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=64" -F AVIF -o /tmp/images-hand-tests/out/t02/5.2.0_AVIF/out_ -f 1
```

It was accepted. Blender printed `Saved: '/tmp/images-hand-tests/out/t02/5.2.0_AVIF/out_0001.avif'`, the exit status was `0`, and `file` reported the written file as `ISO Media, AVIF Image`. So AVIF is reachable from the command line in this version, even though the manual's command-line page lists it in neither of its two lists.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 5.2.0 open. Looked at: Properties ▸ Output Properties ▸ Output panel, with the format changed in turn, and the video menus opened.

There are **two** controls, one above the other: a row labelled **`Media Type`** and, below it, a row labelled **`File Format`** whose entries carry their extensions in the label — `PNG (.png)`, `JPEG (.jpg)`, `TIFF (.tif)`, `OpenEXR (.exr)`, `AVIF (.avif)`.

**The panel's own heading now carries a checkbox.** The words `Output` at the top of the panel sit beside a ticked checkbox, which the panels in 4.5, 5.0 and 5.1 do not have. That is the switch this version's manual describes as deciding whether Blender writes render output from this panel at all.

**AVIF is offered in the window as well as on the command line.** With `AVIF (.avif)` chosen the panel shows `Color` (BW / RGB / RGBA), `Color Depth` (8 / 10 / 12), and `Quality 90%`.

With each format chosen, the panel shows: **PNG** — `Color` (BW / RGB / RGBA), `Color Depth` (8 / 16), `Compression 15%`. **JPEG** — `Color` (BW / RGB), `Quality 90%`. **TIFF** — `Color`, `Color Depth` (8 / 16), `Compression: Deflate`. **OpenEXR** — `Color`, `Color Depth` (Float (Half) / Float (Full)), `Codec: ZIP`, `Preview`. With `Media Type` set to `Multi-Layer EXR` the `File Format` row disappears from the panel altogether, leaving `Color Depth`, `Codec: ZIP`, `Interleave` and `Preview`.

With `Media Type` set to `Video`, the `Encoding` section's `Container` menu and the `Video` section's `Video Codec` menu were opened. Every entry, in the order shown:

- **Container**: `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`.
- **Video Codec**: `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`.

The rows under `Video` are `Video Codec`, `Color Depth`, `Color Space`, `Output Quality`, `Encoding Speed`, `Keyframe Interval`, `Max B-frames`.

Not captured: no picture shows the `Media Type` or `File Format` menus themselves opened in this version, so the full lists as the menus present them are not recorded here. The terminal record above gives the same sets of names.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.2.0 LTS, no window, factory settings. Blender printed:

```
VERSION: 5.2.0 LTS
BUILD_OPTIONS: bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=False, jack=True, pulseaudio=True, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)
FILE_FORMAT_DEFAULT: PNG
FILE_FORMAT_ITEMS: ['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
HAS_MEDIA_TYPE: True
MEDIA_TYPE_DEFAULT: IMAGE
MEDIA_TYPE_ITEMS: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']
HAS_USE_PREVIEW: True
FFMPEG_CONTAINERS: ['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']
FFMPEG_CODECS: ['NONE', 'AV1', 'H264', 'H265', 'WEBM', 'DNXHD', 'DV', 'FFV1', 'FLASH', 'HUFFYUV', 'MPEG1', 'MPEG2', 'MPEG4', 'PNG', 'PRORES', 'QTRLE', 'THEORA']
FFMPEG_AUDIO_CODECS: ['NONE', 'AAC', 'AC3', 'FLAC', 'MP2', 'MP3', 'OPUS', 'PCM', 'VORBIS']
```

Both machines were run and both agreed on everything that bears on this question, `AVIF` at the head of the list included. Three build options differ between the machines, and none is an image format: `coreaudio` is true on the Mac and false here, and `jack` and `pulseaudio` are false on the Mac and true here. [Citation check 2026-08-06: this sentence also said "and `xr_openxr` is false on the Mac and true here", which the Mac printout above contradicts — in this version it reads `xr_openxr=True` on both machines. Cut. The checker re-ran the same script on the rented Linux machine and got this document's Linux line back word for word.] **So the download for the operating system a render actually happens on carries every format this version's Mac build carries.**

*Asking for a format by name on the command line.* `AVIF` wrote a real file on this machine too: `Saved: '.../out_0001.avif'`, 747 bytes, reported by `file` as `ISO Media, AVIF Image`. `WEBP` and `OPEN_EXR_MULTILAYER` wrote their own formats, and only `NOSUCHFORMAT` was refused, with `Error: Format from '-F / --render-format' not known or not compiled in this release.` followed by a PNG. Exit status `0` every time.

*Asking for a format by name from Python.* `AVIF` was accepted — `ASSIGNMENT OK; file_format is now: AVIF` — and the still was written as `out_.avif`. The unknown name gave:

```
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "NOSUCHFORMAT" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
```

*What the container and codec lists mean.* A further test, recorded in full under question 4, chose each container in turn and read the codec list back: all ten containers offer all seventeen codecs after being chosen, and all 170 pairings can be set. So the lists above are what Blender offers, not what it will accept — several pairings fail at the moment of encoding, and this version accepts one pairing the three before it refuse.

**Not covered.** One of the two things that did not line up is settled above — AVIF works from the command line despite its absence from that page's lists. The other, `image_dds=True` while DDS is not a writable format, is untouched by these tests.

The two, as the sources leave them: first, AVIF appears in the manual's table and in the API's enumeration, but appears in *neither* list on the command-line page — not in the always-available list and not in the may-not-be-compiled-in list. Second, the API's build-options constant includes `image_dds=True`, yet DDS is not among the formats you can write to; what that means is not stated anywhere I could find. Also not covered: which container and codec combinations are legal (the manual says only "Not all codecs are available in all containers" and "Some containers and codecs are not compatible with each other"), and what the full set of stereo-3D "views formats" is.

**Outside the interface.** **Observed.** Everything above was done with no window open: `-F` chose the format from the command line and wrote the file, AVIF included; the settings were read and written from Python in a windowless Blender. The one thing to know before scripting against it is the order — Media Type first, then the format. Documented alongside: the command-line page gives `-F, --render-format <format>` and the two-tier list quoted above, which does not include every format named earlier in this answer. One further difference is documented:

> "By default, animation renders are saved automatically, while still renders are not. Still images must be saved manually from the Image Editor or written using a File Output Node."

So a single-frame render driven from outside will not write a file unless the run asks for it (the API's render operator has a `write_still` parameter, "Write Image, Save the rendered image to the output path (used only when animation is disabled)") or a File Output node is set up.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The official build for each system has every optional image format compiled in, the format lists match exactly, and AVIF works from the command line on both. **Windows is still untried** — no Windows machine has run it — so what a build there carries is not settled. From the sources: the command-line page's second list is exactly a statement that availability varies by build, "not available on all systems". For video, the manual adds:

> "Because codecs are required for both encoding and decoding, they must be available on both the system creating the file and the device playing it."

The sources do not say which formats are missing on which operating system.

**Where I looked.** `files/media/image_formats.txt` and `files/media/video_formats.txt` in full; `render/output/properties/output.txt`, `render/output/properties/format.txt`, `render/output/introduction.txt` in full; `advanced/command_line/arguments.txt` (format section); the API enum page `image_type_all_items.txt` and the sibling enum pages `image_color_depth_items`, `image_color_mode_items`, `exr_codec_items`, `views_format_*`; `bpy.types.ImageFormatSettings`; `bpy.app` for `build_options`. Searched the manual for `image formats` (9 hits) and `OpenEXR` (18 files). Opened the Output panel screenshot.

---

## 4. What happens when a file asks for a format that Blender cannot write?

**LABEL: not in the sources.**

I could not find any statement, anywhere in the 5.2 manual, the 5.2 API documentation, the 5.2 release notes or Cycles' own files, about what Blender does when it is asked to write a format it cannot write — whether it refuses, reports an error, silently substitutes another format, or writes nothing.

The question is slightly ambiguous and I answered the reading I think is meant: *a blend-file (or a run) specifies an output format that this copy of Blender is unable to produce.* The other reading — *the output path carries a file extension for a format Blender cannot write* — is equally unaddressed; I searched for that too and found nothing.

What the sources *do* establish, none of which answers the question:

**That the situation is real.** Availability varies from build to build:

> "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30

**That you can ask which formats this copy has.** The API documents a constant that reports what was compiled in:

> "Constant value bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, …)"

— API, https://docs.blender.org/api/5.2/bpy.app.html — read 2026-07-30

Note `codec_avi=False` in that example value: the AVI codec support is off in the build the documentation was generated from.

**That readable and writable are separate properties of a format.** New in 5.2, the image-buffer module exposes both:

> "has_read_file
>
> True when images of this file type can be read from a file."

> "has_write_file
>
> True when images of this file type can be written to a file."

> "has_write_memory
>
> True when images of this file type can be written to memory."

— API, https://docs.blender.org/api/5.2/imbuf.types.html — read 2026-07-30

The 5.2 release notes record that this is new:

> "The image-buffer API can now be used to convert between different file formats as well as direct pixel access using the buffer protocol."
>
> "- Support for reading/writing different file types, as well as known file-type extensions."

— Release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30

**That some near-neighbour failures do produce errors.** The manual describes what happens when there is no *output method*, which is a different thing from an unwritable format:

> "When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30

and it warns generally about mismatched containers and codecs, without saying what the failure looks like:

> "Some containers and codecs are not compatible with each other, so if you are getting errors check that your container and codec are compatible."

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.2.0, started from a terminal with no window. Three cases were tried.

*A name the command line will not take.* Set up: nothing built; the factory startup scene at 64 by 64, its own output format left at the default PNG. Done, in a terminal:

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=64" -F NOSUCHFORMAT -o /tmp/images-hand-tests/out/t02/5.2.0_NOSUCHFORMAT/out_ -f 1
```

Blender printed:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
...
00:00.917  render           | Saved: '/tmp/images-hand-tests/out/t02/5.2.0_NOSUCHFORMAT/out_0001.png'
00:00.917  render           | Time: 00:00.53 (Saving: 00:00.08)
```

The exit status was `0`. One file was written, `out_0001.png`, which `file` reported as `PNG image data, 64 x 64, 8-bit/color RGBA, non-interlaced` — the format already set in the file. So the render carries on and nothing is substituted; the message goes to the console, on a plain line rather than through the logging format.

*The same request from Python.* Set up: nothing built. Done: `bpy.context.scene.render.image_settings.file_format = 'NOSUCHFORMAT'`, then a still rendered with `bpy.ops.render.render(animation=False, write_still=True)`. Blender printed:

```
ASSIGNMENT RAISED: TypeError
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "NOSUCHFORMAT" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
file_format after attempt: PNG
OPERATOR RESULT: {'FINISHED'}
```

*A request the codec refuses.* Set up: the factory startup scene, `image_settings.media_type = 'VIDEO'`, `image_settings.file_format = 'FFMPEG'`, `ffmpeg.format = 'MPEG4'`, `ffmpeg.codec = 'H264'`, resolution 101 by 101, frames 1 to 3. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
00:00.390  video.write      | ERROR Couldn't initialize video codec: Generic error in an external library
Traceback (most recent call last):
  File "/private/tmp/images-hand-tests/scripts/t04_video_size.py", line 24, in <module>
    res = bpy.ops.render.render(animation=True)
RuntimeError: Error: width not divisible by 2 (101x101)
```

and, on its own report line, `Error: width not divisible by 2 (101x101)`. The output folder was empty afterwards. The process exit status was `0` — the Python error did not change it, since `--python-exit-code` was not given.

*The case that could not be produced.* The reading the question was written for — a blend-file asking for a real format that this copy of Blender lacks — could not be made to happen on this machine. The build-options line recorded under question 3 shows every optional format compiled in. That case is recorded as not runnable in `Audits/Blender Research/Images/Hand tests.md`, with the reason: it needs a Blender build made without one of those formats, and both machines the project has carry official builds with every one of them. The Linux run of 2026-08-06 checked the second machine and found the same, so the case is not runnable on either.

**Observed — the same three cases on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.2.0 LTS, no window. All three were repeated with the same scripts and the same commands, and all three came back the same as on the Mac: `Error: Format from '-F / --render-format' not known or not compiled in this release.` on a plain line followed by a PNG and exit status `0`; the same `TypeError` from Python with the same list of names; and, for the 101 by 101 video, `00:00.432  video.write      | ERROR Couldn't initialize video codec: Generic error in an external library` then `RuntimeError: Error: width not divisible by 2 (101x101)`, an empty output folder and exit status `0`.

**Observed — a request Blender itself refuses: every container-and-codec pairing the lists offer, 2026-08-06.** This case could not be produced on the Mac run because nobody had tried encoding with a pairing; only the lists had been read. Set up: the factory startup scene at 64 by 48, frames 1 to 3, Media Type `VIDEO`, format `FFMPEG`. Done: each container chosen in turn and its codec list read back; then all 170 pairings assigned; then three frames really encoded for every container with `H264`, and for the `MPEG4` container with every codec. **The same script was run on both machines** — on the rented Linux machine and on the Creator's own MacBook Pro (M1 Pro, macOS 26.5.2) — **and the two agreed on all 104 encodes.**

Nothing is refused at the setting: every container offers the same seventeen codecs once chosen, and Blender printed `PAIRINGS THE SETTING REFUSED: []`.

The encoder refuses a great deal. The wording that answers this question is:

```
RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
```

It came back for the WebM container with H.264, and for the MPEG-4 container with No Video, Flash Video, HuffYUV, ProRes, QuickTime Animation and Theora. In each of those cases a file of zero or a few hundred bytes was left behind. Three further refusals are about the picture rather than the pairing:

```
RAISED RuntimeError: Error: Render width has to be 720 pixels for DV!
RAISED RuntimeError: Error: Invalid tile sizing: frame not large enough to fit specified tile arrangement.
RAISED RuntimeError: Error: ff_frame_thread_encoder_init failed
```

— the first for the DV container and for the DV codec, the second for AV1, the third for DNxHD.

**One pairing behaves differently in this version.** MPEG-4 with FFV1 is refused in 4.5, 5.0 and 5.1 and **succeeds here**, writing `m_0001-0003.mp4` at 4171 bytes. Both machines agreed on that too.

**The Mac's own words for this test, added 2026-08-06.** The check of this topic ruled that the answer above said the two machines agreed without ever quoting what the Mac printed, which is an assertion rather than a record. So the same script was run again on the Creator's own MacBook Pro — Apple M1 Pro, macOS 26.5.2 — in this version, with its settings pointed at a folder of its own. Eleven of the twenty-six encodes were refused, and these are the lines that came back, verbatim:

```
WEBM/H264      RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
DV/H264        RAISED RuntimeError: Error: Render width has to be 720 pixels for DV!
MPEG4/NONE     RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/AV1      RAISED RuntimeError: Error: Invalid tile sizing: frame not large enough to fit specified tile arrangement.
MPEG4/DNXHD    RAISED RuntimeError: Error: ff_frame_thread_encoder_init failed
MPEG4/DV       RAISED RuntimeError: Error: Render width has to be 720 pixels for DV!
MPEG4/FLASH    RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/HUFFYUV  RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/PRORES   RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/QTRLE    RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/THEORA   RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
```

Every other encode wrote a file. This matches the rented Linux machine's result exactly, refusal for refusal and wording for wording.

**Not covered.** The one reading the question was written for — a real format this build lacks — is still unanswered, and now for a stronger reason: both machines the project has were checked, and the official build for each has every optional format compiled in, so the case cannot be produced on either. Also still unanswered: what a *newer* blend-file's unknown format value does when opened in this version, and whether any of this differs on Windows.

**Outside the interface.** **Observed.** All three cases above were run with no window open, and that is where the answers came from — which is the way it matters most. A format name the command line will not take is reported on one line and then ignored, the render carrying on in whatever format the file already held, with an exit status of `0`. A request the codec refuses stops that render, writes no file, and still leaves the exit status at `0`. (Previously, before the test: unknown.) The manual does say elsewhere that the console shows some of the encoding process, and more with `blender -d`:

> "When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d."

but that is about encoding progress, not about an unwritable format.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The same lines, the same refusals and the same exit statuses came back on both, including every one of the container-and-codec refusals and this version's one acceptance that the earlier three refuse. **Windows is still untried** — no Windows machine has run it — and that gap sits directly on this question, since builds vary by system and that is the reason the situation exists at all. From the sources: nothing, beyond the general statement quoted above that some formats are "not available on all systems"; what the two machines show is that neither of them lacks one.

**Where I looked.** Searched manual and API together for: `unsupported`, `not supported`, `cannot be saved`, `unable to save`, `cannot write`, `cannot be written`, `not writable`, `write ... not supported`, `invalid format`, `unknown format`, `format is not`, `falls back` / `fall back to PNG` / `defaults to PNG`, `compiled in` / `not compiled` / `build option`, `codec ... not available`, `container ... not available`, `reports an error`, `raises an error`, `will fail`, `fails to save`, `fails to write`. Read in full: `render/output/properties/output.txt`, `render/output/animation.txt`, `files/media/image_formats.txt`, `files/media/video_formats.txt`, `files/blend/compatibility.txt`, `advanced/command_line/arguments.txt` (format and render sections), `advanced/command_line/render.txt`, `imbuf.txt`, `imbuf.types.txt`, `bpy.types.ImageFormatSettings.txt`. Checked the operator documentation for `bpy.ops.render.render` (marked "Undocumented, consider contributing") and `bpy.ops.image.save_as`. Listed and skimmed the `troubleshooting/` section of the manual — it covers crashes, startup, GPU, network and Python, and says nothing about output formats. Searched all 5.2 release notes for format-related lines; the only relevant ones are a faster half-precision EXR writer and the imbuf additions quoted above. Ruled out the Cycles files: they are the render engine's own code and contain no image *output* format handling — writing render output is not Cycles' job.

---

## 5. Can a format hold several pictures inside one file, and if so, how can the finished picture be told apart from the others inside it?

**LABEL: documented**, with the closing detail **not in the sources**.

Yes — the sources establish that OpenEXR can do this in two different ways. They do not establish that no other format can.

**Several passes/layers in one file:**

> "ILM's OpenEXR has become a software industry standard for HDR image files, especially because of its flexible and expandable structure.
>
> An OpenEXR file can store multiple layers and passes. This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact."

That is what the "Multi-Layer EXR" media type writes:

> "Multi-Layer EXR:
>
> Saves all inputs together in a single multi-layer OpenEXR file."

and via the File Output node:

> "When using Multi-Layer EXR, each input is stored as a separate layer within the file."

> "Lists all input sockets of the node. The name of each input socket is used to construct the final file name."

The render cache uses the same shape:

> "Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image."

**Several views in one file:**

> "Multi-View OpenEXR
>
> Another option is to use multi-view OpenEXR files. This format can save multiple views in a single file and is backward compatible with old OpenEXR viewers (you see only one view though). Multi-view native support is only available to OpenEXR."

**How Blender lets a reader select among them.** The sources document three places, but they do not document the actual names written into the file.

In the Compositor:

> "Note
>
> Multi-Layer Format
>
> When a multi-layer file format, like EXR, is loaded, each layer is made available as a socket."

In the Image Editor: the Slot, View Layer and Render Pass selectors described in question 1.

In Python, the image user carries an index into each of the three:

> "multilayer_layer
>
> Layer in multilayer image (in [0, 32767], default 0, readonly)"
>
> "multilayer_pass
>
> Pass in multilayer image (in [0, 32767], default 0, readonly)"
>
> "multilayer_view
>
> View in multilayer image (in [0, 32767], default 0, readonly)"

Within Blender, question 1 establishes that the finished render pass is called **Combined**. It does not establish that `Combined` is the exact identifier written into an OpenEXR file.

**What is not in the sources:** the actual names written into the OpenEXR file. Nothing in the manual, the API documentation or the release notes states the layer or channel naming convention Blender uses inside a multi-layer EXR — not the separator, not the order, not whether the view-layer name is prefixed. So the sources do not establish how a program reading the file without Blender can identify the finished picture. The one thing that is documented about naming is on the writing side and only for the File Output node: "The name of each input socket is used to construct the final file name."

**Sources and addresses**

- Manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/input/image.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/stereoscopy/usage.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.ImageUser.html — read 2026-07-30

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.2.0, started from a terminal with no window. This is the answer to what is "not in the sources" above.

*A rendered multi-layer OpenEXR.* Set up: 32 by 32, one frame; the first view layer renamed to `Beauty` and a second added called `Second`; on both, `use_pass_combined`, `use_pass_z`, `use_pass_normal`, `use_pass_mist`, `use_pass_diffuse_color` and `use_pass_emit` switched on; `image_settings.media_type = 'MULTI_LAYER_IMAGE'` and then `file_format = 'OPEN_EXR_MULTILAYER'`; `use_exr_interleave` left at its default, which printed as `False`. The render engine was the factory default, which this version reports as `BLENDER_EEVEE`, so the names below are the ones that engine writes. Done: `bpy.ops.render.render(animation=True)`, then the written file's own bytes read with a short header reader (see `Audits/Blender Research/Images/Hand tests.md`). The reader printed:

```
BYTES: 15348
MAGIC: 0x01312f76
VERSION FIELD: 0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)
--- PART 0 ---
  BlenderMultiChannel      string           'Blender V2.55.1 and newer'
  Camera                   string           'Camera'
  Date                     string           '2026/08/03 22:42:44'
  File                     string           '<untitled>'
  Frame                    string           '1'
  RenderTime               string           '00:00.97'
  Scene                    string           'Scene'
  Software                 string           'Blender 5.2.0 LTS'
  Time                     string           '00:00:00:01'
  channels                 chlist           Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT)
  chunkCount               int              2
  colorInteropID           string           'lin_rec709_scene'
  compression              compression      3
  dataWindow               box2i            (0, 0, 31, 31)
  displayWindow            box2i            (0, 0, 31, 31)
  lineOrder                lineOrder        0
  name                     string           'Beauty.Combined'
  pixelAspectRatio         float            1.0
  screenWindowCenter       v2f              (0.0, 0.0)
  screenWindowWidth        float            1.0
  type                     string           'scanlineimage'
  xDensity                 float            72.0
```

There were twelve parts in all — one for each pass of each view layer. Their `name`, `channels` and `colorInteropID` lines, in the order written:

```
  name 'Beauty.Combined'        channels Beauty.Combined.A/B/G/R              colorInteropID 'lin_rec709_scene'
  name 'Beauty.Depth'           channels Beauty.Depth.Z                       colorInteropID 'data'
  name 'Beauty.Mist'            channels Beauty.Mist.Z                        colorInteropID 'data'
  name 'Beauty.Normal'          channels Beauty.Normal.X/Y/Z                  colorInteropID 'data'
  name 'Beauty.Diffuse Color'   channels Beauty.Diffuse Color.B/G/R           colorInteropID 'lin_rec709_scene'
  name 'Beauty.Emission'        channels Beauty.Emission.B/G/R                colorInteropID 'lin_rec709_scene'
  name 'Second.Combined'        channels Second.Combined.A/B/G/R              colorInteropID 'lin_rec709_scene'
  name 'Second.Depth'           channels Second.Depth.Z                       colorInteropID 'data'
  name 'Second.Mist'            channels Second.Mist.Z                        colorInteropID 'data'
  name 'Second.Normal'          channels Second.Normal.X/Y/Z                  colorInteropID 'data'
  name 'Second.Diffuse Color'   channels Second.Diffuse Color.B/G/R           colorInteropID 'lin_rec709_scene'
  name 'Second.Emission'        channels Second.Emission.B/G/R                colorInteropID 'lin_rec709_scene'
```

So: the file is a multi-part file, each part named `<view layer>.<pass>`, each channel named `<view layer>.<pass>.<channel>`. The finished picture of the first view layer is the part named `Beauty.Combined`. That is the name a program outside Blender needs.

*A multi-layer OpenEXR written by a File Output node.* Set up: the same scene at 32 by 32, main output PNG; a File Output node with `directory` set to the folder, `file_name = "fromnode_"`, `format.media_type = 'MULTI_LAYER_IMAGE'`, `format.file_format = 'OPEN_EXR_MULTILAYER'`, its items cleared, and two items added with `file_output_items.new('RGBA', "MyBeautyLayer")` and `file_output_items.new('RGBA', "second socket")`, both fed from the Render Layers node's Image output. Done: the same render call. Blender printed `Saved: '/tmp/images-hand-tests/out/t08/5.2.0_LTS/fromnode_0001.exr'`, and the header reader printed:

```
VERSION FIELD: 0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)
  channels                 chlist           MyBeautyLayer.A(FLOAT); MyBeautyLayer.B(FLOAT); MyBeautyLayer.G(FLOAT); MyBeautyLayer.R(FLOAT)
  name                     string           'MyBeautyLayer'
  channels                 chlist           second socket.A(FLOAT); second socket.B(FLOAT); second socket.G(FLOAT); second socket.R(FLOAT)
  name                     string           'second socket'
```

So a layer written by that node takes its socket's name, spaces and all, and there is no view-layer part in front of it.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 5.2.0 open and the multi-layer OpenEXR from the test above loaded in the Image Editor. Seen: two dropdown menus in the header. The first, opened, shows `Beauty` and `Second` above a greyed label reading `Layer`. The second, opened, shows `Combined`, `Depth`, `Normal`, `Mist`, `Emission`, `Diffuse Color` above a greyed label reading `Pass`. Those are the names a person sees, and they are the two halves of the part names inside the file — `Beauty.Combined`, `Beauty.Depth`, `Beauty.Mist`, `Beauty.Normal`, `Beauty.Emission`, `Beauty.Diffuse Color` — with the view-layer half shown as its own menu rather than spelled out.

**Not captured: the View menu.** Only two menus were photographed, the one labelled `Layer` and the one labelled `Pass`. No screenshot shows a menu for views, so nothing is recorded here about one, and nothing should be read into its absence from these pictures.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with Blender 5.2.0 LTS, no window, driven by the same scripts.

*A rendered multi-layer OpenEXR.* The reader printed the same version field, `0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)`, and the same twelve parts, each with its own `name` — `Beauty.Combined`, `Beauty.Depth`, `Beauty.Mist`, `Beauty.Normal`, `Beauty.Diffuse Color`, `Beauty.Emission`, and the same six for `Second` — with a `Software` attribute reading `'Blender 5.2.0 LTS'` and `colorInteropID` on every part.

*A multi-layer OpenEXR written by a File Output node.* The same file name, `fromnode_0001.exr`, and the same two parts named `MyBeautyLayer` and `second socket`.

Both machines were run and both agreed, name for name.

**Observed — several views inside one file, on the rented Linux machine, 2026-08-06.** A case neither machine had tried before. Set up: 32 by 32, one frame, the depth and normal passes on, `render.use_multiview = True` and `render.views_format = 'STEREO_3D'`, output `OPEN_EXR_MULTILAYER`. Blender reported `VIEWS: [('left', True), ('right', True)]`, and with the image settings' own `views_format` left at its default it wrote **two** files, `frame_0001_L.exr` and `frame_0001_R.exr`. With `image_settings.views_format = 'MULTIVIEW'` it wrote **one**, and the reader printed six parts, two per pass, the first reading:

```
--- PART 0 ---
  channels                 chlist           Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT)
  name                     string           'Beauty.Combined-left'
```

So a view is a suffix on the part's name, `<view layer>.<pass>-<view>`, and it is not in the channel names at all.

**Observed — the window on the rented Linux machine, 2026-08-06.** Blender 5.2.0 LTS was opened with a window on that machine's own display and photographed with `bpy.ops.screen.screenshot`.

With the ordinary multi-layer OpenEXR loaded, the Image Editor's header holds **two** picture-choosing menus, reading `Beauty` and `Combined`, and no third one. So there is no View menu to photograph for such a file, and the Mac's silence was not a missed picture.

With the multi-view file loaded, a stereo-glasses button appears in the header and the two views are shown together, still with only two menus. Switching that button off — `space.show_stereo_3d = False` — makes a **third** menu appear, after `Layer` and `Pass`, reading `left`. That is the View menu.

**Its contents could not be photographed open.** The pointer was put on it with `xdotool` in Blender 4.5 and Blender drew the control's own tooltip, which reads `Select View.` — so Blender saw the pointer, and that is the control's name. But no button press ever reached Blender, on that machine, with several topics running windowed Blenders on the one display at the same moment. What the file itself records is `left` and `right`, quoted above.

Separately, the Image Editor's own `View` menu in the header — the one beside `Image` — was opened with `bpy.ops.wm.call_menu(name="IMAGE_MT_view")` and photographed. It holds `Toolbar`, `Sidebar`, `Tool Settings`, `Asset Shelf`, `Adjust Last Operation`, `Update Automatically`, `Show Metadata`, `Frame All`, `Center View to Cursor`, `Zoom`, `Area`, and nothing in it chooses which picture is shown.

**Not covered.** The exact naming inside the file is settled by the hand test above; the sources still do not state it. Also: whether a multi-layer EXR can hold layers *and* views at once; how the multi-part vs interleaved storage choice changes the file's structure (the setting is documented — "Use legacy interleaved storage of views, layers and passes for compatibility with applications that do not support more efficient multi-part OpenEXR files" — but its effect on naming is not); and the fact, documented but easy to trip over, that the Video Sequencer will not read these files at all:

> "The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor."

**Outside the interface.** **Observed.** Both files above were written with no window open, and their names were read off the files themselves rather than out of Blender, so the naming holds for anything driving Blender from outside — which is exactly the case the gap mattered for. Documented alongside: `OPEN_EXR_MULTILAYER` is in the file-format setting and on the command line's `-F` list. On the reading side, the Python interface still exposes only indices, but with the names above a program outside Blender does not need it.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The same part names, the same multi-part shape and the same File Output node file name came back on both. The multi-view results and the window pictures above were got on the Linux machine alone, so they settle that system and leave the Mac untried for those. **Windows is still untried** — no Windows machine has run it. From the sources: OpenEXR is one of the formats the command-line page lists as "not available on all systems", so on a build without it the question would not arise.

**Where I looked.** `files/media/image_formats.txt` (OpenEXR section in full), `render/output/properties/output.txt`, `compositing/types/output/file_output.txt`, `compositing/types/input/image.txt`, `compositing/types/input/scene/render_layers.txt`, `render/output/properties/stereoscopy/introduction.txt` and `usage.txt` in full, `editors/image/introduction.txt`, `editors/image/image_settings.txt`. Searched the manual for `multi-layer`, `multilayer`, `multi-view`, `Combined`. Searched the API for `multilayer` and read `bpy.types.ImageUser`. Searched manual and API for a channel-naming convention using `Combined` in quotes, `channel_id`, `fullname` — `bpy.types.RenderPass` does expose `channel_id`, `fullname` and `name`, but all three are documented with an empty description, so they tell me nothing.

---

## 6. Does a format that can hold more light than a screen can show also record the treatment it was made under?

**Ambiguity, flagged.** "The treatment it was made under" has two readings. I answer the one I think is meant — *the colour management applied when the picture was made: the view transform, look, exposure and so on* — and I also answer the narrower reading, *the colour space*, because the sources separate them cleanly and the answers differ.

**LABEL: documented** for Blender's general colour-space-metadata behaviour; **not in the sources** for whether each named HDR-capable format records that metadata, and for the view transform and look.

The formats in question. The manual calls OpenEXR an HDR format outright ("ILM's OpenEXR has become a software industry standard for HDR image files"), and Radiance HDR the other one ("Since Radiance had the first (and for a long time the only) HDR image format…"). Two display formats can also carry HDR in 5.2:

> "HDR images can be written as AVIF or PNG files. They are written with 203 nits diffuse white to match the convention in most browsers and image viewers."

**Blender generally saves colour-space metadata — documented. Whether each named HDR-capable format does so is not established.**

> "By default, Blender integrates with automatic color management in operating systems and saves images and videos with color space metadata. This means that usually, images viewed and saved in Blender look the same when opened in video players and browsers on the same computer."

— Manual, https://docs.blender.org/manual/en/5.2/render/color_management/system_configuration.html — read 2026-07-30

The colour-management page for authors of OpenColorIO configurations confirms it is metadata written on save, and that how well it works depends on the configuration:

> "Include the interop ID from the Color Interop Forum for every color space and display color space that you can. This helps save image and video with correct colorspace information."

> "The icc_profile_name interchange attribute is supported, to embed ICC profiles when saving images."

> "Saving colorspace metadata for images and video also depends on this."

— Manual, https://docs.blender.org/manual/en/5.2/render/color_management/opencolorio.html — read 2026-07-30

**The format table distinguishes them.** It marks OpenEXR with a ✓ in its Metadata column and Radiance HDR with a ✗. It also marks AVIF ✗ and PNG ✓. The manual states the general limitation:

> "Not all formats support metadata, and some may only support a limited subset of fields.
>
> When using formats that do not support metadata, render information is not preserved in the saved file."

**Whether the treatment itself is recorded is not in the sources.** Nothing says the view transform, the look, the exposure or the gamma used at render time is written into the file. Two passages bear on the question, but neither establishes a general answer:

First, the render's own colour management is deliberately *not* applied to an OpenEXR:

> "For intermediate image files in production, it is recommended to use OpenEXR files. These are always stored in scene linear color spaces, with high precision to avoid data loss."

> "Save As Render
>
> Save image with render color management. For display image formats like PNG, apply view and display transform. For intermediate image formats like OpenEXR, use the default render output color space."

Second, Blender cannot tell from an OpenEXR that it *is* a render:

> "However, when loading a render saved to an intermediate OpenEXR file, Blender cannot detect automatically that this is a render (it could be e.g. an image texture or displacement map). We need to specify that this is a render and that we want the transformations applied, with the following settings.
>
> View as Render
>
> Display the image data-block (not only renders) with view, exposure, gamma, RGB curves applied."

— Manual, https://docs.blender.org/manual/en/5.2/render/color_management/displays_views.html — read 2026-07-30

That last passage is the closest the sources come to answering the question. It says the reader has to specify that an opened OpenEXR is a render and ask for the display transformations.

Third, the listed Metadata panel options contain no view transform, look, display or exposure entry. That absence does not establish that no other metadata path records them.

**Sources and addresses**

- Manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/color_management/system_configuration.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/color_management/opencolorio.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/color_management/color_spaces.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/color_management/displays_views.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html — read 2026-07-30

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.2.0, started from a terminal with no window.

Set up: 32 by 32, one still each time, everything else left alone. Five files were written, changing one thing at a time: an OpenEXR with `view_settings.view_transform = "AgX"` and `look = "None"`; the same with `view_transform = "Standard"`; the same with `view_transform = "AgX"`, `look = "AgX - Punchy"`, `exposure = 2.0` and `gamma = 0.5`; a Radiance HDR; and a PNG. Blender printed:

```
DEFAULT view_transform='AgX' look='None' exposure=0.0 gamma=1.0
BLEND FILE COLORSPACE working_space = 'Linear Rec.709'
BLEND FILE COLORSPACE working_space_interop_id = 'lin_rec709_scene'
BLEND FILE COLORSPACE is_missing_opencolorio_config = False
```

Done: each written file's own bytes were read. The OpenEXR made under AgX gave:

```
VERSION FIELD: 0x00000002 (version 2, tiled=False longnames=False deep=False multipart=False)
--- PART 0 ---
  Camera                   string           'Camera'
  Date                     string           '2026/08/03 22:45:46'
  File                     string           '<untitled>'
  Frame                    string           '001'
  RenderTime               string           '00:00.48'
  Scene                    string           'Scene'
  Software                 string           'Blender 5.2.0 LTS'
  Time                     string           '00:00:00:01'
  channels                 chlist           A(FLOAT); B(FLOAT); G(FLOAT); R(FLOAT)
  colorInteropID           string           'lin_rec709_scene'
  compression              compression      3
  dataWindow               box2i            (0, 0, 31, 31)
  displayWindow            box2i            (0, 0, 31, 31)
  lineOrder                lineOrder        0
  name                     string           ''
  pixelAspectRatio         float            1.0
  screenWindowCenter       v2f              (0.0, 0.0)
  screenWindowWidth        float            1.0
  type                     string           'scanlineimage'
  xDensity                 float            72.0
```

So the general statement about colour-space metadata does cover OpenEXR here: the attribute is named `colorInteropID` and its value is `lin_rec709_scene`. Setting the Date and RenderTime lines aside, the three OpenEXR headers were the same text: the one made under Standard matched the one made under AgX, and so did the one made with a Look and with exposure and gamma changed. The only difference between those files was how many bytes of pixels followed the header. No attribute names a view transform, a look, an exposure or a gamma.

To check what `colorInteropID` follows, the working space was changed with `bpy.ops.wm.set_working_color_space(working_space='ACEScg', convert_colors=False)` and another OpenEXR written. Blender printed `working_space after: ACEScg interop id: lin_ap1_scene`, and that file's header reads `colorInteropID string 'lin_ap1_scene'`. So the attribute carries the working space, not the treatment.

The Radiance HDR file begins:

```
#?RADIANCE
FORMAT=32-bit_rle_rgbe

-Y 32 +X 32
```

and then the pixels. Nothing else is in its header — matching the format table's ✗ for metadata.

The PNG's blocks, read in order, were `IHDR, sRGB, gAMA, cHRM, eXIf, oFFs, pHYs, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, IDAT`, with `sRGB` reading `03`, `gAMA` reading `0000b18f` and `cHRM` reading `00007a26000080840000fa00000080e8000075300000ea6000003a9800001770`. Its seven text entries were `File = <untitled>`, `Date = 2026/08/03 22:45:46`, `Time = 00:00:00:01`, `Frame = 001`, `Camera = Camera`, `Scene = Scene`, `RenderTime = 00:00.33`. So a PNG written by this version carries colour blocks describing the display space it is in — and still nothing naming the view transform, look, exposure or gamma.

*Opening the files again.* Done, in a windowless Blender: `bpy.data.images.load(...)` on each file, then its colour space and View-as-Render state printed:

```
OPENED exr_agx.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_standard.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_punchy_exposed.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED radiance.hdr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED display_png.png size=(32, 32) depth=96 is_float=True colorspace='sRGB' use_view_as_render=False
```

All three OpenEXR files come back the same way whatever they were made under, with View as Render off.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.2.0 LTS, no window. The same files were written under the same changed settings and the same readers run over them. Blender printed `DEFAULT view_transform='AgX' look='None' exposure=0.0 gamma=1.0` and `BLEND FILE COLORSPACE working_space = 'Linear Rec.709'`, `working_space_interop_id = 'lin_rec709_scene'`, `is_missing_opencolorio_config = False`.

The three OpenEXR headers came back as the same text apart from the date and the render time: no attribute names a view transform, a look, an exposure or a gamma, and there is no `chromaticities`. Each carries `colorInteropID string 'lin_rec709_scene'`. After `bpy.ops.wm.set_working_color_space(working_space='ACEScg', convert_colors=False)`, which reported `working_space after: ACEScg interop id: lin_ap1_scene`, the next OpenEXR carried `colorInteropID string 'lin_ap1_scene'`.

The Radiance file begins `#?RADIANCE` and `FORMAT=32-bit_rle_rgbe` and holds nothing else. The PNG's blocks, read in order, carry the colour blocks:

```
PNG display_png.png CHUNKS: IHDR, sRGB, gAMA, cHRM, eXIf, oFFs, pHYs, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, IDAT, IEND
```

*Opening the files again.* Blender printed:

```
OPENED exr_agx.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_standard.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_punchy_exposed.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED radiance.hdr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED display_png.png size=(32, 32) depth=96 is_float=True colorspace='sRGB' use_view_as_render=False
```

Both machines were run and both agreed, line for line.

**Not covered.** Whether "saves images and videos with color space metadata" covers OpenEXR is settled above, and so is the form it takes: an attribute named `colorInteropID`. Still open: what happens with the "Override" colour-management option in the Output panel, which lets a file be saved under different settings from the scene's — whether the file then records the override. And the second reading of the question, if "treatment" meant something other than colour management: I found nothing recording render settings such as sample counts either, beyond the Metadata list above.

**Outside the interface.** **Observed.** Every file above was written, and every header read, with no window open — the view transform, the look, the exposure and the gamma were all set from a script, the working space was changed by an operator, and the renders were driven from a terminal. So a background render does write the colour-space attribute, and it does not write the treatment. (Previously, before the test: not stated.) The documented monitor, Wayland and Vulkan conditions concern viewing, not writing.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** A file written on either machine carries the same attributes, including `colorInteropID` and the PNG's colour blocks, and reopens the same way — which matters more here than elsewhere, because the sources frame this behaviour as integration with the operating system's own colour management, and the two systems come out the same. **Windows is still untried** — no Windows machine has run it. From the sources, and this is one place they are explicit: the colour-space metadata claim is framed as integration with the operating system's own colour management ("Blender integrates with automatic color management in operating systems"), and enabling wide gamut and HDR is spelled out per system:

> "macOS: Available on any Apple Silicon device.
>
> Linux: Use Wayland, and set the Vulkan backend in the Blender system preferences.
>
> Windows:
>
> For wide gamut, enable 'Automatically manage color for apps' in the Windows display settings.
>
> For HDR, enable 'Use HDR' in the Windows display settings."

Those are conditions for *viewing*, not for writing. The sources do not say the written file differs by operating system.

**Where I looked.** All five colour-management pages (`color_spaces.txt`, `displays_views.txt`, `opencolorio.txt`, `system_configuration.txt`, `index.txt`) — the first four read in full or in their save/write sections. `files/media/image_formats.txt` (Metadata section, format table, Save As Render, colour-space note). `render/output/properties/metadata.txt` in full. Searched the manual for `chromaticit` (7 hits, all definitional), `view transform` (25 hits, none about writing one into a file), `icc` (1 hit, quoted above), and searched the manual and API for `metadata`. Searched the 5.2 release notes for `color space` / `colorspace` / `color management` / `metadata` / `HDR`: the colour-management entries are about new colour spaces and a new menu organisation; the metadata entries are about stereoscopic and panoramic-camera metadata in *movies*, not colour. Checked `bpy.types.ImageFormatSettings` for anything that would carry the view transform into a file — it has `view_settings`, `display_settings` and `linear_colorspace_settings`, all marked readonly and all described as settings, not as things written out.

---

## 7. Can Blender turn a set of finished frames into a short moving picture?

**LABEL: documented.**

Yes, using the Video Sequencer.

> "Images can then be encoded to a video by adding the rendered image sequence into the Video Sequencer and choosing an appropriate Video Output."

The manual gives the whole procedure:

> "In Blender, open the Video Sequencer."
>
> "Choose Add Image from the add menu. Select all the frames from your output folder that you want to include in your animation. They will be added as a strip in the Sequence editor."
>
> "Scrub through the animation to check if you have included all the frames."
>
> "In the Output panel, choose the container and codec you want (e.g. MPEG H.264) and configure them."
>
> "Click the Animation render button and Blender will render out the Sequence editor output into a movie."

and recommends this over rendering straight to video:

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning. If you first render out a set of static images such as the default PNG format or the higher-quality OpenEXR (which can retain HDR pixel data), you can combine them as an Image Strip in the Video Sequencer."

> "You can then make a movie out of the separate frames with Blender's Sequence editor or use 3rd party encoding software."

The switch that makes the render read the Sequencer instead of the 3D scene is documented:

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera."

For loading the frames, the naming rule is documented:

> "To load image sequence in any of the supported image file formats, the filename of the images must contain a digit to indicate the frame order (e.g. *-0001.jpg, *-0002.jpg, *-0003.jpg, etc, of any image format), indicating the frame."

There is also a documented way to just *watch* a set of frames without making a file:

> "Rendered image sequences can be played back in the Animation Player."

**One documented exception.** Multi-layer EXR frames cannot go this route:

> "The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor."

**Sources and addresses**

- Manual, https://docs.blender.org/manual/en/5.2/render/output/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html — read 2026-07-30

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.2.0, started from a terminal with no window throughout. This is the answer to the "not established" line below.

*The whole job with no window at all.* Set up, in a windowless Blender: six PNG frames at 64 by 48 rendered from the ordinary scene into a folder; then the 3D scene changed so a fresh render could not be mistaken for those frames — the cube deleted and the world background set to pure blue; then an image strip built from the six saved files with `scene.sequence_editor_create()` and `strips.new_image(name="Frames", filepath=<first file>, channel=1, frame_start=1)` followed by `strip.elements.append(<each remaining file name>)`. Blender printed `STRIP: Frames type: IMAGE elements: 6` and `use_sequencer: True`. Four blend-files were saved from that state: video output with the Sequencer pipeline on, video output with it off, PNG output with it on, PNG output with it off.

Done, in a terminal:

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b -noaudio --factory-startup /tmp/images-hand-tests/out/t16/5.2.0_LTS/sequence-test.blend -a
```

Blender printed, for each frame, `Rendering frame N`, `Executing sequencer`, `Video append frame N`. The exit status was `0`, and one file appeared at the output path: `out_0001-0006.mp4`, which `file` reported as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. The same command on the file with the Sequencer pipeline switched off wrote `nosec_0001-0006.mp4`.

To see which pictures went into it, the same two files were rendered again with PNG output instead of video, and the written frames' pixels read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

With the Sequencer pipeline on, what came out is the saved frames — grey cube on dark grey. With it off, what came out is the changed 3D scene — blue, no cube. The six Sequencer frames took 0.5 seconds in total; the six 3D renders took about that each.

*A folder of frames with nothing prepared.* `blender -b --factory-startup -F FFMPEG -o <folder>/n_ -a` against the plain startup file wrote `n_0001-0003.mkv` — a movie of the 3D scene, not of any folder of pictures. There is no command-line argument that turns a folder of frames into a movie by itself.

*A frame missing from the set at the moment of encoding.* Set up: five frames written, a strip built naming all five, the file saved, and then the third frame's file deleted from disk. Done: the same background animation render, to PNG. All five output frames were written, nothing was printed about the missing one, and their pixels read:

```
s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3271
s_0002.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3271
s_0003.png   size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1247
s_0004.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3271
s_0005.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3271
```

The frame whose file was gone came out fully transparent black.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Set up: three multi-layer OpenEXR frames at 64 by 48 with the depth and normal passes on; a strip built from them; PNG output. Done: the same background animation render. Three files were written, with no message about the format:

```
s_0001.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3679
s_0002.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3679
s_0003.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3679
```

Compared with the same scene's PNG frames put through the same route, which gave `centre=(0.5373, ...)` and `corner=(0.2, ...)`, the picture is there but darker.

*How long the result is.* Set up: twenty-four frames, a strip built from all of them, video output with the MPEG-4 container and the H.264 codec. Done: encoded once with `render.fps = 24` and once with `render.fps = 12`, then each movie's own header read for the time scale and duration it records:

```
at24_0001-0024.mp4   bytes=2315   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4   bytes=2593   timescale=1000 duration=2000 -> 2.000 s
```

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 5.2.0 open. Seen: the Output panel's top control, labelled `Media Type`, offers `Image`, `Multi-Layer EXR` and `Video`; with `Video` chosen, the `Encoding` section's `Container` menu offers `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`, and the `Video Codec` menu offers `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`. So this build really does offer video output, and the container and codec used in the test above are both on those menus.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.2.0 LTS, with no window throughout, driven by the same scripts.

*The whole job with no window at all.* `blender -b -noaudio --factory-startup .../sequence-test.blend -a` printed `Executing sequencer` and `Video append frame 1` through `Video append frame 6` on timestamped lines, the exit status was `0`, and `out_0001-0006.mp4` appeared, reported by `file` as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. Rendered again to PNG, the frames read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

*A folder of frames with nothing prepared.* The same command wrote `n_0001-0003.mkv`, reported as `Matroska data` — a movie of the 3D scene.

*A frame missing from the set.* All five output frames were written, nothing was printed about the missing one, and the third came out fully transparent black: `s_0003.png size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1813`.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Three files written with no message about the format, `centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0)`.

*How long the result is.* The two movies' own headers record:

```
at24_0001-0024.mp4 bytes=2315   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4 bytes=2598   timescale=1000 duration=2000 -> 2.000 s
```

Both machines were run and both agreed on every one of these, with only the byte sizes of the written files differing by a few bytes.

*Which container and codec will actually take it.* A further test, recorded in full under question 4 and run on both machines, encoded three frames for every container with H.264 and for the MPEG-4 container with every codec. Nine of those pairings are refused with `Error: Could not initialize streams, probably unsupported codec combination`, and three more with a complaint about the picture size. So the two menus offer more pairings than this version will write — and this version writes one, MPEG-4 with FFV1, that 4.5, 5.0 and 5.1 refuse.

**Not covered.** That every documented step is written as an editor action still stands, but the hand test above shows the same job done without any of them. Also not covered: whether the frames must all be the same size or format (the manual does say a *batch* selection "can be a mix of file types (jpg, png, exr, etc.)"), how the frame rate of the resulting video is decided beyond the scene's Frame Rate setting, and what happens to frames that are missing from the middle of a sequence — except that the Video Sequencer has a Show Missing Media setting (question 2) and, separately, "Placeholder frames are rendered in pink in the VSE preview".

**Outside the interface.** **Observed — yes, the whole job.** Both halves were done with no window open: the image strip was built from the finished frames by a script in a windowless Blender, and the movie was encoded by `blender -b <file> -a` from a terminal, with Blender printing `Executing sequencer` for each frame. The written frames prove the pictures came from the strip and not from a fresh 3D render. (Previously, before the test: not established.)

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The whole job — building the strip with no window and encoding it from a terminal — works on both, and the frames, the file types and the recorded durations match. This is the machine a Smoozy render actually happens on, and FFmpeg is present in its build too (`codec_ffmpeg=True`, recorded under question 3). **Windows is still untried** — no Windows machine has run it. From the sources: video writing goes through FFmpeg, and the codec caveat quoted under question 3 applies ("they must be available on both the system creating the file and the device playing it").

**Where I looked.** `render/output/animation.txt` and `render/output/introduction.txt` in full; `render/output/properties/output.txt` (Encoding panel and Tips) in full; `render/output/properties/post_processing.txt` in full; `render/output/animation_player.txt`; `files/media/image_formats.txt` (Opening an Image Sequence); `files/media/video_formats.txt` in full. Searched the manual for `image sequence`, `Image Strip`, `encode`. Skimmed the video-editing and video-sequencer sections for the proxy and image-strip pages.

---

## 8. Can Blender be made to produce a smaller, plainly viewable copy of a picture a render produced?

**Ambiguity, flagged.** "Smaller" could mean fewer pixels or fewer bytes. The sources contain a feature for each, so I give both and say which is which.

**LABEL: documented** for the individual preview, proxy, resize and write features; **inferred** where separately documented resize and write operations are joined into one route; **not in the sources** for whether the automatic JPEG preview is actually smaller.

**(a) A JPEG copy alongside an EXR render — plainly viewable; whether it is smaller is not stated.** Among the OpenEXR render output options:

> "Preview
>
> When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview."

The API states it as a general property of the image format settings, not an OpenEXR-only one:

> "use_preview
>
> When rendering animations, save JPG preview images in same directory (default False)"

I report the discrepancy rather than resolving it: the manual documents this under "Available options for OpenEXR render output"; the API lists it on `ImageFormatSettings`, which every format shares. Nothing in either says the copy is reduced in pixel size or file size, so I make no claim that it is smaller. The manual calls it a "quick preview" and identifies it as JPEG.

**(b) Proxies — lower quality and/or resolution.** The sources describe them as copies stored at lower quality and/or resolution and say they are available for image sequences:

> "To combat this, Blender can generate proxies – copies of the original footage stored at a lower quality and/or resolution – to maintain a smooth editing experience without compromising visual fidelity in the end result."

> "You can use the Proxy tab if you want to configure proxies in more detail (or create proxies for image sequences)."

> "Resolutions
>
> The resolution(s) of the proxy videos to generate; multiple sizes can be selected."

> "By default, all generated proxy videos are stored to the folder <path of original footage>/BL_proxy/<clip name>, but this can be changed to a custom directory using this option."

The preview sizes offered are documented as 25%, 50%, 75% and 100%:

> "25%, 50%, 75%, 100%:
>
> Preview at a downscaled resolution, optionally using proxies (see below). Even selecting 100% can give a performance benefit due to the reduced image quality and corresponding smaller file size."

**(c) Scale down, then write out — smaller in pixels.** The Compositor has a Scale node:

> "This node scales the size of an image."

> "For instance X: 0.5 and Y: 0.5 would produce an image which width and height would be half of what they used to be."

and a File Output node that writes whatever is fed to it, in whatever format you pick:

> "The File Output node writes one or more images to disk during rendering or compositing."

> "Save intermediate results from anywhere in the node tree."

Putting the two together is not written down as such — that step is **inferred**, though it is a very short step: the Scale node produces a smaller image and the File Output node writes an image to disk in a chosen format.

**(d) Resize an image and save it — smaller in pixels.** In the Image Editor:

> "Resize
>
> Adjusts the image dimensions by scaling its pixel resolution."

> "Size X, Y
>
> Defines the new width and height of the image in pixels."

> "Save a Copy
>
> Save the file under a specified name, but keep the old one open in the Image editor."

with `bpy.ops.image.resize(*, size=(0, 0), all_udims=False)` as the Python equivalent, and, new in 5.2, the image-buffer module can do it outside any data-block:

> "resize(size, *, method='FAST')
>
> Resize the image in-place."
>
> "Parameters: size (tuple[int, int]) – New size. method (str) – Method of resizing ('FAST', 'BILINEAR')."

> "imbuf.write(image, *, filepath=None)
>
> Write an image."

As with (c), joining resize and write/save into one route is **inferred** from the separately documented operations.

**Sources and addresses**

- Manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/sequencer/sidebar/proxy.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/transform/scale.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/editors/image/editing.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/bpy.ops.image.html — read 2026-07-30
- API, https://docs.blender.org/api/5.2/imbuf.types.html and https://docs.blender.org/api/5.2/imbuf.html — read 2026-07-30

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.2.0, started from a terminal with no window.

*The Preview copy — (a).* Set up: 64 by 48, frames 1 to 3, `image_settings.use_preview = True`, first with `file_format = 'OPEN_EXR'` and then with `'PNG'`. Done: `bpy.ops.render.render(animation=True)` each time, then every written file's byte size and pixel size read. Blender printed:

```
--- OPEN_EXR, use_preview set to True, now reads True ---
  FILE anim_0001.exr                4129 bytes  64x48
  FILE anim_0001.jpg                1136 bytes  64x48
  FILE anim_0002.exr                4129 bytes  64x48
  FILE anim_0002.jpg                1136 bytes  64x48
  FILE anim_0003.exr                4129 bytes  64x48
  FILE anim_0003.jpg                1136 bytes  64x48
--- PNG, use_preview set to True, now reads True ---
  FILE anim_0001.png                3625 bytes  64x48
  FILE anim_0002.png                3625 bytes  64x48
  FILE anim_0003.png                3625 bytes  64x48
```

So the Preview copy has the **same number of pixels** as the main file and about a quarter of the bytes; it sits beside it with the same stem and a `.jpg` ending. And it is written for OpenEXR only: with PNG the setting can be switched on and reads back as on, and no JPEG copy appears — which settles the mismatch between the manual and the interface list in the manual's favour.

Then a blend-file was saved with OpenEXR output and Preview on, and one frame rendered from a terminal on the Creator's ordinary configuration (not factory settings):

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b -noaudio /tmp/images-hand-tests/out/t12/5.2.0_LTS/preview-test.blend -f 1
```

Blender printed `Saved: '.../CLI/cli_0001.exr'` and `Saved: '.../CLI/cli_0001.jpg'` — 2816 bytes and 1193 bytes.

*Scale into File Output — (c).* Set up: 64 by 48, one frame, main output PNG; a compositing node group with the Render Layers image going to the Group Output and also through a Scale node with X and Y set to 0.5 into a File Output node writing PNG into the same folder. This version's Scale node has no `space` setting; its Type input socket already read `Relative`. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
SCALE NODE inputs: ['Image', 'Type', 'X', 'Y', 'Frame Type', 'Interpolation', 'Extension X', 'Extension Y']
SCALE NODE has no space property; Type socket reads: 'Relative'
OPERATOR RESULT: {'FINISHED'}
FILE main_0001.png          2819 bytes  64x48  depth=32 is_float=False
FILE small_small_0001.png   1521 bytes  32x24  depth=128 is_float=True
```

So the joined route works, and it works with no window.

*Resize and save — (d).* Set up: one OpenEXR render at 640 by 360, loaded back with `bpy.data.images.load(...)`. Done, in order, with whatever each returned kept:

```
--- bpy.ops.image.resize(size=(320, 180)) ---
RAISED RuntimeError: Operator bpy.ops.image.resize.poll() failed, context is incorrect
--- image.scale(320, 180) ---
scale() returned, size now (320, 180)
--- bpy.ops.image.save_as(filepath='/tmp/images-hand-tests/out/t14/5.2.0_LTS/small.png', save_as_render=True, copy=True) ---
RAISED RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect
small.png exists: False
--- image.save_render('/tmp/images-hand-tests/out/t14/5.2.0_LTS/small_render.png') ---
save_render returned
--- scene format set to PNG, then image.save_render('/tmp/images-hand-tests/out/t14/5.2.0_LTS/small_as_png.png') ---
save_render returned
FILE big.exr              100897 bytes  640x360
FILE small_as_png.png      48451 bytes  320x180
FILE small_render.png     108578 bytes  320x180
```

The two Image Editor commands refuse to run with no window. The two plain calls work. One thing to note about the first `save_render`: although the path ends `.png`, `file` reports that file as `OpenEXR image data, version 2 ... dataWindow: (0 0)-(319 179)`, because `save_render` writes in the scene's own output format. After the scene's format was set to PNG, the second call wrote `PNG image data, 320 x 180, 16-bit/color RGBA, non-interlaced`.

*Proxies in a background run — (b).* Set up: three finished PNG frames at 128 by 96; an image strip built from them; `strip.use_proxy = True` and `strip.proxy.build_25 = True` with the other three sizes off; the sequence editor's proxy storage reading `PER_STRIP`; output PNG; the file saved; no `BL_proxy` folder present. Done, in a terminal: `blender -b .../proxy-test.blend -a`. Blender printed only the three `Saved:` lines for the output frames, the exit status was `0`, and no `BL_proxy` folder existed anywhere under the test folder afterwards.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 5.2.0 open. Looked at: Properties ▸ Output Properties ▸ Output panel, with the format changed in turn, watching for a control named `Preview`. Seen: with `PNG (.png)`, `JPEG (.jpg)`, `TIFF (.tif)` and `AVIF (.avif)` there is no `Preview` control anywhere in the panel. With `OpenEXR (.exr)` a checkbox reading `Preview` appears under `Codec`, unticked. With `Media Type` set to `Multi-Layer EXR` the same `Preview` checkbox appears, unticked, below one reading `Interleave`. So the mismatch this answer flags between the manual and the Python interface comes out in the manual's favour: the panel offers Preview only for the two OpenEXR cases, matching the terminal result above, where switching `use_preview` on for PNG produced no JPEG copies.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.2.0 LTS, no window, driven by the same scripts.

*The Preview copy.* Blender printed:

```
--- OPEN_EXR, use_preview set to True, now reads True ---
  FILE anim_0001.exr                4117 bytes  64x48
  FILE anim_0001.jpg                1136 bytes  64x48
  FILE anim_0002.exr                4117 bytes  64x48
  FILE anim_0002.jpg                1136 bytes  64x48
  FILE anim_0003.exr                4117 bytes  64x48
  FILE anim_0003.jpg                1136 bytes  64x48
--- PNG, use_preview set to True, now reads True ---
  FILE anim_0001.png                3667 bytes  64x48
  FILE anim_0002.png                3667 bytes  64x48
  FILE anim_0003.png                3667 bytes  64x48
```

The single-frame command-line run wrote `cli_0001.exr` at 2812 bytes and `cli_0001.jpg` at 1187 bytes.

*A smaller second copy made during the render.* Blender printed:

```
SCALE NODE inputs: ['Image', 'Type', 'X', 'Y', 'Frame Type', 'Interpolation', 'Extension X', 'Extension Y']
SCALE NODE has no space property; Type socket reads: 'Relative'
FILE main_0001.png          2824 bytes  64x48  depth=32 is_float=False
FILE small_small_0001.png   1518 bytes  32x24  depth=128 is_float=True
```

*Resizing a rendered picture and saving a copy.* The same two refusals and the same two successes:

```
--- bpy.ops.image.resize(size=(320, 180)) ---
RAISED RuntimeError: Operator bpy.ops.image.resize.poll() failed, context is incorrect
--- image.scale(320, 180) ---
scale() returned, size now (320, 180)
--- bpy.ops.image.save_as(filepath='.../small.png', save_as_render=True, copy=True) ---
RAISED RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect
small.png exists: False
FILE big.exr              100596 bytes  640x360
FILE small_as_png.png      48327 bytes  320x180
FILE small_render.png     107625 bytes  320x180
```

`file` reported `small_render.png` as `OpenEXR image data, version 2 … dataWindow: (0 0)-(319 179)` and `small_as_png.png` as `PNG image data, 320 x 180, 16-bit/color RGBA, non-interlaced`, exactly as on the Mac.

*Whether a saved proxy setup builds during a background run.* Only the three output frames were written, the exit status was `0`, and no `BL_proxy` folder existed anywhere afterwards.

Both machines were run and both agreed on every part of this, with only byte sizes differing slightly.

**Not covered.** The JPEG quality the Preview copy is written at, and what colour treatment is applied to it. The stored format of a proxy, and whether proxies can be built at all without a window by some route other than a background animation render.

**Outside the interface.** **Observed, and it splits three ways.** (a) works from outside: the Preview copy was written for a three-frame animation and for a single frame started from a terminal. (c) works from outside: the half-size second file was written by a windowless render. (d) is where the Image Editor commands fail — `bpy.ops.image.resize` and `bpy.ops.image.save_as` both refuse with `poll() failed, context is incorrect` — while the plain calls `Image.scale` and `Image.save_render` do the same job, with the catch that `save_render` writes in the scene's output format. (b) did not happen at all in a background animation render.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The Preview copy, the half-size second file, both refusals, both plain calls and the absence of proxies all came back the same on both. **Windows is still untried** — no Windows machine has run it. From the sources: JPEG is in the always-available list on the command-line page, so (a) does not depend on an optional build component — though OpenEXR, the format it accompanies, does.

**Where I looked.** Searched the manual for `thumbnail` (18 hits, all about asset, file-browser and sequencer strip thumbnails in the interface, none about writing a file), `preview image` (20 hits, mostly asset and brush previews), `JPG preview` / `JPEG copy` / `quick preview` (the one hit quoted above). Read `files/media/image_formats.txt` (OpenEXR Output Options) in full, `editors/image/editing.txt` in full, `compositing/types/transform/scale.txt` in full, `compositing/types/output/file_output.txt` in full, the two proxy pages in full, `render/output/properties/format.txt` (the Percentage setting — that shrinks the render itself, not a copy of it, so it does not answer the question). Checked `imbuf` and `imbuf.types` in the API and `bpy.ops.image` for `resize`. Checked "Playblast" (4 hits) and viewport rendering — those make a quick preview *of the scene*, not a copy of a picture a render produced, so I set them aside.

---

## Things a person with Blender 5.2 open could settle in a few minutes

All six items on this list have been run, on the Creator's own MacBook Pro (M1 Pro, macOS 26.5.2), and the results are written in above under the questions they answer. The whole list is at `Audits/Blender Research/Images/Hand tests.md`. Two were answered only in part: item 1 covered the finished picture in four engines and four cases, but not the separate passes and not a texture feeding roughness; item 3 read the names inside the written file rather than the strings the Image Editor shows.

The three checks that needed eyes on Blender's window were run by the Creator on 2026-08-04 and are written in above, which also closes item 3 on this list: the Layer and Pass menus show `Beauty` / `Second` and `Combined`, `Depth`, `Normal`, `Mist`, `Emission`, `Diffuse Color`.

Everything on that list was then run again on a second machine — a rented Linux machine, Ubuntu 22.04.5 LTS with an NVIDIA GeForce RTX 4090 — on 2026-08-06, together with the tests the one Mac could never reach. That second list is at `Audits/Blender Research/Images/Hand tests — Linux.md`, and its results are written in above beside the Mac's. **Three of the five gaps below are now closed**: the Image Editor's View menu, another make of graphics card, and a container-and-codec pairing Blender itself refuses — the last of which turns out to differ in this version.

What is still open:

1. **Windows.** Every observed answer above was got on macOS on Apple silicon or on Ubuntu on an NVIDIA card, and on both where the test could be repeated. No Windows machine has run it, so that system stays untried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

2. **A build that lacks a format.** (Question 4.) Still cannot be produced. Both machines were checked and the official build for each has every optional format compiled in. It needs a Blender built without one of them.

3. **Which separate passes show a missing texture, and what a texture feeding roughness does.** (Question 2.) The finished picture was read for every engine and every case on both machines; the individual passes and the roughness case were not.

4. **A graphics card that is neither Apple's nor NVIDIA's.** (Question 2.) The missing-texture result now covers Apple's Metal graphics, an NVIDIA card through CUDA and the same card through OptiX. AMD graphics, Intel graphics and a second card of any make cannot be tried.

5. **What the View picker's open menu looks like.** (Questions 1 and 5.) The picker itself is now photographed — it exists only for a file with several views in it, appears when the stereo button is off, is called `Select View.` and reads `left`. Opening it needs a click, and on the shared Linux display no button press reached Blender.

---

2026-08-03 — Opus 5 (Claude Code) — Images, Blender 5.2

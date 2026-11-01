# Images — Blender 5.1

**Sources consulted, and when.** Blender's manual for 5.1 (the `manual_5.1` folder, including pictures in its `_images` folder), Blender's Python API documentation for 5.1 (the `api_5.1` folder), the Blender 5.1 release notes (the `release_notes_5.1` folder), and Cycles' own files for 5.1 (the `cycles_5.1` folder). All four were taken on **2026-07-30**, and that is the date read for every finding below.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.** (Inside `Blender Sources/` I also read the folder's own note, `About this folder.md`. It links onward to a document in `Preparation/`; I did not follow that link.)

I ran no Blender and rendered nothing. Everything below is reading, except where a finding is drawn from Cycles' own files, which is marked **observed**.

---

## 1. What kinds of picture can a render produce, and how can the finished picture be told apart from the others?

**Answer.** A render produces a set of pictures called **passes**, and it produces one set of them per **view layer** (and, when stereo is on, per **view**). The finished picture is a pass with a name of its own: **Combined**. That is how it is told apart — by name.

There is a wrinkle worth stating plainly, because the manual states it. "Combined" is the finished picture *of the render itself*, before compositing. If the file has a compositor set up, the picture that finally leaves Blender is the one plugged into the first image socket of the compositor's Group Output node.

**LABEL: documented** (the pass list, the meaning of Combined, the pass name as a string, the compositor's final output), **plus observed** (Cycles' own list of pass types).

**The words that carry it.**

The manual's Passes page:

> "A Pass is a type of intermediate rendering information that's extracted as a separate image.
> Examples include the diffuse colors of the objects in the scene, the light distribution,
> the depth map, and the normal map."

and, at the head of the list:

> "Combined Cycles, EEVEE, Workbench
>
> The render output before any compositing is applied."

The same page names the rest of them. Under **Data**: Combined, Depth, Mist, Position, Normal, Vector, UV, Grease Pencil, Denoising Data. Under **Indexes**: Object Index, Material Index. Under **Debug**: Render Time, Sample Count. Under **Light** for Cycles: Diffuse (Direct, Indirect, Color), Glossy (Direct, Indirect, Color), Transmission (Direct, Indirect, Color), Volume (Direct, Indirect), and under Other: Emission, Environment, Ambient Occlusion, Shadow Catcher. Under **Light** for EEVEE: Diffuse (Light, Color), Specular (Light, Color), Volume (Light), and under Other: Emission, Environment, Shadow, Ambient Occlusion, Transparent. Then **Cryptomatte** (Object, Material, Asset), **Shader AOV** (custom, named by the user), and **Light Groups**.

Two of those entries are worth quoting because they bear on "which picture is the finished one":

> "Grease Pencil Cycles, EEVEE, Workbench
>
> Outputs only the visible (non-occluded) strokes and fills from the
> Grease Pencil engine into a separate image layer."

and

> "For most scenes, blending this pass over the rest of the image
> using an Alpha Over node will reproduce the Combined render result."

and, on Light Groups:

> "A Light Group provides a limited Combined render pass where the scene is only illuminated by
> certain lights."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/layers/passes.html — read 2026-07-30.

That a render yields one set of these per view layer:

> "Each View Layer can output multiple Render Passes including:
>
> Combined (final image)"

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/layers/introduction.html — read 2026-07-30.

The name is a literal string in the API. The API's own worked example for a render engine reads:

> "layer = result.layers[0].passes["Combined"]"

*Source:* Blender Python API, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

And the switch that turns it on says what it holds:

> "use_pass_combined
>
> Deliver full combined RGBA buffer (default False, readonly)"

*Source:* Blender Python API, https://docs.blender.org/api/5.1/bpy.types.RenderLayer.html — read 2026-07-30.

The container is described as:

> "Result of rendering, including all layers and passes"

with `layers`, `views`, `resolution_x` and `resolution_y` on it.

*Source:* Blender Python API, https://docs.blender.org/api/5.1/bpy.types.RenderResult.html — read 2026-07-30.

On the picture that leaves Blender after compositing:

> "In the Compositor, this node defines the image result that is sent to the renderer or to
> image output after rendering."

and

> "In the Compositor, only the first image input is used as the final render result.
> Any additional sockets will be ignored."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/group_output.html — read 2026-07-30.

A person looking at a render on screen picks between them by name:

> "Render Result: displays renders. When this item is selected, the Slot,
> View Layer, and Render Pass selectors become available (see below)."

and

> "Render Pass
>
> The Render Pass to display."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/editors/image/introduction.html — read 2026-07-30.

**Observed, from Cycles' own files.** Cycles keeps its own list of pass types, and Combined is the first one after "none":

> ```
> enum PassType {
>   PASS_NONE = 0,
>
>   /* Light Passes */
>   PASS_COMBINED = 1,
>   PASS_EMISSION,
>   PASS_BACKGROUND,
>   PASS_AO,
> ```

The list continues through the light passes to `PASS_CATEGORY_LIGHT_END = 31`, then the data passes from `PASS_DEPTH = 32` through `PASS_CATEGORY_DATA_END = 63`, then bake passes to `PASS_CATEGORY_BAKE_END = 95`.

*Source:* Cycles' own files, `cycles_5.1/source/kernel/types.h`, lines 297–301 (the enum opens at line 297 and runs to `PASS_NUM` at line 380; `PASS_CATEGORY_LIGHT_END = 31` is at line 319, `PASS_DEPTH = 32` at line 322, `PASS_CATEGORY_DATA_END = 63` at line 373, `PASS_CATEGORY_BAKE_END = 95` at line 378) — read 2026-07-30.

**Where I looked.** Searched the whole manual for "render pass" (26 files matched) and read `render/layers/passes.txt`, `render/layers/introduction.txt`, `editors/image/introduction.txt`, `compositing/types/input/scene/render_layers.txt`, and every file under `compositing/types/output/`. Searched the manual for "Combined" under `editors/image/`, `compositing/` and `render/output/`. Searched the API folder for "Combined" and read `bpy.types.RenderPass`, `bpy.types.RenderPasses`, `bpy.types.RenderLayer`, `bpy.types.RenderResult`, `bpy.types.RenderEngine`. Searched Cycles' files for "PASS_COMBINED" and read the enum in `source/kernel/types.h`; also read the pass entries in `cycles_5.1/properties.py`.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2. Blender 5.1.2 was started from a terminal with no window (`-b --factory-startup -noaudio`) and driven by a script. The whole list of tests is at `Audits/Blender Research/Images/Hand tests.md`.

*Two view layers, one plain single-image output.* Set up: the factory startup scene at 32 by 32; the world background set to pure red; a second view layer added, called `Second`, with the cube's collection excluded from it; output PNG; frames 1 to 1. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
VIEW LAYERS: ['ViewLayer', 'Second']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.8078, 0.5451, 0.5333, 1.0) corner=(0.8549, 0.2196, 0.1255, 1.0)
```

One file appeared, not one per view layer. Its corner carries the red world and its centre carries the lit cube.

*What lands in the output file when compositing is on.* Set up: the same scene at 32 by 32, PNG, one frame; a new node group of type `CompositorNodeTree` with one colour output socket, holding a `NodeGroupOutput` node and a `CompositorNodeRGB` set to pure blue linked into it, assigned to `scene.compositing_node_group`. Done: the same render call. Blender printed:

```
use_compositing: True
NODES: ['NodeGroupOutput', 'CompositorNodeRGB']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

Centre and corner are both the compositor's blue. So the picture written to the main output path is the compositor's result, not the Combined pass.

*What the finished picture is called once written.* From the multi-layer OpenEXR test recorded under question 5: with the view layer renamed to `Beauty`, that view layer's finished picture is a part of the file whose `name` attribute reads `Beauty.Combined` and whose channels are `Beauty.Combined.R`, `.G`, `.B` and `.A`.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 5.1.2 open. Looked at: the Image Editor header with the multi-layer OpenEXR from the test above loaded, its two picture-choosing menus opened in turn. Seen: the header shows the file name `frame_0001.exr` and then two dropdown menus, the first reading `Beauty` and the second reading `Combined`. Opening the first shows `Beauty` and `Second` above a greyed label reading `Layer`. Opening the second shows `Combined`, `Depth`, `Normal`, `Mist`, `Emission`, `Diffuse Color` above a greyed label reading `Pass`. So the finished picture is picked by choosing `Combined` in the menu labelled `Pass`, within the view layer chosen in the menu labelled `Layer`.

**Observed — the same tests on a second machine, 2026-08-06.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, started with no window (`-b --factory-startup -noaudio`) and driven by the same scripts the Mac run used, with only the folder path changed. The whole list is at `Audits/Blender Research/Images/Hand tests — Linux.md`.

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

**Observed — the window on the rented Linux machine, 2026-08-06.** Blender 5.1.2 was opened with a window on that machine's own display, the largest area turned into an Image Editor, the multi-layer OpenEXR above loaded into it, and the window photographed with Blender's own screenshot command, `bpy.ops.screen.screenshot`. The picture shows the header holding **two** picture-choosing menus, reading `Beauty` and `Combined` — the same two the Creator photographed on the Mac — and no third menu beside them. The Image Editor's own `View` menu, which sits in the header next to `Image`, was then opened with `bpy.ops.wm.call_menu(name="IMAGE_MT_view")` and photographed. It holds, top to bottom: `Toolbar`, `Sidebar`, `Tool Settings`, `Asset Shelf`, `Adjust Last Operation`, `Update Automatically`, `Show Metadata`, `Frame All`, `Center View to Cursor`, `Zoom`, `Area`, with `Asset Shelf` and `Center View to Cursor` greyed out. Nothing in that menu chooses which picture is shown.

**Not covered.** The two things this section left open — the Combined pass's channel names inside a written file, and whether the file at the main output path holds the Combined pass or the compositor's result — are settled by the hand test above. The sources still say neither: the Group Output page says the compositor's result "is sent to the renderer or to image output", but does not spell out the file it lands in. Nothing says what a render produces when a view layer has every pass switched off. The `render_layers_passes_combine.svg` diagram on the Passes page, which shows how the passes add up to the final image, is a drawing file I did not open, so I take nothing from it.

**Outside the interface.** **Observed.** The tests above were all run with no window open, and each did what is recorded: the passes were switched on as view-layer properties, the render was started, the file was written, and the compositor's result was the thing written. So the passes, the names they carry into a file, and the compositor deciding the saved picture all hold when Blender is driven from outside its own window. What stays window-only is the *picking*: the Render Pass selector is documented as an interface control, and outside the window the choice is made by what gets written to a file. Documented alongside: `-f, --render-frame <frame>`, "Render frame <frame> and save it." (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30). (Previously, before the test: not stated.)

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** Every test above was run on both and came back the same, and the Image Editor's `Layer` and `Pass` menus were photographed on both. [Citation check 2026-08-06: this read "the Image Editor's menus were photographed on both". The header's `Layer` and `Pass` menus were indeed photographed on both machines, but the Image Editor's own `View` menu (`IMAGE_MT_view`) was photographed on the Linux machine alone and stays untried on the Mac. Narrowed.] **Windows is still untried** — no Windows machine has run it. From the sources: they say nothing that ties the pass list or the Combined name to any operating system. One neighbouring limit is stated: "Headless rendering is not supported on headless Windows systems." (https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html, read 2026-07-30) — that is about EEVEE running at all, not about which passes it makes.

---

## 2. Do any of the pictures a render produces show whether a scene's textures loaded, and if so, which?

**Answer.** Not established. Nothing in the manual, the API documentation or the release notes describes a render pass that reports whether a scene's image textures loaded. No documented pass is described as carrying that information.

What the sources *do* give is one thing next door to it, and it is the strongest thing available: Cycles substitutes a fixed magenta colour for an image it cannot find. That colour is a texture lookup result, not a pass.

**LABEL for the main answer: not in the sources.**
**LABEL for the magenta substitution: observed** (from Cycles' own files).
**LABEL for "so it would appear in the colour-carrying passes": inferred** — see the step below.

**The observed part, quoted exactly.**

> ```
> /* Color to use when images are not found. */
> #define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)
> ```

*Source:* Cycles' own files, `cycles_5.1/source/util/types_image.h`, lines 12–13 — read 2026-07-30.

That constant is what the shader's image lookup hands back when there is no image behind the slot:

> ```
> ccl_device float4
> svm_image_texture(KernelGlobals kg, const int id, const float x, float y, const uint flags)
> {
>   if (id == -1) {
>     return IMAGE_MISSING_RGBA;
>   }
> ```

*Source:* Cycles' own files, `cycles_5.1/source/kernel/svm/image.h`, lines 21–26 — read 2026-07-30.

The same constant is returned by the 3D-image lookup:

> ```
>   return IMAGE_MISSING_RGBA;
> ```

*Source:* Cycles' own files, `cycles_5.1/source/kernel/util/image_3d.h`, line 258 — read 2026-07-30.

**The inferred step, written out.** The files say the texture lookup returns the colour red 1, green 0, blue 1, alpha 1 when the image is absent. To get from there to "you can see a missing texture in the render" a reader has to supply two things the files do not say: that this returned colour then flows through the material like any other colour, and that a pass carrying surface colour (Combined, or Diffuse Color / Glossy Color / Transmission Color) therefore shows it. Both are reasonable, and neither is written down in what I was given. So: **inferred**, and it is inference about Cycles only — I found nothing at all about what EEVEE or Workbench do with an absent image.

Note also what the constant does *not* tell you. It fires when there is no image behind the slot. Whether that covers "the file on disk is missing", "the file is corrupt", "the slot was left empty", or all three, the files do not say.

**One documented finding nearby, clearly scoped.** The Video Sequencer has an explicit setting for this, but it is about strips in the sequencer, not about a 3D scene's textures, and it sits under the preview's View settings:

> "Show Missing Media
>
> Render missing images/movies with a solid magenta color.
> When disabled, missing content will render fully transparent."

with a following tip: "Strips with missing content will be displayed as red in the timeline."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/sidebar.html (panel: Sidebar ‣ View tab ‣ View Settings) — read 2026-07-30.

And there is a check that is not a picture at all:

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30.

**Where I looked.** Read the whole Passes page and checked every pass entry for anything about textures loading — none mentions it. Read the Image Texture Node page (`render/shader_nodes/textures/image.txt`) end to end; it describes Image, Interpolation, Projection, Extension, Source, Color Space, Alpha and the two outputs, and says nothing about a missing or unloadable image. Searched the whole manual, case-insensitively, for: "magenta" (6 hits, all unrelated except the sequencer one above), "pink" (18 hits, all unrelated — node socket colours, curve handles, sequencer placeholder frames), "missing" narrowed to lines also mentioning texture / image / file / path, "not found", "cannot be found", "could not be found", "fails to load", "failed to load", "texture.\*load", "image.\*fail", "broken link", "placeholder", "unresolved", "no image", "empty image". Searched the API folder for the same. Searched all of Cycles' files for "missing" and for "magenta"/"pink". Ruled out the Denoising Data, Denoising Albedo and Cryptomatte passes as candidates by reading their descriptions on the Passes page — none is described as reporting file state.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2. Blender 5.1.2 started from a terminal with no window and driven by a script. Set up: 32 by 32, one still, output OpenEXR, Cycles samples 1; a plainly green 8 by 8 PNG written to disk; the default cube given a material whose only shading is an Emission node fed by an Image Texture node pointing at that file, so the rendered colour is the texture's colour; Workbench's shading colour type set to `TEXTURE`; Metal switched on in the Cycles preferences, which reported `CYCLES DEVICES: [('Apple M1 Pro', 'CPU', True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`. Done: sixteen renders — four cases in four engines — each written to its own OpenEXR and the centre pixel read back. Blender printed:

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

The "corrupt" case was the same file replaced with the bytes `this is not a picture`.

So: EEVEE and Workbench do substitute a colour, and it is the same magenta Cycles uses, for a missing file and for an unreadable one. The one case where they part is an empty image slot: Cycles still shows magenta, EEVEE shows black, and Workbench shows plain grey.

This version prints messages naming the file:

```
00:01.632  cycles           | ERROR Image file /tmp/images-hand-tests/out/t11/5.1.2/green.png does not exist.
00:01.705  cycles           | ERROR Image file /tmp/images-hand-tests/out/t11/5.1.2/green.png does not exist.
00:01.910  gpu.texture      | ERROR Failed to create GPU texture from Blender image
00:02.243  cycles           | ERROR Image file  does not exist.
00:02.323  cycles           | ERROR Image file  does not exist.
00:02.929  cycles           | ERROR Image file /tmp/images-hand-tests/out/t11/5.1.2/green.png failed to load.
00:03.004  cycles           | ERROR Image file /tmp/images-hand-tests/out/t11/5.1.2/green.png failed to load.
00:03.211  image.read       | ERROR IMB_load_image_from_memory: unknown file-format (/tmp/images-hand-tests/out/t11/5.1.2/green.png)
00:03.211  gpu.texture      | ERROR Failed to create GPU texture from Blender image
OpenImageIO exited with a pending error message that was never
retrieved via OIIO::geterror(). This was the error message:
Not a PNG file
```

The two lines reading `Image file  does not exist.` with nothing between the words are the empty-slot case.

**Observed — the same test on a second machine and a different make of graphics card, 2026-08-06.** Run on the rented Linux machine, Blender 5.1.2, no window. The set-up is the Mac's, with one change: Apple's Metal graphics do not exist here, so the script asks Cycles what it has and then renders on this machine's card twice, once through CUDA and once through OptiX. It reported:

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

The `cycles | ERROR Image file <path> does not exist.` lines, the unquoted path, the `cycles | ERROR Image file  does not exist.` line for the empty slot and the `failed to load` lines all came back exactly as on the Mac. This machine adds two lines the Mac never printed:

```
00:00.495  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
00:04.062  gpu.texture      | ERROR Failed to create GPU texture from Blender image
```

The first is Cycles looking for AMD graphics that are not in this machine.

**Not covered.** Whether the magenta shows up in any particular pass — the test above read the finished picture, not the separate passes.

**Outside the interface.** **Observed.** The whole test above was run with no window open, and the substituted colour appeared in the written files just the same. So the substitution holds when Blender is driven from outside its own window. And something *is* printed to the console during a background render: the lines above name the missing or unreadable file by its full path. "Report Missing Files" puts its warning "in the Info editor's header", which is an interface region, and was not tested. (Previously, before the test: not stated either way.)

**Operating system.** **Observed on two machines and on two makes of graphics card, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro) with Apple's own Metal graphics, and Ubuntu 22.04.5 LTS with an NVIDIA GeForce RTX 4090 used both through CUDA and through OptiX.** The substituted colour is identical everywhere; what Blender prints alongside it gains two machine-specific lines on Linux, quoted above. **Windows is still untried** — no Windows machine has run it — and so is any card that is neither Apple's nor NVIDIA's. From the sources: they say nothing tying any of this to an operating system.

---

## 3. Which file formats can Blender write a render out as?

**Answer.** Two families, plus a multi-layer variant.

**Still-image formats** (from the manual's own table, with the channel depths it gives): JPEG (8-bit), OpenEXR (float 16 and 32-bit), PNG (8, 16-bit), WebP (8-bit), AVIF (8, 10, 12-bit), BMP (8-bit), Cineon (8, 10, 12, 16-bit), DPX (8, 10, 12, 16-bit), Iris (8, 16-bit), JPEG 2000 (8, 12, 16-bit), Radiance HDR (float), Targa (8-bit), Targa Raw (8-bit), TIFF (8, 16-bit).

**A multi-layer variant:** OpenEXR MultiLayer, which the API lists as its own choice.

**Video**, all through FFmpeg: containers MPEG-4, Matroska, WebM, AVI, DV, Flash, MPEG-1, MPEG-2, Ogg, QuickTime; video codecs No Video, AV1, H.264, H.265/HEVC, WEBM/VP9, DNxHD, DV, FFmpeg video codec #1, Flash Video, HuffYUV, MPEG-1, MPEG-2, MPEG-4 (DivX), ProRes, PNG, QuickTime Animation, Theora; audio codecs No Audio, AAC, AC3, FLAC, MP2, MP3, Opus, PCM, Vorbis.

**LABEL: documented.**

**The words that carry it.** The manual's table is headed:

> "This is the list of image file formats supported internally by Blender:"

and its columns are Format, Channel Depth, Alpha, Metadata, DPI, Extensions. Its rows give the extensions as: JPEG `.jpg .jpeg`; OpenEXR `.exr`; PNG `.png`; WebP `.webp`; AVIF `.avif`; BMP `.bmp`; Cineon `.cin`; DPX `.dpx`; Iris `.sgi .rgb .bw`; JPEG 2000 `.jp2 .jp2 .j2c`; Radiance HDR `.hdr`; Targa `.tga`; Targa Raw `.tga`; TIFF `.tif .tiff`.

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html — read 2026-07-30.

The video list, and its opening:

> "Blender uses FFmpeg to handle video encoding and decoding.
> These formats are primarily used to compress rendered image sequences into playable movies."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/files/media/video_formats.html — read 2026-07-30.

The API gives the same set as machine-readable names, and it is the one place OpenEXR MultiLayer appears as a choice of its own:

> "OPEN_EXR_MULTILAYER:
>
> OpenEXR MultiLayer (.exr).
>
> Output image in multilayer OpenEXR format."

and the whole list of identifiers is: `AVIF`, `JPEG`, `OPEN_EXR`, `PNG`, `WEBP`, `BMP`, `CINEON`, `DPX`, `IRIS`, `JPEG2000`, `HDR`, `TARGA`, `TARGA_RAW`, `TIFF`, `OPEN_EXR_MULTILAYER`, `FFMPEG`.

*Source:* Blender Python API, https://docs.blender.org/api/5.1/bpy_types_enum_items/image_type_all_items.html — read 2026-07-30.

The property that takes one of them:

> "file_format
>
> File format to save the rendered images as (default 'PNG')"

*Source:* Blender Python API, https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html — read 2026-07-30.

Above that sits a coarser choice:

> "Media Type
>
> The type of media to save.
>
> Image: Saves each input as a separate image file. Each input can use its own image format settings.
>
> Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file.
>
> Video: Encodes each frame into a video container."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30. The API mirrors it: "media_type — The type of media to save (default 'IMAGE')", one of `'IMAGE'`, `'MULTI_LAYER_IMAGE'`, `'VIDEO'` (https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html, read 2026-07-30).

**This is important and is stated outright:** not every one of these is present in every copy of Blender. The command-line page splits the list in two:

> "-F, --render-format <format>
>
> Set the render format.
> Valid options are:
> TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF.
>
> Formats that can be compiled into Blender, not available on all systems:
> OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. Note that AVIF does not appear in either of those two command-line lists, although the manual's table and the API enum both have it.

A running copy of Blender will say which it was built with. The API documents the values from the build it was generated on:

> "Constant value bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, …)"

*Source:* Blender Python API, https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30. That is one particular build's answer, not a rule.

**New in this version**, stated as a change:

> "Avif: New file format support. AVIF has better compression than other lossy file formats, supports HDR images and is widely supported in web browsers."

and

> "OpenEXR: New lossless encoding option HTJ2K (high throughput JPEG 2000). It produces somewhat smaller files than other lossless
> compression methods, but is very new and not widely supported yet by other software."

and

> "JPEG 2000: Use multi-threading to speed up file saving."

*Source:* Blender 5.1 release notes, https://developer.blender.org/docs/release_notes/5.1/pipeline_io/ — read 2026-07-30.

**Documented, from a picture.** The manual's Output panel screenshot shows the panel with a File Format dropdown reading "PNG", a Color row offering BW / RGB / RGBA with RGBA chosen, a Color Depth row offering 8 / 16 with 8 chosen, and a Compression slider at 15%. Also visible: a path field reading `/tmp/`, and checkboxes Overwrite, File Extensions, Placeholders, Cache Result.

*Source:* Blender manual, picture `_images/render_output_properties_output_panel.png` on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30. What is visible in it is visible; I read nothing more into it. (The screenshot does not show a Media Type row, although the page text describes one. I record that as what the picture shows, not as a claim about the program.)

**Where I looked.** Read `files/media/image_formats.txt` and `files/media/video_formats.txt` in full. Read `render/output/properties/output.txt` and `render/output/properties/format.txt` in full. Read the API's `image_type_all_items` enum page in full and `bpy.types.ImageFormatSettings` in full. Searched the API folder for "Image Type All Items" to find the enum. Read the Format Options section of the command-line arguments page. Read the whole of `release_notes_5.1/pipeline_io.md`. Searched the release notes for "remove" near format/codec/image words (nothing relevant found). Opened the Output panel picture.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, on the Blender 5.1.2 installed there, started from a terminal with no window. Set up: nothing built; a plain start with factory settings. Done: printed the build options, the output-format setting and its choices, the Media Type setting and its choices, and the FFmpeg container and codec choices. Blender printed:

```
VERSION: 5.1.2
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

This build has every optional image format compiled in, and `AVIF` is among the formats.

*Media Type comes first.* Setting `file_format` while `media_type` is `IMAGE` is refused for anything that is not a still image, with a message listing what is allowed:

```
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "NOSUCHFORMAT" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
```

So the sixteen names listed above are the whole set, but which of them the setting will take at any moment depends on the Media Type.

*AVIF from the command line.* This was the open question in this section. Set up: nothing built; the factory startup scene shrunk to 64 by 64. Done, in a terminal:

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=64" -F AVIF -o /tmp/images-hand-tests/out/t02/5.1.2_AVIF/out_ -f 1
```

It was accepted. Blender printed `Saved: '/tmp/images-hand-tests/out/t02/5.1.2_AVIF/out_0001.avif'`, the exit status was `0`, and `file` reported the written file as `ISO Media, AVIF Image`. `AVIF` was also accepted as a value for `file_format` from Python, and a still rendered after that came out as `out_.avif`. So AVIF is reachable from the command line in this version, even though the manual's command-line page lists it in neither of its two lists.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 5.1.2 open. Looked at: Properties ▸ Output Properties ▸ Output panel, with the format changed in turn, and the menus opened.

There are **two** controls, one above the other: a row labelled **`Media Type`** and, below it, a row labelled **`File Format`**. Opening `Media Type` shows `Image`, `Multi-Layer EXR`, `Video`. The `File Format` entries carry their extensions in the label — `PNG (.png)`, `JPEG (.jpg)`, `TIFF (.tif)`, `OpenEXR (.exr)`, `AVIF (.avif)`.

**AVIF is offered in the window as well as on the command line.** With `AVIF (.avif)` chosen the panel shows `Color` (BW / RGB / RGBA), `Color Depth` (8 / 10 / 12), and `Quality 90%`.

With each format chosen, the panel shows: **PNG** — `Color` (BW / RGB / RGBA), `Color Depth` (8 / 16), `Compression 15%`. **JPEG** — `Color` (BW / RGB), `Quality 90%`. **TIFF** — `Color`, `Color Depth` (8 / 16), `Compression: Deflate`. **OpenEXR** — `Color`, `Color Depth` (Float (Half) / Float (Full)), `Codec: ZIP`, `Preview`. With `Media Type` set to `Multi-Layer EXR` the `File Format` row disappears from the panel altogether, leaving `Color Depth`, `Codec: ZIP`, `Interleave` and `Preview`.

With `Media Type` set to `Video`, the `Encoding` section's `Container` menu and the `Video` section's `Video Codec` menu were opened. Every entry, in the order shown:

- **Container**: `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`.
- **Video Codec**: `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`.

The rows under `Video` are `Video Codec`, `Color Depth`, `Color Space`, `Output Quality`, `Encoding Speed`, `Keyframe Interval`, `Max B-frames`.

Not captured: no picture shows the `File Format` menu itself opened in this version, so the full list as the menu presents it is not recorded here. The terminal record above gives the same set of names.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.1.2, no window, factory settings. Blender printed:

```
VERSION: 5.1.2
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

*Asking for a format by name on the command line.* `AVIF` wrote a real file on this machine too: `Saved: '.../out_0001.avif'`, 748 bytes, reported by `file` as `ISO Media, AVIF Image`. `WEBP` wrote `out_0001.webp` (`RIFF (little-endian) data, Web/P image, VP8 encoding, 64x64`), `OPEN_EXR_MULTILAYER` wrote `out_0001.exr`, and only `NOSUCHFORMAT` was refused, with `Error: Format from '-F / --render-format' not known or not compiled in this release.` followed by a PNG. Exit status `0` every time.

*Asking for a format by name from Python.* `AVIF` was accepted — `ASSIGNMENT OK; file_format is now: AVIF` — and the still was written as `out_.avif`. The unknown name gave:

```
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "NOSUCHFORMAT" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
```

*What the container and codec lists mean.* A further test, recorded in full under question 4, chose each container in turn and read the codec list back: all ten containers offer all seventeen codecs after being chosen, and all 170 pairings can be set. So the lists above are what Blender offers, not what it will accept — several pairings fail at the moment of encoding.

**Not covered.** Which formats a *downloaded* build on another system has — the hand test settles it for this Mac's build only. Whether every listed video codec works in every listed container: the manual says only "Not all codecs are available in all containers" and "Some containers and codecs are not compatible with each other". Whether AVIF is reachable from the command line's `-F` (it is absent from both of that page's lists, and the page does not say why).

**Outside the interface.** **Observed.** Everything above was done with no window open: `-F` chose the format from the command line and wrote the file, AVIF included; the settings were read and written from Python in a windowless Blender. The one thing to know before scripting against it is the order — Media Type first, then the format. Documented alongside: `-x, --use-extension <bool>` controls whether the extension is appended, and the second list of formats on the command-line page is "not available on all systems".

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The official build for each system has every optional image format compiled in, the format lists match exactly, and AVIF works from the command line on both. **Windows is still untried** — no Windows machine has run it — so what a build there carries is not settled. From the sources: the manual does not attach any image format to any operating system, but it does say plainly that seven of them are build-time options and "not available on all systems". Separately, of the ProRes and QuickTime entries, the manual describes them by their use, not by any operating system. One unrelated OS note in the same area, for completeness: image copy and paste to the system clipboard supports only PNG, "Windows: Supports pasting images by copying the image's file path… Linux: Requires Wayland for clipboard image support." (https://docs.blender.org/manual/en/5.1/editors/image/editing.html, read 2026-07-30) — that is clipboard, not render output.

---

## 4. What happens when a file asks for a format that Blender cannot write?

**LABEL: not in the sources.**

**Answer.** None of the four sources says what Blender does when a blend-file asks for an output format the running copy of Blender cannot write. Not whether it refuses to render, falls back to another format, warns, writes nothing, or carries on. I looked hard, because the situation is real — the sources say plainly that it can arise — but the outcome is simply never described.

**What the sources do establish, which is the setup and not the answer.** That the situation exists at all:

> "Formats that can be compiled into Blender, not available on all systems:
> OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

And that a copy of Blender knows which it has, through `bpy.app.build_options` with its `image_openexr`, `image_openjpeg`, `image_tiff`, `image_webp`, `image_cineon`, `image_hdr`, `codec_ffmpeg`, `codec_avi` entries.

*Source:* Blender Python API, https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30.

Neither says what happens if a file asks for one that is absent.

**Where I looked, in full.**

Pages read end to end looking for it: `render/output/properties/output.txt` (the Output panel, including File Format, Media Type, Overwrite, Placeholders and the Encoding panel); `render/output/properties/format.txt`; `render/output/animation.txt`; `files/media/image_formats.txt` (including its Limitations note, which is about metadata, not about formats being absent); `files/media/video_formats.txt` (including its "Known Limitations" section, which is about codec output size, not about absent formats); the Format Options and Render Options sections of `advanced/command_line/arguments.txt`; `compositing/types/output/file_output.txt`; `files/blend/compatibility.txt` headings; `troubleshooting/` — its files are `3d_view`, `crash`, `gpu`, `index`, `network`, `python`, `recover`, `report_bug`, `startup`, none of which is about output formats.

Searches run across the manual, the API documentation and the release notes together, case-insensitively, all of which came back with nothing on point: "invalid format" (nothing), "unknown format" (nothing), "unsupported format" (one hit, in the bug-fix list, about a Python string error), "format is not" (nothing), "cannot be saved" (nothing), "error saving" (nothing), "cannot write" (nothing), "unable to save" (nothing), "unable to write" (nothing), "fails to save" (nothing), "cannot save" (nothing), "can't save" (nothing), "defaults to PNG" / "default to PNG" (one hit, about texture paint's new-image default), "falls back" / "fall back" / "fallback" (26 hits, all about ray-tracing, drivers, tools, glTF textures — none about output formats), "not supported" / "unsupported" narrowed to lines also mentioning format, codec, write, save, render or output (30 hits, all about EEVEE features, GPU rendering and FBX), "silently" (hits about data-blocks and property clamping), "ignored", "skipped", "abort", "no output", "not available on all systems" and "compiled into Blender" (the single command-line hit above).

Also searched the API documentation for how the save calls report trouble: read `bpy.types.Image.save`, `Image.save_render` and `Image.file_format` — the parameter lists mention only filepath, scene, quality and save_copy, and describe no failure behaviour. Searched the 5.1 release notes' bug-fix list for image-format entries; the ones there ("Blender instantly crashes when loading an incomplete exr file"; "ARM64 builds have issues saving WEBP images from 32bpp buffers") are about specific defects being fixed, not about the general behaviour asked for here.

**Ambiguity, flagged rather than resolved.** "A file asks for a format" I have read as *a blend-file whose output settings name a format the running Blender cannot produce*, because that is the reading that follows question 3. Two other readings are possible and I answer neither: (a) an output *path* whose extension names a format Blender cannot write — but Blender takes the format from a setting, not from the path, so far as the sources describe it; and (b) a legal format asked for with an illegal option beside it, for instance RGBA on a format that has no alpha. On (b) the manual says only "Note, RGBA is not available for all image formats, check the list above for details", which describes the option's availability and not what happens if it is set anyway (https://docs.blender.org/manual/en/5.1/files/media/image_formats.html, read 2026-07-30).

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.1.2, started from a terminal with no window. Three cases were tried.

*A name the command line will not take.* Set up: nothing built; the factory startup scene at 64 by 64, its own output format left at the default PNG. Done, in a terminal:

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=64" -F NOSUCHFORMAT -o /tmp/images-hand-tests/out/t02/5.1.2_NOSUCHFORMAT/out_ -f 1
```

Blender printed:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
...
00:00.833  render           | Saved: '/tmp/images-hand-tests/out/t02/5.1.2_NOSUCHFORMAT/out_0001.png'
00:00.833  render           | Time: 00:00.52 (Saving: 00:00.06)
```

The exit status was `0`. One file was written, `out_0001.png`, which `file` reported as `PNG image data, 64 x 64, 8-bit/color RGBA, non-interlaced` — the format already set in the file. So the render carries on in the file's own format and nothing is substituted.

*The same request from Python.* Set up: nothing built. Done: `bpy.context.scene.render.image_settings.file_format = 'NOSUCHFORMAT'`, then a still rendered with `bpy.ops.render.render(animation=False, write_still=True)`. Blender printed:

```
ASSIGNMENT RAISED: TypeError
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "NOSUCHFORMAT" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
file_format after attempt: PNG
OPERATOR RESULT: {'FINISHED'}
```

*A request the codec refuses.* Set up: the factory startup scene, `image_settings.media_type = 'VIDEO'`, `image_settings.file_format = 'FFMPEG'`, `ffmpeg.format = 'MPEG4'`, `ffmpeg.codec = 'H264'`, resolution 101 by 101, frames 1 to 3. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
00:00.313  video.write      | ERROR Couldn't initialize video codec: Generic error in an external library
Traceback (most recent call last):
  File "/private/tmp/images-hand-tests/scripts/t04_video_size.py", line 24, in <module>
    res = bpy.ops.render.render(animation=True)
RuntimeError: Error: width not divisible by 2 (101x101)
```

and, on its own report line, `Error: width not divisible by 2 (101x101)`. The output folder was empty afterwards. The process exit status was `0` — the Python error did not change it, since `--python-exit-code` was not given.

*The case that could not be produced.* The reading the question was written for — a blend-file asking for a real format that this copy of Blender lacks — could not be made to happen on this machine. The build-options line recorded under question 3 shows every optional format compiled in. That case is recorded as not runnable in `Audits/Blender Research/Images/Hand tests.md`, with the reason: it needs a Blender build made without one of those formats, and both machines the project has carry official builds with every one of them. The Linux run of 2026-08-06 checked the second machine and found the same, so the case is not runnable on either.

**Observed — the same three cases on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.1.2, no window. All three were repeated with the same scripts and the same commands, and all three came back the same as on the Mac: `Error: Format from '-F / --render-format' not known or not compiled in this release.` on a plain line followed by a PNG and exit status `0`; the same `TypeError` from Python with the same list of names; and, for the 101 by 101 video, `00:00.581  video.write      | ERROR Couldn't initialize video codec: Generic error in an external library` then `RuntimeError: Error: width not divisible by 2 (101x101)`, an empty output folder and exit status `0`.

**Observed — a request Blender itself refuses: every container-and-codec pairing the lists offer, 2026-08-06.** This case could not be produced on the Mac run because nobody had tried encoding with a pairing; only the lists had been read. Set up: the factory startup scene at 64 by 48, frames 1 to 3, Media Type `VIDEO`, format `FFMPEG`. Done: each container chosen in turn and its codec list read back; then all 170 pairings assigned; then three frames really encoded for every container with `H264`, and for the `MPEG4` container with every codec. **The same script was run on both machines** — on the rented Linux machine and on the Creator's own MacBook Pro (M1 Pro, macOS 26.5.2) — **and the two agreed on all 104 encodes.**

Nothing is refused at the setting: every container offers the same seventeen codecs once chosen, and Blender printed `PAIRINGS THE SETTING REFUSED: []`.

The encoder refuses a great deal. The wording that answers this question is:

```
RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
```

It came back for the WebM container with H.264, and for the MPEG-4 container with No Video, FFV1, Flash Video, HuffYUV, ProRes, QuickTime Animation and Theora. In each of those cases a file of zero or a few hundred bytes was left behind. Three further refusals are about the picture rather than the pairing:

```
RAISED RuntimeError: Error: Render width has to be 720 pixels for DV!
RAISED RuntimeError: Error: Invalid tile sizing: frame not large enough to fit specified tile arrangement.
RAISED RuntimeError: Error: ff_frame_thread_encoder_init failed
```

— the first for the DV container and for the DV codec, the second for AV1, the third for DNxHD. Everything else finished and wrote a file. In this version, MPEG-4 with FFV1 is among the refused pairings.

**The Mac's own words for this test, added 2026-08-06.** The check of this topic ruled that the answer above said the two machines agreed without ever quoting what the Mac printed, which is an assertion rather than a record. So the same script was run again on the Creator's own MacBook Pro — Apple M1 Pro, macOS 26.5.2 — in this version, with its settings pointed at a folder of its own. Twelve of the twenty-six encodes were refused, and these are the lines that came back, verbatim:

```
WEBM/H264      RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
DV/H264        RAISED RuntimeError: Error: Render width has to be 720 pixels for DV!
MPEG4/NONE     RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/AV1      RAISED RuntimeError: Error: Invalid tile sizing: frame not large enough to fit specified tile arrangement.
MPEG4/DNXHD    RAISED RuntimeError: Error: ff_frame_thread_encoder_init failed
MPEG4/DV       RAISED RuntimeError: Error: Render width has to be 720 pixels for DV!
MPEG4/FFV1     RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/FLASH    RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/HUFFYUV  RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/PRORES   RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/QTRLE    RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
MPEG4/THEORA   RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
```

Every other encode wrote a file. This matches the rented Linux machine's result exactly, refusal for refusal and wording for wording.

**Not covered.** The one reading the question was written for — a real format this build lacks — is still unanswered, and now for a stronger reason: both machines the project has were checked, and the official build for each has every optional format compiled in, so the case cannot be produced on either. Also still unanswered: whether a missing video codec behaves like a missing image format, and whether any of this differs on Windows.

**Outside the interface.** **Observed.** All three cases above were run with no window open, and that is where the answers came from — which is the way it matters most, since the answer differs sharply by case. A format name the command line will not take is reported on one line and then ignored, the render carrying on in whatever format the file already held, with an exit status of `0`. A request the codec refuses stops that render, writes no file, and still leaves the exit status at `0`. (Previously, before the test: not settled either way.)

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The same lines, the same refusals and the same exit statuses came back on both, including every one of the container-and-codec refusals. **Windows is still untried** — no Windows machine has run it — and that gap sits directly on this question, since builds vary by system and that is the reason the situation exists at all. What the two machines show is that neither of them lacks a format.

---

## 5. Can a format hold several pictures inside one file, and if so, how can the finished picture be told apart from the others inside it?

**Answer.** Yes — OpenEXR. The sources document several **layers and passes** in one file, and several **views** (stereo) in one file.

The sources do not establish how the finished picture can be identified by name inside a rendered multi-layer EXR. They establish that the finished render pass is called **Combined** in Blender (see question 1), but they do not show the names written inside the file. For a file written by the compositor's File Output node, the sources say socket names construct final file names and each input becomes a separate layer. They do not say outright that each layer takes its socket's name.

**LABEL: documented** for OpenEXR holding several layers, passes and views; **not in the sources** for how the finished picture is identified inside the written file.

**The words that carry it.**

> "An OpenEXR file can store multiple layers and passes.
> This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html — read 2026-07-30.

The output setting that does it:

> "Multi-Layer EXR:
>
> Saves all inputs together in a single multi-layer OpenEXR file."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

The compositor's File Output node ties socket names to final file names and inputs to layers, but does not state the layer-naming rule:

> "Lists all input sockets of the node.
> The name of each input socket is used to construct the final file name.
>
> When saving individual images, each output can be renamed and will be written to disk
> using either the global Node Format settings or its own overridden settings.
> When using Multi-Layer EXR, each input is stored as a separate layer within the file."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30.

The other direction, views:

> "Multi-View OpenEXR
>
> Another option is to use multi-view OpenEXR files. This format can save multiple views in
> a single file and is backward compatible with old OpenEXR viewers (you see only one view though).
> Multi-view native support is only available to OpenEXR."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/stereoscopy/usage.html — read 2026-07-30.

How the file is arranged internally has a switch on it:

> "Interleave
>
> Use legacy interleaved storage of views, layers and passes for compatibility with
> applications that do not support more efficient multi-part OpenEXR files."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html — read 2026-07-30. The API says the same: "use_exr_interleave — Use legacy interleaved storage of views, layers and passes for compatibility with applications that do not support more efficient multi-part OpenEXR files. (default False)" (https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html, read 2026-07-30).

Blender also writes a multi-layer EXR for its own cache, and reads it back with everything separate:

> "Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image.
> The Compositor can then use this file to improve performance, especially for heavy compositing.
>
> The image is stored in the Render Cache folder as specified in the
> File Paths Preferences.
> You can also load it back into the Image Editor's Render Result, even after closing
> and reopening Blender."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

One place the manual states an actual layer name convention for a multi-layer EXR — for a different purpose, but it shows that Blender does read layers by name:

> "Two kinds of images are supported for MatCaps. Regular image files and
> multilayered OpenEXR files. When using multilayered OpenEXR files,
> the layer named "diffuse" will be used as a diffuse pass, the layer named "specular"
> will be used as a specular pass."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/editors/preferences/lights.html — read 2026-07-30.

**A caution the manual itself gives**, because it bears on what you can do with such a file afterwards:

> "The Video Sequencer does not support multi-layer EXR files.
> To render to a video format you will have to skip the next three steps and
> instead use an Image Input node in the Compositor."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

**Where I looked.** Read `files/media/image_formats.txt` (its whole OpenEXR section), `render/output/properties/output.txt`, `compositing/types/output/file_output.txt`, `render/output/properties/stereoscopy/usage.txt`, `render/output/animation.txt`, `editors/image/editing.txt` (the Open Cached Render entry), `editors/preferences/lights.txt`. Read `bpy.types.ImageFormatSettings` and the `image_type_all_items` enum in the API. Searched the manual and API for "layer name", "channel name", "named after" (17 hits, the only relevant ones being the File Output node line and the MatCap line above). Checked every other format's row in the manual's table for any mention of multiple pictures — none has one. Checked whether any *video* container is described as holding several pictures per frame: the video page describes containers as holding "multiple video, audio, subtitle, and metadata tracks", which is tracks over time, not several versions of one frame.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.1.2, started from a terminal with no window. This is the answer to the sharp edge below.

*A rendered multi-layer OpenEXR.* Set up: 32 by 32, one frame; the first view layer renamed to `Beauty` and a second added called `Second`; on both, `use_pass_combined`, `use_pass_z`, `use_pass_normal`, `use_pass_mist`, `use_pass_diffuse_color` and `use_pass_emit` switched on; `image_settings.media_type = 'MULTI_LAYER_IMAGE'` and then `file_format = 'OPEN_EXR_MULTILAYER'`; `use_exr_interleave` left at its default, which printed as `False`. The render engine was the factory default, which this version reports as `BLENDER_EEVEE`, so the names below are the ones that engine writes. Done: `bpy.ops.render.render(animation=True)`, then the written file's own bytes read with a short header reader (see `Audits/Blender Research/Images/Hand tests.md`). The reader printed:

```
BYTES: 13901
MAGIC: 0x01312f76
VERSION FIELD: 0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)
--- PART 0 ---
  BlenderMultiChannel      string           'Blender V2.55.1 and newer'
  Camera                   string           'Camera'
  Date                     string           '2026/08/03 22:42:41'
  File                     string           '<untitled>'
  Frame                    string           '1'
  RenderTime               string           '00:01.00'
  Scene                    string           'Scene'
  Software                 string           'Blender 5.1.2'
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

So: the file is a multi-part file, each part named `<view layer>.<pass>`, each channel named `<view layer>.<pass>.<channel>`. The finished picture of the first view layer is the part named `Beauty.Combined`, whose channels are `Beauty.Combined.R`, `.G`, `.B` and `.A`. That is the name a program outside Blender needs.

*A multi-layer OpenEXR written by a File Output node.* Set up: the same scene at 32 by 32, main output PNG; a File Output node with `directory` set to the folder, `file_name = "fromnode_"`, `format.media_type = 'MULTI_LAYER_IMAGE'`, `format.file_format = 'OPEN_EXR_MULTILAYER'`, its items cleared, and two items added with `file_output_items.new('RGBA', "MyBeautyLayer")` and `file_output_items.new('RGBA', "second socket")`, both fed from the Render Layers node's Image output. Done: the same render call. Blender printed `Saved: '/tmp/images-hand-tests/out/t08/5.1.2/fromnode_0001.exr'`, and the header reader printed:

```
VERSION FIELD: 0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)
  channels                 chlist           MyBeautyLayer.A(FLOAT); MyBeautyLayer.B(FLOAT); MyBeautyLayer.G(FLOAT); MyBeautyLayer.R(FLOAT)
  name                     string           'MyBeautyLayer'
  channels                 chlist           second socket.A(FLOAT); second socket.B(FLOAT); second socket.G(FLOAT); second socket.R(FLOAT)
  name                     string           'second socket'
```

So a layer written by that node takes its socket's name, spaces and all, and there is no view-layer part in front of it.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 5.1.2 open and the multi-layer OpenEXR from the test above loaded in the Image Editor. Seen: two dropdown menus in the header. The first, opened, shows `Beauty` and `Second` above a greyed label reading `Layer`. The second, opened, shows `Combined`, `Depth`, `Normal`, `Mist`, `Emission`, `Diffuse Color` above a greyed label reading `Pass`. Those are the names a person sees, and they are the two halves of the part names inside the file — `Beauty.Combined`, `Beauty.Depth`, `Beauty.Mist`, `Beauty.Normal`, `Beauty.Emission`, `Beauty.Diffuse Color` — with the view-layer half shown as its own menu rather than spelled out. So a person at the screen has the same names, split in two, that the file carries joined by a dot.

**Not captured: the View menu.** Only two menus were photographed, the one labelled `Layer` and the one labelled `Pass`. No screenshot shows a menu for views, so nothing is recorded here about one, and nothing should be read into its absence from these pictures.

**Not covered — and this was the sharp edge.** The sources never write out the actual layer or channel names that a rendered multi-layer EXR contains; the hand test above settles them by observation instead, on this machine and this build.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with Blender 5.1.2, no window, driven by the same scripts.

*A rendered multi-layer OpenEXR.* The reader printed the same version field, `0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)`, and the same twelve parts. The first of them:

```
--- PART 0 ---
  Software                 string           'Blender 5.1.2'
  channels                 chlist           Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT)
  colorInteropID           string           'lin_rec709_scene'
  name                     string           'Beauty.Combined'
```

and the renamed passes came out as `Beauty.Diffuse Color` and `Beauty.Emission`, with `Beauty.Depth`, `Beauty.Mist` and `Beauty.Normal` carrying `colorInteropID string 'data'`.

*A multi-layer OpenEXR written by a File Output node.* The same file name, `fromnode_0001.exr`, and the same two parts named `MyBeautyLayer` and `second socket`.

Both machines were run and both agreed, name for name.

**Observed — several views inside one file, on the rented Linux machine, 2026-08-06.** A case neither machine had tried before. Set up: 32 by 32, one frame, the depth and normal passes on, `render.use_multiview = True` and `render.views_format = 'STEREO_3D'`, output `OPEN_EXR_MULTILAYER`. Blender reported `VIEWS: [('left', True), ('right', True)]`, and with the image settings' own `views_format` left at its default it wrote **two** files, `frame_0001_L.exr` and `frame_0001_R.exr`. With `image_settings.views_format = 'MULTIVIEW'` it wrote **one**, and the reader printed six parts, two per pass:

```
--- PART 0 ---
  name                     string           'Beauty.Combined-left'
--- PART 1 ---
  name                     string           'Beauty.Combined-right'
--- PART 2 ---
  name                     string           'Beauty.Depth-left'
```

So a view is a suffix on the part's name, `<view layer>.<pass>-<view>`, and it is not in the channel names at all.

**Observed — the window on the rented Linux machine, 2026-08-06.** Blender 5.1.2 was opened with a window on that machine's own display and photographed with `bpy.ops.screen.screenshot`.

With the ordinary multi-layer OpenEXR loaded, the Image Editor's header holds **two** picture-choosing menus, reading `Beauty` and `Combined`, and no third one. So there is no View menu to photograph for such a file, and the Mac's silence was not a missed picture.

With the multi-view file loaded, a stereo-glasses button appears in the header and the two views are shown together, still with only two menus. Switching that button off — `space.show_stereo_3d = False` — makes a **third** menu appear, after `Layer` and `Pass`, reading `left`. That is the View menu.

**Its contents could not be photographed open.** The pointer was put on it with `xdotool` in Blender 4.5 and Blender drew the control's own tooltip, which reads `Select View.` — so Blender saw the pointer, and that is the control's name. But no button press ever reached Blender, on that machine, with several topics running windowed Blenders on the one display at the same moment. What the file itself records is `left` and `right`, quoted above.

Separately, the Image Editor's own `View` menu in the header — the one beside `Image` — was opened with `bpy.ops.wm.call_menu(name="IMAGE_MT_view")` and photographed. It holds `Toolbar`, `Sidebar`, `Tool Settings`, `Asset Shelf`, `Adjust Last Operation`, `Update Automatically`, `Show Metadata`, `Frame All`, `Center View to Cursor`, `Zoom`, `Area`, and nothing in it chooses which picture is shown.

**Outside the interface.** **Observed.** Both files above were written with no window open, and their names were read off the files themselves rather than out of Blender, so the naming holds for anything driving Blender from outside — which is exactly the case the gap above mattered for. Documented alongside: `OPEN_EXR_MULTILAYER` is one of the command line's `-F` values. What stays window-only is the Image Editor's Render Pass selector, which is how a person *at the screen* tells the pictures apart.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The same part names, the same multi-part shape and the same File Output node file name came back on both. The multi-view results and the window pictures above were got on the Linux machine alone, so they settle that system and leave the Mac untried for those. **Windows is still untried** — no Windows machine has run it. From the sources: nothing ties OpenEXR multi-layer behaviour to an operating system, and OpenEXR is on the list of formats "not available on all systems" if not compiled in.

---

## 6. Does a format that can hold more light than a screen can show also record the treatment it was made under?

I take "a format that can hold more light than a screen can show" to mean a high-dynamic-range format — chiefly OpenEXR, and also Radiance HDR, and (per this version's docs) AVIF and PNG for HDR display output. I take "the treatment it was made under" to mean the colour management applied when the picture was made: the view transform, look, exposure and the rest. That was the reading I answered. The other reading is narrower — "the treatment" meaning only which colour space the numbers are in — and I answer that too, separately, because the sources split on exactly that line.

**Answer, short.** Partly established.

- **OpenEXR can record which colour space the numbers are in**, through an identifier described in this version's release notes. The manual says this depends on the OpenColorIO configuration, so it is not unconditional.
- **Whether the file records the view transform, look, exposure or other treatment is not in the sources.** The manual says Blender cannot automatically detect that a loaded OpenEXR was a render, and its Metadata panel does not list those settings. Neither statement proves that no such information exists anywhere in the file.
- The manual's format table marks Radiance HDR and AVIF as supporting no metadata. It marks PNG as supporting metadata, but does not say that this includes colour treatment.

**LABEL: documented** for conditional colour-space recording and the format table; **not in the sources** for whether the view transform, look, exposure or other treatment is recorded.

**The words that carry the first half.**

> "OpenColorIO was updated to 2.5, and for configuration authors the following new attributes are supported. The bundled Blender configuration was also updated to make use of these.
>
> - `interop_id` for identifying the color space and embedding in OpenEXR files.
> - `icc_profile` to specify the ICC profile to be embedded when saving an image file."

*Source:* Blender 5.1 release notes, https://developer.blender.org/docs/release_notes/5.1/rendering/ — read 2026-07-30.

The manual describes the same thing as conditional on how the configuration is written:

> "Use OpenColorIO v2 display_colorspaces and view_transforms with cie_xyz_d65_interchange
> intermediate display linear space. … In older configs without this, display emulation will be disabled and wide gamut and HDR display will
> not work well. Saving colorspace metadata for images and video also depends on this."

and

> "Include the interop ID from the Color Interop Forum
> for every color space and display color space that you can. This helps save image and video
> with correct colorspace information."

and

> "The icc_profile_name interchange attribute is supported, to embed ICC profiles when saving images."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/color_management/opencolorio.html — read 2026-07-30.

Blender reads some of this back on opening:

> "When opening OpenEXR images, Blender will automatically set the
> Color Space if it is detected as Linear CIE-XYZ E or ACES2065-1."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html — read 2026-07-30.

The API exposes the identifier:

> "working_space_interop_id
>
> Unique identifier for common color spaces, as defined by the Color Interop Forum. May be empty if there is no interop ID for the working space. Common values are lin_rec709_scene, lin_rec2020_scene and lin_ap1_scene (for ACEScg) (default "", readonly, never None)"

*Source:* Blender Python API, https://docs.blender.org/api/5.1/bpy.types.BlendFileColorspace.html — read 2026-07-30. Note the phrase "May be empty".

The release notes also record a defect fixed in this area, which is a documented statement about the behaviour being imperfect:

> "Single-layer EXR does not pick up scene working color space in some cases"

*Source:* Blender 5.1 release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/ — read 2026-07-30.

**The evidence surrounding the unresolved second half.**

The manual states that Blender cannot automatically detect that the loaded file was a render:

> "By default, only renders are displayed and saved with the render View applied.
> These images are the "Render Result" and "Viewer" image data-blocks,
> and the files saved directly to disk with the Render Animation operator.
> However, when loading a render saved to an intermediate OpenEXR file,
> Blender cannot detect automatically that this is a render
> (it could be e.g. an image texture or displacement map).
> We need to specify that this is a render and that we want the transformations
> applied, with the following settings."

with the setting being:

> "View as Render
>
> Display the image data-block (not only renders) with view, exposure, gamma, RGB curves applied.
> Useful for viewing rendered frames in linear OpenEXR files the same as when rendering them directly."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/color_management/displays_views.html — read 2026-07-30.

The manual's Metadata panel lists Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name and Note. It lists nothing about colour management.

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html — read 2026-07-30.

**Documented, from a picture.** The Metadata panel screenshot shows exactly those checkboxes and no others: Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, "Use Strip Me…", a Note field, and a Burn Into Image section with Font Size, Text Color, Background and Include Labels. No colour-management entry is visible.

*Source:* Blender manual, picture `_images/render_output_properties_metadata_panel.png` on https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html — read 2026-07-30.

**Which HDR-capable formats can carry metadata at all**, from the manual's table (columns: Format, Channel Depth, Alpha, Metadata, DPI): **OpenEXR** — float 16, 32-bit, alpha yes, **metadata yes**, DPI yes. **Radiance HDR** — float, alpha yes, **metadata no**, DPI no. **AVIF** — 8, 10, 12-bit, alpha yes, **metadata no**, DPI no. **PNG** — 8, 16-bit, alpha yes, metadata yes, DPI yes. The page adds:

> "Not all formats support metadata,
> and some may only support a limited subset of fields.
>
> When using formats that do not support metadata,
> render information is not preserved in the saved file."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html — read 2026-07-30.

By that table, Radiance HDR does not support metadata. The table does not settle whether some other part of the format records colour information.

**One more documented fact that bears on the question.** For HDR pictures and HDR video, the manual states a fixed convention that is itself part of "the treatment":

> "HDR images can be written as AVIF or PNG files. They are written with 203 nits diffuse
> white to match the convention in most browsers and image viewers."

and

> "HDR videos are written with 100 nits diffuse white, to match the convention in most video
> players and browsers."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/color_management/displays_views.html — read 2026-07-30. The manual does not say this figure is written into the file; it says the pictures are written *with* it.

And the default treatment of the linear formats:

> "Note, Cineon, DPX, OpenEXR, and Radiance HDR
> image types default to being saved in a linear color space."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html — read 2026-07-30.

**Where I looked.** Read all three colour management pages in full (`color_spaces.txt`, `displays_views.txt`, `opencolorio.txt`). Read `render/output/properties/metadata.txt` in full and opened its panel picture. Read the Metadata section and the whole OpenEXR section of `files/media/image_formats.txt`. Read `release_notes_5.1/rendering.md` in full and searched the whole 5.1 release notes for image and colour terms. Searched the manual for "chromaticit" (8 hits), "icc" (1 hit), "interop_id" (1 hit), "colorspace metadata" / "color space metadata" / "colorspace information" / "embed" (20 hits, three on point, all quoted above). Searched the API for "interop" (4 hits) and read `bpy.types.BlendFileColorspace`, `bpy.types.ColorManagedViewSettings` and `bpy.types.ImageFormatSettings`. Listed every `stamp` property on `bpy.types.RenderSettings` — `stamp_background`, `stamp_font_size`, `stamp_foreground`, `stamp_note_text`, `use_stamp`, `use_stamp_camera`, `use_stamp_date`, `use_stamp_filename`, `use_stamp_frame`, `use_stamp_frame_range`, `use_stamp_hostname`, `use_stamp_labels`, `use_stamp_lens`, `use_stamp_marker`, `use_stamp_memory`, `use_stamp_note`, `use_stamp_render_time`, `use_stamp_scene`, `use_stamp_sequencer_strip`, `use_stamp_time` — and confirmed none concerns colour management.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.1.2, started from a terminal with no window.

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
  Date                     string           '2026/08/03 22:45:42'
  File                     string           '<untitled>'
  Frame                    string           '001'
  RenderTime               string           '00:00.45'
  Scene                    string           'Scene'
  Software                 string           'Blender 5.1.2'
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

So the attribute the release note points at is named **`colorInteropID`**, and its value here is `lin_rec709_scene`. Setting the Date and RenderTime lines aside, the three OpenEXR headers were the same text: the one made under Standard matched the one made under AgX, and so did the one made with a Look and with exposure and gamma changed. The only difference between those files was how many bytes of pixels followed the header. No attribute names a view transform, a look, an exposure or a gamma; there is no `chromaticities` attribute and no ICC profile attribute in these files.

To check what `colorInteropID` follows, the working space was changed with `bpy.ops.wm.set_working_color_space(working_space='ACEScg', convert_colors=False)` and another OpenEXR written. Blender printed `working_space after: ACEScg interop id: lin_ap1_scene`, and that file's header reads `colorInteropID string 'lin_ap1_scene'`. So the attribute carries the working space, not the treatment.

The Radiance HDR file begins:

```
#?RADIANCE
FORMAT=32-bit_rle_rgbe

-Y 32 +X 32
```

and then the pixels. Nothing else is in its header — matching the format table's ✗ for metadata.

The PNG's blocks, read in order, were `IHDR, sRGB, gAMA, cHRM, eXIf, oFFs, pHYs, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, IDAT`, with `sRGB` reading `03`, `gAMA` reading `0000b18f` and `cHRM` reading `00007a26000080840000fa00000080e8000075300000ea6000003a9800001770`. Its seven text entries were `File = <untitled>`, `Date = 2026/08/03 22:45:44`, `Time = 00:00:00:01`, `Frame = 001`, `Camera = Camera`, `Scene = Scene`, `RenderTime = 00:00.32`. So a PNG written by this version does carry colour blocks describing the display space it is in — and still nothing naming the view transform, look, exposure or gamma.

*Opening the files again.* Done, in a windowless Blender: `bpy.data.images.load(...)` on each file, then its colour space and View-as-Render state printed:

```
OPENED exr_agx.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_standard.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_punchy_exposed.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED radiance.hdr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED display_png.png size=(32, 32) depth=96 is_float=True colorspace='sRGB' use_view_as_render=False
```

All three OpenEXR files come back the same way whatever they were made under, with View as Render off.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.1.2, no window. The same files were written under the same changed settings and the same readers run over them. Blender printed `DEFAULT view_transform='AgX' look='None' exposure=0.0 gamma=1.0` and `BLEND FILE COLORSPACE working_space = 'Linear Rec.709'`, `working_space_interop_id = 'lin_rec709_scene'`, `is_missing_opencolorio_config = False`.

The three OpenEXR headers came back as the same text apart from the date and the render time: no attribute names a view transform, a look, an exposure or a gamma, and there is no `chromaticities`. Each carries `colorInteropID string 'lin_rec709_scene'`. After `bpy.ops.wm.set_working_color_space(working_space='ACEScg', convert_colors=False)`, which reported `working_space after: ACEScg interop id: lin_ap1_scene`, the next OpenEXR carried `colorInteropID string 'lin_ap1_scene'`.

The Radiance file begins `#?RADIANCE` and `FORMAT=32-bit_rle_rgbe` and holds nothing else. The PNG's blocks, read in order, carry the colour blocks this version added:

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

**Not covered.** Which attribute names and values land in an OpenEXR header is settled above for these cases. What is still open: whether the bundled configuration covers every colour space with an interop identifier, and when an ICC profile would be embedded. Whether the bundled configuration covers every colour space with an interop ID (the API says the value "May be empty"). Whether the ICC profile embedding applies to OpenEXR or only to display formats. Whether the look, exposure, gamma, curves or white balance are recorded anywhere at all — I found nothing saying they are, and nothing saying they are not, beyond the metadata list that omits them.

**Outside the interface.** **Observed.** Every file above was written, and every header read, with no window open — the view transform, the look, the exposure and the gamma were all set from a script, the working space was changed by an operator, and the renders were driven from a terminal. So the answer holds when Blender is driven from outside its own window: the colour space is written, in `colorInteropID`; the treatment is not. Documented alongside: the OpenColorIO configuration can be chosen from outside Blender: "Blender will use the standard OCIO environment variable to read an OpenColorIO configuration other than the default Blender one" and "There is also a BLENDER_OCIO environment variable to change the configuration for Blender only" (same OpenColorIO page, read 2026-07-30). "View as Render" is documented as an interface setting, and the sources do not describe an outside equivalent.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** A file written on either machine carries the same attributes, including `colorInteropID` and the PNG's colour blocks, and reopens the same way. **Windows is still untried** — no Windows machine has run it. From the sources: they say nothing tying colour-space metadata writing to any operating system. The *display* side does carry operating-system conditions — wide gamut and HDR viewing require, per the manual, "macOS: Any Apple Silicon device. Linux: Use Wayland, and set the Vulkan backend in the Blender system preferences. Windows: Enable "Use HDR" in the Windows display settings…" — but that is about seeing HDR on screen, not about what a written file records.

---

## 7. Can Blender turn a set of finished frames into a short moving picture?

**Answer.** Yes. The manual describes this as the recommended way to work, and gives the steps.

**LABEL: documented.**

**The words that carry it.**

The whole approach, named:

> "Frame Sequence
>
> The Frame Sequence is a much more stable approach,
> where you set your output format to a still format (such as JPG, PNG or a multi-layer format).
> Click Animation to render your scene out to a set of images,
> where each image is a frame in the sequence.
>
> Blender creates a file for each frame of the animation.
> You can then use Blender's Compositor to perform any frame manipulation (post-processing).
> You can then use Blender's Video Sequencer to load that final image sequence,
> add an audio track to the animation, and render out to an MPEG format to complete your movie."

and the steps, quoted from the same page:

> "In Blender, open the Video Sequencer."
> "Choose Add Image from the add menu. Select all the frames from your output folder that you want to include
> in your animation. They will be added as a strip in the Sequence editor."
> "In the Output panel, choose the container and codec you want (e.g. MPEG H.264) and configure them."
> "Click the Animation render button and Blender will render out the Sequence editor output into a movie."

and, as a hint:

> "You can then make a movie out of the separate frames with Blender's Sequence editor
> or use 3rd party encoding software."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

The Output page recommends the same thing and says why:

> "It's not recommended to render directly to a video format in the first instance.
> If a problem occurs while rendering, the file might become unplayable and you will
> have to re-render all frames from the beginning. If you first render out a set
> of static images such as the default PNG format or the higher-quality OpenEXR
> (which can retain HDR pixel data), you can combine them as
> an Image Strip
> in the Video Sequencer."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

What kind of strip holds the frames:

> "Image strips are used to add still images or numbered image sequences
> to the Video Sequencer. They are commonly used for slides, graphic overlays,
> background plates, rendered image sequences, or frame-by-frame animation."

and how a numbered set is recognised:

> "In the case of (numbered) image sequences
> (e.g. \*-0001.jpg, \*-0002.jpg, \*-0003.jpg, etc, of any image format), you have a choice:"

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/image.html — read 2026-07-30.

And the switch that makes a render come from the sequencer rather than from the 3D scene:

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30. The API states the same and gives its default: "use_sequencer — Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)" (https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html, read 2026-07-30).

There is a route that does not use the sequencer at all, for files it cannot take:

> "The Video Sequencer does not support multi-layer EXR files.
> To render to a video format you will have to skip the next three steps and
> instead use an Image Input node in the Compositor."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

Also relevant: the frames need not all be the same kind of file.

> "Batch
>
> Shift-LMB click selected non-related stills for batch processing; each image will be one frame,
> in sort order, and can be a mix of file types (jpg, png, exr, etc.)."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html — read 2026-07-30.

**Where I looked.** Read `render/output/animation.txt` in full, `render/output/properties/output.txt` in full, `render/output/properties/post_processing.txt` in full, `video_editing/edit/montage/strips/image.txt`, `files/media/video_formats.txt` in full, and the video-related entries in `bpy.types.RenderSettings`. Listed the whole `video_editing/` tree to check nothing else covered it.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.1.2, started from a terminal with no window throughout. **This settles the disagreement between this document and `Preparation/Blender Research/Findings/5.1/Images — second pass.md`, which reads the outside-the-window half differently. The observed result below is what now stands, and the same result is written into that document too.**

*The whole job with no window at all.* Set up, in a windowless Blender: six PNG frames at 64 by 48 rendered from the ordinary scene into a folder; then the 3D scene changed so a fresh render could not be mistaken for those frames — the cube deleted and the world background set to pure blue; then an image strip built from the six saved files with `scene.sequence_editor_create()` and `strips.new_image(name="Frames", filepath=<first file>, channel=1, frame_start=1)` followed by `strip.elements.append(<each remaining file name>)`. Blender printed `STRIP: Frames type: IMAGE elements: 6` and `use_sequencer: True`. Four blend-files were saved from that state: video output with the Sequencer pipeline on, video output with it off, PNG output with it on, PNG output with it off.

Done, in a terminal:

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b -noaudio --factory-startup /tmp/images-hand-tests/out/t16/5.1.2/sequence-test.blend -a
```

Blender printed, for each frame, `Rendering frame N`, `Executing sequencer`, `Video append frame N`. The exit status was `0`, and one file appeared at the output path: `out_0001-0006.mp4`, which `file` reported as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. The same command on the file with the Sequencer pipeline switched off wrote `nosec_0001-0006.mp4`.

To see which pictures went into it, the same two files were rendered again with PNG output instead of video, and the written frames' pixels read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

With the Sequencer pipeline on, what came out is the saved frames — grey cube on dark grey. With it off, what came out is the changed 3D scene — blue, no cube. The six Sequencer frames took 0.3 seconds in total; the six 3D renders took about that each.

*A folder of frames with nothing prepared.* `blender -b --factory-startup -F FFMPEG -o <folder>/n_ -a` against the plain startup file wrote `n_0001-0003.mkv` — a movie of the 3D scene, not of any folder of pictures. There is no command-line argument that turns a folder of frames into a movie by itself.

*A frame missing from the set at the moment of encoding.* Set up: five frames written, a strip built naming all five, the file saved, and then the third frame's file deleted from disk. Done: the same background animation render, to PNG. All five output frames were written, nothing was printed about the missing one, and their pixels read:

```
s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3254
s_0002.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3254
s_0003.png   size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1243
s_0004.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3254
s_0005.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3254
```

The frame whose file was gone came out fully transparent black.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Set up: three multi-layer OpenEXR frames at 64 by 48 with the depth and normal passes on; a strip built from them; PNG output. Done: the same background animation render. Three files were written, with no message about the format:

```
s_0001.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3002
s_0002.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3002
s_0003.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3002
```

Compared with the same scene's PNG frames put through the same route, which gave `centre=(0.5373, ...)` and `corner=(0.2, ...)`, the picture is there but darker.

*How long the result is.* Set up: twenty-four frames, a strip built from all of them, video output with the MPEG-4 container and the H.264 codec. Done: encoded once with `render.fps = 24` and once with `render.fps = 12`, then each movie's own header read for the time scale and duration it records:

```
at24_0001-0024.mp4   bytes=2314   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4   bytes=2592   timescale=1000 duration=2000 -> 2.000 s
```

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 5.1.2 open. Seen: the Output panel's top control, labelled `Media Type`, offers `Image`, `Multi-Layer EXR` and `Video`; with `Video` chosen, the `Encoding` section's `Container` menu offers `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`, and the `Video Codec` menu offers `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`. So this build really does offer video output, and the container and codec used in the test above are both on those menus.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.1.2, with no window throughout, driven by the same scripts.

*The whole job with no window at all.* `blender -b -noaudio --factory-startup .../sequence-test.blend -a` printed `Executing sequencer` and `Video append frame 1` through `Video append frame 6` on timestamped lines, the exit status was `0`, and `out_0001-0006.mp4` appeared, reported by `file` as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. Rendered again to PNG, the frames read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

*A folder of frames with nothing prepared.* The same command wrote `n_0001-0003.mkv`, reported as `Matroska data` — a movie of the 3D scene.

*A frame missing from the set.* All five output frames were written, nothing was printed about the missing one, and the third came out fully transparent black: `s_0003.png size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1809`.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Three files written with no message about the format, `centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0)`.

*How long the result is.* The two movies' own headers record:

```
at24_0001-0024.mp4 bytes=2314   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4 bytes=2597   timescale=1000 duration=2000 -> 2.000 s
```

Both machines were run and both agreed on every one of these, with only the byte sizes of the written files differing by a few bytes.

*Which container and codec will actually take it.* A further test, recorded in full under question 4 and run on both machines, encoded three frames for every container with H.264 and for the MPEG-4 container with every codec. Ten of those pairings are refused with `Error: Could not initialize streams, probably unsupported codec combination`, and three more with a complaint about the picture size. So the two menus offer more pairings than this version will write.

**Not covered.** How the strip is built from outside Blender's window is settled by the hand test above; the sources still describe only the menu actions. Still open: what a *movie* file shows for a missing frame, as against the PNG frames read here.

**Outside the interface.** **Observed — yes, the whole job.** Both halves were done with no window open: the image strip was built from the finished frames by a script in a windowless Blender, and the movie was encoded by `blender -b <file> -a` from a terminal. The written frames prove the pictures came from the strip and not from a fresh 3D render. (Previously, before the test: the render half was documented and the assembly half was left for a fresh search.)

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The whole job — building the strip with no window and encoding it from a terminal — works on both, and the frames, the file types and the recorded durations match. This is the machine a Smoozy render actually happens on, and FFmpeg is present in its build too (`codec_ffmpeg=True`, recorded under question 3). **Windows is still untried** — no Windows machine has run it. From the sources: nothing ties the conversion steps to an operating system, and `FFMPEG` is on the command-line page's list of formats "not available on all systems".

---

## 8. Can Blender be made to produce a smaller, plainly viewable copy of a picture a render produced?

**Answer.** The sources establish the parts, but not one fully documented route. A **Preview** option writes a JPEG copy alongside the render, including for single frames rendered from the command line. The sources do not say that this copy has fewer pixels.

A smaller, plainly viewable copy can be inferred by pairing the documented Scale node with the documented File Output node in a display format. One source says the Scale node changes an image's pixel dimensions. The other says the File Output node writes each connected input during rendering and lets an output use its own format. The sources do not state this combined recipe in one place.

**LABEL: documented** for the Preview JPEG and for each separate node; **inferred** for the combined smaller-copy route; **not in the sources** for whether the Preview JPEG itself has fewer pixels.

**The words that carry it.**

> "Preview
>
> When rendering animations (or single frames via command line),
> save a JPEG copy of the image, for a quick preview."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html (in "Format Details ‣ OpenEXR ‣ Output Options") — read 2026-07-30.

The API describes the same switch, and describes it on the general image-format settings rather than on anything OpenEXR-specific:

> "use_preview
>
> When rendering animations, save JPG preview images in same directory (default False)"

*Source:* Blender Python API, https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html — read 2026-07-30. Note the two descriptions differ in a small way worth flagging: the manual adds "(or single frames via command line)", the API says only "When rendering animations". They agree that the copy is a JPEG and that it sits beside the original ("in same directory").

**The documented parts of the inferred smaller-copy route, and other nearby routes.**

The thing that makes a copy *plainly viewable* rather than linear:

> "Save As Render
>
> Save image with render color management.
> For display image formats like PNG, apply view and display transform.
> For intermediate image formats like OpenEXR, use the default render output color space."

and, on the same subject from the colour-management side:

> "Save as Render
>
> Option in the image save operator to apply the view, exposure, gamma, RGB curves.
> This is useful for saving linear OpenEXR to e.g. PNG or JPEG files in display space."

*Sources:* Blender manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html and https://docs.blender.org/manual/en/5.1/render/color_management/displays_views.html — both read 2026-07-30.

The thing that makes it *smaller*, in the compositor, so it can be produced as part of a render:

> "Scale Node
>
> This node scales the size of an image.
>
> Space
>
> Coordinate Space to scale relative to.
>
> Relative: Percentage values relative to the dimensions of the image input.
>
> Absolute: Size of an image by using absolute pixel values."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/compositing/types/transform/scale.html — read 2026-07-30.

Paired with a second output written in a display format:

> "The File Output node writes one or more images to disk during rendering or compositing.
> Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence."

and

> "Image:
>
> Saves each input as a separate image file.
> Each input can use its own image format settings."

and

> "Override Node Format
>
> Allows the selected output to use a custom format instead of the global Node Format."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30.

By hand, in the Image Editor, there is a direct resize and a save-a-copy:

> "Resize
>
> Adjusts the image dimensions by scaling its pixel resolution. This is useful for various tasks, such as:
>
> Reducing texture resolution to optimize performance and memory usage."

and

> "Save a Copy
>
> Save the file under a specified name,
> but keep the old one open in the Image editor."

*Source:* Blender manual, https://docs.blender.org/manual/en/5.1/editors/image/editing.html — read 2026-07-30.

**Two things I checked and am ruling out as answers, for the record.** Neither is a copy of a picture the render produced.

- "Percentage — Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image." (https://docs.blender.org/manual/en/5.1/render/output/properties/format.html, read 2026-07-30) — this makes the render itself smaller, rather than a smaller copy of it.
- Sequencer proxies: "Blender can generate proxies – copies of the original footage stored at a lower quality and/or resolution – to maintain a smooth editing experience without compromising visual fidelity in the end result." (https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/sidebar/proxy.html, read 2026-07-30) — these are smaller copies, but of footage in the Sequencer, made for preview performance. If rendered frames were loaded back as a strip they would qualify, but the sources describe them as an editing aid, not as an output of a render.

**Where I looked.** Read `files/media/image_formats.txt` in full (which is where the Preview option lives). Read `bpy.types.ImageFormatSettings` in full and searched the API index for `use_preview` (`use_preview` appears once, on `ImageFormatSettings`). Read `editors/image/editing.txt` in full — Save, Save As, Save a Copy, Save All Images, Resize, Transform, Pack, Unpack, Extract Palette. Read `compositing/types/output/file_output.txt` and `compositing/types/transform/scale.txt`. Read `render/output/properties/format.txt`. Read `editors/video_sequencer/sequencer/sidebar/proxy.txt` and the Proxy Render Size section of `editors/video_sequencer/preview/sidebar.txt`. Searched the whole manual for "thumbnail" (28 hits — File Browser display mode, asset browser, sequencer strip overlays, blend-file previews; none is a copy of a render).

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.1.2, started from a terminal with no window.

*The Preview copy.* Set up: 64 by 48, frames 1 to 3, `image_settings.use_preview = True`, first with `file_format = 'OPEN_EXR'` and then with `'PNG'`. Done: `bpy.ops.render.render(animation=True)` each time, then every written file's byte size and pixel size read. Blender printed:

```
--- OPEN_EXR, use_preview set to True, now reads True ---
  FILE anim_0001.exr                3403 bytes  64x48
  FILE anim_0001.jpg                1136 bytes  64x48
  FILE anim_0002.exr                3403 bytes  64x48
  FILE anim_0002.jpg                1136 bytes  64x48
  FILE anim_0003.exr                3403 bytes  64x48
  FILE anim_0003.jpg                1136 bytes  64x48
--- PNG, use_preview set to True, now reads True ---
  FILE anim_0001.png                3011 bytes  64x48
  FILE anim_0002.png                3011 bytes  64x48
  FILE anim_0003.png                3011 bytes  64x48
```

So the Preview copy has the **same number of pixels** as the main file and about a third of the bytes; it sits beside it with the same stem and a `.jpg` ending. And it is written for OpenEXR only: with PNG the setting can be switched on and reads back as on, and no JPEG copy appears.

Then a blend-file was saved with OpenEXR output and Preview on, and one frame rendered from a terminal on the Creator's ordinary configuration (not factory settings):

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b -noaudio /tmp/images-hand-tests/out/t12/5.1.2/preview-test.blend -f 1
```

Blender printed `Saved: '.../CLI/cli_0001.exr'` and `Saved: '.../CLI/cli_0001.jpg'` — 2193 bytes and 1189 bytes.

*A smaller second copy made during the render.* Set up: 64 by 48, one frame, main output PNG; a compositing node group with the Render Layers image going to the Group Output and also through a Scale node with X and Y set to 0.5 into a File Output node writing PNG into the same folder. This version's Scale node has no `space` setting; its Type input socket already read `Relative`. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
SCALE NODE inputs: ['Image', 'Type', 'X', 'Y', 'Frame Type', 'Interpolation', 'Extension X', 'Extension Y']
SCALE NODE has no space property; Type socket reads: 'Relative'
OPERATOR RESULT: {'FINISHED'}
FILE main_0001.png          2812 bytes  64x48  depth=32 is_float=False
FILE small_small_0001.png   1324 bytes  32x24  depth=128 is_float=True
```

*Resizing a rendered picture and saving a copy.* Set up: one OpenEXR render at 640 by 360, loaded back with `bpy.data.images.load(...)`. Done, in order, with whatever each returned kept:

```
--- bpy.ops.image.resize(size=(320, 180)) ---
RAISED RuntimeError: Operator bpy.ops.image.resize.poll() failed, context is incorrect
--- image.scale(320, 180) ---
scale() returned, size now (320, 180)
--- bpy.ops.image.save_as(filepath='/tmp/images-hand-tests/out/t14/5.1.2/small.png', save_as_render=True, copy=True) ---
RAISED RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect
small.png exists: False
--- image.save_render('/tmp/images-hand-tests/out/t14/5.1.2/small_render.png') ---
save_render returned
--- scene format set to PNG, then image.save_render('/tmp/images-hand-tests/out/t14/5.1.2/small_as_png.png') ---
save_render returned
FILE big.exr               57205 bytes  640x360
FILE small_as_png.png      33686 bytes  320x180
FILE small_render.png      48365 bytes  320x180
```

The two Image Editor commands refuse to run with no window. The two plain calls work. One thing to note about the first `save_render`: although the path ends `.png`, `file` reports that file as `OpenEXR image data, version 2 ... dataWindow: (0 0)-(319 179)`, because `save_render` writes in the scene's own output format. After the scene's format was set to PNG, the second call wrote `PNG image data, 320 x 180, 16-bit/color RGBA, non-interlaced`.

*Whether a saved proxy setup builds during a background run.* Set up: three finished PNG frames at 128 by 96; an image strip built from them; `strip.use_proxy = True` and `strip.proxy.build_25 = True` with the other three sizes off; the sequence editor's proxy storage reading `PER_STRIP`; output PNG; the file saved; no `BL_proxy` folder present. Done, in a terminal: `blender -b .../proxy-test.blend -a`. Blender printed only the three `Saved:` lines for the output frames, the exit status was `0`, and no `BL_proxy` folder existed anywhere under the test folder afterwards.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 5.1.2 open. Looked at: Properties ▸ Output Properties ▸ Output panel, with the format changed in turn, watching for a control named `Preview`. Seen: with `PNG (.png)`, `JPEG (.jpg)`, `TIFF (.tif)` and `AVIF (.avif)` there is no `Preview` control anywhere in the panel. With `OpenEXR (.exr)` a checkbox reading `Preview` appears under `Codec`, unticked. With `Media Type` set to `Multi-Layer EXR` the same `Preview` checkbox appears, unticked, below one reading `Interleave`. So the interface offers Preview only for the two OpenEXR cases — that settles the open point about whether it exists for other formats, and it matches the terminal result above, where switching `use_preview` on for PNG produced no JPEG copies.

**Not covered — and it mattered.** Both open points are settled above: the Preview copy has the same pixel size and about a third the bytes, and it is written for OpenEXR and not for PNG. Still open: the JPEG quality it is written at, and what colour treatment is applied to it.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.1.2, no window, driven by the same scripts.

*The Preview copy.* Blender printed:

```
--- OPEN_EXR, use_preview set to True, now reads True ---
  FILE anim_0001.exr                3378 bytes  64x48
  FILE anim_0001.jpg                1136 bytes  64x48
  FILE anim_0002.exr                3378 bytes  64x48
  FILE anim_0002.jpg                1136 bytes  64x48
  FILE anim_0003.exr                3378 bytes  64x48
  FILE anim_0003.jpg                1136 bytes  64x48
--- PNG, use_preview set to True, now reads True ---
  FILE anim_0001.png                3003 bytes  64x48
  FILE anim_0002.png                3003 bytes  64x48
  FILE anim_0003.png                3003 bytes  64x48
```

The single-frame command-line run wrote `cli_0001.exr` at 2186 bytes and `cli_0001.jpg` at 1183 bytes.

*A smaller second copy made during the render.* Blender printed:

```
SCALE NODE inputs: ['Image', 'Type', 'X', 'Y', 'Frame Type', 'Interpolation', 'Extension X', 'Extension Y']
SCALE NODE has no space property; Type socket reads: 'Relative'
FILE main_0001.png          2816 bytes  64x48  depth=32 is_float=False
FILE small_small_0001.png   1285 bytes  32x24  depth=128 is_float=True
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
FILE big.exr               56576 bytes  640x360
FILE small_as_png.png      33810 bytes  320x180
FILE small_render.png      48325 bytes  320x180
```

`file` reported `small_render.png` as `OpenEXR image data, version 2 … dataWindow: (0 0)-(319 179)` and `small_as_png.png` as `PNG image data, 320 x 180, 16-bit/color RGBA, non-interlaced`, exactly as on the Mac.

*Whether a saved proxy setup builds during a background run.* Only the three output frames were written, the exit status was `0`, and no `BL_proxy` folder existed anywhere afterwards.

Both machines were run and both agreed on every part of this, with only byte sizes differing slightly.

**Outside the interface.** **Observed, and it splits three ways.** The Preview copy works from outside: it was written for a three-frame animation and for a single frame started from a terminal. The Scale-plus-File-Output route works from outside: the half-size second file was written by a windowless render. The two Image Editor commands do **not** work from outside: `bpy.ops.image.resize` and `bpy.ops.image.save_as` both refuse with `poll() failed, context is incorrect`, and the plain calls `Image.scale` and `Image.save_render` do the same job instead — with the catch that `save_render` writes in the scene's output format, whatever the path's ending says. Proxies were not built during a background animation render.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The Preview copy, the half-size second file, both refusals, both plain calls and the absence of proxies all came back the same on both. **Windows is still untried** — no Windows machine has run it. From the sources: nothing ties any of this to an operating system, and JPEG is on the command-line page's always-available list.

---

## Things a person with Blender 5.1 open could settle in a few minutes

All seven items on this list have been run, on the Creator's own MacBook Pro (M1 Pro, macOS 26.5.2), and the results are written in above under the questions they answer. The whole list is at `Audits/Blender Research/Images/Hand tests.md`. Item 3 was answered in part: whether the setting takes and whether a JPEG appears was settled from a terminal, but whether a **Preview checkbox is shown** in the Output panel for each format still needs eyes on the window. Item 6 was answered for the finished picture; which separate passes show the magenta was not read.

The three checks that needed eyes on Blender's window were run by the Creator on 2026-08-04 and are written in above, which also closes the second half of item 3: the Preview control is offered for OpenEXR and Multi-Layer EXR and for nothing else.

Everything on that list was then run again on a second machine — a rented Linux machine, Ubuntu 22.04.5 LTS with an NVIDIA GeForce RTX 4090 — on 2026-08-06, together with the tests the one Mac could never reach. That second list is at `Audits/Blender Research/Images/Hand tests — Linux.md`, and its results are written in above beside the Mac's. **Three of the five gaps below are now closed**: the Image Editor's View menu, another make of graphics card, and a container-and-codec pairing Blender itself refuses.

What is still open:

1. **Windows.** Every observed answer above was got on macOS on Apple silicon or on Ubuntu on an NVIDIA card, and on both where the test could be repeated. No Windows machine has run it, so that system stays untried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

2. **A build that lacks a format.** (Question 4.) Still cannot be produced. Both machines were checked and the official build for each has every optional format compiled in. It needs a Blender built without one of them.

3. **Which separate passes show a missing texture.** (Question 2.) The finished picture was read for every engine and every case on both machines; the individual colour passes were not.

4. **A graphics card that is neither Apple's nor NVIDIA's.** (Question 2.) The missing-texture result now covers Apple's Metal graphics, an NVIDIA card through CUDA and the same card through OptiX. AMD graphics, Intel graphics and a second card of any make cannot be tried.

5. **What the View picker's open menu looks like.** (Questions 1 and 5.) The picker itself is now photographed — it exists only for a file with several views in it, appears when the stereo button is off, is called `Select View.` and reads `left`. Opening it needs a click, and on the shared Linux display no button press reached Blender.

---

2026-08-03 — Opus 5 (Claude Code) — Images, Blender 5.1

# Images — Blender 5.0

**Sources consulted, and when.** Blender's 5.0 manual (`Blender Sources/manual_5.0`, text and pictures), Blender's 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender's 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`). All four are the snapshot taken on **2026-07-30**, and that is the date read for every finding below.

**Declaration.** I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project — with one thing I must name plainly, because it is not quite nothing: before writing this file I ran a directory listing on the folder I was told to save into, `Preparation/Blender Research/Findings/5.0/`, to confirm it existed. That listing showed me the names of six files sitting there. I opened none of them, and I read nothing but their names. Nothing in this document comes from anywhere except the question sheet and `Blender Sources/`.

---

## 1. What kinds of picture can a render produce, and how can the finished picture be told apart from the others?

**The question has two readings.** "Kinds of picture" could mean the different images a single render yields (passes, layers, views), or the file types it can be written as. I answer the first, because question 3 asks the second. If the second was meant, the answer is question 3.

**Answer.** A render can produce more than one image. It produces, per scene:

- one or more **View Layers**, each a separately rendered slice of the scene;
- within each View Layer, one or more **Passes** — separate images carrying different intermediate information;
- optionally more than one **View** (the stereo/multi-view left and right images).

The finished picture is the pass named **Combined**. The manual describes it as the final image, and the Python interface calls it the "full combined RGBA buffer".

Cycles' own file lists the Combined pass with no condition before the conditional pass entries, and shows the exact names used in that list. This establishes how the pass list is declared, not what a running render writes.

**LABEL: documented** (the manual and API statements below) **and observed** (the Cycles file below).

**Quoted words.**

> "A Pass is a type of intermediate rendering information that's extracted as a separate image. Examples include the diffuse colors of the objects in the scene, the light distribution, the depth map, and the normal map."

> "Combined Cycles, EEVEE, Workbench
>
> The render output before any compositing is applied."

Manual — https://docs.blender.org/manual/en/5.0/render/layers/passes.html — read 2026-07-30

The same page lists the full set of passes on offer, under the headings Data (Combined, Depth, Mist, Position, Normal, Vector, UV, Grease Pencil, Denoising Data), Indexes (Object Index, Material Index), Debug (Render Time, Sample Count), Light (Diffuse / Glossy / Transmission / Volume, each split Direct / Indirect / Color; Emission, Environment, Ambient Occlusion, Shadow Catcher, Shadow, Transparent), Cryptomatte, Shader AOV and Light Groups.

> "Each View Layer can output multiple Render Passes including:
>
> Combined (final image)"

Manual — https://docs.blender.org/manual/en/5.0/render/layers/introduction.html — read 2026-07-30

> "use_pass_combined
>
> Deliver full combined RGBA buffer"

Python API, `RenderLayer` — https://docs.blender.org/api/5.0/bpy.types.RenderLayer.html — read 2026-07-30

> "RenderResult(bpy_struct) … Result of rendering, including all layers and passes
>
> layers … views"

Python API, `RenderResult` — https://docs.blender.org/api/5.0/bpy.types.RenderResult.html — read 2026-07-30

**Observed, from Cycles' own file.** `cycles_5.0/engine.py`, function `list_render_passes`, lines 177–269. The Combined entry is yielded first and with no condition on it; every other entry is behind an `if`:

```
def list_render_passes(scene, srl):
    import _cycles

    crl = srl.cycles

    # Combined pass.
    yield ("Combined", "RGBA", 'COLOR')
```
(lines 177–183)

The same function then names each optional pass exactly, e.g. `yield ("Depth", "Z", 'VALUE')` (line 189), `yield ("Diffuse Color", "RGB", 'COLOR')` (line 201), `yield ("Noisy Image", "RGBA", 'COLOR')` (line 244), `yield ("CryptoObject" + '{:02d}'.format(i), "rgba", 'COLOR')` (line 234), and light groups as `yield ("Combined_%s" % lightgroup.name, "RGB", 'COLOR')` (line 263).

Note that the light-group passes are named `Combined_<name>`, so a name beginning "Combined" is not on its own proof of being the finished picture; the finished one is named exactly `Combined`.

**Documented, on pass names having changed.**

> "Many render passes were renamed to avoid obscure abbreviations. For example 'DiffCol' to 'Diffuse Color', 'IndexMA' to 'Material Index' and 'Z' to 'Depth'."

Release notes — https://developer.blender.org/docs/release_notes/5.0/rendering/ — read 2026-07-30

**Documented, on what actually reaches the output file.** The composited result, not the raw Combined pass, is what gets written when compositing is on:

> "In the Compositor, this node defines the image result that is sent to the renderer or to image output after rendering."

> "In the Compositor, only the first image input is used as the final render result. Any additional sockets will be ignored."

Manual, Group Output node — https://docs.blender.org/manual/en/5.0/compositing/types/output/group_output.html — read 2026-07-30

> "The **Composite** node was removed and replaced by a Group Output node whose first input needs to" [be connected]

Release notes — https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2. Blender 5.0.1 was started from a terminal with no window (`-b --factory-startup -noaudio`) and driven by a script. The whole list of tests is at `Audits/Blender Research/Images/Hand tests.md`.

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
ROUTE: scene.compositing_node_group with NodeGroupOutput
NODES: ['NodeGroupOutput', 'CompositorNodeRGB']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

Centre and corner are both the compositor's blue.

*What the finished picture is called once written.* From the multi-layer OpenEXR test recorded under question 5: with the view layer renamed to `Beauty`, that view layer's finished picture is a part of the file whose `name` attribute reads `Beauty.Combined` and whose channels are `Beauty.Combined.R`, `.G`, `.B` and `.A`. The renamed passes announced in the release notes are what is written: `Beauty.Depth`, `Beauty.Diffuse Color`, `Beauty.Emission` — the last two with a space in them.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 5.0.1 open. Looked at: the Image Editor header with the multi-layer OpenEXR from the test above loaded, its two picture-choosing menus opened in turn. Seen: the header shows the file name `frame_0001.exr` and then two dropdown menus, the first reading `Beauty` and the second reading `Combined`. Opening the first shows `Beauty` and `Second` above a greyed label reading `Layer`. Opening the second shows `Combined`, `Depth`, `Normal`, `Mist`, `Emission`, `Diffuse Color` above a greyed label reading `Pass`. So the renaming is what a person sees too, and the finished picture is picked by choosing `Combined` in the menu labelled `Pass`.

**Observed — the same tests on a second machine, 2026-08-06.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, started with no window (`-b --factory-startup -noaudio`) and driven by the same scripts the Mac run used, with only the folder path changed. The whole list is at `Audits/Blender Research/Images/Hand tests — Linux.md`.

*Two view layers, one plain single-image output.* Blender printed:

```
ENGINE: BLENDER_EEVEE
VIEW LAYERS: ['ViewLayer', 'Second']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.8078, 0.5451, 0.5333, 1.0) corner=(0.8549, 0.2196, 0.1255, 1.0)
```

*Compositing switched on.* Blender printed:

```
use_compositing: True
ROUTE: scene.compositing_node_group with NodeGroupOutput
NODES: ['NodeGroupOutput', 'CompositorNodeRGB']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

*The names once written.* The multi-layer OpenEXR written on this machine carries the same part name as the Mac's — the finished picture of the view layer called `Beauty` is the part named `Beauty.Combined`.

Both machines were run and both agreed, to the fourth decimal place.

**Observed — the window on the rented Linux machine, 2026-08-06.** Blender 5.0.1 was opened with a window on that machine's own display, the largest area turned into an Image Editor, the multi-layer OpenEXR above loaded into it, and the window photographed with Blender's own screenshot command, `bpy.ops.screen.screenshot`. The picture shows the header holding **two** picture-choosing menus, reading `Beauty` and `Combined` — the same two the Creator photographed on the Mac — and no third menu beside them. The Image Editor's own `View` menu, which sits in the header next to `Image`, was then opened with `bpy.ops.wm.call_menu(name="IMAGE_MT_view")` and photographed. It holds, top to bottom: `Toolbar`, `Sidebar`, `Tool Settings`, `Asset Shelf`, `Adjust Last Operation`, `Update Automatically`, `Show Metadata`, `Frame All`, `Center View to Cursor`, `Zoom`, `Area`, with `Asset Shelf` and `Center View to Cursor` greyed out. Nothing in that menu chooses which picture is shown.

**Not covered.** What the pass is called at the level of bytes in a written file is answered by the hand test recorded under question 5, and so is what happens with several View Layers and a plain single-image format. The sources do not settle whether "Noisy Image" or "Denoising Data" could ever be mistaken for the finished picture by name alone.

**Outside the interface.** **Observed.** The tests above were all run with no window open, and each did what is recorded: the passes were switched on as view-layer properties, the render was started, the file was written, and the compositor's result was the thing written. So the passes, the names they carry into a file, and the compositor deciding the saved picture all hold when Blender is driven from outside its own window. What stays window-only is the *picking*: the Image Editor's Slot / View Layer / Render Pass selectors (https://docs.blender.org/manual/en/5.0/editors/image/introduction.html) are the interface's way of choosing among them, and outside the window the choice is made by what gets written to a file. Documented alongside: rendering with no graphical display at all — "we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)" (https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html). (Previously, before the test: inferred.)

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** Every test above was run on both and came back the same, and the Image Editor's `Layer` and `Pass` menus were photographed on both. [Citation check 2026-08-06: this read "the Image Editor's menus were photographed on both". The header's `Layer` and `Pass` menus were indeed photographed on both machines, but the Image Editor's own `View` menu (`IMAGE_MT_view`) was photographed on the Linux machine alone and stays untried on the Mac. Narrowed.] **Windows is still untried** — no Windows machine has run it. From the sources: they do not say the passes differ by operating system, and one neighbouring limit is documented — "Headless rendering is not supported on headless Windows systems." (EEVEE limitations, https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html) — which is about being able to render at all on that engine, not about which passes exist.

**Where I looked.** Searched the whole 5.0 manual for `Combined` (60+ hits, all reviewed), `pass name`, `layer name`, `named .*pass`. Read in full: `render/layers/passes.txt`, `render/layers/introduction.txt`, `render/output/introduction.txt`, `render/output/index.txt`, `editors/image/introduction.txt`, `compositing/types/output/` (index, group_output, viewer, file_output, enable_output), `compositing/types/input/scene/render_layers.txt`. Read API pages `bpy.types.RenderPass`, `bpy.types.RenderLayer`, `bpy.types.RenderResult`, and the `add_pass` / `register_pass` entries of `bpy.types.RenderEngine`. Searched Cycles' files for `Combined` and read `engine.py` lines 150–275.

---

## 2. Do any of the pictures a render produces show whether a scene's textures loaded, and if so, which?

**The question has two readings.** "And if so, which?" could mean which of the pictures show it, or which textures they show as loaded. I answer the first, which is the reading that follows on from question 1. On the second reading the answer is that nothing in the sources identifies *which* texture failed — see Not covered.

**Answer.** Nothing in Blender's manual, API documentation or release notes says any render pass reports whether textures loaded. There is no "texture" or "asset" or "diagnostic" pass in the documented pass list; the only passes filed under Debug are Render Time and Sample Count.

But Cycles' own files carry a direct, exact answer to the underlying thing. Cycles has a fixed colour it substitutes when a texture is not found, and that colour is magenta.

**LABEL: observed** for the colour and the condition; **inferred** for the claim that it therefore appears in the finished picture; **not in the sources** for a dedicated pass that reports texture loading.

**Observed, from Cycles' own files.**

`cycles_5.0/source/util/texture.h`, lines 12–18:

```
/* Color to use when textures are not found. */
enum {
  TEX_IMAGE_MISSING_R = 1,
  TEX_IMAGE_MISSING_G = 0,
  TEX_IMAGE_MISSING_B = 1,
  TEX_IMAGE_MISSING_A = 1
};
```

`cycles_5.0/source/kernel/svm/image.h`, lines 20–26 — the 2D image texture lookup:

```
ccl_device float4
svm_image_texture(KernelGlobals kg, const int id, const float x, float y, const uint flags)
{
  if (id == -1) {
    return make_float4(
        TEX_IMAGE_MISSING_R, TEX_IMAGE_MISSING_G, TEX_IMAGE_MISSING_B, TEX_IMAGE_MISSING_A);
  }
```

`cycles_5.0/source/kernel/util/texture_3d.h`, lines 224–226 — the same value is the fall-through return of the 3D (volume) texture lookup:

```
  return make_float4(
      TEX_IMAGE_MISSING_R, TEX_IMAGE_MISSING_G, TEX_IMAGE_MISSING_B, TEX_IMAGE_MISSING_A);
```

Those three places are every use of `TEX_IMAGE_MISSING` in the Cycles files supplied.

Read 2026-07-30. These files have no web address; cite them by file and line as above.

**The inference, written out.** Red 1, green 0, blue 1, alpha 1 is magenta. The file's own comment says this is the "Color to use when textures are not found", and the value is returned from the function that a shader calls to read an image texture. So a surface whose image texture did not load shades magenta rather than black or its intended colour, and that magenta flows into whatever the shader feeds — which includes the Combined pass and the colour passes derived from the same shading. Two steps are supplied by me and not by the file: that "id == -1" is the state a texture is in when it failed to load, and that a shading value reaching the film reaches the saved image. Neither step is stated in the file. Hence **inferred**, not observed.

**Documented, and adjacent but not the same thing.** Blender does have one documented magenta-for-missing behaviour, but it is in the Video Sequencer and it is about strips, not about a scene's textures:

> "Show Missing Media
>
> Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent."

Manual — https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/sidebar.html — read 2026-07-30

The same option in the Python interface sits on the scene's sequence editor, not on a screen region, and is off by default:

> "show_missing_media
>
> Render missing images/movies with a solid magenta color
>
> Type: boolean, default False"

Python API, `SequenceEditor` — https://docs.blender.org/api/5.0/bpy.types.SequenceEditor.html — read 2026-07-30

**Documented, and the one manual sentence closest to the question.** In the page on applying an image to a mesh:

> "You will get a black if you do not load an image."

Manual — https://docs.blender.org/manual/en/5.0/modeling/meshes/uv/applying_image.html — read 2026-07-30

This is about not having assigned an image at all, in a passage about materials being required for rendering. It is not about a texture that was assigned and failed to load, and it does not agree with the magenta in Cycles' files. I record it because a search for this question lands on it, and because the two must not be conflated.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2. Blender 5.0.1 started from a terminal with no window and driven by a script. Set up: 32 by 32, one still, output OpenEXR, Cycles samples 1; a plainly green 8 by 8 PNG written to disk; the default cube given a material whose only shading is an Emission node fed by an Image Texture node pointing at that file, so the rendered colour is the texture's colour; Workbench's shading colour type set to `TEXTURE`; Metal switched on in the Cycles preferences, which reported `CYCLES DEVICES: [('Apple M1 Pro', 'CPU', True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`. Done: sixteen renders — four cases in four engines — each written to its own OpenEXR and the centre pixel read back. Blender printed:

```
--- CASE present (image slot='green.png', file exists=True, has_data=False) ---
RESULT present    CYCLES_CPU  centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    CYCLES_GPU  centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    EEVEE       centre RGBA = (-0.0, 1.0, -0.0, 1.0)
RESULT present    WORKBENCH   centre RGBA = (0.0032, 0.1169, 0.0026, 0.998)
--- CASE missing (image slot='green.png', file exists=False, has_data=False) ---
RESULT missing    CYCLES_CPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    CYCLES_GPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    EEVEE       centre RGBA = (1.0, -0.0, 1.0, 1.0)
RESULT missing    WORKBENCH   centre RGBA = (0.1186, 0.0029, 0.1181, 0.998)
--- CASE emptyslot (image slot=None, file exists=False, has_data=False) ---
RESULT emptyslot  CYCLES_CPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  CYCLES_GPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  EEVEE       centre RGBA = (0.0, 0.0, 0.0, 1.0)
RESULT emptyslot  WORKBENCH   centre RGBA = (0.0955, 0.0941, 0.0951, 0.998)
--- CASE corrupt (image slot='green.png', file exists=True, has_data=False) ---
RESULT corrupt    CYCLES_CPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    CYCLES_GPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    EEVEE       centre RGBA = (1.0, -0.0, 1.0, 1.0)
RESULT corrupt    WORKBENCH   centre RGBA = (0.1186, 0.0029, 0.1181, 0.998)
```

The "corrupt" case was the same file replaced with the bytes `this is not a picture`.

This version does print something naming the file. During the missing-file renders:

```
00:01.687  cycles           | ERROR Image file '/tmp/images-hand-tests/out/t11/5.0.1/green.png' does not exist.
00:01.761  cycles           | ERROR Image file '/tmp/images-hand-tests/out/t11/5.0.1/green.png' does not exist.
00:01.967  gpu.texture      | ERROR Failed to create GPU texture from Blender image
```

and during the corrupt-file renders:

```
00:03.121  cycles           | ERROR Image file '/tmp/images-hand-tests/out/t11/5.0.1/green.png' failed to load.
00:03.198  cycles           | ERROR Image file '/tmp/images-hand-tests/out/t11/5.0.1/green.png' failed to load.
00:03.408  image.read       | ERROR IMB_load_image_from_memory: unknown file-format (/tmp/images-hand-tests/out/t11/5.0.1/green.png)
00:03.408  gpu.texture      | ERROR Failed to create GPU texture from Blender image
```

**Observed — the same test on a second machine and a different make of graphics card, 2026-08-06.** Run on the rented Linux machine, Blender 5.0.1, no window. The set-up is the Mac's, with one change: Apple's Metal graphics do not exist here, so the script asks Cycles what it has and then renders on this machine's card twice, once through CUDA and once through OptiX. It reported:

```
COMPUTE DEVICE TYPES OFFERED: []
CYCLES DEVICES with CUDA: [('NVIDIA GeForce RTX 4090', 'CUDA', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', True), ('NVIDIA GeForce RTX 4090', 'OPTIX', True)]
```

Twenty renders were made — four cases in five engines. Blender printed:

```
--- CASE present (image slot='green.png', file exists=True, has_data=False) ---
RESULT present    CYCLES_CPU   centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    CYCLES_CUDA  centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    CYCLES_OPTIX centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    EEVEE        centre RGBA = (-0.0, 1.0, -0.0, 1.0)
RESULT present    WORKBENCH    centre RGBA = (0.0032, 0.1171, 0.0026, 1.0)
--- CASE missing (image slot='green.png', file exists=False, has_data=False) ---
RESULT missing    CYCLES_CPU   centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    CYCLES_CUDA  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    CYCLES_OPTIX centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    EEVEE        centre RGBA = (1.0, -0.0, 1.0, 1.0)
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
RESULT corrupt    EEVEE        centre RGBA = (1.0, -0.0, 1.0, 1.0)
RESULT corrupt    WORKBENCH    centre RGBA = (0.1188, 0.0029, 0.1183, 1.0)
```

Both machines were run. **They agree on the colour in every case and every engine**, and the NVIDIA card gives the same answer both ways it can be used. **One number differs**: Workbench's alpha is `1.0` on this machine and `0.998` on the Mac, in all four cases. Nothing was read into that. [Citation check 2026-08-06: this read `0.9976`, which is 4.5's figure; the Mac printout above in this document reads `0.998`. Corrected to what the printout says. The checker re-ran the missing-texture script on the rented Linux machine and got `1.0` in all four cases, matching the Linux printout above.]

What Blender printed about the texture differs from the Mac in one way, and it is the machine rather than Blender. The `cycles | ERROR Image file '<path>' does not exist.` and `cycles | ERROR Image file '<path>' failed to load.` lines came back exactly as on the Mac. This machine adds two lines the Mac never printed:

```
00:00.386  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
00:03.077  gpu.texture      | ERROR Failed to create GPU texture from Blender image
```

The first is Cycles looking for AMD graphics that are not in this machine. The second replaces the Mac's `GPUTexture: Blender Texture Not Loaded!`.

**Not covered.** Which texture failed is not recorded anywhere in the sources — the magenta is a single flat colour with no identity in it, so a picture showing magenta cannot say which file was missing. The sources do not say whether EEVEE or Workbench substitute the same colour (EEVEE's own files were not supplied). They do not say whether an image that is present but unreadable takes the same path as one that is absent. They say nothing about a machine-readable report of texture loading alongside the picture. And there is no documented pass whose purpose is to answer this.

**Outside the interface.** **Observed.** The whole test above was run with no window open, and the substituted colour appeared in the written files just the same. So the substitution holds when Blender is driven from outside its own window. And something *is* printed to the console during a background render: the lines quoted above name the missing or unreadable file by its full path. (Previously, before the test: not claimed either way, and the console question was open.)

**Operating system.** **Observed on two machines and on two makes of graphics card, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro) with Apple's own Metal graphics, and Ubuntu 22.04.5 LTS with an NVIDIA GeForce RTX 4090 used both through CUDA and through OptiX.** The substituted colour is identical everywhere; what Blender prints alongside it gains two machine-specific lines on Linux, quoted above. **Windows is still untried** — no Windows machine has run it — and so is any card that is neither Apple's nor NVIDIA's. From the sources: nothing in the three Cycles files above is conditional on operating system; the missing-texture branch in `texture_3d.h` sits after a `#else` covering a non-GPU build path, which is a hardware distinction rather than an operating-system one.

**Where I looked.** Searched the whole 5.0 manual for: `magenta`, `pink`, `missing texture`, `texture (not found|missing)`, `could not be found`, `missing files`, `find missing`, `report missing`, `missing image`, `missing media`, `placeholder`, `broken texture`, `invalid image`, `image is missing`, `file not found`, `no such file`, `texture.*load`, `load.*texture`, `unresolved`. Searched the API documentation for the same and for `has_data`. Searched the release notes for `missing media`. Read `render/layers/passes.txt` in full for a texture or diagnostic pass and found none — the Debug heading holds only Render Time, Sample Count and Alpha Threshold. Read `render/shader_nodes/textures/image.txt` in full; it describes the Image Texture node's inputs, projections and colour space and says nothing about failure. Ruled out `interface/window_system/topbar.txt` (Report Missing Files / Find Missing Files) because it is about locating files on disk before rendering, not about what a rendered picture shows. Searched all of Cycles' files for `missing`, `not found`, `failed to load`, `could not load`, and for every use of `TEX_IMAGE_MISSING`.

---

## 3. Which file formats can Blender write a render out as?

**Answer.** Fourteen still-image formats, plus video through FFmpeg.

**Still images** (the manual's table of formats "supported internally", with channel depth, alpha, metadata, pixel density and extensions):

| Format | Channel depth | Alpha | Metadata | DPI | Extensions |
|---|---|---|---|---|---|
| JPEG | 8bit | ✗ | ✓ | ✓ | .jpg .jpeg |
| OpenEXR | float 16, 32bit | ✓ | ✓ | ✓ | .exr |
| PNG | 8, 16bit | ✓ | ✓ | ✓ | .png |
| WebP | 8bit | ✓ | ✓ | ✗ | .webp |
| BMP | 8bit | ✓ | ✗ | ✓ | .bmp |
| Cineon | 8, 10, 12, 16bit | ✓ | ✗ | ✗ | .cin |
| DPX | 8, 10, 12, 16bit | ✓ | ✗ | ✗ | .dpx |
| Iris | 8, 16bit | ✓ | ✗ | ✗ | .sgi .rgb .bw |
| JPEG 2000 | 8, 12, 16bit | ✓ | ✗ | ✗ | .jp2 .jp2 .j2c |
| Radiance HDR | float | ✓ | ✗ | ✗ | .hdr |
| Targa | 8bit | ✓ | ✗ | ✗ | .tga |
| Targa Raw | 8bit | ✓ | ✗ | ✗ | .tga |
| TIFF | 8, 16bit | ✓ | ✗ | ✓ | .tif .tiff |

(That table is reproduced from the manual page cited below; the "supported internally" wording is the manual's.)

The Python interface adds the fourteenth by naming the multi-layer variant separately, and confirms each entry is an *output* format.

**Video** is a container plus a codec, both through FFmpeg. Containers: MPEG-4, Matroska, WebM, AVI, DV, Flash, MPEG-1, MPEG-2, Ogg, QuickTime. Video codecs: No Video, AV1, H.264, H.265/HEVC, WebM/VP9, DNxHD, DV, FFV1, Flash Video, HuffYUV, MPEG-1, MPEG-2, MPEG-4 (DivX), PNG, ProRes, QuickTime Animation, Theora. Audio codecs: No Audio, AAC, AC3, FLAC, MP2, MP3, Opus, PCM, Vorbis.

**LABEL: documented.**

**Quoted words.**

> "This is the list of image file formats supported internally by Blender:"

(followed by the table above)

> "All these formats support compression which can be important when rendering out animations."

Manual — https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30

The Python interface's list of what the file-format setting can be set to, each entry described as an output:

> "JPEG: JPEG (.jpg). Output image in JPEG format.
> OPEN_EXR: OpenEXR (.exr). Output image in OpenEXR format.
> PNG: PNG (.png). Output image in PNG format.
> WEBP: WebP (.webp). Output image in WebP format.
> BMP: Bitmap (.bmp). Output image in bitmap format.
> CINEON: Cineon (.cin). Output image in Cineon format.
> DPX: DPX (.dpx). Output image in DPX format.
> IRIS: Iris (.rgb). Output image in SGI IRIS format.
> JPEG2000: JPEG 2000 (.jp2). Output image in JPEG 2000 format.
> HDR: Radiance HDR (.hdr). Output image in Radiance HDR format.
> TARGA: Targa (.tga). Output image in Targa format.
> TARGA_RAW: Targa Raw (.tga). Output image in uncompressed Targa format.
> TIFF: TIFF (.tif). Output image in TIFF format.
> OPEN_EXR_MULTILAYER: OpenEXR MultiLayer (.exr). Output image in multilayer OpenEXR format.
> FFMPEG: FFmpeg Video."

Python API — https://docs.blender.org/api/5.0/bpy_types_enum_items/image_type_all_items.html — read 2026-07-30

> "file_format
>
> File format to save the rendered images as
>
> Type: enum in Image Type All Items, default 'PNG'"

> "media_type
>
> The type of media to save
>
> Type: enum in ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO'], default 'IMAGE'"

Python API, `ImageFormatSettings` — https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html — read 2026-07-30

The video containers and codecs, exactly:

> "format
>
> Output file container
>
> Type: enum in ['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME'], default 'MKV'"

> "codec … Type: enum in ['NONE', 'AV1', 'H264', 'H265', 'WEBM', 'DNXHD', 'DV', 'FFV1', 'FLASH', 'HUFFYUV', 'MPEG1', 'MPEG2', 'MPEG4', 'PNG', 'PRORES', 'QTRLE', 'THEORA'], default 'H264'"

> "audio_codec … Type: enum in ['NONE', 'AAC', 'AC3', 'FLAC', 'MP2', 'MP3', 'OPUS', 'PCM', 'VORBIS'], default 'NONE'"

Python API, `FFmpegSettings` — https://docs.blender.org/api/5.0/bpy.types.FFmpegSettings.html — read 2026-07-30

Their descriptions, one by one, are on the manual's video page — https://docs.blender.org/manual/en/5.0/files/media/video_formats.html — read 2026-07-30.

**Documented, and important: not all of these are in every copy of Blender.**

> "-F, --render-format <format>
>
> Set the render format. Valid options are: TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF.
>
> Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

Manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30

Which of the optional ones a given copy has can be read from the Python interface. The documentation shows the setting names and the values of the build that produced the documentation:

> "Constant value bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, …)"

Python API — https://docs.blender.org/api/5.0/bpy.app.html — read 2026-07-30

**Documented, from a picture.** The manual's Output panel screenshot (`_images/render_output_properties_output_panel.png`, shown on https://docs.blender.org/manual/en/5.0/render/output/properties/output.html) shows the panel with a path field reading `/tmp/`, checkboxes Overwrite, Placeholders, File Extensions, Cache Result, a **File Format** dropdown set to **PNG**, a Color row offering **BW / RGB / RGBA** with RGBA selected, a Color Depth row offering **8 / 16**, and a Compression slider at 15%. Read 2026-07-30. Note that the picture does not show a Media Type row, though the text on the same page describes one.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, on the Blender 5.0.1 installed there, started from a terminal with no window. Set up: nothing built; a plain start with factory settings. Done: printed the build options, the output-format setting and its choices, the Media Type setting and its choices, and the FFmpeg container and codec choices. Blender printed:

```
VERSION: 5.0.1
BUILD_OPTIONS: bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=True, jack=False, pulseaudio=False, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=False, potrace=True, pugixml=True, haru=True, experimental_features=False)
FILE_FORMAT_DEFAULT: PNG
FILE_FORMAT_ITEMS: ['JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
HAS_MEDIA_TYPE: True
MEDIA_TYPE_DEFAULT: IMAGE
MEDIA_TYPE_ITEMS: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']
HAS_USE_PREVIEW: True
FFMPEG_CONTAINERS: ['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']
FFMPEG_CODECS: ['NONE', 'AV1', 'H264', 'H265', 'WEBM', 'DNXHD', 'DV', 'FFV1', 'FLASH', 'HUFFYUV', 'MPEG1', 'MPEG2', 'MPEG4', 'PNG', 'PRORES', 'QTRLE', 'THEORA']
FFMPEG_AUDIO_CODECS: ['NONE', 'AAC', 'AC3', 'FLAC', 'MP2', 'MP3', 'OPUS', 'PCM', 'VORBIS']
```

This build has every optional image format compiled in. There is no `AVIF` among the formats.

*Media Type comes first.* Setting `file_format` while `media_type` is `IMAGE` was refused with a message listing only the still-image formats:

```
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "AVIF" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
```

So the fifteen names listed above are the whole set, but which of them the setting will take at any moment depends on the Media Type: with Media Type on `IMAGE`, `OPEN_EXR_MULTILAYER` and `FFMPEG` are not among the choices. Setting `media_type = 'MULTI_LAYER_IMAGE'` and then `file_format = 'OPEN_EXR_MULTILAYER'`, or `media_type = 'VIDEO'` and then `'FFMPEG'`, was accepted (recorded under questions 5 and 7).

*Asking for a format by name on the command line.* Set up: nothing built; the factory startup scene shrunk to 64 by 64. Done, in a terminal, once per name:

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=64" -F AVIF -o /tmp/images-hand-tests/out/t02/5.0.1_AVIF/out_ -f 1
```

`WEBP` was accepted and wrote `out_0001.webp`, which `file` reported as `RIFF (little-endian) data, Web/P image, VP8 encoding, 64x64`. `OPEN_EXR_MULTILAYER` was accepted and wrote `out_0001.exr`, reported as `OpenEXR image data, version 2`. `AVIF` was refused, with `Error: Format from '-F / --render-format' not known or not compiled in this release.`, and a PNG written instead. Exit status was 0 in every case.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 5.0.1 open. Looked at: Properties ▸ Output Properties ▸ Output panel, with the format changed in turn, and the menus opened.

There are **two** controls here, one above the other, and both exist in this version: a row labelled **`Media Type`** and, below it, a row labelled **`File Format`**. This is the change from 4.5, whose panel has only `File Format` and no `Media Type` row at all.

Opening `Media Type` shows three entries: `Image`, `Multi-Layer EXR`, `Video`.

With `Media Type` on `Image`, opening `File Format` shows, in this order: `JPEG (.jpg)`, `OpenEXR (.exr)`, `PNG (.png)`, `WebP (.webp)`, then a dividing line, then `Bitmap (.bmp)`, `Cineon (.cin)`, `DPX (.dpx)`, `Iris (.rgb)`, `JPEG 2000 (.jp2)`, `Radiance HDR (.hdr)`, `Targa (.tga)`, `Targa Raw (.tga)`, `TIFF (.tif)`. Thirteen entries, each carrying its extension in the label, and no `AVIF` among them. That is the same thirteen the terminal reported when the setting refused a name, so the menu and the setting agree.

With `Media Type` set to `Multi-Layer EXR` the `File Format` row disappears from the panel altogether, leaving `Color Depth`, `Codec: ZIP`, `Interleave` and `Preview`.

With each format chosen, the panel shows: **PNG** — `Color` (BW / RGB / RGBA), `Color Depth` (8 / 16), `Compression 15%`. **JPEG** — `Color` (BW / RGB), `Quality 90%`. **TIFF** — `Color` (BW / RGB / RGBA), `Color Depth` (8 / 16), `Compression: Deflate`. **OpenEXR** — `Color`, `Color Depth` (Float (Half) / Float (Full)), `Codec: ZIP`, `Preview`.

With `Media Type` set to `Video`, the `Encoding` section's `Container` menu and the `Video` section's `Video Codec` menu were opened. Every entry, in the order shown:

- **Container**: `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`.
- **Video Codec**: `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`.

Both lists are the same as 4.5's. The rows under `Video` are `Video Codec`, `Color Depth`, `Color Space`, `Output Quality`, `Encoding Speed`, `Keyframe Interval`, `Max B-frames` — a `Color Space` row that 4.5's panel does not have.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.0.1, no window, factory settings. Blender printed:

```
VERSION: 5.0.1
BUILD_OPTIONS: bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=False, jack=True, pulseaudio=True, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)
FILE_FORMAT_DEFAULT: PNG
FILE_FORMAT_ITEMS: ['JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
HAS_MEDIA_TYPE: True
MEDIA_TYPE_DEFAULT: IMAGE
MEDIA_TYPE_ITEMS: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']
HAS_USE_PREVIEW: True
FFMPEG_CONTAINERS: ['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']
FFMPEG_CODECS: ['NONE', 'AV1', 'H264', 'H265', 'WEBM', 'DNXHD', 'DV', 'FFV1', 'FLASH', 'HUFFYUV', 'MPEG1', 'MPEG2', 'MPEG4', 'PNG', 'PRORES', 'QTRLE', 'THEORA']
FFMPEG_AUDIO_CODECS: ['NONE', 'AAC', 'AC3', 'FLAC', 'MP2', 'MP3', 'OPUS', 'PCM', 'VORBIS']
```

Both machines were run and both agreed on everything that bears on this question: every optional image format is compiled in, the Media Type setting is there with the same three choices, there is no AVIF, and the format list and the three FFmpeg lists are identical. Three build options differ between the machines, and none is an image format: `coreaudio` is true on the Mac and false here, `jack` and `pulseaudio` are false on the Mac and true here, and `xr_openxr` is false on the Mac and true here.

*Asking for a format by name on the command line.* The same four names, the same command, the same results: `WEBP` wrote `out_0001.webp` (`RIFF (little-endian) data, Web/P image, VP8 encoding, 64x64`), `OPEN_EXR_MULTILAYER` wrote `out_0001.exr` (`OpenEXR image data, version 2`), and `AVIF` was refused with `Error: Format from '-F / --render-format' not known or not compiled in this release.` followed by a PNG. Exit status `0` every time.

*Asking for a format by name from Python.* The same message, with the same thirteen names the Media Type filters the list down to:

```
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "AVIF" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
```

*What the container and codec lists mean.* A further test, recorded in full under question 4, chose each container in turn and read the codec list back: all ten containers offer all seventeen codecs after being chosen, and all 170 pairings can be set. So the lists above are what Blender offers, not what it will accept — several pairings fail at the moment of encoding.

**Not covered.** This is what can be *chosen*. What this particular installation has is settled by the build-options line above, for this machine only. Still not settled: which container-and-codec pairings are legal, and what Blender does when a chosen pairing is not (question 4). `image_dds=True` appears among the build options but DDS is not in the list of things a render can be written as — **inferred** to be a read-only format, since it is absent from the output list; the sources do not say so. Blender's own `.blend` previews are not covered here (see question 8). Nothing here settles which formats can hold several pictures (question 5).

**Outside the interface.** **Observed.** Everything above was done with no window open: `-F` chose the format from the command line and wrote the file; the settings were read and written from Python in a windowless Blender. The one thing to know before scripting against it is the order — Media Type first, then the format. Documented alongside: the command line can choose every format in the documented output list with `-F`; the command-line page gives the built-in names and then the build-dependent names. The manual's example says: "blender -b file.blend -o /project/renders/frame_##### -F OPEN_EXR -f -2 … -F OPEN_EXR — Override the image format specified in the blend-file and save to an OpenEXR image." (https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html). The manual warns that argument order matters and that "-F and -f are not the same". Availability still depends on how that copy of Blender was built, as documented above.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The official build for each system has every optional image format compiled in, and the format lists match exactly. **Windows is still untried** — no Windows machine has run it — so what a build there carries is not settled, and that is the case the manual is warning about. From the sources: the line "Formats that can be compiled into Blender, not available on all systems" is explicit that seven of them — including OpenEXR and all video — are not guaranteed. For video the manual adds: "There are dozens, if not hundreds, of codecs … Each has advantages and disadvantages, and compatibility with different players on different operating systems." and "Codecs are used to encode and decode the movie, and so must be present on both the encoding machine (Blender) and the target machine." (https://docs.blender.org/manual/en/5.0/files/media/video_formats.html). The sources do not name which systems lack which format.

**Where I looked.** Read in full `files/media/image_formats.txt`, `files/media/video_formats.txt`, `render/output/properties/output.txt`, `render/output/properties/format.txt`, `advanced/command_line/arguments.txt` (format section), `advanced/command_line/render.txt`. Read API pages `bpy.types.ImageFormatSettings`, `bpy.types.FFmpegSettings`, `bpy.app`, and the enum pages `image_type_all_items`, `image_color_depth_items`, `views_format_multiview_items`, `views_format_multilayer_items`, `exr_codec_items`. Listed every image-related enum page under `bpy_types_enum_items/` to be sure no second, narrower list of output formats existed. Opened the Output panel picture. Searched the release notes for format additions and removals.

---

## 4. What happens when a file asks for a format that Blender cannot write?

**The question is ambiguous, and I flag it.** "A file asks for a format" I read as: a blend-file's saved output settings name a format this copy of Blender cannot produce — the case the command-line page warns about. Two other readings exist: (a) the output *path* carries an extension Blender cannot write, and (b) a video container and codec are chosen that do not go together. I address all three, because the sources have something for each, but only the third is documented at all.

**Answer.** **Not in the sources.** No page in the 5.0 manual, the 5.0 API documentation or the 5.0 release notes says what Blender does when the requested format is unavailable — whether it refuses, substitutes another format, writes nothing, stops the render, or reports something. I searched hard for a fallback and found none.

What *is* documented is the surrounding ground, and it is worth having:

**The format is a setting, not the file name.** The extension follows the chosen format, not the other way round:

> "Saving – File Extensions
>
> Adds the correct file extensions per file type to the output files."

Manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30

So reading (a) does not really arise: nothing in the sources suggests a path's extension selects a format.

**Some formats are genuinely absent from some copies of Blender.**

> "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

Manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30

And whether a copy has them can be read before rendering, from `bpy.app.build_options` (`image_openexr`, `image_openjpeg`, `image_tiff`, `image_webp`, `image_cineon`, `image_hdr`, `codec_ffmpeg`, `codec_avi`) — https://docs.blender.org/api/5.0/bpy.app.html — read 2026-07-30.

**For video, an incompatible pairing is documented as producing errors, without saying what the error is or does.**

> "Standards
>
> Some containers and codecs are not compatible with each other, so if you are getting errors check that your container and codec are compatible. Like containers and codecs are sometimes not compatible with each other, some codecs do not work with arbitrary dimensions. So, try to stick with common dimensions or research the limitations of the codec you are trying to use."

Manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30

> "Video Output Size
>
> Some codecs impose limitations on output size, H.264, for example requires both the height and width to be divisible by 2."

Manual — https://docs.blender.org/manual/en/5.0/files/media/video_formats.html — read 2026-07-30

**A narrower, related refusal is documented for options within a format**, without saying what happens if you ask anyway:

> "Note, RGBA is not available for all image formats, check the list above for details."

> "Note, not all file formats support all color depths."

Manual — https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30

**LABEL: not in the sources**, with the documented surroundings above labelled **documented**.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.0.1, started from a terminal with no window. Three cases were tried.

*A name the command line will not take.* Set up: nothing built; the factory startup scene at 64 by 64, its own output format left at the default PNG. Done, in a terminal:

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=64" -F NOSUCHFORMAT -o /tmp/images-hand-tests/out/t02/5.0.1_NOSUCHFORMAT/out_ -f 1
```

Blender printed:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
00:00.374  render           | Rendering single frame
...
00:00.758  render           | Saved: '/tmp/images-hand-tests/out/t02/5.0.1_NOSUCHFORMAT/out_0001.png'
00:00.758  render           | Time: 00:00.47 (Saving: 00:00.05)
```

The exit status was `0`. One file was written, `out_0001.png`, which `file` reported as `PNG image data, 64 x 64, 8-bit/color RGBA, non-interlaced` — the format already set in the file. The same happened when the name was `AVIF`, a real format this version does not have. Note that the error line is printed plain, not through the new logging format.

*The same request from Python.* Set up: nothing built. Done: `bpy.context.scene.render.image_settings.file_format = 'NOSUCHFORMAT'`, then a still rendered with `bpy.ops.render.render(animation=False, write_still=True)`. Blender printed:

```
ASSIGNMENT RAISED: TypeError
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "NOSUCHFORMAT" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
file_format after attempt: PNG
OPERATOR RESULT: {'FINISHED'}
FILES: ['out_.png']
```

*A request the codec refuses.* Set up: the factory startup scene, `image_settings.media_type = 'VIDEO'`, `image_settings.file_format = 'FFMPEG'`, `ffmpeg.format = 'MPEG4'`, `ffmpeg.codec = 'H264'`, resolution 101 by 101, frames 1 to 3. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
00:00.265  video.write      | ERROR Couldn't initialize video codec: Generic error in an external library
Traceback (most recent call last):
  File "/private/tmp/images-hand-tests/scripts/t04_video_size.py", line 24, in <module>
    res = bpy.ops.render.render(animation=True)
...
RuntimeError: Error: width not divisible by 2 (101x101)
```

and, on its own report line, `Error: width not divisible by 2 (101x101)`. The output folder was empty afterwards. The process exit status was `0` — the Python error did not change it, since `--python-exit-code` was not given.

*The case that could not be produced.* The reading the question was written for — a blend-file asking for a real format that this copy of Blender lacks — could not be made to happen on this machine. The build-options line recorded under question 3 shows every optional format compiled in. That case is recorded as not runnable in `Audits/Blender Research/Images/Hand tests.md`, with the reason: it needs a Blender build made without one of those formats, and both machines the project has carry official builds with every one of them. The Linux run of 2026-08-06 checked the second machine and found the same, so the case is not runnable on either.

**Observed — the same three cases on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.0.1, no window. All three were repeated with the same scripts and the same commands.

*A name the command line will not take.* Blender printed `Error: Format from '-F / --render-format' not known or not compiled in this release.` on a plain line, then `00:01.123  render           | Saved: '/root/smoozy/Images/work/out/t02/5.0.1_NOSUCHFORMAT/out_0001.png'`. Exit status `0`, and `file` reported the written file as `PNG image data, 64 x 64, 8-bit/color RGBA, non-interlaced`. The same happened for `AVIF`.

*The same request from Python.* The same `TypeError` came back, with the same list of names.

*A request the codec refuses.* Blender printed `00:00.440  video.write      | ERROR Couldn't initialize video codec: Generic error in an external library`, then `RuntimeError: Error: width not divisible by 2 (101x101)`, the output folder was empty, and the process exit status was `0`.

Both machines were run and both agreed, message for message and status for status.

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

**Not covered.** The one reading the question was written for — a real format this build lacks — is still unanswered, and now for a stronger reason: both machines the project has were checked, and the official build for each has every optional format compiled in, so the case cannot be produced on either. Also still unanswered: whether the behaviour above differs on Windows, and whether the format list shown in the interface is filtered to what the build supports.

**Outside the interface.** **Observed.** All three cases above were run with no window open, and that is where the answers came from. Two things matter for anything driving Blender from outside: a format name the command line will not take is reported on one plain line and then ignored, the render carrying on in whatever format the file already held, with an exit status of `0`; and a request the codec refuses stops that render, writes no file, and still leaves the exit status at `0`. (Previously, before the test: not stated.) On the console format, the release notes give the shape of the new output:

> "Most of Blender now uses the same logging system, including operators reports and background render progress. The output format has changed, and log levels are easier to control."

with an example ending

> "00:28.631  render           | Saved: '/tmp/0001.png'"

Release notes — https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30

That documents that a successful save is logged on the line `render | Saved: '<path>'`. It says nothing about a failure line. `--log-level info` is documented as "useful to enable in production for troubleshooting issues".

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The same lines, the same refusals and the same exit statuses came back on both, including every one of the container-and-codec refusals. **Windows is still untried** — no Windows machine has run it — and that gap sits directly on this question, since the "not available on all systems" line is the reason the situation exists at all. The sources do not name which system lacks which; what the two machines show is that neither of them lacks one.

**Where I looked.** Searched the whole 5.0 manual for: `unsupported`, `not supported`, `cannot be saved`, `fails? to (save|write)`, `falls? back`, `fall back`, `fallback`, `defaults to PNG`, `instead of the requested`, `if the format`, `format is unavailable`, `not compiled`, `disabled at build`, `build option`, `WITH_`, `error message`, `reports an error`, `could not (write|save|open)`, `cannot (write|save)`, `unable to (write|save)`, `write error`. Searched the API documentation for the same set. Every hit was either about EEVEE's unsupported shader nodes, GPU driver messages, add-on warnings or importers — none about output formats. Read `bpy.ops.render.render`'s entry, which is marked "Undocumented, consider contributing." Read the whole of the manual's troubleshooting index (`3d_view`, `crash`, `gpu`, `network`, `python`, `recover`, `report_bug`, `startup`) for a section on output failures; there is none. Searched the 5.0 release notes, including the bug-fix list, for output-format failures; the nearest entries are "OpenEXR MultiLayer does not save renderd frames when switching from other File Format" and "Ffmpeg settings get lost when switching the output file format", both listed as fixed and neither describing behaviour on an unwritable format.

---

## 5. Can a format hold several pictures inside one file, and if so, how can the finished picture be told apart from the others inside it?

**Answer.** Yes — **OpenEXR**. It holds several pictures in two different senses, and both are documented:

- **Layers and passes**: one file holding every View Layer and every pass of a render. Blender exposes this as the separate output format "OpenEXR MultiLayer", and as the Media Type "Multi-Layer EXR".
- **Views**: one file holding the left and right (or more) views of a multi-view render. "Multi-view native support is only available to OpenEXR."

The finished render pass is named **Combined** (see question 1). The sources do not give the exact text by which that pass can be identified inside a written multi-layer file. They also do not establish that OpenEXR is the only writable format that can hold several pictures.

**LABEL: documented** for OpenEXR holding layers, passes and views, and for Combined being the final pass; **not in the sources** for the exact written layer name and for whether any other writable format can hold several pictures.

**Quoted words.**

> "ILM's OpenEXR has become a software industry standard for HDR image files, especially because of its flexible and expandable structure.
>
> An OpenEXR file can store multiple layers and passes. This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact."

Manual — https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30

> "Media Type
>
> The type of media to save.
>
> Image: Saves each input as a separate image file. Each input can use its own image format settings.
>
> Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file."

> "Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image."

Manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30

> "Lists all input sockets of the node. The name of each input socket is used to construct the final file name.
>
> When saving individual images, each output can be renamed and will be written to disk using either the global Node Format settings or its own overridden settings. When using Multi-Layer EXR, each input is stored as a separate layer within the file."

Manual, File Output node — https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — read 2026-07-30

> "Multi-Layer Format
>
> When a multi-layer file format, like EXR, is loaded, each layer is made available as a socket."

Manual, Image node — https://docs.blender.org/manual/en/5.0/compositing/types/input/image.html — read 2026-07-30

The naming convention, as far as the manual states it:

> "Cryptomatte Layer
>
> The image layer to use. This is typically a combination of a View Layer and a Cryptomatte type (Object/Material/Asset)."

Manual, Cryptomatte node — https://docs.blender.org/manual/en/5.0/compositing/types/mask/cryptomatte.html — read 2026-07-30

Views in one file:

> "Multi-View OpenEXR
>
> Another option is to use multi-view OpenEXR files. This format can save multiple views in a single file and is backward compatible with old OpenEXR viewers (you see only one view though). Multi-view native support is only available to OpenEXR."

Manual — https://docs.blender.org/manual/en/5.0/render/output/properties/stereoscopy/usage.html — read 2026-07-30

> "MULTIVIEW: Multi-View. Single file with all the views."

Python API — https://docs.blender.org/api/5.0/bpy_types_enum_items/views_format_multilayer_items.html — read 2026-07-30

**Documented, on how 5.0 writes these files.**

> "Multilayer OpenEXR now saves multi-part files by default, for more efficient loading of individual views, layers and passes in other applications. The 'Interleave' option can be turned on to get the old behavior, for compatibility with older applications that do not support multi-part files well."

Release notes — https://developer.blender.org/docs/release_notes/5.0/pipeline_io/ — read 2026-07-30

The matching setting:

> "Interleave
>
> Use legacy interleaved storage of views, layers and passes for compatibility with applications that do not support more efficient multi-part OpenEXR files."

Manual — https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30

> "use_exr_interleave … Type: boolean, default False"

Python API, `ImageFormatSettings` — https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html — read 2026-07-30

**Documented, and a real limit on reading these files back.**

> "The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor."

Manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.0.1, started from a terminal with no window.

*A rendered multi-layer OpenEXR.* Set up: 32 by 32, one frame; the first view layer renamed to `Beauty` and a second added called `Second`; on both, `use_pass_combined`, `use_pass_z`, `use_pass_normal`, `use_pass_mist`, `use_pass_diffuse_color` and `use_pass_emit` switched on; `image_settings.media_type = 'MULTI_LAYER_IMAGE'` and then `file_format = 'OPEN_EXR_MULTILAYER'`; `use_exr_interleave` left at its default, which printed as `False`. The render engine was the factory default, which this version reports as `BLENDER_EEVEE`, so the names below are the ones that engine writes. Done: `bpy.ops.render.render(animation=True)`, then the written file's own bytes read with a short header reader (see `Audits/Blender Research/Images/Hand tests.md`). The reader printed:

```
BYTES: 13901
MAGIC: 0x01312f76
VERSION FIELD: 0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)
--- PART 0 ---
  BlenderMultiChannel      string           'Blender V2.55.1 and newer'
  Camera                   string           'Camera'
  Date                     string           '2026/08/03 22:42:40'
  File                     string           '<untitled>'
  Frame                    string           '1'
  RenderTime               string           '00:00.98'
  Scene                    string           'Scene'
  Software                 string           'Blender 5.0.1'
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

There were twelve parts in all — one for each pass of each view layer. Their `name` and `channels` and `colorInteropID` lines, in the order written:

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

So: the file is a multi-part file, each part named `<view layer>.<pass>`, each channel named `<view layer>.<pass>.<channel>`. The finished picture of the first view layer is the part named `Beauty.Combined`. The renamed passes are what is written — `Depth`, `Diffuse Color`, `Emission`, the last two with a space in them.

*A multi-layer OpenEXR written by a File Output node.* Set up: the same scene at 32 by 32, main output PNG; a File Output node with `directory` set to the folder, `file_name = "fromnode_"`, `format.media_type = 'MULTI_LAYER_IMAGE'`, `format.file_format = 'OPEN_EXR_MULTILAYER'`, its items cleared, and two items added with `file_output_items.new('RGBA', "MyBeautyLayer")` and `file_output_items.new('RGBA', "second socket")`, both fed from the Render Layers node's Image output. Done: the same render call. Blender printed `Saved: '/tmp/images-hand-tests/out/t08/5.0.1/fromnode_0001.exr'`, and the header reader printed:

```
VERSION FIELD: 0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)
  channels                 chlist           MyBeautyLayer.A(FLOAT); MyBeautyLayer.B(FLOAT); MyBeautyLayer.G(FLOAT); MyBeautyLayer.R(FLOAT)
  name                     string           'MyBeautyLayer'
  channels                 chlist           second socket.A(FLOAT); second socket.B(FLOAT); second socket.G(FLOAT); second socket.R(FLOAT)
  name                     string           'second socket'
```

So a layer written by that node takes its socket's name, spaces and all, and there is no view-layer part in front of it.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 5.0.1 open and the multi-layer OpenEXR from the test above loaded in the Image Editor. Seen: two dropdown menus in the header. The first, opened, shows `Beauty` and `Second` above a greyed label reading `Layer`. The second, opened, shows `Combined`, `Depth`, `Normal`, `Mist`, `Emission`, `Diffuse Color` above a greyed label reading `Pass`. Those are the names a person sees, and they are the two halves of the part names inside the file — `Beauty.Combined`, `Beauty.Depth`, `Beauty.Mist`, `Beauty.Normal`, `Beauty.Emission`, `Beauty.Diffuse Color` — with the view-layer half shown as its own menu rather than spelled out.

**Not captured: the View menu.** Only two menus were photographed, the one labelled `Layer` and the one labelled `Pass`. No screenshot shows a menu for views, so nothing is recorded here about one, and nothing should be read into its absence from these pictures.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with Blender 5.0.1, no window, driven by the same scripts.

*A rendered multi-layer OpenEXR.* The reader printed the same version field, `0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)`, and the same twelve parts. The first three:

```
--- PART 0 ---
  Software                 string           'Blender 5.0.1'
  channels                 chlist           Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT)
  colorInteropID           string           'lin_rec709_scene'
  name                     string           'Beauty.Combined'
--- PART 1 ---
  channels                 chlist           Beauty.Depth.Z(FLOAT)
  colorInteropID           string           'data'
  name                     string           'Beauty.Depth'
--- PART 4 ---
  channels                 chlist           Beauty.Diffuse Color.B(FLOAT); Beauty.Diffuse Color.G(FLOAT); Beauty.Diffuse Color.R(FLOAT)
  colorInteropID           string           'lin_rec709_scene'
  name                     string           'Beauty.Diffuse Color'
```

*A multi-layer OpenEXR written by a File Output node.* The same file name and the same two parts:

```
FILES: ['fromnode_0001.exr', 'main_0001.png']
  name                     string           'MyBeautyLayer'
  name                     string           'second socket'
```

Both machines were run and both agreed, name for name.

**Observed — several views inside one file, on the rented Linux machine, 2026-08-06.** A case neither machine had tried before. Set up: 32 by 32, one frame, the depth and normal passes on, `render.use_multiview = True` and `render.views_format = 'STEREO_3D'`, output `OPEN_EXR_MULTILAYER`. Blender reported `VIEWS: [('left', True), ('right', True)]`, and with the image settings' own `views_format` left at its default it wrote **two** files, `frame_0001_L.exr` and `frame_0001_R.exr`. With `image_settings.views_format = 'MULTIVIEW'` it wrote **one**, and the reader printed six parts, two per pass:

```
VERSION FIELD: 0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)
--- PART 0 ---
  channels                 chlist           Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT)
  name                     string           'Beauty.Combined-left'
--- PART 1 ---
  channels                 chlist           Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT)
  name                     string           'Beauty.Combined-right'
```

So in this version a view is a suffix on the part's name, `<view layer>.<pass>-<view>`, and it is not in the channel names at all.

**Observed — the window on the rented Linux machine, 2026-08-06.** Blender 5.0.1 was opened with a window on that machine's own display and photographed with `bpy.ops.screen.screenshot`.

With the ordinary multi-layer OpenEXR loaded, the Image Editor's header holds **two** picture-choosing menus, reading `Beauty` and `Combined`, and no third one. So there is no View menu to photograph for such a file, and the Mac's silence was not a missed picture.

With the multi-view file loaded, a stereo-glasses button appears in the header and the two views are shown together, still with only two menus. Switching that button off — `space.show_stereo_3d = False` — makes a **third** menu appear, after `Layer` and `Pass`, reading `left`. That is the View menu.

**Its contents could not be photographed open.** The pointer was put on it with `xdotool` in Blender 4.5 and Blender drew the control's own tooltip, which reads `Select View.` — so Blender saw the pointer, and that is the control's name. But no button press ever reached Blender, on that machine, with several topics running windowed Blenders on the one display at the same moment. What the file itself records is `left` and `right`, quoted above.

Separately, the Image Editor's own `View` menu in the header — the one beside `Image` — was opened with `bpy.ops.wm.call_menu(name="IMAGE_MT_view")` and photographed. It holds `Toolbar`, `Sidebar`, `Tool Settings`, `Asset Shelf`, `Adjust Last Operation`, `Update Automatically`, `Show Metadata`, `Frame All`, `Center View to Cursor`, `Zoom`, `Area`, and nothing in it chooses which picture is shown.

**Not covered.** The exact text of a layer name as written into the file is settled by the hand test above, for this version and this machine; the sources still do not state it. The manual gives the ingredients (View Layer plus pass) and the Cryptomatte page gives one worked instance of the combination, but never spells one out. The Python interface has `RenderPass.fullname` and `RenderPass.channel_id` alongside `name` (https://docs.blender.org/api/5.0/bpy.types.RenderPass.html) with no descriptions at all, and `RenderEngine.add_pass` says its name argument is the "Name of the Pass, without view or channel tag" (https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html) — which tells you a view tag and a channel tag get added somewhere, but not how. Also uncovered: whether any non-OpenEXR format Blender writes can hold several pictures (the manual's table gives no such column, and no other format's Format Details section mentions layers); and whether a multi-part file's parts are named differently from an interleaved file's. Note also that this question's other reading — video containers that hold several *streams* — is a different thing; the manual describes Matroska as able to "hold an unlimited number of video, audio, picture or subtitle tracks in one file", but those are tracks of a movie, not several pictures of one render.

**Outside the interface.** **Observed.** Both files above were written with no window open, and their names were read off the files themselves rather than out of Blender, so the naming holds for anything driving Blender from outside. `-F OPEN_EXR_MULTILAYER` on the command line was also observed to write such a file (recorded under question 3). What stays window-only is the *picking* of a layer to look at — `multilayer_layer`, `multilayer_pass` and `multilayer_view` are read-only in the API — but with the names above, a program outside Blender does not need it.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The same part names, the same multi-part shape and the same File Output node file name came back on both. The multi-view results and the window pictures above were got on the Linux machine alone, so they settle that system and leave the Mac untried for those. **Windows is still untried** — no Windows machine has run it. From the sources: OpenEXR support is among the parts "that can be compiled into Blender, not available on all systems", so a copy of Blender may be unable to write the format at all.

**Where I looked.** Searched the whole manual for `multi-?layer` (every hit reviewed), `layer name`, `pass name`, `\.exr.*layer`. Read `files/media/image_formats.txt` (OpenEXR section in full), `render/output/properties/output.txt`, `compositing/types/output/file_output.txt`, `compositing/types/input/image.txt`, `compositing/types/mask/cryptomatte.txt`, `render/output/animation.txt`, `render/output/properties/stereoscopy/usage.txt`. Read API pages `bpy.types.RenderPass`, `bpy.types.RenderResult`, `bpy.types.RenderEngine` (`add_pass`, `register_pass`), `bpy.types.ImageFormatSettings`, and the enums `views_format_multiview_items`, `views_format_multilayer_items`. Searched the release notes for `exr`, `multilayer`, `multi-part`.

---

## 6. Does a format that can hold more light than a screen can show also record the treatment it was made under?

**The question is ambiguous, and I flag it.** "The treatment it was made under" I read as the colour treatment — the view transform, look, exposure, gamma and curves that turn scene light into a viewable picture. The other reading is the render settings generally (engine, samples, and so on); on that reading see the last paragraph of the answer.

**Answer.** OpenEXR records *which colour space the numbers are in*, and in 5.0 it gained metadata for that. The sources do not state whether it also records the view transform, look, exposure, gamma or curves as metadata. They show that Blender does not automatically treat a reopened OpenEXR as a render and that the display treatment is not baked into an intermediate OpenEXR. That supports an inference that the treatment is not carried in a form Blender uses automatically, but it does not prove that no treatment field exists in the file.

The two high-dynamic-range formats also differ from each other: OpenEXR carries metadata; Radiance HDR carries none at all.

**LABEL: documented** for the colour-space metadata, reopening behaviour and Radiance HDR's lack of metadata; **inferred** for the claim that OpenEXR does not carry the treatment in a form Blender uses automatically.

**Quoted words — the format records the colour space.**

> "OpenEXR files now support Color Interop ID metadata, to automatically detect the file colorspace."

Release notes — https://developer.blender.org/docs/release_notes/5.0/pipeline_io/ — read 2026-07-30

> "Note
>
> When opening OpenEXR images, Blender will automatically set the Color Space if it is detected as Linear CIE-XYZ E or ACES2065-1."

Manual — https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30

> "Include the interop ID from the Color Interop Forum for every color space and display color space that you can. This helps save image and video with correct colorspace information."

and, on what that depends on:

> "Use OpenColorIO v2 display_colorspaces and view_transforms with cie_xyz_d65_interchange intermediate display linear space. … Saving colorspace metadata for images and video also depends on this."

Manual — https://docs.blender.org/manual/en/5.0/render/color_management/opencolorio.html — read 2026-07-30

**Quoted words — it does not record the treatment.**

> "By default, only renders are displayed and saved with the render View applied. These images are the 'Render Result' and 'Viewer' image data-blocks, and the files saved directly to disk with the Render Animation operator. However, when loading a render saved to an intermediate OpenEXR file, Blender cannot detect automatically that this is a render (it could be e.g. an image texture or displacement map). We need to specify that this is a render and that we want the transformations applied, with the following settings.
>
> View as Render
>
> Display the image data-block (not only renders) with view, exposure, gamma, RGB curves applied."

Manual — https://docs.blender.org/manual/en/5.0/render/color_management/displays_views.html — read 2026-07-30

This shows that the transformations must be applied again for viewing. It does not say that the file contains no metadata naming them.

Consistent with it, the option that decides whether the treatment is baked in tells you it is *not* baked into OpenEXR:

> "Save As Render
>
> Save image with render color management. For display image formats like PNG, apply view and display transform. For intermediate image formats like OpenEXR, use the default render output color space."

Manual — https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30

**Quoted words — the user-selectable render metadata fields shown by Blender have no colour-treatment field.** The manual's Metadata panel lists: Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Note, plus Burn into Image with its font size, colours and labels.

> "The Metadata panel includes options for writing metadata into render output.
>
> Note
>
> Only some image formats support metadata: See image formats."

Manual — https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html — read 2026-07-30

The Python interface's list of these user-selectable fields is `use_stamp`, `use_stamp_camera`, `use_stamp_date`, `use_stamp_filename`, `use_stamp_frame`, `use_stamp_frame_range`, `use_stamp_hostname`, `use_stamp_labels`, `use_stamp_lens`, `use_stamp_marker`, `use_stamp_memory`, `use_stamp_note`, `use_stamp_render_time`, `use_stamp_scene`, `use_stamp_sequencer_strip`, `use_stamp_time`, plus `stamp_note_text`, `stamp_font_size`, `stamp_background`, `stamp_foreground` and `metadata_input` — https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30. None concerns colour management. This list does not establish that no other automatic metadata is written; the Color Interop ID above is itself outside the list.

**Quoted words — the two HDR formats are not alike.** From the manual's format table (question 3): OpenEXR has Metadata ✓; **Radiance HDR has Metadata ✗**.

> "Radiance is a suite of tools for lighting simulation. Since Radiance had the first (and for a long time the only) HDR image format, this format is supported by many other software packages.
>
> Radiance .hdr files store colors still in 8 bits per component, but with an additional (shared) 8-bit exponent value, making it 32 bits per pixel."

Manual — https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30

**On the other reading** — whether the format records the render settings it was made under — the documented user-selectable fields cover the frame, the scene, the camera, the lens, the machine, the memory and the render time, but nothing about the engine or its settings. This does not establish that no other automatic field exists. There is a free-text Note field: "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number." (same metadata page).

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.0.1, started from a terminal with no window.

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
  Date                     string           '2026/08/03 22:45:40'
  File                     string           '<untitled>'
  Frame                    string           '001'
  RenderTime               string           '00:00.44'
  Scene                    string           'Scene'
  Software                 string           'Blender 5.0.1'
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
END OF HEADERS at byte 658
```

Setting the Date and RenderTime lines aside, the three OpenEXR headers were the same text: the one made under Standard matched the one made under AgX, and so did the one made with a Look and with exposure and gamma changed. The only difference between those files was how many bytes of pixels followed the header. **One attribute does carry colour: `colorInteropID`, and its value is the blend-file's working space, not the treatment.** No attribute names a view transform, a look, an exposure or a gamma. There is no `chromaticities` attribute.

To check what `colorInteropID` follows, the working space was changed with `bpy.ops.wm.set_working_color_space(working_space='ACEScg', convert_colors=False)` and another OpenEXR written. Blender printed `working_space after: ACEScg interop id: lin_ap1_scene`, and that file's header reads `colorInteropID string 'lin_ap1_scene'`.

The Radiance HDR file begins:

```
#?RADIANCE
FORMAT=32-bit_rle_rgbe

-Y 32 +X 32
```

and then the pixels. Nothing else is in its header.

The PNG's blocks, read in order, were `IHDR, eXIf, oFFs, pHYs, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, IDAT`, and the seven text entries were `File = <untitled>`, `Date = 2026/08/03 22:45:41`, `Time = 00:00:00:01`, `Frame = 001`, `Camera = Camera`, `Scene = Scene`, `RenderTime = 00:00.34`. There is no `sRGB`, `gAMA` or `cHRM` block in this version's PNG.

*Opening the files again.* Done, in a windowless Blender: `bpy.data.images.load(...)` on each file, then its colour space and View-as-Render state printed:

```
OPENED exr_agx.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_standard.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_punchy_exposed.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED radiance.hdr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED display_png.png size=(32, 32) depth=96 is_float=True colorspace='sRGB' use_view_as_render=False
```

All three OpenEXR files come back the same way whatever they were made under, with View as Render off.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.0.1, no window. The same files were written under the same changed settings and the same readers run over them. Blender printed:

```
DEFAULT view_transform='AgX' look='None' exposure=0.0 gamma=1.0
BLEND FILE COLORSPACE working_space = 'Linear Rec.709'
BLEND FILE COLORSPACE working_space_interop_id = 'lin_rec709_scene'
BLEND FILE COLORSPACE is_missing_opencolorio_config = False
```

The three OpenEXR headers came back as the same text apart from the date and the render time: no attribute names a view transform, a look, an exposure or a gamma, and there is no `chromaticities`. Each carries `colorInteropID string 'lin_rec709_scene'`. After `bpy.ops.wm.set_working_color_space(working_space='ACEScg', convert_colors=False)`, which reported `working_space after: ACEScg interop id: lin_ap1_scene`, the next OpenEXR carried `colorInteropID string 'lin_ap1_scene'`.

The Radiance file begins `#?RADIANCE` and `FORMAT=32-bit_rle_rgbe` and holds nothing else. The PNG's blocks, read in order:

```
PNG display_png.png CHUNKS: IHDR, eXIf, oFFs, pHYs, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, IDAT, IEND
```

No `sRGB`, `gAMA` or `cHRM` block, as on the Mac.

*Opening the files again.* Blender printed:

```
OPENED exr_agx.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_standard.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_punchy_exposed.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED radiance.hdr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED display_png.png size=(32, 32) depth=96 is_float=True colorspace='sRGB' use_view_as_render=False
```

Both machines were run and both agreed, line for line.

**Not covered.** How Blender behaves when an OpenEXR's Color Interop ID is absent or unrecognised — the release note says it can "automatically detect the file colorspace" but does not say what happens otherwise. What the metadata looks like as written (field names, spellings). Whether the working space (new in 5.0, and the thing that gives the numbers their meaning) is written into a saved image; `BlendFileColorspace.working_space_interop_id` exists as a property of the blend-file (https://docs.blender.org/api/5.0/bpy.types.BlendFileColorspace.html) but the sources do not say it is written into image files. Whether the JPEG preview copy (question 8) carries the treatment. And nothing here settles behaviour for formats other than OpenEXR and Radiance HDR.

**Outside the interface.** **Observed.** Every file above was written, and every header read, with no window open — the view transform, the look, the exposure and the gamma were all set from a script, the working space was changed by an operator, and the renders were driven from a terminal. So the answer holds when Blender is driven from outside its own window: the colour space is written, in `colorInteropID`; the treatment is not. (Previously, before the test: not addressed in these terms.) Documented alongside: the colour management settings used when saving are stored on the scene, and the output panel lets a file override them — "Follow Scene: Uses the same color management settings defined by the active Scene" / "Override: Uses custom color management settings defined by the properties below in the panel; disregarding any color management settings set at the Scene level." (https://docs.blender.org/manual/en/5.0/render/output/properties/output.html), matching `ImageFormatSettings.color_management` (enum `['FOLLOW_SCENE', 'OVERRIDE']`). Since these live in the file rather than in the window, a render driven from outside uses the same ones — **that is inferred**, not stated.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** A file written on either machine carries the same attributes, including `colorInteropID`, and reopens the same way. **Windows is still untried** — no Windows machine has run it. From the sources: they do not say that the information written into the file differs by operating system, and they do say that viewing differs: HDR and wide gamut require "macOS: Any Apple Silicon device. Linux: Use Wayland, and set the Vulkan backend in the Blender system preferences. Windows: Enable 'Use HDR' in the Windows display settings, and set the Vulkan backend in the Blender system preferences." (https://docs.blender.org/manual/en/5.0/render/color_management/displays_views.html), and the release notes add "There are inconsistencies in the HDR implementations in operating systems, browsers and video players." (https://developer.blender.org/docs/release_notes/5.0/color_management/).

**Where I looked.** Searched the whole manual and API documentation for `interop`, `chromaticit`, `colorspace metadata`, `color space.*metadata`, `metadata.*color space`. Read in full `render/output/properties/metadata.txt`, `render/color_management/index.txt`, `render/color_management/displays_views.txt`, the "Writing Configurations for Blender" part of `render/color_management/opencolorio.txt`, and the Saving Images / Format Details parts of `files/media/image_formats.txt`. Read the API pages `bpy.types.ImageFormatSettings`, `bpy.types.ColorManagedViewSettings`, `bpy.types.BlendFileColorspace`, and listed every stamp-related property on `bpy.types.RenderSettings`. Read the 5.0 release notes `color_management.md` and `pipeline_io.md` in full.

---

## 7. Can Blender turn a set of finished frames into a short moving picture?

**Answer.** Yes, and the manual recommends it as the normal way of working rather than rendering straight to video. The frames are loaded into the Video Sequencer as an image strip, the output is set to a video container and codec, and the animation is rendered again — this second render encodes rather than re-renders the 3D scene.

There is one documented exception: multi-layer OpenEXR frames cannot go through the Video Sequencer, and must be brought in through the Compositor's Image node instead.

**LABEL: documented.**

**Quoted words.**

> "Images can then be encoded to a video by adding the rendered image sequence into the Video Sequencer and choosing an appropriate Video Output."

Manual — https://docs.blender.org/manual/en/5.0/render/output/introduction.html — read 2026-07-30

The step-by-step, from the Frame Sequence Workflow:

> "In Blender, open the Video Sequencer.
>
> Note
>
> The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor.
>
> Choose Add Image from the add menu. Select all the frames from your output folder that you want to include in your animation. They will be added as a strip in the Sequence editor.
>
> Now you can edit the strip and add effects or leave it like it is. You can add other strips, like an audio strip.
>
> Scrub through the animation to check if you have included all the frames.
>
> In the Output panel, choose the container and codec you want (e.g. MPEG H.264) and configure them. The video codecs are described in Output Options.
>
> Click the Animation render button and Blender will render out the Sequence editor output into a movie."

Manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30

Why it is the recommended route:

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning. If you first render out a set of static images such as the default PNG format or the higher-quality OpenEXR (which can retain HDR pixel data), you can combine them as an Image Strip in the Video Sequencer. This way, you can easily:
>
> Restart the rendering from the place (the frame) where any problem occurred.
>
> Try out different video encoding options in seconds, rather than minutes or hours as encoding is usually much faster than rendering the 3D scene."

Manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30

The switch that makes the second render encode the strips rather than re-render the scene:

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Manual — https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html — read 2026-07-30

How the frames must be named to be recognised as a sequence:

> "To load image sequence in any of the supported image file formats, the filename of the images must contain a digit to indicate the frame order (e.g. *-0001.jpg, *-0002.jpg, *-0003.jpg, etc, of any image format), indicating the frame."

Manual — https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30

A mismatch between frame size and output size is handled:

> "Dealing with different sized images and different sized outputs is tricky. If you have a mismatch between the size of the input image and the render output size, the Video Sequencer will try to auto-scale the image to fit it entirely in the output."

Manual — https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/image.html — read 2026-07-30

**Also documented, and a different thing.** Blender can *play* a set of frames without making a movie of them, using the animation player: "Rendered image sequences can be played back in the Animation Player." (https://docs.blender.org/manual/en/5.0/render/output/introduction.html), and `-a <options> <file(s)>` on the command line "runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)" (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html). This produces no file.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.0.1, started from a terminal with no window throughout. **This settles the disagreement between this document and `Preparation/Blender Research/Findings/5.0/Images — second pass.md`, which reads the outside-the-window half differently. The observed result below is what now stands, and the same result is written into that document too.**

*The whole job with no window at all.* Set up, in a windowless Blender: six PNG frames at 64 by 48 rendered from the ordinary scene into a folder; then the 3D scene changed so a fresh render could not be mistaken for those frames — the cube deleted and the world background set to pure blue; then an image strip built from the six saved files with `scene.sequence_editor_create()` and `strips.new_image(name="Frames", filepath=<first file>, channel=1, frame_start=1)` followed by `strip.elements.append(<each remaining file name>)`. Blender printed `STRIP: Frames type: IMAGE elements: 6` and `use_sequencer: True`. Four blend-files were saved from that state: video output with the Sequencer pipeline on, video output with it off, PNG output with it on, PNG output with it off.

Done, in a terminal:

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b -noaudio --factory-startup /tmp/images-hand-tests/out/t16/5.0.1/sequence-test.blend -a
```

Blender printed, for each frame:

```
00:00.371  render           | Rendering animation (frames 1..6)
00:00.381  render           | Rendering frame 1
00:00.381  render           | Executing sequencer
00:00.434  render           | Video append frame 1
00:00.434  render           | Time: 00:00.05 (Saving: 00:00.03)
```

The exit status was `0`, and one file appeared at the output path: `out_0001-0006.mp4`, which `file` reported as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. The same command on the file with the Sequencer pipeline switched off wrote `nosec_0001-0006.mp4`.

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
s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3181
s_0002.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3181
s_0003.png   size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1170
s_0004.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3181
s_0005.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3181
```

The frame whose file was gone came out fully transparent black.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Set up: three multi-layer OpenEXR frames at 64 by 48 with the depth and normal passes on; a strip built from them; PNG output. Done: the same background animation render. Three files were written, with no message about the format:

```
s_0001.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=2929
s_0002.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=2929
s_0003.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=2929
```

Compared with the same scene's PNG frames put through the same route, which gave `centre=(0.5373, ...)` and `corner=(0.2, ...)`, the picture is there but darker.

*How long the result is.* Set up: twenty-four frames, a strip built from all of them, video output with the MPEG-4 container and the H.264 codec. Done: encoded once with `render.fps = 24` and once with `render.fps = 12`, then each movie's own header read for the time scale and duration it records:

```
at24_0001-0024.mp4   bytes=2314   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4   bytes=2592   timescale=1000 duration=2000 -> 2.000 s
```

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 5.0.1 open. Seen: the panel's top control, labelled `Media Type`, offers `Image`, `Multi-Layer EXR` and `Video`; with `Video` chosen, the `Encoding` section's `Container` menu offers `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`, and the `Video Codec` menu offers `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`. So this build really does offer video output, and the container and codec used in the test above are both on those menus.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.0.1, with no window throughout, driven by the same scripts.

*The whole job with no window at all.* `blender -b -noaudio --factory-startup .../sequence-test.blend -a` printed:

```
00:00.533  render           | Executing sequencer
00:00.715  render           | Video append frame 1
00:00.715  render           | Executing sequencer
00:00.716  render           | Video append frame 2
```

and so on to frame 6. The exit status was `0`, and `out_0001-0006.mp4` appeared, reported by `file` as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. Rendered again to PNG, the frames read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

*A folder of frames with nothing prepared.* The same command wrote `n_0001-0003.mkv`, reported as `Matroska data` — a movie of the 3D scene.

*A frame missing from the set.* All five output frames were written, nothing was printed about the missing one, and the third came out fully transparent black: `s_0003.png size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1164`.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Three files written with no message about the format, `centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0)`.

*How long the result is.* The two movies' own headers record:

```
at24_0001-0024.mp4 bytes=2314   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4 bytes=2597   timescale=1000 duration=2000 -> 2.000 s
```

Both machines were run and both agreed on every one of these, with only the byte sizes of the written files differing by a few bytes.

*Which container and codec will actually take it.* A further test, recorded in full under question 4 and run on both machines, encoded three frames for every container with H.264 and for the MPEG-4 container with every codec. Ten of those pairings are refused with `Error: Could not initialize streams, probably unsupported codec combination`, and three more with a complaint about the picture size. So the two menus offer more pairings than this version will write.

**Not covered.** The sources quoted here give the workflow as interface actions and name only the Add menu for creating the strip; the hand test above shows the same job done without any of them. The sources do not say what happens if the frames are of mixed sizes or mixed formats within one strip (they do say batch-adding "can be a mix of file types (jpg, png, exr, etc.)"). They do not say whether the encoding step re-applies the scene's colour management to already-treated frames. They do not settle whether a set of frames can be encoded without a blend-file prepared for it.

**Outside the interface.** **Observed — yes, the whole job.** Both halves were done with no window open: the image strip was built from the finished frames by a script in a windowless Blender, and the movie was encoded by `blender -b <file> -a` from a terminal, with Blender printing `Executing sequencer` for each frame. The written frames prove the pictures came from the strip and not from a fresh 3D render. So the answer holds when Blender is driven from outside its own window. The one thing that does not is the animation player, which the command-line page says is ignored when `-b` is given. (Previously, before the test: the encoding half was inferred and the assembly half was not established.) The manual notes for the encoding itself: "When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d." (https://docs.blender.org/manual/en/5.0/render/output/properties/output.html) — note that the 5.0 release notes say `--debug-ffmpeg` was replaced by `--log video` (https://developer.blender.org/docs/release_notes/5.0/core/).

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The whole job — building the strip with no window and encoding it from a terminal — works on both, and the frames, the file types and the recorded durations match. This is the machine a Smoozy render actually happens on, and FFmpeg is present in its build too (`codec_ffmpeg=True`, recorded under question 3). **Windows is still untried** — no Windows machine has run it. From the sources: they do not say the workflow differs by operating system, and they do say the result can differ, since which codecs exist and play back is a per-system matter: "Each has advantages and disadvantages, and compatibility with different players on different operating systems" and "FFMPEG" is among the parts "not available on all systems".

**Where I looked.** Read in full `render/output/animation.txt`, `render/output/introduction.txt`, `render/output/properties/output.txt`, `render/output/properties/post_processing.txt`, `files/media/video_formats.txt`, `video_editing/edit/montage/strips/image.txt`, and the "Opening an Image Sequence" part of `files/media/image_formats.txt`. Listed the manual's `video_editing/` and `editors/video_sequencer/` trees. Read the command-line pages for animation and playback arguments. Searched the release notes' `sequencer.md` heading list for changes to image strips.

---

## 8. Can Blender be made to produce a smaller, plainly viewable copy of a picture a render produced?

**The question is ambiguous, and I flag it.** "Smaller" could mean smaller in file size or smaller in pixels. "Plainly viewable" I read as: openable in an ordinary image viewer, without the special handling a linear high-dynamic-range file needs. I answer the file-size-and-viewability reading first, since Blender has a feature built for exactly it, and cover the pixel-size reading after.

**Answer.** Blender has a purpose-built **Preview** setting that writes a plainly viewable JPEG copy of an OpenEXR render. The sources do not say that the copy is smaller in pixels or in file size, so the "smaller" part of the question is not established. The manual says Preview applies when rendering animations or when rendering single frames from the command line.

**LABEL: documented** for the JPEG preview copy; **not in the sources** for whether it is smaller.

**Quoted words.**

> "Preview
>
> When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview."

Manual, under OpenEXR ‣ Output Options — https://docs.blender.org/manual/en/5.0/files/media/image_formats.html — read 2026-07-30

The same setting in the Python interface, where it sits on the image format settings generally rather than being marked OpenEXR-only:

> "use_preview
>
> When rendering animations, save JPG preview images in same directory
>
> Type: boolean, default False"

Python API, `ImageFormatSettings` — https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html — read 2026-07-30

Note the two descriptions differ: the manual adds "(or single frames via command line)" and says JPEG "copy of the image"; the API says "JPG preview images in same directory". Both agree it is off unless asked for (the API gives `default False`).

**Documented — other routes to the same end.**

*Converting a high-dynamic-range render to a viewable format, with the treatment applied:*

> "Save as Render
>
> Option in the image save operator to apply the view, exposure, gamma, RGB curves. This is useful for saving linear OpenEXR to e.g. PNG or JPEG files in display space."

Manual — https://docs.blender.org/manual/en/5.0/render/color_management/displays_views.html — read 2026-07-30 [Citation check 2026-08-04: the quoted heading read "Save As Render"; that page spells it "Save as Render". Corrected to the page.]

*Writing an extra, differently-formatted copy during the render itself:* the File Output node "writes one or more images to disk during rendering or compositing", each input can carry "a custom format instead of the global Node Format" via Override Node Format, and "Use the File Output node to save raw passes or intermediate steps without overwriting the main render result." — https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — read 2026-07-30.

*Making the picture itself smaller in pixels* (the other reading), before it is rendered:

> "Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image."

Manual — https://docs.blender.org/manual/en/5.0/render/output/properties/format.html — read 2026-07-30

*Making an image smaller in pixels from a script:*

> "scale(width, height, *, frame=0, tile_index=0)
>
> Scale the buffer of the image, in pixels"

and, for writing one out under the scene's output settings:

> "save_render(filepath, *, scene=None, quality=0)
>
> Save image to a specific path using a scenes render settings"

Python API, `Image` — https://docs.blender.org/api/5.0/bpy.types.Image.html — read 2026-07-30

*A thumbnail of the scene, which is a different thing but the nearest neighbour:* "Blender saves by default a small preview of current scene in the blend-file. This will show in the Thumbnail view of the File Browser. During its installation, Blender also adds a small tool to your OS, that will allow your system file browser to show those previews as file thumbnails as well." — https://docs.blender.org/manual/en/5.0/files/blend/previews.html — read 2026-07-30. That preview lives inside the blend-file, not beside the render.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.0.1, started from a terminal with no window.

*The Preview copy.* Set up: 64 by 48, frames 1 to 3, `image_settings.use_preview = True`, first with `file_format = 'OPEN_EXR'` and then with `'PNG'`. Done: `bpy.ops.render.render(animation=True)` each time, then every written file's byte size and pixel size read. Blender printed:

```
--- OPEN_EXR, use_preview set to True, now reads True ---
  FILE anim_0001.exr                3402 bytes  64x48
  FILE anim_0001.jpg                1136 bytes  64x48
  FILE anim_0002.exr                3402 bytes  64x48
  FILE anim_0002.jpg                1136 bytes  64x48
  FILE anim_0003.exr                3402 bytes  64x48
  FILE anim_0003.jpg                1136 bytes  64x48
--- PNG, use_preview set to True, now reads True ---
  FILE anim_0001.png                2946 bytes  64x48
  FILE anim_0002.png                2946 bytes  64x48
  FILE anim_0003.png                2946 bytes  64x48
```

So with OpenEXR a JPEG copy appears for every frame, beside the main file and with the same stem; it has the same number of pixels and about a third of the bytes. With PNG the setting can be switched on and reads back as on, and no JPEG copy is written.

Then a blend-file was saved with OpenEXR output and Preview on, and one frame rendered from a terminal on the Creator's ordinary configuration (not factory settings):

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b -noaudio /tmp/images-hand-tests/out/t12/5.0.1/preview-test.blend -f 1
```

Blender printed `Saved: '.../CLI/cli_0001.exr'` and `Saved: '.../CLI/cli_0001.jpg'` — 2193 bytes and 1189 bytes. So the copy is written for a single frame rendered from the command line, as the manual says.

*A smaller second copy made during the render.* Set up: 64 by 48, one frame, main output PNG; a compositing node group with the Render Layers image going to the Group Output and also through a Scale node with X and Y set to 0.5 into a File Output node writing PNG into the same folder. This version's Scale node has no `space` setting; its Type input socket already read `Relative`. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
SCALE NODE inputs: ['Image', 'Type', 'X', 'Y', 'Frame Type', 'Interpolation', 'Extension X', 'Extension Y']
SCALE NODE has no space property; Type socket reads: 'Relative'
OPERATOR RESULT: {'FINISHED'}
FILE main_0001.png          2739 bytes  64x48  depth=32 is_float=False
FILE small_small_0001.png   1247 bytes  32x24  depth=128 is_float=True
```

The second file is half the size in each direction. Its name is the node's file name and the socket's name run together.

*Resizing a rendered picture and saving a copy.* Set up: one OpenEXR render at 640 by 360, loaded back with `bpy.data.images.load(...)`. Done, in order, with whatever each returned kept:

```
--- bpy.ops.image.resize(size=(320, 180)) ---
RAISED RuntimeError: Operator bpy.ops.image.resize.poll() failed, context is incorrect
--- image.scale(320, 180) ---
scale() returned, size now (320, 180)
--- bpy.ops.image.save_as(filepath='/tmp/images-hand-tests/out/t14/5.0.1/small.png', save_as_render=True, copy=True) ---
RAISED RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect
small.png exists: False
--- image.save_render('/tmp/images-hand-tests/out/t14/5.0.1/small_render.png') ---
save_render returned
--- scene format set to PNG, then image.save_render('/tmp/images-hand-tests/out/t14/5.0.1/small_as_png.png') ---
save_render returned
FILE big.exr               57205 bytes  640x360
FILE small_as_png.png      33613 bytes  320x180
FILE small_render.png      48365 bytes  320x180
```

The two Image Editor commands refuse to run with no window. The two plain calls work. One thing to note about the first `save_render`: although the path ends `.png`, `file` reports that file as `OpenEXR image data, version 2 ... dataWindow: (0 0)-(319 179)`, because `save_render` writes in the scene's own output format. After the scene's format was set to PNG, the second call wrote `PNG image data, 320 x 180, 16-bit/color RGBA, non-interlaced`.

*Whether a saved proxy setup builds during a background run.* Set up: three finished PNG frames at 128 by 96; an image strip built from them; `strip.use_proxy = True` and `strip.proxy.build_25 = True` with the other three sizes off; the sequence editor's proxy storage reading `PER_STRIP`; output PNG; the file saved; no `BL_proxy` folder present. Done, in a terminal: `blender -b .../proxy-test.blend -a`. Blender printed only the three `Saved:` lines for the output frames, the exit status was `0`, and no `BL_proxy` folder existed anywhere under the test folder afterwards.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 5.0.1 open. Looked at: Properties ▸ Output Properties ▸ Output panel, with the format changed in turn, watching for a control named `Preview`. Seen: with `PNG (.png)`, `JPEG (.jpg)` and `TIFF (.tif)` there is no `Preview` control anywhere in the panel. With `OpenEXR (.exr)` a checkbox reading `Preview` appears under `Codec`, unticked. With `Media Type` set to `Multi-Layer EXR` the same `Preview` checkbox appears, unticked, below one reading `Interleave`. So the interface offers Preview only for the two OpenEXR cases, which matches the terminal result above, where switching `use_preview` on for PNG produced no JPEG copies.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 5.0.1, no window, driven by the same scripts.

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
  FILE anim_0001.png                2930 bytes  64x48
  FILE anim_0002.png                2930 bytes  64x48
  FILE anim_0003.png                2930 bytes  64x48
```

The single-frame command-line run wrote `cli_0001.exr` at 2186 bytes and `cli_0001.jpg` at 1183 bytes.

*A smaller second copy made during the render.* Blender printed:

```
SCALE NODE inputs: ['Image', 'Type', 'X', 'Y', 'Frame Type', 'Interpolation', 'Extension X', 'Extension Y']
SCALE NODE has no space property; Type socket reads: 'Relative'
FILE main_0001.png          2743 bytes  64x48  depth=32 is_float=False
FILE small_small_0001.png   1212 bytes  32x24  depth=128 is_float=True
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
FILE small_as_png.png      33737 bytes  320x180
FILE small_render.png      48325 bytes  320x180
```

`file` reported `small_render.png` as `OpenEXR image data, version 2 … dataWindow: (0 0)-(319 179)` and `small_as_png.png` as `PNG image data, 320 x 180, 16-bit/color RGBA, non-interlaced`, exactly as on the Mac.

*Whether a saved proxy setup builds during a background run.* Only the three output frames were written, the exit status was `0`, and no `BL_proxy` folder existed anywhere afterwards.

Both machines were run and both agreed on every part of this, with only byte sizes differing slightly.

**Not covered.** The JPEG quality the Preview copy is written at, and what colour treatment is applied to it. Whether Preview fires for a single frame rendered from inside the window — the hand test settles the command-line case only.

**Outside the interface.** **Observed, and it splits three ways.** The Preview copy works from outside: it was written for a three-frame animation and for a single frame started from a terminal, matching the manual's "(or single frames via command line)". The Scale-plus-File-Output route works from outside: the half-size second file was written by a windowless render. The two Image Editor commands do **not** work from outside: `bpy.ops.image.resize` and `bpy.ops.image.save_as` both refuse with `poll() failed, context is incorrect`, and the plain calls `Image.scale` and `Image.save_render` do the same job instead — with the catch that `save_render` writes in the scene's output format, whatever the path's ending says. Proxies were not built during a background animation render.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The Preview copy, the half-size second file, both refusals, both plain calls and the absence of proxies all came back the same on both. **Windows is still untried** — no Windows machine has run it. From the sources: they do not say Preview differs by operating system. JPEG is among the formats always available (it is in the `-F` list of options that are not marked as build-dependent), whereas OpenEXR itself is not — so on a copy of Blender without OpenEXR the question does not arise. The one operating-system-specific note nearby concerns blend-file thumbnails, not renders: "macOS — The Blender Thumbnail extension can be disabled in the System Settings under 'Login Items & Extensions', within the Quick Look category." (https://docs.blender.org/manual/en/5.0/files/blend/previews.html).

**Where I looked.** Searched the manual and API documentation for `preview` in output and image-format contexts, and read the OpenEXR ‣ Output Options section of `files/media/image_formats.txt` in full. Read `render/output/properties/format.txt`, `render/output/properties/output.txt`, `files/blend/previews.txt`, `compositing/types/output/file_output.txt`, `editors/image/editing.txt` (Save / Save As / Open Cached Render), and the Images ‣ Output part of `render/color_management/displays_views.txt`. Read the API page `bpy.types.ImageFormatSettings` in full and the `save`, `save_render`, `scale` and `pack` entries of `bpy.types.Image`. Opened the Output panel picture (`_images/render_output_properties_output_panel.png`) to see whether a Preview control appears there; it does not — the panel shown holds path, Overwrite, Placeholders, File Extensions, Cache Result, File Format, Color, Color Depth and Compression, with File Format on PNG (a format for which the manual's Preview text, being under OpenEXR, would not apply).

---

## Things a running Blender 5.0 could settle

Every item this document sent to the hand tests has been run, on the Creator's own MacBook Pro (M1 Pro, macOS 26.5.2), and the results are written in above under the questions they answer. The whole list is at `Audits/Blender Research/Images/Hand tests.md`.

The three checks that needed eyes on Blender's window were run by the Creator on 2026-08-04 and are written in above: the Output panel's two controls and its Preview box (questions 3 and 8), the Container and Video Codec menus (questions 3 and 7), and the Image Editor's Layer and Pass menus (questions 1 and 5). Item 5 on this list — which codecs remain selectable for each container — is answered only in part: the two menus were photographed with the container on `Matroska`, and no picture shows the codec list under a different container.

Everything on that list was then run again on a second machine — a rented Linux machine, Ubuntu 22.04.5 LTS with an NVIDIA GeForce RTX 4090 — on 2026-08-06, together with the tests the one Mac could never reach. That second list is at `Audits/Blender Research/Images/Hand tests — Linux.md`, and its results are written in above beside the Mac's. **Three of the five gaps below are now closed**: the Image Editor's View menu, a container-and-codec pairing Blender itself refuses, and another make of graphics card.

What is still open:

1. **Windows.** Every observed answer above was got on macOS on Apple silicon or on Ubuntu on an NVIDIA card, and on both where the test could be repeated. No Windows machine has run it, so that system stays untried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

2. **A build that lacks a format.** (Question 4.) Still cannot be produced. Both machines were checked and the official build for each has every optional format compiled in. It needs a Blender built without one of them.

3. **A graphics card that is neither Apple's nor NVIDIA's.** (Question 2.) The missing-texture result now covers Apple's Metal graphics, an NVIDIA card through CUDA and the same card through OptiX. AMD graphics, Intel graphics and a second card of any make cannot be tried.

4. **What the View picker's open menu looks like.** (Questions 1 and 5.) The picker itself is now photographed — it exists only for a file with several views in it, appears when the stereo button is off, is called `Select View.` and reads `left`. Opening it needs a click, and on the shared Linux display no button press reached Blender.

5. **Which separate passes carry the missing-texture colour.** (Question 2.) Both machines read the finished picture only.

---

2026-08-03 — Opus 5 (Claude Code) — Images, Blender 5.0

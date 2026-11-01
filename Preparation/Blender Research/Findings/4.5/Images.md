# Images — Blender 4.5

**Sources consulted, and when.** Blender's 4.5 manual, Blender's 4.5 Python API documentation, Blender's 4.5 release notes, and Cycles' own files as shipped with 4.5 — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is the date read for every finding below. Research carried out 2026-08-03.

**Declaration.** I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project — with one exception I must declare plainly: to confirm the folder I was told to write into existed, I ran a directory listing of `Preparation/Blender Research/Findings/`. That listing returned four folder names (`4.5`, `5.0`, `5.1`, `5.2`) and nothing else. I opened no file inside `Preparation/` other than the question sheet I was given, and no file anywhere else in the project. No Blender information reached me from anything but `Blender Sources/`.

---

## 1. What kinds of picture can a render produce, and how can the finished picture be told apart from the others?

**Answer.** A render produces a set of separate images called *passes*. The manual defines them and lists them by name. The finished picture is one of them, and it is the one named **Combined**.

The manual's own definition:

> "A Pass is a type of intermediate rendering information that's extracted as a separate image. Examples include the diffuse colors of the objects in the scene, the light distribution, the depth map, and the normal map."

The kinds it lists, under headings, with the engines each is available for given beside the name in the source text:

- **Data** — Combined (Cycles, EEVEE, Workbench); Z (Cycles, EEVEE, Workbench); Mist (Cycles, EEVEE); Position (Cycles, EEVEE); Normal (Cycles, EEVEE); Vector (Cycles, EEVEE); UV (Cycles); Grease Pencil (Cycles, EEVEE, Workbench); Denoising Data (Cycles).
- **Indexes** — Object Index (Cycles); Material Index (Cycles).
- **Debug** — Sample Count (Cycles).
- **Light (Cycles)** — Diffuse Direct/Indirect/Color; Glossy Direct/Indirect/Color; Transmission Direct/Indirect/Color; Volume Direct/Indirect; Emission; Environment; Ambient Occlusion; Shadow Catcher.
- **Light (EEVEE)** — Diffuse Light/Color; Specular Light/Color; Volume Light; Emission; Environment; Shadow; Ambient Occlusion; Transparent.
- **Cryptomatte** — Object, Material, Asset.
- **Shader AOV** — user-created passes, named by the user.
- **Light Groups** (Cycles only) — "A Light Group provides a limited Combined render pass where the scene is only illuminated by certain lights."

How the finished one is told apart — it is the pass called Combined, described as:

> "The render output before any compositing is applied."

The manual also shows the arithmetic by which the other passes add back up to it, in a diagram whose text labels are the light-pass names and, as the last label, "Combined". (I read that diagram's label text, not the rendered picture; the labels are Diffuse Direct, Diffuse Indirect, Diffuse Color, Glossy Direct, Glossy Indirect, Glossy Color, Transmission Direct, Transmission Indirect, Transmission Color, Emission, Environment, Volume Direct, Volume Indirect, Combined, joined by `+` and `×` symbols.)

Inside Blender's own window, the manual documents a selector for choosing among them:

> "Render Result: displays renders. When this item is selected, the Slot, View Layer, and Render Pass selectors become available (see below)."

and, in that list, "Render Pass — The Render Pass to display."

In Cycles' own files the name is a literal string:

```
    # Combined pass.
    yield ("Combined", "RGBA", 'COLOR')
```

The Python API carries a matching machine-readable identifier list in which the first entry is `COMBINED`.

**LABEL: documented** — for the list of passes, for "Combined" being the render output before compositing, and for the Render Pass selector. **inferred** — that `Combined` is the answer to the question's "finished picture" wording; that mapping depends on the reading explained below. **observed** — only for the quoted Cycles file containing the literal name `Combined` and the tuple `("Combined", "RGBA", 'COLOR')`. The file quote does not establish how a completed render writes that pass to disk.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2. Blender 4.5.12 was started from a terminal with no window (`-b --factory-startup -noaudio`) and driven by a script. The whole list of tests is at `Audits/Blender Research/Images/Hand tests.md`.

*Two view layers, one plain single-image output.* Set up: the factory startup scene at 32 by 32; the world background set to pure red; a second view layer added, called `Second`, with the cube's collection excluded from it; output PNG; frames 1 to 1. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
VIEW LAYERS: ['ViewLayer', 'Second']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.8039, 0.5451, 0.5333, 1.0) corner=(0.8549, 0.2196, 0.1255, 1.0)
```

One file appeared, not one per view layer. Its corner carries the red world and its centre carries the lit cube.

*What lands in the output file when compositing is on.* Set up: the same scene at 32 by 32, PNG, one frame, with `scene.use_nodes = True` and a new `CompositorNodeRGB` set to pure blue linked into the first input of the existing `CompositorNodeComposite`. Done: the same render call. Blender printed:

```
use_compositing: True
NODES: ['CompositorNodeComposite', 'CompositorNodeRLayers', 'NodeReroute', 'CompositorNodeViewer', 'CompositorNodeRGB']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

Centre and corner are both the compositor's blue.

*What the finished picture is called once written.* From the multi-layer OpenEXR test recorded under question 5: with the view layer renamed to `Beauty`, the finished picture's channels in the written file are `Beauty.Combined.R`, `Beauty.Combined.G`, `Beauty.Combined.B` and `Beauty.Combined.A`.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 4.5.12 open. Looked at: the Image Editor header with the multi-layer OpenEXR from the test above loaded, its two picture-choosing menus opened in turn. Seen: the header shows the file name `frame_0001.exr` and then two dropdown menus, the first reading `Beauty` and the second reading `Combined`. Opening the first shows `Beauty` and `Second` above a greyed label reading `Layer`. Opening the second shows `Combined`, `Depth`, `Normal`, `Mist`, `Emit`, `DiffCol` above a greyed label reading `Pass`. So a person at the screen picks the finished picture by choosing `Combined` in the menu labelled `Pass`, within the view layer chosen in the menu labelled `Layer`.

**Ambiguity flagged.** "The finished picture" has two readings. I answered the one I take to be meant: among the several images one render produces, the one that is the picture rather than a component of it — that is Combined. The other reading is "the picture that finally gets saved after compositing", which is not Combined: the manual says Combined is "before any compositing is applied", and separately that the Composite node "defines the final output of the Compositor. It is the node where the image result is sent to the renderer or image output after rendering." If compositing is enabled, the saved picture is the Composite node's result, not the raw Combined pass. The sources do not give that composited result a pass name of its own.

**Observed — the same tests on a second machine, 2026-08-06.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 4.5.12 LTS at `/opt/blender/blender-4.5.12-linux-x64/blender`, started with no window (`-b --factory-startup -noaudio`) and driven by the same scripts the Mac run used, with only the folder path changed. The whole list is at `Audits/Blender Research/Images/Hand tests — Linux.md`.

*Two view layers, one plain single-image output.* Blender printed:

```
ENGINE: BLENDER_EEVEE_NEXT
VIEW LAYERS: ['ViewLayer', 'Second']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.8039, 0.5451, 0.5333, 1.0) corner=(0.8549, 0.2196, 0.1255, 1.0)
```

*Compositing switched on.* Blender printed:

```
use_compositing: True
NODES: ['CompositorNodeComposite', 'CompositorNodeRLayers', 'NodeReroute', 'CompositorNodeViewer', 'CompositorNodeRGB']
OPERATOR RESULT: {'FINISHED'}
FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

*The names once written.* The multi-layer OpenEXR written on this machine carries the same channel names as the Mac's — `Beauty.Combined.R`, `.G`, `.B`, `.A`.

Both machines were run and both agreed, to the fourth decimal place.

**Observed — the window on the rented Linux machine, 2026-08-06.** Blender 4.5.12 was opened with a window on that machine's own display, the largest area turned into an Image Editor, the multi-layer OpenEXR above loaded into it, and the window photographed with Blender's own screenshot command, `bpy.ops.screen.screenshot`. The picture shows the header holding **two** picture-choosing menus, reading `Beauty` and `Combined` — the same two the Creator photographed on the Mac — and no third menu beside them. The Image Editor's own `View` menu, which sits in the header next to `Image`, was then opened with `bpy.ops.wm.call_menu(name="IMAGE_MT_view")` and photographed. It holds, top to bottom: `Toolbar`, `Sidebar`, `Tool Settings`, `Asset Shelf`, `Adjust Last Operation`, `Update Automatically`, `Show Metadata`, `Frame All`, `Center View to Cursor`, `Zoom`, `Area`. `Center View to Cursor` is greyed out. Nothing in that menu chooses which picture is shown.

**Not covered.** How the passes are distinguished *inside a saved file* — that is question 5. Also not covered: whether Workbench and EEVEE emit the same literal pass name strings as Cycles (I could only observe Cycles' own files; the manual gives display names, not file names). Nothing here settles what a pass contains numerically, only what it is called and what the manual says it represents.

**Outside the interface.** **Observed.** The three tests above were all run with no window open at all, and each did what is recorded: the passes were switched on as view-layer properties, the render was started, the file was written, and the compositor's result was the thing written. So the pass settings, the pass names inside the file, and the compositor deciding the saved picture all hold when Blender is driven from outside its own window. What remains window-only is the *picking*: the Slot / View Layer / Render Pass selectors are documented as part of the Image Editor, and outside the window the choice is made by what gets written to a file. (Previously, before the test: partly documented, partly inferred.)

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** Every test above was run on both and came back the same, and the Image Editor's `Layer` and `Pass` menus were photographed on both. [Citation check 2026-08-06: this read "the Image Editor's menus were photographed on both". The header's `Layer` and `Pass` menus were indeed photographed on both machines, but the Image Editor's own `View` menu (`IMAGE_MT_view`) was photographed on the Linux machine alone and stays untried on the Mac. Narrowed.] **Windows is still untried** — no Windows machine has run it — so that system stays open. From the sources: nothing on the passes page or in Cycles' `list_render_passes` is conditioned on an operating system.

**Where I looked.** Manual: searched all `.txt` for `pass` in file paths (one hit, the passes page) and read it whole; read the Render Layers node page, the Composite node page, the Image Editor introduction. API: `bpy.types.RenderPass`, `bpy.types.RenderPasses`, `bpy.types.RenderLayer`, `bpy.types.RenderEngine` (`register_pass`), `bpy_types_enum_items/render_pass_type_items`. Cycles: grepped all `.py`/`.h` for `Combined`, `register_pass`, `list_render_passes`; read `engine.py` lines 165–267 and `properties.py` around the pass enums. Release notes: grepped all for `pass`.

**Sources.**
- Manual, https://docs.blender.org/manual/en/4.5/render/layers/passes.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/editors/image/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/composite.html — read 2026-07-30
- Manual image `_images/render_layers_passes_combine.svg`, referenced from the passes page above — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy_types_enum_items/render_pass_type_items.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.types.RenderPasses.html — read 2026-07-30
- Cycles file `cycles_4.5/engine.py`, lines 177–267 (`list_render_passes`); the quoted lines are 182–183 — read 2026-07-30

---

## 2. Do any of the pictures a render produces show whether a scene's textures loaded, and if so, which?

**Answer.** **Not in the sources** as a pass. Nothing in Blender 4.5's documented pass list reports whether textures loaded, and there is no pass type in the Python API's pass identifier list for it. Every pass documented is about light, geometry, depth, motion, identity or denoising; none is about asset loading.

There is one thing close to it, and it is worth stating exactly because it is easy to over-read. Cycles' own files define a colour to stand in for a texture it could not find, and the shader code returns that colour when the image slot is empty:

```
/* Color to use when textures are not found. */
enum {
  TEX_IMAGE_MISSING_R = 1,
  TEX_IMAGE_MISSING_G = 0,
  TEX_IMAGE_MISSING_B = 1,
  TEX_IMAGE_MISSING_A = 1
};
```

and:

```
ccl_device float4
svm_image_texture(KernelGlobals kg, const int id, const float x, float y, const uint flags)
{
  if (id == -1) {
    return make_float4(
        TEX_IMAGE_MISSING_R, TEX_IMAGE_MISSING_G, TEX_IMAGE_MISSING_B, TEX_IMAGE_MISSING_A);
  }
```

The quoted file tells this function to return red 1, green 0, blue 1, alpha 1 when `id == -1`. Calling that a colour seen in a completed render requires a further step.

That is a substituted *colour*, not a picture that reports loading. From it, a person can tell a texture is missing by seeing magenta where a texture should be — but that is a step beyond what the file says, so I label it separately below. It does not say *which* texture; it says only that whatever is shaded by that node came back missing. And it is Cycles' file only; I have no equivalent file for EEVEE or Workbench.

Separately, and outside rendering, Blender does have a magenta convention that is documented in words — but it belongs to the Video Sequencer's preview, not to a render's passes:

> "Show Missing Media
>
> Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent."

with, just below it, "Strips with missing content will be displayed as red in the timeline." That is about strips in a sequence, not about a scene's textures.

Blender also has two operators for the underlying problem, again reporting in words rather than in a picture: "Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files." And in the Python API, `bpy.ops.file.report_missing_files()` — "Report all missing external files" — and `Image.has_data` — "True if the image data is loaded into memory".

**LABEL: not in the sources** — for any render pass that reports texture loading. **observed** — only for the quoted Cycles file contents: the missing-texture comment and the branch that returns RGBA (1, 0, 1, 1) when `id == -1`. **documented** — for the Sequencer's Show Missing Media option, for Report Missing Files, and for `Image.has_data`. **inferred** — that the quoted Cycles branch makes a missing texture visible as magenta in a completed render, and that it appears in the Combined pass or any colour pass the shader feeds. The step: the file says `id == -1` returns that value, and the passes page says the colour passes carry "The colors of Diffuse and Subsurface Scattering BSDFs, modified by any Mix and Add Shader nodes"; putting those together gives magenta appearing wherever that node's colour lands. The sources never state that result in one place or name a specific pass that will show it.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2. Blender 4.5.12 started from a terminal with no window and driven by a script. Set up: 32 by 32, one still, output OpenEXR, Cycles samples 1; a plainly green 8 by 8 PNG written to disk; the default cube given a material whose only shading is an Emission node fed by an Image Texture node pointing at that file, so the rendered colour is the texture's colour; Workbench's shading colour type set to `TEXTURE`; Metal switched on in the Cycles preferences, which reported `CYCLES DEVICES: [('Apple M1 Pro', 'CPU', True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]`. Done: sixteen renders — four cases in four engines — each written to its own OpenEXR and the centre pixel read back. Blender printed:

```
--- CASE present (image slot='green.png', file exists=True, has_data=False) ---
RESULT present    CYCLES_CPU  centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    CYCLES_GPU  centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    EEVEE       centre RGBA = (-0.0, 1.0, -0.0, 1.0)
RESULT present    WORKBENCH   centre RGBA = (0.0032, 0.1169, 0.0026, 0.9976)
--- CASE missing (image slot='green.png', file exists=False, has_data=False) ---
RESULT missing    CYCLES_CPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    CYCLES_GPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    EEVEE       centre RGBA = (1.0, -0.0, 1.0, 1.0)
RESULT missing    WORKBENCH   centre RGBA = (0.1186, 0.0029, 0.1181, 0.9976)
--- CASE emptyslot (image slot=None, file exists=False, has_data=False) ---
RESULT emptyslot  CYCLES_CPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  CYCLES_GPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  EEVEE       centre RGBA = (0.0, 0.0, 0.0, 1.0)
RESULT emptyslot  WORKBENCH   centre RGBA = (0.0955, 0.0941, 0.0951, 0.9976)
--- CASE corrupt (image slot='green.png', file exists=True, has_data=False) ---
RESULT corrupt    CYCLES_CPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    CYCLES_GPU  centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    EEVEE       centre RGBA = (1.0, -0.0, 1.0, 1.0)
RESULT corrupt    WORKBENCH   centre RGBA = (0.1186, 0.0029, 0.1181, 0.9976)
```

The "corrupt" case was the same file replaced with the bytes `this is not a picture`.

Alongside the renders, these were the only messages Blender printed about the texture, in this order across the run:

```
GPUTexture: Blender Texture Not Loaded!
OpenImageIO exited with a pending error message that was never
retrieved via OIIO::geterror(). This was the error message:
Not a PNG file
OpenImageIO exited with a pending error message that was never
retrieved via OIIO::geterror(). This was the error message:
Not a PNG file
IMB_load_image_from_memory: unknown file-format (/tmp/images-hand-tests/out/t11/4.5.12_LTS/green.png)
GPUTexture: Blender Texture Not Loaded!
```

Nothing was printed for the missing-file case that named the file. The `IMB_load_image_from_memory` line, which does name the file, appeared only for the corrupt file.

**Observed — the same test on a second machine and a different make of graphics card, 2026-08-06.** Run on the rented Linux machine, Blender 4.5.12 LTS, no window. The set-up is the Mac's, with one change: Apple's Metal graphics do not exist here, so the script asks Cycles what it has and then renders on this machine's card twice, once through CUDA and once through OptiX. It reported:

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

Both machines were run. **They agree on the colour in every case and every engine**, and the NVIDIA card gives the same answer both ways it can be used. **One number differs**: Workbench's alpha is `1.0` on this machine and `0.9976` on the Mac, in all four cases. Nothing was read into that.

The messages Blender printed about the texture during the run were:

```
GPUTexture: Blender Texture Not Loaded!
OpenImageIO exited with a pending error message that was never
retrieved via OIIO::geterror(). This was the error message:
Not a PNG file
IMB_load_image_from_memory: unknown file-format (/root/smoozy/Images/work/out/t11/4.5.12_LTS/green.png)
GPUTexture: Blender Texture Not Loaded!
```

As on the Mac, nothing named the file for the missing-file case; only the unreadable file was named.

**Not covered.** Which texture is missing — nothing in any source ties the substituted colour back to a named image. Whether a texture that loaded but failed part-way behaves the same. Whether EEVEE or Workbench substitute the same colour, or any colour. Whether the same substitution applies to non-image textures, to volume grids, or to UDIM tiles. Whether a genuinely magenta texture is distinguishable from a missing one (nothing in the sources addresses that, and on the face of it nothing would distinguish them).

**Outside the interface.** **Observed.** The whole test above was run with no window open, and the substituted colour appeared in the written files just the same. So the substitution holds when Blender is driven from outside its own window. What the test also shows is that the console is where any word of it appears, and in 4.5 the only line that names the file is the one for a corrupt file, not for a missing one. "Report Missing Files" is documented as writing "a warning message ... in the Info editor's header", which is part of Blender's window; the Python equivalent `bpy.ops.file.report_missing_files()` exists, and the sources do not say where its report goes when there is no window — that part was not tested. (Previously, before the test: inferred for the Cycles branch, not observed.)

**Operating system.** **Observed on two machines and on two makes of graphics card, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro) with Apple's own Metal graphics, and Ubuntu 22.04.5 LTS with an NVIDIA GeForce RTX 4090 used both through CUDA and through OptiX.** The substituted colour is identical everywhere. **Windows is still untried** — no Windows machine has run it — and so is any card that is neither Apple's nor NVIDIA's. From the sources: nothing in the Cycles files or the pages above is conditioned on an operating system.

**Where I looked.** Manual, searched all `.txt` for: `magenta`, `pink`, `purple`, `missing image`, `missing texture`, `texture is missing`, `image is missing`, `missing file`, `file is missing`, `could not be found`, `not found`, `broken link`, `broken path`, `placeholder color`, `failed to load`, `could not load`, `load failed`. Read the whole passes page, the Image Texture node page, the Image Settings page, the topbar External Data section, the Sequencer preview sidebar. API, searched for `has_data`, `missing` in `bpy.ops.file`, and read the pass type identifier list end to end looking for anything about assets or loading — there is none. Cycles, grepped every `.py` and `.h` for `magenta`, `missing`, `not found`, `invalid`, then read `source/util/texture.h` and `source/kernel/svm/image.h`. Release notes, grepped all files for `pass` and `image`. Ruled out: the Cryptomatte and Debug pass groups, which I read in full — they are about object/material identity and sample counts, not asset state.

**Sources.**
- Cycles file `cycles_4.5/source/util/texture.h`, lines 11–17 — read 2026-07-30
- Cycles file `cycles_4.5/source/kernel/svm/image.h`, lines 20–26 — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.ops.file.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.types.Image.html — read 2026-07-30

---

## 3. Which file formats can Blender write a render out as?

**Answer.** Two families: still-image formats written directly, and movie formats written through FFmpeg.

**Still images.** The manual gives a table, introduced as "This is the list of image file formats supported internally by Blender:", with columns Format / Channel Depth / Alpha / Metadata / DPI / Extensions:

| Format | Channel Depth | Alpha | Metadata | DPI | Extensions |
|---|---|---|---|---|---|
| BMP | 8bit | ✓ | ✗ | ✓ | .bmp |
| Iris | 8, 16bit | ✓ | ✗ | ✗ | .sgi .rgb .bw |
| PNG | 8, 16bit | ✓ | ✓ | ✓ | .png |
| JPEG | 8bit | ✗ | ✓ | ✓ | .jpg .jpeg |
| JPEG 2000 | 8, 12, 16bit | ✓ | ✗ | ✗ | .jp2 .jp2 .j2c |
| Targa | 8bit | ✓ | ✗ | ✗ | .tga |
| Cineon & DPX | 8, 10, 12, 16bit | ✓ | ✗ | ✗ | .cin .dpx |
| OpenEXR | float 16, 32bit | ✓ | ✓ | ✓ | .exr |
| Radiance HDR | float | ✓ | ✗ | ✗ | .hdr |
| TIFF | 8, 16bit | ✓ | ✗ | ✓ | .tif .tiff |
| WebP | 8bit | ✓ | ✓ | ✗ | .webp |

(The `.jp2 .jp2` repetition is in the source as written.)

The Python API's machine-readable list is slightly longer, because it separates three things the manual's table folds together: Targa Raw from Targa, the multi-layer variant of OpenEXR from plain OpenEXR, and Cineon from DPX (the manual's table gives those two as a single row, "Cineon & DPX"). [Citation check 2026-08-04: this read "two variants"; the manual's table has eleven rows and the API's image list fourteen, so the difference is three, not two. Corrected.] Under the heading "Image" it gives: `BMP`, `IRIS`, `PNG`, `JPEG`, `JPEG2000`, `TARGA`, `TARGA_RAW`, `CINEON`, `DPX`, `OPEN_EXR_MULTILAYER`, `OPEN_EXR`, `HDR`, `TIFF`, `WEBP`. Under the heading "Movie" it gives exactly one entry: `FFMPEG` — "FFmpeg Video." The property that takes these values is described as "File format to save the rendered images as", "enum in Image Type Items, default `'PNG'`".

So `OPEN_EXR_MULTILAYER` — "Output image in multilayer OpenEXR format." — and `TARGA_RAW` — "Output image in uncompressed Targa format." — are separately selectable, and the only movie route in the enum is FFmpeg.

**Movies.** "Blender used FFmpeg to handle video encoding/decoding various video formats. These formats are primarily used for compressing rendered sequences into a playable movie. Video formats are composed of a container, a codec, and sometimes audio which is stored using its own codec."

Containers documented: MPEG-4 (.mp4, .mpg, .mpeg), Matroska (.mkv), WebM (.webm), AVI (.avi), DV (.dv), Flash (.flv), MPEG-1 (.mpg, .mpeg), MPEG-2 (.dvd, .vob, .mpg, .mpeg), Ogg (.ogg, .ogv), QuickTime (.mov).

Video codecs documented: No Video, AV1, H.264, H.265 / HEVC, WEBM / VP9, DNxHD, DV, FFmpeg video codec #1 (FFV1), Flash Video, HuffYUV, MPEG-1, MPEG-2, MPEG-4 (DivX), ProRes, PNG, QuickTime Animation, Theora. The page notes "These options are not available with all Containers."

Audio codecs documented: No Audio, AAC, AC3, FLAC, MP2, MP3, Opus, PCM, Vorbis.

**Which of these a given copy of Blender actually has is not fixed.** The command-line page splits the list in two:

> "-F, --render-format <format>
>
> Set the render format. Valid options are: TGA RAWTGA JPEG IRIS AVIRAW AVIJPEG PNG BMP HDR TIFF.
>
> Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

I report that list as printed. Note that `AVIRAW` and `AVIJPEG` appear there but appear nowhere in the Python API's format list, and that the Python API reports, in the build-options constant on the machine that generated the documentation, `codec_avi=False`.

The Python API confirms per-format build switches, in a constant whose printed value includes:

> "codec_avi=False, codec_ffmpeg=True, ... image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True"

The release notes record three 4.5 changes to what can be written:

> "Images written from the render output can now store meta-data for the pixel density, the aspect ratio into account making this useful for storing the aspect for non-square pixels."
>
> "Rendering with an alpha channel is now supported for the BMP format"
>
> "Video rendering to FFV1 codec now supports 10, 12, and 16 bits output."

**LABEL: documented.**

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, on the Blender 4.5.12 installed there, started from a terminal with no window. Set up: nothing built; a plain start with factory settings. Done: printed the build options, the output-format setting and its choices, whether a Media Type setting exists, and the FFmpeg container and codec choices. Blender printed:

```
VERSION: 4.5.12 LTS
BUILD_OPTIONS: bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=True, jack=False, pulseaudio=False, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, collada=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=False, potrace=True, pugixml=True, haru=True, experimental_features=False)
FILE_FORMAT_DEFAULT: PNG
FILE_FORMAT_ITEMS: ['BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG']
HAS_MEDIA_TYPE: False
HAS_USE_PREVIEW: True
FFMPEG_CONTAINERS: ['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']
FFMPEG_CODECS: ['NONE', 'AV1', 'H264', 'H265', 'WEBM', 'DNXHD', 'DV', 'FFV1', 'FLASH', 'HUFFYUV', 'MPEG1', 'MPEG2', 'MPEG4', 'PNG', 'PRORES', 'QTRLE', 'THEORA']
FFMPEG_AUDIO_CODECS: ['NONE', 'AAC', 'AC3', 'FLAC', 'MP2', 'MP3', 'OPUS', 'PCM', 'VORBIS']
```

Three things follow from that printout and nothing more. This build has every optional image format compiled in. There is no Media Type setting on the output settings in this version, although the release notes announce one. And there is no `AVIF` among the formats.

*Asking for a format by name on the command line.* Set up: nothing built; the factory startup scene shrunk to 64 by 64. Done, in a terminal, once per name:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=64" -F AVIF -o /tmp/images-hand-tests/out/t02/4.5.12_AVIF/out_ -f 1
```

`WEBP` was accepted and wrote `out_0001.webp`, which `file` reported as `RIFF (little-endian) data, Web/P image, VP8 encoding, 64x64`. `OPEN_EXR_MULTILAYER` was accepted and wrote `out_0001.exr`, reported as `OpenEXR image data, version 2`. `AVIF` was refused, with Blender printing `Error: Format from '-F / --render-format' not known or not compiled in this release.` and then writing `out_0001.png` instead. Exit status was 0 in every case.

*Asking for a format by name from Python.* Set up: nothing built. Done: `bpy.context.scene.render.image_settings.file_format = 'AVIF'`. Blender raised:

```
ASSIGNMENT RAISED: TypeError
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "AVIF" not found in ('BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG')
```

That message is the complete list of names this version's output-format setting will take.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 4.5.12 open. Looked at: Properties ▸ Output Properties ▸ Output panel, with the format changed in turn, and the format menu opened.

The control that chooses what is written is labelled **`File Format`**. There is no `Media Type` row anywhere in the panel in this version — the release notes announce one, and it is not here.

Opening that menu shows a two-column list under a heading reading `Image` and a third column under a heading reading `Movie`. Under `Image`: `BMP`, `Iris`, `PNG`, `JPEG`, `JPEG 2000`, `Targa`, `Targa Raw`, `Cineon`, `DPX`, `OpenEXR MultiLayer`, `OpenEXR`, `Radiance HDR`, `TIFF`, `WebP`. Under `Movie`, one entry: `FFmpeg Video`. There is no `AVIF`, and nothing animated of the single-picture kind.

With each format chosen, the panel shows: **PNG** — `Color` (BW / RGB / RGBA), `Color Depth` (8 / 16), `Compression 15%`. **JPEG** — `Color` (BW / RGB), `Quality 90%`. **TIFF** — `Color` (BW / RGB / RGBA), `Color Depth` (8 / 16), `Compression: Deflate`. **OpenEXR** — `Color` (BW / RGB / RGBA), `Color Depth` (Float (Half) / Float (Full)), `Codec: ZIP`, `Preview`. **OpenEXR MultiLayer** — `Color Depth`, `Codec: ZIP`, `Preview`, and no `Color` row. **FFmpeg Video** — `Color` (BW / RGB), then an `Encoding` section and `Video` and `Audio` sections.

With `FFmpeg Video` chosen, the `Encoding` section's `Container` menu was opened, and then the `Video` section's `Video Codec` menu. Every entry, in the order shown:

- **Container**: `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`.
- **Video Codec**: `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`.

The rows under `Video` in this version are `Video Codec`, `Color Depth` (8 / 10), `Output Quality: Medium Quality`, `Encoding Speed: Good`, `Keyframe Interval 18`, `Max B-frames`. `Audio Codec` reads `No Audio`.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 4.5.12 LTS, no window, factory settings. Blender printed:

```
VERSION: 4.5.12 LTS
BUILD_OPTIONS: bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=False, jack=True, pulseaudio=True, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, collada=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)
FILE_FORMAT_DEFAULT: PNG
FILE_FORMAT_ITEMS: ['BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG']
HAS_MEDIA_TYPE: False
HAS_USE_PREVIEW: True
FFMPEG_CONTAINERS: ['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']
FFMPEG_CODECS: ['NONE', 'AV1', 'H264', 'H265', 'WEBM', 'DNXHD', 'DV', 'FFV1', 'FLASH', 'HUFFYUV', 'MPEG1', 'MPEG2', 'MPEG4', 'PNG', 'PRORES', 'QTRLE', 'THEORA']
FFMPEG_AUDIO_CODECS: ['NONE', 'AAC', 'AC3', 'FLAC', 'MP2', 'MP3', 'OPUS', 'PCM', 'VORBIS']
```

Both machines were run and both agreed on everything that bears on this question: every optional image format is compiled in, there is no Media Type setting, there is no AVIF, and the format list and the three FFmpeg lists are identical. Three build options differ between the machines, and none of them is an image format: `coreaudio` is true on the Mac and false here, `jack` and `pulseaudio` are false on the Mac and true here, and `xr_openxr` is false on the Mac and true here.

*Asking for a format by name on the command line.* The same four names were tried on this machine, with the same command and the same results as the Mac: `WEBP` wrote `out_0001.webp`, reported by `file` as `RIFF (little-endian) data, Web/P image, VP8 encoding, 64x64`; `OPEN_EXR_MULTILAYER` wrote `out_0001.exr`, reported as `OpenEXR image data, version 2`; and `AVIF` was refused with `Error: Format from '-F / --render-format' not known or not compiled in this release.` followed by a PNG. The exit status was `0` every time.

*Asking for a format by name from Python.* The same message came back:

```
ASSIGNMENT RAISED: TypeError
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "AVIF" not found in ('BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG')
```

*What the container and codec lists mean.* A further test, recorded in full under question 4, chose each container in turn and read the codec list back: all ten containers offer all seventeen codecs after being chosen, and all 170 pairings can be set. So the lists above are what Blender offers, not what it will accept — several pairings fail at the moment of encoding.

**Not covered.** Which combinations of container and codec are legal — the manual explicitly declines to enumerate them ("These options are not available with all Containers"). A test recorded under question 4 now settles which of them can actually be written on the two machines the project has. What the shipped official builds enable, as against what can be compiled: the build-options constant is a snapshot from whichever machine built the documentation, not a promise about any download. The listing does not settle what Blender can *read*: `image_dds=True` is a build option but DDS appears nowhere as a writable format, and the string `DDS` appears nowhere in the 4.5 manual or API documentation at all.

**Outside the interface.** **Observed.** Everything above was done with no window open: `-F` chose the format from the command line and wrote the file; the setting was read and written from Python in a windowless Blender. Documented alongside it: `-x, --use-extension <bool>` for the extension and `-o, --render-output <path>` for the path. The command-line list of names in the manual is not identical to the Python one, which is a difference worth knowing before scripting against it.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The official build for each system has every optional image format compiled in, and the format lists match exactly. **Windows is still untried** — no Windows machine has run it — so what a build there carries is not settled, and that is the case the manual is warning about. From the sources: "Formats that can be compiled into Blender, not available on all systems", and about movie formats, "Each has advantages and disadvantages, and compatibility with different players on different operating systems." The sources do not say which format is missing on which operating system, and neither machine the project has is missing one.

**Where I looked.** Manual: `files/media/image_formats`, `files/media/video_formats`, `render/output/properties/output`, `advanced/command_line/arguments`; searched all `.txt` for `file format`. API: `bpy_types_enum_items/image_type_items` read end to end, `bpy.types.ImageFormatSettings` read whole, `bpy.app` for `build_options`. Release notes: grepped all files for `format`, `codec`, `EXR`, `JPEG`, `PNG`, `WebP`, `AVI`, `image`; read `rendering.md` whole. Opened the manual's Output panel screenshot to check the panel against the text.

**Sources.**
- Manual, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/files/media/video_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy_types_enum_items/image_type_items.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.types.ImageFormatSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/4.5/rendering/ — read 2026-07-30

---

## 4. What happens when a file asks for a format that Blender cannot write?

**Answer.** **Not in the sources.** No page in the 4.5 manual, no entry in the 4.5 Python API documentation, and no line in the 4.5 release notes says what Blender does when a blend-file (or a command line, or a script) asks for an output format this copy of Blender cannot write. Nothing describes a fallback, a substitution, an error message, a refusal to render, or a non-zero exit status.

What *is* documented is that the situation exists.

> "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

and the Python API exposes the corresponding switches as read-only build facts, in a constant whose printed value includes `codec_avi=False, codec_ffmpeg=True, ... image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True`. The release notes record the newest of these being exposed: "Support for `bpy.app.build_options.image_webp`". Together these sources establish that format availability can vary and that a script can inspect build switches. They do not establish how a blend-file preserves or handles a format that its current build lacks.

Three neighbouring situations *are* described, and none of them is the question asked, so I set them out rather than let them stand in for an answer:

- Container and codec mismatch, where the manual says an error is what you get, without saying what the error is or what happens to the render: "Some containers and codecs are not compatible with each other, so if you are getting errors check that your container and codec are compatible. Like containers and codecs are sometimes not compatible with each other, some codecs do not work with arbitrary dimensions."
- A codec refusing the frame size: "Some codecs impose limitations on output size, H.264, for example requires both the height and width to be divisible by 2." What Blender does when the size violates that is not stated.
- A format not supporting a requested channel layout or depth: "Note, RGBA is not available for all image formats, check the list above for details" and "Note, not all file formats support all color depths." Again, no statement of behaviour.

The one place the 4.5 documentation does describe a render-output request failing is unrelated to format — it is about the output path: "`scene.render.frame_path()` used to be infallible but can now fail if there are errors processing Path Templates in the render output path."

**LABEL: not in the sources** for the behaviour. **documented** for the fact that formats vary by build, for the build-options switches, and for the three neighbouring situations quoted.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 4.5.12, started from a terminal with no window. Three cases were tried.

*A name the command line will not take.* Set up: nothing built; the factory startup scene at 64 by 64, its own output format left at the default PNG. Done, in a terminal:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=64" -F NOSUCHFORMAT -o /tmp/images-hand-tests/out/t02/4.5.12_NOSUCHFORMAT/out_ -f 1
```

Blender printed:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
...
Saved: '/tmp/images-hand-tests/out/t02/4.5.12_NOSUCHFORMAT/out_0001.png'
Time: 00:00.50 (Saving: 00:00.06)
```

The exit status was `0`. One file was written, `out_0001.png`, which `file` reported as `PNG image data, 64 x 64, 8-bit/color RGBA, non-interlaced` — the format already set in the file. The same happened when the name was `AVIF`, a real format that this version does not have: the same error line, then a PNG.

*The same request from Python.* Set up: nothing built. Done: `bpy.context.scene.render.image_settings.file_format = 'NOSUCHFORMAT'`, then a still rendered with `bpy.ops.render.render(animation=False, write_still=True)`. Blender printed:

```
ASSIGNMENT RAISED: TypeError
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "NOSUCHFORMAT" not found in ('BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG')
file_format after attempt: PNG
OPERATOR RESULT: {'FINISHED'}
FILES: ['out_.png']
```

*A request the codec refuses.* Set up: the factory startup scene, output `image_settings.file_format = 'FFMPEG'`, `ffmpeg.format = 'MPEG4'`, `ffmpeg.codec = 'H264'`, resolution 101 by 101, frames 1 to 3. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
Couldn't initialize video codec: Generic error in an external library
Traceback (most recent call last):
  File "/private/tmp/images-hand-tests/scripts/t04_video_size.py", line 24, in <module>
    res = bpy.ops.render.render(animation=True)
...
RuntimeError: Error: width not divisible by 2 (101x101)
```

and, on its own report line, `Error: width not divisible by 2 (101x101)`. The output folder was empty afterwards. The process exit status was `0` — the Python error did not change it, since `--python-exit-code` was not given.

*The case that could not be produced.* The reading the question was written for — a blend-file asking for a real format that this copy of Blender lacks — could not be made to happen on this machine. The build-options line recorded under question 3 shows every optional format compiled in. That case is recorded as not runnable in `Audits/Blender Research/Images/Hand tests.md`, with the reason: it needs a Blender build made without one of those formats, and both machines the project has carry official builds with every one of them. The Linux run of 2026-08-06 checked the second machine and found the same, so the case is not runnable on either.

**Observed — the same three cases on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 4.5.12 LTS, no window. All three were repeated with the same scripts and the same commands.

*A name the command line will not take.* Blender printed:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
Saved: '/root/smoozy/Images/work/out/t02/4.5.12_NOSUCHFORMAT/out_0001.png'
```

The exit status was `0`, one file was written, and `file` reported it as `PNG image data, 64 x 64, 8-bit/color RGBA, non-interlaced`. The same happened for `AVIF`.

*The same request from Python.* The same `TypeError` came back, with the same list of names.

*A request the codec refuses.* Blender printed `Couldn't initialize video codec: Generic error in an external library`, then `RuntimeError: Error: width not divisible by 2 (101x101)`, the output folder was empty, and the process exit status was `0`.

Both machines were run and both agreed, message for message and status for status.

**Observed — a request Blender itself refuses: every container-and-codec pairing the lists offer, 2026-08-06.** This case could not be produced on the Mac run because nobody had tried encoding with a pairing; only the lists had been read. Set up: the factory startup scene at 64 by 48, frames 1 to 3, output `FFMPEG`. Done: each container chosen in turn and its codec list read back; then all 170 pairings assigned; then three frames really encoded for every container with `H264`, and for the `MPEG4` container with every codec. **The same script was run on both machines** — on the rented Linux machine and on the Creator's own MacBook Pro (M1 Pro, macOS 26.5.2) — **and the two agreed on all 104 encodes.**

Nothing is refused at the setting: every container offers the same seventeen codecs once chosen, and Blender printed `PAIRINGS THE SETTING REFUSED: []`.

The encoder refuses a great deal. The wording that answers this question is:

```
RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination
```

It came back for the WebM container with H.264, and for the MPEG-4 container with No Video, FFV1, Flash Video, HuffYUV, ProRes, QuickTime Animation and Theora. In each of those cases a file of zero or a few hundred bytes was left behind — for example `m_0001-0003.mp4 0 bytes`. Three further refusals are about the picture rather than the pairing:

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

**Not covered.** The one reading the question was written for — a real format this build lacks — is still unanswered, and now for a stronger reason: both machines the project has were checked, and the official build for each has every optional format compiled in, so the case cannot be produced on either. It needs a Blender built without one of them. Also still unanswered: whether the behaviour above differs on Windows.

**Outside the interface.** **Observed.** All three cases above were run with no window open, and that is where the answers came from. Two things matter for anything driving Blender from outside: a format name the command line will not take is reported on one line and then ignored, the render carrying on in whatever format the file already held, with an exit status of `0`; and a request the codec refuses stops that render, writes no file, and still leaves the exit status at `0`. Blender does have `--python-exit-code <code>` — "Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables" — which was not given in these runs. (Previously, before the test: not settled either way.)

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The same lines, the same refusals and the same exit statuses came back on both, including every one of the container-and-codec refusals. **Windows is still untried** — no Windows machine has run it — and that gap sits directly on this question, since the manual's own reason for a format being absent is that builds differ between systems. From the sources: availability is not uniform — "not available on all systems" — and they do not say which system lacks which format. What the two machines show is that neither of them lacks one.

**Where I looked.** Manual, searched every `.txt` for: `unsupported`, `not supported`, `cannot`, `unable to`, `will fail`, `fails to`, `cannot save`, `unable to save`, `failed to save`, `error saving`, `saving failed`, `unknown format`, `invalid format`, `unrecognized`, `this build`, `built without`, `compiled with`, `not available in`, `fall back`, `falls back`, `fallback`, `reverts to`, `defaults to PNG`, `falls back to PNG`, `will be used instead`, `instead of the requested`. The only relevant hits were the compile-time note on the command-line page and the codec-compatibility notes quoted above; every other hit was about EEVEE feature limitations, GPU support, or unrelated subjects, and I ruled them out by reading them. Read in full: `files/media/image_formats`, `files/media/video_formats`, `render/output/properties/output`, `render/output/animation`, `advanced/command_line/arguments`, the whole `troubleshooting/` folder listing plus `troubleshooting/startup` and `troubleshooting/crash`. API: same search terms across all `.txt`; read `bpy.types.ImageFormatSettings` and `bpy.app` whole. Release notes: grepped all files for `format`, `codec`, `not available`, `unavailable`, `removed`; read `python_api.md` and `rendering.md` whole. Cycles: read `operators.py` whole (its only error paths concern missing frames and denoiser exceptions, not formats).

**Sources.**
- Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/files/media/video_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30 [Citation check 2026-08-04: added. The quote "Note, not all file formats support all color depths." is on this page, which this list omitted.]
- API, https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/4.5/python_api/ — read 2026-07-30

---

## 5. Can a format hold several pictures inside one file, and if so, how can the finished picture be told apart from the others inside it?

**Answer.** Yes — OpenEXR. The manual says so plainly:

> "ILM's OpenEXR has become a software industry standard for HDR image files, especially because of its flexible and expandable structure.
>
> An OpenEXR file can store multiple layers and passes. This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact."

It is selected as its own format: the Python API's format list has `OPEN_EXR_MULTILAYER` — "OpenEXR MultiLayer. Output image in multilayer OpenEXR format." — alongside plain `OPEN_EXR`. Blender also writes such a file automatically for its own use when asked: "Cache Result — Saves the rendered view layers and their passes to a multi-layer OpenEXR image."

The things inside are told apart **by name**. The API property that sets one is described as "OpenEXR layer name used for this slot", on a type described as "Multilayer slot of the file output node". On reading such a file back, the manual says: "When a multi-layer file format, like EXR, is loaded, each layer is made available as a socket." The Python API exposes the position of the chosen one on the image user: `multilayer_layer` — "Layer in multilayer image"; `multilayer_pass` — "Pass in multilayer image"; `multilayer_view` — "View in multilayer image".

The sources do not establish the exact name of the finished picture inside the file. They establish that layers and passes are named, and Cycles' own file contains this literal pass entry:

```
    # Combined pass.
    yield ("Combined", "RGBA", 'COLOR')
```

The step from that entry to the identifier stored inside a multilayer EXR is an inference. The supplied sources do not show the EXR channel names or state whether the view-layer name is included.

One documented limit on where such a file can be used: "The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor."

Two other multi-picture arrangements exist and are not this. Movie containers hold many pictures — "Matroska: A free open-standard container format, a file format that can hold an unlimited number of video, audio, picture or subtitle tracks in one file" — but the pictures there are frames or tracks, not passes. And multi-view (stereoscopic) output has a `views_format` — "Format of multiview media", "enum in Views Format Multiview Items, default `'INDIVIDUAL'`" — with `multilayer_view` picking among views in a multilayer file.

**LABEL: documented** — that OpenEXR holds several layers and passes in one file, that layers carry names, and that a reader picks among them by name or index. **observed** — only for the quoted Cycles file containing the tuple `("Combined", "RGBA", 'COLOR')`. **inferred** — that this becomes the name used for the finished picture inside a multilayer EXR. The exact on-disk name remains not in the sources.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 4.5.12, started from a terminal with no window.

*A rendered multi-layer OpenEXR.* Set up: 32 by 32, one frame; the first view layer renamed to `Beauty` and a second added called `Second`; on both, `use_pass_combined`, `use_pass_z`, `use_pass_normal`, `use_pass_mist`, `use_pass_diffuse_color` and `use_pass_emit` switched on; `image_settings.file_format = 'OPEN_EXR_MULTILAYER'`. This version has no `use_exr_interleave` setting. The render engine was the factory default, which this version reports as `BLENDER_EEVEE_NEXT`, so the names below are the ones that engine writes. Done: `bpy.ops.render.render(animation=True)`, then the written file's own bytes read with a short header reader (see `Audits/Blender Research/Images/Hand tests.md`). The reader printed:

```
BYTES: 6762
MAGIC: 0x01312f76
VERSION FIELD: 0x00000002 (version 2, tiled=False longnames=False deep=False multipart=False)
--- PART 0 ---
  BlenderMultiChannel      string           'Blender V2.55.1 and newer'
  Camera                   string           'Camera'
  Date                     string           '2026/08/03 22:42:39'
  File                     string           '<untitled>'
  Frame                    string           '1'
  RenderTime               string           '00:01.04'
  Scene                    string           'Scene'
  Time                     string           '00:00:00:01'
  channels                 chlist           Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT); Beauty.Depth.Z(FLOAT); Beauty.DiffCol.B(FLOAT); Beauty.DiffCol.G(FLOAT); Beauty.DiffCol.R(FLOAT); Beauty.Emit.B(FLOAT); Beauty.Emit.G(FLOAT); Beauty.Emit.R(FLOAT); Beauty.Mist.Z(FLOAT); Beauty.Normal.X(FLOAT); Beauty.Normal.Y(FLOAT); Beauty.Normal.Z(FLOAT); Second.Combined.A(FLOAT); Second.Combined.B(FLOAT); Second.Combined.G(FLOAT); Second.Combined.R(FLOAT); Second.Depth.Z(FLOAT); Second.DiffCol.B(FLOAT); Second.DiffCol.G(FLOAT); Second.DiffCol.R(FLOAT); Second.Emit.B(FLOAT); Second.Emit.G(FLOAT); Second.Emit.R(FLOAT); Second.Mist.Z(FLOAT); Second.Normal.X(FLOAT); Second.Normal.Y(FLOAT); Second.Normal.Z(FLOAT)
  compression              compression      3
  dataWindow               box2i            (0, 0, 31, 31)
  displayWindow            box2i            (0, 0, 31, 31)
  lineOrder                lineOrder        0
  pixelAspectRatio         float            1.0
  screenWindowCenter       v2f              (0.0, 0.0)
  screenWindowWidth        float            1.0
  xDensity                 float            72.0
END OF HEADERS at byte 1484
```

So the file is one part, and every picture inside it is a set of channels named `<view layer>.<pass>.<channel>`. The finished picture of the first view layer is `Beauty.Combined.R`, `.G`, `.B`, `.A`. The pass names written into the file are not all the names the manual's list uses: the depth pass is written as `Depth` although the manual calls it `Z`, and the diffuse-colour and emission passes are written as `DiffCol` and `Emit`.

*A multi-layer OpenEXR written by a File Output node.* Set up: the same scene at 32 by 32, main output PNG; a File Output node with `base_path` set to the folder, `format.file_format = 'OPEN_EXR_MULTILAYER'`, its file slots cleared, and two layer slots added, named `MyBeautyLayer` and `second socket`, both fed from the Render Layers node's Image output. Done: the same render call. Blender printed `Saved: '/tmp/images-hand-tests/out/t08/4.5.12_LTS/0001.exr'` — the node's own file is named for the frame number alone — and the header reader printed:

```
VERSION FIELD: 0x00000002 (version 2, tiled=False longnames=False deep=False multipart=False)
  channels                 chlist           MyBeautyLayer.A(FLOAT); MyBeautyLayer.B(FLOAT); MyBeautyLayer.G(FLOAT); MyBeautyLayer.R(FLOAT); second socket.A(FLOAT); second socket.B(FLOAT); second socket.G(FLOAT); second socket.R(FLOAT)
```

So a layer written by that node takes its socket's name, spaces and all, and there is no view-layer part in front of it.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 4.5.12 open and the multi-layer OpenEXR from the test above loaded in the Image Editor. Seen: two dropdown menus in the header. The first, opened, shows `Beauty` and `Second` above a greyed label reading `Layer`. The second, opened, shows `Combined`, `Depth`, `Normal`, `Mist`, `Emit`, `DiffCol` above a greyed label reading `Pass`. Those are the names a person sees, and they match the names inside the file — the pass part of `Beauty.Combined`, `Beauty.Depth`, `Beauty.Mist`, `Beauty.Normal`, `Beauty.Emit`, `Beauty.DiffCol` — with the view-layer part shown as its own menu instead of being spelled out.

**The View menu, not captured on the Mac and now photographed on the second machine.** On the Mac only two menus were photographed, the one labelled `Layer` and the one labelled `Pass`, and no screenshot showed a menu for views. The results below settle it.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with Blender 4.5.12 LTS at `/opt/blender/blender-4.5.12-linux-x64/blender`, no window, driven by the same scripts.

*A rendered multi-layer OpenEXR.* The same file was written and read byte by byte. The reader printed the same version field, `0x00000002 (version 2, tiled=False longnames=False deep=False multipart=False)`, and the same channel list:

```
  channels                 chlist           Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT); Beauty.Depth.Z(FLOAT); Beauty.DiffCol.B(FLOAT); Beauty.DiffCol.G(FLOAT); Beauty.DiffCol.R(FLOAT); Beauty.Emit.B(FLOAT); Beauty.Emit.G(FLOAT); Beauty.Emit.R(FLOAT); Beauty.Mist.Z(FLOAT); Beauty.Normal.X(FLOAT); Beauty.Normal.Y(FLOAT); Beauty.Normal.Z(FLOAT); Second.Combined.A(FLOAT); Second.Combined.B(FLOAT); Second.Combined.G(FLOAT); Second.Combined.R(FLOAT); Second.Depth.Z(FLOAT); Second.DiffCol.B(FLOAT); Second.DiffCol.G(FLOAT); Second.DiffCol.R(FLOAT); Second.Emit.B(FLOAT); Second.Emit.G(FLOAT); Second.Emit.R(FLOAT); Second.Mist.Z(FLOAT); Second.Normal.X(FLOAT); Second.Normal.Y(FLOAT); Second.Normal.Z(FLOAT)
```

*A multi-layer OpenEXR written by a File Output node.* The same file name and the same channels:

```
FILES: ['0001.exr', 'main_0001.png']
  channels                 chlist           MyBeautyLayer.A(FLOAT); MyBeautyLayer.B(FLOAT); MyBeautyLayer.G(FLOAT); MyBeautyLayer.R(FLOAT); second socket.A(FLOAT); second socket.B(FLOAT); second socket.G(FLOAT); second socket.R(FLOAT)
```

Both machines were run and both agreed, name for name.

**Observed — several views inside one file, on the rented Linux machine, 2026-08-06.** A case neither machine had tried before. Set up: 32 by 32, one frame, the depth and normal passes on, `render.use_multiview = True` and `render.views_format = 'STEREO_3D'`, output `OPEN_EXR_MULTILAYER`. Blender reported `VIEWS: [('left', True), ('right', True)]`, and with the image settings' own `views_format` left at its default it wrote **two** files:

```
FILES: ['frame_0001_L.exr', 'frame_0001_R.exr']
```

With `image_settings.views_format = 'MULTIVIEW'` it wrote **one**, and the reader printed:

```
VERSION FIELD: 0x00000002 (version 2, tiled=False longnames=False deep=False multipart=False)
--- PART 0 ---
  channels                 chlist           Beauty.Combined.left.A(FLOAT); Beauty.Combined.left.B(FLOAT); Beauty.Combined.left.G(FLOAT); Beauty.Combined.left.R(FLOAT); Beauty.Combined.right.A(FLOAT); Beauty.Combined.right.B(FLOAT); Beauty.Combined.right.G(FLOAT); Beauty.Combined.right.R(FLOAT); Beauty.Depth.left.Z(FLOAT); Beauty.Depth.right.Z(FLOAT); Beauty.Normal.left.X(FLOAT); Beauty.Normal.left.Y(FLOAT); Beauty.Normal.left.Z(FLOAT); Beauty.Normal.right.X(FLOAT); Beauty.Normal.right.Y(FLOAT); Beauty.Normal.right.Z(FLOAT)
  multiView                stringvector     ['left', 'right']
```

So in this version a view is a third part of the channel name, `<view layer>.<pass>.<view>.<channel>`, and the file also names its views outright in a `multiView` attribute.

**Observed — the window on the rented Linux machine, 2026-08-06.** Blender 4.5.12 was opened with a window on that machine's own display and photographed with `bpy.ops.screen.screenshot`.

With the ordinary multi-layer OpenEXR from the test above loaded, the Image Editor's header holds **two** picture-choosing menus, reading `Beauty` and `Combined`, and no third one. So there is no View menu to photograph for such a file, and the Mac's silence was not a missed picture.

With the multi-view file loaded, a stereo-glasses button appears in the header and the two views are shown together, still with only two menus. Switching that button off — `space.show_stereo_3d = False` — makes a **third** menu appear, after `Layer` and `Pass`, reading `left`. That is the View menu.

**Its contents could not be photographed open.** The pointer was put on it with `xdotool` and Blender drew the control's own tooltip, which reads `Select View.` — so Blender saw the pointer, and that is the control's name. But no button press ever reached Blender. Clicking that menu opened nothing; clicking the Image Editor's own `View` menu, which certainly opens on a click, opened nothing; and clicking the stereo button changed nothing, Blender printing `show_stereo_3d before: False` and then `show_stereo_3d after: False`. Pressing `t` over the picture, which would show the Toolbar, did nothing either. The window was raised, active and holding the keyboard focus each time. Several topics were running windowed Blenders on that one display at the same moment, and a check found the keyboard focus sitting on another one's window between runs; whether a button press would arrive on a quiet display was not tried. What the file itself records is `['left', 'right']`, quoted above.

Separately, the Image Editor's own `View` menu in the header — the one beside `Image` — was opened with `bpy.ops.wm.call_menu(name="IMAGE_MT_view")` and photographed. It holds `Toolbar`, `Sidebar`, `Tool Settings`, `Asset Shelf`, `Adjust Last Operation`, `Update Automatically`, `Show Metadata`, `Frame All`, `Center View to Cursor`, `Zoom`, `Area`, and nothing in it chooses which picture is shown.

**Not covered.** How the layer and pass names are spelled out in the EXR channel names on disk (whether Combined appears as `Combined.R` or `ViewLayer.Combined.R` or otherwise) — no source I was given states the channel-naming scheme; the hand test above settles it by observation instead. Whether other formats in the list can hold several pictures: the manual attributes the ability only to OpenEXR, and says nothing either way about TIFF or JPEG 2000. What happens to the layers if a multilayer file is opened by something that does not understand them.

**Outside the interface.** **Observed.** Both files above were written with no window open, and their names were read off the files themselves rather than out of Blender, so the naming holds for anything driving Blender from outside. `-F OPEN_EXR_MULTILAYER` on the command line was also observed to write such a file (recorded under question 3). What stays window-only is the *picking* of a layer to look at: `multilayer_layer`, `multilayer_pass` and `multilayer_view` are marked "(readonly)" in the API, so the sources offer no way to set which layer is shown from a script — but with the channel names above, a program outside Blender does not need to.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The same channel names, the same single-part shape and the same File Output node file name came back on both. The multi-view results and the window pictures above were got on the Linux machine alone, so they settle that system and leave the Mac untried for those. **Windows is still untried** — no Windows machine has run it. From the sources: OpenEXR is one of the formats "that can be compiled into Blender, not available on all systems", which bears on whether the format is present at all, not on how it behaves once it is; both machines have it.

**Where I looked.** Manual, searched every `.txt` for `multi-layer`, `multilayer`, `multi layer`; read `files/media/image_formats` (OpenEXR section in full), `render/output/properties/output`, `render/output/animation`, `compositing/types/input/image`, `compositing/types/input/scene/render_layers`, `compositing/types/output/file_output`, `editors/image/introduction`. API: same search; read `bpy.types.NodeOutputFileSlotLayer`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.ImageUser`, `bpy.types.ImageFormatSettings`, `bpy_types_enum_items/image_type_items`. Cycles: `engine.py` `list_render_passes`. Ruled out as answering a different question: the movie container pages (frames and tracks, not passes) and the stereoscopy pages (views, not passes) — I read both to check.

**Sources.**
- Manual, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/compositing/types/input/image.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/files/media/video_formats.html — read 2026-07-30 [Citation check 2026-08-04: added. The Matroska quote is on this page, which this list omitted.]
- API, https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotLayer.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.types.ImageUser.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy_types_enum_items/image_type_items.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.types.ImageFormatSettings.html — read 2026-07-30 [Citation check 2026-08-04: added. The `views_format` quote is on this page, which this list omitted.]
- Cycles file `cycles_4.5/engine.py`, lines 182–183 — read 2026-07-30

---

## 6. Does a format that can hold more light than a screen can show also record the treatment it was made under?

**Answer.** I read "a format that can hold more light than a screen can show" as a high-dynamic-range format — in Blender 4.5's list, OpenEXR ("float 16, 32bit") and Radiance HDR ("float"); the manual calls OpenEXR "a software industry standard for HDR image files". I read "the treatment it was made under" as the colour-management treatment applied when the render was made — the view transform, the look, the exposure and gamma.

**Not in the sources.** No page states that Blender records the view transform, look, exposure or gamma inside a saved HDR file. What the sources say points the other way, in three places, and I set them out as facts rather than as a conclusion:

First, what an HDR file holds is the untreated data, not the treated picture:

> "For intermediate files in production, it is recommended to use OpenEXR files. These are always stored in scene linear color spaces, without any data loss. That makes them suitable to store renders that can later be composited, color graded and converted to different output formats."

and, on the save option that decides this:

> "Save As Render
>
> Save image with render color management. For display image formats like PNG, apply view and display transform. For intermediate image formats like OpenEXR, use the default render output color space."

and "Note, Cineon, DPX, OpenEXR, and Radiance HDR image types default to being saved in a linear color space."

Second, when such a file is read back, Blender has to be *told* what it is:

> "However, when loading a render saved to an intermediate OpenEXR file, Blender cannot detect automatically that this is a render (it could be e.g. an image texture or displacement map). We need to specify that this is a render and that we want the transformations applied, with these two settings:"

— those two being "View as Render — Display the image data-block (not only renders) with view transform, exposure, gamma, RGB curves applied. Useful for viewing rendered frames in linear OpenEXR files the same as when rendering them directly" and "Save as Render".

Third, Blender's metadata panel offers a fixed, named Include list, and colour management is not on it. The options are: Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Note. This list does not rule out other fields written by the format itself. The panel carries the warning "Only some image formats support metadata: See image formats", and in that table OpenEXR's Metadata column is ✓ while Radiance HDR's is ✗. The one metadata addition in 4.5 is not colour either: "Images written from the render output can now store meta-data for the pixel density, the aspect ratio into account making this useful for storing the aspect for non-square pixels."

There is one thing an EXR file evidently *can* carry about colour, and it is about the colour space of the numbers, not about the treatment:

> "When opening OpenEXR images, Blender will automatically set the Color Space if it is detected as Linear CIE-XYZ E or ACES2065-1."

That is a statement about reading. The sources do not say whether Blender writes such a marker itself, nor what it writes for any other colour space.

**LABEL: not in the sources** for the question as asked. **documented** for the quoted facts: that OpenEXR renders are stored scene-linear, that Blender must be told a loaded EXR is a render, that the metadata panel offers the listed Include options, that OpenEXR supports metadata and Radiance HDR does not, and that Blender detects two specific colour spaces when opening EXR files. **inferred**, and only weakly, that the treatment is therefore not recorded in a form Blender itself reads back — the step being that if it were recorded and read, the manual would not need to say Blender "cannot detect automatically that this is a render" and would not need two settings to reapply the transform by hand. That step is mine, not the manual's; a file could in principle carry a marker Blender writes and ignores, and nothing I read rules that out.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 4.5.12, started from a terminal with no window.

Set up: 32 by 32, one still each time, everything else left alone. Five files were written, changing one thing at a time: an OpenEXR with `view_settings.view_transform = "AgX"` and `look = "None"`; the same with `view_transform = "Standard"`; the same with `view_transform = "AgX"`, `look = "AgX - Punchy"`, `exposure = 2.0` and `gamma = 0.5`; a Radiance HDR; and a PNG. Blender printed `DEFAULT view_transform='AgX' look='None' exposure=0.0 gamma=1.0`, and `bpy.data has no colorspace in this version`.

Done: each written file's own bytes were read. The OpenEXR made under AgX gave:

```
VERSION FIELD: 0x00000002 (version 2, tiled=False longnames=False deep=False multipart=False)
--- PART 0 ---
  Camera                   string           'Camera'
  Date                     string           '2026/08/03 22:45:37'
  File                     string           '<untitled>'
  Frame                    string           '001'
  RenderTime               string           '00:00.42'
  Scene                    string           'Scene'
  Time                     string           '00:00:00:01'
  channels                 chlist           A(FLOAT); B(FLOAT); G(FLOAT); R(FLOAT)
  compression              compression      3
  dataWindow               box2i            (0, 0, 31, 31)
  displayWindow            box2i            (0, 0, 31, 31)
  lineOrder                lineOrder        0
  pixelAspectRatio         float            1.0
  screenWindowCenter       v2f              (0.0, 0.0)
  screenWindowWidth        float            1.0
  xDensity                 float            72.0
END OF HEADERS at byte 538
```

Setting the Date and RenderTime lines aside, the three OpenEXR headers were the same text: the one made under Standard matched the one made under AgX, and so did the one made with a Look and with exposure and gamma changed. No attribute names a colour space, a view transform, a look, an exposure or a gamma. There is no `chromaticities` attribute and no `colorInteropID` attribute.

The Radiance HDR file begins:

```
#?RADIANCE
FORMAT=32-bit_rle_rgbe

-Y 32 +X 32
```

and then the pixels. Nothing else is in its header.

The PNG's blocks, read in order, were `IHDR, eXIf, oFFs, pHYs, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, IDAT`, and the seven text entries were `File = <untitled>`, `Date = 2026/08/03 22:45:38`, `Time = 00:00:00:01`, `Frame = 001`, `Camera = Camera`, `Scene = Scene`, `RenderTime = 00:00.31`. There is no `sRGB`, `gAMA` or `cHRM` block in this version's PNG.

*Opening the files again.* Done, in a windowless Blender: `bpy.data.images.load(...)` on each file, then its colour space and View-as-Render state printed. Blender printed:

```
OPENED exr_agx.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_standard.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_punchy_exposed.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED radiance.hdr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED display_png.png size=(32, 32) depth=96 is_float=True colorspace='sRGB' use_view_as_render=False
```

All three OpenEXR files come back the same way whatever they were made under, with View as Render off.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 4.5.12 LTS, no window. The same five files were written under the same changed settings, and the same reader was run over each. Blender printed `DEFAULT view_transform='AgX' look='None' exposure=0.0 gamma=1.0` and `bpy.data has no colorspace in this version`.

The three OpenEXR headers came back as the same text apart from the date and the render time, exactly as on the Mac: no attribute names a colour space, a view transform, a look, an exposure or a gamma, and there is no `chromaticities` and no `colorInteropID`. The Radiance file begins `#?RADIANCE` and `FORMAT=32-bit_rle_rgbe` and holds nothing else. The PNG's blocks, read in order, were:

```
PNG display_png.png CHUNKS: IHDR, eXIf, oFFs, pHYs, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, IDAT, IEND
   TEXT File = <untitled>
   TEXT Date = 2026/08/06 06:02:31
   TEXT Time = 00:00:00:01
   TEXT Frame = 001
   TEXT Camera = Camera
   TEXT Scene = Scene
   TEXT RenderTime = 00:00.35
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

**Not covered.** Whether any colour-management information at all is written into the file header by Blender — the hand test above answers it for this version: nothing in the OpenEXR header, nothing in the Radiance HDR header, and nothing in the PNG's blocks names the treatment or the colour space. What Blender writes into an EXR's colour-space attributes when the scene linear space is neither Linear CIE-XYZ E nor ACES2065-1. Whether Radiance HDR carries anything (its Metadata column is ✗, but that column is about Blender's metadata feature, not about the format's own header fields). Whether a movie file records the treatment — I did not treat movies as HDR formats here, and the sources make no such claim for them.

**Outside the interface.** **Observed.** Every file above was written, and every header read, with no window open — the view transform, the look, the exposure and the gamma were all set from a script and the renders driven from a terminal. So the answer holds when Blender is driven from outside its own window: an outside reader cannot recover the treatment from the file, because the file does not carry it. Documented alongside: the saving controls are properties — `color_management` ("Which color management settings to use for file saving"), `view_settings` ("Color management settings applied on image before saving") and `has_linear_colorspace` ("File format expects linear color space"). "View as Render" is described as affecting how the image is *displayed*, and the observed reload above shows it starts off for every one of these files.

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** A file written on either machine carries the same nothing, and reopens the same way. **Windows is still untried** — no Windows machine has run it. From the sources, for displaying HDR there is a real difference: "High Dynamic Range — Enable high dynamic range display in rendered viewport, uncapping display brightness. This requires a monitor with HDR support and a view transform designed for HDR (Filmic does not generate HDR colors). This feature is currently only supported on macOS."

**Where I looked.** Manual: read `render/color_management` in full, `files/media/image_formats` in full, `render/output/properties/metadata` in full, `render/output/properties/output`, `editors/image/image_settings`. Searched every `.txt` in `render/`, `files/` and `editors/image/` for `embed`, `written into the file`, `stored in the file`, `recorded in the file`, `color profile`, `ICC`; searched the whole manual and API for `chromaticit` (ten hits, all describing colour spaces themselves, none about file headers — I read each). Searched the whole manual for `metadata` and read every non-import/export hit. API: read `bpy.types.ImageFormatSettings` and `bpy.types.ColorManagedViewSettings` in full; grepped `bpy.types.RenderSettings` for every `stamp` property and listed them against the manual's Include list — they match, and none concerns colour management. Release notes: grepped all for `format`, `image`, `output`, `preview`; read `rendering.md` in full.

**Sources.**
- Manual, https://docs.blender.org/manual/en/4.5/render/color_management.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.types.ImageFormatSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.types.ColorManagedViewSettings.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/4.5/rendering/ — read 2026-07-30

---

## 7. Can Blender turn a set of finished frames into a short moving picture?

**Answer.** Yes. The manual describes it as the recommended way to work, and gives the steps.

> "Finally you can render out from the Sequencer and compress the frames into a playable movie clip."

The full route, from the Frame Sequence section:

> "The Frame Sequence is a much more stable approach, where you set your output format to a still format (such as JPG, PNG or a multi-layer format). Click Animation to render your scene out to a set of images, where each image is a frame in the sequence.
>
> Blender creates a file for each frame of the animation. You can then use Blender's Compositor to perform any frame manipulation (post-processing). You can then use Blender's Video Sequencer to load that final image sequence, add an audio track to the animation, and render out to an MPEG format to complete your movie."

and the concrete steps, abbreviated to the ones that matter here: "In Blender, open the Video Sequencer. … Choose Add Image from the add menu. Select all the frames from your output folder that you want to include in your animation. They will be added as a strip in the Sequence editor. … In the Output panel, choose the container and codec you want (e.g. MPEG H.264) and configure them. … Click the Animation render button and Blender will render out the Sequence editor output into a movie."

The same advice appears on the Output page: "If you first render out a set of static images such as the default PNG format or the higher-quality OpenEXR (which can retain HDR pixel data), you can combine them as an Image Strip in the Video Sequencer."

Loading numbered frames as one strip is documented: "In the case of (numbered) image sequences (e.g. *-0001.jpg, *-0002.jpg, *-0003.jpg, etc, of any image format), you have a choice: Range … Batch … All". A batch "can be a mix of file types (jpg, png, exr, etc.)". Gaps are handled: "Use Placeholders … makes an image sequence with all files in between even if they are missing. This allows you to render an image sequence with a few frames missing and still the image strip will have the correct range to account for the missing frames displayed as black."

The switch that makes a render come from the Sequencer rather than the camera is a scene setting: "Sequencer — Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Two documented limits. First, one format cannot take this route: "The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor." Second, if the frames and the output differ in size, "the Video Sequencer will try to auto-scale the image to fit it entirely in the output. This may result in clipping."

Not to be confused with this: Blender's animation player, which plays frames back but produces no file — "The animation player is a utility typically used for previewing rendered animations, supporting all image and video formats also supported by Blender. This is a convenient way to play back image sequences at the correct frame rate."

**LABEL: documented.**

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 4.5.12, started from a terminal with no window throughout. **This settles the disagreement between this document and `Preparation/Blender Research/Findings/4.5/Images — second pass.md`, which reads the outside-the-window half differently. The observed result below is what now stands, and the same result is written into that document too.**

*The whole job with no window at all.* Set up, in a windowless Blender: six PNG frames at 64 by 48 rendered from the ordinary scene into a folder; then the 3D scene changed so a fresh render could not be mistaken for those frames — the cube deleted and the world background set to pure blue; then an image strip built from the six saved files with `scene.sequence_editor_create()` and `strips.new_image(name="Frames", filepath=<first file>, channel=1, frame_start=1)` followed by `strip.elements.append(<each remaining file name>)`. Blender printed `STRIP: Frames type: IMAGE elements: 6` and `use_sequencer: True`. Four blend-files were saved from that state: video output with the Sequencer pipeline on, video output with it off, PNG output with it on, PNG output with it off.

Done, in a terminal:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b -noaudio --factory-startup /tmp/images-hand-tests/out/t16/4.5.12_LTS/sequence-test.blend -a
```

Blender printed `Append frame 1` through `Append frame 6`, the exit status was `0`, and one file appeared at the output path: `out_0001-0006.mp4`, which `file` reported as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. The same command on the file with the Sequencer pipeline switched off wrote `nosec_0001-0006.mp4`.

To see which pictures went into it, the same two files were rendered again with PNG output instead of video, and the written frames' pixels read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

With the Sequencer pipeline on, what came out is the saved frames — grey cube on dark grey. With it off, what came out is the changed 3D scene — blue, no cube. The six Sequencer frames took 0.3 seconds in total; the six 3D renders took about that each.

*A folder of frames with nothing prepared.* Done, in a terminal:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b -noaudio --factory-startup --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=48;bpy.context.scene.frame_end=3" -F FFMPEG -o /tmp/images-hand-tests/out/t16/4.5.12_LTS/nostrip/n_ -a
```

Blender printed `Append frame 1`, `Append frame 2`, `Append frame 3` and wrote `n_0001-0003.mkv` — a movie of the 3D scene, not of any folder of pictures. There is no command-line argument that turns a folder of frames into a movie by itself.

*A frame missing from the set at the moment of encoding.* Set up: five frames written, a strip built naming all five, the file saved, and then the third frame's file deleted from disk. Done: the same background animation render, to PNG. All five output frames were written, with nothing printed about the missing one, and their pixels read:

```
s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3215
s_0002.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3215
s_0003.png   size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1175
s_0004.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3215
s_0005.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3215
```

The frame whose file was gone came out fully transparent black.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Set up: three multi-layer OpenEXR frames at 64 by 48 with the depth and normal passes on; a strip built from them; PNG output. Done: the same background animation render. Three files were written, with no message about the format, and their pixels read:

```
s_0001.png   size=(64, 48) centre=(0.2609, 0.2609, 0.2609, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3296
s_0002.png   size=(64, 48) centre=(0.2609, 0.2609, 0.2609, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3296
s_0003.png   size=(64, 48) centre=(0.2609, 0.2609, 0.2609, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3296
```

Compared with the same scene's PNG frames put through the same route, which gave `centre=(0.5373, ...)` and `corner=(0.2, ...)`, the picture is there but darker.

*How long the result is.* Set up: twenty-four frames, a strip built from all of them, video output with the MPEG-4 container and the H.264 codec. Done: encoded once with `render.fps = 24` and once with `render.fps = 12`, then each movie's own header read for the time scale and duration it records:

```
at24_0001-0024.mp4   bytes=2313   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4   bytes=2580   timescale=1000 duration=2000 -> 2.000 s
```

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 4.5.12 open, output set to `FFmpeg Video`. Seen: the `Encoding` section's `Container` menu offers `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`. The `Video` section's `Video Codec` menu offers `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`. So this build really does offer video output, and the container and codec used in the test above are both on those menus.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 4.5.12 LTS at `/opt/blender/blender-4.5.12-linux-x64/blender`, with no window throughout, driven by the same scripts.

*The whole job with no window at all.* The same six frames, the same change to the 3D scene, the same strip built by a script, and then `blender -b -noaudio --factory-startup .../sequence-test.blend -a`. Blender printed `Append frame 1` through `Append frame 6`, the exit status was `0`, and `out_0001-0006.mp4` appeared, which `file` reported as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. Rendered again to PNG, the frames read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

*A folder of frames with nothing prepared.* The same command wrote `n_0001-0003.mkv`, reported as `Matroska data` — a movie of the 3D scene.

*A frame missing from the set.* All five output frames were written, nothing was printed about the missing one, and the third came out fully transparent black:

```
s_0003.png   size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1169
```

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Three files written with no message about the format, `centre=(0.2609, 0.2609, 0.2609, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0)`.

*How long the result is.* The two movies' own headers record:

```
at24_0001-0024.mp4 bytes=2312   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4 bytes=2580   timescale=1000 duration=2000 -> 2.000 s
```

Both machines were run and both agreed on every one of these, to the fourth decimal place, with only the byte sizes of the written files differing by a few bytes.

*Which container and codec will actually take it.* A further test, recorded in full under question 4 and run on both machines, encoded three frames for every container with H.264 and for the MPEG-4 container with every codec. Ten of those pairings are refused with `Error: Could not initialize streams, probably unsupported codec combination`, and three more with a complaint about the picture size. So the two menus offer more pairings than this version will write.

**Not covered.** Nothing in the sources states how long "short" may be; the only length-related fact is "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size." Nothing says whether frames rendered by a different program, or under a different naming scheme, load as one strip — the sequence detection is described in terms of "Blender's frame naming scheme (filename + frame number + .extension)". Nothing states what happens if the frames' colour space differs from the scene's.

**Outside the interface.** **Observed — yes, the whole job.** Both halves were done with no window open: the image strip was built from the finished frames by a script in a windowless Blender, and the movie was encoded by `blender -b <file> -a` from a terminal. The written frames prove the pictures came from the strip and not from a fresh 3D render. So the answer holds when Blender is driven from outside its own window, on this machine. The one thing that does not is the animation player, which the command-line page says is ignored when `-b` is given. (Previously, before the test: the assembly half was left for a fresh research pass rather than answered.)

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The whole job — building the strip with no window and encoding it from a terminal — works on both, and the frames, the file types and the recorded durations match. This is the machine a Smoozy render actually happens on, and FFmpeg is present in its build too (`codec_ffmpeg=True`, recorded under question 3). **Windows is still untried** — no Windows machine has run it. From the sources: the resulting movie's portability is not uniform — "Each has advantages and disadvantages, and compatibility with different players on different operating systems" — and FFmpeg is one of the "Formats that can be compiled into Blender, not available on all systems"; both machines have it.

**Where I looked.** Manual: read `render/output/animation` in full, `render/output/properties/output` in full, `render/output/properties/post_processing` in full, `render/output/animation_player` in full, `video_editing/edit/montage/strips/image` in full; listed the whole `video_editing/` tree and searched it for `image strip` and `add image`. Also `files/media/video_formats`, `files/media/image_formats` (Opening an Image Sequence), `advanced/command_line/arguments`. Release notes: read `sequencer.md` hits and grepped all files for `output` and `pass`.

**Sources.**
- Manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/image.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/render/output/animation_player.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

---

## 8. Can Blender be made to produce a smaller, plainly viewable copy of a picture a render produced?

**Answer.** Yes, through documented resize and save tools, or by combining documented Compositor nodes. The sources also describe a JPEG Preview option, but they do not establish that its copy is smaller.

**The direct one.** Under OpenEXR's output options the manual documents:

> "Preview
>
> When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview."

The Python API carries the same switch, and phrases it as a general image-format property rather than an OpenEXR-only one: `use_preview` — "When rendering animations, save JPG preview images in same directory". This produces a JPEG copy automatically alongside the render. Calling it plainly viewable follows from the source calling it a "quick preview". The source does not state that it has fewer pixels or fewer bytes than the EXR.

Note the discrepancy honestly: the manual documents Preview only inside the OpenEXR section, while the API lists `use_preview` on `ImageFormatSettings`, which serves every format. The sources do not settle whether the option is offered for formats other than OpenEXR.

**Smaller in pixels, by hand, after the fact.** "Resize — Adjusts the image dimensions by scaling its pixel resolution. This is useful for various tasks, such as: Reducing texture resolution to optimize performance and memory usage." with "Size X, Y — Defines the new width and height of the image in pixels." The Python equivalent is `bpy.ops.image.resize(size=(0, 0), all_udims=False)` — "Resize the image".

**Plainly viewable, by hand, after the fact.** "Save As — Save the image to a separate file of any type. The image output settings can be configured and are the same as the Render Output Properties" and "Save a Copy — Save the file under a specified name, but keep the old one open in the Image editor." The Python form takes the colour-management step that makes a linear render viewable: `bpy.ops.image.save_as(save_as_render=..., copy=..., ...)`, where `save_as_render` is "Save As Render, Save image with render color management. For display image formats like PNG, apply view and display transform. For intermediate image formats like OpenEXR, use the default render output color space" and `copy` is "Copy, Create a new image file without modifying the current image in Blender".

**Smaller at render time.** "Percentage — Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image." That makes the render itself smaller rather than making a copy, so it answers a different half of the question.

**Smaller as a second output of the same render.** The Scale node "scales the size of an image", with Space options including "Relative: Percentage values relative to the dimensions of the image input" and "Absolute: Size of an image by using absolute pixel values". The File Output node "writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence. … since this node can be hooked in anywhere in the node tree, it can also save intermediate images automatically", with its own `format` and a `save_as_render` property — "Apply render part of display transform when saving byte image". Combining a Scale node with a File Output node to make a smaller second copy is an inference from those two documented capabilities.

One documented fact worth pairing with all of this: "While animation renders are automatically saved, still renders are not. These have to be saved manually."

**LABEL: documented** — for Preview making a JPEG copy, Resize changing pixel dimensions, Save As writing a chosen type, Percentage changing render size, and the stated Scale and File Output node capabilities. **inferred** — that Resize followed by Save As makes a smaller, plainly viewable copy, and that a Scale node followed by a File Output node makes such a second copy in one render. The sources do not state either combined workflow in one place.

**Ambiguity flagged.** "Smaller" can mean fewer pixels or fewer bytes. I answered both: fewer pixels via Resize, the Percentage slider, or a Scale node; fewer bytes via saving to a lossy format ("Use JPEG for on-screen output where file size is a concern and quality loss is acceptable"). The Preview option gives a JPEG copy, but the sources do not establish that this particular copy is smaller by either measure.

**Observed — hand test, 2026-08-03.** Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 4.5.12, started from a terminal with no window.

*The Preview copy.* Set up: 64 by 48, frames 1 to 3, `image_settings.use_preview = True`, first with `file_format = 'OPEN_EXR'` and then with `'PNG'`. Done: `bpy.ops.render.render(animation=True)` each time, then every written file's byte size and pixel size read. Blender printed:

```
--- OPEN_EXR, use_preview set to True, now reads True ---
  FILE anim_0001.exr                3270 bytes  64x48
  FILE anim_0001.jpg                1134 bytes  64x48
  FILE anim_0002.exr                3270 bytes  64x48
  FILE anim_0002.jpg                1134 bytes  64x48
  FILE anim_0003.exr                3270 bytes  64x48
  FILE anim_0003.jpg                1134 bytes  64x48
--- PNG, use_preview set to True, now reads True ---
  FILE anim_0001.png                3260 bytes  64x48
  FILE anim_0002.png                3260 bytes  64x48
  FILE anim_0003.png                3260 bytes  64x48
```

So with OpenEXR a JPEG copy appears for every frame, beside the main file and with the same stem; it has the same number of pixels and about a third of the bytes. With PNG the setting can be switched on and reads back as on, and no JPEG copy is written.

Then a blend-file was saved with OpenEXR output and Preview on, and one frame rendered from a terminal on the Creator's ordinary configuration (not factory settings):

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b -noaudio /tmp/images-hand-tests/out/t12/4.5.12_LTS/preview-test.blend -f 1
```

Blender printed `Saved: '.../CLI/cli_0001.exr'` and `Saved: '.../CLI/cli_0001.jpg'` — 2074 bytes and 1192 bytes. So the copy is written for a single frame rendered from the command line, as the manual says.

*A smaller second copy made during the render.* Set up: 64 by 48, one frame, main output PNG; a compositor with the Render Layers image going to the Composite node and also through a Scale node with `space = 'RELATIVE'` and X and Y set to 0.5 into a File Output node writing PNG into the same folder. Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
SCALE NODE inputs: ['Image', 'X', 'Y']
SCALE NODE space options: ['RELATIVE', 'ABSOLUTE', 'SCENE_SIZE', 'RENDER_SIZE']
OPERATOR RESULT: {'FINISHED'}
FILE main_0001.png          2726 bytes  64x48  depth=32 is_float=False
FILE small_0001.png         1114 bytes  32x24  depth=32 is_float=False
```

*Resizing a rendered picture and saving a copy.* Set up: one OpenEXR render at 640 by 360, loaded back with `bpy.data.images.load(...)`. Done, in order, with whatever each returned kept:

```
--- bpy.ops.image.resize(size=(320, 180)) ---
RAISED RuntimeError: Operator bpy.ops.image.resize.poll() failed, context is incorrect
--- image.scale(320, 180) ---
scale() returned, size now (320, 180)
--- bpy.ops.image.save_as(filepath='/tmp/images-hand-tests/out/t14/4.5.12_LTS/small.png', save_as_render=True, copy=True) ---
RAISED RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect
small.png exists: False
--- image.save_render('/tmp/images-hand-tests/out/t14/4.5.12_LTS/small_render.png') ---
save_render returned
--- scene format set to PNG, then image.save_render('/tmp/images-hand-tests/out/t14/4.5.12_LTS/small_as_png.png') ---
save_render returned
FILE big.exr               57666 bytes  640x360
FILE small_as_png.png      36546 bytes  320x180
FILE small_render.png      47324 bytes  320x180
```

The two Image Editor commands refuse to run with no window. The two plain calls work. One thing to note about the first `save_render`: although the path ends `.png`, `file` reports that file as `OpenEXR image data, version 2 ... dataWindow: (0 0)-(319 179)`, because `save_render` writes in the scene's own output format. After the scene's format was set to PNG, the second call wrote `PNG image data, 320 x 180, 16-bit/color RGBA, non-interlaced`.

*Whether a saved proxy setup builds during a background run.* Set up: three finished PNG frames at 128 by 96; an image strip built from them; `strip.use_proxy = True` and `strip.proxy.build_25 = True` with the other three sizes off; the sequence editor's proxy storage reading `PER_STRIP`; output PNG; the file saved; no `BL_proxy` folder present. Done, in a terminal: `blender -b .../proxy-test.blend -a`. Blender printed only the three `Saved:` lines for the output frames, the exit status was `0`, and no `BL_proxy` folder existed anywhere under the test folder afterwards.

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer, with Blender 4.5.12 open. Looked at: Properties ▸ Output Properties ▸ Output panel, with the file format changed in turn, watching for a control named `Preview`. Seen: with `PNG`, `JPEG` and `TIFF` there is no `Preview` control anywhere in the panel. With `OpenEXR` a checkbox reading `Preview` appears under `Codec`, unticked. With `OpenEXR MultiLayer` the same `Preview` checkbox appears, unticked. So the manual is right to document Preview under OpenEXR's own options: the interface offers it only there, which matches the terminal result above, where switching `use_preview` on for PNG produced no JPEG copies.

**Observed — the same tests on a second machine, 2026-08-06.** Run on the rented Linux machine, Blender 4.5.12 LTS, no window, driven by the same scripts.

*The Preview copy.* Blender printed:

```
--- OPEN_EXR, use_preview set to True, now reads True ---
  FILE anim_0001.exr                3259 bytes  64x48
  FILE anim_0001.jpg                1135 bytes  64x48
  FILE anim_0002.exr                3259 bytes  64x48
  FILE anim_0002.jpg                1135 bytes  64x48
  FILE anim_0003.exr                3259 bytes  64x48
  FILE anim_0003.jpg                1135 bytes  64x48
--- PNG, use_preview set to True, now reads True ---
  FILE anim_0001.png                3313 bytes  64x48
  FILE anim_0002.png                3313 bytes  64x48
  FILE anim_0003.png                3313 bytes  64x48
```

The single-frame command-line run wrote `cli_0001.exr` at 2078 bytes and `cli_0001.jpg` at 1187 bytes.

*A smaller second copy made during the render.* Blender printed:

```
SCALE NODE inputs: ['Image', 'X', 'Y']
SCALE NODE space options: ['RELATIVE', 'ABSOLUTE', 'SCENE_SIZE', 'RENDER_SIZE']
FILE main_0001.png          2730 bytes  64x48  depth=32 is_float=False
FILE small_0001.png         1111 bytes  32x24  depth=32 is_float=False
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
--- image.save_render('.../small_render.png') ---
save_render returned
--- scene format set to PNG, then image.save_render('.../small_as_png.png') ---
save_render returned
FILE big.exr               57169 bytes  640x360
FILE small_as_png.png      36375 bytes  320x180
FILE small_render.png      47131 bytes  320x180
```

`file` reported `small_render.png` as `OpenEXR image data, version 2 … dataWindow: (0 0)-(319 179)` and `small_as_png.png` as `PNG image data, 320 x 180, 16-bit/color RGBA, non-interlaced`, exactly as on the Mac.

*Whether a saved proxy setup builds during a background run.* Only the three output frames were written, the exit status was `0`, and no `BL_proxy` folder existed anywhere afterwards — `find` returned only the test folder, `frames` and `seq_out`.

Both machines were run and both agreed on every part of this, with only byte sizes differing slightly.

**Not covered.** The quality setting and colour treatment of the JPEG the Preview option writes — the manual says only "a JPEG copy … for a quick preview" and the API says "in same directory". Whether Preview is available for formats other than OpenEXR. Whether Resize can be driven meaningfully without a window (see below). Whether any of this applies to movie output.

**Outside the interface.** **Observed, and it splits three ways.** The Preview copy works from outside: it was written for a three-frame animation and for a single frame started from a terminal. The Scale-plus-File-Output route works from outside: the half-size second file was written by a windowless render. The two Image Editor commands do **not** work from outside: `bpy.ops.image.resize` and `bpy.ops.image.save_as` both refuse with `poll() failed, context is incorrect`, and the plain calls `Image.scale` and `Image.save_render` do the same job instead — with the catch that `save_render` writes in the scene's output format, whatever the path's ending says. Proxies were not built during a background animation render. (Previously, before the test: the node route was inferred, and the two operators were unsettled.)

**Operating system.** **Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The Preview copy, the half-size second file, both refusals, both plain calls and the absence of proxies all came back the same on both. **Windows is still untried** — no Windows machine has run it. From the sources: nothing in the Preview option, the Resize operator, the Save As operator or the Compositor nodes is described as operating-system dependent. The only nearby operating-system statement concerns pasting images into the Image Editor, which is not this: "Windows: Supports pasting images by copying the image's file path. … Linux: Requires Wayland for clipboard image support."

**Where I looked.** Manual: searched every `.txt` for `quick preview` and `JPEG copy` (two hits, one of them the viewport-render page, ruled out by reading); searched `render/output/` and `files/media/` for `Preview`; read `files/media/image_formats` in full, `editors/image/editing` (Save, Save As, Save a Copy, Save All Images, Resize sections), `editors/image/index`, `render/output/properties/format` in full, `compositing/types/transform/scale`, `compositing/types/output/file_output`, `editors/3dview/viewport_render`. API: searched all `.txt` for `use_preview`, `JPG preview`; read `bpy.types.ImageFormatSettings` in full, `bpy.ops.image` (`resize`, `save_as`), `bpy.ops.render` (`render`, `opengl`, `play_rendered_anim`), `bpy.types.CompositorNodeOutputFile`. Opened the manual's Output panel screenshot to see which controls the panel shows with PNG selected — it shows Overwrite, Placeholders, File Extensions, Cache Result, File Format, Color, Color Depth and Compression, and no Preview checkbox, which is consistent with the option being format-dependent but does not by itself prove it is OpenEXR-only. Release notes: grepped all for `preview`; the hits concern asset thumbnails and node previews, not render output, and I read them to rule them out.

**Sources.**
- Manual, https://docs.blender.org/manual/en/4.5/files/media/image_formats.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/editors/image/editing.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/format.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/compositing/types/transform/scale.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — read 2026-07-30
- Manual image `_images/render_output_properties_output_panel.png`, referenced from https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.types.ImageFormatSettings.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.ops.image.html — read 2026-07-30
- API, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30

---

## Gaps for the hand-test stage

Everything this document sent to the hand tests has been run, on the Creator's own MacBook Pro (M1 Pro, macOS 26.5.2), and the results are written in above under the questions they answer. The whole list is at `Audits/Blender Research/Images/Hand tests.md`.

The three checks that needed eyes on Blender's window were run by the Creator on 2026-08-04 and are written in above: the Output panel's control and its Preview box (questions 3 and 8), the Container and Video Codec menus (questions 3 and 7), and the Image Editor's Layer and Pass menus (questions 1 and 5).

Everything on that list was then run again on a second machine — a rented Linux machine, Ubuntu 22.04.5 LTS with an NVIDIA GeForce RTX 4090 — on 2026-08-06, together with the tests the one Mac could never reach. That second list is at `Audits/Blender Research/Images/Hand tests — Linux.md`, and its results are written in above beside the Mac's. **Three of the four gaps below are now closed**, and two questions the Mac left open are answered: the Image Editor's View menu (questions 1 and 5) and a container-and-codec pairing Blender itself refuses (question 4).

What is still open:

1. **Windows.** Every observed answer above was got on macOS on Apple silicon or on Ubuntu on an NVIDIA card, and on both where the test could be repeated. No Windows machine has run it, so that system stays untried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

2. **A build that lacks a format.** (Question 4.) Still cannot be produced. Both machines were checked and the official build for each has every optional format compiled in. It needs a Blender built without one of them.

3. **A graphics card that is neither Apple's nor NVIDIA's.** (Question 2.) The missing-texture result now covers Apple's Metal graphics, an NVIDIA card through CUDA and the same card through OptiX. AMD graphics, Intel graphics and a second card of any make cannot be tried.

4. **Which separate passes carry the missing-texture colour.** (Question 2.) Both machines read the finished picture only.

---

2026-08-03 — Opus 5 (Claude Code) — Images, Blender 4.5

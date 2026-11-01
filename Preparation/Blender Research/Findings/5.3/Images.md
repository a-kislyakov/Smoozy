# Images — Blender 5.3 findings

- **Sources consulted, and date read.** Blender 5.3 manual, Blender 5.3 Python API documentation and Blender 5.3 release notes, all from the copies taken on 2026-10-11 (date read: 2026-10-11). Cycles' own files for 5.3, taken on 2026-10-16 (date read: 2026-10-16). Only the 5.3 folders were used; the folders for other versions were not opened.
- `I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

Disclosure, so nothing is hidden. Before I started, the tool I run in put two things into my context without my opening them: the project's `CLAUDE.md` (whose only content for me was an instruction to Blender researchers to ignore it) and a one-line-per-entry index of the user's saved notes. Neither says anything about how Blender handles images, and nothing below draws on them. I also checked, with a yes/no existence test and without listing or opening anything, that the answer file did not already exist and that its folder did. I created an empty scratch folder in `/private/tmp/claude-501/smoozy-research-5.3/Images/` and did not use it. I did not run Blender. I started no helpers. Every "observed" finding below comes from reading Cycles' own files, not from running anything.

The Blender manual is by the Blender Documentation Team and is under CC-BY-SA 4.0. Quotes from it are attributed by their web address.

How the labels are used: one label is given per question. Where an answer mixes kinds, each claim is also marked **documented**, **observed** or **inferred** where it is made.

---

## 1. What kinds of picture can a render produce, and how can the finished picture be told apart from the others?

**Label: documented** for the kinds of picture. How the finished picture is told apart is partly documented, partly observed, partly inferred, and partly not in the sources.

**Answer.**

A render produces a set of separate images called passes, for each view layer. The manual lists these kinds:

- Data passes: Combined, Depth, Mist, Position, Normal, Vector, UV, Grease Pencil, and Denoising Data (Noisy Image, Denoising Albedo, Denoising Specular Albedo, Denoising Normal, Denoising Roughness, Denoising Depth).
- Index passes: Object Index and Material Index.
- Debug passes: Render Time and Sample Count.
- Light passes. Cycles has Diffuse, Glossy and Transmission (each split into Direct, Indirect and Color), Volume Direct and Indirect, Emission, Environment, Ambient Occlusion and Shadow Catcher. EEVEE has Diffuse Light and Color, Specular Light and Color, Volume Light, Emission, Environment, Shadow, Ambient Occlusion and Transparent.
- Cryptomatte passes: Object, Material and Asset.
- Shader AOVs, which are custom passes.
- Light Groups (Cycles only).

A scene can also have several view layers, each giving its own set of passes. A stereo or multi-view render gives one image per view.

**Observed:** Cycles' own file gives the exact pass names, channels and kinds it registers. It also has passes that the manual page does not list: Volume Scatter, Volume Transmit, Volume Majorant, Noisy Shadow Catcher, Denoising Backward Motion, Denoising Specular Motion, and three guiding passes that exist only in debug builds. The file names the sample-count pass "Debug Sample Count", not "Sample Count".

How to tell the finished picture apart:

- **Among the passes, the full image is the one named "Combined".** The manual calls it "The render output before any compositing is applied". **Observed:** Cycles always registers it, first, under the fixed name `"Combined"` with channels RGBA. It sits outside every on/off condition in the pass list.
- **Light Group passes also start with "Combined" but are not the full image.** **Observed:** they are named `"Combined_<group name>"`. The manual calls one "a limited Combined render pass where the scene is only illuminated by certain lights". **Inferred:** only the exact name "Combined" is the full image.
- **When denoising is on, the Combined pass is the denoised image.** The manual says Noisy Image is "The combined render result before any denoising is applied". **Inferred:** the step is that, if Noisy Image is the image before denoising, Combined is the image after it.
- **When compositing is on, the finished picture is what the compositor sends out, not the Combined pass.** The compositor's Group Output node "defines the image result that is sent to the renderer or to image output after rendering". Only the first image input on that node counts. The Combined pass is what goes into the compositor by default.
- **When the Sequencer is on, the finished picture is the Sequencer's output.** It replaces the camera view, and a Scene strip in it carries the compositor's output.
- **Pass names do not change with the interface language.** **Observed:** Cycles wraps pass names in a function that the API documentation calls "a no-op function that will only mark the string to extract, but not perform the actual translation". **Inferred:** so the names Cycles registers are the English ones shown in its file, whatever language the interface uses.
- **The 5.3 compositor works differently from what the pages below describe.** The 5.3 release notes say the compositor "now uses an effects stack instead of a single compositor node group". The manual pages quoted below describe one node tree with one Group Output. They do not say how the final image is picked when several effects are stacked.

**Evidence.**

- Manual, https://docs.blender.org/manual/en/5.3/render/layers/passes.html (read 2026-10-11):
  - "A Pass is a type of intermediate rendering information that’s extracted as a separate image."
  - Combined: "The render output before any compositing is applied."
  - "Noisy Image – The combined render result before any denoising is applied."
  - "A Light Group provides a limited Combined render pass where the scene is only illuminated by certain lights."
  - The picture `_images/render_layers_passes_combine.svg` on that page is documented. It shows (Diffuse Direct + Diffuse Indirect) × Diffuse Color, the same for Glossy and Transmission, plus Emission, Environment, and Volume Direct + Volume Indirect, all added into "Combined".
- Manual, https://docs.blender.org/manual/en/5.3/render/layers/introduction.html (read 2026-10-11): "View Layers allow you to separate a render into multiple layers, which can then be composited together."
- Manual, https://docs.blender.org/manual/en/5.3/compositing/introduction.html (read 2026-10-11):
  - "By default, the Image socket contains the rendered image, equivalent to the Combined render pass."
  - "By default, the Image output is used as the final composited image for rendering."
- Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/group_output.html (read 2026-10-11):
  - "In the Compositor, this node defines the image result that is sent to the renderer or to image output after rendering."
  - "In the Compositor, only the first image input is used as the final render result."
- Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/post_processing.html (read 2026-10-11):
  - Compositing: "Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."
  - Sequencer: "Renders the output of the Video Sequence editor, instead of the view from the 3D scene’s active camera." and "If Compositing is also enabled, the Scene strip will be the output of the Compositor."
- Manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/render_layers.html (read 2026-10-11): Image output: "The combined rendered image."
- Release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/ (read 2026-10-11):
  - "The compositor now uses an effects stack instead of a single compositor node group."
  - "The default compositor node tree now uses a simple **Group Input** connected to a **Group Output** with no **Render Layers** node."
- API, https://docs.blender.org/api/5.3/bpy.app.translations.html (read 2026-10-11), on `pgettext_n`: "This is a no-op function that will only mark the string to extract, but not perform the actual translation."
- Cycles' own file `engine.py`, function `list_render_passes`, lines 183–282 (read 2026-10-16). Observed:
  - line 185: `from bpy.app.translations import pgettext_n as n_`
  - lines 189–190: `# Combined pass.` / `yield ("Combined", "RGBA", 'COLOR')`
  - lines 217–219: `yield (n_("Volume Scatter"), …`, `yield (n_("Volume Transmit"), …`, `yield (n_("Volume Majorant"),        "Z",    'VALUE')`
  - line 228: `yield (n_("Debug Sample Count"), "X", 'VALUE')`
  - line 251: `yield (n_("Noisy Image"), "RGBA", 'COLOR')`
  - line 276: `yield ("Combined_%s" % lightgroup.name, "RGB", 'COLOR')`

**Where I looked.** The passes, view layer, view layer introduction, compositing introduction, Group Output, File Output, Render Layers node, Post Processing and Image Editor pages. The API pages RenderResult, RenderLayer, RenderPass, RenderEngine, SceneCompositorEffect and SceneCompositorEffects. All 5.3 release notes. Cycles' `engine.py`. I searched the manual and API for "Composite", "composited" and "Render Result" to find how the composited image is named inside the render result. Nothing names it.

**Not covered.** *(Hand-tested 2026-10-18, below: the composited picture's name in a multi-layer file, and the part names EEVEE writes, were measured. Workbench's pass names were not read.)*
- How the composited picture is named or located in the render result, or in a saved file, next to the Combined pass.
- The exact pass names EEVEE and Workbench register. Only Cycles' files are available.
- How the 5.3 effects stack picks the final image when it holds more than one effect.

**Outside the interface.**
- **Observed, 2026-10-18 (entries 4 and 5 below): it holds with no window.** The passes were switched on as view-layer settings from a script, the renders were started with no window, and the compositor's result was what reached the single output file; in a multi-layer file it was written as its own part, `Composite.Combined`. Choosing which picture to look at is done in the Image Editor's header menus, photographed with the window open (entry 40); outside the window the choice is made by what is written to the file. What follows is what the sources said before the test.
- Passes are view-layer settings, and they are exposed to scripts through the API (for example `RenderLayer.use_pass_combined`, "Deliver full combined RGBA buffer").
- Choosing a render slot, view layer or pass to look at in the Image Editor is interface-only viewing.
- One documented difference: "Render Single Layer … Only render the active view layer." The same page notes: "This option is ignored when rendering from the command line." (https://docs.blender.org/manual/en/5.3/render/layers/view_layer.html, read 2026-10-11)
- Otherwise the sources do not say that the passes differ outside the interface.

**Operating system.**
- **Observed, 2026-10-18, on macOS 26.6.2 on Apple silicon (the Creator's Mac) only.** No Linux or Windows machine ran 5.3 in this sitting, so those stay untried. What follows is what the sources said before the test.
- The sources do not say that passes differ by operating system.
- The Render Time pass depends on the device. The manual says it "is not supported on GPU rendering". **Observed:** `engine.py` lines 230–232 register it only `if scene.cycles.device == 'CPU'`.
- Cycles' own files were taken from a macOS arm64 build: the compiled shader headers name `lib/macos_arm64`. So the observed lines describe that build.

### Hand-test results — 2026-10-18

**Label for everything in this section: observed.** Run on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`, started from a terminal with no window (`-b --factory-startup -noaudio`), with its temporary folder and settings folders pointed at a work folder of its own, and driven by a script. The test list, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`; the entry numbers below are that list's.

*Two view layers, one plain single-image output (entry 4).* Set up: the factory startup scene at 32 × 32, one frame, output PNG; the world's background colour set to pure red; a second view layer added, called `Second`, with the collection holding the cube excluded from it (that collection also holds the camera and the light). Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
ENGINE: BLENDER_EEVEE | use_compositing: True | use_sequencer: True
Second: excluded collection Collection holding ['Camera', 'Cube', 'Light']
VIEW LAYERS: ['ViewLayer', 'Second']
CASE A_two_layers OPERATOR RESULT: {'FINISHED'}
CASE A_two_layers FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.8078, 0.5451, 0.5333, 1.0) corner=(0.8549, 0.2196, 0.1255, 1.0)
```

One file appeared, not one per view layer. Its corner carries the red world and its centre the lit cube.

*What lands in the output file when the compositor is on (entry 4).* Set up: the same scene with one view layer; a node group of type `CompositorNodeTree` with a colour output socket (and a colour input socket), holding a Group Output fed by a `CompositorNodeRGB` node set to pure blue. Done twice, each with the same render call. First, the group put into the 5.3 effects stack with `scene.compositor_effects.new("BlueEffect")` and the effect's `node_group` set to it. Second, the stack cleared with `scene.compositor_effects.clear()` and the group assigned to the older setting `scene.compositing_node_group`, which printed `DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0`. Blender printed:

```
RGB NODE TYPE USED: CompositorNodeRGB
NODES: ['NodeGroupInput', 'NodeGroupOutput', 'CompositorNodeRGB']
EFFECTS: [('BlueEffect', 'BlueComp', True)]
compositing_node_group: <bpy_struct, CompositorNodeTree("BlueComp") at 0xbac4e9808>
CASE B1_effects_stack OPERATOR RESULT: {'FINISHED'}
CASE B1_effects_stack FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
EFFECTS: ['Scene Effect']
compositing_node_group: BlueComp
CASE B2_node_group_setting OPERATOR RESULT: {'FINISHED'}
CASE B2_node_group_setting FILES: ['frame_0001.png']
FILE frame_0001.png size=32x32 centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

Centre and corner are the compositor's blue both times. So the single file written to the output path holds the compositor's result, not the Combined pass. Two things the run printed about the two routes: with the group in the effects stack, the older setting read back the same group; and assigning the older setting with the stack empty left the stack holding one effect named `Scene Effect`.

*Picking the picture in the Image Editor, with the window open (entry 40).* Recorded in full under question 5. With a multi-layer OpenEXR loaded, the Image Editor's header holds two menus: the first, labelled `Layer` inside, offers the view layers (`Beauty`, `Second`); the second, labelled `Pass`, offers the passes (`Combined`, `Depth`, `Normal`, `Mist`, `Emission`, `Diffuse Color`). The finished picture is picked by choosing `Combined` in the second. A third menu, labelled `View`, appears only for a file with several views and only with the stereo button off.

*Cycles' debug passes as written (entry 38).* Recorded in full under question 10: a multi-layer OpenEXR from Cycles on the processor carries channels named `ViewLayer.Debug Sample Count.X` and `ViewLayer.Render Time.X`; the same render on the Metal graphics carries the first and not the second.

*What the finished picture is called once written (entry 5).* Recorded in full under question 5. With no compositor, a multi-layer OpenEXR names the first view layer's finished picture `Beauty.Combined` (the view layer having been renamed `Beauty`). With a compositor in the effects stack, the same file gains a first part named `Composite.Combined` holding the composited picture — blue at the centre, `[0.0, 0.0, 0.2461, 1.0]`, where `Beauty.Combined` holds `[0.246, 0.246, 0.2461, 1.0]` — and with Interleave on, channels named `Composite.Combined.R`, `.G`, `.B` and `.A`.

---

## 2. Do any of the pictures a render produces show whether a scene's textures loaded, and if so, which?

**Label: not in the sources** in the documentation. Some parts are observed in Cycles' own files or inferred from them, and are marked where they appear.

**Answer.**

- **No pass is documented as showing whether textures loaded.** None of the passes on the passes page is described that way.
- **Observed: Cycles can put a stand-in colour where an image cannot be found.**
  - Cycles defines magenta, (1, 0, 1) with alpha 1, as the "Color to use when images are not found".
  - Image Texture and Environment Texture nodes do not use that fixed colour directly. Each node carries its own "missing" colour and "missing alpha" inputs. The value put into those inputs is set by code that is not in the folder.
  - In the compiled OSL version of the Image Texture shader, that input defaults to black (0 0 0) with alpha 1.
  - So the files do not settle which colour a missing scene texture shows in a Cycles render.
- **Observed: the image lookup in Cycles has four distinct outcomes:**
  - No image at all: it returns the node's missing colour.
  - A streamed tile that failed to load: it returns the missing colour.
  - A tile that has not loaded yet: it returns the texture's average colour.
  - An image record with no pixel data: it returns zero, meaning black with alpha 0.
- **Inferred: where such a colour would show.** If a texture fails to load, whatever colour Cycles substitutes flows on like any texture colour. So it would show in the Combined pass. It would also show in the passes that hold surface colour without lighting, if the texture feeds a surface colour:
  - Diffuse Color, Glossy Color and Transmission Color: "The colors of Diffuse and Subsurface Scattering BSDFs…"
  - Denoising Albedo: "The diffuse base color of surfaces without lighting or shading".

  The steps are: the missing value replaces the texture's output, and those passes record the surface colour. Whether the result can be told apart from a real texture depends on the substituted colour, which the files do not settle. A texture that has not finished streaming would show its average colour, which cannot be told apart from a loaded texture at a glance.
- **The only documented case where missing images show as magenta is the Video Sequencer's own media, not scene textures.**
  - The "Show Missing Media" option will "Render missing images/movies with a solid magenta color". When it is off, "missing content will render fully transparent". Its API default is off.
  - Separately, in an image-sequence strip with placeholders, "Placeholder frames are rendered in pink in the VSE preview".

**Evidence.**

- Manual, https://docs.blender.org/manual/en/5.3/render/layers/passes.html (read 2026-10-11):
  - Diffuse Color: "The colors of Diffuse and Subsurface Scattering BSDFs, modified by any Mix and Add Shader nodes."
  - "Denoising Albedo – The diffuse base color of surfaces without lighting or shading."
- Manual, https://docs.blender.org/manual/en/5.3/editors/video_sequencer/preview/sidebar.html (read 2026-10-11): "Show Missing Media … Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent."
- API, https://docs.blender.org/api/5.3/bpy.types.SequenceEditor.html (read 2026-10-11): `show_missing_media` — "Render missing images/movies with a solid magenta color (default False)".
- Manual, https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/image.html (read 2026-10-11): "Placeholder frames are rendered in pink in the VSE preview."
- Cycles' own files (read 2026-10-16). Observed:
  - `source/util/types_image.h`, lines 17–19: `/* Color to use when images are not found. */` / `#define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)` / `#define IMAGE_MISSING_RGB make_float3(1, 0, 1)`
  - `source/kernel/svm/image.h`, lines 69–75, in the Image Texture node: `const float3 missing = stack_load(stack, node.missing);` / `const float missing_alpha = stack_load(stack, node.missing_alpha);` … `svm_image_texture(kg, sd, node.id, tex_co, node.flags, make_float4(missing, missing_alpha));`
  - `source/kernel/svm/types.h`, lines 30–31: `/* Encodes a node float input, as either float3 value or a stack offset` / ` * encoded in a NaN bit pattern in the x component. */`. So the missing colour is a node input, either fixed or wired.
  - `source/kernel/util/image.h`, function `kernel_image_interp`, lines 336–411:
    - line 342–343: `if (image_texture_id == KERNEL_IMAGE_NONE) {` / `return missing_rgba;`
    - line 361: `return (tile_descriptor == KERNEL_TILE_LOAD_FAILED) ? missing_rgba : tex.average_color;`
    - lines 377–378: `if (UNLIKELY(!info->data)) {` / `return zero_float4();`
  - `source/kernel/device/gpu/image.h`, line 121: the same line as `image.h` line 361, for the GPU path.
  - `shader/node_image_texture.oso`, lines 12–13: `param	color	Missing	0 0 0` and `param	float	MissingAlpha	1`.

**Where I looked.**
- Manual pages: passes, Image Texture node, Video Sequencer sidebar, image strip, and Report/Find Missing Files on the top bar page.
- Searches of the manual, API and release notes for "missing", "pink", "magenta", "purple", "not found", "missing texture", "missing image" and "missing color".
- In Cycles' files: a search for "missing" across every file. I read `types_image.h`, `kernel/util/image.h`, `kernel/util/image_3d.h`, `kernel/svm/image.h`, `kernel/svm/node_types.h` and `kernel/svm/types.h`, and the compiled image and environment texture shaders. I checked the Python files (`properties.py`, `ui.py`, `engine.py` and others): none mentions "missing".
- The manual says nothing about what a scene texture that failed to load looks like in a render.

**Not covered.** *(Hand-tested 2026-10-18, below: the colour Cycles, EEVEE and Workbench show for a missing texture, and the lines a render prints about it, were measured.)*
- Which missing colour Blender hands to Cycles' texture nodes.
- What EEVEE and Workbench show for a missing texture.
- Any report or log of failed texture loads. That would not be a picture anyway.

**Outside the interface.**
- **Observed, 2026-10-18 (entries 8 and 9 below): it holds with no window.** Every render below was run with no window open, and the substituted colour appeared in the written files — in the finished picture and in the colour passes — and a background render printed lines naming the missing or unreadable file. The Video Sequencer's "Show Missing Media" was not tested. What follows is what the sources said before the test.
- The sources do not say.
- The Cycles code read above has nothing tied to the interface. But the code that sets the missing colour is not available, so no difference can be ruled out.
- The Sequencer setting is an API property, so a script can switch it on.

**Operating system.**
- **Observed, 2026-10-18, on macOS 26.6.2 on Apple silicon (the Creator's Mac) only, with Apple's own Metal graphics.** Cycles on the processor and on Metal gave the same numbers. No Linux or Windows machine, and no NVIDIA, AMD or Intel card, ran 5.3 in this sitting. What follows is what the sources said before the test.
- The sources do not say.
- **Observed:** the CPU and GPU lookup paths contain the same failed-tile line (`kernel/util/image.h` line 361 and `kernel/device/gpu/image.h` line 121).
- The Cycles files come from a macOS arm64 build.

### Hand-test results — 2026-10-18

**Label for everything in this section: observed.** Run on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`, started from a terminal with no window (`-b --factory-startup -noaudio`), with its temporary folder and settings folders pointed at a work folder of its own, and driven by a script. The test list, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`; the entry numbers below are that list's.

*A missing texture in four cases and four engines (entry 8).* Set up: 32 × 32, one still, output OpenEXR, Cycles samples 1; a plainly green 8 × 8 PNG written to disk as `green.png`; the default cube given a material whose only shading is an Emission node fed by an Image Texture node pointing at that file, so the rendered colour is the texture's colour; Workbench's shading colour type set to `TEXTURE`; Metal switched on in Cycles' device settings. Four cases: **present**; **missing** (the file moved to another folder); **emptyslot** (the node's image set to nothing); **corrupt** (the file replaced with the bytes `this is not a picture`). Done: sixteen renders — four cases in four engines, Cycles on the processor, Cycles on the Metal graphics, EEVEE and Workbench — each written to its own OpenEXR and its centre pixel read back. Blender printed:

```
CYCLES DEVICES: [('Apple M1 Pro', 'CPU', True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]
--- CASE present (image slot='green.png', file exists=True, has_data=False) ---
RESULT present    CYCLES_CPU   centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    CYCLES_GPU   centre RGBA = (0.0, 0.999, 0.0, 1.0)
RESULT present    EEVEE        centre RGBA = (0.0, 1.0, 0.0, 1.0)
RESULT present    WORKBENCH    centre RGBA = (0.0032, 0.1169, 0.0026, 0.998)
--- CASE missing (image slot='green.png', file exists=False, has_data=True) ---
RESULT missing    CYCLES_CPU   centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    CYCLES_GPU   centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT missing    EEVEE        centre RGBA = (1.0, 0.0, 1.0, 1.0)
RESULT missing    WORKBENCH    centre RGBA = (0.1186, 0.0029, 0.1181, 0.998)
--- CASE emptyslot (image slot=None, file exists=True, has_data=False) ---
RESULT emptyslot  CYCLES_CPU   centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  CYCLES_GPU   centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT emptyslot  EEVEE        centre RGBA = (0.0, 0.0, 0.0, 1.0)
RESULT emptyslot  WORKBENCH    centre RGBA = (0.0955, 0.0941, 0.0951, 0.998)
--- CASE corrupt (image slot='green.png', file exists=True, has_data=False) ---
RESULT corrupt    CYCLES_CPU   centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    CYCLES_GPU   centre RGBA = (0.9994, 0.0, 0.9984, 1.0)
RESULT corrupt    EEVEE        centre RGBA = (1.0, 0.0, 1.0, 1.0)
RESULT corrupt    WORKBENCH    centre RGBA = (0.1186, 0.0029, 0.1181, 0.998)
```

(The `has_data` figure in each heading line was read before the image was reloaded for that case, so the `True` on the missing line is left over from the present case's renders.) Cycles on the processor and on the Metal graphics gave the same numbers in every case. EEVEE and Workbench substitute the same magenta as Cycles for a missing and for an unreadable file; for an empty image slot, Cycles still shows magenta, EEVEE black and Workbench plain grey.

Blender's own lines from the same run, every one that names the picture or a failure, in the order printed:

```
00:01.479  cycles           | WARNING Image file /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e08/green.png does not exist.
00:01.480  cycles           | ERROR Failed to load 1 image files
00:01.560  cycles           | WARNING Image file /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e08/green.png does not exist.
00:01.647  cycles           | ERROR Failed to load 1 image files
00:01.910  image.gpu        | ERROR Image file "/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e08/green.png" does not exist
00:03.174  cycles           | WARNING Image file /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e08/green.png failed to load.
00:03.174  cycles           | WARNING Image file /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e08/green.png failed to load.
00:03.174  cycles           | ERROR Failed to load 1 image files
00:03.254  cycles           | WARNING Image file /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e08/green.png failed to load.
00:03.255  cycles           | WARNING Image file /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e08/green.png failed to load.
00:03.342  cycles           | ERROR Failed to load 1 image files
00:03.490  image.read       | ERROR IMB_load_image_from_memory: unknown file-format (/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e08/green.png)
00:03.923  image.read       | ERROR IMB_load_image_from_memory: unknown file-format (/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e08/green.png)
```

and, after Blender's closing version line:

```
OpenImageIO exited with a pending error message that was never
retrieved via OIIO::geterror(). This was the error message:
Not a PNG file
```

twice. The first five lines are the missing case (Cycles on the processor, Cycles on the graphics, then EEVEE); the rest are the corrupt case. **The empty-slot case printed no line at all.** A missing file is a `WARNING` naming the file followed by an `ERROR` counting it; EEVEE's line is an `ERROR` under `image.gpu` naming the file in quotation marks.

*Which separate passes show a missing texture (entry 9).* No earlier version read the passes. Set up: a plane of size 2 filling an orthographic camera (orthographic width 2, camera at height 5 looking straight down), lit by one sun of strength 3; the plane's material a Principled BSDF whose Base Color is an Image Texture on the green `green.png`; 32 × 32, one frame, output multi-layer OpenEXR; the Diffuse Color pass on; for Cycles, on the processor, 4 samples, denoising on and Denoising Data on (`denoising_store_passes`). Blender printed `use_denoising: True | denoising_store_passes: True | diffuse color pass: True`. Done: rendered in Cycles and in EEVEE with the file present, then with it moved away; the centre pixel of every part read with the `OpenEXR` module that ships inside this Blender. Blender printed:

```
present CYCLES        part 'ViewLayer.Combined'         channel 'ViewLayer.Combined'           centre [0.1559, 1.1213, 0.156, 1.0]
present CYCLES        part 'ViewLayer.Diffuse Color'    channel 'ViewLayer.Diffuse Color'      centre [0.0, 0.9626, 0.0]
present CYCLES        part 'ViewLayer.Noisy Image'      channel 'ViewLayer.Noisy Image'        centre [0.1561, 1.1239, 0.1561, 1.0]
present CYCLES        part 'ViewLayer.Denoising Albedo' channel 'ViewLayer.Denoising Albedo'   centre [0.0374, 1.0, 0.0374]
present CYCLES        part 'ViewLayer.Denoising Specular Albedo' channel 'ViewLayer.Denoising Specular Albedo' centre [0.0, 0.0, 0.0]
present CYCLES        part 'ViewLayer.Denoising Normal' channel 'ViewLayer.Denoising Normal.X' centre [0.0]
present CYCLES        part 'ViewLayer.Denoising Normal' channel 'ViewLayer.Denoising Normal.Y' centre [0.0]
present CYCLES        part 'ViewLayer.Denoising Normal' channel 'ViewLayer.Denoising Normal.Z' centre [-1.0]
present CYCLES        part 'ViewLayer.Denoising Roughness' channel 'ViewLayer.Denoising Roughness.X' centre [0.9478]
present CYCLES        part 'ViewLayer.Denoising Depth'  channel 'ViewLayer.Denoising Depth.Z'  centre [5.0]
present BLENDER_EEVEE part 'ViewLayer.Combined'         channel 'ViewLayer.Combined'           centre [0.153, 1.1211, 0.153, 1.0]
present BLENDER_EEVEE part 'ViewLayer.Diffuse Color'    channel 'ViewLayer.Diffuse Color'      centre [0.0, 0.9629, 0.0]
file exists now: False
missing CYCLES        part 'ViewLayer.Combined'         channel 'ViewLayer.Combined'           centre [1.1197, 0.1562, 1.1192, 1.0]
missing CYCLES        part 'ViewLayer.Diffuse Color'    channel 'ViewLayer.Diffuse Color'      centre [0.9626, 0.0, 0.9626]
missing CYCLES        part 'ViewLayer.Noisy Image'      channel 'ViewLayer.Noisy Image'        centre [1.1241, 0.1562, 1.1241, 1.0]
missing CYCLES        part 'ViewLayer.Denoising Albedo' channel 'ViewLayer.Denoising Albedo'   centre [1.0, 0.0374, 1.0]
missing CYCLES        part 'ViewLayer.Denoising Specular Albedo' channel 'ViewLayer.Denoising Specular Albedo' centre [0.0, 0.0, 0.0]
missing CYCLES        part 'ViewLayer.Denoising Normal' channel 'ViewLayer.Denoising Normal.X' centre [0.0]
missing CYCLES        part 'ViewLayer.Denoising Normal' channel 'ViewLayer.Denoising Normal.Y' centre [0.0]
missing CYCLES        part 'ViewLayer.Denoising Normal' channel 'ViewLayer.Denoising Normal.Z' centre [-1.0]
missing CYCLES        part 'ViewLayer.Denoising Roughness' channel 'ViewLayer.Denoising Roughness.X' centre [0.9725]
missing CYCLES        part 'ViewLayer.Denoising Depth'  channel 'ViewLayer.Denoising Depth.Z'  centre [5.0]
missing BLENDER_EEVEE part 'ViewLayer.Combined'         channel 'ViewLayer.Combined'           centre [1.1211, 0.153, 1.1201, 1.0]
missing BLENDER_EEVEE part 'ViewLayer.Diffuse Color'    channel 'ViewLayer.Diffuse Color'      centre [0.9624, 0.0, 0.9624]
```

and, for the missing case only, `cycles | WARNING Image file …/green.png does not exist.`, `cycles | ERROR Failed to load 1 image files` and `image.gpu | ERROR Image file "…/green.png" does not exist`.

So with the texture feeding Base Color: in Cycles the Combined, Diffuse Color, Noisy Image and Denoising Albedo parts swap green for magenta, and in EEVEE the Combined and Diffuse Color parts do. The Denoising Specular Albedo, Denoising Normal and Denoising Depth parts are the same present and missing; Denoising Roughness moved from 0.9478 to 0.9725.

---

## 3. Which file formats can Blender write a render out as?

**Label: documented.** Some parts are inferred, and are marked where they appear.

**Answer.**

Still-image formats, from the API's list of file formats, each described as "Output image in … format":

| Format | Extension | Notes |
| --- | --- | --- |
| AVIF | .avif | |
| JPEG | .jpg | |
| OpenEXR | .exr | |
| PNG | .png | The default |
| WebP | .webp | |
| BMP | .bmp | |
| Cineon | .cin | |
| DPX | .dpx | |
| Iris | .rgb | |
| JPEG 2000 | .jp2 | Codec choice JP2 or J2K |
| Radiance HDR | .hdr | |
| Targa | .tga | |
| Targa Raw | .tga | |
| TIFF | .tif | |
| OpenEXR MultiLayer | .exr | |

The same API list also offers "FFmpeg Video".

The render output's **Media Type** can be Image, Multi-Layer EXR or Video.

Video goes through FFmpeg:

- **Containers:** MPEG-4, Matroska, WebM, AVI, DV, Flash, MPEG-1, MPEG-2, Ogg and QuickTime. The API default is Matroska.
- **Video codecs:**
  - No Video
  - AV1
  - H.264 (the API default)
  - H.265/HEVC
  - WebM/VP9
  - DNxHD
  - DV
  - FFmpeg video codec #1
  - Flash Video
  - HuffYUV
  - MPEG-1
  - MPEG-2
  - MPEG-4 (DivX)
  - PNG
  - ProRes
  - QuickTime Animation
  - Theora
- **Audio codecs:** AAC, AC3, FLAC, MP2, MP3, Opus, PCM and Vorbis.

The manual's format table gives these bit depths for the image formats:

| Format | Bit depth |
| --- | --- |
| JPEG | 8 |
| OpenEXR | float 16 or 32 |
| PNG | 8 or 16 |
| WebP | 8 |
| AVIF | 8, 10 or 12 |
| BMP | 8 |
| Cineon | 8, 10, 12 or 16 |
| DPX | 8, 10, 12 or 16 |
| Iris | 8 or 16 |
| JPEG 2000 | 8, 12 or 16 |
| Radiance HDR | float |
| Targa | 8 |
| Targa Raw | 8 |
| TIFF | 8 or 16 |

Some formats can be left out of a build. The command-line page splits the formats in two:

- **Always valid:** "TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF".
- **"Formats that can be compiled into Blender, not available on all systems":** "OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP".

The command-line names differ from the API names: TGA against TARGA, and JP2 against JPEG2000. AVIF is in neither command-line list, though it is in the API list and the manual's table.

The API documentation shows a sample of `bpy.app.build_options`, presumably from the build that produced the documentation. In it, `image_cineon`, `image_dds`, `image_hdr`, `image_openexr`, `image_openjpeg`, `image_tiff`, `image_webp` and `codec_ffmpeg` are True, and `codec_avi` is False. **Inferred:** whether a format is present is a property of each build, and these values speak only for the build that produced them. `image_dds` appears among the build options, but DDS is not in the list of formats Blender writes. So the sources do not show Blender writing DDS.

Stereo and multi-view output:

- Any image output can put a stereo pair into a single file ("Stereo 3D – Single file with an encoded stereo pair").
- "Multi-view native support is only available to OpenEXR."

5.3 release notes on writing:

- "Saving sRGB image files now includes an ICC profile."
- "Improved reading and writing of multilayer OpenEXR files with lossy DWA compression."

**Evidence.**
- API, https://docs.blender.org/api/5.3/bpy_types_enum_items/image_type_all_items.html (read 2026-10-11): the list above, for example "AVIF: AVIF (.avif). Output image in AVIF format." and "FFMPEG: FFmpeg Video."
- API, https://docs.blender.org/api/5.3/bpy.types.ImageFormatSettings.html (read 2026-10-11):
  - `file_format` "File format to save the rendered images as (default 'PNG')"
  - `media_type` "The type of media to save (default 'IMAGE')", `Literal['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`
  - `jpeg2k_codec` `Literal['JP2', 'J2K']`
- API, https://docs.blender.org/api/5.3/bpy.types.FFmpegSettings.html (read 2026-10-11):
  - `format` "Output file container (default 'MKV')", `Literal['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']`
  - `codec` "FFmpeg codec to use for video output (default 'H264')", with the codec list above
- Manual, https://docs.blender.org/manual/en/5.3/files/media/image_formats.html (read 2026-10-11): "This is the list of image file formats supported internally by Blender", with the table. In the plain-text copy the table cells follow the column order Format, Channel Depth, Alpha, Metadata, DPI, Extensions. I read the cells in that order.
- Manual, https://docs.blender.org/manual/en/5.3/files/media/video_formats.html (read 2026-10-11): "Blender uses FFmpeg to handle video encoding and decoding." The containers, codecs and audio codecs are listed there.
- Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html (read 2026-10-11), Media Type:
  - "Image: Saves each input as a separate image file."
  - "Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file."
  - "Video: Encodes each frame into a video container."
- Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html (read 2026-10-11): "-F, --render-format <format> … Valid options are: TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF. Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."
- API, https://docs.blender.org/api/5.3/bpy.app.html (read 2026-10-11): "bpy.app.build_options … Constant value bpy.app.build_options(… codec_avi=False, codec_ffmpeg=True, … image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, …)"
- API, https://docs.blender.org/api/5.3/bpy_types_enum_items/views_format_multiview_items.html (read 2026-10-11): "STEREO_3D: Stereo 3D. Single file with an encoded stereo pair." and "MULTIVIEW: Multi-View. Single file with all the views."
- Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/stereoscopy/usage.html (read 2026-10-11): "Multi-view native support is only available to OpenEXR."
- Release notes, https://developer.blender.org/docs/release_notes/5.3/pipeline_io/ (read 2026-10-11): "Saving sRGB image files now includes an ICC profile." and "Improved reading and writing of multilayer OpenEXR files with lossy DWA compression."

**Where I looked.** The image formats, video formats, Output properties, command-line arguments, command-line rendering and stereoscopy pages. The API pages ImageFormatSettings, FFmpegSettings, the image-type list, the views-format lists, RenderSettings and `bpy.app`. All 5.3 release notes. I also searched for "GIF", "animated WebP" and "APNG"; none of these is offered as an output.

**Not covered.** *(Hand-tested 2026-10-18, below: on the Mac's 5.3 build every optional format is present and `-F AVIF` is accepted. Other systems' downloads stay unread.)*
- Which optional formats are present in each official 5.3 download for each operating system.
- Whether `-F AVIF` is accepted on the command line.
- The `bpy.app.build_options` sample is from one unnamed build.

**Outside the interface.**
- **Observed, 2026-10-18 (entries 1 and 2 below): it holds with no window.** The formats were read and set from Python in a windowless Blender, and `-F` chose the format from the command line and wrote it, AVIF included. Media Type has to be set before a multi-layer or video format can be asked for by name. With the window open (entry 41), the Output panel offers the same formats and video choices. What follows is what the sources said before the test.
- The format can be set from the command line with `-F`. The manual says this will "Override the image format specified in the blend-file" (https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html, read 2026-10-11).
- It can also be set from a script through `ImageFormatSettings.file_format`.
- One documented difference: inside the interface, still renders are "not saved automatically". From the command line, `-f` will "Render frame <frame> and save it."

**Operating system.**
- **Observed, 2026-10-18, on macOS 26.6.2 on Apple silicon (the Creator's Mac) only.** The 5.3.0 Beta build there has every optional image format compiled in, and AVIF works from the command line. No Linux or Windows machine ran 5.3 in this sitting, so what the builds for those systems carry is untried. What follows is what the sources said before the test.
- The command-line page says the optional formats are "not available on all systems". It does not name the systems.
- For video: "Because codecs are required for both encoding and decoding, they must be available on both the system creating the file and the device playing it."
- The sources do not otherwise tie formats to operating systems.

### Hand-test results — 2026-10-18

**Label for everything in this section: observed.** Run on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`, started from a terminal with no window (`-b --factory-startup -noaudio`), with its temporary folder and settings folders pointed at a work folder of its own. Driven by a script. The test list, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`; the entry numbers below are that list's.

*What the build offers (entry 1).* Set up: nothing; the factory startup scene. Done: printed the build options, the file-format setting's default and choices, the Media Type setting's default and choices, and the FFmpeg container, video-codec and audio-codec choices. Blender printed:

```
VERSION: 5.3.0 Beta HASH: 5d8151335b6b
BUILD_OPTIONS: bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=True, jack=False, pulseaudio=False, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_spz=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, opentimelineio=True, potrace=True, pugixml=True, haru=True, experimental_features=False)
FILE_FORMAT_DEFAULT: PNG
FILE_FORMAT_ITEMS (static list): ['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
HAS_MEDIA_TYPE: True
MEDIA_TYPE_DEFAULT: IMAGE
MEDIA_TYPE_ITEMS: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']
HAS_USE_PREVIEW: True
FFMPEG_CONTAINERS: ['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']
FFMPEG_CODECS: ['NONE', 'AV1', 'H264', 'H265', 'WEBM', 'DNXHD', 'DV', 'FFV1', 'FLASH', 'HUFFYUV', 'MPEG1', 'MPEG2', 'MPEG4', 'PNG', 'PRORES', 'QTRLE', 'THEORA']
FFMPEG_AUDIO_CODECS: ['NONE', 'AAC', 'AC3', 'FLAC', 'MP2', 'MP3', 'OPUS', 'PCM', 'VORBIS']
```

Every optional image format reads `True` in this build, `codec_ffmpeg=True` and `codec_avi=False`.

*Media Type comes first (entry 1).* With Media Type at `IMAGE`, asking for multi-layer OpenEXR by name was refused, and after Media Type was set to `MULTI_LAYER_IMAGE` the same request went through:

```
ASSIGN OPEN_EXR_MULTILAYER: RAISED TypeError
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
file_format after attempt: PNG
--- media_type set to MULTI_LAYER_IMAGE ; file_format reads OPEN_EXR_MULTILAYER
ASSIGN OPEN_EXR_MULTILAYER: OK; file_format is now: OPEN_EXR_MULTILAYER
--- media_type set to IMAGE ; file_format reads PNG
```

*AVIF from Python (entry 1).* With Media Type back at `IMAGE`, `file_format = 'AVIF'` was accepted (`ASSIGN AVIF: OK; file_format is now: AVIF`), and a 64 × 64 still rendered with `bpy.ops.render.render(write_still=True)` printed `Saved: '…/out/e01/avif/out_.avif'` and `OPERATOR RESULT: {'FINISHED'}`. `file` reported the written file as `ISO Media, AVIF Image`.

*Formats asked for by name on the command line (entry 2).* Set up: the factory startup scene at 64 × 64, its own format left at PNG. Done, in a terminal, once for each of `AVIF`, `WEBP` and `OPEN_EXR_MULTILAYER`, each into its own folder:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=64" -F AVIF -o <folder>/out_ -f 1
```

All three exited with status `0` and each wrote its own format. Blender printed `Saved: '…/AVIF/out_0001.avif'`, `Saved: '…/WEBP/out_0001.webp'` and `Saved: '…/OPEN_EXR_MULTILAYER/out_0001.exr'`, and `file` reported them as `ISO Media, AVIF Image` (768 bytes), `RIFF (little-endian) data, Web/P image` (700 bytes) and `OpenEXR image data, version 2, storage: scanline, compression: zip, dataWindow: (0 0)-(63 63), displayWindow: (0 0)-(63 63), lineOrder: increasing y` (4380 bytes). So `-F AVIF` is accepted, although the manual's command-line page lists AVIF in neither of its two lists.

*The Output panel, with Blender's window open (entry 41).* Run on the same Mac in the same Blender, with its window open on the Mac's own display, started from a terminal with `--factory-startup -noaudio --enable-event-simulate --window-geometry 0 0 1600 1000` and a plain saved start file, and driven by a timed script: the first-run "Quick Setup" box closed with a simulated Escape key, the largest area turned into the Properties editor on its Output tab, the window redrawn before each picture, and pictures taken with Blender's own `bpy.ops.screen.screenshot`. Menus were opened by simulated mouse clicks, and the panel scrolled with simulated mouse-wheel turns. The format was set from the script, one at a time, with whatever colour and depth settings the format before it had left.

Seen in the pictures. The panel's heading reads `Output` beside a ticked checkbox. Under the output path come `Saving` with `File Extensions` (ticked) and `Cache Result`, then a row labelled **`Media Type`** and, below it for still images, a row labelled **`File Format`**, whose entries carry their extensions: `PNG (.png)`, `JPEG (.jpg)`, `TIFF (.tif)`, `OpenEXR (.exr)`, `AVIF (.avif)`. With each format chosen the panel shows:

- **PNG** — `Color` (BW / RGB / RGBA), `Color Depth` (8 / 16), `Compression 15%`.
- **JPEG** — `Color` (BW / RGB), `Quality 90%`.
- **TIFF** — `Color` (BW / RGB / RGBA), `Color Depth` (8 / 16), `Compression` reading `Deflate`.
- **OpenEXR** — `Color` (BW / RGB / RGBA), `Color Depth` (Float (Half) / Float (Full)), `Codec` reading `ZIP`, and a `Preview` checkbox, unticked.
- **AVIF** — `Color` (BW / RGB / RGBA), `Color Depth` (8 / 10 / 12), `Quality 90%`.

each followed by `Image Sequence` with `Overwrite` (ticked) and `Placeholders`. With `Media Type` at `Multi-Layer EXR` the `File Format` row is gone, leaving `Color Depth` (Float (Half) / Float (Full)), `Codec` `ZIP`, an `Interleave` checkbox and a `Preview` checkbox, both unticked. With `Media Type` at `Video` the panel shows `Color` (BW / RGB) and collapsed sub-panels `Color Management`, `Pixel Density` and `Encoding`.

The `Media Type` menu, clicked open, offers `Image`, `Multi-Layer EXR`, `Video`. With the `Encoding` sub-panel opened by a click on its header, it shows `Container` (reading `Matroska`) and `Autosplit Output`, then a `Video` sub-panel with `Video Codec` (`H.264`), `Color Depth` (8 / 10), `Color Space` (`sRGB`, greyed), `Output Quality` (`Medium Quality`), `Encoding Speed` (`Good`), `Keyframe Interval` (18) and `Max B-frames`, and an `Audio` sub-panel with `Audio Codec` (`No Audio`). The `Container` menu, clicked open, lists in order `MPEG-4`, `Matroska`, `WebM`, a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`. The `Video Codec` menu, clicked open, lists `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`.

*The video lists, as Blender holds them (entry 3).* Every one of the ten containers offers all seventeen codecs once chosen, and all 170 pairings can be set; eleven of twenty-six real encodes are then refused by the encoder. The full record is under question 4. So the lists above are what Blender offers, not what it will write.

---

## 4. What happens when a file asks for a format that Blender cannot write?

**Label: not in the sources** for what Blender does when a build lacks a requested format (reading a). For requests a chosen format cannot hold (reading b), some outcomes (clamping, metadata loss, errors) are documented or inferred.

**The question is ambiguous.** It could mean:

- **(a)** A .blend file whose saved output format is one that this build of Blender lacks.
- **(b)** A request for something the chosen format cannot hold, such as an alpha channel in JPEG, a codec in a container that does not take it, or a bit depth the format lacks.

I answer both.

**Answer.**

**Reading (a).**
- The manual confirms that some formats are optional at build time: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."
- The API exposes per-build flags such as `image_openexr` and `codec_ffmpeg` in `bpy.app.build_options`.
- The sources do not say what Blender does when a .blend asks for a missing format. They do not say whether it refuses, reports an error, falls back to another format, or does something else.
- They also do not say what happens when `-F` is given a name that is not available.

**Reading (b).** The sources document limits and two outcomes, but not one general rule:
- Limits:
  - "Note, RGBA is not available for all image formats".
  - "Note, not all file formats support every color depth configuration."
  - Float (Half) and Float (Full) are "Only supported for OpenEXR files".
  - "Not all codecs are available in all containers."
  - Several containers "enforce" their codec: DV, Flash, MPEG-1 and MPEG-2.
  - "some codecs do not work with arbitrary dimensions". "H.264 requires both width and height to be divisible by 2."
- First outcome, errors: "Some containers and codecs are not compatible with each other, so if you are getting errors check that your container and codec are compatible." **Inferred:** a container and codec that do not fit can end in an error. The manual does not say when or how the error is shown.
- Second outcome, clamping: "CRF values outside the allowed range are clamped to the nearest allowed value."
- Other documented losses rather than failures:
  - "When using formats that do not support metadata, render information is not preserved in the saved file."
  - "DPX as well as Cineon only stores and converts the “visible” color range of values between 0.0 and 1.0".

**Evidence.**
- Manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html (read 2026-10-11): as quoted above.
- API, https://docs.blender.org/api/5.3/bpy.app.html (read 2026-10-11): `bpy.app.build_options`, as quoted in answer 3.
- Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html (read 2026-10-11):
  - "Note, RGBA is not available for all image formats, check the list above for details."
  - "Some containers and codecs are not compatible with each other, so if you are getting errors check that your container and codec are compatible."
  - "Note, not all file formats support every color depth configuration."
  - "CRF values outside the allowed range are clamped to the nearest allowed value."
- Manual, https://docs.blender.org/manual/en/5.3/files/media/video_formats.html (read 2026-10-11):
  - "Not all codecs are available in all containers."
  - "This container enforces the DV video codec and stores audio uncompressed."
  - "H.264 requires both width and height to be divisible by 2."
- Manual, https://docs.blender.org/manual/en/5.3/files/media/image_formats.html (read 2026-10-11):
  - "Only supported for OpenEXR files."
  - "When using formats that do not support metadata, render information is not preserved in the saved file."
  - "DPX as well as Cineon only stores and converts the “visible” color range of values between 0.0 and 1.0 (as a result of rendering or composite)."

**Where I looked.**
- Manual pages: image formats, video formats, Output properties, Rendering Animations, command-line arguments and command-line rendering, and File Output node.
- API pages: ImageFormatSettings, RenderSettings, FFmpegSettings, `bpy.app`, and the `bpy.ops.render.render` entry. The operator entry reads "Undocumented, consider contributing."
- All 5.3 release notes.
- Searches of the manual (render, files, compositing, advanced, image editor, troubleshooting and video editing sections) for: "not available on all", "unsupported", "not supported", "compiled into", "compiled with", "built without", "build option", "fall back", "falls back", "fallback", "cannot write", "unable to write", "failed to write", "could not write", "unable to save", "cannot save", "error", "invalid", "unknown" and "warning". Nothing describes what happens when a requested format is missing from the build.

**Not covered.** *(Hand-tested 2026-10-18, below: an unknown name given to `-F` was measured. Reading (a) could not be produced.)*
- The actual behaviour in reading (a): message, fallback or refusal, whether the render still runs, and whether any file is written.
- The behaviour when `-F` is given an unavailable or unknown name.

**Outside the interface.**
- **Observed, 2026-10-18 (entries 1, 2 and 3 below), with no window.** An unknown name given to `-F` is reported on one line and ignored: the render carries on in the format the file already holds, and the exit status is `0`. The same name set from Python raises an error naming every accepted format, and the setting keeps its old value. What a container and codec that do not fit do is recorded below; started from the window (entry 43), the same refusal shows its words in the status bar. The case the question was written for — a real format missing from the build — could not be produced. What follows is what the sources said before the test.
- The sources do not say.
- `-F` overrides the format stored in the .blend. What happens when that override names a missing format is not documented.

**Operating system.**
- **Observed, 2026-10-18, on macOS 26.6.2 on Apple silicon (the Creator's Mac) only.** The 5.3 build there lacks no optional format, so the situation the question asks about does not arise on it. Linux and Windows were not run in this sitting. What follows is what the sources said before the test.
- "not available on all systems". The sources do not say which systems lack which formats.

### Hand-test results — 2026-10-18

**Label for everything in this section: observed.** Run on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`, started from a terminal with no window (`-b --factory-startup -noaudio`), with its temporary folder and settings folders pointed at a work folder of its own. The test list, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`; the entry numbers below are that list's.

*An unknown format name from Python (entry 1).* Set up: the factory startup scene, Media Type at `IMAGE`. Done: `file_format = 'NOSUCHFORMAT'`. Blender printed:

```
ASSIGN NOSUCHFORMAT: RAISED TypeError
ASSIGNMENT MESSAGE: bpy_struct: item.attr = val: enum "NOSUCHFORMAT" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
file_format after attempt: PNG
```

Later in the same run, with the format already set to `AVIF`, the same assignment was refused with the same message and `file_format after attempt: AVIF`; a still then rendered with `bpy.ops.render.render(write_still=True)` returned `{'FINISHED'}` and wrote `out_.avif`, which `file` reported as `ISO Media, AVIF Image`. So the setting keeps whatever format it already held.

*An unknown format name on the command line (entry 2).* Set up: the factory startup scene at 64 × 64, its own format left at the default PNG. Done, in a terminal:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy;bpy.context.scene.render.resolution_x=64;bpy.context.scene.render.resolution_y=64" -F NOSUCHFORMAT -o <folder>/out_ -f 1
```

Blender printed, uncut except for the sample-count lines:

```
Error: Format from '-F / --render-format' not known or not compiled in this release.
00:00.330  render           | Rendering single frame
00:00.330  render           | Rendering frame 1
00:00.330  render           | Start rendering: Scene, ViewLayer
00:00.330  render           | Engine: EEVEE
…
00:00.997  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e02/NOSUCHFORMAT/out_0001.png'
00:00.997  render           | Time: 00:00.66 (Saving: 00:00.07)
```

The exit status was `0`. One file was written, `out_0001.png`, which `file` reported as `PNG image data, 64 x 64, 8-bit/color RGBA, non-interlaced` — the format already set in the scene. The error line is a plain line, not in the timestamped logging form.

*A container and codec that do not fit — reading (b) (entry 3).* Set up: the factory startup scene, Media Type `VIDEO`, format `FFMPEG`. Done, in one windowless run: (a) each of the ten containers chosen in turn and the codec list read back; (b) all 170 container-and-codec pairings assigned; (c) at 64 × 48, frames 1 to 3, three frames encoded with `bpy.ops.render.render(animation=True)` for every container with `H264`, and for the `MPEG4` container with every codec — 26 encodes; (d) at 101 × 101, `MPEG4` with `H264`.

Nothing was refused at the setting. Every container offered the same seventeen codecs after being chosen — `CONTAINER WEBM       CODECS (17): ['NONE', 'AV1', 'H264', 'H265', 'WEBM', 'DNXHD', 'DV', 'FFV1', 'FLASH', 'HUFFYUV', 'MPEG1', 'MPEG2', 'MPEG4', 'PNG', 'PRORES', 'QTRLE', 'THEORA']` and the same for the other nine — and Blender printed:

```
PAIRINGS TRIED: 170
PAIRINGS THE SETTING REFUSED: []
```

The encoder refused eleven of the twenty-six. The script's own lines for every encode, uncut:

```
MPEG4  /H264     RESULT {'FINISHED'} FILES [('m_0001-0003.mp4', 2681)]
MKV    /H264     RESULT {'FINISHED'} FILES [('m_0001-0003.mkv', 2444)]
WEBM   /H264     RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination FILES [('m_0001-0003.webm', 264)]
AVI    /H264     RESULT {'FINISHED'} FILES [('m_0001-0003.avi', 7648)]
DV     /H264     RAISED RuntimeError: Error: Render width has to be 720 pixels for DV! FILES []
FLASH  /H264     RESULT {'FINISHED'} FILES [('m_0001-0003.flv', 724)]
MPEG1  /H264     RESULT {'FINISHED'} FILES [('m_0001-0003.mpg', 2048)]
MPEG2  /H264     RESULT {'FINISHED'} FILES [('m_0001-0003.dvd', 4096)]
OGG    /H264     RESULT {'FINISHED'} FILES [('m_0001-0003.ogv', 3855)]
QUICKTIME/H264     RESULT {'FINISHED'} FILES [('m_0001-0003.mov', 2615)]
MPEG4  /NONE     RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination FILES [('m_0001-0003.mp4', 0)]
MPEG4  /AV1      RAISED RuntimeError: Error: Invalid tile sizing: frame not large enough to fit specified tile arrangement. FILES []
MPEG4  /H265     RESULT {'FINISHED'} FILES [('m_0001-0003.mp4', 4417)]
MPEG4  /WEBM     RESULT {'FINISHED'} FILES [('m_0001-0003.mp4', 1105)]
MPEG4  /DNXHD    RAISED RuntimeError: Error: ff_frame_thread_encoder_init failed FILES []
MPEG4  /DV       RAISED RuntimeError: Error: Render width has to be 720 pixels for DV! FILES []
MPEG4  /FFV1     RESULT {'FINISHED'} FILES [('m_0001-0003.mp4', 4194)]
MPEG4  /FLASH    RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination FILES [('m_0001-0003.mp4', 0)]
MPEG4  /HUFFYUV  RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination FILES [('m_0001-0003.mp4', 0)]
MPEG4  /MPEG1    RESULT {'FINISHED'} FILES [('m_0001-0003.mp4', 1395)]
MPEG4  /MPEG2    RESULT {'FINISHED'} FILES [('m_0001-0003.mp4', 1445)]
MPEG4  /MPEG4    RESULT {'FINISHED'} FILES [('m_0001-0003.mp4', 1310)]
MPEG4  /PNG      RESULT {'FINISHED'} FILES [('m_0001-0003.mp4', 6062)]
MPEG4  /PRORES   RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination FILES [('m_0001-0003.mp4', 0)]
MPEG4  /QTRLE    RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination FILES [('m_0001-0003.mp4', 0)]
MPEG4  /THEORA   RAISED RuntimeError: Error: Could not initialize streams, probably unsupported codec combination FILES [('m_0001-0003.mp4', 0)]
```

Each refusal also printed its own `Error: …` line with the same words. Several refused pairings left a file of 0 or 264 bytes behind. The MPEG-4 container with the FFV1 codec wrote a file.

Blender's own log, in the same run, carried three more lines that the script's lines do not account for:

```
00:11.380  video.write      | ERROR Couldn't initialize video codec: Invalid argument
00:14.102  video.write      | ERROR Couldn't initialize video codec: Invalid argument
00:20.888  video.write      | ERROR Couldn't initialize video codec: Generic error in an external library
```

Blender's log lines and the script's lines reached the log file separately, so which encode each of these belongs to cannot be read from the record. A second run with Python's output unbuffered put them in the same order and did not tie them down either.

*A frame size the codec refuses (entry 3, part d).* At 101 × 101, MPEG-4 with H.264, frames 1 to 3, the render raised:

```
Error: width not divisible by 2 (101x101)
RAISED RuntimeError: Error: width not divisible by 2 (101x101)
FOLDER AFTER: []
```

The output folder was empty afterwards, and the whole run still ended with exit status `0`, since `--python-exit-code` was not given.

*A container and codec that do not fit, rendered from Blender's window (entry 43).* No earlier version ran this. Blender 5.3.0 Beta was opened with its window on the Mac's own display (`--factory-startup -noaudio --enable-event-simulate --window-geometry 0 0 1600 1000` and a plain saved start file), its first-run "Quick Setup" box closed with a simulated Escape key, and driven by a timed script. Set up: the factory scene at 64 × 48, frames 1 to 2, Media Type `VIDEO`, container `WEBM`, codec `H264` (`SET VIDEO WEBM H264 frames 1 2`). Done: an animation render started as the Render menu starts one, `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)`, which returned `{'RUNNING_MODAL'}`; after a fixed pause, a picture with Blender's own screenshot command. Blender's log printed:

```
00:11.574  operator         | ERROR Could not initialize streams, probably unsupported codec combination
```

The picture shows, in the status bar at the foot of the window, a red error mark and the words `Could not initialize streams, probably unsupported codec combination`. The output folder held one file afterwards, `m_0001-0002.webm`, of 264 bytes. (A try at copying the window's reports into a text block with `bpy.ops.ui.reports_to_textblock()` raised `AttributeError: Calling operator "bpy.ops.ui.reports_to_textblock" error, could not be found`.)

**The case the question was written for could not be produced.** The build options recorded under question 3 show every optional format compiled into this build. A blend-file asking for a format its Blender lacks needs a Blender built without one of them; the only builds on this Mac are the official 5.3.0 Beta and 5.2.2, and nothing could be downloaded in this sitting. So reading (a) stays unanswered for 5.3.

---

## 5. Can a format hold several pictures inside one file, and if so, how can the finished picture be told apart from the others inside it?

**Label: documented** that it can. Telling the finished picture apart is partly inferred and partly not in the sources.

**Answer.**

Formats that hold several pictures in one file:

- **OpenEXR.** "An OpenEXR file can store multiple layers and passes."
  - The render output's "Cache Result" "Saves the rendered view layers and their passes to a multi-layer OpenEXR image".
  - The Multi-Layer EXR media type "Saves all inputs together in a single multi-layer OpenEXR file".
  - OpenEXR also holds several views. "Multi-view native support is only available to OpenEXR."
  - By default Blender stores these as a multi-part OpenEXR. An "Interleave" option, off by default, switches to the older interleaved storage.
- **Stereo pairs.** The "Stereo 3D" views format puts "a single file with an encoded stereo pair". **Inferred:** this works in other formats too, because the setting belongs to the general image format settings.
- **Video containers** hold many frames. Matroska is described as "capable of holding multiple video, audio, subtitle, and metadata tracks". The sources do not say that Blender writes more than one video track.

How the pictures inside are told apart:

- **Pictures are told apart by their layer and pass names.**
  - When the compositor's File Output node writes a Multi-Layer EXR, "each input is stored as a separate layer within the file". The inputs are named in the node.
  - The Cryptomatte node calls the layer it reads "typically a combination of a View Layer and a Cryptomatte type (Object/Material/Asset)".
  - When the compositor loads such a file, "each layer is made available as a socket".
  - **Inferred:** layer names inside the file combine the view layer name and the pass name, with a view name when there are several views. So the uncomposited full image of a view layer is the pass named "Combined" under that view layer's name. Cycles registers it under exactly that name (observed, see answer 1).
- **The composited picture can be in such a file, but its name inside is not in the sources.** The manual says: "To use the composited output of another scene instead of its raw render passes, first render that scene to a multilayer image format such as OpenEXR." **Inferred:** a multilayer render output can carry the composited picture as well as the raw passes. How the composited picture is named or marked inside the file is not in the sources.
- **Multi-view OpenEXR** "is backward compatible with old OpenEXR viewers (you see only one view though)". Which view such viewers show is not documented.
- **Observed, channel naming in Cycles' files:** Cryptomatte passes use lowercase channel names on purpose. The comment says this makes Blender's OpenEXR DWA code use lossless compression for them.

**Evidence.**
- Manual, https://docs.blender.org/manual/en/5.3/files/media/image_formats.html (read 2026-10-11):
  - "An OpenEXR file can store multiple layers and passes. This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact."
  - Interleave: "Use legacy interleaved storage of views, layers and passes for compatibility with applications that do not support more efficient multi-part OpenEXR files."
- API, https://docs.blender.org/api/5.3/bpy.types.ImageFormatSettings.html (read 2026-10-11): `use_exr_interleave` "… (default False)".
- Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html (read 2026-10-11): "Saves the rendered view layers and their passes to a multi-layer OpenEXR image."
- Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html (read 2026-10-11): "When using Multi-Layer EXR, each input is stored as a separate layer within the file."
- Manual, https://docs.blender.org/manual/en/5.3/compositing/types/mask/cryptomatte.html (read 2026-10-11): "Cryptomatte Layer … This is typically a combination of a View Layer and a Cryptomatte type (Object/Material/Asset)."
- Manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/image.html (read 2026-10-11): "When a multi-layer file format, like EXR, is loaded, each layer is made available as a socket."
- Manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/render_layers.html (read 2026-10-11): "To use the composited output of another scene instead of its raw render passes, first render that scene to a multilayer image format such as OpenEXR."
- Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/stereoscopy/usage.html (read 2026-10-11): "Another option is to use multi-view OpenEXR files. This format can save multiple views in a single file and is backward compatible with old OpenEXR viewers (you see only one view though)."
- API, https://docs.blender.org/api/5.3/bpy_types_enum_items/views_format_multiview_items.html (read 2026-10-11): "Single file with an encoded stereo pair." and "Single file with all the views."
- Manual, https://docs.blender.org/manual/en/5.3/files/media/video_formats.html (read 2026-10-11): Matroska "capable of holding multiple video, audio, subtitle, and metadata tracks in a single file."
- Release notes, https://developer.blender.org/docs/release_notes/5.3/pipeline_io/ (read 2026-10-11): "Improved reading and writing of multilayer OpenEXR files with lossy DWA compression."
- Cycles' own file `engine.py`, lines 235–237 (read 2026-10-16), observed: `# NOTE: Name channels are lowercase RGBA so that compression rules check in OpenEXR DWA code` / `# uses lossless compression. …`

**Where I looked.** The image formats, Output properties, File Output, compositor Image node, Render Layers node, Cryptomatte node and stereoscopy pages. The API pages ImageFormatSettings, the views-format lists, SceneRenderView, RenderResult, RenderLayer and RenderPass. Cycles' `engine.py`. Searches of the manual for "multilayer", "multi-layer", "multi layer", "multi-part" and "multipart". Searches of the manual and API for "Composite" and "composited", to find the name of the composited layer. None was found.

**Not covered.** *(Hand-tested 2026-10-18, below: the naming inside Blender's multi-layer files and the composited picture's part were measured. Which view an older reader shows was not.)*
- The exact layer, pass and channel naming scheme inside Blender's EXR files.
- The name, or any marker, of the composited picture in a multilayer render output.
- Which view an old viewer shows.

**Outside the interface.**
- **Observed, 2026-10-18 (entries 5, 6 and 7 below): it holds with no window.** Every file below was written with no window open and its names were read off the file itself, so the naming holds for anything driving Blender from outside. What follows is what the sources said before the test.
- All the settings named here are API properties: `media_type`, `use_exr_interleave`, `views_format` and the File Output node items.
- The sources do not say that the file layout differs outside the interface.

**Operating system.**
- **Observed, 2026-10-18, on macOS 26.6.2 on Apple silicon (the Creator's Mac) only.** No Linux or Windows machine ran 5.3 in this sitting. What follows is what the sources said before the test.
- The sources do not say.
- OpenEXR itself is one of the formats that is "not available on all systems" (see answer 3).

### Hand-test results — 2026-10-18

**Label for everything in this section: observed.** Run on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`, started from a terminal with no window (`-b --factory-startup -noaudio`), with its temporary folder and settings folders pointed at a work folder of its own, and driven by a script. The test list, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`; the entry numbers below are that list's. The file headers were read by a short reader written for these tests that walks every part of an OpenEXR file and prints each part's attributes in alphabetical order; pixel values were read with the `OpenEXR` Python module that ships inside this Blender (version 3.4.10).

*A rendered multi-layer OpenEXR (entry 5, case A).* Set up: 32 × 32, one frame; the first view layer renamed `Beauty` and a second added called `Second`; on both, `use_pass_combined`, `use_pass_z`, `use_pass_normal`, `use_pass_mist`, `use_pass_diffuse_color` and `use_pass_emit` switched on; `image_settings.media_type = 'MULTI_LAYER_IMAGE'` and then `file_format = 'OPEN_EXR_MULTILAYER'`; Interleave left at its default, which printed `False`; the render engine the factory default, which printed `BLENDER_EEVEE`; no compositor. Done: `bpy.ops.render.render(animation=True)`, then the file's header read. Part 0, uncut:

```
VERSION FIELD: 0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)
--- PART 0 HEADER ---
  [BlenderMultiChannel] (string, 25 bytes) = Blender V2.55.1 and newer
  [Camera] (string, 6 bytes) = Camera
  [Date] (string, 19 bytes) = 2026/10/18 03:49:21
  [File] (string, 10 bytes) = <untitled>
  [Frame] (string, 1 bytes) = 1
  [RenderTime] (string, 8 bytes) = 00:01.27
  [Scene] (string, 5 bytes) = Scene
  [Software] (string, 18 bytes) = Blender 5.3.0 Beta
  [Time] (string, 11 bytes) = 00:00:00:01
  [channels] (chlist, 137 bytes) = Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT)
  [chunkCount] (int, 4 bytes) = 2
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
  [compression] (compression, 1 bytes) = 3 (ZIP)
  [dataWindow] (box2i, 16 bytes) = (0, 0, 31, 31)
  [displayWindow] (box2i, 16 bytes) = (0, 0, 31, 31)
  [lineOrder] (lineOrder, 1 bytes) = 0
  [name] (string, 15 bytes) = Beauty.Combined
  [pixelAspectRatio] (float, 4 bytes) = 1.0
  [screenWindowCenter] (v2f, 8 bytes) = (0.0, 0.0)
  [screenWindowWidth] (float, 4 bytes) = 1.0
  [type] (string, 13 bytes) = scanlineimage
  [xDensity] (float, 4 bytes) = 72.0
  (22 attributes in part 0)
```

There were twelve parts — `TOTAL PARTS: 12` — one per pass per view layer. Their `channels` and `name` lines, in the order written:

```
  [channels] (chlist, 137 bytes) = Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT)
  [name] (string, 15 bytes) = Beauty.Combined
  [channels] (chlist, 32 bytes) = Beauty.Depth.Z(FLOAT)
  [name] (string, 12 bytes) = Beauty.Depth
  [channels] (chlist, 31 bytes) = Beauty.Mist.Z(FLOAT)
  [name] (string, 11 bytes) = Beauty.Mist
  [channels] (chlist, 97 bytes) = Beauty.Normal.X(FLOAT); Beauty.Normal.Y(FLOAT); Beauty.Normal.Z(FLOAT)
  [name] (string, 13 bytes) = Beauty.Normal
  [channels] (chlist, 118 bytes) = Beauty.Diffuse Color.B(FLOAT); Beauty.Diffuse Color.G(FLOAT); Beauty.Diffuse Color.R(FLOAT)
  [name] (string, 20 bytes) = Beauty.Diffuse Color
  [channels] (chlist, 103 bytes) = Beauty.Emission.B(FLOAT); Beauty.Emission.G(FLOAT); Beauty.Emission.R(FLOAT)
  [name] (string, 15 bytes) = Beauty.Emission
  [channels] (chlist, 137 bytes) = Second.Combined.A(FLOAT); Second.Combined.B(FLOAT); Second.Combined.G(FLOAT); Second.Combined.R(FLOAT)
  [name] (string, 15 bytes) = Second.Combined
  [channels] (chlist, 32 bytes) = Second.Depth.Z(FLOAT)
  [name] (string, 12 bytes) = Second.Depth
  [channels] (chlist, 31 bytes) = Second.Mist.Z(FLOAT)
  [name] (string, 11 bytes) = Second.Mist
  [channels] (chlist, 97 bytes) = Second.Normal.X(FLOAT); Second.Normal.Y(FLOAT); Second.Normal.Z(FLOAT)
  [name] (string, 13 bytes) = Second.Normal
  [channels] (chlist, 118 bytes) = Second.Diffuse Color.B(FLOAT); Second.Diffuse Color.G(FLOAT); Second.Diffuse Color.R(FLOAT)
  [name] (string, 20 bytes) = Second.Diffuse Color
  [channels] (chlist, 103 bytes) = Second.Emission.B(FLOAT); Second.Emission.G(FLOAT); Second.Emission.R(FLOAT)
  [name] (string, 15 bytes) = Second.Emission
```

Parts 1 to 11 each carry 13 attributes, with `colorInteropID` reading `data` on the Depth, Mist and Normal parts and `lin_rec709_scene` on the others. So the file is a multi-part file, each part named `<view layer>.<pass>` and each channel `<view layer>.<pass>.<channel>`; with no compositor, the finished picture of the first view layer is the part named `Beauty.Combined`.

*The same file with a compositor that turns the picture blue (entry 5, case B — the 5.3 audit's test).* Set up: case A, plus a compositor group put into the 5.3 effects stack: the Group Input's Image into the A input of a `ShaderNodeMix` node set to Color data, blend `MULTIPLY`, factor 1, B pure blue, its Result into the Group Output. Blender printed `MIX NODE TYPE USED: ShaderNodeMix`, `EFFECTS: [('BlueEffect', 'BlueComp', True)] | use_compositing: True` and `LINKS: [('NodeGroupInput', 'Image', 'ShaderNodeMix', 'A'), ('ShaderNodeMix', 'Result', 'NodeGroupOutput', 'Image')]`. Done: the same render. The file now has thirteen parts, `TOTAL PARTS: 13`, and a new first one. Part 0, uncut:

```
--- PART 0 HEADER ---
  [BlenderMultiChannel] (string, 25 bytes) = Blender V2.55.1 and newer
  [Camera] (string, 6 bytes) = Camera
  [Date] (string, 19 bytes) = 2026/10/18 03:49:22
  [File] (string, 10 bytes) = <untitled>
  [Frame] (string, 1 bytes) = 1
  [RenderTime] (string, 8 bytes) = 00:00.95
  [Scene] (string, 5 bytes) = Scene
  [Software] (string, 18 bytes) = Blender 5.3.0 Beta
  [Time] (string, 11 bytes) = 00:00:00:01
  [channels] (chlist, 149 bytes) = Composite.Combined.A(FLOAT); Composite.Combined.B(FLOAT); Composite.Combined.G(FLOAT); Composite.Combined.R(FLOAT)
  [chunkCount] (int, 4 bytes) = 2
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
  [compression] (compression, 1 bytes) = 3 (ZIP)
  [dataWindow] (box2i, 16 bytes) = (0, 0, 31, 31)
  [displayWindow] (box2i, 16 bytes) = (0, 0, 31, 31)
  [lineOrder] (lineOrder, 1 bytes) = 0
  [name] (string, 18 bytes) = Composite.Combined
  [pixelAspectRatio] (float, 4 bytes) = 1.0
  [screenWindowCenter] (v2f, 8 bytes) = (0.0, 0.0)
  [screenWindowWidth] (float, 4 bytes) = 1.0
  [type] (string, 13 bytes) = scanlineimage
  [xDensity] (float, 4 bytes) = 72.0
  (22 attributes in part 0)
```

Parts 1 to 12 are case A's twelve, in the same order, `Beauty.Combined` now part 1 with 13 attributes. The centre pixel of the first two parts, and of `Second.Combined`:

```
  part 0 name 'Composite.Combined' channel 'Composite.Combined' centre [0.0, 0.0, 0.2461, 1.0]
  part 1 name 'Beauty.Combined' channel 'Beauty.Combined' centre [0.246, 0.246, 0.2461, 1.0]
  part 7 name 'Second.Combined' channel 'Second.Combined' centre [0.246, 0.246, 0.2461, 1.0]
```

So the composited picture is written into the multi-layer file as its own part, named `Composite.Combined`, and placed first; it carries the record of fields that case A carried on `Beauty.Combined`. The view layers' own Combined parts still hold the picture before the compositor.

*The same with Interleave on (entry 5, case C).* `use_exr_interleave` set to `True` (it printed `True`). The file is one part, `VERSION FIELD: 0x00000002 (version 2, tiled=False longnames=False deep=False multipart=False)`, `TOTAL PARTS: 1`, with no `name` attribute and every channel in one list:

```
  [channels] (chlist, 1173 bytes) = Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT); Beauty.Depth.Z(FLOAT); Beauty.Diffuse Color.B(FLOAT); Beauty.Diffuse Color.G(FLOAT); Beauty.Diffuse Color.R(FLOAT); Beauty.Emission.B(FLOAT); Beauty.Emission.G(FLOAT); Beauty.Emission.R(FLOAT); Beauty.Mist.Z(FLOAT); Beauty.Normal.X(FLOAT); Beauty.Normal.Y(FLOAT); Beauty.Normal.Z(FLOAT); Composite.Combined.A(FLOAT); Composite.Combined.B(FLOAT); Composite.Combined.G(FLOAT); Composite.Combined.R(FLOAT); Second.Combined.A(FLOAT); Second.Combined.B(FLOAT); Second.Combined.G(FLOAT); Second.Combined.R(FLOAT); Second.Depth.Z(FLOAT); Second.Diffuse Color.B(FLOAT); Second.Diffuse Color.G(FLOAT); Second.Diffuse Color.R(FLOAT); Second.Emission.B(FLOAT); Second.Emission.G(FLOAT); Second.Emission.R(FLOAT); Second.Mist.Z(FLOAT); Second.Normal.X(FLOAT); Second.Normal.Y(FLOAT); Second.Normal.Z(FLOAT)
  (19 attributes in part 0)
```

and the channel group `Composite.Combined` read `centre [0.0, 0.0, 0.2461, 1.0]`, `Beauty.Combined` `centre [0.246, 0.246, 0.2461, 1.0]`. So with Interleave the composited picture is the channels whose names begin `Composite.Combined.`.

*A multi-layer OpenEXR written by a File Output node (entry 6).* Set up: 32 × 32, one frame, main output PNG; a compositor group in the effects stack whose Group Input Image goes to the Group Output and to a File Output node with `directory` set to the output folder, `file_name = "fromnode_"`, `format.media_type = 'MULTI_LAYER_IMAGE'`, `format.file_format = 'OPEN_EXR_MULTILAYER'`, its items cleared and two added with `file_output_items.new('RGBA', "MyBeautyLayer")` and `file_output_items.new('RGBA', "second socket")`, both fed from the Group Input's Image. Done: `bpy.ops.render.render(animation=True)`. Blender printed `Saved: '…/out/e06/fromnode_0001.exr'` and `Saved: '…/out/e06/main_0001.png'`, and the header reader printed:

```
VERSION FIELD: 0x00001002 (version 2, tiled=False longnames=False deep=False multipart=True)
--- PART 0 HEADER ---
  [BlenderMultiChannel] (string, 25 bytes) = Blender V2.55.1 and newer
  [Camera] (string, 6 bytes) = <none>
  [Date] (string, 19 bytes) = 2026/10/18 03:50:31
  [File] (string, 10 bytes) = <untitled>
  [Frame] (string, 1 bytes) = 1
  [RenderTime] (string, 8 bytes) = 00:00.00
  [Scene] (string, 5 bytes) = Scene
  [Software] (string, 18 bytes) = Blender 5.3.0 Beta
  [Time] (string, 11 bytes) = 00:00:00:01
  [channels] (chlist, 129 bytes) = MyBeautyLayer.A(FLOAT); MyBeautyLayer.B(FLOAT); MyBeautyLayer.G(FLOAT); MyBeautyLayer.R(FLOAT)
  [chunkCount] (int, 4 bytes) = 32
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
  [compression] (compression, 1 bytes) = 0 (NO)
  [dataWindow] (box2i, 16 bytes) = (0, 0, 31, 31)
  [displayWindow] (box2i, 16 bytes) = (0, 0, 31, 31)
  [lineOrder] (lineOrder, 1 bytes) = 0
  [name] (string, 13 bytes) = MyBeautyLayer
  [pixelAspectRatio] (float, 4 bytes) = 1.0
  [screenWindowCenter] (v2f, 8 bytes) = (0.0, 0.0)
  [screenWindowWidth] (float, 4 bytes) = 1.0
  [type] (string, 13 bytes) = scanlineimage
  [xDensity] (float, 4 bytes) = 72.0
  (22 attributes in part 0)
--- PART 1 HEADER ---
  [channels] (chlist, 129 bytes) = second socket.A(FLOAT); second socket.B(FLOAT); second socket.G(FLOAT); second socket.R(FLOAT)
  …
  [name] (string, 13 bytes) = second socket
  (13 attributes in part 1)
TOTAL PARTS: 2
```

So a layer written by that node takes its item's name, spaces and all, with no view-layer name in front of it.

*Several views inside one file (entry 7).* No earlier version ran this on a Mac. Set up: 32 × 32, one frame, the view layer renamed `Beauty`, the Depth and Normal passes on, `render.use_multiview = True`, `render.views_format = 'STEREO_3D'`, Media Type `MULTI_LAYER_IMAGE`. Done: rendered once with the image settings' own `views_format` at its default, and again with it set to `MULTIVIEW`. Blender printed:

```
VIEWS: [('left', True), ('right', True)]
image_settings.views_format default: INDIVIDUAL
RESULT default: {'FINISHED'}
FILES with views_format at its default: ['frame_0001_L.exr', 'frame_0001_R.exr']
image_settings.views_format now: MULTIVIEW
RESULT multiview: {'FINISHED'}
FILES with MULTIVIEW: ['frame_0001.exr']
```

The one file is multi-part with six parts, two per pass. The `channels`, `name` and `view` attributes of each, as the header reader printed them (the other attributes left out here; part 0 also carries the record of fields and `Software … Blender 5.3.0 Beta`):

```
--- PART 0 HEADER ---
  [channels] (chlist, 137 bytes) = Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT)
  [name] (string, 20 bytes) = Beauty.Combined-left
  [view] (string, 4 bytes) = left
  (23 attributes in part 0)
--- PART 1 HEADER ---
  [channels] (chlist, 137 bytes) = Beauty.Combined.A(FLOAT); Beauty.Combined.B(FLOAT); Beauty.Combined.G(FLOAT); Beauty.Combined.R(FLOAT)
  [name] (string, 21 bytes) = Beauty.Combined-right
  [view] (string, 5 bytes) = right
  (14 attributes in part 1)
--- PART 2 HEADER ---
  [channels] (chlist, 32 bytes) = Beauty.Depth.Z(FLOAT)
  [name] (string, 17 bytes) = Beauty.Depth-left
  [view] (string, 4 bytes) = left
--- PART 3 HEADER ---
  [channels] (chlist, 32 bytes) = Beauty.Depth.Z(FLOAT)
  [name] (string, 18 bytes) = Beauty.Depth-right
  [view] (string, 5 bytes) = right
--- PART 4 HEADER ---
  [channels] (chlist, 97 bytes) = Beauty.Normal.X(FLOAT); Beauty.Normal.Y(FLOAT); Beauty.Normal.Z(FLOAT)
  [name] (string, 18 bytes) = Beauty.Normal-left
  [view] (string, 4 bytes) = left
--- PART 5 HEADER ---
  [channels] (chlist, 97 bytes) = Beauty.Normal.X(FLOAT); Beauty.Normal.Y(FLOAT); Beauty.Normal.Z(FLOAT)
  [name] (string, 19 bytes) = Beauty.Normal-right
  [view] (string, 5 bytes) = right
TOTAL PARTS: 6
```

So a view is a suffix on the part's name, `<view layer>.<pass>-<view>`, and its own `view` attribute; it is not in the channel names.

*The Image Editor's picture menus, with Blender's window open (entry 40).* Run on the same Mac in the same Blender 5.3.0 Beta, this time with its window open on the Mac's own display, started from a terminal with `--factory-startup -noaudio --enable-event-simulate --window-geometry 0 0 1600 1000` and a plain saved start file, and driven by a timed script; the window measured 3024 × 1724 pixels. With the work folder's empty settings, Blender opened its first-run "Quick Setup" box over the window; a simulated Escape key closed it. Pictures were taken with Blender's own screenshot command, `bpy.ops.screen.screenshot`, after forcing the window to redraw; the menus were opened by simulated mouse clicks on the header, which Blender accepts in place of the real mouse when started with `--enable-event-simulate`. (Two earlier window runs, without the forced redraws, returned the same picture for several different steps, so their pictures were not used.) The largest area was turned into an Image Editor showing the multi-layer OpenEXR of entry 5 case A (view layers `Beauty` and `Second`).

Seen in the pictures. The header shows the file name `frame_0001.exr` and then two dropdown menus, reading `Beauty` and `Combined`, and no third. Clicking the first opened a list reading `Beauty`, `Second`, then a dividing line and a greyed label `Layer`. Clicking the second opened `Combined`, `Depth`, `Normal`, `Mist`, `Emission`, `Diffuse Color`, then a dividing line and a greyed label `Pass`. The Image Editor's own View menu, opened with `bpy.ops.wm.call_menu(name="IMAGE_MT_view")`, holds, top to bottom: `Toolbar`, `Sidebar`, `Tool Settings`, `Asset Shelf` (greyed), `Adjust Last Operation`, then `Update Automatically`, `Show Metadata`, then `Frame All`, `Center View to Cursor` (greyed), `Zoom`, then `Area`; nothing in it chooses which picture is shown.

Then the multi-view file of entry 7 was shown. With the editor's stereo button left on, and the picture photographed after two more redraws, the header holds a stereo-glasses button, shown pressed, and two menus reading `Beauty` and `Combined`. With `space.show_stereo_3d` set to `False`, a third menu appears after those two, reading `left`; clicking it opened a list with a greyed `View` label at the top, a dividing line, then `right` and `left`. (A step that tried to read the multi-view picture's first pixel through `img.pixels[0]` raised `IndexError: bpy_prop_array[index]: index 0 out of range`; in the same picture the editor shows the small 32 × 32 frame of the cube with red and cyan fringes on its edges.)

---

## 6. Does a format that can hold more light than a screen can show also record the treatment it was made under?

**Label: inferred.** For OpenEXR, the view treatment is not recorded in a way Blender reads back. Some parts are documented: colour-space information is saved, sRGB files get an ICC profile, HDR output carries a luminance figure, and Radiance HDR holds no metadata.

**The question is ambiguous.** "Treatment" could mean:

- **(a)** The view settings applied for display: view transform (such as AgX), look, exposure, gamma, curves and white balance.
- **(b)** The colour space or target display the pixels are in.

I answer both.

**Answer.**

- **OpenEXR, reading (a):**
  - The manual says OpenEXR files "are always stored in scene linear color spaces".
  - When saving "as render", OpenEXR does not get the view transform: "For intermediate image formats like OpenEXR, use the default render output color space."
  - When such a file is opened again, "Blender cannot detect automatically that this is a render", so the user must turn on "View as Render".
  - **Inferred:** the view treatment is not baked into an OpenEXR. The file also carries no record of it that Blender can read back. The step is that if the file named its view treatment, Blender would not need to be told that it is a render. Whether some record is written that Blender ignores on loading is not in the sources.
- **OpenEXR, reading (b):**
  - The manual says Blender "saves images and videos with color space metadata".
  - The 5.3 release notes add "Improved support for display color interop IDs in OpenEXR files and color management".
  - On opening, Blender sets the colour space of an OpenEXR "if it is detected as Linear CIE-XYZ E or ACES2065-1".
  - **Inferred:** OpenEXR files can carry colour-space information. Which fields Blender writes is not stated.
- **Radiance HDR:** the format table marks Metadata ✗. The manual also says Cineon, DPX, OpenEXR and Radiance HDR "default to being saved in a linear color space".
- **HDR images in display form** (AVIF or PNG), and HDR video:
  - "HDR images can be written as AVIF or PNG files. They are written with 203 nits diffuse white". HDR video is "written with 100 nits diffuse white".
  - For HDR views whose names contain "HDR 500 nits", "HDR 1000 nits" and so on, the number is used "to automatically determine maximum luminance for mastering display metadata".
  - **Inferred:** such files record a maximum-luminance figure taken from the view's name. The sources do not say that the view's name, look or exposure is written.
- **sRGB files:** "Saving sRGB image files now includes an ICC profile" (5.3). An OpenColorIO configuration attribute can name the ICC profile to embed.
- **The render metadata fields do not cover colour management.** The fields are Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name and Note. **Inferred:** none of them names the view transform, look or exposure.

**Evidence.**
- Manual, https://docs.blender.org/manual/en/5.3/render/color_management/color_spaces.html (read 2026-10-11): "For intermediate image files in production, it is recommended to use OpenEXR files. These are always stored in scene linear color spaces, with high precision to avoid data loss."
- Manual, https://docs.blender.org/manual/en/5.3/files/media/image_formats.html (read 2026-10-11):
  - Save As Render: "For display image formats like PNG, apply view and display transform. For intermediate image formats like OpenEXR, use the default render output color space."
  - "Note, Cineon, DPX, OpenEXR, and Radiance HDR image types default to being saved in a linear color space."
  - "When opening OpenEXR images, Blender will automatically set the Color Space if it is detected as Linear CIE-XYZ E or ACES2065-1."
  - Table row for Radiance HDR: float, ✓ (alpha), ✗ (metadata), ✗ (DPI), .hdr.
- Manual, https://docs.blender.org/manual/en/5.3/render/color_management/displays_views.html (read 2026-10-11):
  - "However, when loading a render saved to an intermediate OpenEXR file, Blender cannot detect automatically that this is a render (it could be e.g. an image texture or displacement map)."
  - "View as Render … Display the image data-block (not only renders) with view, exposure, gamma, RGB curves applied."
  - "HDR images can be written as AVIF or PNG files. They are written with 203 nits diffuse white to match the convention in most browsers and image viewers."
  - "HDR videos are written with 100 nits diffuse white"
- Manual, https://docs.blender.org/manual/en/5.3/render/color_management/system_configuration.html (read 2026-10-11): "By default, Blender integrates with automatic color management in operating systems and saves images and videos with color space metadata."
- Manual, https://docs.blender.org/manual/en/5.3/render/color_management/opencolorio.html (read 2026-10-11):
  - "Saving colorspace metadata for images and video also depends on this."
  - "The icc_profile_name interchange attribute is supported, to embed ICC profiles when saving images."
  - "For HDR view transforms, use HDR 500 nits, HDR 1000 nits, HDR 2000 nits or HDR 4000 nits as part of the name to automatically determine maximum luminance for mastering display metadata."
- Release notes, https://developer.blender.org/docs/release_notes/5.3/pipeline_io/ (read 2026-10-11): "Saving sRGB image files now includes an ICC profile." and "Improved support for display color interop IDs in OpenEXR files and color management."
- API, https://docs.blender.org/api/5.3/bpy.types.ImageFormatSettings.html (read 2026-10-11):
  - `view_settings` "Color management settings applied on image before saving"
  - `has_linear_colorspace` "File format expects linear color space"
  - `linear_colorspace_settings` "Output color space settings"
- Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html (read 2026-10-11): the field list.

**Where I looked.**
- The colour management pages: index, color spaces, displays and views, OpenColorIO and system configuration.
- The image formats, video formats, metadata and Output properties pages.
- The API pages ImageFormatSettings and RenderSettings, and the Python API, Pipeline & I/O and Rendering release notes.
- Searches for "metadata", "ICC", "interop", "chromaticit", "header", "EXR", "written" and "save".

**Not covered.** *(Hand-tested 2026-10-18, below: the attributes written into OpenEXR, PNG, Radiance HDR and HDR PNG and AVIF were read. None names the view transform, look or exposure.)*
- Which header attributes Blender actually writes into an OpenEXR or an HDR AVIF/PNG.
- Whether any of them names the view transform, look or exposure.
- What "display color interop IDs in OpenEXR files" stores.

**Outside the interface.**
- **Observed, 2026-10-18 (entries 21 and 22 below), with no window.** Every file below was written and read with no window open, the treatment set from a script and the working space changed by an operator. A background render writes the colour-space attribute (`colorInteropID`), an sRGB ICC profile and a `cICP` block into a PNG, and an `nclx` colour description into an AVIF; it does not write the view transform, look, exposure or gamma. What follows is what the sources said before the test.
- The colour settings for saving ("Follow Scene" or "Override") are API properties, so a script can set them.
- The sources do not say that what is written differs outside the interface.

**Operating system.**
- **Observed, 2026-10-18, on macOS 26.6.2 on Apple silicon (the Creator's Mac) only.** No Linux or Windows machine ran 5.3 in this sitting, and the Linux Wayland sentence below stays untried. What follows is what the sources said before the test.
- The sources do not say that saved files differ by operating system.
- The system configuration page describes operating-system differences for display only. One sentence there could touch files: on Linux Wayland, "Blender automatically switches between sRGB and gamma 2.2 image encoding depending on whether an SDR or HDR display was chosen". The page does not say whether this affects saved files.

### Hand-test results — 2026-10-18

**Label for everything in this section: observed.** Run on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`, started from a terminal with no window (`-b --factory-startup -noaudio`), with its temporary folder and settings folders pointed at a work folder of its own, and driven by a script. The test list, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`; the entry numbers below are that list's. OpenEXR headers were read by a reader written for these tests that prints every attribute in alphabetical order; PNG files by a reader that walks the file block by block, and by `exiftool -a -G1 -s` (version 13.55).

*What a file records about its colour treatment (entry 21).* Set up: the factory startup scene at 32 × 32, one still each time, the render engine the factory default (EEVEE). Seven files, one thing changed at a time: an OpenEXR with view transform `AgX` and look `None`; the same with `Standard`; the same with `AgX`, look `AgX - Punchy`, exposure 2 and gamma 0.5; a Radiance HDR; a PNG under `AgX`; a PNG under `Standard`; and an OpenEXR written after `bpy.ops.wm.set_working_color_space(working_space='ACEScg', convert_colors=False)`. Blender printed:

```
DEFAULT view_transform='AgX' look='None' exposure=0.0 gamma=1.0
BLEND FILE COLORSPACE working_space = 'Linear Rec.709'
BLEND FILE COLORSPACE working_space_interop_id = 'lin_rec709_scene'
BLEND FILE COLORSPACE is_missing_opencolorio_config = False
DISPLAY DEVICE: sRGB
WROTE exr_agx.exr | view_transform='AgX' look='None' exposure=0.0 gamma=1.0
WROTE exr_standard.exr | view_transform='Standard' look='None' exposure=0.0 gamma=1.0
WROTE exr_punchy_exposed.exr | view_transform='AgX' look='AgX - Punchy' exposure=2.0 gamma=0.5
WROTE radiance.hdr | view_transform='AgX' look='None' exposure=0.0 gamma=1.0
WROTE display_png.png | view_transform='AgX' look='None' exposure=0.0 gamma=1.0
WROTE png_standard.png | view_transform='Standard' look='None' exposure=0.0 gamma=1.0
set_working_color_space returned {'FINISHED'}
working_space after: ACEScg interop id: lin_ap1_scene
WROTE exr_acescg.exr | view_transform='AgX' look='None' exposure=0.0 gamma=1.0
```

The OpenEXR made under AgX, every attribute except the layout ones (`compression`, `dataWindow`, `displayWindow`, `lineOrder`, `pixelAspectRatio`, `screenWindowCenter`, `screenWindowWidth`, `type` and `chunkCount`, all present and left out here):

```
VERSION FIELD: 0x00000002 (version 2, tiled=False longnames=False deep=False multipart=False)
--- PART 0 HEADER ---
  [Camera] (string, 6 bytes) = Camera
  [Date] (string, 19 bytes) = 2026/10/18 04:04:07
  [File] (string, 10 bytes) = <untitled>
  [Frame] (string, 3 bytes) = 001
  [RenderTime] (string, 8 bytes) = 00:00.59
  [Scene] (string, 5 bytes) = Scene
  [Software] (string, 18 bytes) = Blender 5.3.0 Beta
  [Time] (string, 11 bytes) = 00:00:00:01
  [channels] (chlist, 73 bytes) = A(FLOAT); B(FLOAT); G(FLOAT); R(FLOAT)
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
  [name] (string, 0 bytes) = 
  [xDensity] (float, 4 bytes) = 72.0
  (20 attributes in part 0)
```

The OpenEXR made under Standard and the one made with a look, exposure and gamma printed the same attribute lines, with only `Date` and `RenderTime` differing (each 20 attributes). No attribute names a view transform, a look, an exposure or a gamma. The OpenEXR written after the working space was changed carries `[colorInteropID] (string, 13 bytes) = lin_ap1_scene`, with channels `B(HALF); G(HALF); R(HALF)` (the settings left from the Radiance HDR file before it); so the attribute follows the working space, not the treatment.

The Radiance HDR file begins `b'#?RADIANCE\nFORMAT=32-bit_rle_rgbe\n\n-Y 32 +X 32\n'` and then the pixels (1126 bytes in all).

Both PNG files, read block by block, printed the same list; the AgX one:

```
CHUNKS: IHDR, cICP, iCCP, sRGB, gAMA, cHRM, eXIf, oFFs, pHYs, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, IDAT, IEND
  IHDR 32x32 bitdepth=16 colortype=2
  cICP = 010d0001
  iCCP profile name = Embedded Profile, 334 bytes
  sRGB = 03
  gAMA = 0000b18f
  cHRM = 00007a26000080840000fa00000080e8000075300000ea6000003a9800001770
  eXIf 54 bytes
  oFFs = 000000000000000000
  pHYs = 00000b1200000b1201
  tEXt File = <untitled>
  tEXt Date = 2026/10/18 04:04:09
  tEXt Time = 00:00:00:01
  tEXt Frame = 001
  tEXt Camera = Camera
  tEXt Scene = Scene
  tEXt RenderTime = 00:00.44
```

(The 16-bit RGB form is the depth and colour settings left from the files written before it in the same run.) `exiftool` read the two new blocks as:

```
[PNG]           ProfileName                     : Embedded Profile
[PNG-cICP]      ColorPrimaries                  : BT.709
[PNG-cICP]      TransferCharacteristics         : sRGB or sYCC
[PNG-cICP]      MatrixCoefficients              : Identity matrix
[PNG-cICP]      VideoFullRangeFlag              : 1
[ICC-header]    ProfileCMMType                  : Little CMS
[ICC-header]    ProfileVersion                  : 4.2.0
[ICC-header]    ProfileClass                    : Display Device Profile
[ICC-header]    ColorSpaceData                  : RGB
[ICC-header]    ProfileConnectionSpace          : XYZ
[ICC-header]    ProfileDateTime                 : 2018:03:20 09:14:29
[ICC-header]    RenderingIntent                 : Perceptual
[ICC_Profile]   ProfileDescription              : sRGB
[ICC_Profile]   ProfileCopyright                : CC0
[ICC_Profile]   MediaWhitePoint                 : 0.9642 1 0.82491
[ICC_Profile]   RedMatrixColumn                 : 0.43604 0.22244 0.0139
[ICC_Profile]   GreenMatrixColumn               : 0.3851 0.71693 0.09708
[ICC_Profile]   BlueMatrixColumn                : 0.14307 0.06062 0.71393
```

along with the older colour lines `SRGBRendering : Absolute Colorimetric`, `Gamma : 2.2`, `WhitePointX : 0.3127`, `WhitePointY : 0.329`, `RedX : 0.64`, `RedY : 0.33`, `GreenX : 0.3`, `GreenY : 0.6`, `BlueX : 0.15`, `BlueY : 0.06`. The Standard PNG's `exiftool` listing is the same, line for line, apart from its size, its times and `RenderTime : 00:00.43`. So a PNG written by this version carries an sRGB ICC profile and a `cICP` block naming BT.709 and sRGB, under AgX and under Standard alike — and still nothing naming the view transform, look, exposure or gamma.

*Opening the files again (entry 21).* Each file loaded with `bpy.data.images.load(...)` in the same windowless Blender:

```
OPENED exr_agx.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_standard.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED exr_punchy_exposed.exr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED radiance.hdr size=(32, 32) depth=128 is_float=True colorspace='Linear Rec.709' use_view_as_render=False
OPENED display_png.png size=(32, 32) depth=96 is_float=True colorspace='sRGB' use_view_as_render=False
OPENED png_standard.png size=(32, 32) depth=96 is_float=True colorspace='sRGB' use_view_as_render=False
OPENED exr_acescg.exr size=(32, 32) depth=96 is_float=True colorspace='ACEScg' use_view_as_render=False
```

All three OpenEXR files made in the Rec.709 working space come back the same way whatever treatment they were made under, with View as Render off; the one written in the ACEScg working space comes back as `ACEScg`.

*HDR output to PNG and AVIF (entry 22).* No earlier version ran this. Set up: the factory startup scene at 32 × 32, one still each. First, the display devices this build offers and the view transforms each allows were printed. The first run of the script stopped there: the setting's ordinary list of choices read back only `['NONE']`, and assigning that gave

```
TypeError: bpy_struct: item.attr = val: enum "NONE" not found in ('sRGB', 'Display P3', 'Rec.1886', 'Rec.2020', 'Rec.2100-PQ', 'Rec.2100-HLG')
```

So the second run read each list from Blender's own refusal of a made-up name, and printed:

```
DISPLAY DEVICES: ['sRGB', 'Display P3', 'Rec.1886', 'Rec.2020', 'Rec.2100-PQ', 'Rec.2100-HLG']
VIEWS FOR 'sRGB': ['Standard', 'ACES 1.3', 'ACES 2.0', 'Khronos PBR Neutral', 'AgX', 'Filmic', 'Filmic Log', 'False Color', 'Raw']
VIEWS FOR 'Display P3': ['Standard', 'ACES 1.3', 'ACES 2.0', 'AgX', 'False Color', 'Raw']
VIEWS FOR 'Rec.1886': ['Standard', 'ACES 1.3', 'ACES 2.0', 'AgX', 'False Color', 'Raw']
VIEWS FOR 'Rec.2020': ['Standard', 'ACES 1.3', 'ACES 2.0', 'AgX', 'False Color', 'Raw']
VIEWS FOR 'Rec.2100-PQ': ['Standard', 'ACES 1.3 - HDR 1000 nits', 'ACES 1.3 - HDR 2000 nits', 'ACES 1.3 - HDR 4000 nits', 'ACES 1.3 - SDR', 'ACES 2.0 - HDR 500 nits', 'ACES 2.0 - HDR 1000 nits', 'ACES 2.0 - HDR 2000 nits', 'ACES 2.0 - HDR 4000 nits', 'ACES 2.0 - SDR', 'AgX - HDR 1000 nits', 'AgX - SDR', 'False Color', 'Raw']
VIEWS FOR 'Rec.2100-HLG': ['Standard', 'ACES 1.3 - HDR 1000 nits', 'ACES 1.3 - SDR', 'ACES 2.0 - HDR 1000 nits', 'ACES 2.0 - SDR', 'AgX - HDR 1000 nits', 'AgX - SDR', 'False Color', 'Raw']
CHOSEN HDR DISPLAY: Rec.2100-PQ
WROTE srgb.png | display='sRGB' view='AgX' look='None' | color_depth='8' color_mode='RGBA'
WROTE srgb.avif | display='sRGB' view='AgX' look='None' | color_depth='8' color_mode='RGBA'
CHOSEN HDR VIEW: ACES 1.3 - HDR 1000 nits
WROTE hdr.png | display='Rec.2100-PQ' view='ACES 1.3 - HDR 1000 nits' look='None' | color_depth='8' color_mode='RGBA'
WROTE hdr.avif | display='Rec.2100-PQ' view='ACES 1.3 - HDR 1000 nits' look='None' | color_depth='8' color_mode='RGBA'
```

The display was the first whose name holds `2100` and `PQ`, and the view the first whose name holds `HDR` and `1000`; all four files are 8 bits per channel. The two PNGs, block by block:

```
===== hdr.png
CHUNKS: IHDR, cICP, sRGB, gAMA, cHRM, eXIf, oFFs, pHYs, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, IDAT, IEND
  IHDR 32x32 bitdepth=8 colortype=6
  cICP = 09100001
  sRGB = 03
  gAMA = 0000b18f
  cHRM = 00007a26000080840000fa00000080e8000075300000ea6000003a9800001770
===== srgb.png
CHUNKS: IHDR, cICP, iCCP, sRGB, gAMA, cHRM, eXIf, oFFs, pHYs, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, tEXt, IDAT, IEND
  IHDR 32x32 bitdepth=8 colortype=6
  cICP = 010d0001
  iCCP profile name = Embedded Profile, 334 bytes
```

with the same seven text fields in both. `exiftool -a -G1 -s` read the colour lines as follows. The HDR PNG: `[PNG-cICP] ColorPrimaries : BT.2020, BT.2100`, `[PNG-cICP] TransferCharacteristics : SMPTE ST 2084, ITU BT.2100 PQ`, `[PNG-cICP] MatrixCoefficients : Identity matrix`, `[PNG-cICP] VideoFullRangeFlag : 1`, beside `SRGBRendering : Absolute Colorimetric`, `Gamma : 2.2` and the sRGB white point and primaries, and no ICC profile. The sRGB PNG: `ColorPrimaries : BT.709`, `TransferCharacteristics : sRGB or sYCC`, and the sRGB ICC profile recorded under entry 21. The HDR AVIF: `[QuickTime] ColorProfiles : nclx`, `ColorPrimaries : BT.2020, BT.2100`, `TransferCharacteristics : SMPTE ST 2084, ITU BT.2100 PQ`, `MatrixCoefficients : BT.2020 non-constant luminance, BT.2100 YCbCr`, `VideoFullRangeFlag : Full`. The sRGB AVIF: `ColorProfiles : nclx`, `ColorPrimaries : BT.709`, `TransferCharacteristics : sRGB or sYCC`, `MatrixCoefficients : BT.709`, `VideoFullRangeFlag : Full`. Neither AVIF carries any of the text fields. A byte search of all four files for `clli`, `mdcv`, `cLLI`, `mDCV`, `cLLi` and `mDCv` found none. So the HDR files name the Rec.2100 PQ encoding and nothing else about the treatment: not the view's name, not a nits figure, not the look.

---

## 7. Can Blender turn a set of finished frames into a short moving picture?

**Label: documented.**

**Answer.**

- **Yes.** Load the rendered frames into the Video Sequencer as an image-sequence strip, choose a video output, and render. Blender then "will render out the Sequence editor output into a movie".
- Video is encoded through FFmpeg, with the containers and codecs listed in answer 3.
- The frames in a strip can be a mix of file types. "each image will be one frame".
- The manual recommends this route over rendering straight to video. Rendering frames first lets you restart from the frame where a problem occurred and try different encodings quickly.
- **Limit:** "The Video Sequencer does not support multi-layer EXR files." For those, the manual says to use an Image node in the compositor instead.
- **Limit:** some codecs restrict the frame size. "H.264 requires both width and height to be divisible by 2."
- No minimum or maximum length is documented.
- The Animation Player only plays sequences back. It does not make a file. It is "a utility typically used for previewing rendered animations".

**Evidence.**
- Manual, https://docs.blender.org/manual/en/5.3/render/output/introduction.html (read 2026-10-11): "Images can then be encoded to a video by adding the rendered image sequence into the Video Sequencer and choosing an appropriate Video Output."
- Manual, https://docs.blender.org/manual/en/5.3/render/output/animation.html (read 2026-10-11):
  - "Click the Animation render button and Blender will render out the Sequence editor output into a movie."
  - "The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor."
- Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html (read 2026-10-11): "If you first render out a set of static images such as the default PNG format or the higher-quality OpenEXR (which can retain HDR pixel data), you can combine them as an Image Strip in the Video Sequencer."
- Manual, https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/image.html (read 2026-10-11):
  - "an image sequence strip plays a series of images over time, with each image representing a single frame."
  - "each image will be one frame, in sort order, and can be a mix of file types (jpg, png, exr, etc.)."
- Manual, https://docs.blender.org/manual/en/5.3/files/media/video_formats.html (read 2026-10-11): "These formats are primarily used to compress rendered image sequences into playable movies." and "H.264 requires both width and height to be divisible by 2."
- Manual, https://docs.blender.org/manual/en/5.3/render/output/animation_player.html (read 2026-10-11): "The animation player is a utility typically used for previewing rendered animations".
- API, https://docs.blender.org/api/5.3/bpy.types.StripsTopLevel.html (read 2026-10-11): "new_image(name, filepath, channel, frame_start, *, fit_method='ORIGINAL') … Add a new image strip".
- API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html (read 2026-10-11): `use_sequencer` "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)".

**Where I looked.** The output introduction, Rendering Animations, Output properties, video formats, image strip and Animation Player pages. The API pages StripsTopLevel, RenderSettings and FFmpegSettings. The Sequencer release notes. Searches for "GIF", "animated WebP" and "APNG".

**Not covered.** *(Hand-tested 2026-10-18, below: the whole job was done with no window.)*
- A complete documented recipe for doing this with no interface at all. See below.

**Outside the interface.**
- **Observed, 2026-10-18 (entry 23 below): yes, the whole job.** The image strip was built from finished frames by a script in a windowless Blender, and the movie was encoded by `blender -b <file> -a` from a terminal, Blender printing `Executing sequencer` for each frame; the written frames show the pictures came from the strip and not from a fresh 3D render. What follows is what the sources said before the test.
- **Inferred:** it can be done without the interface. A script can add an image strip with `new_image`, set the format to FFmpeg video and render the animation. `-a` renders the frame range, and `-F FFMPEG` sets the format. The steps are each documented separately; the sources do not show them joined into one command-line recipe.
- The Animation Player is an interface window. On the command line, "-a <options> <file(s)>" runs it, and it is "ignored if -b is set".

**Operating system.**
- **Observed, 2026-10-18, on macOS 26.6.2 on Apple silicon (the Creator's Mac) only.** No Linux or Windows machine ran 5.3 in this sitting. What follows is what the sources said before the test.
- FFmpeg is one of the formats that are "not available on all systems".
- "Because codecs are required for both encoding and decoding, they must be available on both the system creating the file and the device playing it."
- The sources do not name per-system differences.

### Hand-test results — 2026-10-18

**Label for everything in this section: observed.** Run on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`, started from a terminal with no window throughout, with its temporary folder and settings folders pointed at a work folder of its own. The test list, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`; the entry numbers below are that list's.

*The whole job with no window at all (entry 23).* Set up, in a windowless Blender: six PNG frames at 64 × 48 rendered from the factory startup scene into a folder; then the 3D scene changed so a fresh render could not be mistaken for those frames — the cube deleted and the world background set to pure blue; then an image strip built from the six saved files with `scene.sequence_editor_create()` and `strips.new_image(name="Frames", filepath=<first file>, channel=1, frame_start=1)` followed by `strip.elements.append(<each remaining file name>)`. Blender printed `STRIP: Frames type: IMAGE elements: 6 | use_sequencer: True`. Four blend-files were saved from that state: video output (MPEG-4 container, H.264 codec) with the Sequencer pipeline on, the same with it off, PNG output with it on, PNG output with it off.

Done, in a terminal, for each:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b -noaudio --factory-startup <folder>/video_seq_on.blend -a
```

With the Sequencer on, Blender printed for each frame `Rendering frame N`, `Executing sequencer`, `Video append frame N` (for video) or `Saved: '…/s_000N.png'` (for PNG); with it off, `Start rendering: Scene, ViewLayer` and `Engine: EEVEE` in place of `Executing sequencer`. Every run ended with exit status `0`. The Sequencer-on video run wrote `out_0001-0006.mp4` and the Sequencer-off one `nosec_0001-0006.mp4`; `file` reported both as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`.

Which pictures went into it, from the PNG runs of the same two states:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0) bytes=3182
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3416
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0) bytes=3392
```

With the Sequencer pipeline on, what came out is the saved frames — grey cube on dark grey. With it off, what came out is the changed 3D scene — blue, no cube.

*A folder of frames with nothing prepared (entry 23).* Done, in a terminal, against the plain factory startup scene, with the frame size and range set small first:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup -noaudio --python-expr "import bpy; s=bpy.context.scene; s.render.resolution_x=64; s.render.resolution_y=48; s.frame_end=3" -F FFMPEG -o <folder>/n_ -a
```

Exit status `0`; Blender printed `Start rendering: Scene, ViewLayer`, `Engine: EEVEE` and `Video append frame N` for frames 1 to 3, and wrote `n_0001-0003.mkv`, which `file` reported as `Matroska data` — a movie of the 3D scene, not of any folder of pictures.

*A frame missing from the set at the moment of encoding (entry 23).* Set up: five frames written, a strip built naming all five, the file saved, and then the third frame's file moved out of the folder. Done: the same terminal animation render, to PNG. Exit status `0`; all five output frames were written, nothing was printed about the missing one, and their pixels read:

```
s_0001.png size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3640
s_0002.png size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3640
s_0003.png size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1621
s_0004.png size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3640
s_0005.png size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3640
```

The frame whose file was gone came out fully transparent black.

*Multi-layer OpenEXR frames offered to the Video Sequencer (entry 23).* Set up: three multi-layer OpenEXR frames at 64 × 48 with the Depth and Normal passes on; a strip built from them; PNG output. For comparison, the same scene's PNG frames put through the same route. Done: the same terminal animation render. Three files were written each time, with no message about the format:

```
mlexr_strip s_0001.png size=(64, 48) centre=(0.2603, 0.2603, 0.2605, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=4099
mlexr_strip s_0002.png size=(64, 48) centre=(0.2603, 0.2603, 0.2605, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=4099
mlexr_strip s_0003.png size=(64, 48) centre=(0.2603, 0.2603, 0.2605, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=4099
png3_strip s_0001.png size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3637
png3_strip s_0002.png size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3637
png3_strip s_0003.png size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3637
```

The picture is there, darker than the same scene's PNG frames through the same route.

*How long the result is (entry 23).* Set up: twenty-four frames, a strip built from all of them, video output with the MPEG-4 container and the H.264 codec. Done: encoded once with `render.fps = 24` and once with `render.fps = 12`, then each movie's own `mvhd` time scale and duration read:

```
at12_0001-0024.mp4     bytes=2596   timescale=1000 duration=2000 -> 2.000 s
at24_0001-0024.mp4     bytes=2313   timescale=1000 duration=1000 -> 1.000 s
nosec_0001-0006.mp4    bytes=1679   timescale=1000 duration=250 -> 0.250 s
out_0001-0006.mp4      bytes=1780   timescale=1000 duration=250 -> 0.250 s
```

*The video menus, with the window open (entry 41).* Recorded in full under question 3: the Output panel's `Media Type` menu offers `Image`, `Multi-Layer EXR` and `Video`; with `Video`, the `Container` menu lists `MPEG-4`, `Matroska`, `WebM`, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`, and the `Video Codec` menu lists `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`. The container and codec used above are both on those menus.

*Which container and codec will actually take it.* Entry 3, recorded under question 4, encoded three frames for every container with H.264 and for the MPEG-4 container with every codec: eleven of the twenty-six are refused, among them WebM with H.264.

---

## 8. Can Blender be made to produce a smaller, plainly viewable copy of a picture a render produced?

**Label: documented** that Blender can write a JPEG copy, and that images can be resized and saved in display formats. **Inferred** as to "smaller".

**Answer.**

- **The OpenEXR "Preview" option.** It saves "a JPEG copy of the image, for a quick preview" when rendering animations, "or single frames via command line".
  - The API describes the same setting, `use_preview`, as "When rendering animations, save JPG preview images in same directory". It is off by default.
  - **Inferred:** the copy is in a format that holds 8 bits per channel and no alpha (the JPEG row of the format table).
  - The sources do not say whether the copy is smaller in pixel size, or whether the view transform is applied to it.
- **Other documented building blocks:**
  - "Save as Render" converts with the view transform to display formats: "This is useful for saving linear OpenEXR to e.g. PNG or JPEG files in display space."
  - The compositor's File Output node can write extra images, each in its own format: "Each input can use its own image format settings." It has a `save_as_render` setting ("Apply render part of display transform when saving byte image (default True)").
  - Image ‣ Resize "Adjusts the image dimensions by scaling its pixel resolution". Scripts have `Image.scale(width, height)` ("Scale the buffer of the image, in pixels") and `Image.save_render(filepath, scene, quality)` ("Save image to a specific path using a scenes render settings").
  - Video Sequencer proxies are "copies of the original footage stored at a lower quality and/or resolution". They are for strips being edited, and can also be made "for image sequences".
- **Inferred:** a reduced-size viewable copy can be made by putting these together: open the render, scale it, and save it as PNG or JPEG with the render's colour treatment. No single documented option does all of this at once.

**Evidence.**
- Manual, https://docs.blender.org/manual/en/5.3/files/media/image_formats.html (read 2026-10-11):
  - OpenEXR Output Options, "Preview": "When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview."
  - JPEG row: 8bit, ✗ (alpha).
- API, https://docs.blender.org/api/5.3/bpy.types.ImageFormatSettings.html (read 2026-10-11): `use_preview` "When rendering animations, save JPG preview images in same directory (default False)".
- Manual, https://docs.blender.org/manual/en/5.3/render/color_management/displays_views.html (read 2026-10-11): "Save as Render … This is useful for saving linear OpenEXR to e.g. PNG or JPEG files in display space."
- Manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html (read 2026-10-11): "Each input can use its own image format settings."
- API, https://docs.blender.org/api/5.3/bpy.types.CompositorNodeOutputFile.html (read 2026-10-11): `save_as_render` "Apply render part of display transform when saving byte image (default True)".
- Manual, https://docs.blender.org/manual/en/5.3/editors/image/editing.html (read 2026-10-11): "Image ‣ Resize … Adjusts the image dimensions by scaling its pixel resolution."
- API, https://docs.blender.org/api/5.3/bpy.types.Image.html (read 2026-10-11):
  - "scale(width, height, *, frame=0, tile_index=0) Scale the buffer of the image, in pixels"
  - "save_render(filepath, *, scene=None, quality=0) Save image to a specific path using a scenes render settings"
- Manual, https://docs.blender.org/manual/en/5.3/editors/video_sequencer/sequencer/sidebar/proxy.html (read 2026-10-11): "copies of the original footage stored at a lower quality and/or resolution" and "(or create proxies for image sequences)".

**Where I looked.**
- Manual pages: image formats, Output properties, displays and views, File Output, Image Editor editing, Video Sequencer proxy, Format, file browser and the System preferences.
- API pages: ImageFormatSettings, Image and CompositorNodeOutputFile.
- Searches for "resize", "thumbnail", "proxy", "preview image", "jpeg copy" and "quick preview".

**Not covered.** *(Hand-tested 2026-10-18, below: the Preview JPEG's pixel size and colour treatment, its absence for PNG, and still renders made in the window were measured.)*
- The pixel size, colour treatment and quality of the OpenEXR "Preview" JPEG.
- Whether it exists for formats other than OpenEXR. The manual lists it only under OpenEXR. The API setting sits on the general image format settings.
- Whether it is written for still renders made inside the interface.

**Outside the interface.**
- **Observed, 2026-10-18 (entries 24 to 27 below), with no window; it splits.** The Preview copy was written for a three-frame animation and for a single frame started from a terminal. The half-size second file was written by a windowless render. The Image Editor commands `bpy.ops.image.resize` and `bpy.ops.image.save_as` refuse with `poll() failed, context is incorrect`, while `Image.scale` and `Image.save_render` do the same job (`save_render` writes in the scene's output format). Proxies were not built by a background animation render. With the window open (entries 41 and 42), the Preview control is offered for OpenEXR and Multi-Layer EXR only, and a still rendered with `write_still` writes the JPEG copy as a terminal render does; a still started as the Render menu starts it writes nothing until saved by hand, and that hand save writes no JPEG. What follows is what the sources said before the test.
- Documented: the manual names the command line for the Preview JPEG, "or single frames via command line".
- The API functions and properties above work from scripts.
- Image ‣ Resize is a menu item. **Inferred:** `Image.scale` is its scripting counterpart. The sources do not say they are the same thing.

**Operating system.**
- **Observed, 2026-10-18, on macOS 26.6.2 on Apple silicon (the Creator's Mac) only.** No Linux or Windows machine ran 5.3 in this sitting. What follows is what the sources said before the test.
- The sources do not say.

### Hand-test results — 2026-10-18

**Label for everything in this section: observed.** Run on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`, started from a terminal with no window (`-b --factory-startup -noaudio`), with its temporary folder and settings folders pointed at a work folder of its own, and driven by a script. The test list, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`; the entry numbers below are that list's.

*The Preview copy (entry 24).* Set up: the factory startup scene at 64 × 48, frames 1 to 3, `image_settings.use_preview = True`, first with `file_format = 'OPEN_EXR'` and then with `'PNG'`. Done: `bpy.ops.render.render(animation=True)` each time, then every written file's byte size and pixel size read. Blender printed:

```
--- OPEN_EXR, use_preview set to True, now reads True
  FILE anim_0001.exr            4180 bytes  64x48
  FILE anim_0001.jpg            1632 bytes  64x48
  FILE anim_0002.exr            4180 bytes  64x48
  FILE anim_0002.jpg            1632 bytes  64x48
  FILE anim_0003.exr            4180 bytes  64x48
  FILE anim_0003.jpg            1632 bytes  64x48
--- PNG, use_preview set to True, now reads True
  FILE anim_0001.png            4031 bytes  64x48
  FILE anim_0002.png            4031 bytes  64x48
  FILE anim_0003.png            4031 bytes  64x48
```

So the Preview copy has the same number of pixels as the main file and fewer bytes; it sits beside it with the same stem and a `.jpg` ending. With PNG the setting can be switched on and reads back as on, and no JPEG copy appears.

Then a blend-file was saved with OpenEXR output and Preview on, and one frame rendered from a terminal:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b -noaudio --factory-startup <folder>/preview-test.blend -f 1
```

Exit status `0`. Blender printed `Saved: '…/CLI/cli_0001.exr'` and `Saved: '…/CLI/cli_0001.jpg'`, and the folder held `cli_0001.exr` (2858 bytes) and `cli_0001.jpg` (1701 bytes).

*The Preview copy's colour (entry 24).* The 5.3 audit asked whether the copy looks like the render on screen or flat and linear. First, in the same run as above, under the scene's own view transform `AgX`, the centre pixel of the first frame's OpenEXR, of its JPEG copy, and of the PNG run's first frame, each read through Blender:

```
VIEW TRANSFORM of the scene: AgX
CENTRE EXR  anim_0001.exr (0.2686, 0.2686, 0.2688, 1.0) colorspace='Linear Rec.709'
CENTRE JPEG anim_0001.jpg (0.549, 0.549, 0.549, 1.0) colorspace='sRGB'
CENTRE PNG  anim_0001.png (0.2603, 0.2603, 0.2605, 1.0) colorspace='sRGB'
```

(That PNG was written at 16 bits, the depth left from the OpenEXR before it, and Blender reads a 16-bit PNG as floating point; the JPEG is read as stored bytes. So the numbers are not on one scale.) Second, a separate run with the view transform set to `False Color`, one frame, OpenEXR with Preview on and then an 8-bit PNG:

```
VIEW TRANSFORM: False Color
FILES OPEN_EXR ['fc_0001.exr', 'fc_0001.jpg']
FILES PNG ['fc_0001.png']
CENTRE EXR  fc_0001.exr (0.2686, 0.2686, 0.2688, 1.0) colorspace='Linear Rec.709' is_float=True
CENTRE JPEG fc_0001.jpg (0.3647, 1.0, 0.1843, 1.0) colorspace='sRGB' is_float=False
CENTRE PNG  fc_0001.png (0.4706, 1.0, 0.0, 1.0) colorspace='sRGB' is_float=False
```

Under False Color the OpenEXR stays grey and the JPEG copy comes out green, as the PNG does.

*The Preview copy with the window open (entry 42).* Recorded in full under question 10. In a Blender with its window open, a still rendered with `bpy.ops.render.render(write_still=True)` from the OpenEXR copy of the standard scene with Preview on wrote `still_.exr` and `still_.jpg`, as the terminal render of the same file wrote `term_0007.exr` and `term_0007.jpg`. A still started as the Render menu starts it, `bpy.ops.render.render('INVOKE_DEFAULT')`, wrote nothing to the output path at all — no OpenEXR and no JPEG — and the picture reached disk only when saved from the Image Editor with `bpy.ops.image.save_as(copy=True)`, which wrote the OpenEXR alone.

*Where the Preview control is offered, with the window open (entry 41).* Recorded in full under question 3. In the Output panel, with the format set in turn to PNG, JPEG, TIFF, OpenEXR and AVIF, a checkbox reading `Preview` appears only for `OpenEXR (.exr)`, under `Codec`; with `Media Type` at `Multi-Layer EXR` it appears below `Interleave`. Both were unticked.

*A half-size second copy written during the render (entry 25).* Set up: the factory startup scene at 64 × 48, one frame, main output PNG; a compositor group in the 5.3 effects stack sending its Group Input's Image to the Group Output and through a Scale node with X and Y at 0.5 into a File Output node writing PNG into the same folder (`directory` the folder, `file_name = "small_"`, format Media Type `IMAGE` then `PNG`, one item made with `file_output_items.new('RGBA', "small")`). Done: `bpy.ops.render.render(animation=True)`. Blender printed:

```
00:01.008  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e25/small_small0001.png'
00:01.009  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e25/main_0001.png'
SCALE NODE inputs: ['Image', 'Type', 'X', 'Y', 'Frame Type', 'Interpolation', 'Extension X', 'Extension Y']
SCALE NODE has no space property; Type socket reads: 'Relative'
NODES: ['NodeGroupInput', 'NodeGroupOutput', 'CompositorNodeScale', 'CompositorNodeOutputFile'] | file output inputs: ['small', '']
OPERATOR RESULT: {'FINISHED'}
FILE main_0001.png              3182 bytes  64x48  depth=32 is_float=False
FILE small_small0001.png        1905 bytes  32x24  depth=128 is_float=True
```

The same set-up with a Render Layers node as the source in place of the Group Input (Render Layers' Image to the Group Output and to the Scale node), run once more in 5.3, printed `FILE main_0001.png 3182 bytes 64x48 depth=32 is_float=False` and `FILE small_small0001.png 1905 bytes 32x24 depth=128 is_float=True`. So a half-size second file is written beside the main one with no window, named from the node's file name, then its item's name, then the frame number with nothing between them.

*Resizing a rendered picture and saving a copy, with no window (entry 26).* Set up: the factory startup scene rendered once to OpenEXR at 640 × 360 and loaded back with `bpy.data.images.load(...)`. Done, in order, with whatever each returned kept. Blender printed:

```
--- bpy.ops.image.resize(size=(320, 180)) ---
RAISED RuntimeError: Operator bpy.ops.image.resize.poll() failed, context is incorrect
--- image.scale(320, 180) ---
returned None
size now (320, 180)
--- bpy.ops.image.save_as(filepath='/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e26/small.png', save_as_render=True, copy=True) ---
RAISED RuntimeError: Operator bpy.ops.image.save_as.poll() failed, context is incorrect
small.png exists: False
--- image.save_render('/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e26/small_render.png') ---
returned None
--- scene format set to PNG, then image.save_render('/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e26/small_as_png.png') ---
returned None
FILE big.exr              95795 bytes  640x360
   file says: …/big.exr: OpenEXR image data, version 2, storage: scanline, compression: zip, dataWindow: (0 0)-(639 359), displayWindow: (0 0)-(639 359), lineOrder: increasing y
FILE small_as_png.png     48303 bytes  320x180
   file says: …/small_as_png.png: PNG image data, 320 x 180, 16-bit/color RGBA, non-interlaced
FILE small_render.png    105204 bytes  320x180
   file says: …/small_render.png: OpenEXR image data, version 2, storage: scanline, compression: zip, dataWindow: (0 0)-(319 179), displayWindow: (0 0)-(319 179), lineOrder: increasing y
```

The two Image Editor commands refuse to run with no window; the two plain calls work. The first `save_render` wrote an OpenEXR although its path ends `.png`, because it writes in the scene's own output format; after the scene's format was set to PNG it wrote a PNG.

*Proxies in a background run (entry 27).* Set up: three finished PNG frames at 128 × 96; an image strip built from them; `strip.use_proxy = True` and `strip.proxy.build_25 = True` with the other three sizes off; output PNG; the file saved. Blender printed `STRIP: Frames elements: 3 | use_proxy: True | build 25/50/75/100: True False False False | proxy_storage: PER_STRIP` and `BL_proxy folders before the render: []`. Done, in a terminal:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b -noaudio --factory-startup <folder>/proxy-test.blend -a
```

Exit status `0`. Beyond the version and quit lines, Blender printed `Rendering animation (frames 1..3)` and, for each frame, `Rendering frame N`, `Executing sequencer`, `Saved: '…/out/s_000N.png'` and a `Time:` line — nothing about proxies. A search of the whole test folder afterwards with `find <test folder> -name "BL_proxy*"` found nothing.

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

**Label: not in the sources** in the documentation. Some parts are observed in Cycles' own files or inferred from them, and are marked where they appear.

**Answer.**

- **The manual does not say what a missing image produces in a render, whether it feeds colour or anything else.** The manual only says that images feeding non-colour uses "should be marked as Non-Color Data", for example normal maps and displacement maps.
- **Observed in Cycles' own files:**
  - When an Image Texture node's image cannot be found, or a streamed tile fails to load, the node returns its own "missing" colour and "missing alpha". Both are node inputs.
  - When a tile has merely not loaded yet, it returns the texture's average colour.
  - When an image record has no pixel data, it returns black with alpha 0.
  - The constant magenta (1, 0, 1, alpha 1) is labelled "Color to use when images are not found". It is used directly as the fallback by the sky texture lookup and by the 3D (volume) image lookup.
  - For Image Texture nodes, the missing value comes from outside the folder. In the compiled OSL shader it defaults to `0 0 0` with alpha `1`.
  - So these files do not settle which value a missing texture gives.
- **Observed:** the conversion from sRGB to linear is applied after the lookup. It runs only when the node's flag for it is set, so it also applies to the missing value. Cycles' formula maps 0 to 0 and 1 to 1.
  - **Inferred:** a missing value made only of 0s and 1s, such as magenta or black, comes out the same whether the image is set to an sRGB colour space or to Non-Color.
- **Documented: how node connections convert values.**
  - "Between color and float – the color data is converted to its grayscale equivalent."
  - "Between color and vector – mapping between color channels and vector components."
- **Inferred: what a missing image would feed into a non-colour input.** These steps put the observed lookup together with the documented conversions.
  - If the Color output feeds a single-number input, such as roughness or a bump height, that input gets the grey equivalent of the missing colour. For black it gets 0. For magenta it gets a value between 0 and 1. The weights for "grayscale equivalent" are not documented.
  - If the Color output feeds a vector input, the vector is the missing colour's three channels: (1, 0, 1) for magenta, (0, 0, 0) for black.
  - If the Alpha output is used, it gives the missing alpha, which is 1 at the OSL default.
  - What a downstream node, such as a normal-map node, does with such a value is not in the sources.

**Evidence.**
- Manual, https://docs.blender.org/manual/en/5.3/render/color_management/color_spaces.html (read 2026-10-11): "A common situation where manual changes are needed is when working with or baking normal maps and displacement maps. Such maps do not actually store colors, just data encoded as colors. Those images should be marked as Non-Color Data."
- Manual, https://docs.blender.org/manual/en/5.3/interface/controls/nodes/parts.html (read 2026-10-11): "Between color and vector – mapping between color channels and vector components." and "Between color and float – the color data is converted to its grayscale equivalent."
- Manual, https://docs.blender.org/manual/en/5.3/render/shader_nodes/textures/image.html (read 2026-10-11): describes the node's inputs, properties and outputs, and says nothing about a missing image.
- Cycles' own files (read 2026-10-16). Observed:
  - `source/util/types_image.h`, lines 17–19: `/* Color to use when images are not found. */` / `#define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)`
  - `source/kernel/util/image.h`:
    - lines 342–343: `if (image_texture_id == KERNEL_IMAGE_NONE) {` / `return missing_rgba;`
    - line 361: `return (tile_descriptor == KERNEL_TILE_LOAD_FAILED) ? missing_rgba : tex.average_color;`
    - lines 377–378: `if (UNLIKELY(!info->data)) {` / `return zero_float4();`
  - `source/kernel/svm/image.h`:
    - lines 29 and 37–38: `float4 r = kernel_image_interp_with_udim(kg, sd, id, uv, missing_rgba);` … `if (flags & NODE_IMAGE_COMPRESS_AS_SRGB) {` / `r = color_srgb_to_linear_v4(r);`
    - lines 69–70: `const float3 missing = stack_load(stack, node.missing);` / `const float missing_alpha = stack_load(stack, node.missing_alpha);`
  - `source/kernel/svm/node_types.h`, lines 753–763, `struct SVMNodeTexImage`: it contains `SVMInputFloat3 missing;` and `SVMInputFloat missing_alpha;`.
  - `source/util/color.h`, lines 51–57, `color_srgb_to_linear`: `if (c < 0.04045f) { return (c < 0.0f) ? 0.0f : c * (1.0f / 12.92f); } return powf((c + 0.055f) * (1.0f / 1.055f), 2.4f);`. That gives 0 → 0 and 1 → 1.
  - `source/kernel/util/image_3d.h`, line 259: `return IMAGE_MISSING_RGBA;`
  - `source/kernel/svm/sky.h`, line 180: `kernel_image_interp(kg, sd, texture_id, dual2(make_float2(x, y)), IMAGE_MISSING_RGBA));`
  - `shader/node_image_texture.oso`, lines 12–13: `param	color	Missing	0 0 0` / `param	float	MissingAlpha	1`
  - `shader/node_environment_texture.oso`, lines 8–9: the same defaults.

**Where I looked.**
- Manual pages: Image Texture node, node sockets and conversion, implicit conversion node, colour spaces, Report/Find Missing Files, and Video Sequencer missing media.
- Searches of the manual, API and release notes for "missing", "not found", "pink", "magenta", "purple", "Non-Color" and "fallback".
- In Cycles' files: a search for "missing" and "IMAGE_MISSING" across every file. I read the image lookup, image node, node layout, colour conversion, 3D image and sky code, and the compiled image and environment shaders. I searched the Python files for "missing" and found nothing. The code that fills in each node's missing value is not in the folder.

**Not covered.**
- The value Blender actually hands Cycles for a missing image. *(Measured by the hand tests below, 2026-10-18: red 1, green 0, blue 1 from Color, 1 from Alpha.)*
- EEVEE and Workbench. *(Measured below for colour, a normal map, a bump and a displacement.)*
- What normal-map, bump and displacement nodes do with the result. *(Measured below.)*
- When an image record would have no pixel data. *(Not reached by the hand tests.)*

**Outside the interface.**
- **Observed, 2026-10-18 (entries 10 to 20 below): it holds with no window.** Every result below was produced with Blender started from a terminal with no window. The substitution happened, the render finished, the frame was written and every run ended with exit status 0; where standard error was captured on its own (entry 11) it was empty. The missing-files report runs with no window and prints to the ordinary output. What follows is what the sources said before the test.
- The sources do not say.

**Operating system.**
- **Observed, 2026-10-18, on macOS 26.6.2 on Apple silicon (the Creator's Mac) only, with Apple's own Metal graphics.** The processor and the Metal graphics gave the same numbers where both were run. No Linux or Windows machine, and no other make of graphics card, ran 5.3 in this sitting. What follows is what the sources said before the test.
- The sources do not say.
- **Observed:** CPU and GPU use the same failed-tile rule (`kernel/util/image.h` line 361 and `kernel/device/gpu/image.h` line 121).
- The Cycles files come from a macOS arm64 build.

### Hand-test results — 2026-10-18

**Label for everything in this section: observed.** Run on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`, started from a terminal with no window (`-b --factory-startup -noaudio`), with its temporary folder and settings folders pointed at a work folder of its own, and driven by a script. The test list, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`; the entry numbers below are that list's.

**The base scene for entries 10 to 17.** A 64 × 64 PNG with every pixel red 0, green 1, blue 0, alpha 1, called `flat.png`, and a second, every pixel white with alpha 0.25, called `alpha.png`, both written by this Blender. A plane of size 2 filling an orthographic camera (orthographic width 2, camera at height 5 looking straight down), with an Emission-only material fed by an Image Texture node; no world; Cycles, one sample, on the processor unless said otherwise, filter width 0 (it read back as 0.01), 32 × 32, view transform `Standard`, output OpenEXR at full float; the centre pixel read back out of the saved file. Each blend-file is saved with the picture beside it, so the path inside reads `//flat.png`. **Missing** always means the picture file was moved out of the folder after the blend-file was saved.

*The numbers a missing picture hands out (entry 10) — first run, with Cycles' denoising left as the new scene had it.* Three blend-files: **col** (the node's Color into Emission Color, strength 1), **alpha** (its Alpha into Emission Strength, colour white, on `alpha.png`), **grey** (its Color into Emission Strength, colour white — a colour output plugged into a socket that takes one number). Each rendered present and missing on the processor; col also on the Metal graphics and with the image's colour space set to `Non-Color`; col and grey also with Open Shading Language (`scene.cycles.shading_system = True`). Blender printed:

```
RESULT col_cpu_default present [0.0, 1.0010011196136475, 7.285794367817289e-07, 1.0]
RESULT alpha_cpu_default present [0.25060251355171204, 0.25131887197494507, 0.25067338347435, 1.0]
RESULT grey_cpu_default present [0.7141302824020386, 0.7156305909156799, 0.7144543528556824, 1.0]
RESULT col_gpu_default present [0.0, 1.0010011196136475, 7.285794367817289e-07, 1.0]
RESULT col_cpu_noncolor present [0.0, 1.0010011196136475, 7.285794367817289e-07, 1.0]
RESULT OSL col present [0.0, 1.0010011196136475, 7.285794367817289e-07, 1.0]
RESULT OSL grey present [0.7141608595848083, 0.7156622409820557, 0.7144878506660461, 1.0]
RESULT col_cpu_default missing [0.9959922432899475, 0.0, 0.9968982934951782, 1.0]
RESULT alpha_cpu_default missing [0.9983683824539185, 1.0000611543655396, 0.9982786774635315, 1.0]
RESULT grey_cpu_default missing [0.28446704149246216, 0.28530797362327576, 0.28442278504371643, 1.0]
RESULT col_gpu_default missing [0.9959922432899475, 0.0, 0.9968982934951782, 1.0]
RESULT col_cpu_noncolor missing [0.9959922432899475, 0.0, 0.9968982934951782, 1.0]
RESULT OSL col missing [0.9959922432899475, 0.0, 0.9968982934951782, 1.0]
RESULT OSL grey missing [0.28443318605422974, 0.2852758765220642, 0.28439009189605713, 1.0]
```

The scene this run built had `use_denoising True denoiser OPENIMAGEDENOISE`, read back from the saved col blend-file afterwards. With one sample, the numbers carry the denoiser's own small shifts.

*The same, second run, with `scene.cycles.use_denoising = False` and nothing else changed.* Blender printed:

```
RESULT col_cpu_default present [0.0, 1.0, 0.0, 1.0]
RESULT alpha_cpu_default present [0.2509803771972656, 0.2509803771972656, 0.2509803771972656, 1.0]
RESULT grey_cpu_default present [0.7151691317558289, 0.7151691317558289, 0.7151691317558289, 1.0]
RESULT col_gpu_default present [0.0, 1.0, 0.0, 1.0]
RESULT col_cpu_noncolor present [0.0, 1.0, 0.0, 1.0]
   shading_system reads back as: True
RESULT OSL col present [0.0, 1.0, 0.0, 1.0]
RESULT OSL grey present [0.7152000069618225, 0.7152000069618225, 0.7152000069618225, 1.0]
flat.png exists: False | alpha.png exists: False
RESULT col_cpu_default missing [1.0, 0.0, 1.0, 1.0]
RESULT alpha_cpu_default missing [1.0, 1.0, 1.0, 1.0]
RESULT grey_cpu_default missing [0.28483182191848755, 0.28483182191848755, 0.28483182191848755, 1.0]
RESULT col_gpu_default missing [1.0, 0.0, 1.0, 1.0]
RESULT col_cpu_noncolor missing [1.0, 0.0, 1.0, 1.0]
RESULT OSL col missing [1.0, 0.0, 1.0, 1.0]
RESULT OSL grey missing [0.2847999930381775, 0.2847999930381775, 0.2847999930381775, 1.0]
```

The picture-present lines are the control showing the files really were read: green, an alpha of 0.25098…, and 0.71517 for the colour fed into one number. With the picture missing: the node's Color output carried red 1, green 0, blue 1; its Alpha output carried 1; and a colour output plugged into a socket taking one number gave 0.28483182191848755 (0.2847999930381775 through Open Shading Language). The Metal graphics gave the same numbers as the processor, and `Non-Color` gave the same numbers as the default colour space. Both missing runs printed, for each missing render, `cycles | WARNING Image file …/flat.png does not exist.` (or `…/alpha.png`) and `cycles | ERROR Failed to load 1 image files`.

**Every entry from here to 17 was run with denoising off**, as this second run was.

*What a terminal render says, whether it finishes, and what it writes (entry 11, part a).* The col blend-file from the second run of entry 10 was copied into a fresh folder with `flat.png` beside it and rendered from a terminal with the ordinary output and the error output captured separately, once with the picture present and once with it moved away:

```
"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --background --factory-startup <folder>/col.blend --render-output <folder>/missing_ --render-frame 1 > missing.out 2> missing.err
```

Both runs ended with `EXIT STATUS: 0`, both wrote their frame, and standard error was 0 bytes in both. Comparing the two outputs with the timestamps cut, the missing-picture run printed exactly two lines the present run did not:

```
> cycles           | WARNING Image file /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e11/flat.png does not exist.
> cycles           | ERROR Failed to load 1 image files
```

Both sit on standard output. Both runs printed `Fra: 1 | Mem: 1M | Updating Images | Loading flat.png`, so that line is not the report. That the file really had not been read is shown by the frames:

```
CENTREPIXEL present_0001.exr [0.0, 1.0, 0.0, 1.0] bytes 1540
CENTREPIXEL missing_0001.exr [1.0, 0.0, 1.0, 1.0] bytes 1338
```

*`bpy.ops.file.report_missing_files()` with no window, and `has_data` (entry 11, part b).* The same blend-file opened with `--background --factory-startup` and a script that prints the images and their `has_data` flag, calls `bpy.ops.file.report_missing_files()`, prints what it returned, renders one frame and prints the flags again; run with the picture moved away, then present. With the picture moved away it printed:

```
STAGE1 images before any render: [('flat.png', '//flat.png', False)]
Warning: Path '/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e11/flat.png' not found, from local data-block 'IMflat.png'
OPERATOR report_missing_files RETURNED: {'FINISHED'}
STAGE2 images after the operator: [('flat.png', False)]
STAGE3 images after the render: [('flat.png', False), ('Render Result', False)]
```

and with it present:

```
STAGE1 images before any render: [('flat.png', '//flat.png', False)]
Info: No missing files
OPERATOR report_missing_files RETURNED: {'FINISHED'}
STAGE2 images after the operator: [('flat.png', False)]
STAGE3 images after the render: [('flat.png', False), ('Render Result', False)]
```

So the report reaches the ordinary output with no window and says different things in the two cases; `has_data` read `False` at every stage in both.

*EEVEE and Workbench (entry 12).* The same col blend-file, copied with `flat.png` into a fresh folder. `scene.render.engine` was set to three names in turn and what it read back printed; then the file rendered in EEVEE, and in Workbench twice — once with nothing else touched, and once with `scene.display.shading.color_type = 'TEXTURE'` and `scene.display.shading.light = 'FLAT'` — present, then with the picture moved away. Blender printed:

```
ENGINE NOT ACCEPTED BLENDER_EEVEE_NEXT TypeError('bpy_struct: item.attr = val: enum "BLENDER_EEVEE_NEXT" not found in (\'BLENDER_EEVEE\', \'BLENDER_WORKBENCH\', \'CYCLES\')')
ENGINE BLENDER_EEVEE ACCEPTED, reads back as BLENDER_EEVEE
ENGINE BLENDER_WORKBENCH ACCEPTED, reads back as BLENDER_WORKBENCH
RESULT col BLENDER_EEVEE present [0.0, 1.0, 0.0, 1.0]
   workbench before: color_type=MATERIAL light=STUDIO
RESULT bare     present [0.357666015625, 0.366943359375, 0.37060546875, 0.998046875]
   workbench after: color_type=TEXTURE light=FLAT
RESULT textured present [0.0, 0.99755859375, 0.0, 0.998046875]
flat.png exists: False
RESULT col BLENDER_EEVEE missing [1.0, 0.0, 1.0, 1.0]
   workbench before: color_type=MATERIAL light=STUDIO
RESULT bare     missing [0.357666015625, 0.366943359375, 0.37060546875, 0.998046875]
   workbench after: color_type=TEXTURE light=FLAT
RESULT textured missing [0.99755859375, 0.0, 0.99755859375, 0.998046875]
```

and, for the missing EEVEE render, `image.gpu | ERROR Image file "/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e12/flat.png" does not exist`. So EEVEE gives magenta for the missing picture. Workbench without those two settings gives the same grey whether the picture is there or not; with them, the present render gives the picture's green (the control) and the missing render magenta.

*Which kinds of absence give the same result (entry 13).* Seven copies of the col blend-file, each in its own folder, with the picture in a different state: **present**; **moved** (moved out of the folder); **badfolder** (the blend-file pointing at `<folder>/nosuchfolder/flat.png`); **zerobyte** (a `flat.png` of 0 bytes); **wrongext** (a working PNG renamed `flat.xyz`, the blend-file pointing at that name); **garbage** (a `flat.png` of 200 bytes of zeros); **permdenied** (the real `flat.png` with its permissions set to none, so this user cannot read it). Each rendered in its own windowless Blender with `-b --factory-startup -noaudio <folder>/col.blend --python e13.py -- <case>`. Every run ended with exit status `0`. Each printed:

```
CASE present    blend points at //flat.png | that path exists on disk: True | size: 376
RESULT present    [0.0, 1.0, 0.0, 1.0]
CASE moved      blend points at //flat.png | that path exists on disk: False | size: -
RESULT moved      [1.0, 0.0, 1.0, 1.0]
CASE badfolder  blend points at /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e13/badfolder/nosuchfolder/flat.png | that path exists on disk: False | size: -
RESULT badfolder  [1.0, 0.0, 1.0, 1.0]
CASE zerobyte   blend points at //flat.png | that path exists on disk: True | size: 0
RESULT zerobyte   [1.0, 0.0, 1.0, 1.0]
CASE wrongext   blend points at /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e13/wrongext/flat.xyz | that path exists on disk: True | size: 376
RESULT wrongext   [0.0, 1.0, 0.0, 1.0]
CASE garbage    blend points at //flat.png | that path exists on disk: True | size: 200
RESULT garbage    [1.0, 0.0, 1.0, 1.0]
CASE permdenied blend points at //flat.png | that path exists on disk: True | size: 376
RESULT permdenied [1.0, 0.0, 1.0, 1.0]
```

The lines each run printed about the picture, in each run's own words:

```
moved:      cycles | WARNING Image file …/e13/moved/flat.png does not exist.
            cycles | ERROR Failed to load 1 image files
badfolder:  cycles | WARNING Image file …/e13/badfolder/nosuchfolder/flat.png does not exist.
            cycles | ERROR Failed to load 1 image files
zerobyte:   cycles | WARNING Image file …/e13/zerobyte/flat.png failed to load.
            cycles | WARNING Image file …/e13/zerobyte/flat.png failed to load.
            cycles | ERROR Failed to load 1 image files
garbage:    cycles | WARNING Image file …/e13/garbage/flat.png failed to load.
            cycles | WARNING Image file …/e13/garbage/flat.png failed to load.
            cycles | ERROR Failed to load 1 image files
permdenied: cycles | WARNING Image file …/e13/permdenied/flat.png failed to load.
            cycles | WARNING Image file …/e13/permdenied/flat.png failed to load.
            cycles | ERROR Failed to load 1 image files
```

The present and wrongext runs printed no such line. Where the file is present but cannot be read, the WARNING appears twice. That the permission really bit was shown in the same terminal just before that run:

```
proof the permission bites — this user trying to read it:
cat: /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e13/permdenied/flat.png: Permission denied
(that command's exit status: 1)
----------  1 artist     wheel  376 Oct 18 03:57 /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e13/permdenied/flat.png
```

*One missing tile, one missing frame of a sequence, and a packed picture (entry 14).* Three set-ups on the base scene, in one windowless run. **Tiled:** `tile.1001.png` all green and `tile.1002.png` all blue, on an image with `source = 'TILED'` and its path set to `tile.<UDIM>.png`, holding tiles 1001 and 1002, on the plane with its texture coordinates stretched to run from 0 to 2 across the frame; rendered with both files present, then with `tile.1002.png` alone moved away, reading a pixel in the left quarter and one in the right quarter. **Sequence:** `seq.0001.png` green, `seq.0002.png` blue, `seq.0003.png` red, on an image with `source = 'SEQUENCE'` and a three-frame image user; frames 1, 2 and 3 rendered with all present, then with `seq.0002.png` alone moved away. **Packed:** the col scene with `bpy.ops.file.pack_all()` run before saving, then the picture file moved away, the blend-file opened again and rendered. Blender printed:

```
TILED IMAGE: source=TILED filepath=/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e14/tiled/tile.<UDIM>.png tiles=[1001, 1002]
RESULT present left quarter (tile 1001): [0.0, 1.0, 0.0, 1.0]  right quarter (tile 1002): [0.0, 0.0, 1.0, 1.0]
RESULT missing left quarter (tile 1001): [0.0, 1.0, 0.0, 1.0]  right quarter (tile 1002): [1.0, 0.0, 1.0, 1.0]
SEQUENCE IMAGE: source=SEQUENCE filepath=/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e14/sequence/seq.0001.png frame_duration=3
RESULT present frame 1 [0.0, 1.0, 0.0, 1.0]
RESULT present frame 2 [0.0, 0.0, 1.0, 1.0]
RESULT present frame 3 [1.0, 0.0, 0.0, 1.0]
RESULT missing frame 1 [0.0, 1.0, 0.0, 1.0]
RESULT missing frame 2 [1.0, 0.0, 1.0, 1.0]
RESULT missing frame 3 [1.0, 0.0, 0.0, 1.0]
IMAGE flat.png packed: True | path on disk exists: False
RESULT missing [0.0, 1.0, 0.0, 1.0]
```

with `cycles | WARNING Image file …/tiled/tile.1002.png does not exist.` and `cycles | WARNING Image file …/sequence/seq.0002.png does not exist.`, each followed by `cycles | ERROR Failed to load 1 image files`. The packed run printed no such line. Entry 13's moved case is the control showing that moving this same kind of file away otherwise gives magenta.

*Box projection and the world's Environment Texture (entry 15).* Set up: an empty scene; a cube of size 1 at x = −1, with an Emission material fed by an Image Texture node on `flat.png` whose `projection` is `BOX`, driven by Generated coordinates; a world whose Background is fed by an Environment Texture node on the same picture; an orthographic camera at height 5 looking down, orthographic width 4, so the cube sits in the left of the frame and the world shows on the right; the rest as the base scene. Rendered present and with the picture moved away, reading a pixel on the cube and one on the background. Blender printed:

```
CUBE TEXTURE projection: BOX | WORLD nodes: ['ShaderNodeBackground', 'ShaderNodeOutputWorld', 'ShaderNodeTexEnvironment']
image path in file: ['//flat.png']
RESULT present cube (box projection): [0.0, 1.0, 0.0, 1.0]  background (environment texture): [0.0, 1.0, 0.0, 1.0]
RESULT missing cube (box projection): [1.0, 0.0, 1.0, 1.0]  background (environment texture): [1.0, 0.0, 1.0, 1.0]
```

and, for the missing render, one `cycles | WARNING Image file …/e15/flat.png does not exist.` and one `cycles | ERROR Failed to load 1 image files`.

*A missing picture in the compositor (entry 16).* Set up: the base col scene, plus a compositor group holding an Image node (`CompositorNodeImage`) loading `flat.png`, its Image output into the Factor of a Color Ramp node left at its default black-to-white, and the ramp's Color into the Group Output. The group was put into the 5.3 effects stack in one blend-file (route 1) and assigned to the older `scene.compositing_node_group` setting in another (route 2). Each rendered present and with the picture moved away. Blender printed:

```
COMPOSITOR NODES: ['NodeGroupInput', 'NodeGroupOutput', 'CompositorNodeImage', 'ShaderNodeValToRGB']
RAMP INPUT LINKED: [('Image', 'Factor'), ('Color', 'Image')]
ROUTE 1: effects [('RampEffect', 'RampComp')]
ROUTE 2: effects ['Scene Effect'] | compositing_node_group: RampComp
image paths in file: [('flat.png', '//flat.png')]
RESULT compositor route1 present [0.7152172327041626, 0.7152172327041626, 0.7152172327041626, 1.0]
RESULT compositor route2 present [0.7152172327041626, 0.7152172327041626, 0.7152172327041626, 1.0]
RESULT compositor route1 missing [0.0, 0.0, 0.0, 1.0]
RESULT compositor route2 missing [0.0, 0.0, 0.0, 1.0]
```

The missing renders each printed one `cycles | WARNING Image file …/e16/flat.png does not exist.` and one `cycles | ERROR Failed to load 1 image files`; the plane in the same scene also uses `flat.png`, and nothing printed named the compositor. With the picture there, the ramp gave 0.7152 grey (the control); with it missing, black.

*A geometry-nodes Image Texture node driving position (entry 17).* Set up: an empty scene; a grid made with `bpy.ops.mesh.primitive_grid_add(x_subdivisions=3, y_subdivisions=3, size=2)`, which printed `GRID VERTICES: 16`; a Geometry Nodes modifier whose group feeds Position into an Image Texture node holding `flat.png`, and that node's Color into a Set Position node's Offset. The evaluated coordinates of the sixteen vertices printed with the picture present and with it moved away. Blender printed:

```
GN NODES: ['NodeGroupInput', 'NodeGroupOutput', 'GeometryNodeInputPosition', 'GeometryNodeImageTexture', 'GeometryNodeSetPosition']
image path in file: ['//flat.png']
PLAIN GRID POSITIONS: [(-1.0, -1.0, 0.0), (-0.33333, -1.0, 0.0), (0.33333, -1.0, 0.0), (1.0, -1.0, 0.0), (-1.0, -0.33333, 0.0), (-0.33333, -0.33333, 0.0), (0.33333, -0.33333, 0.0), (1.0, -0.33333, 0.0), (-1.0, 0.33333, 0.0), (-0.33333, 0.33333, 0.0), (0.33333, 0.33333, 0.0), (1.0, 0.33333, 0.0), (-1.0, 1.0, 0.0), (-0.33333, 1.0, 0.0), (0.33333, 1.0, 0.0), (1.0, 1.0, 0.0)]
POSITIONS present [(-1.0, 0.0, 0.0), (-0.33333, 0.0, 0.0), (0.33333, 0.0, 0.0), (1.0, 0.0, 0.0), (-1.0, 0.66667, 0.0), (-0.33333, 0.66667, 0.0), (0.33333, 0.66667, 0.0), (1.0, 0.66667, 0.0), (-1.0, 1.33333, 0.0), (-0.33333, 1.33333, 0.0), (0.33333, 1.33333, 0.0), (1.0, 1.33333, 0.0), (-1.0, 2.0, 0.0), (-0.33333, 2.0, 0.0), (0.33333, 2.0, 0.0), (1.0, 2.0, 0.0)]
POSITIONS missing [(-1.0, -1.0, 0.0), (-0.33333, -1.0, 0.0), (0.33333, -1.0, 0.0), (1.0, -1.0, 0.0), (-1.0, -0.33333, 0.0), (-0.33333, -0.33333, 0.0), (0.33333, -0.33333, 0.0), (1.0, -0.33333, 0.0), (-1.0, 0.33333, 0.0), (-0.33333, 0.33333, 0.0), (0.33333, 0.33333, 0.0), (1.0, 0.33333, 0.0), (-1.0, 1.0, 0.0), (-0.33333, 1.0, 0.0), (0.33333, 1.0, 0.0), (1.0, 1.0, 0.0)]
```

With the picture present every vertex moved by +1 in y, the picture's green. With it missing the coordinates were the plain grid exactly, so the offset received was 0, 0, 0. Nothing was printed about the missing file in this run.

*A missing picture feeding a normal map, a bump height and a displacement height (entry 18).* Set up, every number recorded: a 64 × 64 black-and-white checker of 8-pixel squares, colour space `Non-Color`; three UV spheres of 128 segments by 64 rings, radius 0.4, shaded smooth, at x = −1.5, 0 and +1.5; a world whose background is black; one sun lamp at angle 0, energy 3, turned 45° about X and 30° about Z; an orthographic camera at (0, 0, 5), orthographic width 4.5; the frame 240 × 80 with a transparent film; Cycles at 4 samples on the processor, denoising off, every bounce count 0; OpenEXR at full float. Each sphere carries a Diffuse BSDF of colour 0.5, 0.5, 0.5 and roughness 0. The left one takes the picture through a Normal Map node into the Diffuse's Normal; the middle one through a Bump node's Height with Distance 5; the right one through a Displacement node's Height at Scale 0.4 into the Material Output's Displacement, with the material's displacement method set to `DISPLACEMENT`. Four renders of the same file: **present**; **missing** (the picture moved out of the folder); **unwired** (every link out of each Image Texture node removed); **nonode** (as unwired, with the Normal Map, Bump and Displacement nodes removed too, so each sphere is plain). The same four again in EEVEE. Measured over each third of the frame: the mean and the largest difference between two renders, taken over all four channels (red, green, blue and alpha) of every pixel in that third; and the number of pixels whose alpha is above 0.5. Blender printed:

```
Normal Map node default Color: (0.5, 0.5, 1.0, 1.0)
Bump node default Height: 1.0
Displacement node default Height: 0.0 Midlevel: 0.5
displacement_method set on: [('Material', 'DISPLACEMENT')]
image path in file: ['//checker.png']
```

and, for every render in both engines, `EDGE PIXELS WITH ALPHA > 0 … {'top': 0, 'bottom': 0, 'left': 0, 'right': 0}` — nothing touched the frame's edge. Then:

```
CYCLES DIFF normalmap  present_vs_unwired  mean 0.029426  max 0.476329
CYCLES DIFF normalmap  missing_vs_unwired  mean 0.023413  max 0.385943
CYCLES DIFF normalmap  unwired_vs_nonode   mean 0.000000  max 0.000000
CYCLES DIFF normalmap  missing_vs_nonode   mean 0.023413  max 0.385943
CYCLES DIFF normalmap  missing_vs_present  mean 0.017833  max 0.425671
CYCLES DIFF bump       present_vs_unwired  mean 0.008011  max 0.474188
CYCLES DIFF bump       missing_vs_unwired  mean 0.000000  max 0.000000
CYCLES DIFF bump       unwired_vs_nonode   mean 0.000000  max 0.000000
CYCLES DIFF bump       missing_vs_nonode   mean 0.000000  max 0.000000
CYCLES DIFF bump       missing_vs_present  mean 0.008011  max 0.474188
CYCLES DIFF displace   present_vs_unwired  mean 0.177992  max 1.000000
CYCLES DIFF displace   missing_vs_unwired  mean 0.035524  max 1.000000
CYCLES DIFF displace   unwired_vs_nonode   mean 0.072746  max 1.000000
CYCLES DIFF displace   missing_vs_nonode   mean 0.037353  max 1.000000
CYCLES DIFF displace   missing_vs_present  mean 0.156113  max 1.000000
CYCLES SILHOUETTE normalmap  present 1416  unwired 1416  nonode 1416  missing 1416
CYCLES SILHOUETTE bump       present 1415  unwired 1415  nonode 1415  missing 1415
CYCLES SILHOUETTE displace   present 3045  unwired 348  nonode 1424  missing 868
BLENDER_EEVEE DIFF normalmap  present_vs_unwired  mean 0.033556  max 0.474282
BLENDER_EEVEE DIFF normalmap  missing_vs_unwired  mean 0.022668  max 0.382111
BLENDER_EEVEE DIFF normalmap  unwired_vs_nonode   mean 0.000000  max 0.000000
BLENDER_EEVEE DIFF normalmap  missing_vs_nonode   mean 0.022668  max 0.382111
BLENDER_EEVEE DIFF normalmap  missing_vs_present  mean 0.022959  max 0.437965
BLENDER_EEVEE DIFF bump       present_vs_unwired  mean 0.010596  max 0.436890
BLENDER_EEVEE DIFF bump       missing_vs_unwired  mean 0.000000  max 0.000000
BLENDER_EEVEE DIFF bump       unwired_vs_nonode   mean 0.000000  max 0.000000
BLENDER_EEVEE DIFF bump       missing_vs_nonode   mean 0.000000  max 0.000000
BLENDER_EEVEE DIFF bump       missing_vs_present  mean 0.010596  max 0.436890
BLENDER_EEVEE DIFF displace   present_vs_unwired  mean 0.165763  max 1.000000
BLENDER_EEVEE DIFF displace   missing_vs_unwired  mean 0.035434  max 1.000000
BLENDER_EEVEE DIFF displace   unwired_vs_nonode   mean 0.072542  max 1.000000
BLENDER_EEVEE DIFF displace   missing_vs_nonode   mean 0.037247  max 1.000000
BLENDER_EEVEE DIFF displace   missing_vs_present  mean 0.144200  max 1.000000
BLENDER_EEVEE SILHOUETTE normalmap  present 1433  unwired 1433  nonode 1433  missing 1433
BLENDER_EEVEE SILHOUETTE bump       present 1433  unwired 1433  nonode 1433  missing 1433
BLENDER_EEVEE SILHOUETTE displace   present 3076  unwired 360  nonode 1433  missing 880
```

The missing renders printed `cycles | WARNING Image file …/e18/checker.png does not exist.`, `cycles | ERROR Failed to load 1 image files` and `image.gpu | ERROR Image file "…/e18/checker.png" does not exist`.

**The controls.** `present_vs_unwired` is the first: each wiring changes the picture when the file is there. `unwired_vs_nonode` for the normal map is the second: a Normal Map node holding its own default of 0.5, 0.5, 1 gives a picture identical to no Normal Map node at all, so "unchanged" has a fixed meaning. **With the picture missing, in both engines:** the normal-map sphere differs from the plain one (mean 0.023413, largest 0.385943 in Cycles); the bump sphere is identical to the nothing-connected and the plain sphere, to every digit; and the displaced sphere's outline holds 868 pixels in Cycles (880 in EEVEE), against 3045 present, 348 with nothing connected and 1424 plain, while the other two spheres hold 1416 and 1415 in every state.

*A missing picture feeding roughness (entry 19).* No earlier version ran this. Set up: an empty scene with a black world; one sun lamp of energy 3, turned 30° about X and 20° about Z; a UV sphere of radius 1, 64 segments by 32 rings, shaded smooth; an orthographic camera at (0, 0, 5), width 2.4; 64 × 64; Cycles on the processor, 16 samples, seed 0 and not animated, denoising off; OpenEXR at full float. The sphere's material is a Glossy BSDF of colour white whose Roughness is fed by the Color output of an Image Texture node on a 64 × 64 picture of flat grey 0.8, colour space `Non-Color`. Four renders: **present**; **fixedA** — the link removed and Roughness typed as 0.28483182191848755, the single number entry 10 printed for a missing picture's colour fed into one number; **fixedB** — the link removed and Roughness typed as 0.8; **missing** — the picture moved away. Measured: the mean and largest difference between two renders over the red, green and blue of every pixel.

The first run stopped before rendering anything but the present case, on the script's own way of finding the node again:

```
    g = [n for n in nt.nodes if n.bl_idname == 'ShaderNodeBsdfGlossy'][0]
IndexError: list index out of range
```

The node was then given a name and found by it, and the run repeated. It printed:

```
GLOSSY NODE bl_idname as created: ShaderNodeBsdfAnisotropic | type: BSDF_GLOSSY
image path in file: ['//grey.png'] | present picture value read back: [0.8000000715255737, 0.8000000715255737, 0.8000000715255737, 1.0]
   fixedA: Roughness unlinked and typed as 0.28483182191848755, reads back 0.28483182191848755
   fixedB: Roughness unlinked and typed as 0.8, reads back 0.800000011920929
DIFF missing  vs fixedA  mean 0.000000  max 0.000000
DIFF present  vs fixedB  mean 0.000000  max 0.000003
DIFF present  vs fixedA  mean 0.283819  max 32.909518
DIFF missing  vs fixedB  mean 0.283819  max 32.909515
DIFF missing  vs present mean 0.283819  max 32.909518
```

with `cycles | WARNING Image file …/e19/grey.png does not exist.` and `cycles | ERROR Failed to load 1 image files` for the missing render. So the missing render is identical, to every pixel, to the render with Roughness typed as 0.28483182191848755, and the present render to the one with Roughness 0.8; the two pairs differ from each other by up to 32.9 in a channel (the sun's highlight).

*A Volume object whose OpenVDB file is gone (entry 20).* First the file: an icosphere of radius 0.5 at the origin, `bpy.ops.object.quick_smoke()`, the domain's cache pointed into the work folder with its data format set to `OPENVDB`, frames 1 to 4, resolution maximum 32, then `bpy.ops.fluid.bake_data()`; the frame-1 file copied beside the blend-file as `vol.vdb`. Then the scene: a Volume data-block pointing at `//vol.vdb` on an object at (0, 0, 0), rotation 0, scale 1; a Principled Volume material with Emission Strength 1 and Emission Colour white, the density attribute left at the node's own default; an orthographic camera at (0, 0, 6), no rotation, orthographic width 4; 64 × 64, the film **not** transparent; a world with its nodes switched off and its colour black; Cycles at 128 samples on the processor, denoising off. Three renders: file present; the Volume object deleted; the file moved out of the folder. Measured: the mean and the largest value over the red, green and blue of every pixel. Blender printed:

```
QUICK SMOKE RETURNED {'FINISHED'}
BAKE RETURNED {'FINISHED'}
CACHE FILE config/config_0001.uni 88
CACHE FILE data/fluid_data_0001.vdb 57570
world.use_nodes read back straight after setting False: True
VOLUME OBJECT dimensions with the file loaded: (3.75, 3.75, 5.0)
volume path in file: ['//vol.vdb']
RESULT present   mean 0.21875603743137617  max 1.7763690948486328
RESULT novolume  mean 0.049999967217445374  max 0.049999967217445374
RESULT missing   mean 0.049999967217445374  max 0.049999967217445374
```

Setting the world's nodes off did not take — it read back `True` — so the empty frame carries the world's own default grey rather than black. The file-present render against the object-deleted render is the control, and it holds. With the file gone, the frame matched the render with no Volume object at all, to every digit printed. A search of the whole run's output for `ERROR`, `WARNING`, `Error` and `Traceback` matched no line.

---

## 10. What does a finished picture hold, in every part of the file, about what produced it?

**Label: documented** for which origin fields can be written and which formats hold them. **Not in the sources** for whether every part of a multi-part file carries them, and for whether Blender's name or version is written.

**The question is ambiguous.** It could mean:

- **(a)** Everything, anywhere in the file, that records what produced the picture.
- **(b)** Whether that record is repeated in each part of a file made of several parts: each layer, part or view of an OpenEXR, or each frame of a video.

I answer both.

**Answer.**

**Reading (a). What can be recorded:**

- **Render metadata ("stamp") fields**, each switched on or off in the Metadata panel. The API default is given in brackets.

  | Field | Default |
  | --- | --- |
  | Date | on |
  | Time (frame timecode) | on |
  | Render Time | on |
  | Frame | on |
  | Frame Range | off |
  | Memory (peak) | on |
  | Hostname | off |
  | Camera | on |
  | Lens | off |
  | Scene | on |
  | Marker | off |
  | Filename (of the .blend) | on |
  | Strip Name | off |
  | Note (custom text) | off |

  - The manual's panel picture is documented. It shows the same ticks, except that Memory is unticked in the picture while the API default is on.
  - The metadata can be taken from the scene or from the Sequencer strips.
- **Engine-specific fields.** A render engine can add its own fields ("Add engine-specific stamp data to the result").
  - **Observed:** Cycles' Python files never call this. Whether Cycles' compiled part does cannot be checked from the folder.
- **Pixel density** ("meta-data written to supported image formats").
- **Colour information:**
  - Colour-space metadata.
  - An ICC profile on sRGB files (5.3).
  - Mastering-display luminance for HDR output (see answer 6).
- **The pixels themselves.** "Burn into Image" will "Add metadata as text to the render".
- **Which formats can hold the metadata fields.** The format table marks Metadata ✓ for JPEG, OpenEXR, PNG and WebP, and ✗ for AVIF, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw and TIFF. "When using formats that do not support metadata, render information is not preserved in the saved file."
- **Video.** The API describes these fields as going into "image/video metadata": date, filename, frame range, note and scene. It describes the others as "image metadata": camera, frame, lens, marker, memory, render time, strip and time. Hostname is described as neither. **Inferred:** only the first group is documented as being written into video files.
- **Blender's own name or version** written into the file is not mentioned anywhere in the sources.

**Reading (b).**
- The sources do not say whether metadata is written into each part, layer or view of a multi-part OpenEXR, or into each frame of a video.
- They say only that a video container "wraps video, audio, and metadata into a single file".

**Evidence.**
- Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html (read 2026-10-11):
  - "The Metadata panel includes options for writing metadata into render output."
  - "Only some image formats support metadata".
  - The Include list, from "Date … Includes the current date and time." to "Note … Includes a custom note."
  - "Hostname … Includes the rendering machine’s hostname."
  - "Burn into Image … Add metadata as text to the render."
  - "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it, such as an identifier for the render node or the job number."
  - The picture `_images/render_output_properties_metadata_panel.png` on this page is documented. It shows Date, Time, Render Time, Frame, Camera, Scene and Filename ticked. Frame Range, Memory, Hostname, Lens, Marker, Strip Name and a "Use Strip Me…" box are unticked. Note and Burn Into Image are unticked, and Include Labels is ticked.
- API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html (read 2026-10-11):
  - `use_stamp_date` "Include the current date in image/video metadata (default True)"
  - `use_stamp_memory` "Include the peak memory usage in image metadata (default True)"
  - `use_stamp_hostname` "Include the hostname of the machine that rendered the frame (default False)"
  - `use_stamp` "Render the stamp info text in the rendered image (default False)"
  - `metadata_input` "SCENE … STRIPS"
  - `ppm_factor` "The pixel density meta-data written to supported image formats."
  - The other `use_stamp_*` entries read the same way. Those for filename, frame range, note and scene say "image/video metadata". Those for camera, frame, lens, marker, render time, sequencer strip and time say "image metadata".
- API, https://docs.blender.org/api/5.3/bpy.types.RenderResult.html (read 2026-10-11): "stamp_data_add_field(field, value) Add engine-specific stamp data to the result".
- Manual, https://docs.blender.org/manual/en/5.3/files/media/image_formats.html (read 2026-10-11):
  - The format table's Metadata column.
  - "When using formats that do not support metadata, render information is not preserved in the saved file."
- Manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html (read 2026-10-11): "The pixel-density is meta-data which doesn’t impact the quality of the image."
- Manual, https://docs.blender.org/manual/en/5.3/render/color_management/system_configuration.html (read 2026-10-11): "saves images and videos with color space metadata".
- Release notes, https://developer.blender.org/docs/release_notes/5.3/pipeline_io/ (read 2026-10-11): "Saving sRGB image files now includes an ICC profile."
- Manual, https://docs.blender.org/manual/en/5.3/files/media/video_formats.html (read 2026-10-11): "A container – wraps video, audio, and metadata into a single file."
- Cycles' own files (read 2026-10-16), observed: a search for "stamp" across the Python and header files returned no matches.

**Where I looked.**
- Manual pages: metadata (and its picture), image formats, Output properties, colour management (system configuration and OpenColorIO), video formats, and the Image Editor and Video Sequencer metadata displays.
- API pages: RenderSettings, RenderResult and Scene.
- All 5.3 release notes.
- Searches for "metadata", "stamp", "hostname", "render time", "peak memory", "software", "version", "header", "ICC", "interop" and "chromaticit".

**Not covered.** *(Every item below was measured by the hand tests further down, 2026-10-18, for PNG, JPEG, OpenEXR and an MPEG-4 movie; what a reader other than Blender makes of the fields was not tested.)*
- The exact key names the fields are stored under in each format.
- Whether OpenEXR multi-part files repeat the metadata in every part.
- Whether video files carry it per frame or once.
- Whether Blender's version is written.
- Whether Cycles adds engine fields.
- Whether Cryptomatte information is stored as metadata.

**Outside the interface.**
- **Observed, 2026-10-18 (entries 28 to 39 with no window, entry 42 with the window open): it holds.** Every file was written by a render driven from a terminal or a script, and in entry 42 the same blend-files were rendered both with Blender's window open and with `-b` and no window: the files carry the same field names in the same order, and a picture saved by hand from the Image Editor carries them too. A still started the way the Render menu starts it writes nothing to the output path until it is saved by hand. What follows is what the sources said before the test.
- All metadata settings are API properties, so a script can set them. The manual's own render-farm hint is to "script any information you like" into the Note.
- Viewing metadata in the Image Editor ("Show Metadata") is interface-only.
- The sources do not say that what is written differs outside the interface.

**Operating system.**
- **Observed, 2026-10-18, on macOS 26.6.2 on Apple silicon (the Creator's Mac) only.** `File` held this Mac's own path and `Hostname` read `Artists-Studio-MacBook.local`, exactly what its `hostname` command prints. No Linux or Windows machine ran 5.3 in this sitting, so those stay untried. What follows is what the sources said before the test.
- The sources do not say.
- The Hostname field records the rendering machine's name, so its value differs from machine to machine.

### Hand-test results — 2026-10-18

**Label for everything in this section: observed.** Run on the Creator's Mac — an Apple M1 Pro with its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2, whose `hostname` prints `Artists-Studio-MacBook.local` — in Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`, started from a terminal with no window (`-b --factory-startup -noaudio`) unless said otherwise, with its temporary folder and settings folders pointed at a work folder of its own, and driven by a script. The test list, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`; the entry numbers below are that list's. Files were read with `exiftool -a -G1 -s` (version 13.55), and OpenEXR files also with a header reader written for these tests that walks every part and prints each part's attributes in alphabetical order.

**The standard scene.** The factory startup scene with the scene renamed `SMZSCENE`, the camera renamed `SMZCAM` and its lens set to 37.5, a marker `SMZMARK` on frame 7, frames 7 to 9 with frame 7 current, 64 × 64, Cycles at one sample, every `use_stamp_*` switch on, the note text `SMZNOTE-12345`, Burn into Image off, saved as `smzfile.blend`. The run printed the switches as:

```
SWITCHES: [('use_stamp_time', True), ('use_stamp_date', True), ('use_stamp_frame', True), ('use_stamp_frame_range', True), ('use_stamp_camera', True), ('use_stamp_lens', True), ('use_stamp_scene', True), ('use_stamp_note', True), ('use_stamp_marker', True), ('use_stamp_filename', True), ('use_stamp_sequencer_strip', True), ('use_stamp_render_time', True), ('use_stamp_labels', True), ('use_stamp_memory', True), ('use_stamp_hostname', True)]
```

*The names the fields carry inside a file (entry 28).* Frame 7 written to PNG. Everything `exiftool` reported in the PNG group beyond the picture's own structure and colour lines:

```
[PNG]           File                            : /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e28/smzfile.blend
[PNG]           Note                            : SMZNOTE-12345
[PNG]           Date                            : 2026/10/18 04:12:12
[PNG]           Marker                          : SMZMARK
[PNG]           Time                            : 00:00:00:07
[PNG]           Frame                           : 7
[PNG]           FrameRange                      : 7:9
[PNG]           Camera                          : SMZCAM
[PNG]           Lens                            : 37.50
[PNG]           Scene                           : SMZSCENE
[PNG]           Strip                           : <none>
[PNG]           RenderTime                      : 00:00.11
[PNG]           Memory                          : 1M
[PNG]           Hostname                        : Artists-Studio-MacBook.local
[PNG]           CyclesViewLayersamples          : 1
[PNG]           CyclesViewLayertotal_time       : 00:00.11
[PNG]           CyclesViewLayerrender_time      : 00:00.09
[PNG]           CyclesViewLayersynchronization_time: 00:00.00
```

The same frame written to JPEG holds them as comment markers, `exiftool` reading:

```
[File]          Comment                         : Blender:File:/private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e28/smzfile.blend
[File]          Comment                         : Blender:Note:SMZNOTE-12345
[File]          Comment                         : Blender:Date:2026/10/18 04:12:12
[File]          Comment                         : Blender:Marker:SMZMARK
[File]          Comment                         : Blender:Time:00:00:00:07
[File]          Comment                         : Blender:Frame:7
[File]          Comment                         : Blender:FrameRange:7:9
[File]          Comment                         : Blender:Camera:SMZCAM
[File]          Comment                         : Blender:Lens:37.50
[File]          Comment                         : Blender:Scene:SMZSCENE
[File]          Comment                         : Blender:Strip:<none>
[File]          Comment                         : Blender:RenderTime:00:00.04
[File]          Comment                         : Blender:Memory:1M
[File]          Comment                         : Blender:Hostname:Artists-Studio-MacBook.local
[File]          Comment                         : Blender:cycles.ViewLayer.samples:1
[File]          Comment                         : Blender:cycles.ViewLayer.total_time:00:00.04
[File]          Comment                         : Blender:cycles.ViewLayer.render_time:00:00.04
[File]          Comment                         : Blender:cycles.ViewLayer.synchronization_time:00:00.00
```

and the JPEG also carries an sRGB ICC profile (`[ICC_Profile] ProfileDescription : sRGB`, `ProfileCopyright : CC0`). In a single-layer OpenEXR they are separate header attributes:

```
  [Camera] (string, 6 bytes) = SMZCAM
  [Date] (string, 19 bytes) = 2026/10/18 04:12:12
  [File] (string, 74 bytes) = /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e28/smzfile.blend
  [Frame] (string, 1 bytes) = 7
  [FrameRange] (string, 3 bytes) = 7:9
  [Hostname] (string, 28 bytes) = Artists-Studio-MacBook.local
  [Lens] (string, 5 bytes) = 37.50
  [Marker] (string, 7 bytes) = SMZMARK
  [Memory] (string, 2 bytes) = 1M
  [Note] (string, 13 bytes) = SMZNOTE-12345
  [RenderTime] (string, 8 bytes) = 00:00.04
  [Scene] (string, 8 bytes) = SMZSCENE
  [Software] (string, 18 bytes) = Blender 5.3.0 Beta
  [Strip] (string, 6 bytes) = <none>
  [Time] (string, 11 bytes) = 00:00:00:07
  [cycles.ViewLayer.render_time] (string, 8 bytes) = 00:00.04
  [cycles.ViewLayer.samples] (string, 1 bytes) = 1
  [cycles.ViewLayer.synchronization_time] (string, 8 bytes) = 00:00.00
  [cycles.ViewLayer.total_time] (string, 8 bytes) = 00:00.04
  (31 attributes in part 0)
```

and the multi-layer OpenEXR (Media Type set to `MULTI_LAYER_IMAGE` first) carries the same nineteen in its part 0, plus `[BlenderMultiChannel] (string, 25 bytes) = Blender V2.55.1 and newer`, with `[name] (string, 18 bytes) = ViewLayer.Combined` (33 attributes in part 0).

*Which formats keep them (entry 28).* The same frame was also written to AVIF, BMP, Iris, JPEG 2000, Targa, Targa Raw, Cineon, DPX, Radiance HDR, TIFF and WebP. For each, the number of `exiftool` lines naming a Blender field (any of the scene's own `SMZ` values, `Blender`, `37.50`, `FrameRange`, `Hostname`, the Cycles fields, or the host's name), and a byte search of the file for `SMZNOTE`, `SMZSCENE`, `SMZCAM` and `SMZMARK`:

```
f_AVIF.avif: exiftool lines naming a Blender field: 0 | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
f_BMP.bmp: exiftool lines naming a Blender field: 0 | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
f_IRIS.rgb: exiftool lines naming a Blender field: 0 | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
f_JPEG2000.jp2: exiftool lines naming a Blender field: 0 | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
f_TARGA.tga: exiftool lines naming a Blender field: 0 | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
f_TARGA_RAW.tga: exiftool lines naming a Blender field: 0 | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
f_CINEON.cin: exiftool lines naming a Blender field: 0 | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
f_DPX.dpx: exiftool lines naming a Blender field:  | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
f_HDR.hdr: exiftool lines naming a Blender field: 0 | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
f_TIFF.tif: exiftool lines naming a Blender field: 0 | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
f_WEBP.webp: exiftool lines naming a Blender field: 0 | byte search for SMZNOTE/SMZSCENE/SMZCAM/SMZMARK: 0
```

(The DPX count came back empty from that command; `exiftool` run on the DPX file alone printed only the format's own header fields — among them `Creator : OpenImageIO 3.1.16.0 http://www.openimageio.org` — and no Blender field.) So PNG, JPEG and OpenEXR, single-layer and multi-layer, keep all fourteen and the Cycles fields; AVIF, BMP, Iris, JPEG 2000, Targa, Targa Raw, Cineon, DPX, Radiance HDR, TIFF and WebP keep none, WebP included although the manual's table ticks it. Nothing was printed in Blender's output about a format that could not carry them.

*An untouched blend-file (entry 28).* The factory startup scene saved once as `untouched.blend` with nothing changed, its switches printing `UNTOUCHED switches on: ['use_stamp_time', 'use_stamp_date', 'use_stamp_frame', 'use_stamp_camera', 'use_stamp_scene', 'use_stamp_filename', 'use_stamp_render_time', 'use_stamp_labels'] | engine: BLENDER_EEVEE`, rendered to PNG. Its fields:

```
[PNG]           File                            : /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e28/untouched.blend
[PNG]           Date                            : 2026/10/18 04:12:14
[PNG]           Time                            : 00:00:00:01
[PNG]           Frame                           : 001
[PNG]           Camera                          : Camera
[PNG]           Scene                           : Scene
[PNG]           RenderTime                      : 00:01.02
```

*With every switch off (entry 29).* The standard scene with every `use_stamp_*` switch set to `False` (the run printed all fifteen as `False`), written to OpenEXR, multi-layer OpenEXR, PNG, JPEG, WebP and AVIF. The OpenEXR's header still held, among its 17 attributes:

```
  [Software] (string, 18 bytes) = Blender 5.3.0 Beta
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
  [cycles.ViewLayer.render_time] (string, 8 bytes) = 00:00.05
  [cycles.ViewLayer.samples] (string, 1 bytes) = 1
  [cycles.ViewLayer.synchronization_time] (string, 8 bytes) = 00:00.00
  [cycles.ViewLayer.total_time] (string, 8 bytes) = 00:00.05
```

and the multi-layer one, among 19 in its part 0, the same plus `[BlenderMultiChannel] (string, 25 bytes) = Blender V2.55.1 and newer` and `[name] (string, 18 bytes) = ViewLayer.Combined`; its second part is `ViewLayer.Noisy Image`. The PNG held, beyond its structure and colour lines, only `CyclesViewLayersamples : 1`, `CyclesViewLayertotal_time : 00:00.04`, `CyclesViewLayerrender_time : 00:00.04`, `CyclesViewLayersynchronization_time: 00:00.00`, and the pixel-density lines; the JPEG only the four comments `Blender:cycles.ViewLayer.samples:1`, `Blender:cycles.ViewLayer.total_time:00:00.04`, `Blender:cycles.ViewLayer.render_time:00:00.04`, `Blender:cycles.ViewLayer.synchronization_time:00:00.00`. The WebP and the AVIF held no Blender line (the WebP reads `WebP_Flags : ICC Profile`). So only OpenEXR names Blender and its version, with every switch off; nothing anywhere named the render engine, the resolution or the view transform.

*Cycles against EEVEE (entry 29).* The standard scene, switches on, rendered once in Cycles and once in EEVEE to PNG. The fields:

```
===== on_CYCLES.png
[PNG]           File                            : /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e29/smzfile_CYCLES.blend
[PNG]           Note                            : SMZNOTE-12345
[PNG]           Date                            : 2026/10/18 04:13:41
[PNG]           Marker                          : SMZMARK
[PNG]           Time                            : 00:00:00:07
[PNG]           Frame                           : 7
[PNG]           FrameRange                      : 7:9
[PNG]           Camera                          : SMZCAM
[PNG]           Lens                            : 37.50
[PNG]           Scene                           : SMZSCENE
[PNG]           Strip                           : <none>
[PNG]           RenderTime                      : 00:00.04
[PNG]           Memory                          : 1M
[PNG]           Hostname                        : Artists-Studio-MacBook.local
[PNG]           CyclesViewLayersamples          : 1
[PNG]           CyclesViewLayertotal_time       : 00:00.04
[PNG]           CyclesViewLayerrender_time      : 00:00.04
[PNG]           CyclesViewLayersynchronization_time: 00:00.00
===== on_BLENDER_EEVEE.png
[PNG]           File                            : /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e29/smzfile_BLENDER_EEVEE.blend
[PNG]           Note                            : SMZNOTE-12345
[PNG]           Date                            : 2026/10/18 04:13:41
[PNG]           Marker                          : SMZMARK
[PNG]           Time                            : 00:00:00:07
[PNG]           Frame                           : 7
[PNG]           FrameRange                      : 7:9
[PNG]           Camera                          : SMZCAM
[PNG]           Lens                            : 37.50
[PNG]           Scene                           : SMZSCENE
[PNG]           Strip                           : <none>
[PNG]           RenderTime                      : 00:00.58
[PNG]           Memory                          : 0M
[PNG]           Hostname                        : Artists-Studio-MacBook.local
```

The four Cycles fields are in the Cycles file and not in the EEVEE one.

*Reading (b): where the record sits in a multi-part OpenEXR (entry 30).* The standard scene with its view layer renamed `AAALAYER` and a second added called `BBBLAYER`, the Mist pass on in each, written as multi-layer OpenEXR at the factory setting of Interleave (which printed `False`), then with Interleave on. With Interleave off the reader printed `VERSION FIELD: 0x00001402 (version 2, tiled=False longnames=True deep=False multipart=True)` and `TOTAL PARTS: 6`. Part 0, uncut:

```
--- PART 0 HEADER ---
  [BlenderMultiChannel] (string, 25 bytes) = Blender V2.55.1 and newer
  [Camera] (string, 6 bytes) = SMZCAM
  [Date] (string, 19 bytes) = 2026/10/18 04:14:13
  [File] (string, 74 bytes) = /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e30/smzfile.blend
  [Frame] (string, 1 bytes) = 7
  [FrameRange] (string, 3 bytes) = 7:9
  [Hostname] (string, 28 bytes) = Artists-Studio-MacBook.local
  [Lens] (string, 5 bytes) = 37.50
  [Marker] (string, 7 bytes) = SMZMARK
  [Memory] (string, 2 bytes) = 1M
  [Note] (string, 13 bytes) = SMZNOTE-12345
  [RenderTime] (string, 8 bytes) = 00:00.10
  [Scene] (string, 8 bytes) = SMZSCENE
  [Software] (string, 18 bytes) = Blender 5.3.0 Beta
  [Strip] (string, 6 bytes) = <none>
  [Time] (string, 11 bytes) = 00:00:00:07
  [channels] (chlist, 145 bytes) = AAALAYER.Combined.A(FLOAT); AAALAYER.Combined.B(FLOAT); AAALAYER.Combined.G(FLOAT); AAALAYER.Combined.R(FLOAT)
  [chunkCount] (int, 4 bytes) = 4
  [colorInteropID] (string, 16 bytes) = lin_rec709_scene
  [compression] (compression, 1 bytes) = 3 (ZIP)
  [cycles.AAALAYER.render_time] (string, 8 bytes) = 00:00.05
  [cycles.AAALAYER.samples] (string, 1 bytes) = 1
  [cycles.AAALAYER.synchronization_time] (string, 8 bytes) = 00:00.00
  [cycles.AAALAYER.total_time] (string, 8 bytes) = 00:00.05
  [cycles.BBBLAYER.render_time] (string, 8 bytes) = 00:00.04
  [cycles.BBBLAYER.samples] (string, 1 bytes) = 1
  [cycles.BBBLAYER.synchronization_time] (string, 8 bytes) = 00:00.00
  [cycles.BBBLAYER.total_time] (string, 8 bytes) = 00:00.04
  [dataWindow] (box2i, 16 bytes) = (0, 0, 63, 63)
  [displayWindow] (box2i, 16 bytes) = (0, 0, 63, 63)
  [lineOrder] (lineOrder, 1 bytes) = 0
  [name] (string, 17 bytes) = AAALAYER.Combined
  [pixelAspectRatio] (float, 4 bytes) = 1.0
  [screenWindowCenter] (v2f, 8 bytes) = (0.0, 0.0)
  [screenWindowWidth] (float, 4 bytes) = 1.0
  [type] (string, 13 bytes) = scanlineimage
  [xDensity] (float, 4 bytes) = 72.0
  (37 attributes in part 0)
```

Part 1, uncut:

```
--- PART 1 HEADER ---
  [channels] (chlist, 33 bytes) = AAALAYER.Mist.Z(FLOAT)
  [chunkCount] (int, 4 bytes) = 4
  [colorInteropID] (string, 4 bytes) = data
  [compression] (compression, 1 bytes) = 3 (ZIP)
  [dataWindow] (box2i, 16 bytes) = (0, 0, 63, 63)
  [displayWindow] (box2i, 16 bytes) = (0, 0, 63, 63)
  [lineOrder] (lineOrder, 1 bytes) = 0
  [name] (string, 13 bytes) = AAALAYER.Mist
  [pixelAspectRatio] (float, 4 bytes) = 1.0
  [screenWindowCenter] (v2f, 8 bytes) = (0.0, 0.0)
  [screenWindowWidth] (float, 4 bytes) = 1.0
  [type] (string, 13 bytes) = scanlineimage
  [xDensity] (float, 4 bytes) = 72.0
  (13 attributes in part 1)
```

Parts 2 to 5 are the same thirteen with their own `name` and `channels` — `AAALAYER.Noisy Image`, `BBBLAYER.Combined`, `BBBLAYER.Mist`, `BBBLAYER.Noisy Image` — and `colorInteropID` reading `lin_rec709_scene` on the picture parts and `data` on the Mist parts. So every one of the fourteen fields, `Software`, and both view layers' Cycles fields sit in part 0 and nowhere else. With Interleave on, the same render wrote `VERSION FIELD: 0x00000402 (version 2, tiled=False longnames=True deep=False multipart=False)`, `TOTAL PARTS: 1`, one part of 34 attributes holding the fourteen, `Software`, `BlenderMultiChannel`, the eight Cycles fields and all eighteen channels in one list.

*Burning the fields into the picture (entry 31).* The standard scene at 640 × 360 with Burn into Image on (`use_stamp (burn in): True | size: 640 360`), written to PNG and to Targa. Both pictures were opened and looked at (the Targa after macOS's own `sips` converted it to PNG for viewing). Both carry the same text, in light grey on dark bands: at the top left, one line each, `File /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e31/smzfile.blend`, `Date 2026/10/18 04:14:43`, `RenderTime 00:00.34` (`00:00.30` in the Targa), `Peak Memory 18M`, `Hostname Artists-Studio-MacBook.local`, then `SMZNOTE-12345` on its own line with no label; at the top right `Strip <none>`, which at this width overlaps the end of the `File` line; along the bottom `Marker SMZMARK Timecode 00:00:00:07 Frame 7 Frame Range 7:9 Camera SMZCAM Lens 37.50`, with `Scene SMZSCENE` drawn over the end of `Lens 37.50` at the right edge. `exiftool -a -G1 -s` on the PNG from the same run still counted 18 lines naming the fields in its PNG group — the fourteen and the four Cycles fields. So burning in adds to the named fields rather than replacing them, the text reaches Targa, which keeps no named fields, and two burnt-in labels differ from the field names in the file: `Memory` is drawn as `Peak Memory` and `Time` as `Timecode`.

*A field a render engine adds for itself (entry 32).* A small render engine registered for the test fills its result with grey and calls `result.stamp_data_add_field("SMZFIELD", "SMZVALUE-98765")` before ending the result; the standard scene, switches on, rendered with it to PNG. Blender printed:

```
ENGINE: SMZ_TEST_ENGINE
stamp_data_add_field RETURNED None
FILE engine.png 3547 bytes | SMZFIELD in bytes: False | SMZVALUE-98765 in bytes: False | SMZNOTE-12345 in bytes: True
```

and `exiftool` listed the scene's own fourteen in that file (`Note : SMZNOTE-12345`, `RenderTime : 00:00.00`, `Memory : 0M` among them) and nothing else from Blender. So the call succeeds, and the field it adds does not reach the written file.

*Pixel density and pixel aspect (entry 33).* The standard scene with `render.ppm_factor = 300` (it printed `ppm_factor: 300.0 | ppm_base: 0.02539999969303608`), written to PNG, BMP, TIFF, OpenEXR, JPEG and WebP; then the same with pixel aspect X 2, Y 1. The resolution lines `exiftool` printed, and the OpenEXR's own two attributes:

```
--- square_PNG.png
[IFD0]          XResolution                     : 300
[IFD0]          YResolution                     : 300
[PNG-pHYs]      PixelsPerUnitX                  : 11811
[PNG-pHYs]      PixelsPerUnitY                  : 11811
[PNG-pHYs]      PixelUnits                      : meters
--- square_BMP.bmp
[File]          PixelsPerMeterX                 : 11811
[File]          PixelsPerMeterY                 : 11811
--- square_TIFF.tif
[IFD0]          XResolution                     : 300
[IFD0]          YResolution                     : 300
[IFD0]          ResolutionUnit                  : inches
--- square_JPEG.jpg
[JFIF]          ResolutionUnit                  : None
[JFIF]          XResolution                     : 1
[JFIF]          YResolution                     : 1
--- square_WEBP.webp
EXR square: ['[pixelAspectRatio] (float, 4 bytes) = 1.0', '[xDensity] (float, 4 bytes) = 300.0']
--- aspect2x1_PNG.png
[IFD0]          XResolution                     : 150
[IFD0]          YResolution                     : 300
[PNG-pHYs]      PixelsPerUnitX                  : 5905
[PNG-pHYs]      PixelsPerUnitY                  : 11811
[PNG-pHYs]      PixelUnits                      : meters
--- aspect2x1_BMP.bmp
[File]          PixelsPerMeterX                 : 5906
[File]          PixelsPerMeterY                 : 11811
--- aspect2x1_TIFF.tif
[IFD0]          XResolution                     : 150
[IFD0]          YResolution                     : 150
[IFD0]          ResolutionUnit                  : inches
--- aspect2x1_JPEG.jpg
[JFIF]          ResolutionUnit                  : None
[JFIF]          XResolution                     : 1
[JFIF]          YResolution                     : 1
--- aspect2x1_WEBP.webp
EXR aspect2x1: ['[pixelAspectRatio] (float, 4 bytes) = 2.0', '[xDensity] (float, 4 bytes) = 150.0']
```

So PNG, BMP, TIFF and OpenEXR carry the density; JPEG carries none (`ResolutionUnit : None`), although the manual's table ticks it for DPI; WebP carries none. With a 2:1 pixel aspect, PNG and BMP write the two directions separately, TIFF writes both as the X value, and OpenEXR writes one density with its own `pixelAspectRatio`.

*Cryptomatte in the file's header (entry 34).* The standard scene with the Cryptomatte Object pass on (`use_pass_cryptomatte_object: True | levels: 6`), written as multi-layer OpenEXR. The header lines naming Cryptomatte, and every part's name:

```
  [cryptomatte/542cafa/conversion] (string, 17 bytes) = uint32_to_float32
  [cryptomatte/542cafa/hash] (string, 14 bytes) = MurmurHash3_32
  [cryptomatte/542cafa/manifest] (string, 58 bytes) = {"Cube":"a8fce865","Light":"c77d43e5","object":"055ec73d"}
  [cryptomatte/542cafa/name] (string, 22 bytes) = ViewLayer.CryptoObject
  [name] (string, 18 bytes) = ViewLayer.Combined
  [name] (string, 24 bytes) = ViewLayer.CryptoObject00
  [name] (string, 24 bytes) = ViewLayer.CryptoObject01
  [name] (string, 24 bytes) = ViewLayer.CryptoObject02
  [name] (string, 21 bytes) = ViewLayer.Noisy Image
TOTAL PARTS: 5
```

So the Cryptomatte manifest is written into the image file's own header.

*What a movie holds (entry 35).* The standard scene, frames 7 to 9, written three times as MPEG-4 with H.264: plain; with stereoscopy on (`STEREO: use_multiview True | views_format STEREO_3D | image views_format STEREO_3D | display_mode SIDEBYSIDE`); and with the camera set to Panoramic (`PANO: camera type PANO | panorama_type EQUIRECTANGULAR`). Blender wrote `plain_0007-0009.mp4`, `stereo_0007-0009.mp4` and `pano_0007-0009.mp4`; `ffprobe` gave their pictures as `64,64`, `128,64` and `64,64`. `exiftool -a -G1 -s` on the plain movie, every line beyond the system ones, cut only where the three files' listings are the same:

```
[QuickTime]     MajorBrand                      : MP4 Base Media v1 [IS0 14496-12:2003]
[QuickTime]     CompatibleBrands                : isom, iso2, avc1, mp41
[QuickTime]     CreateDate                      : 0000:00:00 00:00:00
[QuickTime]     TimeScale                       : 1000
[QuickTime]     Duration                        : 0.12 s
[QuickTime]     HandlerType                     : Metadata
[QuickTime]     HandlerVendorID                 : Apple
[Track1]        ImageWidth                      : 64
[Track1]        ImageHeight                     : 64
[Track1]        CompressorID                    : avc1
[Track1]        ColorProfiles                   : nclx
[Track1]        ColorPrimaries                  : BT.709
[Track1]        TransferCharacteristics         : BT.709
[Track1]        MatrixCoefficients              : BT.709
[Track1]        VideoFullRangeFlag              : Limited
[Track1]        PixelAspectRatio                : 1:1
[Track1]        VideoFrameRate                  : 24
[ItemList]      ContentCreateDate               : 2026/10/18 04:16:34
[ItemList]      Encoder                         : Lavf62.12.100
```

A byte search of each movie found none of `st3d`, `sv3d`, `svhd`, `prji`, `proj`, `equi`, `ytmp` (`st3d=0 sv3d=0 svhd=0 prji=0 proj=0 equi=0 ytmp=0` for all three), and none of `SMZNOTE`, `SMZSCENE`, `SMZCAM`, `SMZMARK` or `Blender` (each `=0`). So an MPEG-4 with H.264, every switch on, carries none of the fourteen under any name; the side-by-side stereo movie is one file, twice as wide, with no `_L` or `_R` in its name; and neither the stereo nor the panoramic movie carries a stereo or projection marking that could be found. `Lavf` is the FFmpeg library. Containers other than MPEG-4 were not read.

*The factory switches, the note properties and the panel's own controls (entry 36).* A factory-fresh scene, with no window. Blender printed:

```
FACTORY SWITCHES: [('use_stamp_time', True), ('use_stamp_date', True), ('use_stamp_frame', True), ('use_stamp_frame_range', False), ('use_stamp_camera', True), ('use_stamp_lens', False), ('use_stamp_scene', True), ('use_stamp_note', False), ('use_stamp_marker', False), ('use_stamp_filename', True), ('use_stamp_sequencer_strip', False), ('use_stamp_render_time', True), ('use_stamp', False), ('use_stamp_labels', True), ('use_stamp_memory', False), ('use_stamp_hostname', False)]
PROPERTY RenderSettings.use_stamp_note: type=BOOLEAN default=False | Include a custom note in image/video metadata
PROPERTY RenderSettings.stamp_note_text: type=STRING default='' | Custom text to appear in the stamp note
PROPERTY Scene.use_stamp_note: type=STRING default='' | User defined note for the render stamping
use_stamp_memory property's own stated default: True
METADATA PANEL CLASSES: ['RENDER_PT_stamp', 'RENDER_PT_stamp_burn', 'RENDER_PT_stamp_note']
```

So on a factory-fresh scene `use_stamp_memory` reads `False`, while the property's own stated default is `True`. The Metadata panel's drawing code, read out of the running Blender from `…/5.3/scripts/startup/bl_ui/properties_output.py`, draws these controls under the heading `Include`:

```
        if rd.use_sequencer:
            layout.prop(rd, "metadata_input")
        col = layout.column(heading="Include")
        col.prop(rd, "use_stamp_date", text="Date")
        col.prop(rd, "use_stamp_time", text="Time")
        col.prop(rd, "use_stamp_render_time", text="Render Time")
        col.prop(rd, "use_stamp_frame", text="Frame")
        col.prop(rd, "use_stamp_frame_range", text="Frame Range")
        col.prop(rd, "use_stamp_memory", text="Memory")
        col.prop(rd, "use_stamp_hostname", text="Hostname")
        col.prop(rd, "use_stamp_camera", text="Camera")
        col.prop(rd, "use_stamp_lens", text="Lens")
        col.prop(rd, "use_stamp_scene", text="Scene")
        col.prop(rd, "use_stamp_marker", text="Marker")
        col.prop(rd, "use_stamp_filename", text="Filename")
        if rd.use_sequencer:
            col.prop(rd, "use_stamp_sequencer_strip", text="Strip Name")
```

with two sub-panels, `Note` (its header box `use_stamp_note`, its body `layout.textbox(rd, "stamp_note_text", text="")`) and `Burn Into Image` (its header box `use_stamp`, its body Font Size, the two colours and `use_stamp_labels` as `Include Labels`). No property's interface name begins "Use Strip Me" or "Strip Me" (`[]`); the only Sequencer-related controls are `metadata_input` and `Strip Name`, both drawn only while the Sequencer is in use.

*Metadata taken from the Sequencer's strips (entry 37).* The standard scene; frame 7 rendered to a PNG first, in the same session, carrying its own fields; two image strips, `SMZSTRIP1` on channel 1 and `SMZSTRIP2` on channel 2, both on that PNG; `render.use_sequencer` on; format PNG; frame 7 rendered twice more, once with `metadata_input = 'STRIPS'` and once with `'SCENE'`. Blender printed:

```
SOURCE written: source.png 5442 bytes, hash 68009c4db482830d
STRIP SMZSTRIP1 channel 1 frame_final_start 7 frame_final_duration 1
STRIP SMZSTRIP2 channel 2 frame_final_start 7 frame_final_duration 1
metadata_input: STRIPS | use_sequencer: True
SOURCE after the STRIPS render: 5442 bytes, hash 68009c4db482830d
metadata_input: SCENE | use_sequencer: True
SOURCE after the SCENE render: 5442 bytes, hash 68009c4db482830d
```

So the source file still carried its own fields at the moment of each render. Everything `exiftool -a -G1 -s` printed for the `STRIPS` file from its PNG group on was the picture's structure, the colour lines (`SRGBRendering`, `Gamma`, the white point and primaries, the `cICP` lines and the sRGB ICC profile), `XResolution : 72`, `YResolution : 72` and the `pHYs` lines — not one Blender field and no `Strip` line. The `SCENE` file carried the fourteen and no Cycles field:

```
[PNG]           File                            : /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e37/smzfile.blend
[PNG]           Note                            : SMZNOTE-12345
[PNG]           Date                            : 2026/10/18 04:17:34
[PNG]           Marker                          : SMZMARK
[PNG]           Time                            : 00:00:00:07
[PNG]           Frame                           : 7
[PNG]           FrameRange                      : 7:9
[PNG]           Camera                          : SMZCAM
[PNG]           Lens                            : 37.50
[PNG]           Scene                           : SMZSCENE
[PNG]           Strip                           : SMZSTRIP2
[PNG]           RenderTime                      : 00:00.00
[PNG]           Memory                          : 1M
[PNG]           Hostname                        : Artists-Studio-MacBook.local
```

The source PNG carried those fourteen with `Strip : <none>` and `RenderTime : 00:00.05`, plus the four Cycles fields. Counting the Blender fields in each file: source 18, `STRIPS` 0, `SCENE` 14.

*Cycles' debug passes, on the processor and on the graphics (entry 38).* The standard scene, with the Cycles view-layer settings whose names hold `debug`, `render_time` or `sample_count` switched on; written as multi-layer OpenEXR once with Cycles on the processor and once on the Metal graphics. Blender printed:

```
CYCLES VIEW-LAYER SETTINGS MATCHING debug/render_time/sample_count: ['pass_debug_sample_count', 'pass_render_time']
NOW: [('pass_debug_sample_count', True), ('pass_render_time', True)]
DEVICES: [('Apple M1 Pro', 'CPU', True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]
=== device CPU: channel lists
  [channels] (chlist, 149 bytes) = ViewLayer.Combined.A(FLOAT); ViewLayer.Combined.B(FLOAT); ViewLayer.Combined.G(FLOAT); ViewLayer.Combined.R(FLOAT)
  [channels] (chlist, 48 bytes) = ViewLayer.Debug Sample Count.X(FLOAT)
  [channels] (chlist, 41 bytes) = ViewLayer.Render Time.X(FLOAT)
  [channels] (chlist, 161 bytes) = ViewLayer.Noisy Image.A(FLOAT); ViewLayer.Noisy Image.B(FLOAT); ViewLayer.Noisy Image.G(FLOAT); ViewLayer.Noisy Image.R(FLOAT)
TOTAL PARTS: 4
=== device GPU: channel lists
  [channels] (chlist, 149 bytes) = ViewLayer.Combined.A(FLOAT); ViewLayer.Combined.B(FLOAT); ViewLayer.Combined.G(FLOAT); ViewLayer.Combined.R(FLOAT)
  [channels] (chlist, 48 bytes) = ViewLayer.Debug Sample Count.X(FLOAT)
  [channels] (chlist, 161 bytes) = ViewLayer.Noisy Image.A(FLOAT); ViewLayer.Noisy Image.B(FLOAT); ViewLayer.Noisy Image.G(FLOAT); ViewLayer.Noisy Image.R(FLOAT)
TOTAL PARTS: 3
```

So on the processor the file carries `ViewLayer.Debug Sample Count.X` and `ViewLayer.Render Time.X`; on the Metal graphics the Render Time channel is absent.

*The compositor's File Output node, built step by step (entry 39).* The standard scene. The build, in order, with what the machine printed at each step (the first of two runs shown in full; the second differs only where said):

```
STEP 1: ordinary output — media type then format
  ims.media_type = IMAGE | ims.file_format = PNG | render.filepath = …/e39/effects_stack/panel/panel
STEP 2: the node group
  scene.node_tree present: False
  bpy.data.node_groups.new('SMZComp', 'CompositorNodeTree') -> SMZComp CompositorNodeTree
STEP 3: interface sockets, added before the Group Input/Output nodes
  interface items: [('Image', 'OUTPUT', 'NodeSocketColor'), ('Image', 'INPUT', 'NodeSocketColor')]
  group input sockets: ['Image', ''] | group output sockets: ['Image', '']
STEP 4: the File Output node and its one item
  node: CompositorNodeOutputFile inputs before the item: ['']
  node.format.media_type at birth: MULTI_LAYER_IMAGE | node.format.file_format at birth: OPEN_EXR_MULTILAYER
STEP 4a: setting file_format to PNG BEFORE media_type, on purpose, to record what comes back
  ERROR: TypeError bpy_struct: item.attr = val: enum "PNG" not found in ('OPEN_EXR_MULTILAYER')
STEP 4b: media_type first, then file_format
  fo.format.media_type = IMAGE | fo.format.file_format = PNG | item: SMZNODE
  node inputs after the item: ['SMZNODE', '']
STEP 5: links
  link: NodeGroupInput Image -> NodeGroupOutput Image
  link: NodeGroupInput Image -> CompositorNodeOutputFile SMZNODE
  effects: [('SMZEffect', 'SMZComp', True)]
  render.use_compositing = True
  fo.inputs[0].name: SMZNODE is_linked: True | fo.mute: False | save_as_render: True
STEP 6: directory that does not exist: …/e39/effects_stack/node_missing | file_name: smznode
LIST[before still render, node folder] …/e39/effects_stack/node_missing -> NO SUCH FOLDER
STEP 6a: bpy.ops.render.render(write_still=True) -> {'FINISHED'}
LIST[after still render, node folder] …/e39/effects_stack/node_missing -> ['smznodeSMZNODE.png']
LIST[after still render, panel folder] …/e39/effects_stack/panel -> ['panel.png']
STEP 7: directory made first: …/e39/effects_stack/node_made
STEP 7a: bpy.ops.render.render(write_still=True) -> {'FINISHED'}
LIST[after still render, node folder] …/e39/effects_stack/node_made -> ['smznodeSMZNODE.png']
LIST[after still render, panel folder] …/e39/effects_stack/panel2 -> ['panel.png']
STEP 8: bpy.ops.render.render(animation=True) over frames 7 to 7 -> {'FINISHED'}
LIST[after animation render, node folder] …/e39/effects_stack/node_anim -> ['smznodeSMZNODE0007.png']
LIST[after animation render, panel folder] …/e39/effects_stack/panel3 -> ['panel0007.png']
```

with Blender's own `Saved: '…/node_missing/smznodeSMZNODE.png'`, `Saved: '…/node_made/smznodeSMZNODE.png'` and `Saved: '…/node_anim/smznodeSMZNODE0007.png'` lines beside the panel's. That run put the group into the 5.3 effects stack. The second run built the same thing and put the group on the older `scene.compositing_node_group` setting instead (`effects: ['Scene Effect'] | compositing_node_group: SMZComp`); it printed the same `TypeError` at step 4a and the same three node files: `['smznodeSMZNODE.png']`, `['smznodeSMZNODE.png']`, `['smznodeSMZNODE0007.png']`. So the node wrote its file every time, making its folder when it did not exist, after a still render and after an animation render, by either route; the file name is the node's file name followed by the item's name, with the frame number added for an animation.

The fields in the node's still file against the panel's file from the same render:

```
===== node_missing/smznodeSMZNODE.png
[PNG]           ImageWidth                      : 64
[PNG]           BitDepth                        : 16
[PNG]           File                            : /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e39/effects_stack/smzfile.blend
[PNG]           Note                            : SMZNOTE-12345
[PNG]           Date                            : 2026/10/18 04:18:43
[PNG]           Marker                          : SMZMARK
[PNG]           Time                            : 00:00:00:07
[PNG]           Frame                           : 7
[PNG]           FrameRange                      : 7:9
[PNG]           Camera                          : <none>
[PNG]           Lens                            : <none>
[PNG]           Scene                           : SMZSCENE
[PNG]           Strip                           : <none>
[PNG]           RenderTime                      : 00:00.00
[PNG]           Memory                          : 1M
[PNG]           Hostname                        : Artists-Studio-MacBook.local
===== panel/panel.png
[PNG]           ImageWidth                      : 64
[PNG]           BitDepth                        : 8
[PNG]           File                            : /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e39/effects_stack/smzfile.blend
[PNG]           Note                            : SMZNOTE-12345
[PNG]           Date                            : 2026/10/18 04:18:43
[PNG]           Marker                          : SMZMARK
[PNG]           Time                            : 00:00:00:07
[PNG]           Frame                           : 7
[PNG]           FrameRange                      : 7:9
[PNG]           Camera                          : SMZCAM
[PNG]           Lens                            : 37.50
[PNG]           Scene                           : SMZSCENE
[PNG]           Strip                           : <none>
[PNG]           RenderTime                      : 00:00.19
[PNG]           Memory                          : 1M
[PNG]           Hostname                        : Artists-Studio-MacBook.local
[PNG]           CyclesViewLayersamples          : 1
[PNG]           CyclesViewLayertotal_time       : 00:00.05
[PNG]           CyclesViewLayerrender_time      : 00:00.05
[PNG]           CyclesViewLayersynchronization_time: 00:00.00
```

So the File Output node's file carries the fourteen names with `Camera : <none>`, `Lens : <none>`, `RenderTime : 00:00.00`, and none of the four Cycles fields.

*A render with Blender's window open, against one from the terminal (entry 42).* Three copies of the standard scene were saved with no window: one with OpenEXR output and Preview on, one with PNG output, and one with PNG output and the File Output node of entry 39 (one PNG item `SMZNODE`, file name `smznode`) in the effects stack. Each was then opened in Blender 5.3.0 Beta with its window open on the Mac's own display (`--factory-startup -noaudio --enable-event-simulate --window-geometry 0 0 1600 1000 <file>.blend`), the first-run "Quick Setup" box closed with a simulated Escape key, and driven by a timed script. The same OpenEXR and PNG files were also rendered from a terminal with no window: `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b -noaudio --factory-startup <file>.blend -o <folder>/term_ -f 7`.

In the window, a still rendered with `bpy.ops.render.render(write_still=True)` printed, for the OpenEXR copy, `Saved: '…/A2_window/exec/still_.exr'` and `Saved: '…/A2_window/exec/still_.jpg'`, and `FOLDER exec: ['still_.exr', 'still_.jpg']`; the terminal render of the same file printed `Saved: '…/term_0007.exr'` and `Saved: '…/term_0007.jpg'`. So the Preview JPEG is written beside the OpenEXR both with the window and without it.

A still started the way the Render menu's own command starts it, `bpy.ops.render.render('INVOKE_DEFAULT')`, returned `{'RUNNING_MODAL'}`, opened a second window holding an Image Editor on `Render Result`, and wrote nothing to the output path: `FOLDER invoke (the render's own output path): NO SUCH FOLDER`, for the OpenEXR copy and the PNG copy alike. The Render Result was then saved from that Image Editor with `bpy.ops.image.save_as(filepath=…, copy=True)`, which returned `{'FINISHED'}` and wrote `saved_by_hand.exr` (and `saved_by_hand.png` for the PNG copy). A first try of that save, with the second window's screen named in the call, raised `TypeError: Overriding context with temporary screen isn't supported`; naming only the window and the area worked.

The `exiftool -a -G1 -s` listings were then compared by their field names alone, values left aside:

```
SAME NAMES, SAME ORDER: terminal_smz_exr/term_0007.exr | A2_window/exec/still_.exr (40 lines)
SAME NAMES, SAME ORDER: terminal_smz_exr/term_0007.exr | A2_window/saved/saved_by_hand.exr (40 lines)
SAME NAMES, SAME ORDER: terminal_smz_png/term_0007.png | B2_window/exec/still_.png (78 lines)
SAME NAMES, SAME ORDER: terminal_smz_png/term_0007.png | B2_window/saved/saved_by_hand.png (78 lines)
SAME NAMES, SAME ORDER: terminal_smz_png/term_0007.png | C_window/exec/still_.png (78 lines)
```

The saved-by-hand OpenEXR, for one, held:

```
[OpenEXR]       Camera                          : SMZCAM
[OpenEXR]       Date                            : 2026/10/18 04:30:55
[OpenEXR]       File                            : /private/tmp/claude-501/smoozy-hand-tests-5.3/Images/out/e42/smz_exr.blend
[OpenEXR]       Frame                           : 7
[OpenEXR]       FrameRange                      : 7:9
[OpenEXR]       Hostname                        : Artists-Studio-MacBook.local
[OpenEXR]       Lens                            : 37.50
[OpenEXR]       Marker                          : SMZMARK
[OpenEXR]       Memory                          : 1M
[OpenEXR]       Note                            : SMZNOTE-12345
[OpenEXR]       RenderTime                      : 00:00.05
[OpenEXR]       Scene                           : SMZSCENE
[OpenEXR]       Software                        : Blender 5.3.0 Beta
[OpenEXR]       Strip                           : <none>
[OpenEXR]       Time                            : 00:00:00:07
[OpenEXR]       CyclesViewLayerRender_Time      : 00:00.04
[OpenEXR]       CyclesViewLayerSamples          : 1
[OpenEXR]       CyclesViewLayerSynchronization_Time: 00:00.00
[OpenEXR]       CyclesViewLayerTotal_Time       : 00:00.05
```

the same names as the terminal file's; the values differ only in `Date` and the render-time figures. So a render made with the window open, and a picture saved by hand from the Image Editor, carry exactly the same fields as a render from the terminal.

With the window open, the File Output node wrote its file too: the still printed `Saved: '…/C_window_node/smznodeSMZNODE.png'` beside the panel's own file, and an animation render over frame 7 added `smznodeSMZNODE0007.png` (`FOLDER node (after the animation): ['smznodeSMZNODE.png', 'smznodeSMZNODE0007.png']`).

---

## Things a person at a computer with Blender 5.3 open could settle

Every item that stood here was run on 2026-10-18, on the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta, and the results are written in under the questions they answer: the composited layer's name (questions 1 and 5), a missing texture in the separate passes (question 2), a missing texture feeding roughness, a normal map, a bump and a displacement (question 9), unknown names and an unfitting container and codec (question 4), what HDR files record (question 6), the Preview JPEG (question 8), and the fields in every kind of file (question 10). Every test the earlier versions had run by hand was run as well. The list of tests, with every script word for word, is `Audits/Blender Research/Images/Hand tests — 5.3.md`.

What is still open:

1. **Linux and Windows, and every graphics card that is not Apple's.** Nothing above was run on Linux or Windows, or on an NVIDIA, AMD or Intel card, or on two cards; no such machine was available to this sitting. Those stay untried. (Untried rather than untestable: see the equipment section of `Preparation/Blender Research/Method.md`.) The Linux Wayland sentence quoted under question 6 needs a Linux machine running Wayland.
2. **A Blender built without one of its optional formats** (question 4, reading a). The 5.3 build on the Mac has every one compiled in, and nothing could be downloaded in this sitting.
3. **A texture tile asked for but not yet in memory, and an image record with no pixel data** (questions 2 and 9). No scene was found that holds either state.
4. **Which view an older OpenEXR reader shows for a multi-view file** (question 5). Only Blender and the file's own header were read.
5. **HDR output at more than 8 bits per channel** (question 6). The HDR PNG and AVIF were written at 8 bits; what a deeper file records was not tried.
6. **What a movie in a container other than MPEG-4 carries** (question 10). Only MPEG-4 with H.264 was read.

---

2026-10-17 — Claude Opus 5.5 (Claude Code) — Images, Blender 5.3

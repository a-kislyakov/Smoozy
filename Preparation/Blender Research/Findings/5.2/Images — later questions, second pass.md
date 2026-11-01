# Images — later questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender 5.2 manual (`Blender Sources/manual_5.2`), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files as shipped with Blender 5.2 (`Blender Sources/cycles_5.2`). All four are a snapshot taken on **2026-07-30**, which is the date read for every finding below. I also read `Blender Sources/About this folder.md`, which describes what the snapshot contains.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

Only question 9 was assigned. The rest of the sheet was not touched.

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

> **Changed by the citation check, 2026-08-09 (Opus 5).** Three things were changed, each marked *[citation check]* where it sits: the first lookup's line range corrected from 88–94 to 88–95; "and nothing else" corrected to name the two arguments the quoted signatures also show; and a sentence about what a transparency set-up looks like narrowed to what the value carries. Three hand-test entries were annotated — see entries 2, 5 and 14.

### The question is ambiguous, and I answer both readings

"Feeding something other than colour" can mean two different things, and I could not tell which was meant:

- **Reading A (the one I answer as primary):** the image texture's output is wired into a socket that is not a colour — roughness, metallic, a bump or normal map, displacement, an alpha or mix factor, a volume density, and so on.
- **Reading B:** the image itself is marked as not being colour data — its Colour Space is set to something like Non-Colour, so the numbers in it are data rather than a picture.

As it happens the evidence below covers both, and gives the same answer for each, so the ambiguity does not change the outcome. I say where each reading is addressed.

### Short answer

Cycles has one single substitute value for an image it cannot find, and that value is **opaque magenta — red 1, green 0, blue 1, alpha 1**. It is substituted at the point the texture is sampled, before anything downstream knows what the number is for. There is no separate path for an image that is feeding something other than colour. So the render produces those same numbers pushed into whatever the image was feeding: a roughness input gets a value derived from magenta, a normal or bump input gets magenta read as a vector, an alpha input gets 1 (fully opaque), and so on.

Nothing in the manual, the Python API documentation or the 5.2 release notes states this, or states anything at all about what a render produces where a shading image is missing. The finding comes entirely from Cycles' own files.

### Label

**Mixed, and the parts must be kept apart:**

- **observed** — that Cycles defines one substitute value for images that are not found, that it is magenta with alpha 1, and that this substitution happens in the image-sampling functions with no reference to what the sample will be used for.
- **inferred** — that the rendered pixels therefore show magenta where the missing image feeds colour, and show the consequence of magenta where it feeds anything else. The step is written out below.
- **not in the sources** — the manual, the API documentation and the release notes say nothing about it. My search record is below.

### What Cycles' own files say (observed)

**The substitute value itself.** In `cycles_5.2/source/util/types_image.h`, lines 17–18:

> ```
> /* Color to use when images are not found. */
> #define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)
> ```

That is red 1, green 0, blue 1, alpha 1 — magenta, fully opaque.

**Where it is used.** Searching the whole of `cycles_5.2` for `IMAGE_MISSING_RGBA` returns exactly six lines: the definition above, and five places that return it. In `cycles_5.2/source/kernel/device/gpu/image.h`, inside `kernel_image_interp` (the function that samples a 2D image texture), lines 88–95 *[citation check: was 88–94; the quoted block runs to the closing brace on line 95]*:

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

and again at lines 122–125, for the case where the image exists as a texture but has no image data behind it:

> ```
>     /* Full image sampling. */
>     if (tex.image_info_id == KERNEL_IMAGE_NONE) {
>       return IMAGE_MISSING_RGBA;
>     }
> ```

and at line 113, for a tiled image whose tile failed to load:

> ```
>       return (tile_descriptor == KERNEL_TILE_LOAD_FAILED) ? IMAGE_MISSING_RGBA : tex.average_color;
> ```

The meaning of that failure state is written out in `cycles_5.2/source/util/types_image.h`, lines 114–119:

> ```
> /* Tile descriptor load status. Values lower than these mean the tile has been
>  * successfully loaded, and the value encodes where the tile is stored.
>  * - LOAD_NONE: Tile has never been accessed and is not in memory.
>  * - LOAD_REQUEST: Kernel requests this tile to be loaded.
>  * - LOAD_FAILED: Tile loading failed, will not try again.
>  */
> ```

And at lines 165–168 of `cycles_5.2/source/kernel/device/gpu/image.h`, for a UDIM set where the tile the coordinates land on is not present:

> ```
>   const int image_texture_id = kernel_image_udim_map(kg, udim_id, uv.val);
>   if (image_texture_id == KERNEL_IMAGE_NONE) {
>     return IMAGE_MISSING_RGBA;
>   }
> ```

The fifth is in `cycles_5.2/source/kernel/util/image_3d.h`, line 259 — the last line of `kernel_image_interp_3d`, the 3D/volume sampler, reached when no recognised volume data type matched. I am **not** claiming this one is the missing-file path; it is a fall-through, and I could not establish from the files in the snapshot what reaches it.

**The substitution ignores what the sample is for.** The three functions above take an image identifier and a coordinate, together with the kernel's own globals and the shading data. *[citation check: this said "and nothing else"; the quoted signatures also show `KernelGlobals kg` and `ccl_private ShaderData *sd`.]* None of those tells them whether the result will become a colour, a roughness, a normal or a displacement. The value is returned before any of that is known.

**The Image Texture node then hands the substituted value on unchanged.** In `cycles_5.2/source/kernel/svm/image.h`, lines 22–38 and 60–76:

> ```
> ccl_device float4 svm_image_texture(
>     KernelGlobals kg, ccl_private ShaderData *sd, const int id, const dual2 uv, const uint flags)
> {
>   float4 r = kernel_image_interp_with_udim(kg, sd, id, uv);
>   const float alpha = r.w;
>
>   if ((flags & NODE_IMAGE_ALPHA_UNASSOCIATE) && alpha != 1.0f && alpha != 0.0f) {
>     r /= alpha;
>     r.w = alpha;
>   }
>
>   if (flags & NODE_IMAGE_COMPRESS_AS_SRGB) {
>     r = color_srgb_to_linear_v4(r);
>   }
>
>   return r;
> }
> ```

> ```
>   const float4 f = svm_image_texture(kg, sd, node.id, tex_co, node.flags);
>
>   if (stack_valid(node.out_offset)) {
>     stack_store_float3(stack, node.out_offset, make_float3(f));
>   }
>   if (stack_valid(node.alpha_offset)) {
>     stack_store_float(stack, node.alpha_offset, f.w);
>   }
> ```

Neither of the two adjustments alters the substituted value:

- The alpha un-association is skipped, because it is guarded by `alpha != 1.0f` and the substituted alpha is exactly 1.
- The sRGB-to-linear conversion leaves the substituted value untouched, because it only ever sees 0 and 1. From `cycles_5.2/source/util/color.h`, lines 63–69:

  > ```
  > ccl_device float color_srgb_to_linear(const float c)
  > {
  >   if (c < 0.04045f) {
  >     return (c < 0.0f) ? 0.0f : c * (1.0f / 12.92f);
  >   }
  >   return powf((c + 0.055f) * (1.0f / 1.055f), 2.4f);
  > }
  > ```

  0 goes in and 0 comes out; 1 goes in and 1 comes out. **This is the part that settles Reading B.** Whether or not the image is treated as colour data, the value handed on is the same magenta with alpha 1.

The same substituted value flows through the box-projection and environment-texture versions of the node, in the same file at lines 158–174 and 197–204, which call the same `svm_image_texture` and store into the same two outputs.

### The step from "the file says" to "the render produces" (inferred)

The files establish that the value substituted for a missing image is magenta with alpha 1, and that it is handed to the node's Colour and Alpha outputs like any real sample. To get from there to what a render produces I supply one step: **whatever a node output is connected to receives the numbers on that output**, so a missing image contributes magenta to every place it was feeding, not just to colour.

What that looks like in practice, one reading at a time:

- **Fed into the Alpha slot of anything** — the Alpha output is 1 (`stack_store_float(stack, node.alpha_offset, f.w)` with `f.w == 1`). So the value handed to that slot is 1, the fully opaque end. *[citation check: "Nothing visibly signals a problem in a transparency-driven setup; it just goes solid" was narrowed — no line quoted here bears on what the finished picture looks like.]*
- **Fed into a single-value input such as Roughness, Metallic or a mix factor** — the Colour output is a colour, and Cycles converts a colour to a single value with `linear_rgb_to_gray`, in `cycles_5.2/source/kernel/svm/convert.h`, case `NODE_CONVERT_CF`. That function, in `cycles_5.2/source/kernel/util/colorspace.h` lines 34–37, is:

  > ```
  > template<class T> ccl_device auto linear_rgb_to_gray(KernelGlobals kg, const T c)
  > {
  >   return dot(c, make_float3(kernel_data.film.rgb_to_y));
  > }
  > ```

  The result is the red coefficient plus the blue coefficient of the scene's working colour space. I deliberately do not give a number: `film.rgb_to_y` is filled in from scene data that is not in these files.
- **Fed into a normal map, bump or displacement** — the same magenta triple is read as three numbers by whatever node consumes it. The files do not describe what those nodes then compute for that particular triple, and I did not chase it, because it is a different question from the one asked.

### Not covered

- **Cycles on CPU is only partly evidenced.** `cycles_5.2/source/kernel/image.h` shows that non-GPU builds get their image sampler from `kernel/device/cpu/image.h`, and that file is **not in the snapshot** — the snapshot's `cycles_5.2/source/kernel/device/` contains `gpu`, `metal`, `cuda`, `optix`, `hip` and `hiprt`, but no `cpu`. So the exact `IMAGE_MISSING_RGBA` returns I quoted are the GPU ones. The constant, its comment, the node code in `svm/image.h`, and the shared tile logic in `kernel/util/image_2d.h` are all device-independent, which is why I state the finding as I do — but I cannot show you the CPU return statement, and you should not take my answer as proof the CPU path is byte-for-byte identical.
- **EEVEE is not covered at all.** The snapshot contains Cycles' files and no EEVEE source, and nothing in the manual, the API documentation or the release notes says what EEVEE renders for a missing image. Do not carry this answer over to EEVEE.
- **The workbench/solid viewport display, and the Image Editor's own display, are not covered.**
- **Whether Blender writes a warning anywhere when it renders with a missing image** is not covered — see the search record; the sources are silent.
- **What a missing volume file (OpenVDB) produces** is not settled. The one 3D-image return of `IMAGE_MISSING_RGBA` I found is a fall-through I could not trace to a missing-file cause.
- **The exact single value** a missing image produces when driving a Roughness-style input is not settled, because it depends on scene colour-space data that is not in these files.
- **What a Normal Map or Bump node computes** from the magenta triple is not settled; I did not pursue it.

### Outside the interface

**Inferred.** The substitution lives in Cycles' render kernel — the code that shades points during a render. Nothing in the quoted functions consults a window, an editor, a screen or a user action; they take an image identifier and a coordinate. The step I supply is that code with no interface dependence behaves the same whether Blender was started by a person clicking or by something else driving it, so a render launched from outside Blender's window produces the same magenta substitution. I could find no statement anywhere in the sources that confirms or contradicts this.

Two related things **do** depend on the interface, and neither is the render:

- `bpy.ops.file.find_missing_files(...)` — "Try to find missing external files" (Python API, https://docs.blender.org/api/5.2/bpy.ops.file.html, read 2026-07-30). The manual describes it as opening a file browser: "A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it" (manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html, read 2026-07-30). It does take a `directory` argument, so it may be usable without the browser, but the sources do not say so.
- `bpy.ops.file.report_missing_files()` — "Report all missing external files" (Python API, https://docs.blender.org/api/5.2/bpy.ops.file.html, read 2026-07-30). The manual says where the answer appears: "After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files." (manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html, read 2026-07-30). The Info editor is part of the interface; the sources do not say what, if anything, this operator produces when there is no interface to show it in.

**Settled by hand test, 2026-08-09 — observed.** Every result in the *Hand-test results* section below was produced with Blender driven from a terminal with no window, on both computers. The substitution happened, the render finished, the frame was written, and the command ended with exit status 0. Standard error was empty. The reporting side is settled too: `bpy.ops.file.report_missing_files()` returned `{'FINISHED'}` and printed to the ordinary output — `Info: No missing files` where the file was there, and `Warning: Path '<path>/flat.png' not found, from local data-block 'IMflat.png'` where it was not — so the report does reach a terminal when there is no Info editor. The render itself printed two lines to the ordinary output, `cycles | WARNING Image file <path>/flat.png does not exist.` and `cycles | ERROR Failed to load 1 image files`.

### Operating system

**Not in the sources, with one thing worth saying.** No source I found ties the missing-image substitution to any operating system. The constant and the node code are plain, device-independent lines in Cycles' kernel with no operating-system condition around them — no `#ifdef` for Windows, macOS or Linux anywhere in the quoted regions. That is an absence of a difference, not a stated guarantee of sameness, and I am not going to dress it up as one. What the sources genuinely do not settle is whether the same image counts as *missing* on every system: file-path handling, capitalisation of names and path separators differ between systems, and nothing in the sources describes how Blender resolves an image path on each. A file that is found on one system and not on another would then produce different renders for reasons that have nothing to do with the substitution itself.

**Settled by hand test, 2026-08-09 — observed.** Every test in the *Hand-test results* section below was run twice: on macOS 26.5.2 on an Apple M1 Pro, and on Ubuntu 22.04.5 LTS on kernel 6.8 with an NVIDIA GeForce RTX 5060. The two computers agreed on every test. The substitute values, the lines printed, the exit statuses and the pictures written matched; the only differences were in the last digits of a few figures, and those are given where they occur. Windows was not tested, because no Windows machine has run it, so it is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Where I looked

Everything below is inside `Blender Sources/`. All searches were case-insensitive over the whole of the named folder unless I say otherwise.

**Cycles' own files (`cycles_5.2`) — where the answer came from.**

- Searched all of `cycles_5.2` for `missing` / `MISSING`. Fifteen hits. Most were unrelated (missing energy in the microfacet model, missing address-space qualifiers, missing intersections). The relevant ones were `IMAGE_MISSING_RGBA` in `source/util/types_image.h` and its uses.
- Searched all of `cycles_5.2` for `IMAGE_MISSING_RGBA`. Exactly six lines — one definition, five returns. All five are quoted or described above. That exhaustiveness is what lets me say there is no second, different substitute for a missing image, and no separate path for non-colour use.
- Read in full: `source/util/types_image.h`, `source/kernel/svm/image.h`, `source/kernel/image.h`, `source/kernel/device/gpu/image.h` (lines 60–180), `source/kernel/util/image_2d.h` (lines 1–60 and 130–175), `source/kernel/util/image_3d.h` (lines 203–263), `source/kernel/svm/convert.h`, `source/util/color.h` (the sRGB conversions), `source/kernel/util/colorspace.h` (lines 34–52).
- Checked `source/kernel/svm/types.h` lines 414–417 for the two Image Texture node flags, to be sure I understood what could alter the substituted value.
- Searched for `missingcolor` / `missing_color`. Two hits, both in `source/kernel/osl/services_gpu.h` — `osl_texture_set_missingcolor_arena` and `osl_texture_set_missingcolor_alpha`. **Ruled out:** both are empty function bodies in this snapshot, so they establish nothing.
- Listed `cycles_5.2/shader`. The node shaders there (including `node_image_texture.oso`, `node_environment_texture.oso`) are compiled, not readable text; searching them for `missing` returned nothing. **Ruled out as unreadable**, so I cannot say what the Open Shading Language path does.
- Searched the Cycles add-on's Python files (`cycles_5.2/*.py` — `properties.py`, `ui.py`, `engine.py`, `operators.py` and the rest) for `missing`. Nothing.
- Confirmed the snapshot's shape: `cycles_5.2/source` contains only `kernel` and `util`. There is no `scene` folder and no `device/cpu` folder, which is why the two gaps named under "Not covered" exist.

**The manual (`manual_5.2`) — searched, nothing on this question.**

- Searched the whole manual for `missing`. Twenty-six files. I opened or grepped the context of every one that could plausibly bear on a render: `interface/window_system/topbar.txt` (Report Missing Files and Find Missing Files — about locating and repairing broken links, not about what a render produces), `editors/video_sequencer/preview/sidebar.txt`, `video_editing/edit/montage/strips/image.txt`, `files/linked_libraries/link_append.txt`, `files/linked_libraries/library_overrides.txt`, `video_editing/setup/directory_structure.txt`, `render/eevee/*`, `render/cycles/optimizations/*`. **Ruled out:** the linked-library pages concern placeholder data-blocks for missing *linked blend-files*, not image files — "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones" (https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html, read 2026-07-30). The video-editing page only warns that a tidy directory lowers the chance of a "file is missing" error. The Cycles and EEVEE hits were about noise and ray tracing.
- Searched the whole manual for `magenta`. Five files: `editors/video_sequencer/preview/sidebar.txt`, `render/color_management/displays_views.txt`, `render/output/properties/stereoscopy/usage.txt`, `modeling/meshes/tools/loop.txt`, `compositing/types/color/adjust/color_balance.txt`. Only the first concerns missing media, and it is the **Video Sequencer**, not shading: "Show Missing Media — Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent." (https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html, read 2026-07-30). **I ruled this out as the answer** — a sequencer strip is a picture in a timeline, not an image feeding a shading input, and the question's second clause does not apply to it. It is worth knowing that a switch of that kind exists in one part of Blender; I found no equivalent switch for shading images anywhere in the sources. The matching property is `SequenceEditor.show_missing_media` — "Render missing images/movies with a solid magenta color (default False)" (https://docs.blender.org/api/5.2/bpy.types.SequenceEditor.html, read 2026-07-30). Note the default is False.
- Searched the whole manual for: `pink`, `purple`, `placeholder`, `dummy`, `broken`, `not found`, `cannot be found`, `can't be found`, `does not exist`, `no longer exist`, `unable to load`, `fails to load`, `failed to load`, `invalid image`, `image is missing`, `missing texture`, `missing image`, `missing file`, `no image`, `empty image`, `renders as`, `render as magenta`, `solid magenta`, `unfound`, `absent`. Of these, `can't be found`, `unable to load`, `fails to load`, `failed to load`, `invalid image`, `missing texture`, `dummy`, `renders as`, `render as magenta`, `unfound` and `absent` returned **no matches anywhere in the manual**. The rest returned only matches on unrelated subjects (empties, text formatting, recent-files lists, UV editing, weight paint, geometry-node bundles, OpenColorIO configs, translation files).
- Read the Image Texture node page in full: https://docs.blender.org/manual/en/5.2/render/shader_nodes/textures/image.html (read 2026-07-30). It documents the Vector input, Interpolation, Projection, Extension, Source, Frames, Colour Space, Alpha, and the two outputs. Of the Colour output it says only: "RGB color from image. If the image has transparency, the color is premultiplied if the Alpha output is used, and non-premultiplied (straight) otherwise." **Nothing about a missing image.**
- Grepped the Image Settings page (`editors/image/image_settings.txt`) for `missing`, `not found`, `exist`, `reload`, `magenta`, `pink`. Only "Reload — Reloads the file. Useful when it has been reworked in an external application."
- Grepped the compositor's Image node page (`compositing/types/input/image.txt`) for the same terms. Nothing.

**The Python API documentation (`api_5.2`) — searched, nothing on this question.**

- Searched all of `api_5.2` for `magenta`. Three hits: `SequenceEditor.show_missing_media` (quoted above) and two stereoscopy enum entries. **Nothing for shading.**
- Searched all of `api_5.2` for `missing` crossed with image, texture, file or found. The hits were `ID.is_missing` (repeated on every data-block page — "This data-block is a place-holder for missing linked data", again about linked blend-files, not image files), the four `ImagePaint.missing_*` flags (about a mesh lacking materials, a stencil, a texture or a UV layer for painting — **ruled out**, that is texture painting, not rendering), `BlendFileColorspace.is_missing_opencolorio_config`, `bpy.ops.file.find_missing_files` and `bpy.ops.file.report_missing_files` (both quoted above), and a sequencer `use_placeholders` option.
- Read `bpy.types.Image` around `has_data` — "True if the image data is loaded into memory (default False, readonly)" (https://docs.blender.org/api/5.2/bpy.types.Image.html, read 2026-07-30). This tells you whether an image is loaded; it says nothing about what a render produces when it is not. It is, however, the most useful thing I found for detecting the condition rather than describing it.

**The release notes (`release_notes_5.2`) — searched, nothing on this question.**

- Listed all twenty-one files and searched all of them for `missing`, `magenta`, `pink`, `placeholder`. The only hits were: a fixed EEVEE volume refresh; a Rigify script error; a documentation page gap; "Crash unloading blend file with missing linked sound"; "Regression UDIM numbering missing since Blender 5.0"; missing Blosc compression; missing operator presets; "Missing Freestyle Lines"; a missing icon transparency; "Crash when freeing ImBuf cache of an image sequence with a missing image"; a VSE progress indicator; a Linux OpenGL check; "VSE Transform's placeholder versioning is incorrect"; and "Vulkan EEVEE reflections randomly turn pink". **None of these is a statement about what a render produces for a missing shading image**, and I am reporting them only so the absence is checkable. I flag the last two because a reader scanning for "pink" would otherwise stop there: the Vulkan EEVEE one is a fixed bug about reflections, unrelated to missing files.

**I did not use the web, and did not need to.**

---

## Hand-test results — 2026-08-09

Everything in this section is **observed**. Each result says what was done, to what, what came back, and on what kind of computer. The tests, their controls, and what a negative result would have looked like are written out in `Audits/Blender Research/Images/Hand tests — later questions.md`; the entry numbers below are that list's numbers.

Every run was started with `--background --factory-startup`, with `BLENDER_USER_RESOURCES` pointed at an empty folder made just before the run, and with a fresh working folder of its own. Every render was written as OpenEXR at full float and the numbers were read back out of the saved file, because Blender writes text into a saved picture that nothing controls, so pictures were compared as pixels and never as files. "Missing" below always means the picture file was moved out of the folder after the blend-file had been saved, so the path in the blend-file no longer resolves.

**The two computers, named in full.** Neither speaks for the other, so every test was run on both and both results are given.

- **The rented Linux machine** — rented on Vast.ai and reached over SSH: Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, an AMD EPYC 7K62 with twenty-three of its cores allotted, 49 GB of memory, and no display server at all. Blender 5.2.0 LTS, from `/opt/blender/5.2/blender`.
- **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics, no second card. Blender 5.2.0 LTS.

### 1. The numbers a missing picture hands out (entry 1)

**What was done.** A 64 by 64 picture file was written with every pixel red 0, green 1, blue 0, alpha 1, and a second one with every pixel white and alpha 0.25. A plane filling an orthographic camera was given an Emission-only material with an Image Texture node, in three wirings: the node's **Color** into Emission Color at strength 1; the node's **Alpha** into Emission Strength with a white colour; and the node's **Color** into Emission Strength with a white colour, which is a colour output plugged into a socket that takes one number. Cycles, one sample, 32 by 32 pixels, filter width 0. Each was rendered with the picture present and again with it moved away.

**What came back.** The Creator's Mac printed:

```
RESULT col_cpu_default present [0.0, 1.0, 0.0, 1.0]
RESULT alpha_cpu_default present [0.250980406999588, 0.250980406999588, 0.250980406999588, 1.0]
RESULT grey_cpu_default present [0.7151691317558289, 0.7151691317558289, 0.7151691317558289, 1.0]
RESULT col_gpu_default present [0.0, 1.0, 0.0, 1.0]
RESULT col_cpu_noncolor present [0.0, 1.0, 0.0, 1.0]
RESULT col_cpu_default missing [1.0, 0.0, 1.0, 1.0]
RESULT alpha_cpu_default missing [1.0, 1.0, 1.0, 1.0]
RESULT grey_cpu_default missing [0.28483182191848755, 0.28483182191848755, 0.28483182191848755, 1.0]
RESULT col_gpu_default missing [1.0, 0.0, 1.0, 1.0]
RESULT col_cpu_noncolor missing [1.0, 0.0, 1.0, 1.0]
```

The rented Linux machine printed exactly the same ten lines, digit for digit.

The two machines agreed. On both, the picture-present runs gave red 0, green 1, blue 0 and an alpha of 0.250980406999588, which is the control showing the file really was being read.

So, with the picture missing: the node's Color output carried red 1, green 0, blue 1; its Alpha output carried 1; and a colour output plugged into a socket taking one number gave 0.28483182191848755 on both computers. The graphics card gave the same numbers as the processor. Setting the node's Colour Space to `Non-Color` gave the same numbers as leaving it at `sRGB`.

### 2. A missing picture feeding a normal map, a bump height and a displacement height (entry 2)

> **Note from the citation check, 2026-08-09 (Opus 5).** This entry cannot be repeated from its own words. It does not say how big the spheres are, where they sit, how the camera is framed, which way the sun points or how bright it is, or what colour the surfaces are — and every number below depends on all of those. Re-running it with those details invented reproduces the three results in kind (the normal-map sphere differs from the unwired one; the bump sphere does not; the displacement sphere's outline sits between the two) but not the figures. A test somebody can follow is written out in the hand-test list under this check's heading.

**What was done.** A 64 by 64 black-and-white checker was written. Three UV-unwrapped spheres of 128 by 64 segments sat side by side under one sun lamp with its angle set to 0, against a black world, with Cycles at four samples and no light bounces, so that two identical set-ups give identical pixels. The left sphere took the picture through a Normal Map node into a Diffuse BSDF's Normal input; the middle one through a Bump node's Height with Distance 5; the right one through a Displacement node's Height at scale 0.4 into the Material Output, with the material's displacement method set to `DISPLACEMENT`. Three renders were made: picture present and wired; picture moved away and wired; picture present with the link out of each Image Texture node removed. The frame was 240 by 80 with a transparent film, so each sphere has its own third of the frame and the number of pixels inside its outline can be counted.

**What came back.** The Creator's Mac printed:

```
DIFF normalmap  present_vs_unwired mean 0.083640  max 0.661317
DIFF normalmap  missing_vs_unwired mean 0.056763  max 0.518992
DIFF normalmap  missing_vs_present mean 0.048621  max 0.605540
SILHOUETTE normalmap  present  pixels 2227
SILHOUETTE normalmap  missing  pixels 2227
SILHOUETTE normalmap  unwired  pixels 2227
DIFF bump       present_vs_unwired mean 0.021423  max 0.646899
DIFF bump       missing_vs_unwired mean 0.000030  max 0.006250
DIFF bump       missing_vs_present mean 0.021396  max 0.646899
SILHOUETTE bump       present  pixels 2227
SILHOUETTE bump       missing  pixels 2227
SILHOUETTE bump       unwired  pixels 2227
DIFF displace   present_vs_unwired mean 0.109908  max 0.657192
DIFF displace   missing_vs_unwired mean 0.022689  max 0.650827
DIFF displace   missing_vs_present mean 0.102246  max 0.657192
SILHOUETTE displace   present  pixels 3124
SILHOUETTE displace   missing  pixels 1849
SILHOUETTE displace   unwired  pixels 1424
```

The rented Linux machine printed the same, with two lines differing in their last digits: `DIFF normalmap present_vs_unwired mean 0.083607 max 0.661317` and `DIFF normalmap missing_vs_present mean 0.048588 max 0.605540`. Every silhouette count matched.

The `present_vs_unwired` line in each group is the control showing that each of the three wirings does change the picture when the file is there.

So: with the picture missing, the normal-map sphere differed from the same sphere with nothing connected. The bump sphere did not — the largest difference of any single channel anywhere on it was 0.006250 on both computers, against 0.65 for the control. The displacement sphere's outline held 1849 pixels with the picture missing, 3124 with it present, and 1424 with nothing connected; the undisplaced spheres beside it held 2227.

### 3. What the render says, whether it finishes, and what it writes (entry 3)

**What was done.** The Emission plane from entry 1, with its Color wired to Emission Color, was rendered from a terminal with the ordinary output and the error output captured separately, once with the picture present and once with it moved away. The exact command was `BLENDER_USER_RESOURCES=<fresh folder> <blender> --background --factory-startup <work>/col.blend --render-output <work>/missing_ --render-frame 1`.

**What came back, on both computers.** Both runs ended with exit status 0 and both wrote their frame. Standard error was empty, 0 bytes, in both runs. The missing-picture run printed two lines that the picture-present run did not. On the Creator's Mac:

```
00:00.412  cycles           | WARNING Image file <work folder>/flat.png does not exist.
00:00.413  cycles           | ERROR Failed to load 1 image files
```

and on the rented Linux machine:

```
00:00.603  cycles           | WARNING Image file /root/ht-images/linuxC/e3-5.2/flat.png does not exist.
00:00.605  cycles           | ERROR Failed to load 1 image files
```

Both sit on standard output, not on standard error. The first names the file and is a WARNING; the second is an ERROR and counts the files rather than naming them.

Both runs also printed the line `Fra: 1 | Mem: 1M | Updating Images | Loading flat.png` — that line appears whether or not the file is there, so it is not the report.

That the file really had not been read is shown by the frames themselves, on the Creator's Mac:

```
CENTREPIXEL present_0001.exr [0.0, 1.0, 0.0, 1.0] bytes 1612
CENTREPIXEL missing_0001.exr [1.0, 0.0, 1.0, 1.0] bytes 1410
```

and on the rented Linux machine, the same two centre pixels at 1508 and 1306 bytes.

### 4. `bpy.ops.file.report_missing_files()` with no window, and `has_data` (entry 4)

**What was done.** The same blend-file was opened with `--background --factory-startup` and a script printed the images and their `has_data` flag, called `bpy.ops.file.report_missing_files()`, printed its return value, rendered one frame, and printed the flags again. Run once with the picture present and once with it moved away.

**What came back, the same on both computers.** The operator returned `{'FINISHED'}` in both states. With the picture present, the run printed to standard output:

```
Info: No missing files
```

With the picture moved away, it printed instead:

```
Warning: Path '<work folder>/flat.png' not found, from local data-block 'IMflat.png'
```

So the report does reach the ordinary output when there is no window, and it says different things in the two cases.

`has_data` did not distinguish the two cases. It read `False` for the image before the render, after the operator, and after a completed render, in both states:

```
STAGE1 images before any render: [('flat.png', '//flat.png', False)]
STAGE2 images after the operator: [('flat.png', False)]
STAGE3 images after the render: [('flat.png', False), ('Render Result', False)]
```

### 5. EEVEE and Workbench (entry 5)

> **Note from the citation check, 2026-08-09 (Opus 5).** The EEVEE half of this entry repeats exactly, on both machines. The Workbench half does not, from these words alone: followed as written, Workbench shades a plain grey material and gives the same value whether the picture is there or not. Two settings this entry never mentions are needed — `scene.display.shading.color_type = 'TEXTURE'` and `scene.display.shading.light = 'FLAT'` — and with them the numbers below come back exactly. This check found and added them.

**What was done.** The blend-files from entries 1 and 2 were rendered again with the engine changed. Setting `scene.render.engine` was tried for `BLENDER_EEVEE_NEXT`, `BLENDER_EEVEE` and `BLENDER_WORKBENCH`, and what the property read back as was printed before each render. Both engines started and rendered on the rented Linux machine, which has no display server at all.

**What came back.** In 5.2, `BLENDER_EEVEE_NEXT` was refused:

```
ENGINE NOT ACCEPTED BLENDER_EEVEE_NEXT TypeError('bpy_struct: item.attr = val: enum "BLENDER_EEVEE_NEXT" not found in (\'BLENDER_EEVEE\', \'BLENDER_WORKBENCH\', \'CYCLES\')')
```

With `BLENDER_EEVEE`, the Creator's Mac and the rented Linux machine both printed:

```
RESULT col BLENDER_EEVEE present [0.0, 1.0, 0.0, 1.0]
RESULT col BLENDER_EEVEE missing [1.0, 0.0, 1.0, 1.0]
```

With `BLENDER_WORKBENCH`, the Creator's Mac printed:

```
RESULT col BLENDER_WORKBENCH present [0.0, 0.99755859375, 0.0, 0.998046875]
RESULT col BLENDER_WORKBENCH missing [0.99755859375, 0.0, 0.99755859375, 0.998046875]
```

and the rented Linux machine printed:

```
RESULT col BLENDER_WORKBENCH present [0.0, 0.9990234375, 0.0, 1.0]
RESULT col BLENDER_WORKBENCH missing [0.9990234375, 0.0, 0.9990234375, 1.0]
```

The three-sphere file rendered in `BLENDER_EEVEE` gave, on the Creator's Mac:

```
EEVEE DIFF normalmap  present_vs_unwired mean 0.094576  max 0.637192
EEVEE DIFF normalmap  missing_vs_unwired mean 0.052869  max 0.514374
EEVEE DIFF bump       present_vs_unwired mean 0.028460  max 0.627426
EEVEE DIFF bump       missing_vs_unwired mean 0.000000  max 0.000000
EEVEE SILHOUETTE displace   present  pixels 3140
EEVEE SILHOUETTE displace   missing  pixels 1852
EEVEE SILHOUETTE displace   unwired  pixels 1436
```

and on the rented Linux machine `EEVEE DIFF normalmap missing_vs_unwired mean 0.052861 max 0.514374`, `EEVEE DIFF bump missing_vs_unwired mean 0.000000 max 0.000000`, and the same three silhouette counts, 3140, 1852 and 1436.

### 6. Which kinds of absence give the same result (entry 6)

**What was done.** Six copies of the entry-1 blend-file, each in its own folder, with the picture in a different state, plus a seventh run with the picture correct as the control.

**What came back, the same on both computers.** The centre pixel in each state:

```
CASE present   blend points at //flat.png | that path exists on disk: True  | size: 376
RESULT present   [0.0, 1.0, 0.0, 1.0]
CASE moved     blend points at //flat.png | that path exists on disk: False | size: -
RESULT moved     [1.0, 0.0, 1.0, 1.0]
CASE badfolder blend points at <work>/nosuchfolder/flat.png | that path exists on disk: False | size: -
RESULT badfolder [1.0, 0.0, 1.0, 1.0]
CASE zerobyte  blend points at //flat.png | that path exists on disk: True  | size: 0
RESULT zerobyte  [1.0, 0.0, 1.0, 1.0]
CASE wrongext  blend points at <work>/flat.xyz | that path exists on disk: True | size: 376
RESULT wrongext  [0.0, 1.0, 0.0, 1.0]
CASE garbage   blend points at //flat.png | that path exists on disk: True  | size: 200
RESULT garbage   [1.0, 0.0, 1.0, 1.0]
```

`wrongext` is a working PNG file whose name ends `.xyz`, with the blend-file pointing at that name; `garbage` is a file named `flat.png` holding two hundred bytes of zeros.

5.2 printed lines in four of the six states, and the wording is not the same for all four:

```
moved:     cycles | WARNING Image file <path>/flat.png does not exist.
           cycles | ERROR Failed to load 1 image files
badfolder: cycles | WARNING Image file <path>/nosuchfolder/flat.png does not exist.
           cycles | ERROR Failed to load 1 image files
zerobyte:  cycles | WARNING Image file <path>/flat.png failed to load.
           cycles | WARNING Image file <path>/flat.png failed to load.
           cycles | ERROR Failed to load 1 image files
garbage:   cycles | WARNING Image file <path>/flat.png failed to load.
           cycles | WARNING Image file <path>/flat.png failed to load.
           cycles | ERROR Failed to load 1 image files
```

In the two states where the file is present but cannot be read, the WARNING line appears twice.

The `present` and `wrongext` states printed no such line.

A seventh state was run only on the rented Linux machine, because everything there otherwise runs as root and root reads past permissions without complaint. The work folder was handed to the ordinary user `tester` and Blender was run as that user. With the picture set to `chmod 644` the run printed `RESULT permok [0.0, 1.0, 0.0, 1.0]`; with it set to `chmod 600` and owned by root it printed `RESULT permdenied [1.0, 0.0, 1.0, 1.0]`, together with `cycles | WARNING Image file <path>/flat.png failed to load.` twice and `cycles | ERROR Failed to load 1 image files`. That the permission really bit was shown in the same run:

```
proof the permission really bites — tester trying to read it:
cat: <work>/flat.png: Permission denied
(that command's exit status: 1)
```

### 7. One missing tile out of a tiled picture (entry 7)

**What was done.** Two picture files, `tile.1001.png` all green and `tile.1002.png` all blue, on an image with `source = 'TILED'` holding tiles 1001 and 1002, on a plane whose texture coordinates run from 0 to 2 across the frame. Rendered with both files present, then with `tile.1002.png` alone moved away.

**What came back, the same on both computers.**

```
RESULT present left quarter (tile 1001): [0.0, 1.0, 0.0, 1.0]  right quarter (tile 1002): [0.0, 0.0, 1.0, 1.0]
RESULT missing left quarter (tile 1001): [0.0, 1.0, 0.0, 1.0]  right quarter (tile 1002): [1.0, 0.0, 1.0, 1.0]
```

### 8. One missing frame out of a picture sequence (entry 8)

**What was done.** Three picture files, `seq.0001.png` green, `seq.0002.png` blue, `seq.0003.png` red, on an image with `source = 'SEQUENCE'` and a three-frame image user. Frames 1, 2 and 3 rendered with all three files present, then with `seq.0002.png` alone moved away.

**What came back, the same on both computers.**

```
RESULT present frame 1 [0.0, 1.0, 0.0, 1.0]
RESULT present frame 2 [0.0, 0.0, 1.0, 1.0]
RESULT present frame 3 [1.0, 0.0, 0.0, 1.0]
RESULT missing frame 1 [0.0, 1.0, 0.0, 1.0]
RESULT missing frame 2 [1.0, 0.0, 1.0, 1.0]
RESULT missing frame 3 [1.0, 0.0, 0.0, 1.0]
```

### 9. A picture packed into the blend-file whose file on disk is gone (entry 9)

**What was done.** The entry-1 scene with `bpy.ops.file.pack_all()` run before saving, then the picture file moved away, then rendered.

**What came back, the same on both computers.**

```
IMAGE flat.png packed: True | path on disk exists: False
RESULT missing [0.0, 1.0, 0.0, 1.0]
```

Entry 6's `moved` case is the control showing that moving this same file away otherwise gives red 1, green 0, blue 1.

### 10. Box projection and the world's Environment Texture (entry 10)

**What was done.** One blend-file with a cube on the left of the frame carrying an Emission material fed by an Image Texture node whose `projection` was `BOX`, driven by Generated coordinates, and a world using an Environment Texture node on the same picture, visible as the right of the frame.

**What came back, the same on both computers.**

```
RESULT present cube (box projection): [0.0, 1.0, 0.0, 1.0]  background (environment texture): [0.0, 1.0, 0.0, 1.0]
RESULT missing cube (box projection): [1.0, 0.0, 1.0, 1.0]  background (environment texture): [1.0, 0.0, 1.0, 1.0]
```

### 11. Open Shading Language (entry 11)

**What was done.** The entry-1 **col** and **grey** files rendered on the processor with `scene.cycles.shading_system = True`, the property read back and printed before each render.

**What came back, the same on both computers.**

```
shading_system reads back as: True
RESULT OSL col present [0.0, 1.0, 0.0, 1.0]
RESULT OSL grey present [0.7152000069618225, 0.7152000069618225, 0.7152000069618225, 1.0]
RESULT OSL col missing [1.0, 0.0, 1.0, 1.0]
RESULT OSL grey missing [0.2847999930381775, 0.2847999930381775, 0.2847999930381775, 1.0]
```

The colour is the same as the ordinary shading path's. The single number differs in its fifth decimal place onward: 0.2847999930381775 here against 0.28483182191848755 on both computers in entry 1.

### 12. The compositor (entry 12)

**What was first tried, and what came back.** The 4.5 route was tried first and failed on both computers:

```
Traceback (most recent call last):
AttributeError: 'Scene' object has no attribute 'node_tree'
```

**What was then done.** 5.2 holds the compositor as a node group on `scene.compositing_node_group`, so the scene was built that way instead: an Image node loading the picture, its Image output into the `Factor` input of a Color Ramp node left at its default black-to-white, and the Color Ramp's Color into a Group Output whose interface socket is a colour. `CompositorNodeValToRGB` and `CompositorNodeComposite` do not exist in 5.2; the node types used were printed:

```
COMPOSITOR ROUTE: scene.compositing_node_group
COMPOSITOR NODES: ['CompositorNodeImage', 'ShaderNodeValToRGB', 'NodeGroupOutput']
```

**What came back, the same on both computers.**

```
RESULT compositor present [0.7152172327041626, 0.7152172327041626, 0.7152172327041626, 1.0]
RESULT compositor missing [0.0, 0.0, 0.0, 1.0]
```

### 13. A geometry-nodes Image Texture node driving position (entry 13)

**What was done.** A 4 by 4 grid of vertices with a Geometry Nodes modifier: an Image Texture node holding the picture, fed by Position, its Color output into a Set Position node's Offset. The evaluated coordinates of the sixteen vertices were printed with the picture present and with it moved away.

**What came back, the same on both computers.** The plain grid, before the modifier, runs from y −1 to y +1. With the picture present every vertex moved by about +1 in y — the picture's green channel:

```
PICTURE PRESENT, POSITIONS: [(-1.0, 2e-05, 0.0), (-0.33333, 2e-05, 0.0), ... (1.0, 2.00002, 0.0)]
```

With the picture moved away the coordinates were the plain grid exactly:

```
POSITIONS missing [(-1.0, -1.0, 0.0), (-0.33333, -1.0, 0.0), (0.33333, -1.0, 0.0), (1.0, -1.0, 0.0), (-1.0, -0.33333, 0.0), ... (1.0, 1.0, 0.0)]
```

so the offset it received was 0, 0, 0.

### 14. A Volume object whose OpenVDB file is gone (entry 14)

> **Note from the citation check, 2026-08-09 (Opus 5).** This entry does not record the frame size, where the volume sits, how the camera is framed, or what the world is, and the mean and largest pixel values below depend on all four. It was not re-run by this check. A test somebody can follow is written out in the hand-test list under this check's heading.

**What was first tried, and what came back.** Building the material with `pv.density_attribute = "density"` failed on both computers:

```
Traceback (most recent call last):
AttributeError: 'ShaderNodeVolumePrincipled' object has no attribute 'density_attribute'
```

The node's inputs were then printed and the density attribute was left at the node's own default:

```
PRINCIPLED VOLUME INPUTS: ['Color', 'Color Attribute', 'Density', 'Density Attribute', 'Anisotropy', 'Absorption Color', 'Emission Strength', 'Emission Color', 'Blackbody Intensity', 'Blackbody Tint', 'Temperature', 'Temperature Attribute', 'Weight']
```

**What was done.** A real OpenVDB file was produced first, by baking a small smoke simulation with the cache format set to OpenVDB, so that the test has a control showing a present volume file does render: `BAKE RETURNED {'FINISHED'}` and `CACHE FILE .../fluid_data_0001.vdb 57570`. That file was given to a Volume object with a Principled Volume material at emission strength 1, in front of an orthographic camera, rendered with Cycles at 128 samples. Three renders: file present, volume object removed altogether, file moved away.

**What came back, the same on both computers**, as the mean and largest pixel value over the frame:

```
RESULT present   mean/max: (0.2186993509531021, 1.7763690948486328)
RESULT novolume  mean/max: (0.049999963492155075, 0.049999967217445374)
RESULT missing   mean/max: (0.049999963492155075, 0.049999967217445374)
```

The missing-file render matched the render with no volume object at all, to every digit printed. Nothing in either run's output matched a search for `ERROR`, `WARNING`, `Error` or `Traceback`.

### The second batch of hand tests — 2026-08-09

The three results that follow were run on the same day as the fourteen above, after the citation check found three of those fourteen recorded too thinly for anybody to repeat. They are entries 15, 16 and 17 in the hand-test list. Nothing above is changed or withdrawn by them. Both computers are the two named at the head of this section, and both were used for all three.

### 15. A missing picture feeding a normal map, a bump height and a displacement height, rebuilt so it can be repeated (entry 15)

> **Why there are two of these.** The citation check found that result 2 above cannot be repeated from its own words and wrote out a test with every measurement in it. That test is entry 15 in the hand-test list, and this is what it gave. **Result 2 above is untouched and its figures stand.** This is a second run over the same ground, not a correction of it, and the two set-ups are different so the figures are not meant to match.

**What was done.** A 64 by 64 black-and-white checker of 8-pixel squares, colour space `Non-Color`. Three UV spheres of 128 segments by 64 rings, radius 0.4, shaded smooth, on a black world, lit by one sun lamp at angle 0, energy 3, turned 45 degrees about X and 30 degrees about Z. An orthographic camera at (0, 0, 5). The frame 240 by 80 with a transparent film, Cycles at 4 samples with denoising off and every bounce count set to 0, output OpenEXR at full float. Each sphere carries a Diffuse BSDF with colour 0.5, 0.5, 0.5 and roughness 0. The left sphere takes the picture through a Normal Map node into the Normal input; the middle one through a Bump node's Height with Distance 5; the right one through a Displacement node's Height at Scale 0.4 into the Material Output, with the material's displacement method set to `DISPLACEMENT`.

Four renders of the same file, not three: **present** (picture there, everything wired), **missing** (picture moved out of the folder), **unwired** (the link out of each Image Texture node removed), and **nonode** (as unwired, and the Normal Map, Bump and Displacement node taken out of the chain as well, so each sphere is plain and untouched).

The fourth render is there because of something the run itself printed: the Normal Map node's own default Colour is **0.5, 0.5, 1**. So *unwired* is not "no normal map" — it is "a normal map holding 0.5, 0.5, 1", and without a plain sphere to compare against, nothing in the run fixes what "unchanged" means. The Displacement node's own default Height printed as **0.0** and its Midlevel as **0.5**; the Bump node's own default Height printed as **1.0**.

**What came back, first — with the framing exactly as it was written out.** The check's framing puts the three spheres at x = −1, 0, +1 in an orthographic width of 3. Run that way, the displaced sphere runs off the picture. Counting lit pixels on each edge row of the frame, the picture-present render gave top 51, bottom 52, left 0, right 52, on both computers; the other three renders touched no edge at all. In the same render the middle third of the frame held 3469 lit pixels against 3217 in every other state, because part of the displaced sphere had grown into its neighbour's third. The displaced sphere's own third counted 5877 present, 1968 missing, 800 nothing-connected, 3207 plain, and those counts are cut short by the frame edge rather than measured. So the figures from that framing are recorded here as what happened and are not used to answer anything.

**What came back, second — with the framing widened.** Two numbers were changed and nothing else: the orthographic width to 4.5 and the spheres to x = −1.5, 0, +1.5. Nothing touched any edge of the frame in any of the four renders, on either computer. The figures below are the mean and the largest single-channel difference over each third of the frame; the two computers agreed except in the last digits of one mean.

```
DIFF normalmap  present_vs_unwired mean 0.042219  max 0.501329
DIFF normalmap  missing_vs_unwired mean 0.033153  max 0.400838
DIFF normalmap  unwired_vs_nonode  mean 0.000000  max 0.000000
DIFF normalmap  missing_vs_nonode  mean 0.033153  max 0.400838
DIFF bump       present_vs_unwired mean 0.011654  max 0.493388
DIFF bump       missing_vs_unwired mean 0.000028  max 0.006250
DIFF bump       unwired_vs_nonode  mean 0.000051  max 0.006250
DIFF displace   present_vs_unwired mean 0.103849  max 0.501674
DIFF displace   missing_vs_unwired mean 0.022120  max 0.501927
DIFF displace   unwired_vs_nonode  mean 0.045107  max 0.502084
DIFF displace   missing_vs_nonode  mean 0.023163  max 0.495619
SILHOUETTE normalmap  present 1416  missing 1416  unwired 1416  nonode 1416
SILHOUETTE bump       present 1415  missing 1415  unwired 1415  nonode 1415
SILHOUETTE displace   present 3045  missing 868  unwired 348  nonode 1424
```

That block is the Creator's Mac. The rented Linux machine gave `present_vs_unwired mean 0.042198` for the normal map where the Mac gave 0.042219, and every other figure above to the digit printed.

**The controls.** `present_vs_unwired` in each group is the first control: each of the three wirings changes the picture when the file is really there. `unwired_vs_nonode` for the normal map is the second: a Normal Map node fed 0.5, 0.5, 1 gives a picture identical to no Normal Map node at all, mean 0.000000 and largest 0.000000, so "unchanged" has a fixed meaning in this run.

**So, with the picture missing:**

- **The normal map.** The surface is not left as it was. The missing-picture render differs from the plain sphere by mean 0.033153 and largest 0.400838 of a channel, against a control that shows what "identical" looks like here — exactly zero.
- **The bump height.** The surface is left as it was. The largest single-channel difference anywhere on that sphere between the missing-picture render and the nothing-connected render was 0.006250, against 0.493388 for the control.
- **The displacement height.** The sphere gets smaller. Its outline held 1424 pixels untouched, 868 with the picture missing, 348 with nothing connected to the Displacement node, and 3045 with the picture present. The other two spheres, which are not displaced, held 1416 and 1415 in every state.

### 16. Workbench with a missing picture, with the two settings it needs (entry 16)

> **Why there is a second one.** The citation check found that the Workbench half of result 5 above cannot produce its own numbers from its own words, and wrote the test out again with the two missing settings in it. That test is entry 16 in the hand-test list. Result 5 above is untouched.

**What was done.** The `col` blend-file from result 1 above — a plane filling an orthographic camera, an Emission-only material, the Image Texture node's Colour into Emission Colour, 32 by 32 pixels, OpenEXR at full float, view transform `Standard`. The render engine was set to `BLENDER_WORKBENCH` and the frame rendered twice in the same run: once with nothing else touched, and once with `scene.display.shading.color_type` set to `TEXTURE` and `scene.display.shading.light` set to `FLAT`. Both were run with the picture present and again with it moved out of the folder. Four renders. The two settings were printed before and after each change.

**What came back.** Both settings read `MATERIAL` and `STUDIO` before the change, in every version on both computers.

The Creator's Mac:

```
RESULT bare     present [0.357666015625, 0.366943359375, 0.37060546875, 0.998046875]
RESULT bare     missing [0.357666015625, 0.366943359375, 0.37060546875, 0.998046875]
RESULT textured present [0.0, 0.99755859375, 0.0, 0.998046875]
RESULT textured missing [0.99755859375, 0.0, 0.99755859375, 0.998046875]
```

The rented Linux machine, which has no display server at all and rendered Workbench anyway:

```
RESULT bare     present [0.358154296875, 0.3671875, 0.37109375, 1.0]
RESULT bare     missing [0.358154296875, 0.3671875, 0.37109375, 1.0]
RESULT textured present [0.0, 0.9990234375, 0.0, 1.0]
RESULT textured missing [0.9990234375, 0.0, 0.9990234375, 1.0]
```

**So:** without those two settings Workbench gives the same grey whether the picture is there or not, which is why the entry could not be followed as written. With them, the picture-present render gives the picture's own green — that is the control — and the picture-missing render gives magenta, the same substitution Cycles makes. The two computers differ in the last digits and in the alpha, and agree on everything the test asks.

### 17. A Volume object whose OpenVDB file is gone, with the whole scene written down (entry 17)

> **Why there is a second one.** The citation check found that result 14 above does not record the frame, the camera, the world or where the volume sits, and did not re-run it. It asked for a test with all of that written down. That test is entry 17 in the hand-test list. Result 14 above is untouched.

**What was done.** First the file: an icosphere of radius 0.5 at the origin, `bpy.ops.object.quick_smoke()`, the domain's cache pointed into the working folder with its data format set to `OPENVDB`, frames 1 to 4, resolution maximum 32, then `bpy.ops.fluid.bake_data()`. The bake returned `{'FINISHED'}` and wrote two files, `config_0001.uni` at 88 bytes and `fluid_data_0001.vdb` at **57,570 bytes**, the same sizes in every version on both computers. That file was copied beside the blend-file as `vol.vdb`.

Then the scene, with every number recorded: a Volume data-block pointing at `//vol.vdb`, on an object at location (0, 0, 0), rotation 0, scale (1, 1, 1); a Principled Volume material with Emission Strength 1 and Emission Colour white, the density attribute left at the node's own default; an orthographic camera at (0, 0, 6), no rotation, orthographic width 4; the frame 64 by 64 with the film **not** transparent; a world data-block with its nodes switched off and its colour black; Cycles at 128 samples with denoising off. With the file loaded, the object's dimensions read (3.75, 3.75, 5.0). In 5.2 setting the world's nodes off did not take — the property read back as True straight after it was set — so the empty frame carries the world's own default grey rather than black.

Three renders: file present; the Volume object deleted altogether; the file moved out of the folder.

**What came back, the same on both computers**, as the mean and the largest pixel value over the frame:

```
RESULT present   mean 0.2186993509531021  max 1.7763690948486328
RESULT novolume  mean 0.049999963492155075  max 0.049999967217445374
RESULT missing   mean 0.049999963492155075  max 0.049999967217445374
```

**So:** the file-present render against the object-deleted render is the control, and it holds — the volume plainly changes the frame when its file is there. With the file gone, the frame matched the render with no Volume object at all, to every digit printed, on both computers. Searching the whole of every run's output for `ERROR`, `WARNING`, `Error` and `Traceback` matched no line, in either state, on either computer. A missing volume file draws nothing and says nothing.

### What these tests did not settle

- **Windows.** No Windows machine has run it. Everything above settles macOS on Apple silicon and Linux on an NVIDIA card, and leaves Windows untested rather than answered.
- **The second substitute value a tile can be given when it is asked for but not yet in memory.** No set-up is known that reliably holds a tile in that state at the moment a lookup happens; it is a passing state during a render, not something a scene can be built to sit in. Entry 7 reaches the case where the tile's file is absent, and not that one.
- **What a panel shows.** Where the report of `File ▸ External Data ▸ Report Missing Files` appears when there **is** a window was dropped this round: it needs eyes on a window, the rented Linux machine has no display server, and the Creator ran nothing. What the same command does with no window is a different test and is result 4 above.

---

## Things a person with Blender open could settle in a few minutes

Every item that stood here has been run, and the results are in the section above. Nothing from this list is still open.

Two things are still open and are not on this list: Windows, which no Windows machine has run, and the second substitute value a tile can be given when it is asked for but not yet in memory, which no machine this project has can reach. Both are named under *What these tests did not settle* above.

---

2026-08-09 — Opus 5 (Claude Code) — Images, Blender 5.2

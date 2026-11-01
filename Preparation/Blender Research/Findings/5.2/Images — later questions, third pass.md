# Images — later questions, third pass (Blender 5.2)

Sources consulted, all inside `Blender Sources/`, all snapshotted 2026-07-30 and read by me on that snapshot date:
the Blender 5.2 manual (`manual_5.2`), the Blender 5.2 Python API reference (`api_5.2`), the Blender 5.2 release notes
(`release_notes_5.2`), and Cycles' own files (`cycles_5.2`). Plus the question sheet
`Preparation/Blender Research/Questions/Images.md`.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

(Only question 9 was assigned to me. The rest of the sheet is untouched.)

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

**How I read the question.** It was ambiguous, and I answer the reading I think is meant: a blend-file points at an
image file on disk, that file is gone, and the Image Texture is wired into a non-colour input — a height/bump input,
a Normal Map node, a displacement, a roughness or metallic slot, an alpha, and so on. The other reading is that
"feeding something other than colour" means the image's **Color Space** is set to **Non-Color** (the manual's own
term for an image that carries data rather than colour). I cover that second reading too at the end of the answer,
because in Cycles the two readings turn out to hit the same code.

**LABEL: observed** for the substituted value; **inferred** for what that value does once it reaches a non-colour
input; **not in the sources** for EEVEE and for any user-visible warning in the finished picture.

### What is substituted (observed)

Cycles has a single, named colour it uses when an image cannot be found, and it is opaque magenta.

> ```
> /* Color to use when images are not found. */
> #define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)
> ```
> — Cycles' own files, `cycles_5.2/source/util/types_image.h`, lines 17–18. Read 2026-07-30.

That constant is returned from the image lookup at four points, and nowhere else in the files I was given:

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
> — `cycles_5.2/source/kernel/device/gpu/image.h`, lines 88–95. Read 2026-07-30.

> ```
>     if (!kernel_tile_descriptor_loaded(tile_descriptor)) {
>       return (tile_descriptor == KERNEL_TILE_LOAD_FAILED) ? IMAGE_MISSING_RGBA : tex.average_color;
>     }
> ```
> — same file, lines 112–114. Read 2026-07-30.

> ```
>     /* Full image sampling. */
>     if (tex.image_info_id == KERNEL_IMAGE_NONE) {
>       return IMAGE_MISSING_RGBA;
>     }
> ```
> — same file, lines 122–125. Read 2026-07-30.

> ```
>   const int image_texture_id = kernel_image_udim_map(kg, udim_id, uv.val);
>   if (image_texture_id == KERNEL_IMAGE_NONE) {
>     return IMAGE_MISSING_RGBA;
>   }
> ```
> — same file, lines 165–168 (the UDIM entry point). Read 2026-07-30.

The Image Texture node in the shader goes through exactly that lookup, and splits the four numbers into its two
outputs — the first three into Color, the fourth into Alpha:

> ```
> ccl_device float4 svm_image_texture(
>     KernelGlobals kg, ccl_private ShaderData *sd, const int id, const dual2 uv, const uint flags)
> {
>   float4 r = kernel_image_interp_with_udim(kg, sd, id, uv);
> ```
> — `cycles_5.2/source/kernel/svm/image.h`, lines 22–25. Read 2026-07-30.

> ```
>   if (stack_valid(node.out_offset)) {
>     stack_store_float3(stack, node.out_offset, make_float3(f));
>   }
>   if (stack_valid(node.alpha_offset)) {
>     stack_store_float(stack, node.alpha_offset, f.w);
>   }
> ```
> — same file, lines 70–75. Read 2026-07-30.

So, in Cycles, the missing image is not skipped and does not stop the render. Every sample of it returns the same
four numbers — red 1, green 0, blue 1, alpha 1 — everywhere on the surface, whatever the texture coordinate.
This is the same substitution whether the node feeds colour or anything else; there is no separate, quieter
fallback for non-colour use in any file I was given.

**One important gap in what I could check.** The Cycles folder I was given contains only the render kernel and its
utilities (`source/kernel` and `source/util`) plus the add-on's Python files. It does not contain the scene-side code
that loads image files and decides an image is unavailable — there is no `source/scene/` and no
`source/kernel/device/cpu/` in the snapshot. So I can show you the colour Cycles uses "when images are not found",
in its own words, and I can show you the three internal states that trigger it, but I cannot show you from these
files the line that puts a blend-file's broken image path into one of those states. Treat the link between
"the file on disk is gone" and "this constant is returned" as strongly indicated by the constant's own comment
rather than traced end to end.

### What that value does at a non-colour input (inferred)

Each of these is a step beyond what the file says. I write the step out.

**Into a single-number input** (Roughness, Metallic, Specular, IOR, a Height input, a Factor…). Cycles converts a
colour to a number with a luminance weighting:

> ```
>     case NODE_CONVERT_CF: {
>       const Float3Type f = stack_load<Float3Type>(stack, node.from_offset);
>       stack_store(stack, node.to_offset, linear_rgb_to_gray(kg, f));
> ```
> — `cycles_5.2/source/kernel/svm/convert.h`, lines 35–37. Read 2026-07-30.

> ```
> template<class T> ccl_device auto linear_rgb_to_gray(KernelGlobals kg, const T c)
> {
>   return dot(c, make_float3(kernel_data.film.rgb_to_y));
> }
> ```
> — `cycles_5.2/source/kernel/util/colorspace.h`, lines 34–37. Read 2026-07-30.

Step taken: magenta is (1, 0, 1), so the number handed to the input is the red weight plus the blue weight of the
scene's colour setup. The weights come from `kernel_data.film.rgb_to_y`, which these files do not fix to any
particular numbers, so I cannot state the resulting value. What I can say is that it is a single constant across
the whole surface, it is not zero, and it is not the value the artist intended. A missing roughness map therefore
does not render as "no roughness map" — it renders as a flat, wrong roughness.

**Into a bump chain.** The Bump node works from the difference between the height at the shading point and the
height a little to each side:

> ```
>     /* get bump values */
>     const float h_c = stack_load_float(stack, node.center_offset);
>     const float h_x = stack_load_float(stack, node.dx_offset);
>     const float h_y = stack_load_float(stack, node.dy_offset);
>
>     /* compute surface gradient and determinant */
>     const float det = dot(dP.dx, Rx);
>     const float3 surfgrad = (h_x - h_c) * Rx + (h_y - h_c) * Ry;
> ```
> — `cycles_5.2/source/kernel/svm/displace.h`, lines 52–59. Read 2026-07-30.

Step taken: because the missing image returns the same value at every coordinate, the three heights are equal, both
differences are zero, and the surface gradient is zero — so the bump contributes nothing and the surface is shaded
with its unperturbed normal. A missing bump map renders as a smooth surface, not as a magenta one.

**Into a Normal Map node.** The node decodes the colour as a direction:

> ```
>   float3 color = stack_load(stack, node.color);
>   color = 2.0f * make_float3(color.x - 0.5f, color.y - 0.5f, color.z - 0.5f);
> ```
> — `cycles_5.2/source/kernel/svm/tex_coord.h`, lines 225–226. Read 2026-07-30.

Step taken: magenta (1, 0, 1) decodes to (1, −1, 1), not to the neutral (0, 0, 1) that a flat normal map would give.
So a missing normal map does not fall back to flat shading: it tilts every shading normal by the same amount in
tangent space, which reads as a uniform, wrong lighting direction over the whole surface rather than as an obvious
magenta patch.

**Into displacement.** True displacement takes a vector and adds it to the surface position:

> ```
>     const float3 dP = stack_load_float3(stack, node.fac_offset);
>     sd->P += dP;
> ```
> — `cycles_5.2/source/kernel/svm/displace.h`, lines 109–110. Read 2026-07-30.

Step taken: a constant substituted value gives a constant displacement, so the surface is pushed by the same amount
everywhere rather than gaining or losing detail. I have not traced the full Displacement node here, only the node
that applies the result, so treat this as the weaker of these four inferences.

**Into Alpha.** The fourth number is 1, so a missing image feeding an alpha or transparency input makes the surface
fully opaque, not transparent.

### The short answer, plainly

In Cycles, the render still completes. Where the missing image is sampled, Cycles substitutes one fixed opaque
magenta value at every point. If that image had been feeding colour you would see magenta and know instantly.
Feeding something other than colour, you generally do **not** see magenta: you see a plausible-looking picture in
which one material property is a flat, wrong constant — smooth where there should be bump, uniformly mis-lit where
there should be a normal map, uniformly rough or shiny where there should be a map, opaque where there should be
transparency. The failure is silent inside the picture.

### The second reading — an image marked Non-Color

If the question means an image whose Color Space is set to Non-Color, the manual defines that setting:

> "Generic data that is not color, will not apply any color transform (e.g. normal maps or displacement maps)."
> — manual, https://docs.blender.org/manual/en/5.2/render/color_management/color_spaces.html . Read 2026-07-30.

> "Some images such as normal, bump or stencil maps do not strictly contain “colors” and should never have a color
> conversion applied to them. For such images, the color space should be set to Non-Color."
> — manual, https://docs.blender.org/manual/en/5.2/editors/image/image_settings.html . Read 2026-07-30.

Under that reading the answer does not change. The only colour-space step inside the image node is:

> ```
>   if (flags & NODE_IMAGE_COMPRESS_AS_SRGB) {
>     r = color_srgb_to_linear_v4(r);
>   }
> ```
> — `cycles_5.2/source/kernel/svm/image.h`, lines 33–35. Read 2026-07-30.

Step taken (inferred): that step is skipped for a Non-Color image, and in any case a value made only of 1s and 0s
is unchanged by an sRGB-to-linear conversion. So the substituted value is the same magenta either way.

### The one place the manual does document a missing image in a render

Not the shading case, but I record it because it is the only documented statement I found about what a Blender
render does with a missing image file, and it shows the same magenta choice:

> "Show Missing Media
>
> Render missing images/movies with a solid magenta color.
> When disabled, missing content will render fully transparent."
> — manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html . Read 2026-07-30.

The corresponding switch, and its default:

> "show_missing_media
>
> Render missing images/movies with a solid magenta color (default False)"
> — Python API, https://docs.blender.org/api/5.2/bpy.types.SequenceEditor.html . Read 2026-07-30.

This is the Video Sequencer, and the strips it applies to are pictures, not non-colour inputs. It does not answer
the question; it is the nearest documented neighbour, and by default it is off, meaning missing media in the
sequencer renders as nothing at all rather than as magenta.

### NOT COVERED

- **EEVEE.** Nothing in the manual, the API reference or the release notes says what EEVEE puts in a render where an
  image is missing, and no EEVEE source was in the sources I was given. Everything above is Cycles only.
- **The CPU path in Cycles.** The magenta returns I quoted are in the GPU image lookup. The CPU equivalent
  (`kernel/device/cpu/image.h`, referenced by `cycles_5.2/source/kernel/image.h`) is not present in the snapshot, so
  I cannot show that the CPU device does the same thing, only that the shared constant exists for both.
- **Whether the render reports the problem.** Nothing I found says whether a missing image produces a warning in the
  render result, in the status bar, in the log, or in a process exit code. The `Report Missing Files` menu entry
  writes "a warning message … in the Info editor's header", but that is a command a person runs, not something a
  render does.
- **Volumes.** `IMAGE_MISSING_RGBA` also appears as the final fallback in the 3D (volume) image lookup
  (`cycles_5.2/source/kernel/util/image_3d.h`, line 259). I did not trace whether a missing volume file reaches it,
  and volumes are not "an image" in the sense the question asks.
- **The exact number a scalar input receives**, because the luminance weights are scene data these files do not fix.
- **Whether the substitution changes with the new texture cache.** The `KERNEL_TILE_LOAD_FAILED` branch shows magenta
  is used when a tile fails to load, and `tex.average_color` when a tile is merely not loaded yet, but I could not
  find the code that sets `KERNEL_TILE_LOAD_FAILED`, so I cannot say which situations produce which.

### Outside the interface

The substitution I quoted lives in the render kernel — the part of Cycles that computes the picture — not in any
panel, editor or drawing code. **Inferred:** it therefore applies the same way when Blender is started and driven
from outside its own window, because nothing about it depends on a window being open. The sources do not say this
outright.

What clearly *is* interface-bound is the recovery: `Report Missing Files` and `Find Missing Files` are described in
the manual as menu commands, and `Find Missing Files` opens "A File Browser" for the person to pick a directory
(manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html , read 2026-07-30). Both do have
Python equivalents that a script can call —

> "bpy.ops.file.report_missing_files()
>
> Report all missing external files"
> — Python API, https://docs.blender.org/api/5.2/bpy.ops.file.html . Read 2026-07-30.

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)
>
> Try to find missing external files"
> — same page, read 2026-07-30.

— and `find_missing_files` takes a `directory` argument, so **inferred**, it can be pointed at a search folder
without a person choosing one. The 5.2 release notes also add a way for outside tooling to enumerate the files a
render will need:

> "Three new options for `bpy.data.file_path_foreach` so render farms and other scripts can find all relevant image
> files for rendering, including the new Cycles texture cache."
> — release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ . Read 2026-07-30.

The sources do not say whether a render driven from outside the window reports missing images any differently.

**Settled further by hand test, 2026-08-09 — observed.** The three tests in the *Hand-test results, second batch* section below were all run with Blender driven from a terminal, with no window at all, on both computers — including the rented Linux machine, which has no display server of any kind. Every one of them finished, wrote its frame and gave the numbers recorded there. Workbench, which is the viewport's own renderer, rendered there too. So what a render puts on the surface for a missing file does not depend on a window being open.

### Operating system

The sources do not say. The magenta constant is in a plain shared header with no operating-system conditions around
it, and the four places it is returned are likewise unconditional
(`cycles_5.2/source/util/types_image.h` line 18; `cycles_5.2/source/kernel/device/gpu/image.h` lines 94, 113, 124,
167), so **inferred**, the substituted value does not vary by operating system. The step I am supplying is that a
header with no platform conditions compiles the same everywhere. Nothing in the manual, the API reference or the
release notes ties missing-image behaviour to a particular operating system. Note that file paths themselves are
handled differently across systems in ways these files do not cover, so *which* files count as missing may well
differ even if what happens next does not.

**Settled further by hand test, 2026-08-09 — observed.** All three tests in the *Hand-test results, second batch* section below were run twice: on macOS 26.5.2 on an Apple M1 Pro, and on Ubuntu 22.04.5 LTS on kernel 6.8 with an NVIDIA GeForce RTX 5060. The two computers agreed on every one. Where any figure differed it differed in its last digits only, and each such difference is given where it occurs. Windows was not tested, because no Windows machine has run it, so it is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Where I looked

Searches were run over the whole of each source folder unless noted.

**Cycles' own files (`cycles_5.2`)**
- Listed the folder: it holds `__init__.py`, `camera.py`, `engine.py`, `license`, `maketx.py`, `operators.py`,
  `osl.py`, `presets.py`, `properties.py`, `shader/`, `source/`, `ui.py`, `version_update.py`. Under `source/` there
  is only `kernel/` and `util/`. **Ruled out as unavailable:** `source/scene/` (the image-loading side) and
  `source/kernel/device/cpu/` — neither exists in the snapshot.
- Searched all files for `missing` — 15 hits, of which the image-related ones are the five quoted above; the rest are
  about missing energy in microfacet BSDFs, missing intersections in curves, missing address-space qualifiers, and
  two no-op OSL stubs (`osl_texture_set_missingcolor_arena`, `osl_texture_set_missingcolor_alpha` in
  `source/kernel/osl/services_gpu.h`, which have empty bodies).
- Searched for `IMAGE_MISSING` — exactly six hits, all listed above.
- Read in full or in part: `source/util/types_image.h`, `source/kernel/device/gpu/image.h`,
  `source/kernel/util/image_2d.h`, `source/kernel/util/image_3d.h`, `source/kernel/image.h`,
  `source/kernel/svm/image.h`, `source/kernel/svm/convert.h`, `source/kernel/util/colorspace.h`,
  `source/kernel/svm/displace.h`, `source/kernel/svm/normal.h`, `source/kernel/svm/tex_coord.h`.
- Searched `shader/node_image_texture.oso` (the compiled OSL image-texture shader) for `missing`, `magenta` and
  `1 0 1` — no hits, so the OSL build carries no separate missing-image colour of its own that I could see.
- Searched the add-on's Python files for `missing`, `not found`, `exists`, `error` — the hits are about OSL
  compilation, `maketx` arguments and a missing animation frame, none about missing image textures.

**Manual (`manual_5.2`)**
- Searched the whole manual, case-insensitively, for: `missing`, `missing file`, `Find Missing Files`,
  `Report Missing Files`, `magenta`, `pink`, `purple`, `placeholder`, `not found`, `cannot be found`, `broken path`,
  `unresolved`, `fails to load`, `failed to load`, `unable to load`, `cannot load`, `no image is`, `If no image`,
  `image is not`, `does not exist`, `deleted or moved`, `moved or deleted`, `no longer exist`, `relink`, `absent`,
  `invalid path`, `highlighted red`, `Non-Color`, `texture cache`.
- Every `missing` hit in the manual is listed and accounted for: the topbar's Report/Find Missing Files entries; the
  sequencer's Show Missing Media; missing NLA; glTF alpha; missing caustics/highlights/shadow-updates in Cycles and
  EEVEE; missing node inputs in Cycles optimisation; missing EEVEE node features; a missing Python folder at startup;
  missing data layers in Data Transfer and Transfer Mesh Data; missing data in geometry-node and compositor Warning
  nodes; missing output directories in the File Output node; missing linked data-blocks; missing frames in an image
  sequence strip; and a line in the release history. None of them describes what a render puts on screen where a
  shading image is missing.
- Read in full or in part: `render/shader_nodes/textures/image.txt` (the Image Texture node page — it documents
  Vector, Interpolation, Projection, Extension, Source, Color Space, Alpha, and the Color and Alpha outputs, and says
  nothing about the image being unavailable), `interface/window_system/topbar.txt`,
  `editors/video_sequencer/preview/sidebar.txt`, `render/color_management/color_spaces.txt`,
  `editors/image/image_settings.txt`, `render/cycles/render_settings/performance.txt`, `files/file_paths.txt`,
  `files/blend/packed_data.txt`, `advanced/command_line/arguments.txt`.
- **Ruled out and why:** the `troubleshooting/` section (its five pages are about startup, crashes, the 3D Viewport,
  graphics drivers, networking and bug reporting — no page on missing assets); `files/blend/open_save.txt` (its
  "cannot be found" line is about pruning the recent-files list); `files/linked_libraries/` (its missing-data
  handling is about linked blend-files, not image files); `render/output/properties/output.txt` (its "Placeholders"
  are empty frames Blender writes out, the opposite direction).
- **Pictures:** the Image Texture node page shows `_images/node-types_ShaderNodeTexImage.webp` and four projection
  examples. I did not open them, because the question is about what happens when the image is absent and a screenshot
  of the node with an image assigned cannot show that. No page I found illustrates a missing texture.

**Python API reference (`api_5.2`)**
- Searched everything for `missing` and kept the file/image/texture/path hits: `ID.is_missing` (linked-data, appears
  on every ID page), `ImagePaint.missing_materials / missing_stencil / missing_texture / missing_uvs` (texture-paint
  readiness flags, not render behaviour), `BlendFileColorspace.is_missing_opencolorio_config`,
  `SequenceEditor.show_missing_media`, `bpy.ops.file.find_missing_files`, `bpy.ops.file.report_missing_files`,
  `bpy.ops.sequencer` `use_placeholders`.
- Searched everything for `magenta` — three hits: the sequencer property above, the green/magenta anaglyph type, and
  nothing else.
- Read `bpy.types.Image.txt` around `has_data` ("True if the image data is loaded into memory (default False,
  readonly)") and `ID.is_missing`; read `bpy.types.SequenceEditor.txt` and `bpy.ops.file.txt` at the lines quoted.
- **Ruled out:** `bpy.types.ShaderNodeTexImage` — I found no statement there about an unavailable image (the `missing`
  search over the whole API folder would have caught one).

**Release notes (`release_notes_5.2`)**
- Searched all subject files for `missing`, `magenta`, `texture cache`, `image cache`. The image-related hits are the
  new Cycles Texture Cache (`cycles.md`), the `bpy.data.file_path_foreach` options (`python_api.md`), and two entries
  in `bugfixes.md` — "Crash when freeing ImBuf cache of an image sequence with a missing image" and "Crash unloading
  blend file with missing linked sound". None states what a render produces for a missing image.

---

## Hand-test results, second batch — 2026-08-09

Everything in this section is **observed**. Each result says what was done, to what, what came back, and on what kind of computer. The tests, their controls, and what a negative result would have looked like are written out in `Audits/Blender Research/Images/Hand tests — later questions.md` under the heading *Added from the citation check — a second batch*; the entry numbers below are that list's numbers.

These three are a second batch. The topic's first fourteen hand tests were run on the same day and are written up in this version's `Images — later questions.md` and `Images — later questions, second pass.md`; they are not repeated here. The citation check found three of those fourteen recorded too thinly for anybody to repeat and wrote out what each should have been. Those three are entries 15, 16 and 17.

Every run was started with `--background --factory-startup`, with `BLENDER_USER_RESOURCES` pointed at an empty folder made just before that run, and with a fresh working folder of its own. Every render was written as OpenEXR at full float and the numbers were read back out of the saved file, so pictures were compared as pixels and never as files. "Missing" below always means the file was moved out of the folder after the blend-file had been saved, so the path in the blend-file no longer resolves.

**The two computers, named in full.** Neither speaks for the other, so every test was run on both and both results are given.

- **The rented Linux machine** — rented on Vast.ai and reached over SSH: Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, an AMD EPYC 7K62 with twenty-three of its cores allotted, 49 GB of memory, and no display server at all. Blender 5.2.0, from `/opt/blender/5.2/blender`.
- **The Creator's Mac** — MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics, no second card. Blender 5.2.0.

### 15. A missing picture feeding a normal map, a bump height and a displacement height, rebuilt so it can be repeated (entry 15)

> **Why there are two of these.** The citation check found that hand-test result 2 — which is written up in this version's `Images — later questions.md` and `Images — later questions, second pass.md`, not here — cannot be repeated from its own words and wrote out a test with every measurement in it. That test is entry 15 in the hand-test list, and this is what it gave. **That earlier result is untouched and its figures stand.** This is a second run over the same ground, not a correction of it, and the two set-ups are different so the figures are not meant to match.

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

> **Why there is a second one.** The citation check found that the Workbench half of hand-test result 5 — written up in this version's first and second pass documents, not here — cannot produce its own numbers from its own words, and wrote the test out again with the two missing settings in it. That test is entry 16 in the hand-test list. The earlier result is untouched.

**What was done.** The `col` blend-file from hand-test result 1, built again by the same script — a plane filling an orthographic camera, an Emission-only material, the Image Texture node's Colour into Emission Colour, 32 by 32 pixels, OpenEXR at full float, view transform `Standard`. The render engine was set to `BLENDER_WORKBENCH` and the frame rendered twice in the same run: once with nothing else touched, and once with `scene.display.shading.color_type` set to `TEXTURE` and `scene.display.shading.light` set to `FLAT`. Both were run with the picture present and again with it moved out of the folder. Four renders. The two settings were printed before and after each change.

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

> **Why there is a second one.** The citation check found that hand-test result 14 — written up in this version's first and second pass documents, not here — does not record the frame, the camera, the world or where the volume sits, and did not re-run it. It asked for a test with all of that written down. That test is entry 17 in the hand-test list. The earlier result is untouched.

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

### What this second batch did not settle

- **Windows.** No Windows machine has run it. The three results above settle macOS on Apple silicon and Linux on an NVIDIA card, and leave Windows untested rather than answered.

---

## Things a person at a computer with Blender open could settle in a few minutes

1. **The basic case, Cycles.** New file. Add a plane and a Principled BSDF. Add an Image Texture node, load any PNG,
   plug Color into Base Color. Save the blend-file, quit Blender, rename the PNG on disk, reopen the blend-file,
   render (F12). Note the colour of the plane in the render. Expected from the sources: magenta.
2. **The non-colour case, Cycles.** Same setup, but plug the Image Texture's Color into **Roughness** instead, with
   Base Color left at its default grey. Render before and after renaming the image on disk, side by side. Note
   whether the render turns magenta at all, and what roughness the surface appears to have. Also read off the actual
   number: temporarily plug the same Image Texture Color into an Emission shader on a flat plane facing the camera
   and sample the rendered pixel value.
3. ~~**Bump.**~~ **Answered** — see result 15 in the second batch of hand tests below, run 2026-08-09 in all four
   versions on both computers. With the picture gone the sphere is left as it was: the largest single-channel
   difference from a plain sphere with no Bump node at all was 0.006250, against 0.493388 for the control.
4. ~~**Normal map.**~~ **Answered** — see result 15 in the second batch of hand tests below, run 2026-08-09 in all
   four versions on both computers. A Normal Map node fed 0.5, 0.5, 1 renders identically to no Normal Map node at
   all, to mean 0.000000 and largest 0.000000; a missing picture on the same input differs from the plain sphere by
   mean 0.033153 and largest 0.400838 of a channel. So the surface is changed rather than left alone.
5. **Alpha.** Image Texture Alpha into the Principled BSDF Alpha. Render with the image renamed. Check whether the
   object is fully opaque.
6. **EEVEE.** Repeat steps 1, 2 and 4 with the render engine set to EEVEE and compare. The sources say nothing about
   EEVEE here, so anything found is new.
7. **Does anything warn you?** With the image renamed, render from the interface and watch the status bar and the
   Info editor. Then run the same file from a terminal with `blender -b scene.blend -f 1` and read everything the
   terminal prints, and check the process exit code afterwards. That settles whether a render farm could detect the
   problem without looking at the picture.
8. **Texture Cache on.** Enable Render Properties > Performance > Texture Cache with Auto Generate, render once so
   the `.tx` file is made, then delete or corrupt the `.tx` file (and separately, the original image) and render
   again. Note whether the result is magenta, a flat average colour, or something else — this is the one branch in
   the code I could not pin down.
9. **CPU versus GPU.** Run step 2 once with Cycles device set to CPU and once set to GPU Compute, and compare the
   rendered pixel value. The CPU lookup file was not in my sources.
10. **UDIM.** Use a tiled (UDIM) image where one tile file is missing and the others are present. Render and check
    whether only the missing tile's area is affected.

---

2026-08-09 — Opus 5 (Claude Code) — Images, Blender 5.2
